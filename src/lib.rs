//! # Brainfuck-exe
//!
//! a simple [`brainfuck`](https://esolangs.org/wiki/Brainfuck) interpreter crate implemented in rust 🦀
//! with many available customizations for flexibility
//!
//! see the [`Brainfuck`] struct for more information on usage
//!
//! ## Usage
//!
//! In your `Cargo.toml`:
//! ```toml
//! brainfuck-exe = "*"
//! ```
//!
//! If you are only using it as a library, and the CLI is not needed,
//! disable the `cli` (included by default) feature to remove unecessary dependencies:
//! ```toml
//! brainfuck-exe = { version = "*", default-features = false }
//! ```
//!
//! ## Example
//! Below is a basic example on how to use the crate
//!
//! ```rust
//!
//! use std::fs::File;
//! // import Result typealias and interpreter struct
//! use brainfuck_exe::{Result, Brainfuck};
//!
//! fn main() -> Result<()> {
//!     // brainfuck code to print "Hello, World!"
//!     let code = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
//!     +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
//!     // instantiate a new interpreter instance with the code
//!     Brainfuck::new(code)
//!         // optional builder method to write the output into a file not STDOUT
//!         .with_output(
//!             File::options()
//!                 .write(true)
//!                 .open("tests/output.txt")
//!                 .unwrap()
//!         )
//!         // executes the code
//!         .execute()?;
//!
//!     // alternatively use this to retrieve the code from an existing source file
//!     Brainfuck::from_file("tests/hello_world.bf")?
//!         .execute()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## CLI
//! You can also use this crate as a CLI program
//!
//! ```bash
//! # installation
//! $ cargo install brainfuck-exe
//! # usage
//! $ brainfuck --help
//! $ brainfuck [CODE] [-f FILE] [OPTIONS]
//! ```

use std::{
    fs::File,
    path::Path,
    io::{Read, Write},
    ops::{Deref, DerefMut},
    time::{Instant, Duration},
};
pub use error::{Error, Result};

pub mod error;

/// default max value a cell can have
///
/// it is `255`, the same as [`std::u8::MAX`]
pub const DEFAULT_MAX_CELL_VALUE: u32 = 255;


/// a helper wrapper enum that is used for storing the input stream
/// this allows for it to be passed by value OR reference
pub enum Reader<'a> {
    /// used when passing in the input stream by value
    Value(Box<dyn Read>),
    /// used when passing in the input stream as a mutable reference
    Ref(&'a mut dyn Read),
}

/// a helper wrapper enum that is used for storing the output stream
/// this allows for it to be passed by value OR reference
pub enum Writer<'a> {
    /// used when passing in the output stream by value
    Value(Box<dyn Write>),
    /// used when passing in the output stream as a mutable reference
    Ref(&'a mut dyn Write),
}

impl<'a> Deref for Reader<'a> {
    type Target = dyn Read + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Value(v) => &**v,
            Self::Ref(r) => &**r,
        }
    }
}

impl<'a> DerefMut for Reader<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Value(v) => &mut **v,
            Self::Ref(r) => &mut **r,
        }
    }
}

impl<'a> Deref for Writer<'a> {
    type Target = dyn Write + 'a;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Value(v) => &**v,
            Self::Ref(r) => &**r,
        }
    }
}

impl<'a> DerefMut for Writer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Value(v) => &mut **v,
            Self::Ref(r) => &mut **r,
        }
    }
}

/// struct containing various information regarding the program execution
/// such as the final memory array and the final pointer index etc.
#[derive(Debug, Clone)]
pub struct ExecutionInfo {
    /// the final memory array (cells) of the brainfuck program
    pub cells: Vec<u32>,
    /// the size of the final memory array of the brainfuck program
    pub mem_size: usize,
    /// the final pointer index
    pub pointer: usize,
    /// the length of the brainfuck code
    pub code_len: usize,
    /// the amount of instructions execute
    ///
    /// this also can be retrieved with `Brainfuck::instructions_count`
    pub instructions: usize,
    /// the time it took for the program execution as a [`Duration`]
    ///
    /// it is [`None`] if it was not specified in [`Brainfuck`] to `bench_execution`
    pub time: Option<Duration>,
}

