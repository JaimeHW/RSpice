//! Numeric shapes shared by real and complex expression evaluation.
//! A complex signal remains rectangular until an explicit display projection.

use num_complex::Complex64;
use serde::{Deserialize, Serialize};

use super::evaluator::EvaluationError;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NumericValue<T> {
    Scalar(T),
    Waveform(Vec<f64>, Vec<T>),
}

pub type RealValue = NumericValue<f64>;
pub type ComplexValue = NumericValue<Complex64>;

impl<T> NumericValue<T> {
    pub fn create_waveform(x: Vec<f64>, y: Vec<T>) -> Self {
        debug_assert_eq!(x.len(), y.len());
        Self::Waveform(x, y)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalcValue {
    Real(RealValue),
    Complex(ComplexValue),
}

impl From<RealValue> for CalcValue {
    fn from(value: RealValue) -> Self {
        Self::Real(value)
    }
}

impl From<ComplexValue> for CalcValue {
    fn from(value: ComplexValue) -> Self {
        Self::Complex(value)
    }
}

impl CalcValue {
    pub fn create_waveform(x: Vec<f64>, y: Vec<f64>) -> Self {
        RealValue::create_waveform(x, y).into()
    }

    pub fn into_complex(self) -> ComplexValue {
        match self {
            Self::Complex(value) => value,
            Self::Real(RealValue::Scalar(value)) => ComplexValue::Scalar(value.into()),
            Self::Real(RealValue::Waveform(x, y)) => {
                ComplexValue::Waveform(x, y.into_iter().map(Complex64::from).collect())
            }
        }
    }

    pub fn into_real(self) -> Result<RealValue, EvaluationError> {
        match self {
            Self::Real(value) => Ok(value),
            Self::Complex(_) => Err(EvaluationError::TypeMismatch(
                "complex values require an explicit real(), imag(), mag(), or phase() projection"
                    .to_owned(),
            )),
        }
    }

    /// Reject a non-finite scalar rather than presenting a successful reading.
    /// Series retain explicit holes at undefined samples.
    pub fn checked(self) -> Result<Self, EvaluationError> {
        match &self {
            Self::Real(RealValue::Scalar(value)) if !value.is_finite() => {
                Err(EvaluationError::MathError(
                    "scalar result is undefined or outside the finite range".to_owned(),
                ))
            }
            Self::Complex(ComplexValue::Scalar(value)) if !finite(*value) => {
                Err(EvaluationError::MathError(
                    "complex scalar result is undefined or outside the finite range".to_owned(),
                ))
            }
            Self::Real(RealValue::Waveform(x, y)) => {
                super::interpolation::validate_samples(x, y)
                    .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
                Ok(self)
            }
            Self::Complex(ComplexValue::Waveform(x, y)) => {
                super::interpolation::validate_samples(x, y)
                    .map_err(|error| EvaluationError::WaveformMismatch(error.to_string()))?;
                Ok(self)
            }
            _ => Ok(self),
        }
    }
}

/// The unit a value reads in, as a product of powers of named units.
///
/// SI's electrical units are reduced to volts, amperes and seconds, so V·A is
/// W and V/A is Ω whichever way an expression spells them. Exponents are kept
/// in halves, which is what a noise density's √Hz needs. A unit the algebra
/// does not know (a degree, a decibel, a percent, a swept parameter's own
/// unit) is a symbol of its own: it still multiplies and cancels, and two
/// values in it still add. No symbols at all is a plain number.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Unit {
    /// Each symbol with its exponent in halves, never zero, in symbol order.
    factors: std::collections::BTreeMap<String, i8>,
}

/// The SI units that reduce to volts, amperes and seconds, with their
/// exponents in halves.
const DERIVED_UNITS: [(&str, &[(&str, i8)]); 10] = [
    ("W", &[("V", 2), ("A", 2)]),
    ("Ω", &[("V", 2), ("A", -2)]),
    ("ohm", &[("V", 2), ("A", -2)]),
    ("S", &[("A", 2), ("V", -2)]),
    ("F", &[("A", 2), ("s", 2), ("V", -2)]),
    ("H", &[("V", 2), ("s", 2), ("A", -2)]),
    ("C", &[("A", 2), ("s", 2)]),
    ("J", &[("V", 2), ("A", 2), ("s", 2)]),
    ("Hz", &[("s", -2)]),
    ("ratio", &[]),
];

/// The names a reader expects for a reduced unit, tried before the reduced
/// product is spelled out.
/// A product two names share (A·s is C and A/Hz) takes the first.
const NAMED_UNITS: [&str; 17] = [
    "V", "A", "s", "W", "Ω", "S", "F", "H", "C", "J", "Hz", "V/s", "A/s", "V²/Hz", "A²/Hz",
    "V/√Hz", "A/√Hz",
];

impl Unit {
    /// A plain number: a ratio, a count, a gain.
    pub fn none() -> Self {
        Self::default()
    }

