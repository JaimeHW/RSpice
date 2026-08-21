//! What a result family retained that its waveforms cannot say.
//!
//! A plotted family is a set of curves. What produced them — the swept values,
//! the corner labels, the trial count and seed, the statistical samples — is
//! not recoverable from the curves, so it is retained here as source evidence
//! and persisted with the result rather than recomputed by a viewer.
//!
//! Split out of `analysis_result` so the family contract has a file of its own:
//! it is the record every multi-point and statistical analysis is judged from,
//! and it grows whenever one of them learns to retain something new.

use std::collections::{BTreeMap, HashSet};

use super::super::{FamilyMemberId, FamilyMemberMeasurements};
use super::{
    AnalysisType, MonteCarloVariableMetadata, PeriodicNoiseOutputQuantity, require_finite_values,
    require_non_empty, strictly_increasing,
};

/// Typed, lossless metadata for result families whose execution contract is
/// richer than a collection of plotted waveforms.
///
/// This payload is part of the immutable analysis result. It preserves exact
/// axes, labels, run counts, statistical samples, and convergence outcomes
/// that cannot be reconstructed truthfully from display waveforms alone.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "family", rename_all = "snake_case", deny_unknown_fields)]
pub enum AnalysisResultFamilyMetadata {
    Parametric {
        target: String,
        sweep_values: Vec<f64>,
        failed_points: usize,
        /// What each swept point measured. See [`Self::member_measurements`].
        #[serde(default)]
        member_measurements: Vec<FamilyMemberMeasurements>,
    },
    Corner {
        x_values: Vec<f64>,
        x_label: String,
        x_unit: String,
        temperatures_c: Vec<f64>,
        corner_labels: Vec<String>,
        failed_corners: usize,
        /// What each corner measured. See [`Self::member_measurements`].
        #[serde(default)]
        member_measurements: Vec<FamilyMemberMeasurements>,
    },
    MonteCarlo {
        seed: u64,
        runs_requested: usize,
        runs_completed: usize,
        failures: usize,
        all_converged: bool,
        variables: Vec<MonteCarloVariableMetadata>,
        /// What each retained trial measured. See
        /// [`Self::member_measurements`].
        #[serde(default)]
        member_measurements: Vec<FamilyMemberMeasurements>,
    },
    Reliability {
        years: Vec<f64>,
    },
    Optimization {
        iterations: Vec<f64>,
        best_cost: f64,
        best_variables: BTreeMap<String, f64>,
        converged: bool,
    },
    Soa {
        time: Vec<f64>,
    },
    /// Periodic-noise quantity and carrier authority. This prevents an
    /// output-noise PSD from being mislabeled as phase noise downstream.
    PeriodicNoise {
        output_quantity: PeriodicNoiseOutputQuantity,
        carrier_frequency_hz: Option<f64>,
    },
    /// Per-port power-wave reference impedances in physical port-number order.
    /// Smith impedance and VSWR readouts are invalid without this authority.
    SParameter {
        reference_impedances_ohm: Vec<f64>,
    },
}

impl AnalysisResultFamilyMetadata {
    /// What the family's individual members measured, one entry per member.
    ///
    /// Empty is the normal answer for most families and for every result
    /// retained before in-analysis families evaluated their members: a family
    /// that measured nothing per member is not a family whose members all
    /// passed. A specification reading this must treat empty as "no evidence
    /// from the members" and fall back to the analysis-level measurement, never
    /// as a spread it may judge.
    ///
    /// The point-family route deliberately reports nothing here. Its points are
    /// separate authorized tasks that each retain their own `AnalysisResult`,
    /// so their measurements already answer a limit through the ordinary join,
    /// and restating them on the reduction would count every point twice.
    #[must_use]
    pub fn member_measurements(&self) -> &[FamilyMemberMeasurements] {
        match self {
            Self::Parametric {
                member_measurements,
                ..
            }
            | Self::Corner {
                member_measurements,
                ..
            }
            | Self::MonteCarlo {
                member_measurements,
                ..
            } => member_measurements,
            Self::Reliability { .. }
            | Self::Optimization { .. }
            | Self::Soa { .. }
            | Self::PeriodicNoise { .. }
            | Self::SParameter { .. } => &[],
        }
    }

