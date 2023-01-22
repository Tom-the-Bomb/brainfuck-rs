use clap::{CommandFactory, Parser};
use brainfuck_exe::{Result, Brainfuck};

#[derive(Parser, Debug)]
#[command(name = "Brainfuck-rs", author, version, about, arg_required_else_help = true)]
struct Args {
    /// The code of the brainfuck program
    /// this argument is required unless `-f` is specified (file)
    #[arg(value_parser, verbatim_doc_comment)]
    code: Option<String>,
    /// specifies a file to use for the brainfuck program instead
    #[arg(short = 'f', long = "file", action)]
    file: Option<String>,
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
    /// specifies the limit on the amount of instructions we can process in one program
    /// if not provided, there will be no limit
    #[arg(long, action, verbatim_doc_comment)]
    instructions_limit: Option<usize>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut interp =
        if let Some(code) = args.code {
            Brainfuck::new(code)
        } else if let Some(file) = args.file {
            Brainfuck::from_file(file)?
        } else {
            let mut cmd = Args::command();
            if let Err(_) = cmd.print_long_help() {
                println!("Something went wrong when printing the output.")
            }
            return Ok(());
        };

    if let Some(value) = args.max_cell_value {
        interp = interp.with_max_value(value);
    }
    if let Some(size) = args.memory_size {
        interp = interp.with_mem_size(size);
    }
    if let Some(limit) = args.instructions_limit {
        interp = interp.with_instructions_limit(limit);
    }

    interp.execute()?;
    Ok(())
}