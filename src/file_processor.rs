use std::fs;
use std::io;

#[derive(Debug)]
pub enum MazeFormatError {
    EmptyFileError,
    InvalidDimensionError(String)
}

#[derive(Debug)]
pub enum ProcessFileError {
    FileReadError(String, io::Error),
    MazeGenerationError(MazeFormatError)
}

pub struct Maze;

impl std::fmt::Display for MazeFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MazeFormatError::InvalidDimensionError(s) => write!(f, "Unable to process dimensions {}", s),
            MazeFormatError::EmptyFileError => write!(f, "Provided maze file is empty")
        }
    }
}

impl std::fmt::Display for ProcessFileError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
          ProcessFileError::FileReadError(name, e) => write!(f, "Problem reading {}: {}", name, e),
          ProcessFileError::MazeGenerationError(reason) => write!(f, "Problem processing maze: {}", reason)
      }
  }
}

impl std::error::Error for MazeFormatError {}

impl std::error::Error for ProcessFileError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ProcessFileError::FileReadError(_, e) => Some(e),
            ProcessFileError::MazeGenerationError(e) => Some(e)
        }
    }
}

impl From<MazeFormatError> for ProcessFileError {
   fn from(value: MazeFormatError) -> Self {
       ProcessFileError::MazeGenerationError(value)
   }
}

pub fn process_file(file_path: &str) -> Result<Maze, ProcessFileError> {
    println!("Processing file");

    let file_contents = fs::read_to_string(file_path).map_err(|e| ProcessFileError::FileReadError(file_path.to_owned(), e))?;
    println!("Contents of file were: {}", file_contents);
    let maze = convert_to_maze(&file_contents)?;
    Ok(maze)
}

fn convert_to_maze(input: &str) -> Result<Maze, MazeFormatError> {
    let mut lines = input.lines();

    let dimension_str = lines.next().ok_or(MazeFormatError::EmptyFileError)?;
    let (width, height) = read_dimensions(dimension_str)?;

    println!("Maze Width: {}, Maze Height: {}", width, height);

    Ok(Maze)
}

fn read_dimensions(dimension_str: &str) -> Result<(usize, usize), MazeFormatError> {
    let components: Vec<&str> = dimension_str.split(',').collect();
    if components.len() != 2 {
        return Err(MazeFormatError::InvalidDimensionError(dimension_str.to_owned()));
    }
    let width = components[0].parse::<usize>().or(Err(MazeFormatError::InvalidDimensionError(components[0].to_owned())))?;
    let height = components[1].parse::<usize>().or(Err(MazeFormatError::InvalidDimensionError(components[1].to_owned())))?;
    Ok((width, height))
}
