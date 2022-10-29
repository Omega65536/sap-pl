mod formula;
mod lexer;
mod simplifier;

use formula::Formula;
use lexer::Lexer;
use simplifier::simplify;

fn main() {
    let source = "A or (not A and B)".to_string();
    let tokens = Lexer::new(source).lex();
    println!("{:?}", tokens);
}

