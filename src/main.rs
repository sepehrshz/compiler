
use syntax::parser::{ParseTreeNode, Parser};

pub(crate) mod lexial;
pub(crate) mod syntax;
pub(crate) mod token;

const TEST_IN: &str = include_str!("./../tests/testSim.in");
const GRAMMER: &str = include_str!("./../grammer.g");

fn main() {
    // first::first_and_follow(GRAMMER.to_owned());
    let mut syntax = Parser::new(TEST_IN.to_owned());
    match syntax.parse() {
        Ok(ast) => {
            // ast.iter().for_each(|f| print_tree(&f, "", "", ""))
            print_tree(&ast, "", "", "")
            // println!("{:?}", ast);
        }
        Err(err) => {
            panic!("Syntax error: {}", err);
        }
    }
}

fn print_tree(t: &ParseTreeNode, parent_prefix: &str, immediate_prefix: &str, parent_suffix: &str) {
    // print the line for node t
    let sym = match &t.symbol {
        syntax::Symbol::Token(token) => token.as_ref(),
        syntax::Symbol::NonTerminal(ter) => ter.as_ref(),
        syntax::Symbol::Def => unreachable!(),
    };
    println!("{0}{1}{2}", parent_prefix, immediate_prefix, sym);

    // print all children of t recursively
    let mut it = t.children.iter().peekable();
    let child_prefix = format!("{0}{1}", parent_prefix, parent_suffix);

    while let Some(child) = it.next() {
        match it.peek() {
            None => print_tree(child, &child_prefix, "└─ ", "   "),
            Some(_) => print_tree(child, &child_prefix, "├─ ", "│  "),
        }
    }
}
