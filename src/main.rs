use file_processor::ProcessFileError;

pub mod file_processor;

fn main() -> Result<(), ProcessFileError> {
    let file_name = "examples/maze.txt";
    file_processor::process_file(file_name)?;
    Ok(())
}