    /// Validate retained source evidence independently of any viewer.
    pub fn validate_for(&self, analysis_type: AnalysisType) -> Result<(), String> {
        validate_member_measurements(self)?;
        let compatible = matches!(
            (self, analysis_type),
            (Self::Parametric { .. }, AnalysisType::Parametric)
                | (Self::Corner { .. }, AnalysisType::Corner)
                | (Self::MonteCarlo { .. }, AnalysisType::MonteCarlo)
                | (Self::Reliability { .. }, AnalysisType::Reliability)
                | (Self::Optimization { .. }, AnalysisType::Optimization)
                | (Self::Soa { .. }, AnalysisType::Soa)
                | (
                    Self::PeriodicNoise { .. },
                    AnalysisType::Pnoise | AnalysisType::Qpnoise
                )
                | (
                    Self::SParameter { .. },
                    AnalysisType::SParameter | AnalysisType::Psp | AnalysisType::Hbsp
                )
        );
        if !compatible {
            return Err(format!(
                "retained family metadata does not match analysis type {analysis_type:?}"
            ));
        }

        match self {
            Self::Parametric {
                target,
                sweep_values,
                ..
            } => {
                require_non_empty(target, "parametric target")?;
                require_finite_values(sweep_values, "parametric sweep values")?;
            }
            Self::Corner {
                x_values,
                x_label,
                temperatures_c,
                corner_labels,
                ..
            } => {
                require_non_empty(x_label, "corner x-axis label")?;
                require_finite_values(x_values, "corner x-axis values")?;
                require_finite_values(temperatures_c, "corner temperatures")?;
                if temperatures_c.len() != x_values.len() || corner_labels.len() != x_values.len() {
                    return Err(
                        "corner x values, temperatures, and labels have different lengths"
                            .to_owned(),
                    );
                }
                if corner_labels.iter().any(|label| label.trim().is_empty()) {
                    return Err("corner metadata contains an empty corner label".to_owned());
                }
            }
            Self::MonteCarlo {
                runs_requested,
                runs_completed,
                failures,
                all_converged,
                variables,
                ..
            } => {
                if runs_completed.saturating_add(*failures) > *runs_requested {
                    return Err(
                        "Monte Carlo completed and failed counts exceed requested runs".to_owned(),
                    );
                }
                if *all_converged && (*failures != 0 || runs_completed != runs_requested) {
                    return Err(
                        "Monte Carlo all_converged contradicts retained run counts".to_owned()
                    );
                }
                let mut names = HashSet::with_capacity(variables.len());
                for variable in variables {
                    require_non_empty(&variable.name, "Monte Carlo variable name")?;
                    if !names.insert(variable.name.as_str()) {
                        return Err(format!(
                            "Monte Carlo metadata repeats variable '{}'",
                            variable.name
                        ));
                    }
                    require_finite_values(&variable.samples, "Monte Carlo samples")?;
                    for (label, value) in [
                        ("mean", variable.mean),
                        ("standard deviation", variable.std_dev),
                        ("minimum", variable.min),
                        ("maximum", variable.max),
                    ] {
                        if !value.is_finite() {
                            return Err(format!(
                                "Monte Carlo variable '{}' has non-finite {label}",
                                variable.name
                            ));
                        }
                    }
                    if variable.std_dev < 0.0 || variable.min > variable.max {
                        return Err(format!(
                            "Monte Carlo variable '{}' has inconsistent statistics",
                            variable.name
                        ));
                    }
                }
            }
            Self::Reliability { years } => {
                require_finite_values(years, "reliability years")?;
                if years.is_empty()
                    || years.iter().any(|years| *years <= 0.0)
                    || !strictly_increasing(years)
                {
                    return Err(
                        "reliability years must be non-empty, positive, unique, and strictly increasing"
                            .to_owned(),
                    );
                }
            }
            Self::Optimization {
                iterations,
                best_cost,
                best_variables,
                ..
            } => {
                require_finite_values(iterations, "optimization iterations")?;
                if !best_cost.is_finite() {
                    return Err("optimization best cost is non-finite".to_owned());
                }
                for (name, value) in best_variables {
                    require_non_empty(name, "optimization variable name")?;
                    if !value.is_finite() {
                        return Err(format!(
                            "optimization variable '{name}' has a non-finite best value"
                        ));
                    }
                }
            }
            Self::Soa { time } => {
                require_finite_values(time, "SOA time")?;
                if time.is_empty()
                    || time.iter().any(|time| *time < 0.0)
                    || !strictly_increasing(time)
                {
                    return Err(
                        "SOA time must be non-empty, nonnegative, unique, and strictly increasing"
                            .to_owned(),
                    );
                }
            }
            Self::PeriodicNoise {
                output_quantity,
                carrier_frequency_hz,
            } => {
                if let Some(carrier_frequency_hz) = carrier_frequency_hz
                    && (!carrier_frequency_hz.is_finite() || *carrier_frequency_hz <= 0.0)
                {
                    return Err(
                        "periodic-noise carrier frequency must be finite and positive".to_owned(),
                    );
                }
                if *output_quantity == PeriodicNoiseOutputQuantity::PhaseNoiseDbcPerHz
                    && carrier_frequency_hz.is_none()
                {
                    return Err(
                        "phase-noise evidence is missing its retained carrier frequency".to_owned(),
                    );
                }
            }
            Self::SParameter {
                reference_impedances_ohm,
            } => {
                if reference_impedances_ohm.len() < 2 {
                    return Err(
                        "S-parameter metadata requires at least two port impedances".to_owned()
                    );
                }
                if reference_impedances_ohm
                    .iter()
                    .any(|impedance| !impedance.is_finite() || *impedance <= 0.0)
                {
                    return Err(
                        "S-parameter reference impedances must be finite and positive".to_owned(),
                    );
                }
            }
        }
        Ok(())
    }
}

