//! Bounded unit-expression parser. Exponents are stored in half-unit steps.

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Unit {
    // Time, voltage, current, temperature, angle, logarithmic ratio.
    pub(super) dimensions: [i16; 6],
    pub(super) scale: f64,
    pub(super) bias: f64,
}

impl Unit {
    pub(super) fn parse(raw: &str) -> Result<Self, String> {
        if raw.len() > 256 || raw.chars().any(char::is_control) {
            return Err("measurement unit exceeds its text bounds".to_owned());
        }
        let normalized = raw
            .trim()
            .replace(['µ', 'μ'], "u")
            .replace('Ω', "ohm")
            .replace('°', "deg")
            .replace('²', "^2")
            .replace('³', "^3")
            .replace(['·', '⋅'], "*");
        let mut parser = Parser {
            text: normalized.as_bytes(),
            at: 0,
        };
        let unit = parser
            .expression(0)
            .ok_or_else(|| format!("{raw:?} is not a recognized engineering unit"))?;
        parser.space();
        if parser.at != parser.text.len() || !unit.scale.is_finite() || unit.scale <= 0.0 {
            return Err(format!("{raw:?} is not a recognized engineering unit"));
        }
        Ok(unit)
    }

    pub(super) fn product(self, rhs: Self, divide: bool) -> Option<Self> {
        if self.bias != 0.0 || rhs.bias != 0.0 {
            return None;
        }
        let mut dimensions = self.dimensions;
        for (out, b) in dimensions.iter_mut().zip(rhs.dimensions) {
            *out = if divide {
                out.checked_sub(b)?
            } else {
                out.checked_add(b)?
            };
            if out.abs() > 128 {
                return None;
            }
        }
        let scale = if divide {
            self.scale / rhs.scale
        } else {
            self.scale * rhs.scale
        };
        (scale.is_finite() && scale > 0.0).then_some(Self {
            dimensions,
            scale,
            bias: 0.0,
        })
    }

    pub(super) fn power(self, power: f64) -> Option<Self> {
        if !power.is_finite() || power.abs() > 64.0 || self.bias != 0.0 {
            return None;
        }
        let mut dimensions = self.dimensions;
        for exponent in &mut dimensions {
            let value = f64::from(*exponent) * power;
            if value.fract() != 0.0 || value.abs() > 128.0 {
                return None;
            }
            *exponent = value as i16;
        }
        let scale = self.scale.powf(power);
        (scale.is_finite() && scale > 0.0).then_some(Self {
            dimensions,
            scale,
            bias: 0.0,
        })
    }

    pub(super) fn canonical_symbol(self) -> String {
        for name in [
            "1", "s", "Hz", "V", "A", "ohm", "S", "W", "F", "H", "C", "J", "K", "deg", "dB",
        ] {
            if atom(name).is_some_and(|unit| unit.dimensions == self.dimensions) {
                return name.to_owned();
            }
        }
        let parts = ["s", "V", "A", "K", "deg", "dB"]
            .iter()
            .zip(self.dimensions)
            .filter_map(|(symbol, exponent)| match exponent {
                0 => None,
                2 => Some((*symbol).to_owned()),
                _ => Some(format!("{symbol}^{}", f64::from(exponent) / 2.0)),
            })
            .collect::<Vec<_>>();
        parts.join("*")
    }
}

fn base(symbol: &str) -> Option<Unit> {
    let (dimensions, scale, bias) = match symbol {
        "1" | "unitless" | "dimensionless" | "count" | "bits" => ([0; 6], 1.0, 0.0),
        "%" | "percent" => ([0; 6], 0.01, 0.0),
        "s" | "second" | "seconds" => ([2, 0, 0, 0, 0, 0], 1.0, 0.0),
        "Hz" | "hz" | "hertz" => ([-2, 0, 0, 0, 0, 0], 1.0, 0.0),
        "V" | "volt" | "volts" => ([0, 2, 0, 0, 0, 0], 1.0, 0.0),
        "A" | "amp" | "amps" | "ampere" | "amperes" => ([0, 0, 2, 0, 0, 0], 1.0, 0.0),
        "ohm" | "ohms" => ([0, 2, -2, 0, 0, 0], 1.0, 0.0),
        "S" | "siemens" => ([0, -2, 2, 0, 0, 0], 1.0, 0.0),
        "W" | "watt" | "watts" => ([0, 2, 2, 0, 0, 0], 1.0, 0.0),
        "F" | "farad" | "farads" => ([2, -2, 2, 0, 0, 0], 1.0, 0.0),
        "H" | "henry" | "henries" => ([2, 2, -2, 0, 0, 0], 1.0, 0.0),
        "C" | "coulomb" | "coulombs" => ([2, 0, 2, 0, 0, 0], 1.0, 0.0),
        "J" | "joule" | "joules" => ([2, 2, 2, 0, 0, 0], 1.0, 0.0),
        "K" | "kelvin" => ([0, 0, 0, 2, 0, 0], 1.0, 0.0),
        "degC" | "celsius" => ([0, 0, 0, 2, 0, 0], 1.0, 273.15),
        "deg" | "degree" | "degrees" => ([0, 0, 0, 0, 2, 0], 1.0, 0.0),
        "rad" | "radian" | "radians" => ([0, 0, 0, 0, 2, 0], 180.0 / std::f64::consts::PI, 0.0),
        "dB" | "db" => ([0, 0, 0, 0, 0, 2], 1.0, 0.0),
        _ => return None,
    };
    Some(Unit {
        dimensions,
        scale,
        bias,
    })
}

