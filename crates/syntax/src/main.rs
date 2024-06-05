mod ast;
mod lexer;
mod source;

use oxc_allocator::Allocator;

fn main() {
    let input = "func hello() { return 3; }";
    let allocator = Allocator::default();

    let mut lexer = lexer::Lexer::new(input);

    while !lexer.is_at_end() {
        let tok = lexer.next_token();
        let span = tok.span;
        let c = &input[span.start..span.end];
        println!("{:?}: `{}`", tok.kind, c);
    }
}