/// A family's retained member evidence must belong to that family.
///
/// The member identity is what a verdict names when it reports a worst case, so
/// a corner label filed under a Monte Carlo family would send an operator to
/// re-run a trial that does not exist. Checking the pairing here means no
/// consumer has to distrust the identity it was handed.
fn validate_member_measurements(metadata: &AnalysisResultFamilyMetadata) -> Result<(), String> {
    let members = metadata.member_measurements();
    if members.is_empty() {
        return Ok(());
    }

    let expected = match metadata {
        AnalysisResultFamilyMetadata::Parametric { .. } => "sweep point",
        AnalysisResultFamilyMetadata::Corner { .. } => "corner",
        AnalysisResultFamilyMetadata::MonteCarlo { .. } => "Monte Carlo trial",
        _ => return Ok(()),
    };

    let mut seen = HashSet::with_capacity(members.len());
    for member in members {
        let matches_family = matches!(
            (metadata, &member.member),
            (
                AnalysisResultFamilyMetadata::Parametric { .. },
                FamilyMemberId::SweepPoint { .. }
            ) | (
                AnalysisResultFamilyMetadata::Corner { .. },
                FamilyMemberId::Corner { .. }
            ) | (
                AnalysisResultFamilyMetadata::MonteCarlo { .. },
                FamilyMemberId::MonteCarloTrial { .. }
            )
        );
        if !matches_family {
            return Err(format!(
                "retained member evidence is not a {expected}: {:?}",
                member.member
            ));
        }
        if !seen.insert(member.member.index()) {
            return Err(format!(
                "family member evidence repeats {expected} {}",
                member.member.index()
            ));
        }
        let mut names = HashSet::with_capacity(member.measurements.len());
        for measurement in &member.measurements {
            require_non_empty(&measurement.name, "family member measurement name")?;
            if !names.insert(measurement.name.to_ascii_lowercase()) {
                return Err(format!(
                    "{expected} {} repeats measurement '{}'",
                    member.member.index(),
                    measurement.name
                ));
            }
            if measurement.value.is_some_and(|value| !value.is_finite()) {
                return Err(format!(
                    "{expected} {} retained a non-finite '{}'",
                    member.member.index(),
                    measurement.name
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod member_measurement_tests {
    use super::*;
    use crate::state::FamilyMeasurementEvidence;

    fn evidence(name: &str, value: f64) -> FamilyMeasurementEvidence {
        FamilyMeasurementEvidence {
            name: name.to_owned(),
            value: Some(value),
            passed: true,
            error: None,
        }
    }

    fn monte_carlo(members: Vec<FamilyMemberMeasurements>) -> AnalysisResultFamilyMetadata {
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed: 7,
            runs_requested: 3,
            runs_completed: 3,
            failures: 0,
            all_converged: true,
            variables: Vec::new(),
            member_measurements: members,
        }
    }

    #[test]
    fn trial_evidence_validates_and_is_readable_through_one_accessor() {
        let metadata = monte_carlo(vec![FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloTrial { index: 0, seed: 11 },
            vec![evidence("gain", 12.5)],
        )]);

        metadata
            .validate_for(AnalysisType::MonteCarlo)
            .expect("well-formed trial evidence validates");
        assert_eq!(metadata.member_measurements().len(), 1);
    }

    #[test]
    fn a_family_refuses_member_evidence_that_belongs_to_a_different_family() {
        let metadata = monte_carlo(vec![FamilyMemberMeasurements::new(
            FamilyMemberId::Corner {
                index: 0,
                label: "TT".to_owned(),
            },
            vec![evidence("gain", 12.5)],
        )]);

        let error = metadata
            .validate_for(AnalysisType::MonteCarlo)
            .expect_err("a corner is not a Monte Carlo trial");
        assert!(
            error.contains("Monte Carlo trial"),
            "the refusal must name what was expected, got: {error}"
        );
    }

    #[test]
    fn a_family_refuses_two_members_with_the_same_identity() {
        let metadata = monte_carlo(vec![
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial { index: 4, seed: 11 },
                vec![evidence("gain", 12.5)],
            ),
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial { index: 4, seed: 12 },
                vec![evidence("gain", 9.0)],
            ),
        ]);

        assert!(
            metadata.validate_for(AnalysisType::MonteCarlo).is_err(),
            "two trials numbered 4 make a worst-trial verdict unresolvable"
        );
    }

    /// A project written before families measured their members must still load,
    /// and must load as "no member evidence" rather than as a family that
    /// measured nothing.
    #[test]
    fn a_family_persisted_without_member_evidence_still_deserializes() {
        let legacy = r#"{
            "family": "monte_carlo",
            "seed": 7,
            "runs_requested": 3,
            "runs_completed": 3,
            "failures": 0,
            "all_converged": true,
            "variables": []
        }"#;

        let metadata: AnalysisResultFamilyMetadata = serde_json::from_str(legacy)
            .expect("a pre-carriage family metadata record still loads");

        assert!(metadata.member_measurements().is_empty());
        metadata
            .validate_for(AnalysisType::MonteCarlo)
            .expect("an empty carriage is valid");
    }

    #[test]
    fn member_evidence_survives_a_serde_round_trip() {
        let metadata = monte_carlo(vec![FamilyMemberMeasurements::new(
            FamilyMemberId::MonteCarloTrial { index: 2, seed: 99 },
            vec![evidence("gain", 12.5)],
        )]);

        let encoded = serde_json::to_string(&metadata).expect("family metadata serializes");
        let decoded: AnalysisResultFamilyMetadata =
            serde_json::from_str(&encoded).expect("family metadata round-trips");

        assert_eq!(decoded, metadata);
    }
}
