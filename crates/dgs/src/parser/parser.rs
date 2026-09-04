use super::ast::{BinOp, Expr, UnaryOp};
use super::tokenizer::{Token, Tokenizer};

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let tokens = Tokenizer::new(input).tokenize();
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        let tok = self.advance();
        if &tok == expected {
            Ok(())
        } else {
            Err(format!("Expected {:?}, got {:?}", expected, tok))
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_term()?;
        while let Token::Plus | Token::Minus = self.peek() {
            let op = match self.advance() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => unreachable!(),
            };
            let right = self.parse_term()?;
            left = Expr::BinOp(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_power()?;
        while let Token::Star | Token::Slash = self.peek() {
            let op = match self.advance() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                _ => unreachable!(),
            };
            let right = self.parse_power()?;
            left = Expr::BinOp(op, Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, String> {
        let left = self.parse_unary()?;
        if self.peek() == &Token::Caret {
            self.advance();
            let right = self.parse_power()?;
            return Ok(Expr::BinOp(
                BinOp::Pow,
                Box::new(left),
                Box::new(right),
            ));
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, String> {
        if self.peek() == &Token::Minus {
            self.advance();
            let expr = self.parse_unary()?;
            return Ok(Expr::Unary(UnaryOp::Neg, Box::new(expr)));
        }
        self.parse_call()
    }

    fn parse_call(&mut self) -> Result<Expr, String> {
        if let Token::Var(name) = self.peek().clone() {
            if let Token::LParen = self.tokens.get(self.pos + 1).unwrap_or(&Token::Eof) {
                let name = name.clone();
                self.advance(); // consume variable name
                self.advance(); // consume '('
                let mut args = Vec::new();
                if self.peek() != &Token::RParen {
                    args.push(self.parse_expr()?);
                    while self.peek() == &Token::Comma {
                        self.advance(); // consume ','
                        args.push(self.parse_expr()?);
                    }
                }
                self.expect(&Token::RParen)?;
                return Ok(Expr::Call(name, args));
            }
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match self.peek().clone() {
            Token::Num(n) => {
                self.advance();
                Ok(Expr::Num(n))
            }
            Token::Var(name) => {
                self.advance();
                Ok(Expr::Var(name))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            tok => Err(format!("Unexpected token: {:?}", tok)),
        }
    }
}

pub fn parse(input: &str) -> Result<Expr, String> {
    let mut parser = Parser::new(input);
    let expr = parser.parse_expr()?;
    if parser.peek() != &Token::Eof {
        return Err(format!(
            "Unexpected token after expression: {:?}",
            parser.peek()
        ));
    }
    Ok(expr)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number() {
        assert_eq!(parse("42").unwrap(), Expr::Num(42.0));
    }

    #[test]
    fn test_addition() {
        assert_eq!(
            parse("1 + 2").unwrap(),
            Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Num(2.0))
            )
        );
    }

    #[test]
    fn test_precedence_mul_add() {
        assert_eq!(
            parse("1 + 2 * 3").unwrap(),
            Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::BinOp(
                    BinOp::Mul,
                    Box::new(Expr::Num(2.0)),
                    Box::new(Expr::Num(3.0))
                ))
            )
        );
    }

    #[test]
    fn test_power_right_assoc() {
        assert_eq!(
            parse("2 ^ 3 ^ 2").unwrap(),
            Expr::BinOp(
                BinOp::Pow,
                Box::new(Expr::Num(2.0)),
                Box::new(Expr::BinOp(
                    BinOp::Pow,
                    Box::new(Expr::Num(3.0)),
                    Box::new(Expr::Num(2.0))
                ))
            )
        );
    }

    #[test]
    fn test_unary_neg() {
        assert_eq!(
            parse("-5").unwrap(),
            Expr::Unary(UnaryOp::Neg, Box::new(Expr::Num(5.0)))
        );
    }

    #[test]
    fn test_function_call() {
        assert_eq!(
            parse("sin(x)").unwrap(),
            Expr::Call("sin".into(), vec![Expr::Var("x".into())])
        );
    }

    #[test]
    fn test_multi_arg_call() {
        assert_eq!(
            parse("f(a, b)").unwrap(),
            Expr::Call(
                "f".into(),
                vec![Expr::Var("a".into()), Expr::Var("b".into())]
            )
        );
    }

    #[test]
    fn test_paren_expr() {
        assert_eq!(
            parse("(1 + 2)").unwrap(),
            Expr::BinOp(
                BinOp::Add,
                Box::new(Expr::Num(1.0)),
                Box::new(Expr::Num(2.0))
            )
        );
    }

    #[test]
    fn test_complex_expr() {
        let expr = parse("x * (2 + sin(t))").unwrap();
        match expr {
            Expr::BinOp(BinOp::Mul, left, right) => {
                assert_eq!(*left, Expr::Var("x".into()));
                match *right {
                    Expr::BinOp(BinOp::Add, a, b) => {
                        assert_eq!(*a, Expr::Num(2.0));
                        assert_eq!(
                            *b,
                            Expr::Call(
                                "sin".into(),
                                vec![Expr::Var("t".into())]
                            )
                        );
                    }
                    _ => panic!("Expected Add"),
                }
            }
            _ => panic!("Expected Mul"),
        }
    }

    #[test]
    fn test_nested_calls() {
        assert_eq!(
            parse("sin(cos(x))").unwrap(),
            Expr::Call(
                "sin".into(),
                vec![Expr::Call("cos".into(), vec![Expr::Var("x".into())])]
            )
        );
    }

    #[test]
    fn test_unexpected_token() {
        assert!(parse("1 +").is_err());
    }

    #[test]
    fn test_trailing_tokens() {
        assert!(parse("1 + 2 extra").is_err());
    }

    #[test]
    fn test_leading_dot_number() {
        assert_eq!(parse(".5").unwrap(), Expr::Num(0.5));
    }

    #[test]
    fn test_decimal() {
        assert_eq!(parse("3.14").unwrap(), Expr::Num(3.14));
    }
}
