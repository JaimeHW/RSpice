//! Numerical quality retained with a transient and analyses derived from it.
//!
//! Source indices belong to the engine trajectory. Source times are captured
//! before output cropping or envelope projection changes the displayed axis.

use super::convergence_attribution::ConvergenceAttribution;
use rspice_core::abort_signal::AbortSignal;
use serde::{Deserialize, Serialize};

/// Existing result and artifact writers keep their own framing and accounting.
/// The field walk is shared so the two identities cannot omit different facts.
pub(crate) trait ConvergenceEncoder {
    fn u64(&mut self, value: u64);
    fn f64(&mut self, value: f64);
    fn string(&mut self, value: &str);
    fn tag(&mut self, value: u8);
}

/// Quality of the transient source, including a preceding periodic solve.
/// Absence of this record means unknown quality, never verified clean quality.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransientConvergenceEvidence {
    pub transient: ConvergenceReport,
    pub initialization: Option<PeriodicConvergenceEvidence>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PeriodicConvergenceEvidence {
    pub method: PeriodicInitializationMethod,
    /// Shooting corrections across qualification grids, or HB Newton iterations.
    #[serde(with = "decimal_u64")]
    pub solver_iterations: u64,
    pub final_residual: f64,
    /// Auxiliary solves recorded by the engine during initialization. These
    /// counters do not describe every iteration of the periodic solver itself.
    pub report: ConvergenceReport,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PeriodicInitializationMethod {
    Shooting,
    HarmonicBalance,
}

/// Engine metrics. Integer counters use decimal strings in JSON so a browser
/// cannot round a valid 64-bit count while parsing worker or project metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConvergenceReport {
    #[serde(with = "decimal_u64")]
    pub total_iterations: u64,
    #[serde(with = "decimal_u64")]
    pub gmin_stepping_count: u64,
    #[serde(with = "decimal_u64")]
    pub source_stepping_count: u64,
    /// Newton-converged points accepted despite local truncation error rejection.
    #[serde(with = "decimal_u64")]
    pub force_accepted_points: u64,
    #[serde(with = "decimal_indices")]
    pub force_accepted_indices: Vec<u64>,
    pub max_residual: f64,
    pub avg_iterations_per_solve: f64,
    #[serde(with = "decimal_u64")]
    pub timestep_reductions: u64,
    #[serde(with = "decimal_u64")]
    pub lte_rejections: u64,
    #[serde(with = "decimal_u64")]
    pub bypassed_device_evaluations: u64,
    /// A recovered attempt can leave a diagnostic even when the run succeeds.
    /// This is not the retained analysis' terminal failure attribution.
    pub failure_diagnostic: Option<ConvergenceAttribution>,
    /// Unavailable for periodic initialization, whose internal integration
    /// trajectory is not the final periodic display orbit.
    pub time_basis: Option<ConvergenceTimeBasis>,
}

/// Compact source-axis evidence: store affected times, not another full axis.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConvergenceTimeBasis {
    #[serde(with = "decimal_u64")]
    pub sample_count: u64,
    pub start_s: f64,
    pub stop_s: f64,
    /// One source time per `force_accepted_indices` entry, in the same order.
    pub force_accepted_times_s: Vec<f64>,
}

impl TransientConvergenceEvidence {
    /// Float64 values used by the compact worker/dependency representation.
    pub(crate) fn transfer_value_count(&self) -> usize {
        self.transient
            .force_accepted_indices
            .len()
            .saturating_mul(2)
            .saturating_add(
                self.transient
                    .time_basis
                    .as_ref()
                    .map_or(0, |basis| basis.force_accepted_times_s.len()),
            )
            .saturating_add(self.initialization.as_ref().map_or(0, |initialization| {
                initialization
                    .report
                    .force_accepted_indices
                    .len()
                    .saturating_mul(2)
            }))
    }

    pub fn has_lte_exceptions(&self) -> bool {
        self.transient.force_accepted_points != 0
            || self
                .initialization
                .as_ref()
                .is_some_and(|initialization| initialization.report.force_accepted_points != 0)
    }

    pub(crate) fn encode(&self, writer: &mut impl ConvergenceEncoder) {
        self.transient.encode(writer);
        writer.tag(u8::from(self.initialization.is_some()));
        if let Some(initialization) = &self.initialization {
            writer.u64(initialization.solver_iterations);
            writer.f64(initialization.final_residual);
            writer.tag(match initialization.method {
                PeriodicInitializationMethod::Shooting => 0,
                PeriodicInitializationMethod::HarmonicBalance => 1,
            });
            initialization.report.encode(writer);
        }
    }

