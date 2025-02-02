use crate::types::{Task, AppState};
use druid::Lens;
use std::sync::Arc;
use std::fs::File;
use rand::random;
use std::error::Error;
use std::io::{self, Write};
use serde_json::Error as SerdeError;

pub fn add_task(data: &mut AppState) {
    let description = data.new_task_description.trim();
    if (!description.is_empty()) {
        let mut new_tasks = Vec::clone(&data.tasks);
        new_tasks.push(Task {
            id: random(),
            description: description.to_string(),
            done: false,
        });
        data.tasks = Arc::new(new_tasks);
        data.new_task_description.clear();
        if let Err(e) = save_tasks(&data.tasks) {
            eprintln!("Error saving tasks: {}", e);
        }
    }
}

pub fn filtered_tasks_lens() -> impl Lens<AppState, Arc<Vec<Task>>> {
    druid::lens::Map::new(
        |data: &AppState| {
            if data.show_completed {
                data.tasks.clone()
            } else {
                Arc::new(data.tasks.iter()
                    .filter(|task| !task.done)
                    .cloned()
                    .collect())
            }
        },
        |data: &mut AppState, filtered: Arc<Vec<Task>>| {
            if data.show_completed {
                data.tasks = filtered;
            }
        }
    )
}

pub fn load_tasks() -> Result<Arc<Vec<Task>>, Box<dyn Error>> {
    if std::path::Path::new("tasks.json").exists() {
        let file = File::open("tasks.json").map_err(|e| {
            eprintln!("Error opening tasks.json: {}", e);
            e
        })?;
        let tasks: Vec<Task> = serde_json::from_reader(file).map_err(|e| {
            eprintln!("Error reading tasks.json: {}", e);
            e
        })?;
        Ok(Arc::new(tasks))
    } else {
        Ok(Arc::new(Vec::new()))
    }
}

pub fn save_tasks(tasks: &Arc<Vec<Task>>) -> Result<(), Box<dyn Error>> {
    let file = File::create("tasks.json").map_err(|e| {
        eprintln!("Error creating tasks.json: {}", e);
        e
    })?;
    serde_json::to_writer(file, tasks.as_ref()).map_err(|e| {
        eprintln!("Error writing to tasks.json: {}", e);
        e
    })?;
    Ok(())
}
