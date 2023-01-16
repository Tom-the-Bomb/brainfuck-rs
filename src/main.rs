use brainfuck::Brainfuck;

fn main() {
    let code = r#">+.>+."#;
    Brainfuck::new(code)
        .execute()
        .unwrap();
}