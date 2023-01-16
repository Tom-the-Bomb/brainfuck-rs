use error::{Error, Result};
use std::io::Write;

pub mod error;

#[derive(Debug, Clone)]
pub struct Brainfuck {
    pub code: String,
    pub input: String,
    pub use_stdin: bool,
    pub max_cell_value: u32,
    pub memory_size: Option<usize>,
}

impl Default for Brainfuck {
    fn default() -> Self {
        Self::new("")
    }
}

impl Brainfuck {
    #[must_use]
    pub fn new(code: &str) -> Self {
        Self {
            code: code.to_string(),
            input: String::new(),
            use_stdin: false,
            max_cell_value: 255,
            memory_size: None,
        }
    }

    #[must_use]
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = code.to_string();
        self
    }

    #[must_use]
    pub fn with_input(mut self, input: &str) -> Self {
        self.input = input.to_string();
        self
    }

    #[must_use]
    pub const fn with_max_value(mut self, cell_value: u32) -> Self {
        self.max_cell_value = cell_value;
        self
    }

    #[must_use]
    pub const fn use_stdin(mut self) -> Self {
        self.use_stdin = true;
        self
    }

    fn read_from_console() -> Result<char> {
        let mut buffer = String::new();
        std::io::stdin()
            .read_line(&mut buffer)?;
        buffer
            .chars()
            .next()
            .ok_or_else(|| Error::InvalidInput(buffer))
    }

    /// executes the provided brainfuck code
    /// which is stored in the struct field: `code`
    ///
    /// brainfuck supports 8 operands which are as following:
    /// `+ - < > . , [ ]`
    /// different implementations vary on wraparound rules
    ///
    /// - `+`: increments the current cell by `1`
    ///   if the value exceeds `self.max_cell_value`, it gets wrapped back to `0`
    /// - `-`: decrements the current cell by `1`
    ///   if the value goes below `0`, it gets wrapped back to `self.max_cell_value`
    /// - `>`: moves the pointer up 1 cell
    ///   if the the pointer exceeds `self.memory_size`, it gets wrapped back to `0`;
    ///   however, if `self.memory_size` is `None`, it will grow the array by 1 additional cell
    /// - `<`: moves the pointer down 1 cell
    ///   if the value goes below `0`, it gets wrapped back to the end of the memory array
    /// - `.`: writes value of the current cell as ASCII into the stdout
    /// 
    /// # Errors
    /// - [`Error::InvalidInput`]: invalid input read (empty)
    /// - [`Error::InputStreamFailure`]: failed to read from stdin
    pub fn execute(&self) -> Result<()> {
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

        let mut code_idx = 0;
        let mut ptr = 0;
        let mut input_index = 0;

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
                    if let Some(ascii) =
                        std::char::from_u32(cells[ptr])
                    {
                        print!("{ascii}");
                        std::io::stdout()
                            .flush()
                            .ok();
                    }
                Some(',') =>
                    if self.use_stdin {
                        cells[ptr] = Self::read_from_console()? as u32;
                    } else {
                        if let Some(chr) = self.input.chars()
                            .nth(input_index)
                        {
                            cells[ptr] = chr as u32;
                        }
                        input_index += 1;
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
        }

        Ok(())
    }
}