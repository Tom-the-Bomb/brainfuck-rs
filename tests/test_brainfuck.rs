#[cfg(test)]
mod tests {
    use brainfuck::{Brainfuck, Result};

    #[test]
    fn test_sierpinski() -> Result<()> {
        let code = r#"
        ++++++++[>+>++++<<-]>++>>+<[-[>>+<<-]+>>]>+[
            -<<<[
                ->[+[-]+>++>>>-<<]<[<]>>++++++[<<+++++>>-]+<<++.[-]<<
            ]>.>+[>>]>+
        ]"#;
        Brainfuck::new(code)
            .execute()?;

        Ok(())
    }
}