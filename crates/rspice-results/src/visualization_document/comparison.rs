//! Comparing two signals, and the progressive operations that carry the work.
//!
//! A comparison is only meaningful if both sides agree on how rows were
//! aligned, so the execution contract records the alignment, interpolation,
//! resampling, extrapolation, and precision policies that produced a receipt
//! rather than leaving them implicit. A progressive operation reports its own
//! state, so a comparison still running is never read as one that concluded.

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RowAlignmentPolicy {
    RequireIdentical,
    ExactIntersection,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NumericTolerance {
    pub absolute: f64,
    pub relative: f64,
}

impl NumericTolerance {
    pub fn new(absolute: f64, relative: f64) -> Result<Self, VisualizationError> {
        if !absolute.is_finite() || absolute < 0.0 || !relative.is_finite() || relative < 0.0 {
            return Err(VisualizationError::InvalidValue {
                field: "comparison.tolerance",
                message: "absolute and relative tolerances must be finite and non-negative"
                    .to_owned(),
            });
        }
        Ok(Self { absolute, relative })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonPolicy {
    pub row_alignment: RowAlignmentPolicy,
    pub tolerance: NumericTolerance,
    pub require_identical_units: bool,
    #[serde(default)]
    pub execution: ComparisonExecutionContract,
}

/// Fully declared numerical behavior for the currently implemented exact
/// comparison engine. New algorithms must add explicit variants rather than
/// silently changing alignment or interpolation semantics.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ComparisonExecutionContract {
    #[serde(default)]
    pub alignment: ComparisonAlignmentMethod,
    #[serde(default)]
    pub interpolation: ComparisonInterpolationPolicy,
    #[serde(default)]
    pub resampling: ComparisonResamplingPolicy,
    #[serde(default)]
    pub extrapolation: ComparisonExtrapolationPolicy,
    #[serde(default)]
    pub precision: ComparisonPrecisionPolicy,
}

impl ComparisonExecutionContract {
    pub fn validate(&self) -> Result<(), VisualizationError> {
        match &self.alignment {
            ComparisonAlignmentMethod::ExactCoordinateRows
            | ComparisonAlignmentMethod::AbsoluteXAxis => {}
            ComparisonAlignmentMethod::FirstThresholdCrossing {
                signal_key,
                threshold,
                baseline_crossing,
                candidate_crossing,
            } => {
                validate_key("comparison.execution.alignment.signal-key", signal_key)?;
                if !threshold.is_finite()
                    || !baseline_crossing.is_finite()
                    || !candidate_crossing.is_finite()
                {
                    return Err(VisualizationError::InvalidValue {
                        field: "comparison.execution.alignment",
                        message: "threshold-crossing parameters must be finite".to_owned(),
                    });
                }
            }
            ComparisonAlignmentMethod::CrossCorrelation {
                signal_key,
                maximum_lag_samples,
                selected_lag_samples,
                sample_interval,
                coefficient,
                baseline_shift,
            } => {
                validate_key("comparison.execution.alignment.signal-key", signal_key)?;
                if *maximum_lag_samples == 0
                    || selected_lag_samples.unsigned_abs() > u64::from(*maximum_lag_samples)
                    || !sample_interval.is_finite()
                    || *sample_interval <= 0.0
                    || !coefficient.is_finite()
                    || !(-1.0..=1.0).contains(coefficient)
                    || !baseline_shift.is_finite()
                {
                    return Err(VisualizationError::InvalidValue {
                        field: "comparison.execution.alignment",
                        message:
                            "cross-correlation parameters are outside their exact numeric contract"
                                .to_owned(),
                    });
                }
            }
        }
        let exact_policy = matches!(
            self.alignment,
            ComparisonAlignmentMethod::ExactCoordinateRows
                | ComparisonAlignmentMethod::AbsoluteXAxis
        );
        let exact_resampling = matches!(
            self.resampling,
            ComparisonResamplingPolicy::NoneRetainSourceGrid
                | ComparisonResamplingPolicy::ExactCoordinateIntersection
        );
        if exact_policy
            != matches!(
                self.interpolation,
                ComparisonInterpolationPolicy::NoneExactOnly
            )
            || exact_policy != exact_resampling
        {
            return Err(VisualizationError::InvalidValue {
                field: "comparison.execution",
                message:
                    "alignment, interpolation, and resampling policies form an inconsistent contract"
                        .to_owned(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonAlignmentMethod {
    /// Compatibility spelling for receipts written by the original exact-row engine.
    #[default]
    ExactCoordinateRows,
    AbsoluteXAxis,
    FirstThresholdCrossing {
        #[serde(deserialize_with = "deserialize_key_string")]
        signal_key: String,
        threshold: f64,
        baseline_crossing: f64,
        candidate_crossing: f64,
    },
    CrossCorrelation {
        #[serde(deserialize_with = "deserialize_key_string")]
        signal_key: String,
        maximum_lag_samples: u32,
        selected_lag_samples: i64,
        sample_interval: f64,
        coefficient: f64,
        baseline_shift: f64,
    },
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonInterpolationPolicy {
    #[default]
    NoneExactOnly,
    MonotoneLinear,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonResamplingPolicy {
    #[default]
    NoneRetainSourceGrid,
    ExactCoordinateIntersection,
    BaselineOntoCandidateGrid,
    UniformOverlapGrid,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonExtrapolationPolicy {
    #[default]
    Forbid,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonPrecisionPolicy {
    #[default]
    SourceF64NoRounding,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonRequest {
    pub baseline: DatasetBinding,
    pub candidate: DatasetBinding,
    #[serde(deserialize_with = "deserialize_comparison_signal_keys")]
    pub signal_keys: Vec<String>,
    pub policy: ComparisonPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonDisposition {
    Passed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalComparison {
    #[serde(deserialize_with = "deserialize_key_string")]
    pub signal_key: String,
    pub compared_rows: usize,
    pub failed_rows: usize,
    pub maximum_absolute_error: f64,
    pub maximum_relative_error: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComparisonReceipt {
    pub baseline: DatasetBinding,
    pub candidate: DatasetBinding,
    pub policy: ComparisonPolicy,
    pub rows_compared: usize,
    #[serde(deserialize_with = "deserialize_comparison_signals")]
    pub signals: Vec<SignalComparison>,
    pub disposition: ComparisonDisposition,
}

impl NestedResourceCount for ComparisonReceipt {
    fn nested_resource_count(&self) -> usize {
        self.signals.len()
    }
}

impl ComparisonReceipt {
    pub fn validate_structure(&self) -> Result<(), VisualizationError> {
        self.policy.tolerance.validate()?;
        self.policy.execution.validate()?;
        if self.baseline.dataset_id == self.candidate.dataset_id {
            return Err(VisualizationError::InvalidValue {
                field: "comparison-receipt.datasets",
                message: "baseline and candidate must be distinct immutable datasets".to_owned(),
            });
        }
        if self.rows_compared == 0
            || self.signals.is_empty()
            || self.signals.len() > MAX_COMPARISON_SIGNALS
        {
            return Err(VisualizationError::InvalidValue {
                field: "comparison-receipt",
                message: format!(
                    "a receipt must contain compared rows and 1 to {MAX_COMPARISON_SIGNALS} signals"
                ),
            });
        }
        let mut signal_keys = HashSet::new();
        for signal in &self.signals {
            validate_key("comparison-receipt.signal-key", &signal.signal_key)?;
            if !signal_keys.insert(signal.signal_key.as_str()) {
                return Err(VisualizationError::DuplicateKey(signal.signal_key.clone()));
            }
            if signal.compared_rows != self.rows_compared
                || signal.failed_rows > signal.compared_rows
                || !signal.maximum_absolute_error.is_finite()
                || signal.maximum_absolute_error < 0.0
                || !signal.maximum_relative_error.is_finite()
                || signal.maximum_relative_error < 0.0
            {
                return Err(VisualizationError::InvalidValue {
                    field: "comparison-receipt.signal",
                    message: "signal row counts and maximum errors must agree with the receipt"
                        .to_owned(),
                });
            }
        }
        let expected = if self.signals.iter().any(|signal| signal.failed_rows > 0) {
            ComparisonDisposition::Failed
        } else {
            ComparisonDisposition::Passed
        };
        if self.disposition != expected {
            return Err(VisualizationError::InvalidValue {
                field: "comparison-receipt.disposition",
                message: "disposition does not agree with signal outcomes".to_owned(),
            });
        }
        Ok(())
    }
}
