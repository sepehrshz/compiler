use syntax::parser::Parser;

pub(crate) mod lexial;
pub(crate) mod syntax;
pub(crate) mod token;

const TEST_IN: &str = include_str!("./../tests/testSim.in");

fn main() {
    let mut syntax = Parser::new(TEST_IN.to_owned());
    match syntax.parse() {
        Ok(ast) => {
            println!("{}", ast);
        }
        Err(err) => {
            panic!("Syntax error: {}", err);
        }
    }
}
