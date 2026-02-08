//! Shared output specification parsing/evaluation helpers.
//!
//! These helpers are used by both the legacy services simulation runner and
//! the engine bridge path to keep sensitivity/output behavior identical.

use num_complex::Complex64;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::Engine;
use rspice_core::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OutputVoltageSpec {
    pub pos: usize,
    pub neg: Option<usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OutputSpec {
    Voltage(OutputVoltageSpec),
    BranchCurrent {
        branch_ordinal: usize, // 1-based branch ordinal from CircuitData
        branch_name: String,
    },
}

#[inline]
fn parse_branch_current_name(output_var: &str) -> Option<&str> {
    let trimmed = output_var.trim();
    if trimmed.len() <= 3 || !trimmed[..2].eq_ignore_ascii_case("I(") || !trimmed.ends_with(')') {
        return None;
    }
    let branch_name = trimmed[2..trimmed.len() - 1].trim();
    if branch_name.is_empty() {
        return None;
    }
    Some(branch_name)
}

#[inline]
pub(crate) fn is_branch_current_output(output_var: &str) -> bool {
    parse_branch_current_name(output_var).is_some()
}

#[inline]
pub(crate) fn sensitivity_raw_unit(output_var: &str) -> &'static str {
    if is_branch_current_output(output_var) {
        "A/unit"
    } else {
        "V/unit"
    }
}

#[inline]
fn is_ground_node(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "0" | "gnd" | "ground"
    )
}

#[inline]
fn resolve_node_index(node_name: &str, node_names: &[String]) -> Option<usize> {
    let trimmed = node_name.trim();
    if trimmed.is_empty() {
        return None;
    }

    if let Ok(idx) = trimmed.parse::<usize>() {
        if idx < node_names.len() {
            return Some(idx);
        }
        return None;
    }

    let upper = trimmed.to_ascii_uppercase();
    node_names
        .iter()
        .position(|name| name.to_ascii_uppercase() == upper)
}

#[inline]
pub(crate) fn resolve_node_or_ground_index(node_name: &str, node_names: &[String]) -> Option<usize> {
    let trimmed = node_name.trim();
    if trimmed.is_empty() {
        return None;
    }
    if is_ground_node(trimmed) {
        return Some(0);
    }
    resolve_node_index(trimmed, node_names)
}

pub(crate) fn parse_output_spec(
    output_var: &str,
    node_names: &[String],
    circuit: &rspice_core::CircuitData,
) -> Option<OutputSpec> {
    let trimmed = output_var.trim();
    if let Some(branch_name) = parse_branch_current_name(trimmed) {
        let branch_ordinal = circuit.get_branch_by_name(branch_name)? as usize;
        return Some(OutputSpec::BranchCurrent {
            branch_ordinal,
            branch_name: branch_name.to_string(),
        });
    }

    parse_output_voltage_spec(trimmed, node_names).map(OutputSpec::Voltage)
}

pub(crate) fn parse_output_voltage_spec(
    output_var: &str,
    node_names: &[String],
) -> Option<OutputVoltageSpec> {
    let trimmed = output_var.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.len() > 3 && trimmed[..2].eq_ignore_ascii_case("V(") && trimmed.ends_with(')') {
        let inner = trimmed[2..trimmed.len() - 1].trim();
        if inner.is_empty() {
            return None;
        }

        if let Some((pos, neg)) = inner.split_once(',') {
            let pos_idx = resolve_node_or_ground_index(pos.trim(), node_names)?;
            let neg_idx = resolve_node_or_ground_index(neg.trim(), node_names)?;
            return Some(OutputVoltageSpec {
                pos: pos_idx,
                neg: Some(neg_idx),
            });
        }

        let pos_idx = resolve_node_or_ground_index(inner, node_names)?;
        return Some(OutputVoltageSpec {
            pos: pos_idx,
            neg: None,
        });
    }

    // I(...) output handling is owned by parse_output_spec().
    if is_branch_current_output(trimmed) {
        return None;
    }

    let pos_idx = resolve_node_or_ground_index(trimmed, node_names)?;
    Some(OutputVoltageSpec {
        pos: pos_idx,
        neg: None,
    })
}

pub(crate) fn parse_output_node(output_var: &str, node_names: &[String]) -> Option<usize> {
    parse_output_voltage_spec(output_var, node_names).and_then(|spec| {
        if spec.neg.is_none() {
            Some(spec.pos)
        } else {
            None
        }
    })
}

