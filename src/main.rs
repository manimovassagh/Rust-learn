use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

fn main() {
    let task = Task {
        id: 1,
        description: "Learn Rust".to_string(),
        done: false,
    };

    println!("{:?}", task);
}