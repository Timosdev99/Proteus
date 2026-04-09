use crate::lexer::token::Token;

pub struct Lexer {
    source: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
        }
    }

    fn current(&self) -> Option<char> {
        self.source.get(self.pos).copied()
    }

    fn peek(&self) -> Option<char> {
        self.source.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.current();
        self.pos += 1;
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current() {
            if ch.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_comments(&mut self) {
        if self.current() == Some('/') && self.peek() == Some('/') {
            self.advance();
            self.advance();

            while let Some(ch) = self.current() {
                if ch == '\n' {
                    break;
                }
                self.advance();
            }
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            let start_pos = self.pos;
            self.skip_whitespace();
            self.skip_comments();
            if self.pos == start_pos {
                break;
            }
        }
    }

    fn read_words(&mut self) -> String {
        let mut word = String::new();

        while let Some(ch) = self.current() {
            if ch.is_alphanumeric() || ch == '_' {
                word.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        word
    }

    fn read_string(&mut self) -> String {
        self.advance();
        let mut s = String::new();
        while let Some(ch) = self.current() {
            if ch == '"' {
                self.advance();
                break;
            }
            s.push(ch);
            self.advance();
        }
        s
    }

    fn read_number(&mut self) -> f64 {
        let mut s = String::new();
        while let Some(ch) = self.current() {
            if ch.is_ascii_digit() || ch == '.' {
                s.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        s.parse::<f64>().unwrap_or(0.0)
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        loop {
            self.skip_whitespace_and_comments();

            match self.current() {
                None => {
                    tokens.push(Token::EOF);
                    break;
                }

                Some('(') => {
                    self.advance();
                    tokens.push(Token::LParen);
                }
                Some(')') => {
                    self.advance();
                    tokens.push(Token::RParen);
                }
                Some('{') => {
                    self.advance();
                    tokens.push(Token::LBrace);
                }

                Some('}') => {
                    self.advance();
                    tokens.push(Token::RBrace);
                }

                Some(':') => {
                    self.advance();
                    tokens.push(Token::Colon);
                }
                Some(';') => {
                    self.advance();
                    tokens.push(Token::Semicolon);
                }
                Some(',') => {
                    self.advance();
                    tokens.push(Token::Comma);
                }

                Some('*') => {
                    self.advance();
                    tokens.push(Token::Multiplication)
                }

                Some('+') => {
                    self.advance();
                    tokens.push(Token::Addition);
                }

                Some('-') => {
                    self.advance();
                    tokens.push(Token::Substraction);
                }

                Some('/') => {
                    self.advance();
                    tokens.push(Token::Division);
                }

                Some('>') => {
                    self.advance();
                    tokens.push(Token::Greater);
                }

                Some('<') => {
                    self.advance();
                    tokens.push(Token::Less);
                }

                Some(ch) if ch.is_ascii_digit() => {
                    let n = self.read_number();
                    tokens.push(Token::NumberLiteral(n))
                }

                Some(ch) if ch.is_alphabetic() => {
                    let word = self.read_words();
                    let token = match word.as_str() {
                        "function" => Token::Function,
                        "return" => Token::Return,
                        "interface" => Token::Interface,
                        "const" => Token::Const,
                        "let" => Token::Let,
                        "export" => Token::Export,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "for" => Token::For,
                        _ => Token::Identifier(word),
                    };
                    tokens.push(token)
                }

                Some('=') => {
                    if self.peek() == Some('>') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::Arrow);
                    } else if self.peek() == Some('=') {
                        self.advance();
                        self.advance();
                        tokens.push(Token::Equate);
                    } else {
                        self.advance();
                        tokens.push(Token::Assign);
                    }
                }

                Some('"') => {
                    let s = self.read_string();
                    tokens.push(Token::StringLiteral(s))
                }

                _ => {
                    // skipping unknow characters for now
                    self.advance();
                }
            }
        }

        tokens
    }
}
