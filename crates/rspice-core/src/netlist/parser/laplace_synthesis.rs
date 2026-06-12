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
//! The rational is parsed exactly — no sampling or fitting — by evaluating
//! the brace expression over the field of rational functions in `s`. The
//! denominator is then made monic and the realization is frequency-scaled
//! by `ω₀ = a₀^(1/n)` (the geometric mean of the pole magnitudes) so state
//! magnitudes and stamped conductances stay O(1) wherever the poles live.

use super::{Element, ElementKind, ParseError};
use crate::Value;

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

/// Rational function `num/den` in `s`.
#[derive(Debug, Clone)]
struct Rational {
    num: Poly,
    den: Poly,
}

impl Rational {
    fn constant(v: Value) -> Self {
        Self {
            num: vec![v],
            den: vec![1.0],
        }
    }

    fn s() -> Self {
        Self {
            num: vec![0.0, 1.0],
            den: vec![1.0],
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            num: poly_add(
                &poly_mul(&self.num, &other.den),
                &poly_mul(&other.num, &self.den),
            ),
            den: poly_mul(&self.den, &other.den),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        self.add(&Self {
            num: poly_neg(&other.num),
            den: other.den.clone(),
        })
    }

    fn mul(&self, other: &Self) -> Self {
        Self {
            num: poly_mul(&self.num, &other.num),
            den: poly_mul(&self.den, &other.den),
        }
    }

    fn div(&self, other: &Self) -> Result<Self, String> {
        if poly_is_zero(&other.num) {
            return Err("division by zero inside LAPLACE expression".to_string());
        }
        Ok(Self {
            num: poly_mul(&self.num, &other.den),
            den: poly_mul(&self.den, &other.num),
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

fn tokenize_rational(text: &str) -> Result<Vec<RatTok>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
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
            's' | 'S' if !chars.get(i + 1).is_some_and(|c| c.is_ascii_alphanumeric()) => {
                tokens.push(RatTok::S);
                i += 1;
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
                tokens.push(RatTok::Num(base * mult));
            }
            other => {
                return Err(format!(
                    "unsupported symbol '{other}' in LAPLACE expression \
                     (allowed: numbers, s, + - * / ^ and parentheses)"
                ));
            }
        }
    }
    Ok(tokens)
}

struct RatParser {
    tokens: Vec<RatTok>,
    pos: usize,
}

impl RatParser {
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

    fn parse_expr(&mut self) -> Result<Rational, String> {
        let mut value = self.parse_term()?;
        while let Some(op) = self.peek() {
            match op {
                RatTok::Plus => {
                    self.next();
                    value = value.add(&self.parse_term()?);
                }
                RatTok::Minus => {
                    self.next();
                    value = value.sub(&self.parse_term()?);
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_term(&mut self) -> Result<Rational, String> {
        let mut value = self.parse_power()?;
        while let Some(op) = self.peek() {
            match op {
                RatTok::Star => {
                    self.next();
                    value = value.mul(&self.parse_power()?);
                }
                RatTok::Slash => {
                    self.next();
                    value = value.div(&self.parse_power()?)?;
                }
                _ => break,
            }
        }
        Ok(value)
    }

    fn parse_power(&mut self) -> Result<Rational, String> {
        let base = self.parse_unary()?;
        if self.peek() == Some(&RatTok::Caret) {
            self.next();
            match self.next() {
                Some(RatTok::Num(v)) if v >= 0.0 && v.fract() == 0.0 && v <= 16.0 => {
                    Ok(base.powi(v as u32))
                }
                _ => Err("'^' in LAPLACE expressions requires a small non-negative \
                          integer exponent"
                    .to_string()),
            }
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Rational, String> {
        match self.peek() {
            Some(RatTok::Minus) => {
                self.next();
                Ok(Rational::constant(0.0).sub(&self.parse_unary()?))
            }
            Some(RatTok::Plus) => {
                self.next();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Rational, String> {
        match self.next() {
            Some(RatTok::Num(v)) => Ok(Rational::constant(v)),
            Some(RatTok::S) => Ok(Rational::s()),
            Some(RatTok::LParen) => {
                let inner = self.parse_expr()?;
                if self.next() != Some(RatTok::RParen) {
                    return Err("unbalanced parentheses in LAPLACE expression".to_string());
                }
                Ok(inner)
            }
            other => Err(format!(
                "unexpected token {:?} in LAPLACE expression",
                other
            )),
        }
    }
}

/// Parse a rational transfer function in `s` into ascending-coefficient
/// numerator and denominator polynomials.
pub(super) fn parse_rational_in_s(text: &str) -> Result<(Vec<Value>, Vec<Value>), String> {
    let tokens = tokenize_rational(text)?;
    if tokens.is_empty() {
        return Err("empty LAPLACE transfer function".to_string());
    }
    let mut parser = RatParser { tokens, pos: 0 };
    let rational = parser.parse_expr()?;
    if parser.pos != parser.tokens.len() {
        return Err("trailing input after LAPLACE transfer function".to_string());
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

    Ok((num, den))
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
/// for E forms, current for G forms) on the original output nodes.
pub(super) fn synthesize_laplace(
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

    let (num, den) = parse_rational_in_s(rational_text).map_err(&fail)?;

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
    let num: Vec<Value> = num.iter().map(|c| c / lead).collect();

    // Direct feedthrough and strictly-proper remainder: N = d·D + R.
    let d = if num.len() - 1 == n && n > 0 {
        num[n]
    } else if n == 0 {
        num[0]
    } else {
        0.0
    };
    let r: Vec<Value> = (0..n).map(|k| num.get(k).unwrap_or(&0.0) - d * a[k]).collect();

    let output_kind = |expression: String| -> ElementKind {
        if voltage_output {
            ElementKind::BehavioralVoltage {
                expression,
                tc1: 0.0,
                tc2: 0.0,
            }
        } else {
            ElementKind::BehavioralCurrent {
                expression,
                tc1: 0.0,
                tc2: 0.0,
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

    let state_node = |i: usize| format!("{}.__X{}", name, i);
    let cap_value = 1.0 / omega0;

    for i in 1..=n {
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
            nodes: vec![state_node(i), "0".to_string()],
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
            },
            nodes: vec!["0".to_string(), state_node(i)],
        });
    }

    // Last state: dz_n/dτ = −Σ â_k z_{k+1} + u.
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
        },
        nodes: vec!["0".to_string(), state_node(n)],
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
    });

    Ok(elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rational equality up to scaling: `got_n/got_d == want_n/want_d`
    /// iff `got_n·want_d == want_n·got_d` as polynomials.
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
        let (num, den) = parse_rational_in_s("1/(1+s/1000)").unwrap();
        assert_ratio_eq((&num, &den), (&[1.0], &[1.0, 1e-3]));
    }

    #[test]
    fn parses_polynomial_ratio_with_suffixes_and_powers() {
        // (1 + s/1k)^2 / (1 + s*1m) — suffixes k and m, integer power.
        let (num, den) = parse_rational_in_s("(1+s/1k)^2/(1+s*1m)").unwrap();
        assert_ratio_eq((&num, &den), (&[1.0, 2e-3, 1e-6], &[1.0, 1e-3]));
    }

    #[test]
    fn cancels_common_s_factors_exactly() {
        let (num, den) = parse_rational_in_s("s/(s*(1+s))").unwrap();
        // After exact s-cancellation the DC value must be finite (1.0).
        assert!(num[0] != 0.0 || den[0] != 0.0, "common s factor must cancel");
        assert_ratio_eq((&num, &den), (&[1.0], &[1.0, 1.0]));
    }

    #[test]
    fn rejects_improper_and_malformed_input() {
        let err = synthesize_laplace("e1", "out", "0", "v(in)", "s", true, 1).unwrap_err();
        assert!(format!("{err}").contains("improper"), "{err}");

        assert!(parse_rational_in_s("1/(1+q)").is_err());
        assert!(parse_rational_in_s("1/0").is_err());
        assert!(parse_rational_in_s("").is_err());
    }

    #[test]
    fn first_order_synthesis_has_expected_structure() {
        // H = 1/(1 + s/wc): one state, cap 1/wc, unit feedback, unit output.
        let wc = 6283.185307179586;
        let elements = synthesize_laplace(
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
