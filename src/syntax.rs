use crate::token::{Token, TokenType};

#[derive(Debug)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Function {
        return_type: String,
        name: String,
        params: Vec<(String, String)>,
        body: Vec<ASTNode>,
    },
    VariableDeclaration {
        var_type: String,
        name: String,
        value: Option<String>,
    },
    PrintStatement {
        value: String,
    },
    IfStatement {
        condition: Box<ASTNode>,
        then_branch: Vec<ASTNode>,
        else_branch: Option<Vec<ASTNode>>,
    },
    Loop {
        initialization: Option<Box<ASTNode>>,
        condition: Option<Box<ASTNode>>,
        increment: Option<Box<ASTNode>>,
        body: Vec<ASTNode>,
    },
    BinaryOperation {
        operator: TokenType,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    UnaryOperation {
        operator: TokenType,
        operand: Box<ASTNode>,
    },
    Literal(String),
    Identifier(String),
    Expression(Vec<String>), // Added for IfStatement condition parsing
    FunctionCall {
        name: String,
        arguments: Vec<ASTNode>,
    },
}

struct TokenIterator<'a> {
    tokens: &'a [Token],
    position: usize,
}

impl<'a> TokenIterator<'a> {
    fn new(tokens: &'a [Token]) -> Self {
        TokenIterator {
            tokens,
            position: 0,
        }
    }

    fn next(&mut self) -> Option<&'a Token> {
        while self.position < self.tokens.len() {
            let token = &self.tokens[self.position];
            self.position += 1;
            if token.token != TokenType::T_Comment {
                return Some(token);
            }
        }
        None
    }

    fn peek(&self) -> Option<&'a Token> {
        let mut position = self.position;
        while position < self.tokens.len() {
            let token = &self.tokens[position];
            if token.token != TokenType::T_Comment {
                return Some(token);
            }
            position += 1;
        }
        None
    }
}

pub fn parse_program(tokens: &[Token]) -> Result<ASTNode, String> {
    let mut iter = TokenIterator::new(tokens);
    let mut nodes = Vec::new();
    let mut line_number = 1;

    while let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Int => {
                nodes.push(parse_function(&mut iter, line_number)?);
            }
            TokenType::T_Newline => {
                iter.next();
                line_number += 1;
            }
            _ => {
                return Err(format!(
                    "Unexpected token '{}' at line {}",
                    token.literal, line_number
                ));
            }
        }
    }

    Ok(ASTNode::Program(nodes))
}

fn parse_function(iter: &mut TokenIterator, mut line_number: usize) -> Result<ASTNode, String> {
    line_number += 1;
    let return_type = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err(format!("Expected return type at line {}", line_number));
    };

    let name = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err(format!("Expected function name at line {}", line_number));
    };

    let mut params = Vec::new();
    if let Some(Token {
        token: TokenType::T_LP,
        ..
    }) = iter.next()
    {
        while let Some(token) = iter.next() {
            match token.token {
                TokenType::T_Int => {
                    let param_type = token.literal.clone();
                    if let Some(token) = iter.next() {
                        let param_name = token.literal.clone();
                        params.push((param_type, param_name));
                    } else {
                        return Err(format!("Expected parameter name at line {}", line_number));
                    }
                }
                TokenType::T_RP => break,
                _ => {
                    return Err(format!(
                        "Unexpected token '{}' in parameters at line {}",
                        token.literal, line_number
                    ))
                }
            }
        }
    } else {
        return Err(format!(
            "Expected '(' after function name at line {}",
            line_number
        ));
    }

    let mut body = Vec::new();
    if let Some(Token {
        token: TokenType::T_LC,
        ..
    }) = iter.next()
    {
        while let Some(token) = iter.peek() {
            if token.token == TokenType::T_RC {
                iter.next(); // Consume the closing brace
                break;
            } else {
                body.push(parse_statement(iter, line_number)?);
            }
        }
    } else {
        return Err(format!(
            "Expected '{{' to start function body at line {}",
            line_number
        ));
    }

    Ok(ASTNode::Function {
        return_type,
        name,
        params,
        body,
    })
}

