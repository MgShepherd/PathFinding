pub mod file_processor;

fn main() {
    let file_name = "examples/maze.txt";
    if let Some(file_error) = file_processor::process_file(file_name).err() {
        println!("Error occurred processing file {}: {}", file_name, file_error);
    }
}