pub(crate) fn dc_output_value(
    dc_result: &rspice_core::SimulationResult,
    output_spec: &OutputSpec,
) -> Result<Value, String> {
    match output_spec {
        OutputSpec::Voltage(vspec) => {
            let v_pos = if vspec.pos == 0 {
                0.0
            } else {
                dc_result.node_voltages.get(vspec.pos).copied().ok_or_else(|| {
                    format!(
                        "Voltage output node index {} is out of range ({} available)",
                        vspec.pos,
                        dc_result.node_voltages.len()
                    )
                })?
            };
            let v_neg = match vspec.neg {
                Some(0) => 0.0,
                Some(idx) => dc_result.node_voltages.get(idx).copied().ok_or_else(|| {
                    format!(
                        "Voltage reference node index {} is out of range ({} available)",
                        idx,
                        dc_result.node_voltages.len()
                    )
                })?,
                None => 0.0,
            };
            Ok(v_pos - v_neg)
        }
        OutputSpec::BranchCurrent {
            branch_ordinal,
            branch_name,
        } => {
            let idx = branch_ordinal.saturating_sub(1);
            dc_result.branch_currents.get(idx).copied().ok_or_else(|| {
                format!(
                    "Branch current for '{}' is unavailable (index {})",
                    branch_name, idx
                )
            })
        }
    }
}

pub(crate) fn ac_output_value(ac_result: &AcResult, output_spec: &OutputSpec) -> Result<Complex64, String> {
    match output_spec {
        OutputSpec::Voltage(vspec) => {
            let v_pos = if vspec.pos == 0 {
                Complex64::new(0.0, 0.0)
            } else {
                ac_result
                    .voltages
                    .get(vspec.pos.saturating_sub(1))
                    .copied()
                    .ok_or_else(|| {
                        format!(
                            "AC voltage output node index {} is out of range ({} available)",
                            vspec.pos,
                            ac_result.voltages.len()
                        )
                    })?
            };
            let v_neg = match vspec.neg {
                Some(0) => Complex64::new(0.0, 0.0),
                Some(idx) => ac_result
                    .voltages
                    .get(idx.saturating_sub(1))
                    .copied()
                    .ok_or_else(|| {
                        format!(
                            "AC voltage reference node index {} is out of range ({} available)",
                            idx,
                            ac_result.voltages.len()
                        )
                    })?,
                None => Complex64::new(0.0, 0.0),
            };
            Ok(v_pos - v_neg)
        }
        OutputSpec::BranchCurrent {
            branch_ordinal,
            branch_name,
        } => {
            let idx = branch_ordinal.saturating_sub(1);
            ac_result.currents.get(idx).copied().ok_or_else(|| {
                format!(
                    "AC branch current for '{}' is unavailable (index {})",
                    branch_name, idx
                )
            })
        }
    }
}

pub(crate) fn run_ac_output_at_frequency(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_spec: &OutputSpec,
    frequency: Value,
) -> Result<Complex64, String> {
    let ac_results = engine
        .run_ac(netlist, &[frequency])
        .map_err(|e| format!("AC analysis error at {} Hz: {}", frequency, e))?;
    let point = ac_results
        .first()
        .ok_or_else(|| format!("AC analysis produced no data at {} Hz", frequency))?;
    ac_output_value(point, output_spec)
}

