use std::fmt::{Display, Formatter};
use std::io::Error as IOError;

#[derive(Debug)]
pub enum ExerciseError {
    IO(IOError),
}

impl Display for ExerciseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExerciseError::IO(parse_int_error) =>
                write!(f, "{}", parse_int_error)
        }
    }
}

impl std::error::Error for ExerciseError {}
