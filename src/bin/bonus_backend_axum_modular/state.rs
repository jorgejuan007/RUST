use crate::storage::TaskStore;

#[derive(Clone)]
pub struct AppState {
    pub store: TaskStore,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            store: TaskStore::seed(),
        }
    }
}
