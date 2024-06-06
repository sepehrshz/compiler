use std::collections::HashSet;

use crate::token::Token;
use crate::{lexial::Lexer, token::TokenType};

use super::{NonTerminal, ParsingTable, Symbol};

#[derive(Debug)]
pub struct ParseTreeNode {
    pub symbol: Symbol,
    pub children: Vec<ParseTreeNode>,
}

pub(crate) struct Parser {
    parsing_table: ParsingTable,
    input: Vec<Token>,
    stack: Vec<Symbol>,
    // ast_stack: Vec<AST>, // Stack for AST nodes.
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            parsing_table: super::add_rules(),
            input: Lexer::new(input.clone(), false).to_vec(),
            stack: vec![Symbol::NonTerminal(NonTerminal::Program)],
            // ast_stack: vec![AST::new((NonTerminal::Program).as_ref().to_owned())],
        }
    }

    pub fn parse(&mut self) -> Result<ParseTreeNode, String> {
        let mut root = ParseTreeNode {
            symbol: Symbol::NonTerminal(NonTerminal::Program),
            children: vec![],
        };

        while let Some(symbol) = self.stack.pop() {
            match symbol {
                Symbol::NonTerminal(non_terminal) => {
                    let token = self.input.first().ok_or("Unexpected end of input")?.clone();
                    match self
                        .parsing_table
                        .get(&(non_terminal.clone(), token.token.clone()))
                    {
                        Some(production) => {
                            if production.is_empty() {
                                continue;
                            }

                            // let mut ast_node = self.ast_stack.pop().unwrap();

                            let mut node = ParseTreeNode {
                                symbol: Symbol::NonTerminal(non_terminal),
                                children: vec![],
                            };

                            for symbol in production.iter().rev() {
                                self.stack.push(symbol.clone());

                                node.children.push(ParseTreeNode {
                                    symbol: symbol.clone(),
                                    children: vec![],
                                });
                            }
                            root.children.push(node)
                        }
                        None => {
                            println!("No rule for {:?}  with {:?}", &non_terminal, &token);
                            self.handel_err();
                        }
                    }
                }
                Symbol::Token(expected_token) => {
                    if let Some(token) = self.input.first() {
                        if expected_token == token.token {
                            // self.ast_stack.pop();
                            self.input.remove(0); // Consume the token.
                        } else {
                            println!("Expected {:?}, found {:?}", expected_token, token);
                            self.handel_err();
                        }
                    } else {
                        return Err("Unexpected end of input".to_string());
                    }
                }
                _ => panic!("how the fuck "),
            }
        }

        if self.input.first().unwrap().token != TokenType::End {
            return Err("Input not fully consumed".to_string());
        }

        Ok(root)
    }
    fn is_synchronization_token(&self, token: &TokenType) -> bool {
        [TokenType::T_Semicolon , TokenType::T_RP].contains(&token)
    }

    fn handel_err(&mut self) {
        while let Some(Symbol::Token(token)) = self.stack.pop() {
            if self.is_synchronization_token(&token) {
                break;
            }
        }

        // Skip input tokens until a synchronization token is found
        while let Some(next_token) = self.input.iter().peekable().peek() {
            if self.is_synchronization_token(&next_token.token) {
                break;
            }
            self.input.remove(0);
        }
    }
}
