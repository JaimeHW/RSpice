//! LAPLACE controlled-source synthesis.
//!
//! `E1 out 0 LAPLACE {expr} = {N(s)/D(s)}` compiles, at parse time, into an
//! exact state-space realization built from existing, fully-validated
//! elements: one grounded capacitor per state plus behavioral sources for
//! the state derivatives and the output. Every analysis then handles the
//! transfer function with no dedicated device code:
//!
//! - **DC**: capacitors open, the state equations collapse to the s = 0
//!   algebraic system, so the output settles to `H(0)·u` by construction
//!   (and an ideal integrator correctly has no DC operating point).
//! - **AC**: the behavioral linearization of the realization *is* `H(jω)`.
//! - **Transient**: states integrate under the trap/Gear LTE controller —
//!   variable-step accuracy a fixed bilinear discretization cannot match.
//! - **PSS/HB/noise**: inherit the realization like any other circuit.
//!
//! A multiplicative `exp(-s*T)` factor is a pure transport delay: the input
//! first crosses a matched lossless line of delay `T`, whose history and
//! breakpoints the line keeps, and the rational part acts on its far end.
//!
//! The rational is parsed exactly — no sampling or fitting — by evaluating
//! the brace expression over the field of rational functions in `s`. The
//! denominator is then made monic and the realization is frequency-scaled
//! by `ω₀ = a₀^(1/n)` (the geometric mean of the pole magnitudes) so state
//! magnitudes and stamped conductances stay O(1) wherever the poles live.
//!
//! An expression with a factor that is not rational in `s`, such as
//! `1/sqrt(1+s*tau)`, has no such realization. As QSPICE does, small-signal
//! analyses evaluate it exactly at `s = jω` and the operating point takes its
//! value at `s = 0`; analyses that need its response in time refuse it.

use super::{DelayTime, Element, ElementKind, ParseError, SourceMultiplicity, delay_line};
use crate::Value;
use crate::netlist::{ElementProvenance, FrequencyResponse};
use num_complex::Complex64;

//=============================================================================
// Polynomials and rational functions in s
//=============================================================================

/// Polynomial in `s`, ascending coefficients: `c[0] + c[1]·s + c[2]·s² …`.
type Poly = Vec<Value>;

fn poly_trim(mut p: Poly) -> Poly {
    while p.len() > 1 && p.last() == Some(&0.0) {
        p.pop();
    }
    p
}

fn poly_is_zero(p: &Poly) -> bool {
    p.iter().all(|c| *c == 0.0)
}

fn poly_add(a: &Poly, b: &Poly) -> Poly {
    let n = a.len().max(b.len());
    poly_trim(
        (0..n)
            .map(|i| a.get(i).unwrap_or(&0.0) + b.get(i).unwrap_or(&0.0))
            .collect(),
    )
}

fn poly_neg(a: &Poly) -> Poly {
    a.iter().map(|c| -c).collect()
}

fn poly_mul(a: &Poly, b: &Poly) -> Poly {
    let mut out = vec![0.0; a.len() + b.len() - 1];
    for (i, ca) in a.iter().enumerate() {
        if *ca == 0.0 {
            continue;
        }
        for (j, cb) in b.iter().enumerate() {
            out[i + j] += ca * cb;
        }
    }
    poly_trim(out)
}

/// Strip floating-point dust left by cancellation (e.g. `(1+s) - 1`):
/// coefficients below 1e-12 of the largest magnitude become exact zeros.
fn poly_clean(p: Poly) -> Poly {
    let scale = p.iter().fold(0.0f64, |m, c| m.max(c.abs()));
    if scale == 0.0 {
        return vec![0.0];
    }
    poly_trim(
        p.into_iter()
            .map(|c| if c.abs() < 1e-12 * scale { 0.0 } else { c })
            .collect(),
    )
}

/// Rational function `num/den` in `s` times the delay `e^(-s·delay)`.
#[derive(Debug, Clone)]
struct Rational {
    num: Poly,
    den: Poly,
    delay: Value,
}

impl Rational {
    fn constant(v: Value) -> Self {
        Self {
            num: vec![v],
            den: vec![1.0],
            delay: 0.0,
        }
    }

    fn s() -> Self {
        Self {
            num: vec![0.0, 1.0],
            den: vec![1.0],
            delay: 0.0,
        }
    }

    /// The value of a rational that does not depend on `s`.
    fn constant_value(&self) -> Option<Value> {
        let num = poly_trim(self.num.clone());
        let den = poly_trim(self.den.clone());
        (num.len() == 1 && den.len() == 1 && self.delay == 0.0).then(|| num[0] / den[0])
    }

    /// Whether the rational is `c - s·T`, the argument a delay factor takes.
    fn is_affine(&self) -> bool {
        let num = poly_trim(self.num.clone());
        let den = poly_trim(self.den.clone());
        den.len() == 1 && num.len() <= 2 && self.delay == 0.0
    }

    /// `exp(argument)`, which is realizable only for an argument `c - s·T`.
    fn exp(argument: &Self) -> Result<Self, String> {
        let num = poly_trim(argument.num.clone());
        let den = poly_trim(argument.den.clone());
        let affine = den.len() == 1 && num.len() <= 2 && argument.delay == 0.0;
        let delay = -num.get(1).copied().unwrap_or(0.0) / den[0];
        if !(affine && delay.is_finite() && delay >= 0.0) {
            return Err(
                "exp() in a LAPLACE expression must be a delay exp(-s*T) with T >= 0; \
                 other irrational transfer functions cannot be realized"
                    .to_string(),
            );
        }
        Ok(Self {
            num: vec![(num[0] / den[0]).exp()],
            den: vec![1.0],
            delay,
        })
    }

