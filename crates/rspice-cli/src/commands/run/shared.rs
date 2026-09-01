//! Helpers shared by the `run` analysis modules: node lookup by name or
//! index, `.STEP`/frequency sweep point generation, HDF5 error mapping, and
//! the non-finite result guard behind `--allow-nonfinite`.

use crate::cli::CliError;
use rspice_core::{Engine, Netlist};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub(super) struct NodeResolver {
    pub(super) node_name_to_index: HashMap<String, usize>,
    ground_policy: rspice_core::netlist::GroundPolicy,
}

impl NodeResolver {
    pub(super) fn from_netlist(engine: &Engine, netlist: &Netlist) -> Result<Self, CliError> {
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| CliError::simulation_error_in(e.to_string(), "Node Resolution"))?;

        let node_name_to_index = circuit
            .node_names_sorted()
            .iter()
            .enumerate()
            .map(|(idx, name)| (name.to_ascii_uppercase(), idx + 1))
            .collect();

        Ok(Self {
            node_name_to_index,
            ground_policy: netlist.ground_policy(),
        })
    }

    pub(super) fn resolve_node(&self, node: &str) -> Option<usize> {
        let node = node.trim();
        if node.is_empty() {
            return None;
        }
        if self.ground_policy.is_ground(node) {
            return Some(0);
        }
        if let Ok(idx) = node.parse::<usize>() {
            return Some(idx);
        }

        self.node_name_to_index
            .get(&node.to_ascii_uppercase())
            .copied()
    }
}

