//! AC, pole-zero, noise, sensitivity, and transfer-function regression runners.

use super::*;

impl TestRunner {
    pub(super) fn run_ac_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        sweep_type: AcSweepType,
        points: usize,
        fstart: Value,
        fstop: Value,
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
                    analysis_type: Some("AC".to_string()),
                };
            }
        };

        // Generate frequency points based on sweep type
        let frequencies = match sweep_type {
            AcSweepType::Dec => self.generate_decade_points(fstart, fstop, points),
            AcSweepType::Oct => self.generate_octave_points(fstart, fstop, points),
            AcSweepType::Lin => self.generate_linear_points(fstart, fstop, points),
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_ac(&netlist, &frequencies);
        let ac_result = match primary_result {
            Ok(results) => Ok(results),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => {
                robust_engine.run_ac(&netlist, &frequencies)
            }
            Err(err) => Err(err),
        };

        match ac_result {
            Ok(results) => {
                let mismatches = match self.compare_ac_reference(cir_path, &netlist, &results) {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("AC".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("AC".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("AC".to_string()),
            },
        }
    }

    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Frequency Point Generation
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•

    #[inline]
    pub(super) fn optional_probe_node(name: &str) -> Option<String> {
        let trimmed = name.trim();
        if trimmed.is_empty() || trimmed == "0" || trimmed.eq_ignore_ascii_case("gnd") {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    pub(super) fn resolve_circuit_node_index(
        circuit: &crate::circuit::Circuit,
        node: &str,
        role: &str,
    ) -> Result<usize, String> {
        circuit
            .get_node_by_name(node)
            .ok_or_else(|| format!("Unknown {role} node '{node}'"))
    }

    pub(super) fn resolve_optional_circuit_node_index(
        circuit: &crate::circuit::Circuit,
        node: Option<&str>,
        role: &str,
    ) -> Result<Option<usize>, String> {
        match node {
            Some(name) => Ok(Some(Self::resolve_circuit_node_index(circuit, name, role)?)),
            None => Ok(None),
        }
    }

    pub(super) fn run_pz_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        input_pos: &str,
        input_neg: Option<&str>,
        output_pos: &str,
        output_neg: Option<&str>,
        input_is_current: bool,
        compute_poles: bool,
        compute_zeros: bool,
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
                    analysis_type: Some("PZ".to_string()),
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
                    analysis_type: Some("PZ".to_string()),
                };
            }
        };

        let input_pos_idx = match Self::resolve_circuit_node_index(&circuit, input_pos, "PZ input+")
        {
            Ok(idx) => idx,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("PZ".to_string()),
                };
            }
        };
        let input_neg_idx =
            match Self::resolve_optional_circuit_node_index(&circuit, input_neg, "PZ input-") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("PZ".to_string()),
                    };
                }
            };
        let output_pos_idx =
            match Self::resolve_circuit_node_index(&circuit, output_pos, "PZ output+") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("PZ".to_string()),
                    };
                }
            };
        let output_neg_idx =
            match Self::resolve_optional_circuit_node_index(&circuit, output_neg, "PZ output-") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("PZ".to_string()),
                    };
                }
            };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_pz_ports(
            &netlist,
            input_pos_idx,
            input_neg_idx,
            output_pos_idx,
            output_neg_idx,
            input_is_current,
            compute_poles,
            compute_zeros,
        );
        let pz_result = match primary_result {
            Ok(result) => Ok(result),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => robust_engine
                .run_pz_ports(
                    &netlist,
                    input_pos_idx,
                    input_neg_idx,
                    output_pos_idx,
                    output_neg_idx,
                    input_is_current,
                    compute_poles,
                    compute_zeros,
                ),
            Err(err) => Err(err),
        };

        match pz_result {
            Ok(result) => {
                let mismatches = match self.compare_pz_reference(cir_path, &result) {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("PZ".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("PZ".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("PZ".to_string()),
            },
        }
    }

    pub(super) fn run_noise_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        output_pos: &str,
        output_neg: Option<&str>,
        input_source: &str,
        sweep_type: AcSweepType,
        points: usize,
        fstart: Value,
        fstop: Value,
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
                    analysis_type: Some("Noise".to_string()),
                };
            }
        };

        let frequencies = match sweep_type {
            AcSweepType::Dec => self.generate_decade_points(fstart, fstop, points),
            AcSweepType::Oct => self.generate_octave_points(fstart, fstop, points),
            AcSweepType::Lin => self.generate_linear_points(fstart, fstop, points),
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
                    analysis_type: Some("Noise".to_string()),
                };
            }
        };

        let output_pos_idx =
            match Self::resolve_circuit_node_index(&circuit, output_pos, "noise output+") {
                Ok(idx) => idx,
                Err(err) => {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("Noise".to_string()),
                    };
                }
            };
        let output_neg_idx = match Self::resolve_optional_circuit_node_index(
            &circuit,
            output_neg,
            "noise output-",
        ) {
            Ok(idx) => idx,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Noise".to_string()),
                };
            }
        };

        let primary_engine = self.create_dynamic_engine();
        let robust_engine = self.create_dc_engine();
        let primary_result = primary_engine.run_noise_with_input_source(
            &netlist,
            output_pos_idx,
            output_neg_idx,
            input_source,
            &frequencies,
            300.15,
        );
        let noise_result = match primary_result {
            Ok(result) => Ok(result),
            Err(err) if Self::is_recoverable_dc_convergence_error(&err) => robust_engine
                .run_noise_with_input_source(
                    &netlist,
                    output_pos_idx,
                    output_neg_idx,
                    input_source,
                    &frequencies,
                    300.15,
                ),
            Err(err) => Err(err),
        };

        match noise_result {
            Ok(results) => {
                let mismatches = match self.compare_noise_reference(cir_path, &results) {
                    Ok(m) => m,
                    Err(e) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Reference comparison error: {}", e)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("Noise".to_string()),
                        };
                    }
                };
                let passed = mismatches.is_empty();
                TestResult {
                    name: name.to_string(),
                    passed,
                    error: if passed {
                        None
                    } else {
                        Some(format!("{} reference mismatch(es)", mismatches.len()))
                    },
                    mismatches,
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("Noise".to_string()),
                }
            }
            Err(e) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Simulation error: {}", e)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("Noise".to_string()),
            },
        }
    }

    pub(super) fn run_sensitivity_test(
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

    pub(super) fn transfer_output_value_ac(
        &self,
        result: &crate::analysis::AcResult,
        output: &str,
    ) -> Result<Value, String> {
        if let Some((pos, neg)) = Self::parse_voltage_probe(output) {
            let pos_value = if pos.eq_ignore_ascii_case("0") || pos.eq_ignore_ascii_case("gnd") {
                Complex64::new(0.0, 0.0)
            } else {
                let idx = result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(&pos))
                    .ok_or_else(|| format!("Unknown TF output voltage node '{}'", pos))?;
                result
                    .voltages
                    .get(idx)
                    .copied()
                    .unwrap_or_else(|| Complex64::new(0.0, 0.0))
            };
            let neg_value = match neg.as_deref() {
                None => Complex64::new(0.0, 0.0),
                Some(name)
                    if name.eq_ignore_ascii_case("0") || name.eq_ignore_ascii_case("gnd") =>
                {
                    Complex64::new(0.0, 0.0)
                }
                Some(name) => {
                    let idx = result
                        .node_names
                        .iter()
                        .position(|candidate| candidate.eq_ignore_ascii_case(name))
                        .ok_or_else(|| format!("Unknown TF output voltage node '{}'", name))?;
                    result
                        .voltages
                        .get(idx)
                        .copied()
                        .unwrap_or_else(|| Complex64::new(0.0, 0.0))
                }
            };
            return Ok((pos_value - neg_value).re);
        }

        if let Some(element) = Self::parse_current_probe(output) {
            let idx = result
                .branch_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case(&element))
                .ok_or_else(|| format!("Unknown TF output current '{}'", output))?;
            return Ok(result
                .currents
                .get(idx)
                .copied()
                .unwrap_or_else(|| Complex64::new(0.0, 0.0))
                .re);
        }

        Err(format!("Unsupported .TF output probe '{}'", output))
    }

    pub(super) fn transfer_output_value_linearized(
        &self,
        netlist: &Netlist,
        output: &str,
        input_source: &str,
    ) -> Result<Option<Value>, String> {
        let Some((output_pos, output_neg)) = Self::parse_voltage_probe(output) else {
            // Keep the AC fallback for non-voltage .TF probes until the branch-current
            // path is upgraded to use the same exact linearized adjoint formulation.
            return Ok(None);
        };

        let resolver_engine = self.create_dynamic_engine();
        let circuit = resolver_engine
            .build_circuit(netlist)
            .map_err(|err| format!("Circuit build error: {err}"))?;
        let output_pos_idx =
            Self::resolve_circuit_node_index(&circuit, &output_pos, "transfer output+")?;
        let output_neg_idx = Self::resolve_optional_circuit_node_index(
            &circuit,
            output_neg.as_deref(),
            "transfer output-",
        )?;

        let engine = self.create_dc_engine();
        let sensitivity = engine
            .run_sensitivity_linearized(netlist, output_pos_idx, output_neg_idx)
            .map_err(|err| format!("Linearized transfer analysis error: {err}"))?;

        let gain = sensitivity
            .sensitivities
            .iter()
            .find(|entry| entry.element.eq_ignore_ascii_case(input_source))
            .map(|entry| entry.absolute)
            .ok_or_else(|| {
                format!(
                    "Linearized transfer analysis found no independent source sensitivity for '{}'",
                    input_source
                )
            })?;

        Ok(Some(gain))
    }

    pub(super) fn get_source_dc_value(
        &self,
        netlist: &Netlist,
        source_name: &str,
    ) -> Result<Value, String> {
        for element in &netlist.elements {
            if !element.name.eq_ignore_ascii_case(source_name) {
                continue;
            }
            match &element.kind {
                crate::netlist::ElementKind::VoltageSource(spec)
                | crate::netlist::ElementKind::CurrentSource(spec) => {
                    return Ok(match spec {
                        crate::netlist::SourceSpec::Dc(v) => *v,
                        crate::netlist::SourceSpec::DcAc { dc_value, .. } => *dc_value,
                        crate::netlist::SourceSpec::DcTransient { dc_value, .. } => *dc_value,
                        crate::netlist::SourceSpec::DcAcTransient { dc_value, .. } => *dc_value,
                        crate::netlist::SourceSpec::Ac { .. }
                        | crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Exp { .. } => 0.0,
                    });
                }
                _ => {
                    return Err(format!(
                        "Transfer source '{}' is not an independent source",
                        source_name
                    ));
                }
            }
        }
        Err(format!("Transfer source '{}' not found", source_name))
    }

    pub(super) fn run_transfer_function_test(
        &self,
        name: &str,
        cir_path: &Path,
        source: &str,
        output: &str,
        input_source: &str,
        start: std::time::Instant,
    ) -> TestResult {
        let base_netlist = match Netlist::parse(source) {
            Ok(n) => n,
            Err(e) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Parse error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("TF".to_string()),
                };
            }
        };

        if let Err(err) = self.get_source_dc_value(&base_netlist, input_source) {
            return TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(err),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("TF".to_string()),
            };
        }

        let gain = match self.transfer_output_value_linearized(&base_netlist, output, input_source)
        {
            Ok(Some(gain)) => gain,
            Ok(None) => {
                let mut ac_netlist = base_netlist.clone();
                self.clear_all_source_ac_values(&mut ac_netlist);
                if let Err(err) = self.set_source_ac_value(&mut ac_netlist, input_source, 1.0, 0.0)
                {
                    return TestResult {
                        name: name.to_string(),
                        passed: false,
                        error: Some(err),
                        mismatches: Vec::new(),
                        duration_ms: start.elapsed().as_millis(),
                        analysis_type: Some("TF".to_string()),
                    };
                }

                let engine = self.create_dc_engine();
                let ac_result = match engine.run_ac(&ac_netlist, &[0.0]) {
                    Ok(mut results) => match results.pop() {
                        Some(result) => result,
                        None => {
                            return TestResult {
                                name: name.to_string(),
                                passed: false,
                                error: Some(
                                    "Transfer-function analysis produced no AC sample".to_string(),
                                ),
                                mismatches: Vec::new(),
                                duration_ms: start.elapsed().as_millis(),
                                analysis_type: Some("TF".to_string()),
                            };
                        }
                    },
                    Err(err) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(format!("Simulation error: {}", err)),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("TF".to_string()),
                        };
                    }
                };

                match self.transfer_output_value_ac(&ac_result, output) {
                    Ok(value) => value,
                    Err(err) => {
                        return TestResult {
                            name: name.to_string(),
                            passed: false,
                            error: Some(err),
                            mismatches: Vec::new(),
                            duration_ms: start.elapsed().as_millis(),
                            analysis_type: Some("TF".to_string()),
                        };
                    }
                }
            }
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("TF".to_string()),
                };
            }
        };
        if !gain.is_finite() {
            return TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!(
                    "Transfer-function result for '{}' driven by '{}' is not finite",
                    output, input_source
                )),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("TF".to_string()),
            };
        }

        let mut mismatches = Vec::new();
        match self.load_transfer_function_reference(cir_path, output, input_source) {
            Ok(Some(reference)) => {
                if let Some(expected) = reference.transfer_function
                    && let Some(relative_error) = self.compare_values(expected, gain)
                {
                    mismatches.push(ValueMismatch {
                        x_value: 0.0,
                        node: format!("tf({}, {})", output, input_source),
                        expected,
                        actual: gain,
                        relative_error,
                    });
                }
            }
            Ok(None) => {}
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(err),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("TF".to_string()),
                };
            }
        }

        TestResult {
            name: name.to_string(),
            passed: mismatches.is_empty(),
            error: if mismatches.is_empty() {
                None
            } else {
                Some(format!(
                    "{} transfer-function mismatch(es)",
                    mismatches.len()
                ))
            },
            mismatches,
            duration_ms: start.elapsed().as_millis(),
            analysis_type: Some("TF".to_string()),
        }
    }

    pub(super) fn generate_decade_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_decade: usize,
    ) -> Vec<Value> {
        let mut freqs = Vec::new();
        let decades = (fstop / fstart).log10();
        let total_points = (decades * points_per_decade as f64).ceil() as usize;

        for i in 0..=total_points {
            let f = fstart * 10f64.powf(i as f64 / points_per_decade as f64);
            if f <= fstop {
                freqs.push(f);
            }
        }
        freqs
    }

    pub(super) fn generate_octave_points(
        &self,
        fstart: Value,
        fstop: Value,
        points_per_octave: usize,
    ) -> Vec<Value> {
        let mut freqs = Vec::new();
        let octaves = (fstop / fstart).log2();
        let total_points = (octaves * points_per_octave as f64).ceil() as usize;

        for i in 0..=total_points {
            let f = fstart * 2f64.powf(i as f64 / points_per_octave as f64);
            if f <= fstop {
                freqs.push(f);
            }
        }
        freqs
    }

    pub(super) fn generate_linear_points(
        &self,
        fstart: Value,
        fstop: Value,
        num_points: usize,
    ) -> Vec<Value> {
        let step = (fstop - fstart) / (num_points - 1).max(1) as f64;
        (0..num_points).map(|i| fstart + i as f64 * step).collect()
    }

    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
    // Feature Detection
    // â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•â•
}