    /// The common delay of two summed terms; a zero term takes the other's.
    fn sum_delay(&self, other: &Self) -> Result<Value, String> {
        if poly_is_zero(&self.num) {
            Ok(other.delay)
        } else if poly_is_zero(&other.num) || self.delay == other.delay {
            Ok(self.delay)
        } else {
            Err(
                "LAPLACE terms with different exp(-s*T) delays cannot be realized; \
                 factor the delay out of the sum"
                    .to_string(),
            )
        }
    }

    fn add(&self, other: &Self) -> Result<Self, String> {
        Ok(Self {
            num: poly_add(
                &poly_mul(&self.num, &other.den),
                &poly_mul(&other.num, &self.den),
            ),
            den: poly_mul(&self.den, &other.den),
            delay: self.sum_delay(other)?,
        })
    }

    fn sub(&self, other: &Self) -> Result<Self, String> {
        self.add(&Self {
            num: poly_neg(&other.num),
            den: other.den.clone(),
            delay: other.delay,
        })
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            num: poly_mul(&self.num, &other.num),
            den: poly_mul(&self.den, &other.den),
            delay: self.delay + other.delay,
        }
    }

    fn div(&self, other: &Self) -> Result<Self, String> {
        if poly_is_zero(&other.num) {
            return Err("division by zero inside LAPLACE expression".to_string());
        }
        Ok(Self {
            num: poly_mul(&self.num, &other.den),
            den: poly_mul(&self.den, &other.num),
            delay: self.delay - other.delay,
        })
    }

    fn powi(&self, exp: u32) -> Self {
        let mut out = Self::constant(1.0);
        for _ in 0..exp {
            out = out.mul(self);
        }
        out
    }
}

//=============================================================================
// Rational-expression parser (numbers, s, + - * / ^, parentheses)
//=============================================================================

#[derive(Debug, Clone, PartialEq)]
enum RatTok {
    Num(Value),
    S,
    Plus,
    Minus,
    Star,
    Slash,
    Caret,
    LParen,
    RParen,
    Comma,
    /// A named function, applied to the parenthesized arguments that follow.
    Function(String),
}

fn suffix_multiplier(suffix: &str) -> Option<Value> {
    match suffix.to_ascii_lowercase().as_str() {
        "" => Some(1.0),
        "t" => Some(1e12),
        "g" => Some(1e9),
        "meg" => Some(1e6),
        "k" => Some(1e3),
        "m" => Some(1e-3),
        "u" => Some(1e-6),
        "n" => Some(1e-9),
        "p" => Some(1e-12),
        "f" => Some(1e-15),
        _ => None,
    }
}

/// The tokens of `text`, and the characters each was read from.
fn tokenize_rational(
    parameter: &dyn Fn(&str) -> Option<Value>,
    text: &str,
) -> Result<(Vec<RatTok>, Vec<(usize, usize)>), String> {
    let mut tokens = Vec::new();
    let mut spans = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        let start = i;
        let read = tokens.len();
        match c {
            ' ' | '\t' | '{' | '}' => i += 1,
            '+' => {
                tokens.push(RatTok::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(RatTok::Minus);
                i += 1;
            }
            // `**` is the power operator, as `^` is.
            '*' if chars.get(i + 1) == Some(&'*') => {
                tokens.push(RatTok::Caret);
                i += 2;
            }
            '*' => {
                tokens.push(RatTok::Star);
                i += 1;
            }
            '/' => {
                tokens.push(RatTok::Slash);
                i += 1;
            }
            '^' => {
                tokens.push(RatTok::Caret);
                i += 1;
            }
            '(' => {
                tokens.push(RatTok::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(RatTok::RParen);
                i += 1;
            }
            ',' => {
                tokens.push(RatTok::Comma);
                i += 1;
            }
            's' | 'S' if !chars.get(i + 1).is_some_and(|c| c.is_ascii_alphanumeric()) => {
                tokens.push(RatTok::S);
                i += 1;
            }
            c if c.is_ascii_alphabetic() => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let name: String = chars[start..i].iter().collect();
                if chars.get(i) == Some(&'(') {
                    tokens.push(RatTok::Function(name));
                } else if let Some(value) = parameter(&name) {
                    // A parameter stands for its value, as LTspice and QSPICE
                    // read one.
                    tokens.push(RatTok::Num(value));
                } else {
                    return Err(format!(
                        "unknown symbol '{name}' in LAPLACE expression \
                         (allowed: numbers, parameters, s, + - * / ^, parentheses and functions)"
                    ));
                }
            }
            c if c.is_ascii_digit() || c == '.' => {
                // Number: digits, optional fraction, optional exponent,
                // optional SPICE magnitude suffix (k, meg, u, ...).
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                if i < chars.len() && (chars[i] == 'e' || chars[i] == 'E') {
                    let mantissa_end = i;
                    let mut j = i + 1;
                    if j < chars.len() && (chars[j] == '+' || chars[j] == '-') {
                        j += 1;
                    }
                    if j < chars.len() && chars[j].is_ascii_digit() {
                        while j < chars.len() && chars[j].is_ascii_digit() {
                            j += 1;
                        }
                        i = j;
                    } else {
                        i = mantissa_end; // 'e' begins a suffix word, not an exponent
                    }
                }
                let number_end = i;
                let mut suffix = String::new();
                while i < chars.len() && chars[i].is_ascii_alphabetic() {
                    suffix.push(chars[i]);
                    i += 1;
                }
                let body: String = chars[start..number_end].iter().collect();
                let base: Value = body
                    .parse()
                    .map_err(|_| format!("invalid number '{body}' in LAPLACE expression"))?;
                let mult = suffix_multiplier(&suffix).ok_or_else(|| {
                    format!("unknown magnitude suffix '{suffix}' in LAPLACE expression")
                })?;
                tokens.push(RatTok::Num(crate::spice_number::scaled_spice_value(
                    &body, base, mult,
                )));
            }
            other => {
                return Err(format!(
                    "unsupported symbol '{other}' in LAPLACE expression \
                     (allowed: numbers, s, + - * / ^, parentheses and functions)"
                ));
            }
        }
        if tokens.len() > read {
            spans.push((start, i));
        }
    }
    Ok((tokens, spans))
}