fn atom(symbol: &str) -> Option<Unit> {
    base(symbol).or_else(|| {
        for (prefix, scale) in [
            ("Meg", 1e6),
            ("meg", 1e6),
            ("f", 1e-15),
            ("p", 1e-12),
            ("n", 1e-9),
            ("u", 1e-6),
            ("m", 1e-3),
            ("k", 1e3),
            ("K", 1e3),
            ("M", 1e6),
            ("G", 1e9),
            ("T", 1e12),
        ] {
            if let Some(suffix) = symbol.strip_prefix(prefix) {
                // Prefixes apply to physical SI symbols, not percentages/angles/affine scales.
                if matches!(
                    suffix,
                    "s" | "Hz" | "V" | "A" | "ohm" | "S" | "W" | "F" | "H" | "C" | "J" | "K"
                ) {
                    let mut unit = base(suffix)?;
                    unit.scale *= scale;
                    return Some(unit);
                }
            }
        }
        None
    })
}

struct Parser<'a> {
    text: &'a [u8],
    at: usize,
}
impl Parser<'_> {
    fn space(&mut self) {
        while self.text.get(self.at).is_some_and(u8::is_ascii_whitespace) {
            self.at += 1;
        }
    }
    fn expression(&mut self, depth: usize) -> Option<Unit> {
        if depth > 16 {
            return None;
        }
        let mut unit = self.factor(depth)?;
        loop {
            let before = self.at;
            self.space();
            let divide = match self.text.get(self.at) {
                Some(b'*') => {
                    self.at += 1;
                    false
                }
                Some(b'/') => {
                    self.at += 1;
                    true
                }
                Some(b')') | None => break,
                Some(_) if self.at > before => false,
                _ => return None,
            };
            unit = unit.product(self.factor(depth)?, divide)?;
        }
        Some(unit)
    }
    fn factor(&mut self, depth: usize) -> Option<Unit> {
        self.space();
        let mut unit = if self.text.get(self.at..)?.starts_with(b"sqrt(") {
            self.at += 5;
            let inner = self.expression(depth + 1)?;
            if self.text.get(self.at) != Some(&b')') {
                return None;
            }
            self.at += 1;
            inner.power(0.5)?
        } else if self.text.get(self.at) == Some(&b'(') {
            self.at += 1;
            let inner = self.expression(depth + 1)?;
            if self.text.get(self.at) != Some(&b')') {
                return None;
            }
            self.at += 1;
            inner
        } else {
            let start = self.at;
            while self
                .text
                .get(self.at)
                .is_some_and(|b| b.is_ascii_alphanumeric() || *b == b'%')
            {
                self.at += 1;
            }
            atom(std::str::from_utf8(self.text.get(start..self.at)?).ok()?)?
        };
        let after_factor = self.at;
        self.space();
        if self.text.get(self.at) == Some(&b'^') {
            self.at += 1;
            let start = self.at;
            if matches!(self.text.get(self.at), Some(b'+' | b'-')) {
                self.at += 1;
            }
            while self
                .text
                .get(self.at)
                .is_some_and(|b| b.is_ascii_digit() || *b == b'.')
            {
                self.at += 1;
            }
            let power = std::str::from_utf8(self.text.get(start..self.at)?)
                .ok()?
                .parse::<f64>()
                .ok()?;
            unit = unit.power(power)?;
        } else {
            self.at = after_factor;
        }
        Some(unit)
    }
}
