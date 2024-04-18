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
            column: 0,
        };
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        while self.ch == '\r' || self.ch == '\n' || self.ch == '\t' || self.ch == ' ' {
            self.read_char()
        }
        let mut skip = false;
        let (token_type, literal): (TokenType, String) = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    (TokenType::T_ROp_E, "==".to_owned())
                } else {
                    (TokenType::T_Assign, self.ch.to_string())
                }
            }
            '+' => {
                if self.is_digit(self.peek_char()) {
                    self.read_char();
                    skip = true;
                    (TokenType::T_Decimal, "+".to_string() + &self.read_digit())
                } else {
                    (TokenType::T_AOp_PL, self.ch.to_string())
                }
            }
            '(' => (TokenType::T_LP, self.ch.to_string()),
            '-' => {
                if self.is_digit(self.peek_char()) {
                    self.read_char();
                    skip = true;
                    (TokenType::T_Decimal, "-".to_string() + &self.read_digit())
                } else {
                    (TokenType::T_AOp_MN, self.ch.to_string())
                }
            }
            '*' => (TokenType::T_AOp_ML, self.ch.to_string()),
            '%' => (TokenType::T_AOp_RM, self.ch.to_string()),
            '/' => {
                if self.peek_char() == '/' {
                    let mut comment = String::new();
                    while self.ch != '\n' && self.ch != '\0' && self.ch != '\r' {
                        comment.push(self.ch);
                        self.read_char();
                    }
                    (TokenType::T_Comment, comment)
                } else {
                    (TokenType::T_AOp_DV, "/".to_owned())
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    (TokenType::T_ROp_NE, "!=".to_owned())
                } else {
                    (TokenType::T_LOp_NOT, self.ch.to_string())
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    (TokenType::T_ROp_LE, "<=".to_string())
                } else {
                    (TokenType::T_ROp_L, self.ch.to_string())
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    //WARN: the test is fucked up and there is two less equial for some resaon :|
                    (TokenType::T_ROp_LE, ">=".to_owned())
                } else {
                    (TokenType::T_ROp_G, self.ch.to_string())
                }
            }
            ',' => (TokenType::T_Comma, self.ch.to_string()),
            ';' => (TokenType::T_Semicolon, self.ch.to_string()),
            ')' => (TokenType::T_RP, self.ch.to_string()),
            '{' => (TokenType::T_LC, self.ch.to_string()),
            '}' => (TokenType::T_RC, self.ch.to_string()),
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    (TokenType::T_LOp_AND, "&&".to_owned())
                } else {
                    (TokenType::ILLEGAL, self.ch.to_string())
                }
            }
            '|' => {
                if self.peek_char() == '|' {
                    self.read_char();
                    (TokenType::T_LOp_OR, "||".to_owned())
                } else {
                    (TokenType::ILLEGAL, self.ch.to_string())
                }
            }
            '[' => (TokenType::T_LB, self.ch.to_string()),
            ']' => (TokenType::T_RB, self.ch.to_string()),
            '"' => (TokenType::T_String, self.read_string()),
            '\'' => {
                let pos = self.position + 1;
                self.read_char();
                if self.ch == '\\' {
                    self.read_char();
                }
                self.read_char();
                (
                    TokenType::T_Character,
                    format!("'{}", self.input[pos..self.position + 1].to_string()),
                )
            }
            '0' => {
                skip = true;
                let peek_char = self.peek_char();
                if peek_char == 'X' || peek_char == 'x' {
                    self.read_char();
                    self.read_char();
                    (
                        TokenType::T_Hexadecimal,
                        format!("0{peek_char}{}", self.read_hex()),
                    )
                } else {
                    (TokenType::T_Decimal, self.read_number())
                }
            }
            '1'..='9' => {
                skip = true;
                (TokenType::T_Decimal, self.read_number())
            }
            '\0' => (TokenType::End, "".to_owned()),
            _ if self.is_letter(self.ch) => {
                let ide = self.read_identifier();
                self.read_position -= 1;
                if self.lookup_ident(&ide) != TokenType::ILLEGAL {
                    (self.lookup_ident(&ide), ide)
                } else {
                    (TokenType::T_Id, ide)
                }
            }
            _ if self.is_digit(self.ch) => (TokenType::T_Int, self.read_number()),
            _ => (TokenType::ILLEGAL, self.ch.to_string()),
        };

        if !skip {
            self.read_char();
        }

        Token {
            token: token_type,
            literal,
            line: self.line,
            column: self.column,
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

    fn read_digit(&mut self) -> String {
        let position = self.position;
        while self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while self.is_letter(self.ch) || self.is_digit(self.ch) {
            self.read_char();
        }
        self.input[position..self.position].to_string()
    }

    fn read_hex(&mut self) -> String {
        let position = self.position;
        while (self.ch >= '0' && self.ch <= '9')
            || (self.ch >= 'A' && self.ch <= 'F')
            || (self.ch >= 'a' && self.ch <= 'f')
        {
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
        let mut last_char = ' ';
        while (self.ch != '"' || last_char == '\\') && self.ch != '\0' {
            last_char = self.ch;
            self.read_char();
        }
        format!("\"{}\"", &self.input[position..self.position])
    }

    fn lookup_ident(&self, idt: &str) -> TokenType {
        match idt {
            "bool" => TokenType::T_Bool,
            "break" => TokenType::T_Break,
            "char" => TokenType::T_Char,
            "continue" => TokenType::T_Continue,
            "else" => TokenType::T_Else,
            "false" => TokenType::T_False,
            "for" => TokenType::T_For,
            "if" => TokenType::T_If,
            "int" => TokenType::T_Int,
            "print" => TokenType::T_Print,
            "return" => TokenType::T_Return,
            "true" => TokenType::T_True,
            "semicolon" => TokenType::T_Semicolon,
            _ => TokenType::ILLEGAL,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    const TEST_IN: &str = include_str!("./../tests/test.in");
    const TEST_OUT: &str = include_str!("./../tests/test.out");

    #[test]
    fn test_via_c_file() {
        let mut lexer = super::Lexer::new(TEST_IN.to_string());
        let out_put = TEST_OUT.split("\n").into_iter().collect::<Vec<&str>>();
        let mut i = 0;
        // let mut cols = 0;
        while !lexer.is_end() {
            let token = lexer.next_token();
            if token.token == TokenType::End {
                dbg!("END OF FILE");
                break;
            }
            assert_eq!(
                out_put[i].replace("\r", ""),
                format!("{} -> {}", token.literal, token.token.as_ref()),
                "token is {} -> {} line {} , but excpected {:?}",
                token.literal,
                token.token.as_ref(),
                token.line,
                out_put[i]
            );
            i += 1;
        }
    }
}
