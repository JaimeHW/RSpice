use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn transfer_output_value_ac(
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

    pub(in crate::testing::ngspice_runner) fn transfer_output_value_linearized(
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

        // Linearized transfer uses the converged DC operating point as the
        // small-signal expansion point. Use the primary regression engine here
        // to match .SENS and ngspice's direct operating-point path; the robust
        // fallback engine is intentionally reserved for analyses that need a
        // recovery pass after the primary solve fails.
        let engine = self.create_dynamic_engine();
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

    pub(in crate::testing::ngspice_runner) fn get_source_dc_value(
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

    pub(in crate::testing::ngspice_runner) fn run_transfer_function_test(
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
}