/// A function a LAPLACE expression may apply. Each is read over the complex
/// numbers on its principal branch, as QSPICE and LTspice read it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TransferFunction {
    Sqrt,
    Exp,
    Ln,
    Log10,
    Abs,
    Pow,
    Sin,
    Cos,
    Tan,
    Sinh,
    Cosh,
    Tanh,
}

impl TransferFunction {
    const NAMES: &'static str =
        "sqrt, exp, ln, log, log10, abs, pow, sin, cos, tan, sinh, cosh and tanh";

    fn named(name: &str) -> Option<Self> {
        Some(match name.to_ascii_lowercase().as_str() {
            "sqrt" => Self::Sqrt,
            "exp" => Self::Exp,
            "ln" | "log" => Self::Ln,
            "log10" => Self::Log10,
            "abs" => Self::Abs,
            "pow" => Self::Pow,
            "sin" => Self::Sin,
            "cos" => Self::Cos,
            "tan" => Self::Tan,
            "sinh" => Self::Sinh,
            "cosh" => Self::Cosh,
            "tanh" => Self::Tanh,
            _ => return None,
        })
    }

    const fn arity(self) -> usize {
        if matches!(self, Self::Pow) { 2 } else { 1 }
    }
}

/// A LAPLACE expression as written: numbers, `s`, arithmetic, powers and
/// functions. A power and a function call keep the text they were written
/// as, to name a factor that has no rational realization.
#[derive(Debug, Clone)]
enum Term {
    Number(Value),
    S,
    Negate(Box<Term>),
    Sum(Box<Term>, Box<Term>),
    Difference(Box<Term>, Box<Term>),
    Product(Box<Term>, Box<Term>),
    Quotient(Box<Term>, Box<Term>),
    Power(Box<Term>, Box<Term>, String),
    Call(TransferFunction, Vec<Term>, String),
}

/// Why a term has no state-space realization.
enum Unrealizable {
    /// A factor, as written, that is not rational in `s`; the expression is
    /// still defined at every `s = jω`.
    Factor(String),
    /// The expression is not a transfer at all.
    Error(String),
}

/// `base ** exponent` on the principal branch; a small integer exponent is
/// an exact product.
fn complex_power(base: Complex64, exponent: Complex64) -> Complex64 {
    if exponent.im != 0.0 {
        return base.powc(exponent);
    }
    let n = exponent.re;
    if n.fract() == 0.0 && n.abs() <= 64.0 {
        base.powi(n as i32)
    } else {
        base.powf(n)
    }
}

impl Term {
    /// The value at the complex frequency `s`.
    fn at(&self, s: Complex64) -> Complex64 {
        match self {
            Self::Number(value) => Complex64::new(*value, 0.0),
            Self::S => s,
            Self::Negate(term) => -term.at(s),
            Self::Sum(a, b) => a.at(s) + b.at(s),
            Self::Difference(a, b) => a.at(s) - b.at(s),
            Self::Product(a, b) => a.at(s) * b.at(s),
            Self::Quotient(a, b) => a.at(s) / b.at(s),
            Self::Power(base, exponent, _) => complex_power(base.at(s), exponent.at(s)),
            Self::Call(function, args, _) => {
                let x = args[0].at(s);
                match function {
                    TransferFunction::Sqrt => x.sqrt(),
                    TransferFunction::Exp => x.exp(),
                    TransferFunction::Ln => x.ln(),
                    TransferFunction::Log10 => x.log10(),
                    TransferFunction::Abs => Complex64::new(x.norm(), 0.0),
                    TransferFunction::Pow => complex_power(x, args[1].at(s)),
                    TransferFunction::Sin => x.sin(),
                    TransferFunction::Cos => x.cos(),
                    TransferFunction::Tan => x.tan(),
                    TransferFunction::Sinh => x.sinh(),
                    TransferFunction::Cosh => x.cosh(),
                    TransferFunction::Tanh => x.tanh(),
                }
            }
        }
    }

    /// The exact rational function in `s` the term is, times a delay.
    fn rational(&self) -> Result<Rational, Unrealizable> {
        use Unrealizable::Error;
        match self {
            Self::Number(value) => Ok(Rational::constant(*value)),
            Self::S => Ok(Rational::s()),
            Self::Negate(term) => Rational::constant(0.0)
                .sub(&term.rational()?)
                .map_err(Error),
            Self::Sum(a, b) => {
                let (a, b) = Self::both(a, b)?;
                a.add(&b).map_err(Error)
            }
            Self::Difference(a, b) => {
                let (a, b) = Self::both(a, b)?;
                a.sub(&b).map_err(Error)
            }
            Self::Product(a, b) => {
                let (a, b) = Self::both(a, b)?;
                Ok(a.mul(&b))
            }
            Self::Quotient(a, b) => {
                let (a, b) = Self::both(a, b)?;
                a.div(&b).map_err(Error)
            }
            Self::Power(base, exponent, text) => {
                let (base, exponent) = Self::both(base, exponent)?;
                match exponent.constant_value() {
                    Some(n) if n.fract() == 0.0 && n.abs() <= 16.0 => {
                        let power = base.powi(n.abs() as u32);
                        if n < 0.0 {
                            Rational::constant(1.0).div(&power).map_err(Error)
                        } else {
                            Ok(power)
                        }
                    }
                    _ => Err(Unrealizable::Factor(text.clone())),
                }
            }
            Self::Call(TransferFunction::Exp, args, text) => {
                let argument = args[0].rational()?;
                if !argument.is_affine() {
                    return Err(Unrealizable::Factor(text.clone()));
                }
                Rational::exp(&argument).map_err(Error)
            }
            Self::Call(_, _, text) => Err(Unrealizable::Factor(text.clone())),
        }
    }

