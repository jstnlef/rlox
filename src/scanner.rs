
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
                Ok(token) => self.tokens.push(token),
                Err(err) => {}
            }
        }
        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
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
            _ => Err(ScanError::new(self.line, "Unexpected character.")),
        }
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        let b = self.source.as_bytes();
        b[self.current] as char
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
        let s = &self.source[self.start as usize..self.current as usize];
        Token::new(token_type, s, self.line)
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: i32) -> Self {
        Token {
            token_type: token_type,
            lexeme: lexeme.to_owned(),
            line: line,
        }
    }
}

#[derive(Clone, Debug)]
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

    COMMENT,
    EOF,
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