pub(crate) fn run_dc_output_sensitivity(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    output_spec: OutputVoltageSpec,
    param_name: &str,
    param_value: Value,
) -> Result<Value, String> {
    let pos_sensitivity = if output_spec.pos == 0 {
        0.0
    } else {
        engine
            .run_sensitivity(netlist, output_spec.pos, param_name, param_value, None)
            .map_err(|e| e.to_string())?
    };

    let neg_sensitivity = match output_spec.neg {
        Some(0) | None => 0.0,
        Some(idx) => engine
            .run_sensitivity(netlist, idx, param_name, param_value, None)
            .map_err(|e| e.to_string())?,
    };

    Ok(pos_sensitivity - neg_sensitivity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_node_or_ground_index_helper() {
        let names = vec![
            "0".to_string(),
            "IN".to_string(),
            "OUT".to_string(),
            "VDD".to_string(),
        ];

        assert_eq!(resolve_node_or_ground_index("IN", &names), Some(1));
        assert_eq!(resolve_node_or_ground_index("out", &names), Some(2));
        assert_eq!(resolve_node_or_ground_index("gnd", &names), Some(0));
        assert_eq!(resolve_node_or_ground_index("3", &names), Some(3));
        assert_eq!(resolve_node_or_ground_index("99", &names), None);
    }

    #[test]
    fn test_parse_output_node_helper() {
        let names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
        assert_eq!(parse_output_node("V(OUT)", &names), Some(2));
        assert_eq!(parse_output_node("out", &names), Some(2));
        assert_eq!(parse_output_node("2", &names), Some(2));
        assert_eq!(parse_output_node("V(OUT,IN)", &names), None);
        assert_eq!(parse_output_node("I(R1)", &names), None);
        assert_eq!(parse_output_node("99", &names), None);
    }

    #[test]
    fn test_parse_output_voltage_spec_helper() {
        let names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
        assert_eq!(
            parse_output_voltage_spec("V(OUT)", &names),
            Some(OutputVoltageSpec { pos: 2, neg: None })
        );
        assert_eq!(
            parse_output_voltage_spec("V(OUT,IN)", &names),
            Some(OutputVoltageSpec {
                pos: 2,
                neg: Some(1)
            })
        );
        assert_eq!(
            parse_output_voltage_spec("V(OUT,GND)", &names),
            Some(OutputVoltageSpec {
                pos: 2,
                neg: Some(0)
            })
        );
        assert_eq!(parse_output_voltage_spec("I(R1)", &names), None);
        assert_eq!(parse_output_voltage_spec("99", &names), None);
    }

    #[test]
    fn test_parse_output_spec_current_helper() {
        let netlist = rspice_core::netlist::parse_netlist("* t\nV1 in 0 1\nR1 in 0 1k\n")
            .expect("netlist should parse");
        let engine = Engine::new(rspice_core::SimulationConfig::default());
        let circuit = engine
            .build_circuit(&netlist)
            .expect("circuit build should succeed");
        let node_names = vec!["0".to_string(), "IN".to_string()];

        let spec = parse_output_spec("I(V1)", &node_names, &circuit);
        assert!(matches!(
            spec,
            Some(OutputSpec::BranchCurrent {
                branch_ordinal: 1,
                ..
            })
        ));
    }

    #[test]
    fn test_branch_current_output_detection() {
        assert!(is_branch_current_output("I(V1)"));
        assert!(is_branch_current_output(" i(vsrc) "));
        assert!(!is_branch_current_output("I()"));
        assert!(!is_branch_current_output("V(out)"));
        assert!(!is_branch_current_output("out"));
    }

    #[test]
    fn test_sensitivity_raw_unit_helper() {
        assert_eq!(sensitivity_raw_unit("I(V1)"), "A/unit");
        assert_eq!(sensitivity_raw_unit(" i(v1) "), "A/unit");
        assert_eq!(sensitivity_raw_unit("V(out)"), "V/unit");
        assert_eq!(sensitivity_raw_unit("out"), "V/unit");
    }

    #[test]
    fn test_dc_output_value_voltage_and_branch() {
        let mut dc = rspice_core::SimulationResult::new(2, 1);
        dc.node_voltages[1] = 1.5;
        dc.node_voltages[2] = 0.5;
        dc.branch_currents[0] = -2e-3;

        let v = dc_output_value(
            &dc,
            &OutputSpec::Voltage(OutputVoltageSpec {
                pos: 2,
                neg: Some(1),
            }),
        )
        .expect("differential voltage should resolve");
        assert!((v + 1.0).abs() < 1e-15);

        let i = dc_output_value(
            &dc,
            &OutputSpec::BranchCurrent {
                branch_ordinal: 1,
                branch_name: "V1".to_string(),
            },
        )
        .expect("branch current should resolve");
        assert!((i + 2e-3).abs() < 1e-15);
    }

    #[test]
    fn test_dc_output_value_reports_out_of_range_node() {
        let dc = rspice_core::SimulationResult::new(1, 0);
        let err = dc_output_value(
            &dc,
            &OutputSpec::Voltage(OutputVoltageSpec { pos: 2, neg: None }),
        )
        .expect_err("out-of-range output node should error");
        assert!(err.contains("out of range"));
    }

    #[test]
    fn test_ac_output_value_voltage_and_branch() {
        let ac = AcResult {
            frequency: 1e3,
            voltages: vec![Complex64::new(2.0, 0.0), Complex64::new(1.0, 0.5)],
            currents: vec![Complex64::new(-1e-3, 2e-4)],
        };

        let v = ac_output_value(
            &ac,
            &OutputSpec::Voltage(OutputVoltageSpec {
                pos: 2,
                neg: Some(1),
            }),
        )
        .expect("differential AC voltage should resolve");
        assert!((v.re + 1.0).abs() < 1e-15);
        assert!((v.im - 0.5).abs() < 1e-15);

        let i = ac_output_value(
            &ac,
            &OutputSpec::BranchCurrent {
                branch_ordinal: 1,
                branch_name: "V1".to_string(),
            },
        )
        .expect("AC branch current should resolve");
        assert!((i.re + 1e-3).abs() < 1e-15);
        assert!((i.im - 2e-4).abs() < 1e-15);
    }

    #[test]
    fn test_ac_output_value_reports_out_of_range_node() {
        let ac = AcResult {
            frequency: 1e3,
            voltages: vec![Complex64::new(2.0, 0.0)],
            currents: vec![],
        };
        let err = ac_output_value(
            &ac,
            &OutputSpec::Voltage(OutputVoltageSpec { pos: 2, neg: None }),
        )
        .expect_err("out-of-range output node should error");
        assert!(err.contains("out of range"));
    }
}