    /// Both operands as rationals; a factor with no realization in either is
    /// named before any other failure.
    fn both(a: &Self, b: &Self) -> Result<(Rational, Rational), Unrealizable> {
        match (a.rational(), b.rational()) {
            (Ok(a), Ok(b)) => Ok((a, b)),
            (Err(factor @ Unrealizable::Factor(_)), _)
            | (_, Err(factor @ Unrealizable::Factor(_))) => Err(factor),
            (Err(error), _) | (_, Err(error)) => Err(error),
        }
    }

    /// Structure and numbers bit for bit.
    fn same(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a.to_bits() == b.to_bits(),
            (Self::S, Self::S) => true,
            (Self::Negate(a), Self::Negate(b)) => a.same(b),
            (Self::Sum(a, b), Self::Sum(c, d))
            | (Self::Difference(a, b), Self::Difference(c, d))
            | (Self::Product(a, b), Self::Product(c, d))
            | (Self::Quotient(a, b), Self::Quotient(c, d))
            | (Self::Power(a, b, _), Self::Power(c, d, _)) => a.same(c) && b.same(d),
            (Self::Call(f, a, _), Self::Call(g, b, _)) => {
                f == g && a.len() == b.len() && a.iter().zip(b).all(|(a, b)| a.same(b))
            }
            _ => false,
        }
    }
}

/// A LAPLACE expression with a factor that is not rational in `s`, such as
/// `1/sqrt(1+s*tau)`. It has no state-space realization, so it is known only
/// where small-signal analyses ask for it: exactly at `s = jω`.
#[derive(Debug, Clone)]
pub(crate) struct TransferExpression {
    term: Term,
    /// The first factor with no rational realization, as written.
    factor: String,
}

impl TransferExpression {
    /// The transfer at the complex frequency `s`.
    pub(crate) fn at(&self, s: Complex64) -> Complex64 {
        self.term.at(s)
    }

    /// The first factor with no rational realization, as written.
    pub(crate) fn factor(&self) -> &str {
        &self.factor
    }

    /// The same expression, numbers bit for bit.
    pub(crate) fn same(&self, other: &Self) -> bool {
        self.term.same(&other.term)
    }
}

/// What a LAPLACE expression parses to.
pub(super) enum LaplaceTransfer {
    /// `N(s)/D(s)·e^(-s·T)`: ascending numerator and denominator
    /// coefficients, and the delay `T`.
    Rational {
        num: Vec<Value>,
        den: Vec<Value>,
        delay: Value,
    },
    /// An expression with a factor that is not rational in `s`.
    Pointwise(TransferExpression),
}

struct TermParser {
    text: Vec<char>,
    tokens: Vec<RatTok>,
    /// The characters of `text` each token was read from.
    spans: Vec<(usize, usize)>,
    pos: usize,
}

impl TermParser {
    fn peek(&self) -> Option<&RatTok> {
        self.tokens.get(self.pos)
    }

    fn next(&mut self) -> Option<RatTok> {
        let tok = self.tokens.get(self.pos).cloned();
        if tok.is_some() {
            self.pos += 1;
        }
        tok
    }

    /// The text of the tokens from `first` to the last one read, in lower
    /// case: the netlist reader may have folded it to upper.
    fn written_since(&self, first: usize) -> String {
        let start = self.spans[first].0;
        let end = self.spans[self.pos - 1].1;
        let text: String = self.text[start..end].iter().collect();
        text.trim().to_ascii_lowercase()
    }

    fn expect_close(&mut self) -> Result<(), String> {
        if self.next() == Some(RatTok::RParen) {
            Ok(())
        } else {
            Err("unbalanced parentheses in LAPLACE expression".to_string())
        }
    }

