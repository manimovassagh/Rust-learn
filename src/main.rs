use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = load_tasks().unwrap_or_else(|_| vec![]);

    loop {
        println!("What would you like to do?");
        println!("1. Add a new task");
        println!("2. List all tasks");
        println!("3. Exit");

        let choice = get_input("Choose an option: ");
        match choice.trim() {
            "1" => {
                let description = get_input("Enter task description: ");
                let new_task = Task {
                    id: tasks.len() as u32 + 1,
                    description,
                    done: false,
                };
                tasks.push(new_task);
                save_tasks(&tasks);
            }
            "2" => {
                list_tasks(&tasks);
            }
            "3" => {
                break;
            }
            _ => println!("Invalid choice, try again."),
        }
    }
}

// Function to get input from the user
fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

// Function to save tasks to a JSON file
fn save_tasks(tasks: &Vec<Task>) {
    let file = File::create("tasks.json").expect("Failed to create file");
    serde_json::to_writer(file, tasks).expect("Failed to write tasks to file");
    println!("Tasks saved!");
}

// Function to load tasks from a JSON file
fn load_tasks() -> Result<Vec<Task>, serde_json::Error> {
    if std::path::Path::new("tasks.json").exists() {
        let file = File::open("tasks.json").expect("Failed to open file");
        let tasks: Vec<Task> = serde_json::from_reader(file)?;
        Ok(tasks)
    } else {
        Ok(vec![])
    }
}

// Function to list tasks
fn list_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("No tasks available.");
    } else {
        for task in tasks {
            println!("ID: {}, Description: {}, Done: {}", task.id, task.description, task.done);
        }
    }
}