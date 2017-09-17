
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source: source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(token) => {
                    if !token.token_type.is_ignored() {
                        self.tokens.push(token);
                    }
                }
                Err(err) => {}
            }
        }
        self.tokens
            .push(Token::new(TokenType::EOF, "", Literal::Nil, self.line));
        self.tokens.to_vec()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_token(&mut self) -> Result<Token, ScanError> {
        let c = self.advance();
        match c {
            '(' => Ok(self.create_token(TokenType::LEFT_PAREN)),
            ')' => Ok(self.create_token(TokenType::RIGHT_PAREN)),
            '{' => Ok(self.create_token(TokenType::LEFT_BRACE)),
            '}' => Ok(self.create_token(TokenType::RIGHT_BRACE)),
            ',' => Ok(self.create_token(TokenType::COMMA)),
            '.' => Ok(self.create_token(TokenType::DOT)),
            '-' => Ok(self.create_token(TokenType::MINUS)),
            '+' => Ok(self.create_token(TokenType::PLUS)),
            ';' => Ok(self.create_token(TokenType::SEMICOLON)),
            '*' => Ok(self.create_token(TokenType::STAR)),
            '!' => {
                if self.match_char('=') {
                    self.advance();
                    Ok(self.create_token(TokenType::BANG_EQUAL))
                } else {
                    Ok(self.create_token(TokenType::BANG))
                }
            }
            '=' => {
                if self.match_char('=') {
                    self.advance();
                    Ok(self.create_token(TokenType::EQUAL_EQUAL))
                } else {
                    Ok(self.create_token(TokenType::EQUAL))
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.advance();
                    Ok(self.create_token(TokenType::LESS_EQUAL))
                } else {
                    Ok(self.create_token(TokenType::LESS))
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.advance();
                    Ok(self.create_token(TokenType::GREATER_EQUAL))
                } else {
                    Ok(self.create_token(TokenType::GREATER))
                }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                    Ok(self.create_token(TokenType::COMMENT))
                } else {
                    Ok(self.create_token(TokenType::SLASH))
                }
            }
            ' ' | '\r' | '\t' => Ok(self.create_token(TokenType::WHITESPACE)),
            '\n' => {
                self.line += 1;
                Ok(self.create_token(TokenType::NEWLINE))
            }
            '"' => self.scan_string(),
            c if c.is_digit(10) => self.scan_number(),
            _ => Err(ScanError::new(self.line, "Unexpected character.")),
        }
    }

    fn scan_string(&mut self) -> Result<Token, ScanError> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        // Unterminated string.
        if self.is_at_end() {
            return Err(ScanError::new(self.line, "Unterminated string."));
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = &self.source[self.start + 1..self.current - 1];
        Ok(self.create_token_with_literal(TokenType::STRING, Literal::String(value.to_owned())))
    }

    fn scan_number(&mut self) -> Result<Token, ScanError> {
        while self.peek().is_digit(10) {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            // Consume the "."
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value = &self.source[self.start..self.current];
        Ok(self.create_token_with_literal(TokenType::NUMBER,
                                          Literal::Number(value.parse::<f64>().unwrap())))
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let b = self.source.as_bytes();
        b[self.current] as char
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        let b = self.source.as_bytes();
        b[self.current + 1] as char
    }

    fn match_char(&self, c: char) -> bool {
        self.peek() == c
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        let b = self.source.as_bytes();
        b[self.current - 1] as char
    }

    fn create_token(&self, token_type: TokenType) -> Token {
        self.create_token_with_literal(token_type, Literal::Nil)
    }

    fn create_token_with_literal(&self, token_type: TokenType, literal: Literal) -> Token {
        let s = &self.source[self.start as usize..self.current as usize];
        Token::new(token_type, s, literal, self.line)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: i32) -> Self {
        Token {
            token_type: token_type,
            lexeme: lexeme.to_owned(),
            literal: literal,
            line: line,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[allow(non_camel_case_types)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,

    // IGNORED lexemes
    COMMENT,
    WHITESPACE,
    NEWLINE,
}

impl TokenType {
    fn is_ignored(&self) -> bool {
        let n = *self as u8;
        n > 38
    }
}

#[derive(Clone, Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
    Nil,
}

pub struct ScanError {
    line: i32,
    message: String,
}

impl ScanError {
    fn new(line: i32, message: &str) -> Self {
        ScanError {
            line: line,
            message: message.to_owned(),
        }
    }
}
