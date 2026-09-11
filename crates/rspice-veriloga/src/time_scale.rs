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
    /// Seconds-valued queries are module declarations, independent of a
    /// process's current tick or an externally supplied simulator environment.
    pub fn parameter_value(self, name: &str) -> Result<Option<f64>, &'static str> {
        self.validate()?;
        let exponent = if name.eq_ignore_ascii_case("timeUnit") {
            self.unit_exponent
        } else if name.eq_ignore_ascii_case("timePrecision") {
            self.precision_exponent
        } else {
            return Ok(None);
        };
        Ok(Some(Self::seconds(exponent)))
    }

    pub fn unit_seconds(self) -> Result<f64, &'static str> {
        self.validate()?;
        Ok(Self::seconds(self.unit_exponent))
    }

    fn seconds(exponent: i8) -> f64 {
        // Use the same correctly rounded decimal values as the event host,
        // without repeated multiplication or a runtime powi implementation.
        const SECONDS: [f64; 18] = [
            1e-15, 1e-14, 1e-13, 1e-12, 1e-11, 1e-10, 1e-9, 1e-8, 1e-7, 1e-6, 1e-5, 1e-4, 1e-3,
            1e-2, 1e-1, 1e0, 1e1, 1e2,
        ];
        SECONDS[(exponent + 15) as usize]
    }

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

    /// Exact denominator for converting a design timestamp to module units.
    pub(crate) fn ticks_per_unit(self, design_precision: i8) -> Result<u64, &'static str> {
        self.validate()?;
        if !(-15..=self.precision_exponent).contains(&design_precision) {
            return Err("design precision cannot be coarser than a module's precision");
        }
        Ok(10_u64.pow((self.unit_exponent - design_precision) as u32))
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

    /// Round at module precision, then rescale exactly. Negative procedural
    /// delays convert to unsigned 64-bit time. Unsupported scheduling ranges
    /// fail explicitly instead of saturating or wrapping the host clock.
    pub(crate) fn delay_ticks(self, units: f64, design_precision: i8) -> Result<i64, &'static str> {
        self.ticks_per_unit(design_precision)?;
        if !units.is_finite() {
            return Err("a delay must be finite");
        }
        let local_scale = 10_u64.pow((self.unit_exponent - self.precision_exponent) as u32);
        let local_ticks = (units * local_scale as f64).round();
        const TIME_MODULUS: f64 = 18_446_744_073_709_551_616.0;
        if !local_ticks.is_finite() || local_ticks >= TIME_MODULUS {
            return Err("delay exceeds the representable tick range");
        }
        let local_ticks = if local_ticks < 0.0 {
            ((-local_ticks % TIME_MODULUS) as u64).wrapping_neg()
        } else {
            local_ticks as u64
        };
        let design_scale = 10_u64.pow((self.precision_exponent - design_precision) as u32);
        Self::checked_delay_product(local_ticks, design_scale)
    }

    pub(crate) fn integer_delay_ticks(
        self,
        units: u64,
        design_precision: i8,
    ) -> Result<i64, &'static str> {
        Self::checked_delay_product(units, self.ticks_per_unit(design_precision)?)
    }

    fn checked_delay_product(ticks: u64, scale: u64) -> Result<i64, &'static str> {
        ticks
            .checked_mul(scale)
            .and_then(|value| i64::try_from(value).ok())
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
