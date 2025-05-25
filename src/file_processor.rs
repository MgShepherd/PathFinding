use std::fs;
use std::io;

#[derive(Debug)]
pub enum ProcessFileError {
    FileReadError(String, io::Error),
    MazeGenerationError
}

impl std::fmt::Display for ProcessFileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          ProcessFileError::FileReadError(name, e) => write!(f, "Problem reading {}: {}", name, e),
          ProcessFileError::MazeGenerationError => write!(f, "Problem generating valid maze from input file")
      }
  }
}

impl std::error::Error for ProcessFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessFileError::FileReadError(_, e) => Some(e),
            ProcessFileError::MazeGenerationError => None
        }
    }
}

pub fn process_file(file_path: &str) -> Result<(), ProcessFileError> {
    println!("Processing file");

    let file_contents = fs::read_to_string(file_path).map_err(|e| ProcessFileError::FileReadError(file_path.to_owned(), e))?;
    println!("Contents of file were: {}", file_contents);
    Ok(())
}