fn parse_statement(iter: &mut TokenIterator, mut line_number: usize) -> Result<ASTNode, String> {
    line_number += 1;
    if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Id => parse_variable_declaration(iter, line_number),
            TokenType::T_Int => parse_variable_declaration(iter, line_number),
            TokenType::T_Print => parse_print_statement(iter, line_number),
            TokenType::T_If => parse_if_statement(iter, line_number),
            TokenType::T_For => parse_loop(iter, line_number),
            TokenType::T_Semicolon => {
                iter.next(); // Consume the ';' token
                Ok(ASTNode::Literal(";".to_string()))
            }
            _ => Err(format!(
                "Unexpected token '{}' in statement at line {}",
                token.literal, line_number
            )),
        }
    } else {
        Err(format!("Unexpected end of input at line {}", line_number))
    }
}

fn parse_variable_declaration(
    iter: &mut TokenIterator,
    mut line_number: usize,
) -> Result<ASTNode, String> {
    line_number += 1;
    let var_type = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err(format!("Expected variable type at line {}", line_number));
    };

    let name = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err(format!("Expected variable name at line {}", line_number));
    };

    let value = if let Some(Token {
        token: TokenType::T_Assign,
        ..
    }) = iter.peek()
    {
        iter.next(); // Consume the '=' token
        if let Some(token) = iter.next() {
            Some(token.literal.clone())
        } else {
            return Err(format!("Expected value after '=' at line {}", line_number));
        }
    } else {
        None
    };

    Ok(ASTNode::VariableDeclaration {
        var_type,
        name,
        value,
    })
}

fn parse_print_statement(
    iter: &mut TokenIterator,
    mut line_number: usize,
) -> Result<ASTNode, String> {
    line_number += 1;
    if let Some(Token {
        token: TokenType::T_LP,
        ..
    }) = iter.next()
    {
        if let Some(token) = iter.next() {
            if let TokenType::T_String | TokenType::T_Id | TokenType::T_Decimal = token.token {
                let value = token.literal.clone();
                if let Some(Token {
                    token: TokenType::T_RP,
                    ..
                }) = iter.next()
                {
                    return Ok(ASTNode::PrintStatement { value });
                } else {
                    return Err(format!(
                        "Expected ')' after print statement value at line {}",
                        line_number
                    ));
                }
            } else {
                return Err(format!(
                    "Invalid print statement value at line {}",
                    line_number
                ));
            }
        } else {
            return Err(format!(
                "Expected value after '(' in print statement at line {}",
                line_number
            ));
        }
    } else {
        return Err(format!(
            "Expected '(' in print statement at line {}",
            line_number
        ));
    }
}

fn parse_if_statement(iter: &mut TokenIterator, mut line_number: usize) -> Result<ASTNode, String> {
    line_number += 1;
    // Consume the 'if' token
    iter.next();

    // Parse the condition
    if let Some(Token {
        token: TokenType::T_LP,
        ..
    }) = iter.next()
    {
        let condition = parse_expression(iter, line_number)?;

        if let Some(Token {
            token: TokenType::T_RP,
            ..
        }) = iter.next()
        {
            // Parse the 'then' branch
            if let Some(Token {
                token: TokenType::T_LC,
                ..
            }) = iter.next()
            {
                let mut then_branch = Vec::new();
                while let Some(token) = iter.peek() {
                    if token.token == TokenType::T_RC {
                        iter.next(); // Consume the '}'
                        break;
                    } else {
                        then_branch.push(parse_statement(iter, line_number)?);
                    }
                }

                // Check for an 'else' branch
                let else_branch = if let Some(Token {
                    token: TokenType::T_Else,
                    ..
                }) = iter.peek()
                {
                    iter.next(); // Consume the 'else'
                    if let Some(Token {
                        token: TokenType::T_LC,
                        ..
                    }) = iter.next()
                    {
                        let mut else_branch = Vec::new();
                        while let Some(token) = iter.peek() {
                            if token.token == TokenType::T_RC {
                                iter.next(); // Consume the '}'
                                break;
                            } else {
                                else_branch.push(parse_statement(iter, line_number)?);
                            }
                        }
                        Some(else_branch)
                    } else {
                        return Err(format!(
                            "Expected '{{' after 'else' at line {}",
                            line_number
                        ));
                    }
                } else {
                    None
                };

                return Ok(ASTNode::IfStatement {
                    condition: Box::new(condition),
                    then_branch,
                    else_branch,
                });
            } else {
                return Err(format!(
                    "Expected '{{' after condition in 'if' statement at line {}",
                    line_number
                ));
            }
        } else {
            return Err(format!(
                "Expected ')' after condition in 'if' statement at line {}",
                line_number
            ));
        }
    } else {
        return Err(format!("Expected '(' after 'if' at line {}", line_number));
    }
}

