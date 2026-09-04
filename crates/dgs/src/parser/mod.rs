//! Dependency-free math expression parser and evaluator.
//!
//! Tokenizer, recursive-descent parser, AST, and evaluator for the expression
//! DSL used by curves (e.g. `"sin(x) / x"` with constants `pi`/`e`).

pub mod ast;
pub mod evaluator;
pub mod parser;
pub mod tokenizer;

pub use ast::Expr;
pub use evaluator::evaluate;
pub use parser::parse;

pub fn eval(expr: &ast::Expr, vars: &[(&str, f64)]) -> Result<f64, String> {
    let converted: Vec<(String, f64)> = vars.iter().map(|(k, v)| (k.to_string(), *v)).collect();
    evaluate(expr, &converted)
}

pub fn eval_str(expr: &str, vars: &[(&str, f64)]) -> Result<f64, String> {
    let ast = parse(expr)?;
    eval(&ast, vars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_and_eval() {
        let expr = parse("2 + 3 * 4").unwrap();
        assert_eq!(eval(&expr, &[]).unwrap(), 14.0);
    }

    #[test]
    fn test_eval_str() {
        assert_eq!(eval_str("2 + 3", &[]).unwrap(), 5.0);
    }

    #[test]
    fn test_eval_str_with_vars() {
        assert_eq!(eval_str("x * 2", &[("x", 5.0)]).unwrap(), 10.0);
    }

    #[test]
    fn test_eval_str_complex() {
        let result = eval_str("sin(pi / 2)", &[]).unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_round_trip() {
        let ast = parse("sqrt(x^2 + y^2)").unwrap();
        let result = eval(&ast, &[("x", 3.0), ("y", 4.0)]).unwrap();
        assert!((result - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_error() {
        assert!(parse("1 +").is_err());
    }

    #[test]
    fn test_eval_error() {
        assert!(eval_str("1 / 0", &[]).is_err());
    }
}
