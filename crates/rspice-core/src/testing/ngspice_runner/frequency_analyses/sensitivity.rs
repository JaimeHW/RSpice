use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn run_sensitivity_test(
        &self,
        name: &str,
        source: &str,
        output_pos: &str,
        output_neg: Option<&str>,
        sweep: Option<(AcSweepType, usize, Value, Value)>,
        start: std::time::Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        };

        let resolver_engine = self.create_dynamic_engine();
        let circuit = match resolver_engine.build_circuit(&netlist) {
            Ok(circuit) => circuit,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Circuit build error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        };

        let output_pos_idx =
            match Self::resolve_circuit_node_index(&circuit, output_pos, "sensitivity output+") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Sensitivity".to_string()),
                    };
                }
            };
        let output_neg_idx = match Self::resolve_optional_circuit_node_index(
            &circuit,
            output_neg,
            "sensitivity output-",
        ) {
            Ok(idx) => idx,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        };

        let mut params: Vec<(String, Value)> = netlist
            .params
            .all_params()
            .into_iter()
            .filter(|(param_name, value)| {
                !param_name.starts_with("IC_")
                    && !param_name.starts_with("NODESET_")
                    && value.is_finite()
                    && value.abs() > 0.0
            })
            .collect();
        params.sort_by(|a, b| a.0.cmp(&b.0));
        let engine = self.create_dynamic_engine();
        if params.is_empty() {
            if sweep.is_some() {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(
                        "AC .SENS currently requires at least one non-zero top-level .PARAM"
                            .to_string(),
                    ),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity AC".to_string()),
                };
            }

            return match engine.run_sensitivity_linearized(&netlist, output_pos_idx, output_neg_idx)
            {
                Ok(result) if !result.sensitivities.is_empty() => TestResult {
                    name: name.to_string(),
                    passed: true,
                    error: None,
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                },
                Ok(_) => TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(
                        "Sensitivity analysis found no eligible elements to differentiate"
                            .to_string(),
                    ),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                },
                Err(err) => TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Simulation error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                },
            };
        }

        if let Some((sweep_type, points, fstart, fstop)) = sweep {
            let frequencies = match sweep_type {
                AcSweepType::Dec => self.generate_decade_points(fstart, fstop, points),
                AcSweepType::Oct => self.generate_octave_points(fstart, fstop, points),
                AcSweepType::Lin => self.generate_linear_points(fstart, fstop, points),
            };
            if frequencies.is_empty() {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some("Invalid .SENS AC frequency sweep configuration".to_string()),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity AC".to_string()),
                };
            }

            for (param_name, param_value) in &params {
                let pos = match engine.run_sensitivity_ac(
                    &netlist,
                    output_pos_idx,
                    param_name,
                    *param_value,
                    &frequencies,
                    None,
                ) {
                    Ok(values) => values,
                    Err(err) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Simulation error: {}", err)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("Sensitivity AC".to_string()),
                        };
                    }
                };
                if let Some(neg_idx) = output_neg_idx
                    && let Err(err) = engine.run_sensitivity_ac(
                        &netlist,
                        neg_idx,
                        param_name,
                        *param_value,
                        &frequencies,
                        None,
                    )
                {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!("Simulation error: {}", err)),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Sensitivity AC".to_string()),
                    };
                }
                if pos.is_empty() {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(format!(
                            "Sensitivity AC returned no samples for parameter '{}'",
                            param_name
                        )),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Sensitivity AC".to_string()),
                    };
                }
            }

            return TestResult {
                name: name.to_string(),
                passed: true,
                error: None,
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("Sensitivity AC".to_string()),
            };
        }

        for (param_name, param_value) in &params {
            if let Err(err) =
                engine.run_sensitivity(&netlist, output_pos_idx, param_name, *param_value, None)
            {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Simulation error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
            if let Some(neg_idx) = output_neg_idx
                && let Err(err) =
                    engine.run_sensitivity(&netlist, neg_idx, param_name, *param_value, None)
            {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Simulation error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Sensitivity".to_string()),
                };
            }
        }

        TestResult {
            name: name.to_string(),
            passed: true,
            error: None,
            mismatches: Vec::new(),
            duration_ms: start.elapsed().as_millis(),
            analysis_type: Some("Sensitivity".to_string()),
        }
    }
}