fn parse_loop(iter: &mut TokenIterator, mut line_number: usize) -> Result<ASTNode, String> {
    line_number += 1;
    let initialization = if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Semicolon => None,
            _ => Some(Box::new(parse_variable_declaration(iter, line_number)?)),
        }
    } else {
        return Err(format!(
            "Expected loop initialization at line {}",
            line_number
        ));
    };

    if let Some(Token {
        token: TokenType::T_Semicolon,
        ..
    }) = iter.next()
    {
        // Consume the semicolon after initialization
    } else {
        return Err(format!(
            "Expected ';' after loop initialization at line {}",
            line_number
        ));
    }

    let condition = if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Semicolon => None,
            _ => Some(Box::new(parse_expression(iter, line_number)?)),
        }
    } else {
        return Err(format!("Expected loop condition at line {}", line_number));
    };

    if let Some(Token {
        token: TokenType::T_Semicolon,
        ..
    }) = iter.next()
    {
        // Consume the semicolon after condition
    } else {
        return Err(format!(
            "Expected ';' after loop condition at line {}",
            line_number
        ));
    }

    let increment = if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_LC => None,
            _ => {
                let increment_expr = parse_expression(iter, line_number)?;
                if let ASTNode::Literal(literal) = increment_expr {
                    Some(Box::new(ASTNode::Literal(literal)))
                } else {
                    Some(Box::new(increment_expr))
                }
            }
        }
    } else {
        return Err(format!("Expected loop increment at line {}", line_number));
    };

    let mut body = Vec::new();
    if let Some(Token {
        token: TokenType::T_LC,
        ..
    }) = iter.next()
    {
        while let Some(token) = iter.peek() {
            match token.token {
                TokenType::T_RC => {
                    iter.next(); // Consume the closing brace
                    break;
                }
                _ => body.push(parse_statement(iter, line_number)?),
            }
        }
    } else {
        return Err(format!(
            "Expected '{{' to start loop body at line {}",
            line_number
        ));
    }

    Ok(ASTNode::Loop {
        initialization,
        condition,
        increment,
        body,
    })
}

fn parse_expression(iter: &mut TokenIterator, mut line_number: usize) -> Result<ASTNode, String> {
    line_number += 1;
    // For simplicity, just directly returning a string expression
    // In reality, you'd parse based on your language's expression syntax
    if let Some(token) = iter.next() {
        match token.token {
            TokenType::T_Id | TokenType::T_Decimal | TokenType::T_String => {
                Ok(ASTNode::Expression(vec![token.literal.clone()]))
            }
            _ => Err("Invalid expression".to_string()),
        }
    } else {
        Err("Expected expression".to_string())
    }
}

