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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonAlignmentMethod {
    #[default]
    ExactCoordinateRows,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonInterpolationPolicy {
    #[default]
    NoneExactOnly,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComparisonResamplingPolicy {
    #[default]
    NoneRetainSourceGrid,
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
    pub(crate) fn validate_structure(&self) -> Result<(), VisualizationError> {
        self.policy.tolerance.validate()?;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProgressiveOperationKind {
    Export,
    Transform,
    Comparison,
    MeasurementEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "state")]
pub enum ProgressiveOperationState {
    Running,
    Completed { output_digest: ContentDigest },
    Cancelling,
    Cancelled,
    Failed { message: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressiveOperation {
    id: OperationId,
    kind: ProgressiveOperationKind,
    document_id: ResultDocumentId,
    source_revision: ObjectRevision,
    total_units: u64,
    completed_units: u64,
    checkpoint_digest: Option<ContentDigest>,
    recovery_count: u32,
    state: ProgressiveOperationState,
}

impl ProgressiveOperation {
    pub fn start(
        id: OperationId,
        kind: ProgressiveOperationKind,
        document_id: ResultDocumentId,
        source_revision: ObjectRevision,
        total_units: u64,
    ) -> Result<Self, VisualizationError> {
        if total_units == 0 {
            return Err(VisualizationError::InvalidValue {
                field: "operation.total-units",
                message: "total units must be greater than zero".to_owned(),
            });
        }
        Ok(Self {
            id,
            kind,
            document_id,
            source_revision,
            total_units,
            completed_units: 0,
            checkpoint_digest: None,
            recovery_count: 0,
            state: ProgressiveOperationState::Running,
        })
    }

    #[must_use]
    pub const fn id(&self) -> OperationId {
        self.id
    }

    #[must_use]
    pub const fn completed_units(&self) -> u64 {
        self.completed_units
    }

    #[must_use]
    pub const fn total_units(&self) -> u64 {
        self.total_units
    }

    #[must_use]
    pub const fn state(&self) -> &ProgressiveOperationState {
        &self.state
    }

    #[must_use]
    pub const fn checkpoint_digest(&self) -> Option<ContentDigest> {
        self.checkpoint_digest
    }

    #[must_use]
    pub const fn recovery_count(&self) -> u32 {
        self.recovery_count
    }

    pub fn advance(
        &mut self,
        completed_units: u64,
        checkpoint_digest: ContentDigest,
        completed_output: Option<ContentDigest>,
    ) -> Result<(), VisualizationError> {
        if self.state != ProgressiveOperationState::Running {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "advance",
            });
        }
        if completed_units <= self.completed_units || completed_units > self.total_units {
            return Err(VisualizationError::InvalidProgress {
                previous: self.completed_units,
                next: completed_units,
                total: self.total_units,
            });
        }
        if completed_units == self.total_units {
            let output_digest = completed_output.ok_or(VisualizationError::MissingOutputDigest)?;
            self.completed_units = completed_units;
            self.checkpoint_digest = Some(checkpoint_digest);
            self.state = ProgressiveOperationState::Completed { output_digest };
        } else {
            if completed_output.is_some() {
                return Err(VisualizationError::UnexpectedOutputDigest);
            }
            self.completed_units = completed_units;
            self.checkpoint_digest = Some(checkpoint_digest);
        }
        Ok(())
    }

    pub fn request_cancel(&mut self) -> Result<(), VisualizationError> {
        if self.state != ProgressiveOperationState::Running {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "request-cancel",
            });
        }
        self.state = ProgressiveOperationState::Cancelling;
        Ok(())
    }

    pub fn confirm_cancelled(&mut self) -> Result<(), VisualizationError> {
        if self.state != ProgressiveOperationState::Cancelling {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "confirm-cancelled",
            });
        }
        self.state = ProgressiveOperationState::Cancelled;
        Ok(())
    }

    pub fn fail(
        &mut self,
        message: impl Into<String>,
        checkpoint_digest: Option<ContentDigest>,
    ) -> Result<(), VisualizationError> {
        if !matches!(
            &self.state,
            ProgressiveOperationState::Running | ProgressiveOperationState::Cancelling
        ) {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "fail",
            });
        }
        let message = message.into();
        validate_label("operation.failure", &message)?;
        if checkpoint_digest.is_some() {
            self.checkpoint_digest = checkpoint_digest;
        }
        self.state = ProgressiveOperationState::Failed { message };
        Ok(())
    }

    pub fn recover(&mut self) -> Result<(), VisualizationError> {
        if !matches!(
            &self.state,
            ProgressiveOperationState::Cancelled | ProgressiveOperationState::Failed { .. }
        ) {
            return Err(VisualizationError::InvalidOperationTransition {
                from: self.state_label(),
                event: "recover",
            });
        }
        self.recovery_count = self
            .recovery_count
            .checked_add(1)
            .ok_or(VisualizationError::RecoverySpaceExhausted)?;
        self.state = ProgressiveOperationState::Running;
        Ok(())
    }

    fn state_label(&self) -> &'static str {
        match &self.state {
            ProgressiveOperationState::Running => "running",
            ProgressiveOperationState::Completed { .. } => "completed",
            ProgressiveOperationState::Cancelling => "cancelling",
            ProgressiveOperationState::Cancelled => "cancelled",
            ProgressiveOperationState::Failed { .. } => "failed",
        }
    }
}
