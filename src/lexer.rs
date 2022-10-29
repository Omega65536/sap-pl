#[derive(Debug, PartialEq)]
pub enum Token {
    End,
    Variable(String),
    LeftParen,
    RightParen,
    And,
    Or,
    Not,
    Unknown,
}

pub struct Lexer {
    chars: Vec<char>,
    index: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Lexer {
            chars: source.chars().collect(),
            index: 0,
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        loop {
            let next_token = self.next_token();
            match next_token {
                Token::End => {
                    tokens.push(next_token); 
                    return tokens
                },
                _ => tokens.push(next_token),
            }
        }
    }

    fn next_token(&mut self) -> Token {
        match self.peek() {
            Some(ch) => {
                if *ch == ' ' {
                    self.advance();
                    return self.next_token()
                }
                if ch.is_alphabetic() || *ch == '_' {
                    return self.next_name();
                }
                let token = match ch {
                    '(' => Token::LeftParen,
                    ')' => Token::RightParen,
                    ch => Token::Unknown,
                };
                self.advance();
                token
            },
            None => Token::End
        }
    }

    fn next_name(&mut self) -> Token {
        let mut string = String::new();
        while let Some(ch) = self.peek() {
            if ch.is_alphabetic() || *ch == '_' {
                string.push(*ch);
            } else {
                break;
            }
            self.advance();
        }
        match string.as_str() {
            "and" => Token::And,
            "or" => Token::Or,
            "not" => Token::Not,
            _ => Token::Variable(string),
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }
}
