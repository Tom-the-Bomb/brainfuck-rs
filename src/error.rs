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
    /// propogated from opening or reading files for the brainfuck source code
    /// to be interpreted, in [`crate::Brainfuck::from_file`]
    FileReadError(
        /// propogated error
        IoError
    ),
    /// propogated from `.` and `,` I/O operations
    IoError(
        /// propogated error
        IoError
    ),
    /// returned when the amount of instructions executed
    /// reaches the limit of instructions to be executed that is set
    MaxInstructionsExceeded(
        /// the instructions limit that's set
        usize
    ),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

/// result type alias for [`Error`]
pub type Result<T, E = Error> = std::result::Result<T, E>;