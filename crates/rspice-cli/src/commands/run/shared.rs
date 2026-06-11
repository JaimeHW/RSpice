use crate::cli::CliError;
use rspice_core::{Engine, Netlist};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone)]
pub(super) struct NodeResolver {
    pub(super) node_name_to_index: HashMap<String, usize>,
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

        Ok(Self { node_name_to_index })
    }

    pub(super) fn resolve_node(&self, node: &str) -> Option<usize> {
        let node = node.trim();
        if node.is_empty() {
            return None;
        }
        if node == "0" || node.eq_ignore_ascii_case("gnd") {
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
            if (*stop - *start).abs() <= f64::EPSILON {
                return Ok(vec![*start]);
            }

            let ascending = stop > start;
            let ratio = if ascending {
                stop / start
            } else {
                start / stop
            };
            let total_steps = (ratio.log10() * (*points_per_decade as f64)).ceil() as usize;
            let tol = stop.abs().max(1.0) * 1e-12;

            let mut values = Vec::with_capacity(total_steps + 2);
            for i in 0..=total_steps {
                let factor = 10_f64.powf(i as f64 / *points_per_decade as f64);
                let value = if ascending {
                    start * factor
                } else {
                    start / factor
                };

                let in_range = if ascending {
                    value <= *stop + tol
                } else {
                    value >= *stop - tol
                };
                if in_range {
                    values.push(value);
                }
            }

            if values.is_empty() {
                values.push(*start);
            }
            let last = *values.last().unwrap_or(start);
            if (last - *stop).abs() > tol {
                values.push(*stop);
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
            if (*stop - *start).abs() <= f64::EPSILON {
                return Ok(vec![*start]);
            }

            let ascending = stop > start;
            let ratio = if ascending {
                stop / start
            } else {
                start / stop
            };
            let total_steps = (ratio.log2() * (*points_per_octave as f64)).ceil() as usize;
            let tol = stop.abs().max(1.0) * 1e-12;

            let mut values = Vec::with_capacity(total_steps + 2);
            for i in 0..=total_steps {
                let factor = 2_f64.powf(i as f64 / *points_per_octave as f64);
                let value = if ascending {
                    start * factor
                } else {
                    start / factor
                };

                let in_range = if ascending {
                    value <= *stop + tol
                } else {
                    value >= *stop - tol
                };
                if in_range {
                    values.push(value);
                }
            }

            if values.is_empty() {
                values.push(*start);
            }
            let last = *values.last().unwrap_or(start);
            if (last - *stop).abs() > tol {
                values.push(*stop);
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
