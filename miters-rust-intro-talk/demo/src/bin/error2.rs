#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::io::Read;

fn read_file_contents_with_auto_handle(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_contents_with_pattern_matching(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(err) => return Err(Box::new(err)),
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn read_file_contents_with_unwrap(path: &str) -> Result<String, Box<dyn Error>> {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "example.txt";

    match read_file_contents_with_auto_handle(path) {
        Ok(contents) => println!("File contents: \n{}", contents),
        Err(e) => println!("Failed to read the file: {}", e),
    }

    Ok(())
}
