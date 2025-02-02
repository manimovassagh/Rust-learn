use druid::{Data, Lens};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub done: bool,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub tasks: Arc<Vec<Task>>,
    pub new_task_description: String,
    pub show_completed: bool,
}
