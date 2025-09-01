use std::{collections::HashMap, fmt};

use lazy_static::lazy_static;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = {
        let mut map = HashMap::new();
        map.insert("and", TokenType::And);
        map.insert("class", TokenType::Class);
        map.insert("else", TokenType::Else);
        map.insert("false", TokenType::False);
        map.insert("fun", TokenType::Fun);
        map.insert("for", TokenType::For);
        map.insert("if", TokenType::If);
        map.insert("nil", TokenType::Nil);
        map.insert("or", TokenType::Or);
        map.insert("print", TokenType::Print);
        map.insert("return", TokenType::Return);
        map.insert("super", TokenType::Super);
        map.insert("this", TokenType::This);
        map.insert("true", TokenType::True);
        map.insert("var", TokenType::Var);
        map.insert("while", TokenType::While);
        map
    };
}

#[derive(Debug, Clone)]
pub struct ScannerError {
    line: usize,
    column: usize,
    message: String,
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}:{}] Error: {}",
            self.line, self.column, self.message
        )
    }
}

#[derive(Debug, Clone)]
enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug, Clone)]
enum Literal {
    Number(f64),
    Str(String),
    Nil,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
    value: Option<Literal>,
}

pub struct Scanner {
    source: String,
    current: usize,
    line: usize,
    start: usize,
    tokens: Vec<ScannerResult>,
}

#[derive(Debug, Clone)]
pub enum ScannerResult {
    Token(Token),
    Error(ScannerError),
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Self {
            source,
            current: 0,
            line: 1,
            start: 0,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<ScannerResult> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(ScannerResult::Token(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            line: self.line,
            value: None,
        }));

        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.emit(TokenType::LeftParen, None),
            ')' => self.emit(TokenType::RightParen, None),
            '{' => self.emit(TokenType::LeftBrace, None),
            '}' => self.emit(TokenType::RightBrace, None),
            ',' => self.emit(TokenType::Comma, None),
            '.' => self.emit(TokenType::Dot, None),
            '-' => self.emit(TokenType::Minus, None),
            '+' => self.emit(TokenType::Plus, None),
            ';' => self.emit(TokenType::Semicolon, None),
            '*' => self.emit(TokenType::Star, None),
            '!' => {
                let token_type = if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.emit(token_type, None);
            }
            '<' => {
                let token_type = if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.emit(token_type, None);
            }
            '>' => {
                let token_type = if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.emit(token_type, None);
            }
            '=' => {
                let token_type = if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.emit(token_type, None);
            }
            '/' => self.scan_slash(),
            '"' => self.scan_string(),
            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => {
                self.line += 1;
            }
            c if c.is_digit(10) => self.scan_number(),
            c if c.is_alphabetic() => self.scan_identifier_or_keyword(),
            _ => self.emit_error(format!("Unexpected character '{}'", c)),
        }
    }

    fn scan_identifier_or_keyword(&mut self) {
        while self.peek().is_alphabetic() || self.peek().is_digit(10) || self.peek() == '_' {
            self.advance();
        }

        let value = self.source[self.start..self.current].to_string();
        if let Some(token_type) = KEYWORDS.get(value.as_str()) {
            self.emit(token_type.clone(), None);
        } else {
            self.emit(TokenType::Identifier, Some(Literal::Str(value)));
        }
    }

    fn emit_error(&mut self, message: String) {
        self.tokens.push(ScannerResult::Error(ScannerError {
            line: self.line,
            column: self.current,
            message,
        }));
    }

    fn scan_number(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }

        if self.peek() == '.' {
            self.advance();

            while self.peek().is_digit(10) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        self.emit(TokenType::Number, Some(Literal::Number(value)));
    }

    fn scan_string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return self.emit_error("Unterminated string".to_string());
        }

        self.advance();
        let value = self.source[self.start + 1..self.current - 1].to_string();
        self.emit(TokenType::String, Some(Literal::Str(value)));
    }

    fn scan_slash(&mut self) {
        if self.match_next('/') {
            while self.peek() != '\n' {
                self.advance();
            }
        } else {
            self.emit(TokenType::Slash, None);
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() == c {
            self.current += 1;
            return true;
        }

        false
    }

    fn emit(&mut self, token_type: TokenType, value: Option<Literal>) {
        let lexeme = self.source[self.start..self.current].to_string();
        let line = self.line;
        self.tokens.push(ScannerResult::Token(Token {
            token_type,
            lexeme,
            line,
            value,
        }));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        c
    }
}
