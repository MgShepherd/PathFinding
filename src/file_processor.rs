use std::fs;
use std::io;

pub fn process_file(file_path: &str) -> Result<(), io::Error> {
    println!("Processing file");

    let file_contents = fs::read_to_string(file_path)?;
    println!("Contents of file were: {}", file_contents);
    Ok(())
}
