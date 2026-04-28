//! Single-deck execution orchestration for the ngspice regression harness.

use super::*;

impl TestRunner {
    /// Run a single test circuit
    pub fn run_test(&self, cir_path: &Path) -> TestResult {
        let name = cir_path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let start = std::time::Instant::now();

        // Read source file
        let source = match fs::read_to_string(cir_path) {
            Ok(s) => s,
            Err(e) => {
                return TestResult {
                    name,
                    passed: false,
                    error: Some(format!("Failed to read file: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
        };

        // Preprocess includes using file path for relative path resolution
        let preprocessed_source = match Netlist::preprocess_includes(&source, cir_path) {
            Ok(preprocessed) => preprocessed,
            Err(_) => source, // Keep original if preprocessing fails
        };

        let contract = self.validation_contract_for(cir_path);
        let analyses = if matches!(contract, Some(ValidationContract::ScriptedControl)) {
            self.parse_analyses_with_control(&preprocessed_source, false)
        } else {
            self.parse_analyses(&preprocessed_source)
        };

        // Strip .control/.endc blocks (ngspice scripting) before parsing
        let source = Netlist::strip_control_blocks(&preprocessed_source);

        // Check for unsupported features
        if let Some(reason) = self.check_unsupported(&source) {
            if self.config.skip_unsupported {
                return TestResult {
                    name,
                    passed: true, // Mark as passed (skipped)
                    error: Some(format!("SKIPPED: {}", reason)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
            return TestResult {
                name,
                passed: false,
                error: Some(format!("Unsupported test deck: {}", reason)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: None,
            };
        }

        // Run all analyses in deck order. Control-block analysis commands are
        // parsed from the preprocessed source even though the script body is
        // stripped before circuit parsing.
        let analysis_plan = if analyses.is_empty()
            && !matches!(contract, Some(ValidationContract::ScriptedControl))
        {
            vec![AnalysisSpec::DcOp]
        } else {
            analyses
        };

        if analysis_plan.is_empty() {
            return self.run_parse_build_smoke_test(&name, &source, start);
        }

        let mut all_mismatches = Vec::new();
        let mut analysis_labels = Vec::new();
        let mut first_error: Option<String> = None;

        for analysis in &analysis_plan {
            let analysis_start = std::time::Instant::now();
            let mut result = match analysis {
                AnalysisSpec::DcOp => self.run_dc_op_test(&name, cir_path, &source, analysis_start),
                AnalysisSpec::DcSweep {
                    source: src,
                    start: st,
                    stop: sp,
                    step: stp,
                } => self.run_dc_sweep_test(
                    &name,
                    cir_path,
                    &source,
                    src,
                    *st,
                    *sp,
                    *stp,
                    analysis_start,
                ),
                AnalysisSpec::DcSweep2 {
                    inner_source,
                    inner_start,
                    inner_stop,
                    inner_step,
                    outer_source,
                    outer_start,
                    outer_stop,
                    outer_step,
                } => self.run_dc_sweep_2d_test(
                    &name,
                    cir_path,
                    &source,
                    inner_source,
                    *inner_start,
                    *inner_stop,
                    *inner_step,
                    outer_source,
                    *outer_start,
                    *outer_stop,
                    *outer_step,
                    analysis_start,
                ),
                AnalysisSpec::Transient {
                    tstep,
                    tstop,
                    tstart,
                    tmax,
                } => self.run_transient_test(
                    &name,
                    cir_path,
                    &source,
                    *tstep,
                    *tstop,
                    *tstart,
                    *tmax,
                    analysis_start,
                ),
                AnalysisSpec::Ac {
                    sweep_type,
                    points,
                    fstart,
                    fstop,
                } => self.run_ac_test(
                    &name,
                    cir_path,
                    &source,
                    *sweep_type,
                    *points,
                    *fstart,
                    *fstop,
                    analysis_start,
                ),
                AnalysisSpec::PoleZero {
                    input_pos,
                    input_neg,
                    output_pos,
                    output_neg,
                    input_is_current,
                    compute_poles,
                    compute_zeros,
                } => self.run_pz_test(
                    &name,
                    cir_path,
                    &source,
                    input_pos,
                    input_neg.as_deref(),
                    output_pos,
                    output_neg.as_deref(),
                    *input_is_current,
                    *compute_poles,
                    *compute_zeros,
                    analysis_start,
                ),
                AnalysisSpec::Noise {
                    output_pos,
                    output_neg,
                    input_source,
                    sweep_type,
                    points,
                    fstart,
                    fstop,
                } => self.run_noise_test(
                    &name,
                    cir_path,
                    &source,
                    output_pos,
                    output_neg.as_deref(),
                    input_source,
                    *sweep_type,
                    *points,
                    *fstart,
                    *fstop,
                    analysis_start,
                ),
                AnalysisSpec::Sensitivity {
                    output_pos,
                    output_neg,
                    sweep,
                } => self.run_sensitivity_test(
                    &name,
                    &source,
                    output_pos,
                    output_neg.as_deref(),
                    *sweep,
                    analysis_start,
                ),
                AnalysisSpec::TransferFunction {
                    output,
                    input_source,
                } => self.run_transfer_function_test(
                    &name,
                    cir_path,
                    &source,
                    output,
                    input_source,
                    analysis_start,
                ),
                AnalysisSpec::Unsupported { directive } => TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!(
                        "Regression runner does not implement requested analysis '{}'",
                        directive
                    )),
                    mismatches: Vec::new(),
                    duration_ms: analysis_start.elapsed().as_millis(),
                    analysis_type: Some(directive.clone()),
                },
            };

            if let Some(label) = result.analysis_type.clone() {
                analysis_labels.push(label);
            }

            if result.passed
                && let Err(err) =
                    self.enforce_validation_coverage(cir_path, &preprocessed_source, analysis)
            {
                result.passed = false;
                result.error = Some(err);
            }

            all_mismatches.extend(result.mismatches);

            if !result.passed {
                first_error = result.error.clone();
                break;
            }
        }

        let mut final_result = TestResult {
            name,
            passed: first_error.is_none(),
            error: first_error,
            mismatches: all_mismatches,
            duration_ms: start.elapsed().as_millis(),
            analysis_type: if analysis_labels.is_empty() {
                None
            } else {
                Some(analysis_labels.join(" + "))
            },
        };

        if final_result.duration_ms > self.config.max_time_per_test_ms {
            final_result.passed = false;
            let timeout_msg = format!(
                "Test exceeded timeout ({}ms > {}ms)",
                final_result.duration_ms, self.config.max_time_per_test_ms
            );
            final_result.error = Some(match final_result.error {
                Some(err) => format!("{err}; {timeout_msg}"),
                None => timeout_msg,
            });
        }

        final_result
    }

    pub(super) fn run_parse_build_smoke_test(
        &self,
        name: &str,
        source: &str,
        start: Instant,
    ) -> TestResult {
        let netlist = match Netlist::parse(source) {
            Ok(netlist) => netlist,
            Err(err) => {
                return TestResult {
                    name: name.to_string(),
                    passed: false,
                    error: Some(format!("Netlist parse error: {}", err)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("SMOKE".to_string()),
                };
            }
        };

        let engine = self.create_dynamic_engine();
        match engine.build_circuit(&netlist) {
            Ok(_) => TestResult {
                name: name.to_string(),
                passed: true,
                error: None,
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("SMOKE".to_string()),
            },
            Err(err) => TestResult {
                name: name.to_string(),
                passed: false,
                error: Some(format!("Circuit build error: {}", err)),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("SMOKE".to_string()),
            },
        }
    }
}