pub(super) fn validate_step_sweep(sweep: &rspice_core::netlist::StepSweep) -> Result<(), CliError> {
    use rspice_core::netlist::StepSweep;

    match sweep {
        StepSweep::Linear { start, stop, step } => {
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep requires finite start/stop/step values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if *step == 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep step cannot be zero".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if (*stop > *start && *step < 0.0) || (*stop < *start && *step > 0.0) {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep step sign is inconsistent with start/stop range"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
        }
        StepSweep::Decade {
            points_per_decade,
            start,
            stop,
        } => {
            if *points_per_decade == 0 {
                return Err(CliError::SimulationError {
                    message: ".STEP DEC sweep requires points_per_decade > 0".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP DEC sweep requires finite positive start/stop values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
        }
        StepSweep::Octave {
            points_per_octave,
            start,
            stop,
        } => {
            if *points_per_octave == 0 {
                return Err(CliError::SimulationError {
                    message: ".STEP OCT sweep requires points_per_octave > 0".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP OCT sweep requires finite positive start/stop values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
        }
        StepSweep::List(values) => {
            if values.is_empty() {
                return Err(CliError::SimulationError {
                    message: ".STEP LIST requires at least one value".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if values.iter().any(|v| !v.is_finite()) {
                return Err(CliError::SimulationError {
                    message: ".STEP LIST values must be finite".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
        }
        StepSweep::Data { .. } => {}
    }

    Ok(())
}

#[cfg(test)]
fn generate_step_values(
    sweep: &rspice_core::netlist::StepSweep,
    timeout_seconds: Option<f64>,
) -> Result<Vec<f64>, CliError> {
    use rspice_core::netlist::{StepSweep, SweepPointGenerationError};

    const MAX_STEP_POINTS: usize = 1_000_000;

    validate_step_sweep(sweep)?;
    if matches!(sweep, StepSweep::Data { .. }) {
        return Ok(Vec::new());
    }

    sweep
        .values_bounded_with_abort(MAX_STEP_POINTS, &crate::abort::ProcessAbort)
        .map_err(|error| match error {
            SweepPointGenerationError::LimitExceeded { requested, limit } => {
                CliError::SimulationError {
                    message: format!(
                        ".STEP sweep requires at least {requested} points, exceeding the {limit}-point resource limit"
                    ),
                    analysis: Some("Step".to_string()),
                }
            }
            SweepPointGenerationError::Aborted => super::cancellation_cli_error(timeout_seconds),
            _ => CliError::InternalError {
                message: format!("unrecognized sweep generation failure: {error}"),
            },
        })
}

#[cfg(test)]
fn assert_linear_grid(actual: &[f64], expected: &[f64]) {
    assert_eq!(actual.len(), expected.len());
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        let tolerance = expected.abs().max(1.0e-300) * 1.0e-14;
        assert!(
            (actual - expected).abs() <= tolerance,
            "linear grid point {index}: actual={actual:.17e}, expected={expected:.17e}"
        );
    }
}

pub(super) fn generate_frequency_sweep(
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
) -> Vec<f64> {
    rspice_core::analysis::ac::ac_sweep_frequencies(variation, points, start_freq, stop_freq)
}

pub(super) fn map_hdf5_output_error(path: &Path, err: crate::hdf5::Hdf5Error) -> CliError {
    CliError::OutputError {
        path: path.to_path_buf(),
        source: std::io::Error::other(err.to_string()),
    }
}

/// Reject results containing NaN/Inf: the solver produced a non-physical
/// solution (singular system, voltage-source loop, blow-up) and exporting
/// it — or judging measurements against it — would hand automation garbage
/// with a passing exit status.
///
/// `signals` yields `(display name, series)` pairs; scalars pass a
/// one-element slice. Skipped entirely under `--allow-nonfinite`.
pub(super) fn ensure_finite_series<'a>(
    allow: bool,
    analysis: &str,
    signals: impl IntoIterator<Item = (&'a str, &'a [f64])>,
) -> Result<(), CliError> {
    if allow {
        return Ok(());
    }
    for (name, values) in signals {
        if let Some(index) = values.iter().position(|v| !v.is_finite()) {
            let location = if values.len() > 1 {
                format!(" at point {index}")
            } else {
                String::new()
            };
            return Err(CliError::SimulationError {
                message: format!(
                    "{name} is non-finite ({}){location}; the solution is not \
                     physical — check the circuit topology (e.g. conflicting \
                     voltage sources). Use --allow-nonfinite to export anyway.",
                    values[index]
                ),
                analysis: Some(analysis.to_string()),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{GroundPolicy, StepSweep};

    #[test]
    fn node_resolver_uses_the_effective_ground_policy() {
        let xyce_false = NodeResolver {
            node_name_to_index: HashMap::new(),
            ground_policy: GroundPolicy::OnlyZero,
        };
        assert_eq!(xyce_false.resolve_node("0"), Some(0));
        assert_eq!(xyce_false.resolve_node("GND"), None);

        let xyce_replace = NodeResolver {
            node_name_to_index: HashMap::new(),
            ground_policy: GroundPolicy::XyceReplace,
        };
        assert_eq!(xyce_replace.resolve_node("GND"), Some(0));
        assert_eq!(xyce_replace.resolve_node("gnd!"), Some(0));
        assert_eq!(xyce_replace.resolve_node("GROUND"), Some(0));
    }

    fn assert_step_error_contains(sweep: StepSweep, expected: &str) {
        let err = generate_step_values(&sweep, None).expect_err("invalid step sweep must fail");
        let CliError::SimulationError { message, analysis } = err else {
            panic!("expected a step simulation error");
        };
        assert_eq!(analysis.as_deref(), Some("Step"));
        assert!(
            message.contains(expected),
            "error '{message}' does not contain '{expected}'"
        );
    }

    #[test]
    fn logarithmic_step_values_follow_core_geometric_grid_without_appending_stop() {
        let dec = generate_step_values(
            &StepSweep::Decade {
                points_per_decade: 5,
                start: 1.0,
                stop: 10_001.0,
            },
            None,
        )
        .expect("valid DEC sweep");
        assert_eq!(dec.len(), 21);
        assert!((dec[20] - 10_000.0).abs() <= 1.0e-10);
        assert!(dec.iter().all(|value| *value != 10_001.0));

        let octave = generate_step_values(
            &StepSweep::Octave {
                points_per_octave: 1,
                start: 1.0,
                stop: 9.0,
            },
            None,
        )
        .expect("valid OCT sweep");
        assert_eq!(octave, vec![1.0, 2.0, 4.0, 8.0]);
    }

    #[test]
    fn linear_step_values_use_the_core_scale_aware_grid() {
        let femto = generate_step_values(
            &StepSweep::Linear {
                start: 100.0e-15,
                stop: 150.0e-15,
                step: 10.0e-15,
            },
            None,
        )
        .expect("valid femto-scale sweep");
        assert_linear_grid(
            &femto,
            &[
                100.0e-15, 110.0e-15, 120.0e-15, 130.0e-15, 140.0e-15, 150.0e-15,
            ],
        );

        let sub_atto = generate_step_values(
            &StepSweep::Linear {
                start: 1.0e-20,
                stop: 3.0e-20,
                step: 1.0e-20,
            },
            None,
        )
        .expect("valid sub-atto sweep");
        assert_linear_grid(&sub_atto, &[1.0e-20, 2.0e-20, 3.0e-20]);

        let descending = generate_step_values(
            &StepSweep::Linear {
                start: 3.0e-20,
                stop: 1.0e-20,
                step: -1.0e-20,
            },
            None,
        )
        .expect("valid descending sub-atto sweep");
        assert_linear_grid(&descending, &[3.0e-20, 2.0e-20, 1.0e-20]);

        assert_step_error_contains(
            StepSweep::Linear {
                start: 0.0,
                stop: 1_000_000.0,
                step: 1.0,
            },
            "resource limit",
        );
    }

    #[test]
    fn logarithmic_step_values_include_an_exact_grid_stop() {
        let dec = generate_step_values(
            &StepSweep::Decade {
                points_per_decade: 1,
                start: 1.0,
                stop: 10_000.0,
            },
            None,
        )
        .expect("valid exact-grid DEC sweep");
        assert_eq!(dec, vec![1.0, 10.0, 100.0, 1_000.0, 10_000.0]);

        let octave = generate_step_values(
            &StepSweep::Octave {
                points_per_octave: 1,
                start: 1.0,
                stop: 8.0,
            },
            None,
        )
        .expect("valid exact-grid OCT sweep");
        assert_eq!(octave, vec![1.0, 2.0, 4.0, 8.0]);
    }

    #[test]
    fn descending_logarithmic_step_values_match_core_one_start_semantics() {
        let dec = generate_step_values(
            &StepSweep::Decade {
                points_per_decade: 5,
                start: 100.0,
                stop: 1.0,
            },
            None,
        )
        .expect("valid descending DEC sweep");
        assert_eq!(dec, vec![100.0]);

        let octave = generate_step_values(
            &StepSweep::Octave {
                points_per_octave: 3,
                start: 8.0,
                stop: 1.0,
            },
            None,
        )
        .expect("valid descending OCT sweep");
        assert_eq!(octave, vec![8.0]);
    }

    #[test]
    fn logarithmic_step_values_preserve_strong_cli_validation() {
        assert_step_error_contains(
            StepSweep::Decade {
                points_per_decade: 0,
                start: 1.0,
                stop: 10.0,
            },
            "points_per_decade > 0",
        );
        assert_step_error_contains(
            StepSweep::Decade {
                points_per_decade: 5,
                start: 0.0,
                stop: 10.0,
            },
            "finite positive start/stop",
        );
        assert_step_error_contains(
            StepSweep::Octave {
                points_per_octave: 0,
                start: 1.0,
                stop: 8.0,
            },
            "points_per_octave > 0",
        );
        assert_step_error_contains(
            StepSweep::Octave {
                points_per_octave: 1,
                start: 1.0,
                stop: f64::INFINITY,
            },
            "finite positive start/stop",
        );
    }

    #[test]
    fn finite_series_gate_rejects_nonfinite_values_unless_allowed() {
        let values = [f64::INFINITY];
        let err = ensure_finite_series(false, "DC OP", [("V(out)", values.as_slice())])
            .expect_err("non-finite values should fail by default");

        let CliError::SimulationError { message, analysis } = err else {
            panic!("expected simulation error for non-finite value");
        };
        assert_eq!(analysis.as_deref(), Some("DC OP"));
        assert!(message.contains("V(out) is non-finite"));
        assert!(message.contains("--allow-nonfinite"));

        ensure_finite_series(true, "DC OP", [("V(out)", values.as_slice())])
            .expect("allow flag should bypass non-finite export gate");
    }
}
