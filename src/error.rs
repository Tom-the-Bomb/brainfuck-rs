use std::io::Error as IoError;

/// error enum for brainfuck runtime errors
#[derive(Debug)]
pub enum Error {
    /// returned when the amount of `[` in the code is unequal to the amount of `]`
    MismatchedBrackets {
        /// the amount of `[` in the code
        opening: usize,
        /// the amount of `]` in the code
        closing: usize,
    },
    IoError(IoError),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

/// result type alias for [`Error`]
pub type Result<T, E = Error> = std::result::Result<T, E>;