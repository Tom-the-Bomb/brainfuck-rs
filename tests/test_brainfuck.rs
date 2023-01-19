#[cfg(test)]
mod tests {
    use brainfuck::{Brainfuck, Result};

    #[test]
    fn test_hello_world_file() -> Result<()> {
        println!("\n\ncells: {:?}", Brainfuck::from_file("tests/hello_world.bf")?
            .execute()?);
        
        Ok(())
    }

    #[test]
    fn test_input() -> Result<()> {
        let code = r#"
        ,+.>,++.
        "#;
        println!("\n\ncells: {:?}", Brainfuck::new(code)
            .execute()?);

        Ok(())
    }

    #[test]
    fn test_sierpinski() -> Result<()> {
        let code = r#"
        ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
            -<<<[
                ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
            ]>.>+[>>]>+
        ]"#;
        println!("\n\ncells: {:?}", Brainfuck::new(code)
            .execute()?);

        Ok(())
    }
}