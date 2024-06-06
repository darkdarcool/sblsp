mod ast;
mod lexer;
mod source;

use oxc_allocator::Allocator;

const INPUT: &str = include_str!("../../../test.sn");

fn main() {
    let input = "\n";
    let allocator = Allocator::default();

    let mut lexer = lexer::Lexer::new(INPUT);

    while !lexer.is_at_end() {
        let tok = lexer.next_token();
        let span = tok.span;
        let c = &INPUT[span.start..span.end];
        println!("{:?}: `{}`", tok.kind, c);
    }
}