fn parse_binary_expression(
    iter: &mut TokenIterator,
    min_precedence: u8,
    line_number: usize,
) -> Result<ASTNode, String> {
    let mut left_expr = parse_primary_expression(iter, line_number)?;

    while let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_AOp_PL
            | TokenType::T_AOp_MN
            | TokenType::T_AOp_ML
            | TokenType::T_AOp_DV
            | TokenType::T_ROp_L
            | TokenType::T_ROp_G
            | TokenType::T_ROp_LE
            | TokenType::T_ROp_GE
            | TokenType::T_ROp_NE
            | TokenType::T_ROp_E
            | TokenType::T_LOp_AND
            | TokenType::T_LOp_OR => {
                let op_precedence = operator_precedence(&token.token);
                if op_precedence < min_precedence {
                    break;
                }
                let operator = token.token.clone();
                iter.next(); // Consume the operator token
                let right_expr = parse_binary_expression(iter, op_precedence + 1, line_number)?;
                left_expr = ASTNode::BinaryOperation {
                    operator,
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                };
            }
            _ => break,
        }
    }

    Ok(left_expr)
}
////f
fn parse_primary_expression(
    iter: &mut TokenIterator,
    line_number: usize,
) -> Result<ASTNode, String> {
    if let Some(token) = iter.next() {
        match token.token {
            TokenType::T_Id => {
                let mut expr = ASTNode::Identifier(token.literal.clone());
                if let Some(Token {
                    token: TokenType::T_LP,
                    ..
                }) = iter.peek()
                {
                    // Function call
                    iter.next(); // Consume the '(' token
                    let mut arguments = Vec::new();
                    while let Some(token) = iter.peek() {
                        match token.token {
                            TokenType::T_RP => {
                                iter.next(); // Consume the ')' token
                                break;
                            }
                            TokenType::T_Comma => {
                                iter.next(); // Consume the ',' token
                            }
                            _ => {
                                arguments.push(parse_expression(iter, line_number)?);
                            }
                        }
                    }
                    expr = ASTNode::FunctionCall {
                        name: token.literal.clone(), // clone to avoid moving
                        arguments,
                    };
                }
                Ok(expr)
            }
            TokenType::T_Decimal
            | TokenType::T_Hexadecimal
            | TokenType::T_Character
            | TokenType::T_String => Ok(ASTNode::Literal(token.literal.clone())), // clone to avoid moving
            TokenType::T_LP => {
                let expr = parse_binary_expression(iter, 1, line_number)?;
                if let Some(Token {
                    token: TokenType::T_RP,
                    ..
                }) = iter.next()
                {
                    Ok(expr)
                } else {
                    Err("Expected ')' after expression".to_string())
                }
            }
            TokenType::T_AOp_PL | TokenType::T_AOp_MN | TokenType::T_LOp_NOT => {
                let operator = token.token.clone();
                let expr = parse_primary_expression(iter, line_number)?;
                Ok(ASTNode::UnaryOperation {
                    operator,
                    operand: Box::new(expr),
                })
            }
            _ => Err("Invalid expression".to_string()),
        }
    } else {
        Err("Expected expression".to_string())
    }
}
////a
// fn parse_primary_expression(
//     iter: &mut TokenIterator,
//     line_number: usize,
// ) -> Result<ASTNode, String> {
//     if let Some(token) = iter.next() {
//         match token.token {
//             TokenType::T_Id => {
//                 let mut expr = ASTNode::Identifier(token.literal.to_string());
//                 if let Some(Token {
//                     token: TokenType::T_LP,
//                     ..
//                 }) = iter.peek()
//                 {
//                     // Function call
//                     iter.next(); // Consume the '(' token
//                     let mut arguments = Vec::new();
//                     while let Some(token) = iter.peek() {
//                         match token.token {
//                             TokenType::T_RP => {
//                                 iter.next(); // Consume the ')' token
//                                 break;
//                             }
//                             TokenType::T_Comma => {
//                                 iter.next(); // Consume the ',' token
//                             }
//                             _ => {
//                                 arguments.push(parse_expression(iter, line_number)?);
//                             }
//                         }
//                     }
//                     expr = ASTNode::FunctionCall {
//                         name: token.literal.to_string(),
//                         arguments,
//                     };
//                 }
//                 Ok(expr)
//             }
//             TokenType::T_Decimal
//             | TokenType::T_Hexadecimal
//             | TokenType::T_Character
//             | TokenType::T_String => Ok(ASTNode::Literal(token.literal.clone())),
//             TokenType::T_LP => {
//                 let expr = parse_binary_expression(iter, 1, line_number)?;
//                 if let Some(Token {
//                     token: TokenType::T_RP,
//                     ..
//                 }) = iter.next()
//                 {
//                     Ok(expr)
//                 } else {
//                     Err(format!(
//                         "Expected ')' after expression at line {}",
//                         line_number
//                     ))
//                 }
//             }
//             TokenType::T_AOp_PL | TokenType::T_AOp_MN | TokenType::T_LOp_NOT => {
//                 let operator = token.token.clone();
//                 let expr = parse_primary_expression(iter, line_number)?;
//                 Ok(ASTNode::UnaryOperation {
//                     operator,
//                     operand: Box::new(expr),
//                 })
//             }
//             _ => Err(format!("Invalid expression at line {}", line_number)),
//         }
//     } else {
//         Err(format!("Expected expression at line {}", line_number))
//     }
// }

fn operator_precedence(token: &TokenType) -> u8 {
    match token {
        TokenType::T_ROp_L
        | TokenType::T_ROp_G
        | TokenType::T_ROp_LE
        | TokenType::T_ROp_GE
        | TokenType::T_ROp_NE
        | TokenType::T_ROp_E => 2,
        TokenType::T_LOp_AND | TokenType::T_LOp_OR => 1,
        TokenType::T_AOp_PL | TokenType::T_AOp_MN => 3,
        TokenType::T_AOp_ML | TokenType::T_AOp_DV | TokenType::T_AOp_RM => 4,
        _ => 0,
    }
}
