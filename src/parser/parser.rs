use scanner::{Literal, Token, TokenType};
use parser::expr::Expr;

macro_rules! binary {
    ($self:expr, $func:expr, $token_types:expr) => {{
        let mut expr = $func;

        while $self.match_token($token_types) {
            let operator = $self.previous().clone();
            let right = $func;
            expr = Box::new(Expr::Binary(expr, operator, right))
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

    fn expression(&mut self) -> Box<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<Expr> {
        binary!(
            self,
            self.comparison(),
            &[TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]
        )
    }

    fn comparison(&mut self) -> Box<Expr> {
        binary!(
            self,
            self.addition(),
            &[
                TokenType::GREATER,
                TokenType::GREATER_EQUAL,
                TokenType::LESS,
                TokenType::LESS_EQUAL
            ]
        )
    }

    fn addition(&mut self) -> Box<Expr> {
        binary!(
            self,
            self.multiplication(),
            &[TokenType::MINUS, TokenType::PLUS]
        )
    }

    fn multiplication(&mut self) -> Box<Expr> {
        binary!(self, self.unary(), &[TokenType::SLASH, TokenType::STAR])
    }

    fn unary(&mut self) -> Box<Expr> {
        if self.match_token(&[TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Box::new(Expr::Unary(operator, right));
        }
        self.primary()
    }

    fn primary(&mut self) -> Box<Expr> {
        if self.match_token(&[
            TokenType::FALSE,
            TokenType::TRUE,
            TokenType::NIL,
            TokenType::NUMBER,
            TokenType::STRING,
        ]) {
            return Box::new(Expr::Literal(self.previous().literal.clone()));
        };

        Box::new(Expr::Literal(Literal::Nil))

        // TODO: Need to write grouping code!!
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
}
