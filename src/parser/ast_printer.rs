use parser::ast::{Expr, ExprVisitor};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> Self {
        AstPrinter {}
    }

    // pub fn print(&mut self, ast: &AST) -> String {
    //     self.visit_expr(&ast.root)
    // }

    fn parenthesize(&mut self, name: &str, exprs: &[&Box<Expr>]) -> String {
        let mut expr_str = String::new();
        for expr in exprs {
            expr_str += " ";
            expr_str += &self.visit_expr(expr);
        }
        format!("({}{})", name, expr_str)
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Box<Expr>) -> String {
        match **expr {
            Expr::Literal(ref literal) => literal.to_string(),
            Expr::Binary(ref lhs, ref token, ref rhs) => {
                self.parenthesize(&token.lexeme, &[lhs, rhs])
            }
            Expr::Unary(ref token, ref e) => self.parenthesize(&token.lexeme, &[e]),
            Expr::Grouping(ref e) => self.parenthesize("group", &[e]),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use scanner::{Literal, Token, TokenType};

    #[test]
    fn test_printer() {
        let expr = Box::new(Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::MINUS, "-", Literal::Nil, 1),
                Box::new(Expr::Literal(Literal::Number(123.0))),
            )),
            Token::new(TokenType::STAR, "*", Literal::Nil, 1),
            Box::new(Expr::Grouping(
                Box::new(Expr::Literal(Literal::Number(45.67))),
            )),
        ));
        // let expr = Expr::Literal(Literal::Number(123.0));

        let mut printer = AstPrinter {};
        println!("{}", printer.print_expr(&expr));
    }
}
