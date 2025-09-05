use crate::{
    expr::{Binary, Expression, Literal, Unary},
    scanner::{Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    fn next(&mut self) -> Option<Token> {
        self.current += 1;
        self.tokens.get(self.current - 1).cloned()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn expression(&mut self) -> Expression {
        return self.equality();
    }

    fn equality(&mut self) -> Expression {
        let mut expr = self.comparison();
        while self.match_token(TokenType::Equal, TokenType::BangEqual) {
            let op = self.next().unwrap();
            let right = self.comparison();
            expr = Expression::Binary(Binary::new(Box::new(expr), op, Box::new(right)));
        }
        expr
    }

    fn comparison(&mut self) -> Expression {
        let mut expr = self.term();

        while self.match_token(TokenType::Greater, TokenType::GreaterEqual)
            || self.match_token(TokenType::Less, TokenType::LessEqual)
        {
            let op = self.next().unwrap();
            let right = self.term();
            expr = Expression::Binary(Binary::new(Box::new(expr), op, Box::new(right)));
        }
        expr
    }

    fn term(&mut self) -> Expression {
        let mut expr = self.factor();

        while self.match_token(TokenType::Minus, TokenType::Plus) {
            let op = self.next().unwrap();
            let right = self.factor();
            expr = Expression::Binary(Binary::new(Box::new(expr), op, Box::new(right)));
        }
        expr
    }

    fn factor(&mut self) -> Expression {
        let mut expr = self.unary();

        while self.match_token(TokenType::Slash, TokenType::Star) {
            let op = self.next().unwrap();
            let right = self.unary();
            expr = Expression::Binary(Binary::new(Box::new(expr), op, Box::new(right)));
        }
        expr
    }

    fn unary(&mut self) -> Expression {
        if self.match_token(TokenType::Minus, TokenType::Minus) {
            let op = self.next().unwrap();
            let right = self.unary();
            Expression::Unary(Unary::new(op, Box::new(right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        if self.match_token(TokenType::LeftParen, TokenType::RightParen) {
            let _ = self.next().unwrap();
            let expr = self.expression();
            let _ = self.next().unwrap();
            expr
        } else if self.match_token(TokenType::Number, TokenType::String) {
            let value = self.next().unwrap().value.clone().unwrap();
            Expression::Literal(Literal::new(value))
        } else {
            panic!("Unexpected token");
        }
    }

    fn match_token(&mut self, type1: TokenType, type2: TokenType) -> bool {
        self.peek()
            .map(|t| t.token_type == type1 || t.token_type == type2)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        expr::AstPrinter,
        scanner::{LiteralValue, Scanner, ScannerResult, TokenType},
    };

    use super::*;

    #[test]
    fn test_parser() {
        let program = "123 + 45 * 67 + 4";
        let tokens = Scanner::new(program.to_string()).scan_tokens();

        let filtered = tokens
            .iter()
            .filter_map(|t| match t {
                ScannerResult::Token(t) => Some(t.clone()),
                _ => None,
            })
            .collect();

        let mut parser = Parser::new(filtered);
        let expr = parser.expression();
        assert_eq!(AstPrinter::print(&expr), "(+ (+ 123 (* 45 67)) 4)");
    }
}
