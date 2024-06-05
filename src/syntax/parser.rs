use crate::token::Token;
use crate::{lexial::Lexer, token::TokenType};

use super::{ASTNode, NonTerminal, ParsingTable, Symbol};
pub(crate) struct Parser {
    parsing_table: ParsingTable,
    input: Vec<Token>,
    stack: Vec<Symbol>,
    ast_stack: Vec<ASTNode>, // Stack for AST nodes.
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            parsing_table: super::add_rules(),
            input: Lexer::new(input.clone(), false).to_vec(),
            stack: vec![Symbol::NonTerminal(NonTerminal::Program)],
            ast_stack: vec![],
        }
    }

    pub fn parse(&mut self) -> Result<ASTNode, String> {
        while let Some(symbol) = self.stack.pop() {
            match symbol {
                Symbol::NonTerminal(non_terminal) => {
                    let token = self.input.first().ok_or("Unexpected end of input")?.clone();
                    match self
                        .parsing_table
                        .get(&(non_terminal.clone(), token.token.clone()))
                    {
                        Some(production) => {
                            for symbol in production.iter().rev() {
                                self.stack.push(symbol.clone());
                            }
                        }
                        None => {
                            return Err(format!(
                                "No rule for {:?}  with {:?}",
                                &non_terminal, &token
                            ));
                        }
                    }
                }
                Symbol::Token(expected_token) => {
                    if let Some(token) = self.input.first() {
                        if expected_token == token.token {
                            self.input.remove(0); // Consume the token.
                        } else {
                            return Err(format!(
                                "Expected {:?}, found {:?}",
                                expected_token, token
                            ));
                        }
                    } else {
                        return Err("Unexpected end of input".to_string());
                    }
                }
                Symbol::Action(action) => {
                    // Perform actions to generate AST nodes.
                    match action.as_str() {
                        "make_number" => {
                            // if let Some(token) = self.input.first() {
                            //     self.ast_stack.push(AstNode::Number(1));
                            // }
                        }

                        "make_binary_expr" => {
                            // let right = self.ast_stack.pop().ok_or("Missing right operand")?;
                            // let op = self.ast_stack.pop().ok_or("Missing operator")?;
                            // let left = self.ast_stack.pop().ok_or("Missing left operand")?;
                            // if let (AstNode::Number(left_val), AstNode::Number(right_val)) =
                            //     (left, right)
                            // {
                            //     let operator = match op {
                            //         AstNode::Number(op_val) => match op_val {
                            //             1 => "+",
                            //             2 => "*",
                            //             _ => return Err("Unknown operator".to_string()),
                            //         },
                            //         _ => return Err("Expected operator".to_string()),
                            //     };
                            //     // let binary_expr = AstNode::BinaryExpr(
                            //     // Box::new(left),
                            //     // operator.to_string(),
                            //     // Box::new(right),
                            //     // );
                            //     // self.ast_stack.push(binary_expr);
                            // }
                        }
                        // Add other actions as needed.
                        _ => return Err("Unknown action".to_string()),
                    }
                }
            }
        }

        if self.input.first().unwrap().token != TokenType::End {
            return Err("Input not fully consumed".to_string());
        }

        self.ast_stack
            .pop()
            .ok_or("AST generation failed".to_string())
    }
}