    /// Read a unit as it is written: `V`, `V²/Hz`, `V^2/Hz`, `nV/√Hz` is not
    /// one (a prefix scales the value), `A·s`, `°`, `dB`. A symbol that is not
    /// an SI electrical unit stays a symbol; `None` only for text that is not
    /// a product of powers at all.
    pub fn parse(text: &str) -> Option<Self> {
        let text = text.trim();
        if text.is_empty() || text == "1" {
            return Some(Self::none());
        }
        let mut unit = Self::none();
        let mut sign = 1_i8;
        let mut rest = text;
        while !rest.is_empty() {
            let (factor, next) = match rest.find(['/', '·', '*', ' ']) {
                Some(at) => (&rest[..at], Some(&rest[at..])),
                None => (rest, None),
            };
            if !factor.is_empty() && factor != "1" {
                let (symbol, halves) = parse_factor(factor)?;
                unit.multiply_symbol(symbol, sign * halves);
            }
            let Some(next) = next else { break };
            let separator = next.chars().next()?;
            if separator == '/' {
                if sign < 0 {
                    // `a/b/c` is ambiguous; nothing the producers write.
                    return None;
                }
                sign = -1;
            }
            rest = &next[separator.len_utf8()..];
        }
        Some(unit)
    }

    pub fn is_none(&self) -> bool {
        self.factors.is_empty()
    }

    fn multiply_symbol(&mut self, symbol: &str, halves: i8) {
        let reduced = DERIVED_UNITS
            .iter()
            .find(|(name, _)| *name == symbol)
            .map(|(_, factors)| *factors);
        match reduced {
            Some(factors) => {
                for (base, base_halves) in factors {
                    // A derived unit's own exponent is in halves too.
                    self.add_exponent(base, base_halves * halves / 2);
                }
            }
            None => self.add_exponent(symbol, halves),
        }
    }

    fn add_exponent(&mut self, symbol: &str, halves: i8) {
        let entry = self.factors.entry(symbol.to_owned()).or_insert(0);
        *entry += halves;
        if *entry == 0 {
            self.factors.remove(symbol);
        }
    }

    pub fn times(&self, other: &Self) -> Self {
        let mut product = self.clone();
        for (symbol, halves) in &other.factors {
            product.add_exponent(symbol, *halves);
        }
        product
    }

    pub fn divided_by(&self, other: &Self) -> Self {
        self.times(&other.powi_halves(-2).expect("negation keeps whole halves"))
    }

    /// This unit raised to `exponent / 2`, or `None` when an exponent would
    /// fall between halves.
    pub fn powi_halves(&self, exponent: i8) -> Option<Self> {
        let mut factors = std::collections::BTreeMap::new();
        for (symbol, halves) in &self.factors {
            let product = i16::from(*halves) * i16::from(exponent);
            if product % 2 != 0 {
                return None;
            }
            factors.insert(symbol.clone(), i8::try_from(product / 2).ok()?);
        }
        Some(Self { factors })
    }

