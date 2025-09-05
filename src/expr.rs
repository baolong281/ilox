use std::any::Any;

use crate::scanner::{LiteralValue, Token};

pub trait ExprVisitor {
    type Output;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output;
    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output;
    fn visit_literal(&mut self, expr: &Literal) -> Self::Output;
    fn visit_unary(&mut self, expr: &Unary) -> Self::Output;
}

pub struct AstPrinter;

impl AstPrinter {
    pub fn print(expr: &Expression) -> String {
        expr.accept(&mut AstPrinter)
    }
}

impl ExprVisitor for AstPrinter {
    type Output = String;

    fn visit_binary(&mut self, expr: &Binary) -> Self::Output {
        format!(
            "({} {} {})",
            expr.op,
            expr.left.accept(self),
            expr.right.accept(self)
        )
    }

    fn visit_grouping(&mut self, expr: &Grouping) -> Self::Output {
        format!("(group {})", expr.expr.accept(self))
    }

    fn visit_literal(&mut self, expr: &Literal) -> Self::Output {
        format!("{}", expr.value)
    }

    fn visit_unary(&mut self, expr: &Unary) -> Self::Output {
        format!("({} {})", expr.op, expr.right.accept(self))
    }
}

trait Visitable {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output;
}

pub enum Expression {
    Binary(Binary),
    Grouping(Grouping),
    Literal(Literal),
    Unary(Unary),
}

impl Visitable for Expression {
    fn accept<V: ExprVisitor>(&self, visitor: &mut V) -> V::Output {
        match self {
            Expression::Binary(expr) => visitor.visit_binary(expr),
            Expression::Grouping(expr) => visitor.visit_grouping(expr),
            Expression::Literal(expr) => visitor.visit_literal(expr),
            Expression::Unary(expr) => visitor.visit_unary(expr),
        }
    }
}

pub struct Binary {
    left: Box<Expression>,
    op: Token,
    right: Box<Expression>,
}

impl Binary {
    pub fn new(left: Box<Expression>, op: Token, right: Box<Expression>) -> Self {
        Self { left, op, right }
    }
}

pub struct Grouping {
    expr: Box<Expression>,
}

pub struct Literal {
    value: LiteralValue,
}

impl Literal {
    pub fn new(value: LiteralValue) -> Self {
        Self { value }
    }
}

pub struct Unary {
    op: Token,
    right: Box<Expression>,
}

impl Unary {
    pub fn new(op: Token, right: Box<Expression>) -> Self {
        Self { op, right }
    }
}

#[cfg(test)]
mod tests {
    use crate::scanner::TokenType;

    use super::*;

    #[test]
    fn test_printer() {
        let expr = Expression::Binary(Binary {
            left: Box::new(Expression::Unary(Unary {
                op: Token::new(TokenType::Minus, String::from("-"), 0, None),
                right: Box::new(Expression::Literal(Literal {
                    value: LiteralValue::Number(123.0),
                })),
            })),
            op: Token::new(TokenType::Star, String::from("*"), 0, None),
            right: Box::new(Expression::Grouping(Grouping {
                expr: Box::new(Expression::Literal(Literal {
                    value: LiteralValue::Number(45.67), // Changed from Int(2) to Number(45.67)
                })),
            })),
        });

        // The expected output should match what the Java version would produce
        assert_eq!(AstPrinter::print(&expr), "(* (- 123) (group 45.67))");
    }
}
