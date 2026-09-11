//! Declared Verilog module timing. Units and precision are decimal powers of
//! ten; delay rounding precedes rescaling to the elaborated design's tick.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ModuleTimeScale {
    unit_exponent: i8,
    precision_exponent: i8,
}

impl Default for ModuleTimeScale {
    fn default() -> Self {
        Self {
            unit_exponent: -9,
            precision_exponent: -9,
        }
    }
}

impl ModuleTimeScale {
    pub fn new(unit_exponent: i8, precision_exponent: i8) -> Result<Self, &'static str> {
        let result = Self {
            unit_exponent,
            precision_exponent,
        };
        result.validate()?;
        Ok(result)
    }

    pub fn validate(self) -> Result<(), &'static str> {
        if !(-15..=2).contains(&self.unit_exponent)
            || !(-15..=2).contains(&self.precision_exponent)
            || self.precision_exponent > self.unit_exponent
        {
            return Err(
                "time units and precision must be powers of ten from 1 fs to 100 s, with precision no coarser than the time unit",
            );
        }
        Ok(())
    }

    pub const fn unit_exponent(self) -> i8 {
        self.unit_exponent
    }
    pub const fn precision_exponent(self) -> i8 {
        self.precision_exponent
    }

    /// Parse the two operands of an active `timescale directive.
    pub(crate) fn parse(operands: &str) -> Result<Self, &'static str> {
        fn exponent(operand: &str) -> Option<i8> {
            let operand = operand.trim();
            let split = operand.find(|ch: char| !ch.is_ascii_digit())?;
            let magnitude = match &operand[..split] {
                "1" => 0,
                "10" => 1,
                "100" => 2,
                _ => return None,
            };
            let unit = match operand[split..].trim() {
                "s" => 0,
                "ms" => -3,
                "us" => -6,
                "ns" => -9,
                "ps" => -12,
                "fs" => -15,
                _ => return None,
            };
            Some(magnitude + unit)
        }
        let (unit, precision) = operands
            .split_once('/')
            .ok_or("`timescale requires time_unit / time_precision")?;
        Self::new(
            exponent(unit).ok_or("invalid `timescale time unit: expected 1, 10 or 100 followed by s, ms, us, ns, ps or fs")?,
            exponent(precision).ok_or("invalid `timescale precision: expected 1, 10 or 100 followed by s, ms, us, ns, ps or fs")?,
        )
    }

    /// Round a finite, nonnegative delay in module units at its declared
    /// precision, then exactly rescale that integer to the design precision.
    pub(crate) fn delay_ticks(self, units: f64, design_precision: i8) -> Result<i64, &'static str> {
        self.validate()?;
        if !(-15..=self.precision_exponent).contains(&design_precision) {
            return Err("design precision cannot be coarser than a module's precision");
        }
        if !units.is_finite() || units < 0.0 {
            return Err("a delay must be finite and nonnegative");
        }
        let local_scale = 10_u64.pow((self.unit_exponent - self.precision_exponent) as u32);
        let local_ticks = (units * local_scale as f64).round();
        // Cast only after checking the exclusive upper bound: i64::MAX as
        // f64 rounds to 2^63 and must not saturate into a valid delay.
        if !local_ticks.is_finite() || local_ticks >= 9_223_372_036_854_775_808.0 {
            return Err("delay exceeds the representable tick range");
        }
        let design_scale = 10_i64.pow((self.precision_exponent - design_precision) as u32);
        (local_ticks as i64)
            .checked_mul(design_scale)
            .ok_or("delay exceeds the representable tick range")
    }

    pub(crate) fn integer_delay_ticks(
        self,
        units: i64,
        design_precision: i8,
    ) -> Result<i64, &'static str> {
        self.validate()?;
        if !(-15..=self.precision_exponent).contains(&design_precision) || units < 0 {
            return Err("a delay must be nonnegative and use a compatible design precision");
        }
        units
            .checked_mul(10_i64.pow((self.unit_exponent - design_precision) as u32))
            .ok_or("delay exceeds the representable tick range")
    }
}

/// A compiled HDL design's root declaration and finest elaborated precision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalTiming {
    pub root: ModuleTimeScale,
    pub precision_exponent: i8,
}

impl Default for DigitalTiming {
    fn default() -> Self {
        Self {
            root: ModuleTimeScale::default(),
            precision_exponent: -9,
        }
    }
}

impl DigitalTiming {
    pub fn validate(self) -> Result<(), &'static str> {
        self.root.validate()?;
        if !(-15..=self.root.precision_exponent()).contains(&self.precision_exponent) {
            return Err("invalid elaborated digital precision");
        }
        Ok(())
    }
}
