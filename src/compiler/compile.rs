fn parser(str: &str) -> &str {
    todo!();
}

/// Lexer
/// Make code to token stream
fn lexer(str: &str) -> &str {
    todo!();
}

pub fn compile(str: &str) -> &str {
    let lexer_result = lexer(str);
    return parser(lexer_result);
}
