#[cfg(test)]
mod tests {
    use std::{
        fs::File,
        io::Cursor,
    };
    use brainfuck_exe::{Brainfuck, Result};

    #[test]
    fn test_hello_world_file() -> Result<()> {
        println!();
        Brainfuck::from_file("tests/hello_world.bf")?
            .execute()?;

        println!();
        Ok(())
    }

    #[test]
    fn test_file_input() -> Result<()> {
        println!();
        let file = File::open("tests/input.txt")
            .unwrap();
        let code = r#"
        ,+.>,+.>,+.
        "#;
        Brainfuck::new(code)
            .with_input(file)
            .execute()?;

        println!();
        Ok(())
    }

    #[test]
    fn test_sierpinski() -> Result<()> {
        println!();
        let code = r#"
        ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
            -<<<[
                ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
            ]>.>+[>>]>+
        ]"#;
        Brainfuck::new(code)
            .execute()?;

        println!();
        Ok(())
    }

    #[test]
    fn test_file_output() -> Result<()> {
        println!();
        let file = File::options()
            .write(true)
            .open("tests/output.txt")
            .unwrap();
        let code = r#"
        >++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<+
        +.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-
        ]<+.
        "#;
        Brainfuck::new(code)
            .with_output(file)
            .execute()?;

        println!();
        Ok(())
    }

    #[test]
    fn test_cursor() -> Result<()> {
        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);

        let _interp = Brainfuck::new("-.")
            .with_output_ref(&mut cursor)
            .execute()?;

        println!("{}",
            String::from_utf8(cursor.into_inner())
                .unwrap()
        );

        Ok(())
    }
}