use std::{
    fmt,
    io::Error as IoError,
};

/// Error enum for brainfuck runtime errors
#[derive(Debug)]
pub enum Error {
    /// returned when the amount of `[` in the code does not equal the amount of `]`
    MismatchedBrackets {
        /// the amount of `[` in the code
        opening: usize,
        /// the amount of `]` in the code
        closing: usize,
    },
    /// propogated from opening or reading files for the brainfuck source code
    /// to be interpreted, in [`crate::Brainfuck::from_file`]
    FileReadError(
        /// the propogated error
        IoError
    ),
    /// propogated from `.` and `,` I/O operations
    IoError(
        /// the propogated error
        IoError
    ),
    /// returned when the amount of instructions executed
    /// reaches the limit of instructions to be executed that is set
    MaxInstructionsExceeded(
        /// the instructions limit that was set
        usize
    ),
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Self {
        Self::IoError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            match self {
                Self::MismatchedBrackets { opening, closing } =>
                    format!("Mismatched brackets; there were {opening} '[' found but only {closing} ']' found"),
                Self::FileReadError(err) =>
                    format!("Failed to read the provided file:\n{err}"),
                Self::IoError(err) =>
                    format!("An I/O error occured:\n{err}"),
                Self::MaxInstructionsExceeded(cap) =>
                    format!("The amount of instructions executed has reached the set limit of `{cap}`"),
            }
            .as_str()
        )
    }
}

/// result type alias for [`Error`]
pub type Result<T, E = Error> = std::result::Result<T, E>;