    pub fn capture(
        quality: rspice_core::diagnostics::ConvergenceQuality,
        time: &[f64],
        abort: &dyn AbortSignal,
    ) -> Result<Self, rspice_core::SimulationError> {
        Ok(Self {
            transient: ConvergenceReport::capture(quality, Some(time), abort)?,
            initialization: None,
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        self.transient.validate()?;
        if self.transient.time_basis.is_none() {
            return Err(
                "Transient convergence evidence is missing its source time basis".to_owned(),
            );
        }
        if let Some(initialization) = &self.initialization {
            if !initialization.final_residual.is_finite() || initialization.final_residual < 0.0 {
                return Err("Periodic initialization residual is invalid".to_owned());
            }
            initialization.report.validate()?;
            if initialization.report.time_basis.is_some() {
                return Err("Periodic initialization must not claim the display orbit as its integration time basis".to_owned());
            }
        }
        Ok(())
    }
}

impl ConvergenceReport {
    /// Copy scalar metadata without cloning the two potentially large vectors.
    #[cfg(any(target_arch = "wasm32", test))]
    pub(crate) fn metadata_only(&self) -> Self {
        let Self {
            total_iterations,
            gmin_stepping_count,
            source_stepping_count,
            force_accepted_points,
            force_accepted_indices: _,
            max_residual,
            avg_iterations_per_solve,
            timestep_reductions,
            lte_rejections,
            bypassed_device_evaluations,
            failure_diagnostic,
            time_basis,
        } = self;
        Self {
            total_iterations: *total_iterations,
            gmin_stepping_count: *gmin_stepping_count,
            source_stepping_count: *source_stepping_count,
            force_accepted_points: *force_accepted_points,
            force_accepted_indices: Vec::new(),
            max_residual: *max_residual,
            avg_iterations_per_solve: *avg_iterations_per_solve,
            timestep_reductions: *timestep_reductions,
            lte_rejections: *lte_rejections,
            bypassed_device_evaluations: *bypassed_device_evaluations,
            failure_diagnostic: failure_diagnostic.clone(),
            time_basis: time_basis.as_ref().map(|basis| ConvergenceTimeBasis {
                sample_count: basis.sample_count,
                start_s: basis.start_s,
                stop_s: basis.stop_s,
                force_accepted_times_s: Vec::new(),
            }),
        }
    }

    fn encode(&self, writer: &mut impl ConvergenceEncoder) {
        let Self {
            total_iterations,
            gmin_stepping_count,
            source_stepping_count,
            force_accepted_points,
            force_accepted_indices,
            max_residual,
            avg_iterations_per_solve,
            timestep_reductions,
            lte_rejections,
            bypassed_device_evaluations,
            failure_diagnostic,
            time_basis,
        } = self;
        for value in [
            total_iterations,
            gmin_stepping_count,
            source_stepping_count,
            force_accepted_points,
            timestep_reductions,
            lte_rejections,
            bypassed_device_evaluations,
        ] {
            writer.u64(*value);
        }
        writer.u64(force_accepted_indices.len() as u64);
        for index in force_accepted_indices {
            writer.u64(*index);
        }
        writer.f64(*max_residual);
        writer.f64(*avg_iterations_per_solve);
        writer.tag(u8::from(failure_diagnostic.is_some()));
        if let Some(diagnostic) = failure_diagnostic {
            use super::convergence_attribution::{ConvergenceFailureClass, ConvergenceSiteKind};
            writer.tag(match diagnostic.class {
                ConvergenceFailureClass::NoDcPathToGround => 0,
                ConvergenceFailureClass::ConditioningDependentBias => 1,
                ConvergenceFailureClass::SingularSystem => 2,
                ConvergenceFailureClass::NewtonNonConvergence => 3,
            });
            writer.u64(diagnostic.sites.len() as u64);
            for site in &diagnostic.sites {
                writer.string(&site.name);
                writer.tag(match site.kind {
                    ConvergenceSiteKind::Node => 0,
                    ConvergenceSiteKind::Branch => 1,
                });
                writer.tag(u8::from(site.residual.is_some()));
                if let Some(residual) = site.residual {
                    writer.f64(residual);
                }
            }
            writer.u64(diagnostic.elided_sites as u64);
            writer.string(&diagnostic.failure_message);
        }
        writer.tag(u8::from(time_basis.is_some()));
        if let Some(basis) = time_basis {
            writer.u64(basis.sample_count);
            writer.f64(basis.start_s);
            writer.f64(basis.stop_s);
            writer.u64(basis.force_accepted_times_s.len() as u64);
            for time in &basis.force_accepted_times_s {
                writer.f64(*time);
            }
        }
    }

    pub fn capture(
        quality: rspice_core::diagnostics::ConvergenceQuality,
        time: Option<&[f64]>,
        abort: &dyn AbortSignal,
    ) -> Result<Self, rspice_core::SimulationError> {
        check_abort(abort)?;
        // Exhaustive: a new engine metric requires an explicit retention choice.
        let rspice_core::diagnostics::ConvergenceQuality {
            total_iterations,
            gmin_stepping_count,
            source_stepping_count,
            force_accepted_points,
            force_accepted_indices,
            max_residual,
            avg_iterations_per_solve,
            timestep_reductions,
            lte_rejections,
            bypassed_device_evaluations,
            failure_diagnostic,
        } = quality;
        let mut indices = Vec::with_capacity(force_accepted_indices.len());
        let mut times = Vec::new();
        if let Some(time) = time {
            if time.is_empty() {
                return Err(invalid("Convergence source time axis is empty"));
            }
            for (index, value) in time.iter().copied().enumerate() {
                if index.is_multiple_of(64) {
                    check_abort(abort)?;
                }
                if !value.is_finite() || value < 0.0 || (index > 0 && value <= time[index - 1]) {
                    return Err(invalid("Convergence source time axis is invalid"));
                }
            }
            times.reserve(force_accepted_indices.len());
        }
        for (position, index) in force_accepted_indices.into_iter().enumerate() {
            if position.is_multiple_of(64) {
                check_abort(abort)?;
            }
            indices.push(index as u64);
            if let Some(time) = time {
                times.push(*time.get(index).ok_or_else(|| {
                    invalid("Convergence point index is outside its source trajectory")
                })?);
            }
        }
        let report = Self {
            total_iterations: total_iterations as u64,
            gmin_stepping_count: gmin_stepping_count as u64,
            source_stepping_count: source_stepping_count as u64,
            force_accepted_points: force_accepted_points as u64,
            force_accepted_indices: indices,
            max_residual,
            avg_iterations_per_solve,
            timestep_reductions: timestep_reductions as u64,
            lte_rejections: lte_rejections as u64,
            bypassed_device_evaluations,
            failure_diagnostic: failure_diagnostic
                .as_ref()
                .map(ConvergenceAttribution::from),
            time_basis: time.map(|time| ConvergenceTimeBasis {
                sample_count: time.len() as u64,
                start_s: time[0],
                stop_s: time[time.len() - 1],
                force_accepted_times_s: times,
            }),
        };
        report.validate_with_abort(abort)?;
        check_abort(abort)?;
        Ok(report)
    }

    pub fn validate(&self) -> Result<(), String> {
        self.validate_with_abort(&rspice_core::abort_signal::NoAbort)
            .map_err(|error| error.to_string())
    }

    fn validate_with_abort(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<(), rspice_core::SimulationError> {
        check_abort(abort)?;
        if !self.max_residual.is_finite()
            || self.max_residual < 0.0
            || !self.avg_iterations_per_solve.is_finite()
            || self.avg_iterations_per_solve < 0.0
        {
            return Err(invalid(
                "Convergence residual and iteration average must be finite and nonnegative",
            ));
        }
        if self.force_accepted_points != self.force_accepted_indices.len() as u64 {
            return Err(invalid("Convergence point count is invalid"));
        }
        if let Some(basis) = &self.time_basis {
            if basis.sample_count == 0
                || !basis.start_s.is_finite()
                || basis.start_s < 0.0
                || !basis.stop_s.is_finite()
                || basis.stop_s < basis.start_s
                || ((basis.sample_count == 1) != (basis.start_s == basis.stop_s))
                || basis.force_accepted_times_s.len() != self.force_accepted_indices.len()
            {
                return Err(invalid("Convergence source time basis is invalid"));
            }
        }
        for (position, &index) in self.force_accepted_indices.iter().enumerate() {
            if position.is_multiple_of(64) {
                check_abort(abort)?;
            }
            if position > 0 && index <= self.force_accepted_indices[position - 1] {
                return Err(invalid("Convergence source-index order is invalid"));
            }
            if let Some(basis) = &self.time_basis {
                let time = basis.force_accepted_times_s[position];
                if index >= basis.sample_count
                    || !time.is_finite()
                    || time < basis.start_s
                    || time > basis.stop_s
                    || (position > 0 && time <= basis.force_accepted_times_s[position - 1])
                    || (index == 0 && time != basis.start_s)
                    || (index == basis.sample_count - 1 && time != basis.stop_s)
                {
                    return Err(invalid("Convergence source point time is invalid"));
                }
            }
        }
        if let Some(diagnostic) = &self.failure_diagnostic {
            if diagnostic.sites.len()
                > rspice_core::diagnostics::ConvergenceDiagnostic::MAX_NAMED_SITES
                || diagnostic.sites.iter().any(|site| {
                    site.name.trim().is_empty()
                        || site
                            .residual
                            .is_some_and(|value| !value.is_finite() || value < 0.0)
                })
            {
                return Err(invalid(
                    "Convergence diagnostic contains invalid named sites",
                ));
            }
        }
        check_abort(abort)
    }
}

fn check_abort(abort: &dyn AbortSignal) -> Result<(), rspice_core::SimulationError> {
    if abort.is_aborted() {
        Err(rspice_core::SimulationError::Aborted)
    } else {
        Ok(())
    }
}

fn invalid(message: &str) -> rspice_core::SimulationError {
    rspice_core::SimulationError::Circuit(message.to_owned())
}

mod decimal_u64 {
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _};

    pub(super) fn serialize<S: Serializer>(value: &u64, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&value.to_string())
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
        let encoded = String::deserialize(deserializer)?;
        let value: u64 = encoded.parse().map_err(D::Error::custom)?;
        if value.to_string() != encoded {
            return Err(D::Error::custom("noncanonical convergence counter"));
        }
        Ok(value)
    }
}

mod decimal_indices {
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _, ser::SerializeSeq};

