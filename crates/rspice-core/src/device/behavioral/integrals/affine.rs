//! Certify invariance to unknown common voltages using exact coefficients.
//!
//! Evaluating a derivative at a few points cannot establish this identity.
//! Literal coefficients are binary rationals, so exact bounded arithmetic
//! distinguishes cancellation from a small but real common-mode dependence.

use crate::expr::{BinaryOp, Expr, UnaryOp};
use num_bigint::BigInt;
use std::collections::BTreeMap;

#[derive(Clone)]
struct Ratio {
    numerator: BigInt,
    denominator: BigInt,
}

impl Ratio {
    fn literal(value: f64) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }
        let bits = value.to_bits();
        let biased = ((bits >> 52) & 0x7ff) as i32;
        let mut mantissa = (bits & ((1_u64 << 52) - 1)) | if biased == 0 { 0 } else { 1_u64 << 52 };
        if mantissa == 0 {
            return Some(Self {
                numerator: 0.into(),
                denominator: 1.into(),
            });
        }
        let zeros = mantissa.trailing_zeros();
        mantissa >>= zeros;
        let exponent = if biased == 0 { -1074 } else { biased - 1075 } + zeros as i32;
        let mut numerator = BigInt::from(mantissa);
        let mut denominator = BigInt::from(1);
        if bits >> 63 != 0 {
            numerator = -numerator;
        }
        if exponent >= 0 {
            numerator <<= exponent as usize;
        } else {
            denominator <<= (-exponent) as usize;
        }
        Some(Self {
            numerator,
            denominator,
        })
    }

    fn zero(&self) -> bool {
        self.numerator == BigInt::from(0)
    }

    fn product(a: &BigInt, b: &BigInt) -> Option<BigInt> {
        // Refuse an oversized proof rather than approximate a coefficient.
        (a.bits().saturating_add(b.bits()) <= 8192).then(|| a * b)
    }

    fn add(&self, other: &Self) -> Option<Self> {
        Some(Self {
            numerator: Self::product(&self.numerator, &other.denominator)?
                + Self::product(&other.numerator, &self.denominator)?,
            denominator: Self::product(&self.denominator, &other.denominator)?,
        })
    }

    fn multiply(&self, other: &Self) -> Option<Self> {
        Some(Self {
            numerator: Self::product(&self.numerator, &other.numerator)?,
            denominator: Self::product(&self.denominator, &other.denominator)?,
        })
    }
}

type Terms = BTreeMap<usize, Ratio>;

fn scale(mut terms: Terms, factor: &Ratio) -> Option<Terms> {
    for coefficient in terms.values_mut() {
        *coefficient = coefficient.multiply(factor)?;
    }
    terms.retain(|_, coefficient| !coefficient.zero());
    Some(terms)
}

fn literal(expression: &Expr) -> Option<Ratio> {
    match expression {
        Expr::Const(value) => Ratio::literal(*value),
        Expr::Unary {
            op: UnaryOp::Neg,
            operand,
        } => {
            let mut value = literal(operand)?;
            value.numerator = -value.numerator;
            Some(value)
        }
        _ => None,
    }
}

fn terms(expression: &Expr, group: &impl Fn(&str) -> Option<Option<usize>>) -> Option<Terms> {
    match expression {
        Expr::NodeVoltage(name) => Some(match group(name)? {
            Some(group) => BTreeMap::from([(
                group,
                Ratio {
                    numerator: 1.into(),
                    denominator: 1.into(),
                },
            )]),
            None => BTreeMap::new(),
        }),
        Expr::Unary { op, operand } => {
            let operand = terms(operand, group)?;
            if operand.is_empty() {
                Some(operand)
            } else if *op == UnaryOp::Neg {
                scale(
                    operand,
                    &Ratio {
                        numerator: (-1).into(),
                        denominator: 1.into(),
                    },
                )
            } else {
                None
            }
        }
        Expr::Binary { op, left, right } => {
            let left_terms = terms(left, group)?;
            let right_terms = terms(right, group)?;
            if left_terms.is_empty() && right_terms.is_empty() {
                return Some(left_terms);
            }
            match op {
                BinaryOp::Add | BinaryOp::Sub => {
                    let mut combined = left_terms;
                    for (group, mut coefficient) in right_terms {
                        if *op == BinaryOp::Sub {
                            coefficient.numerator = -coefficient.numerator;
                        }
                        if let Some(previous) = combined.get(&group) {
                            coefficient = previous.add(&coefficient)?;
                        }
                        if coefficient.zero() {
                            combined.remove(&group);
                        } else {
                            combined.insert(group, coefficient);
                        }
                    }
                    Some(combined)
                }
                BinaryOp::Mul if left_terms.is_empty() => scale(right_terms, &literal(left)?),
                BinaryOp::Mul if right_terms.is_empty() => scale(left_terms, &literal(right)?),
                BinaryOp::Div if right_terms.is_empty() => {
                    let factor = literal(right)?;
                    if factor.zero() {
                        return None;
                    }
                    scale(
                        left_terms,
                        &Ratio {
                            numerator: factor.denominator,
                            denominator: factor.numerator,
                        },
                    )
                }
                _ => None,
            }
        }
        Expr::Function { args, .. } => {
            for arg in args {
                if !terms(arg, group)?.is_empty() {
                    return None;
                }
            }
            Some(Terms::new())
        }
        Expr::LookupTable { input, .. } => {
            let input = terms(input, group)?;
            input.is_empty().then_some(input)
        }
        Expr::BranchCurrent(_) | Expr::StringLiteral(_) => None,
        Expr::Const(_)
        | Expr::Time
        | Expr::Frequency
        | Expr::Temperature
        | Expr::ThermalVoltage
        | Expr::Gmin => Some(Terms::new()),
    }
}

pub(super) fn invariant(expression: &Expr, group: impl Fn(&str) -> Option<Option<usize>>) -> bool {
    terms(expression, &group).is_some_and(|terms| terms.is_empty())
}
