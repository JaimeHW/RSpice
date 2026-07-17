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

    pub(super) fn parse_voltage_probe(&self, spec: &str) -> Option<(usize, usize)> {
        let (pos_spec, neg_spec) = parse_voltage_probe_spec(spec)?;
        let pos = self.resolve_node(&pos_spec)?;
        let neg = match neg_spec {
            Some(ref_name) => self.resolve_node(&ref_name)?,
            None => 0,
        };
        Some((pos, neg))
    }
}

pub(super) fn parse_voltage_probe_spec(spec: &str) -> Option<(String, Option<String>)> {
    let trimmed = spec.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.len() >= 3
        && trimmed.get(..2).map(|s| s.eq_ignore_ascii_case("V(")) == Some(true)
        && trimmed.ends_with(')')
    {
        let inner = &trimmed[2..trimmed.len() - 1];
        let mut parts = inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty());
        let pos = parts.next()?.to_string();
        let neg = parts.next().map(|s| s.to_string());
        if parts.next().is_some() {
            return None;
        }
        return Some((pos, neg));
    }

    Some((trimmed.to_string(), None))
}

pub(super) fn generate_step_values(
    sweep: &rspice_core::netlist::StepSweep,
) -> Result<Vec<f64>, CliError> {
    use rspice_core::netlist::StepSweep;

    const MAX_STEP_POINTS: usize = 1_000_000;

    match sweep {
        StepSweep::Linear { start, stop, step } => {
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep requires finite start/stop/step values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if step.abs() <= f64::EPSILON {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep step cannot be zero".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if (*stop - *start).abs() <= f64::EPSILON {
                return Ok(vec![*start]);
            }
            if (*stop - *start) * *step < 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep step sign is inconsistent with start/stop range"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }

            let mut values = Vec::new();
            let mut v = *start;
            let tol = start.abs().max(stop.abs()).max(1.0) * 1e-12;

            for _ in 0..MAX_STEP_POINTS {
                values.push(v);
                let next = v + *step;
                let finished = if *step > 0.0 {
                    next > *stop + tol
                } else {
                    next < *stop - tol
                };
                if finished {
                    return Ok(values);
                }
                v = next;
            }

            Err(CliError::SimulationError {
                message: format!(
                    ".STEP linear sweep exceeded {} points; check start/stop/step values",
                    MAX_STEP_POINTS
                ),
                analysis: Some("Step".to_string()),
            })
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
            let values = sweep.values();
            if values.len() > MAX_STEP_POINTS {
                return Err(CliError::SimulationError {
                    message: format!(
                        ".STEP DEC sweep exceeded {} points; reduce the range or points_per_decade",
                        MAX_STEP_POINTS
                    ),
                    analysis: Some("Step".to_string()),
                });
            }
            Ok(values)
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
            let values = sweep.values();
            if values.len() > MAX_STEP_POINTS {
                return Err(CliError::SimulationError {
                    message: format!(
                        ".STEP OCT sweep exceeded {} points; reduce the range or points_per_octave",
                        MAX_STEP_POINTS
                    ),
                    analysis: Some("Step".to_string()),
                });
            }
            Ok(values)
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
            Ok(values.clone())
        }
        StepSweep::Data { .. } => Ok(Vec::new()),
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
        let err = generate_step_values(&sweep).expect_err("invalid step sweep must fail");
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
        let dec = generate_step_values(&StepSweep::Decade {
            points_per_decade: 5,
            start: 1.0,
            stop: 10_001.0,
        })
        .expect("valid DEC sweep");
        assert_eq!(dec.len(), 21);
        assert!((dec[20] - 10_000.0).abs() <= 1.0e-10);
        assert!(dec.iter().all(|value| *value != 10_001.0));

        let octave = generate_step_values(&StepSweep::Octave {
            points_per_octave: 1,
            start: 1.0,
            stop: 9.0,
        })
        .expect("valid OCT sweep");
        assert_eq!(octave, vec![1.0, 2.0, 4.0, 8.0]);
    }

    #[test]
    fn logarithmic_step_values_include_an_exact_grid_stop() {
        let dec = generate_step_values(&StepSweep::Decade {
            points_per_decade: 1,
            start: 1.0,
            stop: 10_000.0,
        })
        .expect("valid exact-grid DEC sweep");
        assert_eq!(dec, vec![1.0, 10.0, 100.0, 1_000.0, 10_000.0]);

        let octave = generate_step_values(&StepSweep::Octave {
            points_per_octave: 1,
            start: 1.0,
            stop: 8.0,
        })
        .expect("valid exact-grid OCT sweep");
        assert_eq!(octave, vec![1.0, 2.0, 4.0, 8.0]);
    }

    #[test]
    fn descending_logarithmic_step_values_match_core_one_start_semantics() {
        let dec = generate_step_values(&StepSweep::Decade {
            points_per_decade: 5,
            start: 100.0,
            stop: 1.0,
        })
        .expect("valid descending DEC sweep");
        assert_eq!(dec, vec![100.0]);

        let octave = generate_step_values(&StepSweep::Octave {
            points_per_octave: 3,
            start: 8.0,
            stop: 1.0,
        })
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
