use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    InvalidInput(String),
    InputStreamFailure(IoError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::InputStreamFailure(err)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;