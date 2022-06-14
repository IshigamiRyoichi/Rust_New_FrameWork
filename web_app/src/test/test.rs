use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::collections::HashMap;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
struct Count {
    form: i32,
    controller: i32,
}

type Counts = HashMap<String, Count>;

fn read_json() -> HashMap<String, Count> {
    println!("Hello");
    let model_file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("src/test/model_file.json")?;

    let reader = BufReader::new(model_file);
    let model_files : Counts = serde_json::from_reader(reader)?;
    println!("model_file:{:?}", model_files);
    model_files
}

fn main() {
    read_json();
}