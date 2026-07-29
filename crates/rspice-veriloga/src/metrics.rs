//! Structured, portable measurements for compiler and Rust-backend work.
//!
//! Timings are operational evidence, not artifact identity. They are therefore
//! reported alongside compilation products but are never folded into source,
//! compiler-contract, or runtime-contract digests.

use std::collections::BTreeMap;
use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

/// A stable phase identifier shared by the front end and offline Rust backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PipelinePhase {
    Preprocess,
    Lex,
    Parse,
    Semantic,
    BytecodeGeneration,
    HirLowering,
    MirLowering,
    CanonicalNoisePlanning,
    RuntimeQualification,
    IntegrityValidation,
    CfgLowering,
    DerivativePreparation,
    Differentiation,
    DerivativeExtraction,
    NoisePlanning,
    StampPlanning,
    CfgOptimization,
    Scheduling,
    StampEmission,
    StateEmission,
    NoiseEmission,
    CheckpointFinalization,
}

impl fmt::Display for PipelinePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Preprocess => "preprocess",
            Self::Lex => "lex",
            Self::Parse => "parse",
            Self::Semantic => "semantic",
            Self::BytecodeGeneration => "bytecode_generation",
            Self::HirLowering => "hir_lowering",
            Self::MirLowering => "mir_lowering",
            Self::CanonicalNoisePlanning => "canonical_noise_planning",
            Self::RuntimeQualification => "runtime_qualification",
            Self::IntegrityValidation => "integrity_validation",
            Self::CfgLowering => "cfg_lowering",
            Self::DerivativePreparation => "derivative_preparation",
            Self::Differentiation => "differentiation",
            Self::DerivativeExtraction => "derivative_extraction",
            Self::NoisePlanning => "noise_planning",
            Self::StampPlanning => "stamp_planning",
            Self::CfgOptimization => "cfg_optimization",
            Self::Scheduling => "scheduling",
            Self::StampEmission => "stamp_emission",
            Self::StateEmission => "state_emission",
            Self::NoiseEmission => "noise_emission",
            Self::CheckpointFinalization => "checkpoint_finalization",
        })
    }
}

/// Elapsed time accumulated for one pipeline phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhaseTiming {
    pub phase: PipelinePhase,
    /// Monotonic elapsed time, saturated at `u64::MAX`.
    pub elapsed_nanos: u64,
}

impl PhaseTiming {
    pub fn elapsed(&self) -> Duration {
        Duration::from_nanos(self.elapsed_nanos)
    }
}

/// Structured measurements returned by compiler and transpiler entry points.
///
/// Counts that do not apply to a particular entry point remain zero. Phase
/// timings preserve execution order; [`Self::phase_elapsed_nanos`] sums a phase
/// if it occurred more than once.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineMetrics {
    pub phases: Vec<PhaseTiming>,
    pub total_elapsed_nanos: u64,
    pub input_bytes: u64,
    pub preprocessed_bytes: u64,
    pub token_count: u64,
    pub top_level_item_count: u64,
    pub module_count: u64,
    pub dependency_count: u64,
    pub generated_rust_bytes: u64,
    pub generated_rust_lines: u64,
}

impl PipelineMetrics {
    pub fn total_elapsed(&self) -> Duration {
        Duration::from_nanos(self.total_elapsed_nanos)
    }

    pub fn phase_elapsed_nanos(&self, phase: PipelinePhase) -> u64 {
        self.phases
            .iter()
            .filter(|timing| timing.phase == phase)
            .fold(0_u64, |total, timing| {
                total.saturating_add(timing.elapsed_nanos)
            })
    }

    pub fn phase_elapsed(&self, phase: PipelinePhase) -> Duration {
        Duration::from_nanos(self.phase_elapsed_nanos(phase))
    }

    pub fn has_phase(&self, phase: PipelinePhase) -> bool {
        self.phases.iter().any(|timing| timing.phase == phase)
    }
}

/// A compilation product paired with the measurements that produced it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Measured<T> {
    pub output: T,
    pub metrics: PipelineMetrics,
}

/// Disabled-by-default wall-clock limits for a pipeline invocation.
///
/// Values are nanoseconds rather than [`Duration`] so the policy serializes
/// identically on native and WebAssembly targets. A zero limit is valid and is
/// useful for asserting that a phase is never entered.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerformanceBudget {
    pub max_total_nanos: Option<u64>,
    pub max_phase_nanos: BTreeMap<PipelinePhase, u64>,
}

impl PerformanceBudget {
    pub fn with_max_total(mut self, limit: Duration) -> Self {
        self.max_total_nanos = Some(duration_nanos(limit));
        self
    }

    pub fn with_phase_limit(mut self, phase: PipelinePhase, limit: Duration) -> Self {
        self.max_phase_nanos.insert(phase, duration_nanos(limit));
        self
    }