    /// This unit raised to a real power, when every exponent stays in halves.
    pub fn powf(&self, exponent: f64) -> Option<Self> {
        if self.is_none() {
            return Some(Self::none());
        }
        let halves = exponent * 2.0;
        if !halves.is_finite() || halves.fract() != 0.0 || halves.abs() > 16.0 {
            return None;
        }
        self.powi_halves(halves as i8)
    }

    /// The unit as a reader writes it: a named unit when it is one (`W`, not
    /// `A·V`), else its factors, positive powers first.
    pub fn label(&self) -> String {
        if let Some(name) = NAMED_UNITS
            .iter()
            .find(|name| Self::parse(name).as_ref() == Some(self))
        {
            return (*name).to_owned();
        }
        // A unit of its own (°, dB, a swept parameter's unit) leads, then
        // volts, amperes and seconds: `°·s`, `V·s`.
        let rank = |symbol: &str| ["V", "A", "s"].iter().position(|base| *base == symbol);
        let mut factors: Vec<(&String, &i8)> = self.factors.iter().collect();
        factors.sort_by_key(|(symbol, _)| (rank(symbol), (*symbol).clone()));
        let spell = |positive: bool| {
            factors
                .iter()
                .filter(|(_, halves)| (**halves > 0) == positive)
                .map(|(symbol, halves)| power_label(symbol, halves.unsigned_abs()))
                .collect::<Vec<_>>()
                .join("·")
        };
        let (numerator, denominator) = (spell(true), spell(false));
        match (numerator.is_empty(), denominator.is_empty()) {
            (_, true) => numerator,
            (true, false) => format!("1/{denominator}"),
            (false, false) if self.factors.values().filter(|h| **h < 0).count() > 1 => {
                format!("{numerator}/({denominator})")
            }
            (false, false) => format!("{numerator}/{denominator}"),
        }
    }
}

impl std::fmt::Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label())
    }
}

/// One factor of a written unit: a symbol and its exponent in halves.
fn parse_factor(factor: &str) -> Option<(&str, i8)> {
    if let Some(root) = factor.strip_prefix('√') {
        return valid_symbol(root).map(|symbol| (symbol, 1));
    }
    let (symbol, power) = match factor.find(['^', '²', '³']) {
        Some(at) => (&factor[..at], &factor[at..]),
        None => (factor, ""),
    };
    let halves = match power {
        "" => 2,
        "²" | "^2" => 4,
        "³" | "^3" => 6,
        "^0.5" | "^(1/2)" => 1,
        power => 2 * power.strip_prefix('^')?.parse::<i8>().ok()?,
    };
    valid_symbol(symbol).map(|symbol| (symbol, halves))
}

fn valid_symbol(symbol: &str) -> Option<&str> {
    let ok = !symbol.is_empty()
        && !symbol.starts_with(|c: char| c.is_ascii_digit() || c == '-' || c == '.')
        && !symbol.contains(['(', ')', '^', '√']);
    ok.then_some(symbol)
}

fn power_label(symbol: &str, halves: u8) -> String {
    let whole = halves / 2;
    let power = match whole {
        0 | 1 => String::new(),
        2 => "²".to_owned(),
        3 => "³".to_owned(),
        n => format!("^{n}"),
    };
    match (whole, halves % 2 == 1) {
        (0, true) => format!("√{symbol}"),
        (_, true) => format!("{symbol}{power}·√{symbol}"),
        _ => format!("{symbol}{power}"),
    }
}

pub(super) fn finite(value: Complex64) -> bool {
    value.re.is_finite() && value.im.is_finite()
}

pub(super) fn hole(value: Complex64) -> Complex64 {
    if finite(value) {
        value
    } else {
        Complex64::new(f64::NAN, f64::NAN)
    }
}
