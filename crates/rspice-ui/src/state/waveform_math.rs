//! Waveform Math Expression Parser
//!
//! Allows mathematical operations on waveform traces:
//! - Arithmetic: V(out) - V(in), I(R1) * 1000
//! - Functions: abs(), log(), sqrt(), db()
//! - Calculus: deriv(), integ()

use std::collections::HashMap;

/// Token types for expression parsing
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// A number literal
    Number(f64),
    /// A signal reference like V(out) or I(R1)
    Signal(String),
    /// Addition operator
    Plus,
    /// Subtraction operator
    Minus,
    /// Multiplication operator
    Multiply,
    /// Division operator
    Divide,
    /// Power operator
    Power,
    /// Left parenthesis
    LParen,
    /// Right parenthesis
    RParen,
    /// Comma (for function arguments)
    Comma,
    /// Function name
    Function(String),
}

/// Expression AST node
#[derive(Debug, Clone)]
pub enum Expr {
    /// Numeric constant
    Number(f64),
    /// Signal reference (name)
    Signal(String),
    /// Binary operation
    BinaryOp {
        op: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    /// Unary negation
    Negate(Box<Expr>),
    /// Function call
    FunctionCall { name: String, args: Vec<Expr> },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

/// Tokenize an expression string
pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ' ' | '\t' | '\n' => {
                chars.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                chars.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                chars.next();
            }
            '*' => {
                chars.next();
                if chars.peek() == Some(&'*') {
                    chars.next();
                    tokens.push(Token::Power);
                } else {
                    tokens.push(Token::Multiply);
                }
            }
            '/' => {
                tokens.push(Token::Divide);
                chars.next();
            }
            '^' => {
                tokens.push(Token::Power);
                chars.next();
            }
            '(' => {
                tokens.push(Token::LParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::RParen);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit()
                        || c == '.'
                        || c == 'e'
                        || c == 'E'
                        || (c == '-' && num_str.ends_with(|x| x == 'e' || x == 'E'))
                    {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let num: f64 = num_str
                    .parse()
                    .map_err(|_| format!("Invalid number: {}", num_str))?;
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut name = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        name.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Check if it's a signal reference like V(out) or I(R1)
                if (name.eq_ignore_ascii_case("v") || name.eq_ignore_ascii_case("i"))
                    && chars.peek() == Some(&'(')
                {
                    chars.next(); // consume '('
                    let mut signal_name = String::new();
                    while let Some(&c) = chars.peek() {
                        if c == ')' {
                            chars.next();
                            break;
                        }
                        signal_name.push(c);
                        chars.next();
                    }
                    let full_name = format!("{}({})", name.to_uppercase(), signal_name);
                    tokens.push(Token::Signal(full_name));
                } else if chars.peek() == Some(&'(') {
                    // It's a function call
                    tokens.push(Token::Function(name.to_lowercase()));
                } else {
                    // Bare identifier - treat as signal
                    tokens.push(Token::Signal(name));
                }
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    Ok(tokens)
}

/// Parse tokens into an AST
pub fn parse(tokens: &[Token]) -> Result<Expr, String> {
    let mut pos = 0;
    parse_expr(tokens, &mut pos)
}

fn parse_expr(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    parse_additive(tokens, pos)
}

fn parse_additive(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut left = parse_multiplicative(tokens, pos)?;

    while *pos < tokens.len() {
        match &tokens[*pos] {
            Token::Plus => {
                *pos += 1;
                let right = parse_multiplicative(tokens, pos)?;
                left = Expr::BinaryOp {
                    op: BinaryOperator::Add,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::Minus => {
                *pos += 1;
                let right = parse_multiplicative(tokens, pos)?;
                left = Expr::BinaryOp {
                    op: BinaryOperator::Subtract,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_multiplicative(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let mut left = parse_power(tokens, pos)?;

    while *pos < tokens.len() {
        match &tokens[*pos] {
            Token::Multiply => {
                *pos += 1;
                let right = parse_power(tokens, pos)?;
                left = Expr::BinaryOp {
                    op: BinaryOperator::Multiply,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            Token::Divide => {
                *pos += 1;
                let right = parse_power(tokens, pos)?;
                left = Expr::BinaryOp {
                    op: BinaryOperator::Divide,
                    left: Box::new(left),
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }

    Ok(left)
}

fn parse_power(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    let base = parse_unary(tokens, pos)?;

    if *pos < tokens.len() && tokens[*pos] == Token::Power {
        *pos += 1;
        let exp = parse_power(tokens, pos)?; // Right associative
        Ok(Expr::BinaryOp {
            op: BinaryOperator::Power,
            left: Box::new(base),
            right: Box::new(exp),
        })
    } else {
        Ok(base)
    }
}

fn parse_unary(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if *pos < tokens.len() && tokens[*pos] == Token::Minus {
        *pos += 1;
        let expr = parse_unary(tokens, pos)?;
        return Ok(Expr::Negate(Box::new(expr)));
    }
    parse_primary(tokens, pos)
}

fn parse_primary(tokens: &[Token], pos: &mut usize) -> Result<Expr, String> {
    if *pos >= tokens.len() {
        return Err("Unexpected end of expression".to_string());
    }

    match &tokens[*pos] {
        Token::Number(n) => {
            let n = *n;
            *pos += 1;
            Ok(Expr::Number(n))
        }
        Token::Signal(name) => {
            let name = name.clone();
            *pos += 1;
            Ok(Expr::Signal(name))
        }
        Token::Function(name) => {
            let name = name.clone();
            *pos += 1;

            // Expect '('
            if *pos >= tokens.len() || tokens[*pos] != Token::LParen {
                return Err(format!("Expected '(' after function {}", name));
            }
            *pos += 1;

            // Parse arguments
            let mut args = Vec::new();
            if *pos < tokens.len() && tokens[*pos] != Token::RParen {
                args.push(parse_expr(tokens, pos)?);
                while *pos < tokens.len() && tokens[*pos] == Token::Comma {
                    *pos += 1;
                    args.push(parse_expr(tokens, pos)?);
                }
            }

            // Expect ')'
            if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                return Err(format!("Expected ')' after function arguments"));
            }
            *pos += 1;

            Ok(Expr::FunctionCall { name, args })
        }
        Token::LParen => {
            *pos += 1;
            let expr = parse_expr(tokens, pos)?;
            if *pos >= tokens.len() || tokens[*pos] != Token::RParen {
                return Err("Expected ')'".to_string());
            }
            *pos += 1;
            Ok(expr)
        }
        _ => Err(format!("Unexpected token: {:?}", tokens[*pos])),
    }
}

/// Evaluate an expression against waveform data
///
/// # Arguments
/// * `expr` - The parsed expression
/// * `signals` - Map of signal names to (x, y) data
///
/// # Returns
/// (x, y) vectors for the result
pub fn evaluate(
    expr: &Expr,
    signals: &HashMap<String, (Vec<f64>, Vec<f64>)>,
) -> Result<(Vec<f64>, Vec<f64>), String> {
    match expr {
        Expr::Number(n) => {
            // Get x from first available signal
            if let Some((_, (x, _))) = signals.iter().next() {
                let y = vec![*n; x.len()];
                Ok((x.clone(), y))
            } else {
                Err("No signals available".to_string())
            }
        }
        Expr::Signal(name) => signals
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Signal not found: {}", name)),
        Expr::Negate(inner) => {
            let (x, y) = evaluate(inner, signals)?;
            let y_neg: Vec<f64> = y.iter().map(|v| -v).collect();
            Ok((x, y_neg))
        }
        Expr::BinaryOp { op, left, right } => {
            let (x1, y1) = evaluate(left, signals)?;
            let (_, y2) = evaluate(right, signals)?;

            // Ensure same length
            let len = x1.len().min(y2.len());
            let result: Vec<f64> = (0..len)
                .map(|i| match op {
                    BinaryOperator::Add => y1[i] + y2[i],
                    BinaryOperator::Subtract => y1[i] - y2[i],
                    BinaryOperator::Multiply => y1[i] * y2[i],
                    BinaryOperator::Divide => y1[i] / y2[i],
                    BinaryOperator::Power => y1[i].powf(y2[i]),
                })
                .collect();

            Ok((x1[..len].to_vec(), result))
        }
        Expr::FunctionCall { name, args } => evaluate_function(name, args, signals),
    }
}

fn evaluate_function(
    name: &str,
    args: &[Expr],
    signals: &HashMap<String, (Vec<f64>, Vec<f64>)>,
) -> Result<(Vec<f64>, Vec<f64>), String> {
    if args.is_empty() {
        return Err(format!("Function {} requires at least one argument", name));
    }

    let (x, y) = evaluate(&args[0], signals)?;

    let result_y: Vec<f64> = match name {
        "abs" => y.iter().map(|v| v.abs()).collect(),
        "sqrt" => y.iter().map(|v| v.sqrt()).collect(),
        "log" | "ln" => y.iter().map(|v| v.ln()).collect(),
        "log10" => y.iter().map(|v| v.log10()).collect(),
        "exp" => y.iter().map(|v| v.exp()).collect(),
        "sin" => y.iter().map(|v| v.sin()).collect(),
        "cos" => y.iter().map(|v| v.cos()).collect(),
        "tan" => y.iter().map(|v| v.tan()).collect(),
        "db" => y.iter().map(|v| 20.0 * v.abs().log10()).collect(),
        "deriv" | "d" => {
            // Numerical derivative
            if y.len() < 2 {
                return Err("Need at least 2 points for derivative".to_string());
            }
            let mut dy = vec![0.0; y.len()];
            for i in 1..y.len() {
                let dt = x[i] - x[i - 1];
                dy[i] = if dt > 0.0 {
                    (y[i] - y[i - 1]) / dt
                } else {
                    0.0
                };
            }
            dy[0] = dy[1]; // Forward fill first point
            dy
        }
        "integ" => {
            // Numerical integration (trapezoidal)
            let mut integral = vec![0.0; y.len()];
            for i in 1..y.len() {
                let dt = x[i] - x[i - 1];
                integral[i] = integral[i - 1] + (y[i] + y[i - 1]) * dt / 2.0;
            }
            integral
        }
        "avg" | "mean" => {
            let avg = y.iter().sum::<f64>() / y.len() as f64;
            vec![avg; y.len()]
        }
        "rms" => {
            let rms = (y.iter().map(|v| v * v).sum::<f64>() / y.len() as f64).sqrt();
            vec![rms; y.len()]
        }
        "min" => {
            let min = y.iter().cloned().fold(f64::INFINITY, f64::min);
            vec![min; y.len()]
        }
        "max" => {
            let max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            vec![max; y.len()]
        }
        _ => return Err(format!("Unknown function: {}", name)),
    };

    Ok((x, result_y))
}

/// Parse and evaluate an expression in one step
pub fn eval_expression(
    expr_str: &str,
    signals: &HashMap<String, (Vec<f64>, Vec<f64>)>,
) -> Result<(Vec<f64>, Vec<f64>), String> {
    let tokens = tokenize(expr_str)?;
    let ast = parse(&tokens)?;
    evaluate(&ast, signals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("V(out) - V(in)").unwrap();
        assert_eq!(tokens.len(), 3);
        assert!(matches!(&tokens[0], Token::Signal(s) if s == "V(out)"));
        assert!(matches!(&tokens[1], Token::Minus));
        assert!(matches!(&tokens[2], Token::Signal(s) if s == "V(in)"));
    }

    #[test]
    fn test_parse_simple() {
        let tokens = tokenize("1 + 2").unwrap();
        let expr = parse(&tokens).unwrap();
        assert!(matches!(
            expr,
            Expr::BinaryOp {
                op: BinaryOperator::Add,
                ..
            }
        ));
    }

    #[test]
    fn test_evaluate() {
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), (vec![0.0, 1.0], vec![5.0, 6.0]));
        signals.insert("V(in)".to_string(), (vec![0.0, 1.0], vec![1.0, 2.0]));

        let (_, y) = eval_expression("V(out) - V(in)", &signals).unwrap();
        assert_eq!(y, vec![4.0, 4.0]);
    }

    #[test]
    fn test_functions() {
        let mut signals = HashMap::new();
        signals.insert("V(out)".to_string(), (vec![0.0, 1.0], vec![4.0, 9.0]));

        let (_, y) = eval_expression("sqrt(V(out))", &signals).unwrap();
        assert_eq!(y, vec![2.0, 3.0]);
    }
}
