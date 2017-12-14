use scanner::{Literal, Token, TokenType};
use parser::ast::{Expr, Stmt, AST};

macro_rules! binary {
    ($self:expr, $func:expr, $token_types:expr) => {{
        let mut expr = $func;

        while $self.match_token($token_types) {
            let operator = $self.previous().clone();
            let right = $func;
            expr = Ok(Box::new(Expr::Binary(expr?, operator, right?)))
        }
        expr
    }}
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> ParseResult<AST> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        Ok(AST { root: statements })
    }

    fn declaration(&mut self) -> ParseResult<Box<Stmt>> {
        let result = if self.match_token(&[TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match result {
            Ok(_) => result,
            Err(_) => {
                self.synchronize();
                result
            }
        }
    }

    fn var_declaration(&mut self) -> ParseResult<Box<Stmt>> {
        let name = self.consume_token(TokenType::IDENTIFIER, "Expect variable name.")?
            .clone();

        let mut initializer = Box::new(Expr::Literal(Literal::Nil));
        if self.match_token(&[TokenType::EQUAL]) {
            initializer = self.expression()?;
        }

        self.consume_token(
            TokenType::SEMICOLON,
            "Expect ';' after value.",
        )?;
        Ok(Box::new(Stmt::Var(name.clone(), initializer)))
    }

    fn statement(&mut self) -> ParseResult<Box<Stmt>> {
        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        self.expression_statement()
    }

    fn print_statement(&mut self) -> ParseResult<Box<Stmt>> {
        let value = self.expression()?;
        self.consume_token(
            TokenType::SEMICOLON,
            "Expect ';' after value.",
        )?;
        Ok(Box::new(Stmt::Print(value)))
    }

    fn expression_statement(&mut self) -> ParseResult<Box<Stmt>> {
        let expr = self.expression()?;
        self.consume_token(
            TokenType::SEMICOLON,
            "Expect ';' after expression.",
        )?;
        Ok(Box::new(Stmt::Expression(expr)))
    }

    fn expression(&mut self) -> ParseResult<Box<Expr>> {
        self.equality()
    }

    fn equality(&mut self) -> ParseResult<Box<Expr>> {
        binary!(
            self,
            self.comparison(),
            &[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]
        )
    }

    fn comparison(&mut self) -> ParseResult<Box<Expr>> {
        binary!(
            self,
            self.addition(),
            &[
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                TokenType::LESS,
                TokenType::LESS_EQUAL,
            ]
        )
    }

    fn addition(&mut self) -> ParseResult<Box<Expr>> {
        binary!(
            self,
            self.multiplication(),
            &[TokenType::MINUS, TokenType::PLUS]
        )
    }

    fn multiplication(&mut self) -> ParseResult<Box<Expr>> {
        binary!(self, self.unary(), &[TokenType::SLASH, TokenType::STAR])
    }

    fn unary(&mut self) -> ParseResult<Box<Expr>> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Ok(Box::new(Expr::Unary(operator, right?)));
        }
        self.primary()
    }

    fn primary(&mut self) -> ParseResult<Box<Expr>> {
        if self.match_token(
            &[
                TokenType::FALSE,
                TokenType::TRUE,
                TokenType::NIL,
                TokenType::NUMBER,
                TokenType::STRING,
            ],
        )
        {
            return Ok(Box::new(Expr::Literal(self.previous().literal.clone())));
        };

        if self.match_token(&[TokenType::IDENTIFIER]) {
            return Ok(Box::new(Expr::Variable(self.previous().clone())));
        }

        if self.match_token(&[TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            self.consume_token(
                TokenType::RIGHT_PAREN,
                "Expect ')' after expression.",
            )?;
            return Ok(Box::new(Expr::Grouping(expr)));
        }

        Err(self.error(self.peek(), "Expect expression."))
    }

    fn consume_token(&mut self, token_type: TokenType, message: &str) -> ParseResult<&Token> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }
        Err(self.error(self.peek(), message))
    }

    fn match_token(&mut self, token_types: &[TokenType]) -> bool {
        for t in token_types {
            if self.check(t) {
                self.advance();
                return true;
            }
        }

        false
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == *token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }

    fn error(&self, token: &Token, message: &str) -> ParseError {
        ParseError::new(token, message)
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SEMICOLON {
                return;
            }

            match self.peek().token_type {
                TokenType::CLASS | TokenType::FUN | TokenType::VAR | TokenType::FOR |
                TokenType::IF | TokenType::WHILE | TokenType::PRINT | TokenType::RETURN => {
                    return;
                }
                _ => {}
            }
            self.advance();
        }
    }
}

type ParseResult<T> = Result<T, ParseError>;


pub struct ParseError {
    pub token: Token,
    pub message: String,
}

impl ParseError {
    fn new(token: &Token, message: &str) -> Self {
        ParseError {
            token: token.to_owned(),
            message: message.to_owned(),
        }
    }
}
