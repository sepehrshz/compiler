use crate::lexial::Lexer;
use crate::token::{Token, TokenType};

pub(crate) mod lexial;
pub(crate) mod token;

#[derive(Debug)]
enum ASTNode {
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
        initialization: Option<String>,
        condition: Option<String>,
        increment: Option<String>,
        body: Vec<ASTNode>,
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
        if self.position < self.tokens.len() {
            self.position += 1;
            Some(&self.tokens[self.position - 1])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&'a Token> {
        if self.position < self.tokens.len() {
            Some(&self.tokens[self.position])
        } else {
            None
        }
    }
}

pub fn parse(lexer: &mut Lexer) -> Result<ASTNode, String> {
    let mut tokens = Vec::new();
    while let Some(token) = lexer.next_token() {
        tokens.push(token?);
    }
    parse_program(&tokens)
}

fn parse_program(tokens: &[Token]) -> Result<ASTNode, String> {
    let mut iter = TokenIterator::new(tokens);
    let mut nodes = Vec::new();

    while let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Int => {
                nodes.push(parse_function(&mut iter)?);
            }
            _ => return Err(format!("Unexpected token: {:?}", token)),
        }
    }

    Ok(ASTNode::Program(nodes))
}

fn parse_function(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    let return_type = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err("Expected return type".to_string());
    };

    let name = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err("Expected function name".to_string());
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
                        return Err("Expected parameter name".to_string());
                    }
                }
                TokenType::T_RP => break,
                _ => return Err(format!("Unexpected token in parameters: {:?}", token)),
            }
        }
    } else {
        return Err("Expected '(' after function name".to_string());
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
                body.push(parse_statement(iter)?);
            }
        }
    } else {
        return Err("Expected '{' to start function body".to_string());
    }

    Ok(ASTNode::Function {
        return_type,
        name,
        params,
        body,
    })
}

fn parse_statement(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Id => parse_variable_declaration(iter),
            TokenType::T_Print => parse_print_statement(iter),
            TokenType::T_If => parse_if_statement(iter),
            TokenType::T_For => parse_loop(iter),
            _ => Err(format!("Unexpected token in statement: {:?}", token)),
        }
    } else {
        Err("Unexpected end of input".to_string())
    }
}

fn parse_variable_declaration(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    let var_type = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err("Expected variable type".to_string());
    };

    let name = if let Some(token) = iter.next() {
        token.literal.clone()
    } else {
        return Err("Expected variable name".to_string());
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
            return Err("Expected value after '='".to_string());
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

fn parse_print_statement(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    let value = if let Some(Token {
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
                    return Err("Expected ')' after print statement value".to_string());
                }
            } else {
                return Err("Invalid print statement value".to_string());
            }
        } else {
            return Err("Expected value after '(' in print statement".to_string());
        }
    } else {
        return Err("Expected '(' in print statement".to_string());
    };
}

fn parse_if_statement(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    let mut condition = Vec::new();
    while let Some(token) = iter.next() {
        match token.token {
            TokenType::T_LP => break,
            _ => condition.push(token.literal.clone()),
        }
    }

    let mut then_branch = Vec::new();
    while let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Else | TokenType::T_End => break,
            _ => then_branch.push(parse_statement(iter)?),
        }
    }

    let else_branch = if let Some(Token {
        token: TokenType::T_Else,
        ..
    }) = iter.peek()
    {
        iter.next(); // Consume the 'else' token
        let mut else_branch = Vec::new();
        while let Some(token) = iter.peek() {
            match token.token {
                TokenType::T_End => break,
                _ => else_branch.push(parse_statement(iter)?),
            }
        }
        Some(else_branch)
    } else {
        None
    };

    Ok(ASTNode::IfStatement {
        condition: Box::new(ASTNode::Expression(condition)),
        then_branch,
        else_branch,
    })
}

fn parse_loop(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    let initialization = if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Semicolon => None,
            _ => Some(parse_variable_declaration(iter)?),
        }
    } else {
        return Err("Expected loop initialization".to_string());
    };

    if let Some(Token {
        token: TokenType::T_Semicolon,
        ..
    }) = iter.next()
    {
        // Consume the semicolon after initialization
    } else {
        return Err("Expected ';' after loop initialization".to_string());
    }

    let condition = if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_Semicolon => None,
            _ => Some(parse_expression(iter)?),
        }
    } else {
        return Err("Expected loop condition".to_string());
    };

    if let Some(Token {
        token: TokenType::T_Semicolon,
        ..
    }) = iter.next()
    {
        // Consume the semicolon after condition
    } else {
        return Err("Expected ';' after loop condition".to_string());
    }

    let increment = if let Some(token) = iter.peek() {
        match token.token {
            TokenType::T_LC => None,
            _ => Some(parse_expression(iter)?),
        }
    } else {
        return Err("Expected loop increment".to_string());
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
                _ => body.push(parse_statement(iter)?),
            }
        }
    } else {
        return Err("Expected '{' to start loop body".to_string());
    }

    Ok(ASTNode::Loop {
        initialization,
        condition,
        increment,
        body,
    })
}

fn parse_expression(iter: &mut TokenIterator) -> Result<ASTNode, String> {
    parse_binary_expression(iter, 1)
}

fn parse_binary_expression(
    iter: &mut TokenIterator,
    min_precedence: u8,
) -> Result<ASTNode, String> {
    let mut left_expr = parse_primary_expression(iter)?;

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
                iter.next(); // Consume the operator token
                let right_expr = parse_binary_expression(iter, op_precedence + 1)?;
                left_expr = ASTNode::BinaryOperation {
                    operator: token.token.clone(),
                    left: Box::new(left_expr),
                    right: Box::new(right_expr),
                };
            }
            _ => break,
        }
    }

    Ok(left_expr)
}

fn parse_primary_expression(iter: &mut TokenIterator) -> Result<ASTNode, String> {
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
                                arguments.push(parse_expression(iter)?);
                            }
                        }
                    }
                    expr = ASTNode::FunctionCall {
                        name: token.literal,
                        arguments,
                    };
                }
                Ok(expr)
            }
            TokenType::T_Decimal
            | TokenType::T_Hexadecimal
            | TokenType::T_Character
            | TokenType::T_String => Ok(ASTNode::Literal(token.literal)),
            TokenType::T_LP => {
                let expr = parse_binary_expression(iter, 1)?;
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
                let expr = parse_primary_expression(iter)?;
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
