use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    city: String,
}

fn main() -> std::io::Result<()> {
    let person = Person {
        name: "Mani".to_string(),
        age: 25,
        city: "Rustland".to_string(),
    };

    let json_data = serde_json::to_string_pretty(&person).expect("Failed to serialize JSON");

    // Write JSON to rustyMani.json
    let mut file = File::create("rustyMani.json")?;
    file.write_all(json_data.as_bytes())?;
    println!("JSON data written to rustyMani.json");

    // Read from the file
    let mut file_read = OpenOptions::new().read(true).open("rustyMani.json")?;
    let mut content = String::new();
    file_read.read_to_string(&mut content)?;

    // Print the contents
    println!("Read from file: \n{}", content);

    Ok(())
}