//! Borrowed interpolation: compare selected signals without expanding a table.
use super::{CliError, WaveformData, types_compatible, units_compatible, variable_name_matches};

pub(super) struct Interpolation<'a> {
    source: &'a [f64],
    pub target: &'a [f64],
}

impl<'a> Interpolation<'a> {
    /// Only retained coordinates establish a categorical sample determination.
    /// Endpoint rounding follows the same bounded clamping as numeric samples.
    pub fn observed_index(&self, index: usize) -> Option<usize> {
        let point = self.target[index];
        let upper = self.source.partition_point(|&value| value < point);
        if upper == 0 {
            Some(0)
        } else if upper == self.source.len() {
            Some(upper - 1)
        } else {
            (self.source[upper] == point).then_some(upper)
        }
    }

    /// Availability codes are categorical. Between retained coordinates only
    /// two available endpoints (code zero) support an interpolated value.
    pub fn availability_status(
        &self,
        series: &[f64],
        validity: Option<&[bool]>,
        index: usize,
    ) -> Option<f64> {
        let defined = |index| validity.is_none_or(|valid| valid[index]);
        if let Some(observed) = self.observed_index(index) {
            return defined(observed).then_some(series[observed]);
        }
        let upper = self
            .source
            .partition_point(|&value| value < self.target[index]);
        ([upper - 1, upper]
            .into_iter()
            .all(|index| defined(index) && series[index] == 0.0))
        .then_some(0.0)
    }

    pub fn new(result: &'a WaveformData, golden: &'a WaveformData) -> Result<Self, CliError> {
        let invalid = |message: String| CliError::VerificationFailed { message };

        if !variable_name_matches(&result.variables[0], &golden.variables[0]) {
            return Err(invalid(format!(
                "independent coordinates differ: '{}' versus '{}'",
                result.variables[0], golden.variables[0]
            )));
        }
        if !types_compatible(&result.variable_types[0], &golden.variable_types[0]) {
            return Err(invalid(format!(
                "independent coordinate types differ: '{}' versus '{}'",
                result.variable_types[0], golden.variable_types[0]
            )));
        }
        if !units_compatible(result, 0, golden, 0) {
            return Err(invalid(
                "independent coordinate units differ; cannot interpolate".into(),
            ));
        }

        let result_scale = result
            .values
            .first()
            .ok_or_else(|| invalid("result file has no data to interpolate".to_string()))?;
        let golden_scale = golden
            .values
            .first()
            .ok_or_else(|| invalid("golden file has no data to interpolate against".to_string()))?;

        if result_scale.len() < 2 {
            return Err(invalid(
                "result needs at least two points to interpolate".to_string(),
            ));
        }
        if result_scale.windows(2).any(|pair| pair[1] <= pair[0]) {
            return Err(invalid(
                "result scale is not strictly increasing; cannot interpolate".to_string(),
            ));
        }

        let low = result_scale[0];
        let high = result_scale[result_scale.len() - 1];
        // Permit only a few representable rounding steps at either endpoint,
        // never an allowance measured in fixed seconds/hertz or a fraction of 1.
        let lower_bound = (0..4).fold(low, |value, _| value.next_down());
        let upper_bound = (0..4).fold(high, |value, _| value.next_up());
        for &point in golden_scale {
            if point < lower_bound || point > upper_bound {
                return Err(invalid(format!(
                    "golden scale point {point:e} lies outside the result range [{low:e}, {high:e}]; interpolation would extrapolate"
                )));
            }
        }

        Ok(Self {
            source: result_scale,
            target: golden_scale,
        })
    }

    pub fn sample(
        &self,
        series: &[f64],
        validity: Option<&[bool]>,
        index: usize,
        held: bool,
    ) -> Result<Option<f64>, CliError> {
        let invalid = |message: String| CliError::VerificationFailed { message };
        let defined = |index| validity.is_none_or(|valid| valid[index]);
        let x = self.target[index];
        // Index of the first scale point >= x (the scale is sorted).
        let upper = self.source.partition_point(|&s| s < x);
        if upper == 0 {
            return series
                .first()
                .copied()
                .map(|value| defined(0).then_some(value))
                .ok_or_else(|| invalid("result series is empty; cannot interpolate".to_string()));
        }
        if upper >= self.source.len() {
            return series
                .last()
                .copied()
                .map(|value| defined(series.len() - 1).then_some(value))
                .ok_or_else(|| invalid("result series is empty; cannot interpolate".to_string()));
        }
        let (x0, x1) = (self.source[upper - 1], self.source[upper]);
        let (y0, y1) = (series[upper - 1], series[upper]);
        if x == x1 {
            return Ok(defined(upper).then_some(y1));
        }
        if held {
            return Ok(defined(upper - 1).then_some(y0));
        }
        if !defined(upper - 1) || !defined(upper) {
            return Ok(None);
        }
        // Evaluate (y0 * (x1 - x) + y1 * (x - x0)) / (x1 - x0)
        // with a single rounding. Both differences and intermediate products
        // can overflow, while even a normalized weight can underflow before
        // multiplication by a large signal. Reuse the shared exact arithmetic.
        rspice_veriloga_runtime::arithmetic::sum_products_ratio(
            [(y0, x1), (-y0, x), (y1, x), (-y1, x0)].into_iter(),
            [(x1, 1.0), (x0, -1.0)].into_iter(),
        )
        .map(Some)
        .map_err(|error| invalid(format!("cannot interpolate at {x:e}: {error:?}")))
    }
}
