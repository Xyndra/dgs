#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Num(f64),
    Var(String),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Neg,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_expr() {
        assert_eq!(Expr::Num(3.14), Expr::Num(3.14));
    }

    #[test]
    fn test_binop_expr() {
        let expr = Expr::BinOp(
            BinOp::Add,
            Box::new(Expr::Num(1.0)),
            Box::new(Expr::Num(2.0)),
        );
        match expr {
            Expr::BinOp(op, l, r) => {
                assert_eq!(op, BinOp::Add);
                assert_eq!(*l, Expr::Num(1.0));
                assert_eq!(*r, Expr::Num(2.0));
            }
            _ => panic!("Expected BinOp"),
        }
    }

    #[test]
    fn test_unary_expr() {
        let expr = Expr::Unary(UnaryOp::Neg, Box::new(Expr::Num(5.0)));
        assert_eq!(
            expr,
            Expr::Unary(UnaryOp::Neg, Box::new(Expr::Num(5.0)))
        );
    }

    #[test]
    fn test_call_expr() {
        let expr = Expr::Call(
            "sin".to_string(),
            vec![Expr::Var("x".to_string())],
        );
        match expr {
            Expr::Call(name, args) => {
                assert_eq!(name, "sin");
                assert_eq!(args.len(), 1);
            }
            _ => panic!("Expected Call"),
        }
    }
}
