# Brainfuck-exe

a simple [`brainfuck`](https://esolangs.org/wiki/Brainfuck) interpreter crate implemented in rust 🦀
with many available customizations for flexibility  
For more information visit the documentation [here](https://docs.rs/brainfuck-exe)  

## Example
Below is a basic example on how to use the crate
```rust

use std::fs::File;
// import Result typealias and interpreter struct
use brainfuck_exe::{Result, Brainfuck};

fn main() -> Result<()> {
    // brainfuck code to print "Hello, World!"
    let code = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
    +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.";
    // instantiate a new interpreter instance with the code
    Brainfuck::new(code)
        // optional builder method to write the output into a file not STDOUT
        .with_output(
            File::options()
                .write(true)
                .open("tests/output.txt")
                .unwrap()
        )
        // executes the code
        .execute()?;

    // alternatively use this to retrieve the code from an existing source file
    Brainfuck::from_file("tests/hello_world.bf")?
        .execute()?;

    Ok(())
}
```  

## CLI
You can also use this crate as a CLI program
```bash
# installation
$ cargo install brainfuck-exe
# usage
$ brainfuck --help
$ brainfuck [CODE] [-f FILE] [OPTIONS]
```