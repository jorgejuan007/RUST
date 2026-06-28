use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq)]
struct TaskRecord {
    id: i64,
    titulo: String,
    estado: String,
}

fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            titulo TEXT NOT NULL,
            estado TEXT NOT NULL
        );
        ",
    )
    .context("no se pudo crear el esquema SQLite")?;

    Ok(())
}

fn insert_task(conn: &Connection, titulo: &str, estado: &str) -> Result<i64> {
    conn.execute(
        "INSERT INTO tasks (titulo, estado) VALUES (?1, ?2)",
        params![titulo, estado],
    )
    .with_context(|| format!("no se pudo insertar la tarea {titulo}"))?;

    Ok(conn.last_insert_rowid())
}

fn mark_done(conn: &Connection, id: i64) -> Result<usize> {
    conn.execute(
        "UPDATE tasks SET estado = 'hecha' WHERE id = ?1",
        params![id],
    )
    .with_context(|| format!("no se pudo actualizar la tarea {id}"))
}

fn load_tasks(conn: &Connection) -> Result<Vec<TaskRecord>> {
    let mut stmt = conn
        .prepare("SELECT id, titulo, estado FROM tasks ORDER BY id")
        .context("no se pudo preparar la consulta de tareas")?;

    let rows = stmt
        .query_map([], |row| {
            Ok(TaskRecord {
                id: row.get(0)?,
                titulo: row.get(1)?,
                estado: row.get(2)?,
            })
        })
        .context("no se pudieron consultar las tareas")?;

    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(row.context("fila SQLite invalida")?);
    }

    Ok(tasks)
}

fn count_pending(conn: &Connection) -> Result<i64> {
    conn.query_row(
        "SELECT COUNT(*) FROM tasks WHERE estado != 'hecha'",
        [],
        |row| row.get(0),
    )
    .context("no se pudo contar las tareas pendientes")
}

fn search_by_estado(conn: &Connection, estado: &str) -> Result<Vec<TaskRecord>> {
    let mut stmt = conn
        .prepare("SELECT id, titulo, estado FROM tasks WHERE estado = ?1 ORDER BY id")
        .with_context(|| format!("no se pudo preparar la busqueda por estado {estado}"))?;

    let rows = stmt
        .query_map(params![estado], |row| {
            Ok(TaskRecord {
                id: row.get(0)?,
                titulo: row.get(1)?,
                estado: row.get(2)?,
            })
        })
        .with_context(|| format!("no se pudo ejecutar la busqueda por estado {estado}"))?;

    let mut tasks = Vec::new();
    for row in rows {
        tasks.push(row.context("fila SQLite invalida")?);
    }

    Ok(tasks)
}

fn seed(conn: &Connection) -> Result<()> {
    insert_task(conn, "Diseñar esquema SQLite", "hecha")?;
    insert_task(conn, "Practicar consultas SQL", "pendiente")?;
    insert_task(conn, "Conectar storage al backend", "en_progreso")?;
    Ok(())
}

fn run_demo(db_path: &Path) -> Result<()> {
    if db_path.exists() {
        fs::remove_file(db_path)
            .with_context(|| format!("no se pudo limpiar {}", db_path.display()))?;
    }

    if let Some(parent) = db_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("no se pudo crear {}", parent.display()))?;
    }

    let conn = Connection::open(db_path)
        .with_context(|| format!("no se pudo abrir {}", db_path.display()))?;

    init_schema(&conn)?;
    seed(&conn)?;
    let nueva_id = insert_task(&conn, "Persistir tareas localmente", "pendiente")?;
    mark_done(&conn, 2)?;

    let tasks = load_tasks(&conn)?;
    let pending = count_pending(&conn)?;
    let hechas = search_by_estado(&conn, "hecha")?;

    println!("Base SQLite: {}", db_path.display());
    println!("Tareas totales: {}", tasks.len());
    println!("Pendientes o en progreso: {pending}");
    println!("Ultima tarea insertada: #{nueva_id}");
    println!("Tareas hechas: {}", hechas.len());

    for task in tasks {
        println!("- #{} [{}] {}", task.id, task.estado, task.titulo);
    }

    Ok(())
}

fn main() -> Result<()> {
    run_demo(Path::new("target/bonus_tasks.sqlite3"))
}

#[cfg(test)]
mod tests {
    use super::{count_pending, init_schema, insert_task, load_tasks, mark_done, search_by_estado};
    use rusqlite::Connection;

    #[test]
    fn crea_y_lista_tareas_en_memoria() {
        let conn = Connection::open_in_memory().unwrap();
        init_schema(&conn).unwrap();

        insert_task(&conn, "Repasar SQLite", "pendiente").unwrap();
        insert_task(&conn, "Probar selects", "hecha").unwrap();

        let tasks = load_tasks(&conn).unwrap();

        assert_eq!(tasks.len(), 2);
        assert_eq!(tasks[0].titulo, "Repasar SQLite");
        assert_eq!(tasks[1].estado, "hecha");
    }

    #[test]
    fn actualiza_y_filtra_por_estado() {
        let conn = Connection::open_in_memory().unwrap();
        init_schema(&conn).unwrap();

        let id = insert_task(&conn, "Cambiar estado", "pendiente").unwrap();
        insert_task(&conn, "Dejar otra pendiente", "pendiente").unwrap();

        assert_eq!(count_pending(&conn).unwrap(), 2);
        assert_eq!(mark_done(&conn, id).unwrap(), 1);
        assert_eq!(count_pending(&conn).unwrap(), 1);

        let hechas = search_by_estado(&conn, "hecha").unwrap();
        assert_eq!(hechas.len(), 1);
        assert_eq!(hechas[0].id, id);
    }
}
