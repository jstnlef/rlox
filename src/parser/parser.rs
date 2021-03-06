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
            tokens,
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
        let name = self.consume_token(TokenType::IDENTIFIER, "Expect variable name.")?.clone();

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
        if self.match_token(&[TokenType::FOR]) {
            return self.for_statement();
        }
        if self.match_token(&[TokenType::IF]) {
            return self.if_statement();
        }
        if self.match_token(&[TokenType::PRINT]) {
            return self.print_statement();
        }
        if self.match_token(&[TokenType::WHILE]) {
            return self.while_statement();
        }
        if self.match_token(&[TokenType::LEFT_BRACE]) {
            return Ok(Box::new(Stmt::Block(self.block()?)));
        }
        self.expression_statement()
    }

    fn for_statement(&mut self) -> ParseResult<Box<Stmt>> {
        self.consume_token(TokenType::LEFT_PAREN, "Expect '(' after 'for'.")?;

        let maybe_initializer = if self.match_token(&[TokenType::SEMICOLON]) {
            None
        } else if self.match_token(&[TokenType::VAR]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let maybe_condition = if !self.check(&TokenType::SEMICOLON) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume_token(TokenType::SEMICOLON, "Expect ';' after loop condition.")?;

        let maybe_increment = if !self.check(&TokenType::RIGHT_PAREN) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume_token(TokenType::RIGHT_PAREN, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        // Desugar the for loop into a while loop construct
        if let Some(increment) = maybe_increment {
            body = Box::new(Stmt::Block(vec![body, Box::new(Stmt::Expression(increment))]));
        }

        let condition = match maybe_condition {
            Some(condition) => condition,
            None => Box::new(Expr::Literal(Literal::Boolean(true)))
        };
        body = Box::new(Stmt::While(condition, body));

        if let Some(initializer) = maybe_initializer {
            body = Box::new(Stmt::Block(vec![initializer, body]));
        }

        Ok(body)
    }

    fn if_statement(&mut self) -> ParseResult<Box<Stmt>> {
        self.consume_token(TokenType::LEFT_PAREN, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume_token(TokenType::RIGHT_PAREN, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token(&[TokenType::ELSE]) {
            Some(self.statement()?)
        } else {
            None
        };

        Ok(Box::new(Stmt::If(condition, then_branch, else_branch)))
    }

    fn print_statement(&mut self) -> ParseResult<Box<Stmt>> {
        let value = self.expression()?;
        self.consume_token(
            TokenType::SEMICOLON,
            "Expect ';' after value.",
        )?;
        Ok(Box::new(Stmt::Print(value)))
    }

    fn while_statement(&mut self) -> ParseResult<Box<Stmt>> {
        self.consume_token(TokenType::LEFT_PAREN, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume_token(TokenType::RIGHT_PAREN, "Expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Box::new(Stmt::While(condition, body)))
    }

    fn block(&mut self) -> ParseResult<Vec<Box<Stmt>>> {
        let mut statements = Vec::new();
        while !self.check(&TokenType::RIGHT_BRACE) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }
        self.consume_token(TokenType::RIGHT_BRACE, "Expect '}' after block.")?;
        Ok(statements)
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
        self.assignment()
    }

    fn assignment(&mut self) -> ParseResult<Box<Expr>> {
        let expr = self.or()?;

        if self.match_token(&[TokenType::EQUAL]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expr::Variable(name) = *expr {
                return Ok(Box::new(Expr::Assign(name, value)));
            }

            return Err(self.error(&equals, "Invalid assignment target."));
        }
        Ok(expr)
    }

    fn or(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.and()?;

        while self.match_token(&[TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Box::new(Expr::Logical(expr, operator, right));
        }
        Ok(expr)
    }

    fn and(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.equality()?;

        while self.match_token(&[TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Box::new(Expr::Logical(expr, operator, right));
        }
        Ok(expr)
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
        self.call()
    }

    fn call(&mut self) -> ParseResult<Box<Expr>> {
        let mut expr = self.primary()?;

        loop {
            if self.match_token(&[TokenType::LEFT_PAREN]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Box<Expr>) -> ParseResult<Box<Expr>> {
        let mut arguments = Vec::new();
        if !self.check(&TokenType::RIGHT_PAREN) {
            loop {
                if arguments.len() >= 8 {
                    return Err(self.error(self.peek(), "Cannot have more than 8 arguments."));
                }
                arguments.push(self.expression()?);
                if !self.match_token(&[TokenType::COMMA]) {
                    break;
                }
            }
        }

        let paren = self.consume_token(TokenType::RIGHT_PAREN, "Expect ')' after arguments.")?;

        Ok(Box::new(Expr::Call(callee, paren.clone(), arguments)))
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