    pub fn validate(&self, metrics: &PipelineMetrics) -> Result<(), PerformanceBudgetExceeded> {
        for (&phase, &limit_nanos) in &self.max_phase_nanos {
            let elapsed_nanos = metrics.phase_elapsed_nanos(phase);
            if elapsed_nanos > limit_nanos {
                return Err(PerformanceBudgetExceeded {
                    phase: Some(phase),
                    elapsed_nanos,
                    limit_nanos,
                });
            }
        }
        if let Some(limit_nanos) = self.max_total_nanos
            && metrics.total_elapsed_nanos > limit_nanos
        {
            return Err(PerformanceBudgetExceeded {
                phase: None,
                elapsed_nanos: metrics.total_elapsed_nanos,
                limit_nanos,
            });
        }
        Ok(())
    }
}

/// Evidence that an opt-in performance budget was exceeded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerformanceBudgetExceeded {
    pub phase: Option<PipelinePhase>,
    pub elapsed_nanos: u64,
    pub limit_nanos: u64,
}

impl fmt::Display for PerformanceBudgetExceeded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.phase {
            Some(phase) => write!(
                f,
                "pipeline phase {phase} took {:?}, exceeding its {:?} budget",
                Duration::from_nanos(self.elapsed_nanos),
                Duration::from_nanos(self.limit_nanos)
            ),
            None => write!(
                f,
                "pipeline took {:?}, exceeding its {:?} total budget",
                Duration::from_nanos(self.elapsed_nanos),
                Duration::from_nanos(self.limit_nanos)
            ),
        }
    }
}

impl std::error::Error for PerformanceBudgetExceeded {}

pub(crate) struct MetricsRecorder {
    metrics: PipelineMetrics,
    budget: PerformanceBudget,
}

impl MetricsRecorder {
    pub(crate) fn new(input_bytes: usize, budget: PerformanceBudget) -> Self {
        Self {
            metrics: PipelineMetrics {
                input_bytes: usize_to_u64(input_bytes),
                ..PipelineMetrics::default()
            },
            budget,
        }
    }

    pub(crate) fn record(
        &mut self,
        phase: PipelinePhase,
        elapsed: Duration,
    ) -> Result<(), PerformanceBudgetExceeded> {
        let elapsed_nanos = duration_nanos(elapsed);
        self.metrics.phases.push(PhaseTiming {
            phase,
            elapsed_nanos,
        });
        self.metrics.total_elapsed_nanos = self
            .metrics
            .total_elapsed_nanos
            .saturating_add(elapsed_nanos);
        self.budget.validate(&self.metrics)
    }

    pub(crate) fn metrics(&self) -> &PipelineMetrics {
        &self.metrics
    }

    pub(crate) fn metrics_mut(&mut self) -> &mut PipelineMetrics {
        &mut self.metrics
    }

    pub(crate) fn finish(self) -> PipelineMetrics {
        self.metrics
    }
}

pub(crate) fn usize_to_u64(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}

fn duration_nanos(value: Duration) -> u64 {
    u64::try_from(value.as_nanos()).unwrap_or(u64::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_phases_are_accumulated_without_losing_order() {
        let mut recorder = MetricsRecorder::new(17, PerformanceBudget::default());
        recorder
            .record(PipelinePhase::Lex, Duration::from_nanos(3))
            .unwrap();
        recorder
            .record(PipelinePhase::Parse, Duration::from_nanos(5))
            .unwrap();
        recorder
            .record(PipelinePhase::Lex, Duration::from_nanos(7))
            .unwrap();
        let metrics = recorder.finish();

        assert_eq!(metrics.input_bytes, 17);
        assert_eq!(metrics.total_elapsed_nanos, 15);
        assert_eq!(metrics.phase_elapsed_nanos(PipelinePhase::Lex), 10);
        assert_eq!(
            metrics
                .phases
                .iter()
                .map(|timing| timing.phase)
                .collect::<Vec<_>>(),
            vec![PipelinePhase::Lex, PipelinePhase::Parse, PipelinePhase::Lex]
        );
    }

    #[test]
    fn phase_budget_fails_as_soon_as_the_phase_exceeds_it() {
        let budget = PerformanceBudget::default()
            .with_phase_limit(PipelinePhase::Differentiation, Duration::from_nanos(9));
        let mut recorder = MetricsRecorder::new(0, budget);
        let error = recorder
            .record(PipelinePhase::Differentiation, Duration::from_nanos(10))
            .unwrap_err();

        assert_eq!(error.phase, Some(PipelinePhase::Differentiation));
        assert_eq!(error.elapsed_nanos, 10);
        assert_eq!(error.limit_nanos, 9);
    }

    #[test]
    fn total_budget_covers_the_sum_of_recorded_phases() {
        let budget = PerformanceBudget::default().with_max_total(Duration::from_nanos(7));
        let mut recorder = MetricsRecorder::new(0, budget);
        recorder
            .record(PipelinePhase::Lex, Duration::from_nanos(4))
            .unwrap();
        let error = recorder
            .record(PipelinePhase::Parse, Duration::from_nanos(4))
            .unwrap_err();

        assert_eq!(error.phase, None);
        assert_eq!(error.elapsed_nanos, 8);
        assert_eq!(error.limit_nanos, 7);
    }
}