    fn parse_expr(&mut self) -> Result<Term, String> {
        let mut value = self.parse_term()?;
        while let Some(op) = self.peek() {
            match op {
                RatTok::Plus => {
                    self.next();
                    value = Term::Sum(Box::new(value), Box::new(self.parse_term()?));
                }
                RatTok::Minus => {
                    self.next();
                    value = Term::Difference(Box::new(value), Box::new(self.parse_term()?));
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<Term, String> {
        let mut value = self.parse_power()?;
        while let Some(op) = self.peek() {
            match op {
                RatTok::Star => {
                    self.next();
                    value = Term::Product(Box::new(value), Box::new(self.parse_power()?));
                }
                RatTok::Slash => {
                    self.next();
                    value = Term::Quotient(Box::new(value), Box::new(self.parse_power()?));
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_power(&mut self) -> Result<Term, String> {
        let first = self.pos;
        let base = self.parse_unary()?;
        if self.peek() != Some(&RatTok::Caret) {
            return Ok(base);
        }
        self.next();
        let exponent = self.parse_unary()?;
        let text = self.written_since(first);
        Ok(Term::Power(Box::new(base), Box::new(exponent), text))
    }

    fn parse_unary(&mut self) -> Result<Term, String> {
        match self.peek() {
            Some(RatTok::Minus) => {
                self.next();
                Ok(Term::Negate(Box::new(self.parse_unary()?)))
            }
            Some(RatTok::Plus) => {
                self.next();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Term, String> {
        let first = self.pos;
        match self.next() {
            Some(RatTok::Num(v)) => Ok(Term::Number(v)),
            Some(RatTok::S) => Ok(Term::S),
            Some(RatTok::Function(name)) => {
                let function = TransferFunction::named(&name).ok_or_else(|| {
                    format!(
                        "{}() is not a function a LAPLACE expression can use; it can use {}",
                        name.to_ascii_lowercase(),
                        TransferFunction::NAMES
                    )
                })?;
                // The tokenizer reads a name as a function only before `(`.
                self.next();
                let mut args = vec![self.parse_expr()?];
                while self.peek() == Some(&RatTok::Comma) {
                    self.next();
                    args.push(self.parse_expr()?);
                }
                self.expect_close()?;
                if args.len() != function.arity() {
                    return Err(format!(
                        "{}() in a LAPLACE expression takes {} argument(s), not {}",
                        name.to_ascii_lowercase(),
                        function.arity(),
                        args.len()
                    ));
                }
                Ok(Term::Call(function, args, self.written_since(first)))
            }
            Some(RatTok::LParen) => {
                let inner = self.parse_expr()?;
                self.expect_close()?;
                Ok(inner)
            }
            other => Err(format!(
                "unexpected token {:?} in LAPLACE expression",
                other
            )),
        }
    }
}

/// Parse a LAPLACE expression: exactly as `N(s)/D(s)·e^(-s·T)` when it is
/// one, and as an expression known pointwise when a factor is not rational
/// in `s`. `parameter` gives the value of a name the expression uses, if it
/// has one.
pub(super) fn parse_laplace(
    parameter: &dyn Fn(&str) -> Option<Value>,
    text: &str,
) -> Result<LaplaceTransfer, String> {
    let (tokens, spans) = tokenize_rational(parameter, text)?;
    if tokens.is_empty() {
        return Err("empty LAPLACE transfer function".to_string());
    }
    let mut parser = TermParser {
        text: text.chars().collect(),
        tokens,
        spans,
        pos: 0,
    };
    let term = parser.parse_expr()?;
    if parser.pos != parser.tokens.len() {
        return Err("trailing input after LAPLACE transfer function".to_string());
    }
    let rational = match term.rational() {
        Ok(rational) => rational,
        Err(Unrealizable::Factor(factor)) => {
            return Ok(LaplaceTransfer::Pointwise(TransferExpression {
                term,
                factor,
            }));
        }
        Err(Unrealizable::Error(message)) => return Err(message),
    };

    if !(rational.delay.is_finite() && rational.delay >= 0.0) {
        return Err("LAPLACE expression divides by exp(-s*T), a time advance".to_string());
    }
    let mut num = poly_clean(rational.num);
    let mut den = poly_clean(rational.den);
    if poly_is_zero(&den) {
        return Err("LAPLACE transfer function has a zero denominator".to_string());
    }

    // Cancel a common s^k factor (exact: both leading coefficient runs of
    // zeros), which keeps DC limits and properness checks honest for forms
    // like (s) / (s*(1+s)).
    let common = num
        .iter()
        .take_while(|c| **c == 0.0)
        .count()
        .min(den.iter().take_while(|c| **c == 0.0).count())
        .min(num.len() - 1)
        .min(den.len() - 1);
    if common > 0 && !poly_is_zero(&num) {
        num.drain(0..common);
        den.drain(0..common);
    }

    Ok(LaplaceTransfer::Rational {
        num,
        den,
        delay: rational.delay,
    })
}

//=============================================================================
// State-space synthesis
//=============================================================================

fn fmt_value(v: Value) -> String {
    // `Display` for f64 is shortest-round-trip; wrap negatives so the
    // generated expressions never produce `--` or `+-` sequences.
    if v < 0.0 {
        format!("({v})")
    } else {
        format!("{v}")
    }
}

/// Synthesize the parse-time realization of a LAPLACE controlled source.
///
/// Returns the replacement elements: `n` grounded capacitors (one per
/// state), `n` behavioral current sources implementing the controller-
/// canonical state derivatives, and the behavioral output source (voltage
/// for E forms, current for G forms) on the original output nodes. An
/// expression with a factor that is not rational in `s` becomes the source
/// [`pointwise_transfer`] builds instead.
#[allow(clippy::too_many_arguments)]
pub(super) fn synthesize_laplace(
    parameter: &dyn Fn(&str) -> Option<Value>,
    name: &str,
    node_pos: &str,
    node_neg: &str,
    input_expr: &str,
    rational_text: &str,
    voltage_output: bool,
    line_num: usize,
) -> Result<Vec<Element>, ParseError> {
    let fail = |message: String| ParseError::Syntax {
        line: line_num,
        message: format!("LAPLACE source '{name}': {message}"),
    };
    let (num, den, delay) = match parse_laplace(parameter, rational_text).map_err(fail)? {
        LaplaceTransfer::Rational { num, den, delay } => (num, den, delay),
        LaplaceTransfer::Pointwise(transfer) => {
            return pointwise_transfer(
                name,
                (node_pos, node_neg),
                input_expr,
                transfer,
                voltage_output,
            )
            .map_err(fail);
        }
    };
    let form = crate::netlist::SynthesizedTransferForm::Laplace;
    let mut elements = Vec::new();
    let delayed;
    let input_expr = if delay > 0.0 {
        delayed = delay_line(
            name,
            1,
            form,
            input_expr,
            DelayTime::Fixed(delay),
            &mut elements,
        );
        delayed.as_str()
    } else {
        input_expr
    };
    elements.extend(synthesize_rational_transfer(
        name,
        node_pos,
        node_neg,
        input_expr,
        num,
        den,
        voltage_output,
        form,
        line_num,
    )?);
    Ok(elements)
}

/// The source a LAPLACE expression with a factor that is not rational in `s`
/// stands for.
///
/// A behavioral source drives the input onto a private node, and the owner's
/// output is the transfer's value at `s = 0` times that node: the operating
/// point. Small-signal analyses answer the owner with the transfer itself,
/// evaluated at `s = jω`, through the response the input source carries.
/// The transfer has no response in time, so the analyses that need one refuse
/// the owner; and a transfer with no finite real value at `s = 0` has no
/// operating point, so it is refused here.
fn pointwise_transfer(
    name: &str,
    (node_pos, node_neg): (&str, &str),
    input_expr: &str,
    transfer: TransferExpression,
    voltage_output: bool,
) -> Result<Vec<Element>, String> {
    let dc = transfer.at(Complex64::new(0.0, 0.0));
    if !(dc.re.is_finite() && dc.im == 0.0) {
        return Err(format!(
            "the transfer has no finite real value at s = 0 ({}), so the source has no \
             operating point",
            if dc.re.is_finite() && dc.im.is_finite() {
                format!("{}{:+}j", dc.re, dc.im)
            } else {
                "unbounded".to_string()
            }
        ));
    }
    let input_node = format!("{name}.__FIN");
    let behavioral = |expression: String, voltage: bool| {
        if voltage {
            ElementKind::BehavioralVoltage {
                expression,
                tc1: 0.0,
                tc2: 0.0,
                multiplicity: SourceMultiplicity::default(),
                jacobian: true,
            }
        } else {
            ElementKind::BehavioralCurrent {
                expression,
                tc1: 0.0,
                tc2: 0.0,
                multiplicity: SourceMultiplicity::default(),
                jacobian: true,
            }
        }
    };
    Ok(vec![
        Element {
            name: format!("{name}.__BFIN"),
            kind: behavioral(input_expr.to_string(), true),
            nodes: vec![input_node.clone(), "0".to_string()],
            provenance: ElementProvenance::GeneratedFrequencyResponseInput {
                owner: name.to_string(),
                node: input_node.clone(),
                response: FrequencyResponse::transfer(transfer),
            },
        },
        Element {
            name: name.to_string(),
            kind: behavioral(
                format!("{}*V({input_node})", fmt_value(dc.re)),
                voltage_output,
            ),
            nodes: vec![node_pos.to_string(), node_neg.to_string()],
            provenance: ElementProvenance::Authored,
        },
    ])
}

/// Realize a real rational transfer function as circuit elements.
///
/// This is the half of [`synthesize_laplace`] that follows parsing, split out
/// because a filter designed from a specification rather than written down as
/// a ratio of polynomials -- PSpice's CHEBYSHEV sources -- has to reach exactly
/// the same realization. `form` names the authoring keyword in error messages
/// and travels with the generated elements, so a failure points at the source
/// the user actually wrote and nothing downstream has to guess where the
/// realization came from.
#[allow(clippy::too_many_arguments)]
pub(super) fn synthesize_rational_transfer(
    name: &str,
    node_pos: &str,
    node_neg: &str,
    input_expr: &str,
    num: Vec<Value>,
    den: Vec<Value>,
    voltage_output: bool,
    form: crate::netlist::SynthesizedTransferForm,
    line_num: usize,
) -> Result<Vec<Element>, ParseError> {
    let fail = |message: String| ParseError::Syntax {
        line: line_num,
        message: format!("{} source '{name}': {message}", form.label()),
    };

    let require_stable_filter = matches!(form, crate::netlist::SynthesizedTransferForm::Chebyshev);

    if num.is_empty() || den.is_empty() {
        return Err(fail(
            "transfer function requires non-empty numerator and denominator polynomials"
                .to_string(),
        ));
    }
    if num
        .iter()
        .chain(&den)
        .any(|coefficient| !coefficient.is_finite())
    {
        return Err(fail(
            "transfer-function coefficients must all be finite".to_string(),
        ));
    }
    if den.last() == Some(&0.0) {
        return Err(fail(
            "transfer function denominator must have a non-zero leading coefficient".to_string(),
        ));
    }

    let n = den.len() - 1; // denominator degree
    if num.len() - 1 > n {
        return Err(fail(format!(
            "improper transfer function (numerator degree {} exceeds denominator degree {})",
            num.len() - 1,
            n
        )));
    }

    // Monic denominator.
    let lead = *den.last().expect("non-empty denominator");
    let a: Vec<Value> = den.iter().map(|c| c / lead).collect();
    let normalized_num: Vec<Value> = num.iter().map(|c| c / lead).collect();
    if require_stable_filter
        && (den
            .iter()
            .zip(&a)
            .any(|(before, after)| *before != 0.0 && *after == 0.0)
            || num
                .iter()
                .zip(&normalized_num)
                .any(|(before, after)| *before != 0.0 && *after == 0.0))
    {
        return Err(fail(
            "filter normalization lost a required nonzero coefficient".to_string(),
        ));
    }
    let num = normalized_num;
    if a.iter()
        .chain(&num)
        .any(|coefficient| !coefficient.is_finite())
    {
        return Err(fail(
            "transfer-function normalization produced non-finite coefficients".to_string(),
        ));
    }

    // Direct feedthrough and strictly-proper remainder: N = d·D + R.
    let d = if num.len() - 1 == n && n > 0 {
        num[n]
    } else if n == 0 {
        num[0]
    } else {
        0.0
    };
    let r: Vec<Value> = (0..n)
        .map(|k| num.get(k).unwrap_or(&0.0) - d * a[k])
        .collect();

    let output_kind = |expression: String| -> ElementKind {
        if voltage_output {
            ElementKind::BehavioralVoltage {
                expression,
                tc1: 0.0,
                tc2: 0.0,
                multiplicity: SourceMultiplicity::default(),
                jacobian: true,
            }
        } else {
            ElementKind::BehavioralCurrent {
                expression,
                tc1: 0.0,
                tc2: 0.0,
                multiplicity: SourceMultiplicity::default(),
                jacobian: true,
            }
        }
    };

    let mut elements = Vec::new();

    // Pure gain: no states needed.
    if n == 0 {
        elements.push(Element {
            name: name.to_string(),
            kind: output_kind(format!("{}*({})", fmt_value(d), input_expr)),
            nodes: vec![node_pos.to_string(), node_neg.to_string()],
            provenance: crate::netlist::ElementProvenance::Authored,
        });
        return Ok(elements);
    }

    // Frequency scaling ω₀ keeps state magnitudes and conductances O(1):
    // a₀^(1/n) is the geometric mean of the pole magnitudes; for poles at
    // the origin fall back to the largest remaining coefficient scale.
    let mut omega0 = if a[0].abs() > 0.0 {
        a[0].abs().powf(1.0 / n as Value)
    } else {
        (0..n)
            .filter(|k| a[*k].abs() > 0.0)
            .map(|k| a[k].abs().powf(1.0 / (n - k) as Value))
            .fold(0.0f64, f64::max)
    };
    if !(omega0.is_finite() && omega0 > 0.0) {
        omega0 = 1.0;
    }
    omega0 = omega0.clamp(1e-6, 1e15);

    let a_hat: Vec<Value> = (0..n).map(|k| a[k] / omega0.powi((n - k) as i32)).collect();
    let b_hat: Vec<Value> = (0..n).map(|k| r[k] / omega0.powi((n - k) as i32)).collect();
    if a_hat
        .iter()
        .chain(&b_hat)
        .any(|coefficient| !coefficient.is_finite())
    {
        return Err(fail(
            "state-space scaling produced non-finite coefficients".to_string(),
        ));
    }
    let dc_determined = a_hat[0] != 0.0;
    let scaling_lost_nonzero = a[..n]
        .iter()
        .zip(&a_hat)
        .chain(r.iter().zip(&b_hat))
        .any(|(before, after)| *before != 0.0 && *after == 0.0);
    if require_stable_filter
        && (!dc_determined || scaling_lost_nonzero || b_hat.iter().all(|value| *value == 0.0))
    {
        return Err(fail(
            "filter coefficients lost a required nonzero term during state-space scaling"
                .to_string(),
        ));
    }

    let state_node = |i: usize| format!("{}.__X{}", name, i);
    let cap_value = 1.0 / omega0;
    for i in 1..=n {
        let node = state_node(i);
        elements.push(Element {
            name: format!("{}.__CX{}", name, i),
            kind: ElementKind::Capacitor {
                value: cap_value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params: Vec::new(),
                deferred_params: Vec::new(),
            },
            nodes: vec![node.clone(), "0".to_string()],
            provenance: crate::netlist::ElementProvenance::GeneratedDynamicInternalNode {
                owner: name.to_string(),
                form,
                node,
            },
        });
    }

    // Chain integrators: dz_i/dτ = z_{i+1} (current injected into z_i).
    for i in 1..n {
        elements.push(Element {
            name: format!("{}.__DX{}", name, i),
            kind: ElementKind::BehavioralCurrent {
                expression: format!("V({})", state_node(i + 1)),
                tc1: 0.0,
                tc2: 0.0,
                multiplicity: SourceMultiplicity::default(),
                jacobian: true,
            },
            nodes: vec!["0".to_string(), state_node(i)],
            provenance: crate::netlist::ElementProvenance::GeneratedDynamicStateDerivative {
                owner: name.to_string(),
                form,
                dc_determined,
            },
        });
    }

    // Last state: dz_n/dτ = −Σ a_hat[k] z_{k+1} + u.
    let mut feedback_terms: Vec<String> = (0..n)
        .filter(|k| a_hat[*k] != 0.0)
        .map(|k| format!("-{}*V({})", fmt_value(a_hat[k]), state_node(k + 1)))
        .collect();
    feedback_terms.push(format!("({})", input_expr));
    elements.push(Element {
        name: format!("{}.__DX{}", name, n),
        kind: ElementKind::BehavioralCurrent {
            expression: feedback_terms.join(" + "),
            tc1: 0.0,
            tc2: 0.0,
            multiplicity: SourceMultiplicity::default(),
            jacobian: true,
        },
        nodes: vec!["0".to_string(), state_node(n)],
        provenance: crate::netlist::ElementProvenance::GeneratedDynamicStateDerivative {
            owner: name.to_string(),
            form,
            dc_determined,
        },
    });

    // Output: y = Σ b̂_k z_{k+1} + d·u.
    let mut output_terms: Vec<String> = (0..n)
        .filter(|k| b_hat[*k] != 0.0)
        .map(|k| format!("{}*V({})", fmt_value(b_hat[k]), state_node(k + 1)))
        .collect();
    if d != 0.0 {
        output_terms.push(format!("{}*({})", fmt_value(d), input_expr));
    }
    if output_terms.is_empty() {
        output_terms.push("0".to_string());
    }
    elements.push(Element {
        name: name.to_string(),
        kind: output_kind(output_terms.join(" + ")),
        nodes: vec![node_pos.to_string(), node_neg.to_string()],
        provenance: crate::netlist::ElementProvenance::Authored,
    });

    Ok(elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The rational a LAPLACE expression is, or why it is not one.
    fn parse_rational_in_s(
        parameter: &dyn Fn(&str) -> Option<Value>,
        text: &str,
    ) -> Result<(Vec<Value>, Vec<Value>, Value), String> {
        match parse_laplace(parameter, text)? {
            LaplaceTransfer::Rational { num, den, delay } => Ok((num, den, delay)),
            LaplaceTransfer::Pointwise(transfer) => {
                Err(format!("{} has no rational realization", transfer.factor()))
            }
        }
    }

    /// Rational equality up to scaling: `got_n/got_d == want_n/want_d`
    /// iff `got_n·want_d == want_n·got_d` as polynomials.
    fn no_parameter(_: &str) -> Option<Value> {
        None
    }

    #[test]
    fn a_parameter_stands_for_its_value() {
        let parameter = |name: &str| name.eq_ignore_ascii_case("w0").then_some(2.0);
        let (num, den, _) = parse_rational_in_s(&parameter, "w0/(s+W0)").unwrap();
        assert_ratio_eq((&num, &den), (&[2.0], &[2.0, 1.0]));
        let error = parse_rational_in_s(&parameter, "1/(s+w1)").unwrap_err();
        assert!(error.contains("unknown symbol 'w1'"), "{error}");
    }

    fn assert_ratio_eq(got: (&[Value], &[Value]), want: (&[Value], &[Value])) {
        let lhs = poly_mul(&got.0.to_vec(), &want.1.to_vec());
        let rhs = poly_mul(&want.0.to_vec(), &got.1.to_vec());
        let n = lhs.len().max(rhs.len());
        let scale = lhs
            .iter()
            .chain(rhs.iter())
            .fold(1.0f64, |m, c| m.max(c.abs()));
        for i in 0..n {
            let l = lhs.get(i).unwrap_or(&0.0);
            let r = rhs.get(i).unwrap_or(&0.0);
            assert!(
                (l - r).abs() < 1e-9 * scale,
                "ratio mismatch: {got:?} vs {want:?}"
            );
        }
    }

    #[test]
    fn parses_first_order_lowpass() {
        let (num, den, _) = parse_rational_in_s(&no_parameter, "1/(1+s/1000)").unwrap();
        assert_ratio_eq((&num, &den), (&[1.0], &[1.0, 1e-3]));
    }

    #[test]
    fn double_star_is_a_power_and_a_negative_exponent_a_reciprocal() {
        let (num, den, delay) = parse_rational_in_s(&no_parameter, "1./(1+.0005*s)**3").unwrap();
        let (want_num, want_den, _) =
            parse_rational_in_s(&no_parameter, "1/((1+.0005*s)^3)").unwrap();
        assert_ratio_eq((&num, &den), (&want_num, &want_den));
        assert_eq!(delay, 0.0);
        let (num, den, delay) =
            parse_rational_in_s(&no_parameter, "exp(-.001*s)*(1+.001*s)**-2").unwrap();
        let (want_num, want_den, _) = parse_rational_in_s(&no_parameter, "1/(1+.001*s)^2").unwrap();
        assert_ratio_eq((&num, &den), (&want_num, &want_den));
        assert!((delay - 1e-3).abs() < 1e-18);
    }

    #[test]
    fn parses_polynomial_ratio_with_suffixes_and_powers() {
        // (1 + s/1k)^2 / (1 + s*1m) — suffixes k and m, integer power.
        let (num, den, _) = parse_rational_in_s(&no_parameter, "(1+s/1k)^2/(1+s*1m)").unwrap();
        assert_ratio_eq((&num, &den), (&[1.0, 2e-3, 1e-6], &[1.0, 1e-3]));
    }

    #[test]
    fn delay_factors_multiply_out_of_the_rational_part() {
        let (num, den, delay) =
            parse_rational_in_s(&no_parameter, "2*exp(-s*1m)*exp(1-s/1k)/(1+s*0.2m)").unwrap();
        assert!((delay - 2e-3).abs() < 1e-18, "{delay}");
        assert_ratio_eq((&num, &den), (&[2.0 * 1f64.exp()], &[1.0, 2e-4]));
        let (_, _, delay) =
            parse_rational_in_s(&no_parameter, "exp(-s*1m)/(1+s)+exp(-s*1m)").unwrap();
        assert_eq!(delay, 1e-3);
    }

    #[test]
    fn cancels_common_s_factors_exactly() {
        let (num, den, _) = parse_rational_in_s(&no_parameter, "s/(s*(1+s))").unwrap();
        // After exact s-cancellation the DC value must be finite (1.0).
        assert!(
            num[0] != 0.0 || den[0] != 0.0,
            "common s factor must cancel"
        );
        assert_ratio_eq((&num, &den), (&[1.0], &[1.0, 1.0]));
    }

    #[test]
    fn rejects_improper_and_malformed_input() {
        let err =
            synthesize_laplace(&no_parameter, "e1", "out", "0", "v(in)", "s", true, 1).unwrap_err();
        assert!(format!("{err}").contains("improper"), "{err}");

        assert!(parse_rational_in_s(&no_parameter, "1/(1+q)").is_err());
        for (text, needle) in [
            ("sqrt(s)/(1+s)", "sqrt(s) has no rational realization"),
            ("exp(-s*s)/(1+s)", "exp(-s*s) has no rational realization"),
            ("(1+s)**0.5/(1+s)", "(1+s)**0.5 has no rational realization"),
            ("erf(s)/(1+s)", "erf() is not a function"),
            ("pow(s)/(1+s)", "takes 2 argument(s), not 1"),
            ("exp(s*1m)", "must be a delay"),
            ("1/exp(-s*1m)", "time advance"),
            ("exp(-s*1m)+exp(-s*2m)", "different exp(-s*T) delays"),
        ] {
            let error = parse_rational_in_s(&no_parameter, text).unwrap_err();
            assert!(error.contains(needle), "{text}: {error}");
        }
        assert!(parse_rational_in_s(&no_parameter, "1/0").is_err());
        assert!(parse_rational_in_s(&no_parameter, "").is_err());
    }

    #[test]
    fn first_order_synthesis_has_expected_structure() {
        // H = 1/(1 + s/wc): one state, cap 1/wc, unit feedback, unit output.
        let wc = 6283.185307179586;
        let elements = synthesize_laplace(
            &no_parameter,
            "e1",
            "out",
            "0",
            "v(in)",
            &format!("1/(1+s/{wc})"),
            true,
            1,
        )
        .unwrap();

        // cap + state derivative + output
        assert_eq!(elements.len(), 3);
        match &elements[0].kind {
            ElementKind::Capacitor { value, .. } => {
                assert!((value - 1.0 / wc).abs() < 1e-12 / wc);
            }
            other => panic!("expected capacitor, got {other:?}"),
        }
        match &elements[2].kind {
            ElementKind::BehavioralVoltage { expression, .. } => {
                assert!(expression.contains("V(e1.__X1)"), "{expression}");
            }
            other => panic!("expected behavioral output, got {other:?}"),
        }
    }
}
