//! Calculator core: parsing and evaluating simple binary expressions
//!
//! The library exposes 'parse_expression 'and 'evaluate' for testing

use std::fmt;
use std::num::ParseFloatError;

/// Supported binary operations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

/// Error type for parse/eval issues
#[derive(Debug)]
pub enum CalcError {
    ParseFloat(ParseFloatError),
    UnknownOperator(String),
    WrongArity,
    DivisionByZero,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::ParseFloat(e) => write!(f, "number parse error: {}", e),
            CalcError::UnknownOperator(op) => write!(f, "unknown operator: {}", op),
            CalcError::WrongArity => write!(f, "wrong number of operands"),
            CalcError::DivisionByZero => write!(f, "division by zero"),
        }
    }
}
impl std::error::Error for CalcError {}

impl From<ParseFloatError> for CalcError {
    fn from(e: ParseFloatError) -> Self {
        CalcError::ParseFloat(e)
    }
}

/// Evaluate an operation on two f64 operands
pub fn evaluate(op: Op, a: f64, b: f64) -> Result<f64, CalcError> {
    match op {
        Op::Add => Ok(a + b),
        Op::Sub => Ok(a - b),
        Op::Mul => Ok(a * b),
        Op::Div => {
            if b == 0.0 {
                Err(CalcError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        Op::Pow => Ok(a.powf(b)),
    }
}

/// Try to parse operator token
fn parse_op(token: &str) -> Result<Op, CalcError> {
    match token {
        "+" | "add" => Ok(Op::Add),
        "-" | "sub" => Ok(Op::Sub),
        "*" | "mul" => Ok(Op::Mul),
        "/" | "div" => Ok(Op::Div),
        "^" | "pow" => Ok(Op::Pow),
        other => Err(CalcError::UnknownOperator(other.to_string())),
    }
}

/// Parse a tokenized expression into (Op, operand1, operand2)
pub fn parse_expression(tokens: &[&str]) -> Result<(Op, f64, f64), CalcError> {
    match tokens {
        [op, a, b] => {
            let op = parse_op(*op)?;
            let a = a.parse::<f64>()?;
            let b = b.parse::<f64>()?;
            Ok((op, a, b))
        }
        _ => Err(CalcError::WrongArity),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_prefix() {
        let tokens = ["add", "2", "3"];
        let (op, a, b) = parse_expression(&tokens).unwrap();
        assert_eq!(op, Op::Add);
        assert_eq!(evaluate(op, a, b).unwrap(), 5.0);
    }

    #[test]
    fn test_mul_infix() {
        let tokens = ["*", "4", "2.5"];
        let (op, a, b) = parse_expression(&tokens).unwrap();
        assert_eq!(op, Op::Mul);
        assert!((evaluate(op, a, b).unwrap() - 10.0).abs() < 1e-12);
    }

    #[test]
    fn test_div_by_zero() {
        let tokens = ["/", "10", "0"];
        let (op, a, b) = parse_expression(&tokens).unwrap();
        let r = evaluate(op, a, b);
        println!("{:?}", r);
        assert!(matches!(evaluate(op, a, b), Err(CalcError::DivisionByZero)));
    }

    #[test]
    fn test_unknown_operator() {
        let tokens = ["%", "10", "3"];
        assert!(matches!(
            parse_expression(&tokens),
            Err(CalcError::UnknownOperator(_))
        ));
    }
}