/// The struct representing a brainfuck interpreter instance
pub struct Brainfuck<'a> {
    /// the brainfuck source code to execute
    pub code: String,
    /// the input stream used for `,` operations
    pub input: Option<Reader<'a>>,
    /// the output stream used for `.` operations
    pub output: Option<Writer<'a>>,
    /// sets the maximum value of a cell, defaults to `255`
    pub max_cell_value: u32,
    /// sets the maximum length of the memory array
    ///
    /// defaults to [`None`], which is "infinite"
    pub memory_size: Option<usize>,
    /// indicates whether or not to manually flush the output buffer every write
    ///
    /// if set to `false` it will let the process automatically flush (end of program or at every newline),
    /// defaults to `true`
    pub flush_output: bool,
    /// this field is only of use if the input stream used is [`std::io::stdin`]
    ///
    /// it specifies whether or not to retrieve all the input data needed in one prompt the first time
    /// or rather prompt the user every time for a character,
    /// defaults to `false`
    pub prompt_stdin_once: bool,
    /// sets the limit on the amount of instructions we can process in one program
    ///
    /// defaults to [`None`], which is *no* limit
    /// (for safety and debugging usage)
    pub instructions_limit: Option<usize>,
    /// specifies whether or not to bench the execution
    ///
    /// useful for use cases in `WASM` where the system clock cannot be accessed,
    /// defaults to `true`
    pub bench_execution: bool,
    /// an optional fallback [`char`] for the input operation
    /// in instances of EOL (end of input) on the input stream
    pub fallback_input: Option<char>,
    /// an instructions counter to count the number of instructions executed thus far
    instructions_ctn: usize,
}

impl<'a> Default for Brainfuck<'a> {
    fn default() -> Self {
        Self::new(String::new())
    }
}

