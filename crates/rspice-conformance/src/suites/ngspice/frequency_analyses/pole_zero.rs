use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn run_pz_test(
        &self,
        name: &str,
        cir_path: &Path,
        source_path: &Path,
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
        let netlist = match Self::parse_regression_source(source, source_path) {
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
}
