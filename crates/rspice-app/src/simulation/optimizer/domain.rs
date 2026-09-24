//! Search coordinates are separate from the physical parameter values.

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum OptimizationVariableDomain {
    #[default]
    Linear,
    Logarithmic,
    /// Values min + n*step inside the configured bounds.
    Quantized {
        step: f64,
    },
    /// Strictly increasing allowed physical values.
    Discrete {
        values: Vec<f64>,
    },
}

pub(crate) struct OptimizationCoordinates {
    domain: OptimizationVariableDomain,
    physical_min: f64,
    physical_max: f64,
    physical_initial: f64,
    pub min: f64,
    pub max: f64,
    pub initial: f64,
}
impl OptimizationVariableDomain {
    pub(crate) fn is_discrete(&self) -> bool {
        matches!(self, Self::Quantized { .. } | Self::Discrete { .. })
    }
    pub(crate) fn coordinates(
        &self,
        min: f64,
        max: f64,
        initial: f64,
    ) -> Result<OptimizationCoordinates, String> {
        if !min.is_finite()
            || !max.is_finite()
            || !initial.is_finite()
            || min >= max
            || initial < min
            || initial > max
        {
            return Err("Variable bounds and initial value must be finite and ordered".into());
        }
        let (lo, hi, start) = match self {
            Self::Linear => (min, max, initial),
            Self::Logarithmic => {
                if min <= 0.0 {
                    return Err("Logarithmic variables require strictly positive bounds".into());
                }
                (min.ln(), max.ln(), initial.ln())
            }
            Self::Quantized { step } => {
                if !step.is_finite() || *step <= 0.0 || min + step == min {
                    return Err("Grid step must be finite, positive, and distinguishable at the lower bound".into());
                }
                let quotient = (max - min) / step;
                let nearest = quotient.round();
                let count = if (quotient - nearest).abs()
                    <= (8.0 * f64::EPSILON * quotient.abs()).min(0.25)
                {
                    nearest
                } else {
                    quotient.floor()
                };
                if !count.is_finite() || !(1.0..=9_007_199_254_740_991.0).contains(&count) {
                    return Err(
                        "Grid must contain at least two representable values within the bounds"
                            .into(),
                    );
                }
                let start = ((initial - min) / step).round();
                let snapped = min + start * step;
                if start > count
                    || (snapped - initial).abs()
                        > (8.0 * f64::EPSILON * initial.abs().max(min.abs()).max(step.abs()))
                            .min(step * 0.25)
                {
                    return Err(
                        "Initial value must lie on the grid anchored at the lower bound".into(),
                    );
                }
                (0.0, count, start)
            }
            Self::Discrete { values } => {
                if values.len() < 2
                    || values
                        .iter()
                        .any(|v| !v.is_finite() || *v < min || *v > max)
                    || values.windows(2).any(|pair| pair[0] >= pair[1])
                {
                    return Err("Allowed values must contain at least two distinct increasing values within the bounds".into());
                }
                let start = values
                    .iter()
                    .position(|value| *value == initial)
                    .ok_or("Initial value must be one of the allowed values")?;
                (0.0, (values.len() - 1) as f64, start as f64)
            }
        };
        if !lo.is_finite() || !hi.is_finite() || !(hi - lo).is_finite() || lo >= hi {
            return Err(
                "Variable range cannot be represented in the selected search coordinates".into(),
            );
        }
        Ok(OptimizationCoordinates {
            domain: self.clone(),
            physical_min: min,
            physical_max: max,
            physical_initial: initial,
            min: lo,
            max: hi,
            initial: start,
        })
    }
}
impl OptimizationCoordinates {
    pub fn quantum(&self) -> Option<f64> {
        self.domain.is_discrete().then_some(1.0)
    }
    pub fn physical(&self, coordinate: f64) -> f64 {
        let coordinate = coordinate.clamp(self.min, self.max);
        match &self.domain {
            OptimizationVariableDomain::Linear => coordinate,
            OptimizationVariableDomain::Logarithmic => {
                if coordinate == self.initial {
                    self.physical_initial
                } else if coordinate <= self.min {
                    self.physical_min
                } else if coordinate >= self.max {
                    self.physical_max
                } else {
                    coordinate.exp().clamp(self.physical_min, self.physical_max)
                }
            }
            OptimizationVariableDomain::Quantized { step } => (self.physical_min
                + coordinate.round() * step)
                .clamp(self.physical_min, self.physical_max),
            OptimizationVariableDomain::Discrete { values } => values[coordinate.round() as usize],
        }
    }
}