impl<'a> Brainfuck<'a> {
    /// creates a new instance of a brainfuck interpeter with the provided `code`
    ///
    /// - input and output streams default to [`std::io::stdin`] and [`std::io::stdout`] respectively
    /// - the maximum value a cell can have is `255` (8 bits / 1 byte)
    /// - the program's memory array can grow indefinitely
    #[must_use]
    pub fn new<S: AsRef<str>>(code: S) -> Self {
        Self {
            code: code
                .as_ref()
                .to_string(),
            input: None,
            output: None,
            max_cell_value: DEFAULT_MAX_CELL_VALUE,
            memory_size: None,
            flush_output: true,
            prompt_stdin_once: false,
            instructions_limit: None,
            bench_execution: true,
            fallback_input: None,
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
    pub fn with_code<S: AsRef<str>>(mut self, code: S) -> Self {
        self.code = code
            .as_ref()
            .to_string();
        self
    }

    /// builder method to specify the input stream **passing by value**, for the `,` operation
    #[must_use]
    pub fn with_input<I>(mut self, input: I) -> Self
    where
        I: Read + 'static
    {
        self.input = Some(
            Reader::Value(Box::new(input))
        );
        self
    }

    /// builder method to specify the output stream **passing by value**, for the `.` operation
    #[must_use]
    pub fn with_output<O>(mut self, output: O) -> Self
    where
        O: Write + 'static
    {
        self.output = Some(
            Writer::Value(Box::new(output))
        );
        self
    }

    /// builder method to specify the input stream **passing by reference**, for the `,` operation
    #[must_use]
    pub fn with_input_ref<I>(mut self, input: &'a mut I) -> Self
    where
        I: Read + 'static
    {
        self.input = Some(
            Reader::Ref(input)
        );
        self
    }

    /// builder method to specify the output stream **passing by reference**, for the `.` operation
    #[must_use]
    pub fn with_output_ref<O>(mut self, output: &'a mut O) -> Self
    where
        O: Write + 'static
    {
        self.output = Some(
            Writer::Ref(output)
        );
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

    /// builder method to indicate whether or not to flush the output stream on every write
    #[must_use]
    pub const fn with_flush(mut self, flush: bool) -> Self {
        self.flush_output = flush;
        self
    }

    /// builder method to indicate whether or not to only prompt [`std::io::stdin`] once
    #[must_use]
    pub const fn prompt_stdin_once(mut self, once: bool) -> Self {
        self.prompt_stdin_once = once;
        self
    }

    /// builder method to set the maximum amount of instructions we can process in one program
    #[must_use]
    pub const fn with_instructions_limit(mut self, limit: usize) -> Self {
        self.instructions_limit = Some(limit);
        self
    }

    /// builder method to specify whether or not to bench the program execution
    #[must_use]
    pub const fn with_bench_execution(mut self, bench: bool) -> Self {
        self.bench_execution = bench;
        self
    }
    /// builder method to set a fallback [`char`] for instances of EOL on the input stream
    #[must_use]
    pub const fn with_fallback_input(mut self, fallback: char) -> Self {
        self.fallback_input = Some(fallback);
        self
    }

    /// a getter that returns the number of instructions executed thus far
    #[must_use]
    pub const fn instructions_count(&self) -> usize {
        self.instructions_ctn
    }

    /// consumes itself and returns the input stream in an [`Option`]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_input(self) -> Option<Reader<'a>> {
        self.input
    }

    /// consumes itself and returns the output stream in an [`Option`]
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn into_output(self) -> Option<Writer<'a>> {
        self.output
    }

    /// basic helper function to retrieve the fallback char for the input stream
    #[inline]
    fn get_fallback_char(&self) -> u32 {
        self.fallback_input
            .map_or(0, u32::from)
    }

    /// helper method to read from [`std::io::stdin`]
    ///
    /// it accomplishes such in one prompt, retrieving all the data at once
    /// as a fallback to if no other input stream is specified for the `,` operation
    #[must_use]
    fn read_from_stdin_once(&self) -> u32 {
        let mut buffer = [0];
        match std::io::stdin()
            .read_exact(&mut buffer[0..1])
        {
            Ok(_) => u32::from(buffer[0]),
            Err(_) => self.get_fallback_char(),
        }
    }

    /// helper method to read from [`std::io::stdin`]
    ///
    /// it prompts every time this function is called however
    /// as a fallback to if no other input stream is specified for the `,` operation
    #[must_use]
    fn read_from_stdin(&self) -> u32 {
        let mut buffer = String::new();
        match std::io::stdin()
            .read_line(&mut buffer)
        {
            Ok(_) => buffer
                .chars()
                .next()
                .map_or_else(
                    || self.get_fallback_char(),
                    u32::from,
                ),
            Err(_) => self.get_fallback_char(),
        }
    }

    /// executes the provided brainfuck code
    /// which is stored in the struct field: `code`
    ///
    /// brainfuck supports 8 operations which are as following:
    /// `+ - < > . , [ ]`
    ///
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
    ///   if reading fails (e.g. there were no bytes to read (EOF) or other error), the current cell gets set back to `0`
    /// - `[`: always should be paired with a `]`, acts as a "loop" in brainfuck
    ///   the code that is enclosed within a pair of `[ ]` gets looped over until the current cell != 0
    /// - `]`: the closing bracket for a loop, paired with `[`
    ///   if the current cell != 0, jump back to corresponding `[`
    ///
    /// returns [`ExecutionInfo`]: a struct containing various information on the program's execution
    /// such as the used memory array, the final pointer, instructions count etc.
    ///
    /// # Errors
    /// - [`Error::MismatchedBrackets`]: the amount of `[` in the code does not equal the amount of `]`
    /// - [`Error::IoError`]: Propogated from [`std::io::Error`] in the `.` operation
    ///
    #[allow(clippy::too_many_lines)]
    pub fn execute(&mut self) -> Result<ExecutionInfo> {
        let (opening, closing) = (
            self.code.chars()
                .filter(|c| *c == '[')
                .count(),
            self.code.chars()
                .filter(|c| *c == ']')
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
        let time = self.bench_execution
            .then(Instant::now);

        while code_idx < self.code
            .chars()
            .count()
        {
            let mut incr_inst = true;

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
                            Err(_) => self.get_fallback_char(),
                        }
                    } else if self.prompt_stdin_once {
                        self.read_from_stdin_once()
                    } else {
                        self.read_from_stdin()
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
                _ => incr_inst = false,
            }
            code_idx += 1;

            if incr_inst {
                self.instructions_ctn += 1;
            }

            if let Some(cap) = self.instructions_limit {
                if self.instructions_ctn > cap {
                    return Err(Error::MaxInstructionsExceeded(cap));
                }
            }
        }
        let mem_size = cells.len();

        Ok(ExecutionInfo {
            cells,
            mem_size,
            pointer: ptr,
            code_len: code_idx,
            instructions: self.instructions_count(),
            time: time
                .map(|t| t.elapsed()),
        })
    }
}