    pub(super) fn serialize<S: Serializer>(
        values: &[u64],
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            sequence.serialize_element(&value.to_string())?;
        }
        sequence.end()
    }

    pub(super) fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<u64>, D::Error> {
        Vec::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|encoded| {
                let value: u64 = encoded.parse().map_err(D::Error::custom)?;
                if value.to_string() != encoded {
                    return Err(D::Error::custom("noncanonical convergence point index"));
                }
                Ok(value)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::diagnostics::ConvergenceQuality;

    #[test]
    fn quality_keeps_source_times_before_output_projection() {
        let mut metrics = ConvergenceQuality::default();
        metrics.record_force_accept(1);
        metrics.record_force_accept(3);
        let evidence =
            TransientConvergenceEvidence::capture(metrics, &[0.0, 0.2, 0.7, 1.0], &NoAbort)
                .unwrap();
        let basis = evidence.transient.time_basis.as_ref().unwrap();
        assert_eq!(basis.sample_count, 4);
        assert_eq!(basis.force_accepted_times_s, [0.2, 1.0]);
        assert_eq!(evidence.transient.force_accepted_indices, [1, 3]);
        evidence.validate().unwrap();
    }

    #[test]
    fn quality_round_trip_preserves_large_counters_and_extreme_floats() {
        let mut evidence = TransientConvergenceEvidence::capture(
            ConvergenceQuality::default(),
            &[0.0, 1.0],
            &NoAbort,
        )
        .unwrap();
        evidence.transient.total_iterations = u64::MAX;
        evidence.transient.bypassed_device_evaluations = (1_u64 << 53) + 1;
        evidence.transient.max_residual = f64::from_bits(1);
        evidence.transient.avg_iterations_per_solve = f64::MAX;
        let json = serde_json::to_string(&evidence).unwrap();
        assert!(json.contains("\"18446744073709551615\""));
        assert!(json.contains("\"9007199254740993\""));
        let restored: TransientConvergenceEvidence = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, evidence);
        restored.validate().unwrap();
    }

    #[test]
    fn quality_rejects_invalid_time_and_metric_evidence() {
        for time in [vec![], vec![0.0, f64::NAN], vec![0.0, 0.0], vec![-1.0, 0.0]] {
            assert!(
                TransientConvergenceEvidence::capture(
                    ConvergenceQuality::default(),
                    &time,
                    &NoAbort
                )
                .is_err()
            );
        }
        for metrics in [
            ConvergenceQuality {
                max_residual: f64::INFINITY,
                ..Default::default()
            },
            ConvergenceQuality {
                avg_iterations_per_solve: -1.0,
                ..Default::default()
            },
            ConvergenceQuality {
                force_accepted_points: 1,
                ..Default::default()
            },
            ConvergenceQuality {
                force_accepted_points: 1,
                force_accepted_indices: vec![2],
                ..Default::default()
            },
            ConvergenceQuality {
                force_accepted_points: 2,
                force_accepted_indices: vec![1, 1],
                ..Default::default()
            },
        ] {
            assert!(TransientConvergenceEvidence::capture(metrics, &[0.0, 1.0], &NoAbort).is_err());
        }
    }

    #[test]
    fn quality_capture_cancels_during_source_axis_validation() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        struct AbortAfter(AtomicUsize);
        impl AbortSignal for AbortAfter {
            fn is_aborted(&self) -> bool {
                self.0.fetch_add(1, Ordering::Relaxed) >= 2
            }
        }
        let time = (0..256).map(f64::from).collect::<Vec<_>>();
        assert!(matches!(
            TransientConvergenceEvidence::capture(
                ConvergenceQuality::default(),
                &time,
                &AbortAfter(AtomicUsize::new(0)),
            ),
            Err(rspice_core::SimulationError::Aborted)
        ));
    }
}
