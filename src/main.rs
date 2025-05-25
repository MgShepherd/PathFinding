pub mod file_processor;

fn main() {
    let file_name = "examples/maze.txt";
    let result = file_processor::process_file(file_name);
    if result.is_err() {
        println!("Error occurred processing file {}: {}", file_name, result.err().unwrap());
    }
}
