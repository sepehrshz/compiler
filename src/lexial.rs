use crate::token::{Token, TokenType};

pub(crate) struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
            line: 1,
            column: 1,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let (token_type, literal): (TokenType, String) = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    (TokenType::EQUAIL, format!("{}{}", ch, self.ch))
                } else {
                    (TokenType::ASSIGN, self.ch.to_string())
                }
            }
            '+' => (TokenType::PLUS, self.ch.to_string()),
            '(' => (TokenType::OpenParenthesis, self.ch.to_string()),
            '-' => (TokenType::MINUS, self.ch.to_string()),
            '*' => (TokenType::MUTPLY, self.ch.to_string()),
            '%' => (TokenType::MOD, self.ch.to_string()),
            '/' => {
                if self.peek_char() == '/' {
                    let mut comment = String::new();
                    while self.ch != '\n' && self.ch != '\0' {
                        comment.push(self.ch);
                        self.read_char();
                    }
                    (TokenType::COMMENTS, comment)
                } else {
                    (TokenType::DIVIDE, "/".to_owned())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    let ch = self.ch;
                    self.read_char();
                    let literal = format!("{}{}", ch, self.ch);
                    (TokenType::NOTEQUIL, literal)
                } else {
                    (TokenType::NOT, self.ch.to_string())
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    (TokenType::SMALLERANDEQUAIL, self.ch.to_string())
                } else {
                    (TokenType::SMALLER, self.ch.to_string())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    (TokenType::BIGGERANDEQUAIL, self.ch.to_string())
                } else {
                    (TokenType::BIGGER, self.ch.to_string())
                }
            }
            ',' => (TokenType::COMMA, self.ch.to_string()),
            ';' => (TokenType::SEMICOLON, self.ch.to_string()),
            ')' => (TokenType::CloseParenthesis, self.ch.to_string()),
            '{' => (TokenType::OpenBracket, self.ch.to_string()),
            '}' => (TokenType::CloseBracket, self.ch.to_string()),
            '&' => {
                if self.peek_char() == '&' {
                    (TokenType::AndLogical, self.ch.to_string())
                } else {
                    (TokenType::AND, self.ch.to_string())
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    (TokenType::OrLogical, self.ch.to_string())
                } else {
                    (TokenType::OR, self.ch.to_string())
                }
            }
            '[' => (TokenType::OpenBraces, self.ch.to_string()),
            ']' => (TokenType::CloseBraces, self.ch.to_string()),
            '"' => (TokenType::STRING, self.read_string()),
            '0'..='9' => (TokenType::INT, self.read_number()),
            '\0' => (TokenType::End, "".to_owned()),
            _ if self.is_letter(self.ch) => {
                let ide = self.read_identifier();
                self.read_position -= 1;
                if self.lookup_ident(&ide) != TokenType::ILLEGAL {
                    (self.lookup_ident(&ide), ide)
                } else {
                    (TokenType::Identifer, ide)
                }
            }
            _ if self.is_digit(self.ch) => (TokenType::INT, self.read_number()),
            _ => (TokenType::ILLEGAL, self.ch.to_string()),
        };

        self.read_char();

        Token {
            token: token_type,
            literal,
            line: self.line,
            column: self.column,
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    fn is_letter(&self, ch: char) -> bool {
        (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || ch == '_'
    }

    pub fn is_end(&self) -> bool {
        self.ch == '\0'
    }

    fn is_digit(&self, ch: char) -> bool {
        (ch >= '0' && ch <= '9') || ch == '.'
    }
    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
        if self.ch == '\n' {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input.chars().nth(self.read_position).unwrap()
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        self.read_char();
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn lookup_ident(&self, idt: &str) -> TokenType {
        match idt {
            "bool" => TokenType::BOOL,
            "break" => TokenType::BREAK,
            "char" => TokenType::CHAR,
            "continue" => TokenType::CONTINUE,
            "else" => TokenType::ELSE,
            "false" => TokenType::FALSE,
            "for" => TokenType::FOR,
            "if" => TokenType::IF,
            "int" => TokenType::INT,
            "print" => TokenType::PRINT,
            "return" => TokenType::RETURN,
            "true" => TokenType::TRUE,
            "semicolon" => TokenType::SEMICOLON,
            _ => TokenType::ILLEGAL,
        }
    }
}

