//! a simple brainfuck interpreter implemented in rust

use std::{
    fs::File,
    path::Path,
    fmt::Display,
    io::{Read, Write},
};
pub use error::{Error, Result};

pub mod error;

/// default max value a cell can have
/// it is `255`, the same as [`std::u8::MAX`]
pub const DEFAULT_MAX_CELL_VALUE: u32 = 255;


/// struct representing a brainfuck interpreter instance
pub struct Brainfuck {
    /// the brainfuck code to execute
    pub code: String,
    /// the input stream used for `,` operations
    input: Option<Box<dyn Read>>,
    /// the output stream used for `.` operations
    output: Option<Box<dyn Write>>,
    /// sets the maximum value of a cell, defaults to `255`
    pub max_cell_value: u32,
    /// sets the maximum length of the memory array
    /// defaults to [`None`], which is "infinite"
    pub memory_size: Option<usize>,
    /// indicates whether or not to manually flush the output buffer every write
    /// if set to `false` it will let the process automatically flush (end of program or at every newline)
    pub flush_output: bool,
    /// sets the limit on the amount of instructions we can process in one program
    /// defaults to [`None`], which is no limit
    /// (for safety and debugging usage)
    pub instructions_limit: Option<usize>,
    /// an instructions counter to count the number of instructions executed thus far
    instructions_ctn: usize,
}

impl Default for Brainfuck {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl Brainfuck {
    /// creates a new instance of a brainfuck interpeter with the provided `code`
    /// input and output streams default to [`std::io::stdin`] and [`std::io::stdout`] respectively
    /// maximum value a cell can have is `255`
    /// and the memory array length can grow infinitely
    #[must_use]
    pub fn new<S: Display>(code: S) -> Self {
        Self {
            code: code.to_string(),
            input: None,
            output: None,
            max_cell_value: DEFAULT_MAX_CELL_VALUE,
            memory_size: None,
            flush_output: false,
            instructions_limit: None,
            instructions_ctn: 0,
        }
    }

    /// an alternative to `Self::new`,
    /// used when the code is in a source file instead of being directly accessible as a string in the code
    ///
    /// # Errors
    /// - [`Error::FileReadError`]: propogated from [`std::io::Error`]
    ///   when opening or reading the source file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut buf = String::new();
        let mut file = File::open(path)
            .map_err(Error::FileReadError)?;

        file.read_to_string(&mut buf)
            .map_err(Error::FileReadError)?;
        Ok(Self::new(buf))
    }

    /// builder method to specify the brainfuck code for the interpreter
    #[must_use]
    pub fn with_code<S: Display>(mut self, code: S) -> Self {
        self.code = code.to_string();
        self
    }

    /// builder method to specify the input stream for the `,` operation
    #[must_use]
    pub fn with_input<I>(mut self, input: I) -> Self
    where
        I: Read + 'static
    {
        self.input = Some(Box::new(input));
        self
    }

    /// builder method to specify the output s tream for the `.` operation
    #[must_use]
    pub fn with_output<O>(mut self, output: O) -> Self
    where
        O: Write + 'static
    {
        self.output = Some(Box::new(output));
        self
    }

    /// builder method to specify the max value of a cell
    #[must_use]
    pub const fn with_max_value(mut self, cell_value: u32) -> Self {
        self.max_cell_value = cell_value;
        self
    }

    /// builder method to specify the maximum memory array length
    #[must_use]
    pub const fn with_mem_size(mut self, mem_size: usize) -> Self {
        self.memory_size = Some(mem_size);
        self
    }

    /// builder method to indicate to flush the output stream every write
    #[must_use]
    pub const fn flush_manually(mut self) -> Self {
        self.flush_output = true;
        self
    }

    /// builder method to set the maximum amount of instructions we can process in one program
    #[must_use]
    pub const fn with_instructions_limit(mut self, limit: usize) -> Self {
        self.instructions_limit = Some(limit);
        self
    }

    /// a getter that returns the number of instructions executed thus far
    #[must_use]
    pub const fn instructions_count(&self) -> usize {
        self.instructions_ctn
    }

    /// helper method to read from [`std::io::stdin`]
    /// as a fallback to if no other input stream is specified for the `,` operation
    #[must_use]
    fn read_from_stdin() -> u32 {
        let mut buffer = [0];
        match std::io::stdin()
            .read_exact(&mut buffer[0..1])
        {
            Ok(_) => u32::from(buffer[0]),
            Err(_) => 0,
        }
    }

