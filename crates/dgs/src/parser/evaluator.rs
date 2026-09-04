use super::ast::{BinOp, Expr, UnaryOp};

pub fn evaluate(expr: &Expr, vars: &[(String, f64)]) -> Result<f64, String> {
    match expr {
        Expr::Num(n) => Ok(*n),
        Expr::Var(name) => {
            let lower = name.to_lowercase();
            match lower.as_str() {
                "pi" => Ok(std::f64::consts::PI),
                "e" => Ok(std::f64::consts::E),
                _ => {
                    for (vname, val) in vars {
                        if vname.to_lowercase() == lower {
                            return Ok(*val);
                        }
                    }
                    Err(format!("Unknown variable: {}", name))
                }
            }
        }
        Expr::BinOp(op, left, right) => {
            let l = evaluate(left, vars)?;
            let r = evaluate(right, vars)?;
            match op {
                BinOp::Add => Ok(l + r),
                BinOp::Sub => Ok(l - r),
                BinOp::Mul => Ok(l * r),
                BinOp::Div => {
                    if r == 0.0 {
                        Err("Division by zero".to_string())
                    } else {
                        Ok(l / r)
                    }
                }
                BinOp::Pow => Ok(l.powf(r)),
            }
        }
        Expr::Unary(op, inner) => {
            let val = evaluate(inner, vars)?;
            match op {
                UnaryOp::Neg => Ok(-val),
            }
        }
        Expr::Call(name, args) => {
            let mut evaluated_args = Vec::with_capacity(args.len());
            for arg in args {
                evaluated_args.push(evaluate(arg, vars)?);
            }
            call_function(name, &evaluated_args)
        }
    }
}

fn call_function(name: &str, args: &[f64]) -> Result<f64, String> {
    let lower = name.to_lowercase();
    match lower.as_str() {
        "sin" => {
            check_arity(name, args, 1)?;
            Ok(args[0].sin())
        }
        "cos" => {
            check_arity(name, args, 1)?;
            Ok(args[0].cos())
        }
        "tan" => {
            check_arity(name, args, 1)?;
            Ok(args[0].tan())
        }
        "asin" => {
            check_arity(name, args, 1)?;
            Ok(args[0].asin())
        }
        "acos" => {
            check_arity(name, args, 1)?;
            Ok(args[0].acos())
        }
        "atan" => {
            check_arity(name, args, 1)?;
            Ok(args[0].atan())
        }
        "sqrt" => {
            check_arity(name, args, 1)?;
            Ok(args[0].sqrt())
        }
        "abs" => {
            check_arity(name, args, 1)?;
            Ok(args[0].abs())
        }
        "log" => {
            check_arity(name, args, 1)?;
            if args[0] <= 0.0 {
                return Err("log: argument must be positive".to_string());
            }
            Ok(args[0].log10())
        }
        "ln" => {
            check_arity(name, args, 1)?;
            if args[0] <= 0.0 {
                return Err("ln: argument must be positive".to_string());
            }
            Ok(args[0].ln())
        }
        "exp" => {
            check_arity(name, args, 1)?;
            Ok(args[0].exp())
        }
        "ceil" => {
            check_arity(name, args, 1)?;
            Ok(args[0].ceil())
        }
        "floor" => {
            check_arity(name, args, 1)?;
            Ok(args[0].floor())
        }
        "round" => {
            check_arity(name, args, 1)?;
            Ok(args[0].round())
        }
        _ => Err(format!("Unknown function: {}", name)),
    }
}

fn check_arity(name: &str, args: &[f64], expected: usize) -> Result<(), String> {
    if args.len() != expected {
        Err(format!(
            "{}: expected {} argument(s), got {}",
            name,
            expected,
            args.len()
        ))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::parser::parse;

    fn eval_str(expr: &str) -> Result<f64, String> {
        let ast = parse(expr)?;
        evaluate(&ast, &[])
    }

    #[test]
    fn test_num() {
        assert_eq!(eval_str("42").unwrap(), 42.0);
    }

    #[test]
    fn test_add() {
        assert_eq!(eval_str("1 + 2").unwrap(), 3.0);
    }

    #[test]
    fn test_mul_precedence() {
        assert_eq!(eval_str("2 + 3 * 4").unwrap(), 14.0);
    }

    #[test]
    fn test_paren() {
        assert_eq!(eval_str("(2 + 3) * 4").unwrap(), 20.0);
    }

    #[test]
    fn test_power() {
        assert!((eval_str("2 ^ 3").unwrap() - 8.0).abs() < 1e-10);
    }

    #[test]
    fn test_unary_neg() {
        assert_eq!(eval_str("-5").unwrap(), -5.0);
    }

    #[test]
    fn test_double_neg() {
        assert_eq!(eval_str("--5").unwrap(), 5.0);
    }

    #[test]
    fn test_variable() {
        let ast = parse("x + 1").unwrap();
        assert_eq!(evaluate(&ast, &[("x".into(), 4.0)]).unwrap(), 5.0);
    }

    #[test]
    fn test_pi() {
        let result = eval_str("pi").unwrap();
        assert!((result - std::f64::consts::PI).abs() < 1e-10);
    }

    #[test]
    fn test_e() {
        let result = eval_str("e").unwrap();
        assert!((result - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_sin() {
        let result = eval_str("sin(0)").unwrap();
        assert!(result.abs() < 1e-10);
    }

    #[test]
    fn test_cos() {
        let result = eval_str("cos(0)").unwrap();
        assert!((result - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_sqrt() {
        assert!((eval_str("sqrt(9)").unwrap() - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_log() {
        assert!((eval_str("log(100)").unwrap() - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_ln() {
        assert!((eval_str("ln(e)").unwrap() - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_abs() {
        assert_eq!(eval_str("abs(-5)").unwrap(), 5.0);
    }

    #[test]
    fn test_ceil() {
        assert_eq!(eval_str("ceil(2.3)").unwrap(), 3.0);
    }

    #[test]
    fn test_floor() {
        assert_eq!(eval_str("floor(2.7)").unwrap(), 2.0);
    }

    #[test]
    fn test_round() {
        assert_eq!(eval_str("round(2.5)").unwrap(), 3.0);
    }

    #[test]
    fn test_exp() {
        assert!((eval_str("exp(1)").unwrap() - std::f64::consts::E).abs() < 1e-10);
    }

    #[test]
    fn test_div_by_zero() {
        assert!(eval_str("1 / 0").is_err());
    }

    #[test]
    fn test_unknown_var() {
        assert!(eval_str("x").is_err());
    }

    #[test]
    fn test_unknown_func() {
        assert!(eval_str("foo(1)").is_err());
    }

    #[test]
    fn test_wrong_arity() {
        assert!(eval_str("sin(1, 2)").is_err());
    }
}
