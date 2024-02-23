//tokenization file

use crate::ukiyoCompiler::Ukiyo;


#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,

    // One or two character tokens.
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.
    Identifier, String, Number,

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<String>,
    line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, literal: Option<String>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}

// struct: data type with multiple fields
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

//Methods for a struct are defined inside an impl block.
impl Scanner {
    //defining a new scanner(constructor)
    pub fn new(source: String) -> Scanner {
        Scanner {
            source, // from here we will extract the tokens
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }
//if current is at the end of the source, return true
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self)-> char{
        let result = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        result
      }
      fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<String>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(token_type, text, literal, self.line));
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end(){return false;}
        else if self.source.chars().nth(self.current).unwrap() != expected { return false;} //The unwrap method is used to get the character at the current position, and it will panic if there's no character at that position
        
        else {
            self.current +=1;
            return true;
        }
    }
    //the scan_token method, which scans the source code from the start position, creates a Token, and adds it to the tokens vector
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenType::BangEqual);
                } else {
                    self.add_token(TokenType::Bang);
                }
            },
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenType::EqualEqual);
                } else {
                    self.add_token(TokenType::Equal);
                }
            },874359
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenType::LessEqual);
                } else {
                    self.add_token(TokenType::Less);
                }
            },
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenType::GreaterEqual);
                } else {
                    self.add_token(TokenType::Greater);
                }
            },
            _ =>  Ukiyo::error(self.line, "Unexpected character.".to_string()), // Do nothing for other characters
        }
    }

//tokenizes the source code returns a vector of tokens

//The &mut self means the method can modify the Scanner instance.
    pub fn scan_tokens(&mut self) -> Vec<Token> {
    
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
//After all tokens have been scanned, this line adds an EOF (end-of-file) token to the tokens vector. This token signifies the end of the source code.
        self.tokens.push(Token::new(TokenType::Eof, "".to_string(), None, self.line));
        self.tokens.clone() // This line returns a copy of the tokens vector. The clone method is used because self.tokens is borrowed, and we can't move out of borrowed content.
    }

}