    /// executes the provided brainfuck code
    /// which is stored in the struct field: `code`
    ///
    /// brainfuck supports 8 operands which are as following:
    /// `+ - < > . , [ ]`
    /// different implementations vary on wraparound rules
    ///
    /// # Operations
    /// - `+`: increments the current cell by `1`
    ///   if the value exceeds `self.max_cell_value`, it gets wrapped back to `0`
    /// - `-`: decrements the current cell by `1`
    ///   if the value goes below `0`, it gets wrapped back to `self.max_cell_value`
    /// - `>`: moves the pointer up 1 cell
    ///   if the the pointer exceeds `self.memory_size`, it gets wrapped back to `0`;
    ///   however, if `self.memory_size` is [`None`], it will grow the array by 1 additional cell
    /// - `<`: moves the pointer down 1 cell
    ///   if the value goes below `0`, it gets wrapped back to the end of the memory array
    /// - `.`: writes the value of the current cell as ASCII into the provided output stream, `self.output`
    ///   defaulting to [`std::io::stdout`]
    /// - `,`: reads 1 byte from the provided input stream, `self.input`
    ///   defaulting to [`std::io::stdin`]
    ///   if reading fails (e.g. there were no bytes to read) the current cell gets set back to `0`
    /// - `[`: always should be paired with a `]`, acts as a "loop" in brainfuck
    ///   the code that is enclosed within a pair of `[ ]` gets looped over until the current cell != 0
    /// - `]`: the closing bracket for a loop, paired with `[`
    ///   if current cell != jump back to corresponding `[`
    ///
    /// returns the used memory array of the program, ([`Vec<u32>`])
    ///
    /// # Errors
    /// - [`Error::MismatchedBrackets`]: the amount of `[` in the code is unequal to the amount of `]`
    /// - [`Error::IoError`]: Propogated from [`std::io::Error`] in the `.` operation
    ///
    #[allow(clippy::too_many_lines)]
    pub fn execute(&mut self) -> Result<Vec<u32>> {
        let (opening, closing) = (
            self.code.chars()
                .filter(|c| c == &'[')
                .count(),
            self.code.chars()
                .filter(|c| c == &']')
                .count()
        );

        if opening != closing {
            return Err(Error::MismatchedBrackets {
                opening, closing
            });
        }

        let mut cells =
            self.memory_size
                .map_or_else(
                    || vec![0],
                    |mem_size| vec![0; mem_size],
                );

        self.instructions_ctn = 0;
        let mut code_idx = 0usize;
        let mut ptr = 0usize;

        while code_idx < self.code
            .chars()
            .count()
        {
            match self.code
                .chars()
                .nth(code_idx)
            {
                Some('+') =>
                    if cells[ptr] >= self.max_cell_value {
                        cells[ptr] = 0;
                    } else {
                        cells[ptr] += 1;
                    },
                Some('-') =>
                    if cells[ptr] == 0 {
                        cells[ptr] = self.max_cell_value;
                    } else {
                        cells[ptr] -= 1;
                    },
                Some('<') =>
                    if ptr == 0 {
                        ptr = cells.len() - 1;
                    } else {
                        ptr -= 1;
                    },
                Some('>') => {
                    ptr += 1;
                    if let Some(mem_size) = self.memory_size {
                        if ptr >= mem_size {
                            ptr = 0;
                        }
                    } else if ptr >= cells.len() {
                        cells.push(0);
                    }
                },
                Some('.') =>
                    if let Some(chr) =
                        std::char::from_u32(cells[ptr])
                    {
                        if let Some(ref mut writer) =
                            self.output
                        {
                            let mut buf = vec![0; chr.len_utf8()];
                            chr.encode_utf8(&mut buf);

                            writer.write_all(&buf)?;
                            if self.flush_output {
                                writer.flush()?;
                            }
                        } else {
                            print!("{chr}");
                            if self.flush_output {
                                std::io::stdout()
                                    .flush()?;
                            }
                        }
                    },
                #[allow(clippy::option_if_let_else)]
                Some(',') =>
                    cells[ptr] = if let Some(ref mut reader) =
                        self.input
                    {
                        let mut buffer = [0];
                        match reader
                            .read_exact(&mut buffer[0..1])
                        {
                            Ok(_) => u32::from(buffer[0]),
                            Err(_) => 0,
                        }
                    } else {
                        Self::read_from_stdin()
                    },
                Some('[') =>
                    if cells[ptr] == 0 {
                        let mut loop_ = 1;
                        while loop_ > 0 {
                            code_idx += 1;
                            match self.code
                                .chars()
                                .nth(code_idx)
                            {
                                Some('[') => loop_ += 1,
                                Some(']') => loop_ -= 1,
                                _ => (),
                            }
                        }
                    },
                Some(']') => {
                    let mut loop_ = 1;
                    while loop_ > 0 {
                        code_idx -= 1;
                        match self.code
                            .chars()
                            .nth(code_idx)
                        {
                            Some('[') => loop_ -= 1,
                            Some(']') => loop_ += 1,
                            _ => (),
                        }
                    }
                    code_idx -= 1;
                },
                _ => (),
            }
            code_idx += 1;
            self.instructions_ctn += 1;

            if let Some(cap) = self.instructions_limit {
                if self.instructions_ctn > cap {
                    return Err(Error::MaxInstructionsExceeded(cap));
                }
            }
        }

        Ok(cells)
    }
}