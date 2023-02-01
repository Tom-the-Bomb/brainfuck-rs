//! CLI implementation for the brainfuck interpreter
//! powered by [`clap`]
//!
//! requires the `cli` feature which is enabled by default

use std::{fs::File, io::Cursor};
use clap::{CommandFactory, Parser};
use brainfuck_exe::Brainfuck;

#[derive(Parser, Debug)]
#[command(name = "Brainfuck-exe", author, version, about, arg_required_else_help = true)]
struct Args {
    /// The code of the brainfuck program
    /// this argument is required unless [-f] [--file] is specified (file)
    #[arg(value_parser, verbatim_doc_comment)]
    code: Option<String>,
    /// specifies a file to use for the brainfuck program instead
    #[arg(short = 'f', long = "file", action)]
    file: Option<String>,
    /// manually enters the inputs (used in `,`) for the brainfuck program instead of STDIN
    #[arg(short = 'i', long, action)]
    input: Option<String>,
    /// specifies a file to write the program output to instead of STDOUT
    #[arg(short = 'o', long, action)]
    output: Option<String>,
    /// specifies the maximum value a cell can have
    /// defaults to 255 (8 bits / 1 byte)
    #[arg(long, action, verbatim_doc_comment)]
    max_cell_value: Option<u32>,
    /// specifies a set size for the memory array of the brainfuck program
    /// if not set, the array is growable and has no set size
    #[arg(long, action, verbatim_doc_comment)]
    memory_size: Option<usize>,
    /// specifies whether or not to manually flush the output buffer every write
    /// if not set it will let the process automatically flush (end of program or at every newline)
    #[arg(long, action, verbatim_doc_comment)]
    flush_output: bool,
    /// specifies whether or not to prompt the STDIN once at the beginning for all the input data
    /// or instead get 1 character every time it is needed
    #[arg(long, action, verbatim_doc_comment)]
    prompt_stdin_once: bool,
    /// specifies the limit on the amount of instructions we can process in one program
    /// if not provided, there will be no limit
    #[arg(long, action, verbatim_doc_comment)]
    instructions_limit: Option<usize>,
    /// specifies whether or not to print the memory array after execution
    #[arg(long, action)]
    print_cells: bool,
}

#[allow(clippy::option_if_let_else, clippy::single_match_else)]
fn main() {
    let args = Args::parse();

    let mut interp =
        if let Some(code) = args.code {
            Brainfuck::new(code)
        } else if let Some(file) = args.file {
            match Brainfuck::from_file(&file) {
                Ok(interp) => interp,
                Err(_) => {
                    println!("Could not open the provided file: {file}");
                    std::process::exit(1);
                }
            }
        } else {
            let mut cmd = Args::command();
            if cmd.print_long_help().is_err() {
                println!("Something went wrong when printing the output.");
                std::process::exit(1);
            }
            std::process::exit(0);
        }
        .with_flush(args.flush_output)
        .prompt_stdin_once(args.prompt_stdin_once);

    if let Some(input) = args.input {
        let bytes = Cursor::new(
            input.into_bytes()
        );
        interp = interp.with_input(bytes);
    }

    if let Some(path) = args.output {
        if let Ok(file) = File::create(&path) {
            interp = interp.with_output(file);
        } else {
            println!("Failed to open the provided file: {path}");
            std::process::exit(1);
        }
    }

    if let Some(value) = args.max_cell_value {
        interp = interp.with_max_value(value);
    }
    if let Some(size) = args.memory_size {
        interp = interp.with_mem_size(size);
    }
    if let Some(limit) = args.instructions_limit {
        interp = interp.with_instructions_limit(limit);
    }

    match interp.execute() {
        Ok(cells) => if args.print_cells {
            println!("\n\nCELLS: {cells:?}");
        },
        Err(e) => println!(
            "Something went wrong: {e}"
        ),
    }
}