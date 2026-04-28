use super::*;

impl TestRunner {
    pub(in crate::testing::ngspice_runner) fn run_noise_test(
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
}
