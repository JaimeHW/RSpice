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

        let source_cir_path = match self.authoritative_circuit_path(cir_path) {
            Ok(path) => path,
            Err(e) => {
                return TestResult {
                    name,
                    passed: false,
                    error: Some(format!("Live ngspice reference setup error: {}", e)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
        };

        // Read source file
        let source = match fs::read_to_string(&source_cir_path) {
            Ok(s) => s,
            Err(e) => {
                return TestResult {
                    name,
                    passed: false,
                    error: Some(format!(
                        "Failed to read circuit '{}': {}",
                        source_cir_path.display(),
                        e
                    )),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
        };

        // Preprocess includes using file path for relative path resolution
        let preprocessed_source = match Netlist::preprocess_includes(&source, &source_cir_path) {
            Ok(preprocessed) => preprocessed,
            Err(_) => source, // Keep original if preprocessing fails
        };

        let contract = self.validation_contract_for(cir_path);
        let analyses = if matches!(contract, Some(ValidationContract::ScriptedControl)) {
            self.parse_analyses_with_control(&preprocessed_source, false)
        } else {
            self.parse_analyses(&preprocessed_source)
        };

        // Strip .control/.endc blocks (ngspice scripting) before parsing.
        let source = match Netlist::strip_control_blocks(&preprocessed_source) {
            Ok(source) => source,
            Err(err) => {
                return TestResult {
                    name,
                    passed: false,
                    error: Some(format!("Invalid control block syntax: {err}")),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: None,
                };
            }
        };

        // Check for unsupported features
        if let Some(reason) = self.check_unsupported(&source) {
            if contract.is_some_and(ValidationContract::expects_unsupported_in_this_build) {
                return TestResult {
                    name,
                    passed: true,
                    error: Some(expected_unsupported_message(&reason)),
                    mismatches: Vec::new(),
                    duration_ms: start.elapsed().as_millis(),
                    analysis_type: Some("EXPECTED_UNSUPPORTED".to_string()),
                };
            }
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
        if contract.is_some_and(ValidationContract::expects_unsupported_in_this_build) {
            let contract_name = match contract {
                Some(ValidationContract::ExpectedUnsupportedWithoutGeneratedBuiltins) => {
                    "expected_unsupported_without_veriloga_builtins"
                }
                _ => "expected_unsupported",
            };
            return TestResult {
                name,
                passed: false,
                error: Some(format!(
                    "deck has {contract_name} contract but no unsupported feature was detected -- re-adjudicate the contract"
                )),
                mismatches: Vec::new(),
                duration_ms: start.elapsed().as_millis(),
                analysis_type: Some("EXPECTED_UNSUPPORTED".to_string()),
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
            return self.run_parse_build_smoke_test(&name, &source_cir_path, &source, start);
        }

        let mut all_mismatches = Vec::new();
        let mut analysis_labels = Vec::new();
        let mut first_error: Option<String> = None;

        for analysis in &analysis_plan {
            let analysis_start = std::time::Instant::now();
            let mut result = match analysis {
                AnalysisSpec::DcOp => {
                    self.run_dc_op_test(&name, cir_path, &source_cir_path, &source, analysis_start)
                }
                AnalysisSpec::DcSweep {
                    source: src,
                    start: st,
                    stop: sp,
                    step: stp,
                } => self.run_dc_sweep_test(
                    &name,
                    cir_path,
                    &source_cir_path,
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
                    &source_cir_path,
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
                    &source_cir_path,
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
                    &source_cir_path,
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
                    &source_cir_path,
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
                    &source_cir_path,
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
                    &source_cir_path,
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
                    &source_cir_path,
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

        // Some contracts admit a clean refusal diagnostic as a successful
        // adjudicated outcome. expected_unsolvable is stricter than
        // reference_unsolvable: converging every analysis demands
        // re-adjudication only for expected_unsolvable.
        if contract.is_some_and(ValidationContract::accepts_clean_refusal) {
            match &final_result.error {
                Some(message) if is_clean_refusal_diagnostic(message) => {
                    log::info!(
                        "'{}' failed with an accepted clean refusal diagnostic: {message}",
                        final_result.name
                    );
                    final_result.passed = true;
                    final_result.error = None;
                    final_result.mismatches.clear();
                }
                Some(_) => {
                    // A non-convergence failure (parse error, panic,
                    // timeout) is a genuine defect even on an unsolvable
                    // deck; keep it failing as reported.
                }
                None if matches!(contract, Some(ValidationContract::ExpectedUnsolvable)) => {
                    final_result.passed = false;
                    final_result.error = Some(
                        "deck is adjudicated unsolvable (expected_unsolvable contract) but every analysis converged -- re-adjudicate the contract"
                            .to_string(),
                    );
                }
                None => {}
            }
        }

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
        source_path: &Path,
        source: &str,
        start: Instant,
    ) -> TestResult {
        let netlist = match Self::parse_regression_source(source, source_path) {
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

/// Whether a deck-level failure message is a clean refusal diagnostic for
/// the `expected_unsolvable` contract.
///
/// A certified-unsolvable deck refuses through whichever discipline gives up
/// first, and the phrasing legitimately moves as solver disciplines evolve.
/// vbic/diffamp demonstrated both classes within one day: with the
/// 2026-06-12 solver line it converges its (certified right-half-plane,
/// hence unstable) operating point — out-solving ngspice-46, which hits a
/// singular matrix there — and then refuses the transient, first as
/// "transient timestep pinned at the minimum near t=1.27e-9s … integration
/// restart did not escape", later as "Convergence failed after 319
/// iterations". Both are the same adjudicated pathology; gating on one
/// phrasing made the contract flip on solver-discipline changes that were
/// each individually sound.
///
/// The accepted classes stay an explicit, closed list: Newton refusal and
/// timestep-collapse refusal. Timeouts, panics, parse and build errors stay
/// genuine defects even on an unsolvable deck.
fn is_clean_refusal_diagnostic(message: &str) -> bool {
    message.contains("Convergence failed") || message.contains("timestep pinned at the minimum")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refusal_classes_cover_both_observed_diffamp_diagnostics() {
        // The two phrasings diffamp actually produced on 2026-06-12,
        // before and after the merge of the globalized-Newton line.
        assert!(is_clean_refusal_diagnostic(
            "Simulation error: Convergence failed after 319 iterations"
        ));
        assert!(is_clean_refusal_diagnostic(
            "Simulation error: Circuit error: transient timestep pinned at the \
             minimum near t=1.270828e-9s (dt=7.451e-19s, delmin=1.000e-19s): \
             integration restart did not escape; the circuit is numerically \
             ill-conditioned at this operating point"
        ));
    }

    #[test]
    fn dirty_failures_are_not_clean_refusals() {
        assert!(!is_clean_refusal_diagnostic(
            "Test exceeded hard process timeout (30000ms)"
        ));
        assert!(!is_clean_refusal_diagnostic(
            "Simulation error: Simulation aborted by user"
        ));
        assert!(!is_clean_refusal_diagnostic(
            "Parse error: unknown card .foo"
        ));
        assert!(!is_clean_refusal_diagnostic(
            "Circuit build error: node q1_e has no DC path to ground"
        ));
    }

    #[test]
    fn expected_unsupported_contract_accepts_named_unsupported_feature() {
        let (root, deck_path) = expected_unsupported_fixture(
            "xspice/digital/d_source.cir",
            "digital source deck
a_source [a1] d_source1
.model d_source1 d_source (input_file=\"stimulus.txt\")
.end
",
        );

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&deck_path);

        assert!(
            is_expected_unsupported_result(&result),
            "expected explicit unsupported result, got {result:?}"
        );
        assert_eq!(
            result.analysis_type.as_deref(),
            Some("EXPECTED_UNSUPPORTED")
        );

        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    #[test]
    fn expected_unsupported_contract_fails_when_detector_no_longer_matches() {
        let (root, deck_path) = expected_unsupported_fixture(
            "general/plain_resistor.cir",
            "plain resistor deck
r1 in 0 1k
.end
",
        );

        let runner = TestRunner::new(&root, TestRunnerConfig::default());
        let result = runner.run_test(&deck_path);

        assert!(!result.passed, "stale contract must fail: {result:?}");
        assert!(
            result
                .error
                .as_deref()
                .is_some_and(|error| error.contains("expected_unsupported contract")),
            "unexpected stale-contract error: {result:?}"
        );

        fs::remove_dir_all(root).expect("remove temporary test directory");
    }

    fn expected_unsupported_fixture(relative_deck: &str, source: &str) -> (PathBuf, PathBuf) {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after the Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("rspice_expected_unsupported_{unique}"));
        let deck_path = root.join(relative_deck);
        let deck_dir = deck_path.parent().expect("deck has a parent");

        fs::create_dir_all(deck_dir).expect("create temporary test directory");
        fs::write(
            root.join("validation-manifest.tsv"),
            format!("{relative_deck}\texpected_unsupported\n"),
        )
        .expect("write validation manifest");
        fs::write(&deck_path, source).expect("write circuit");

        (root, deck_path)
    }

    #[test]
    fn regression_source_parsing_preserves_deck_relative_model_string_paths() {
        let deck_path = std::env::temp_dir()
            .join("rspice-ngspice-harness-source-path")
            .join("models")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("deck has a parent");
        let source = "\
deck
.model d_source1 d_source (input_file=\"stimulus.txt\")
.model d_state1 d_state (state_file=\"state.tbl\")
.end
";

        let netlist = TestRunner::parse_regression_source(source, &deck_path)
            .expect("regression harness source parses");

        let d_source = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("d_source1"))
            .expect("d_source model parsed");
        let input_file = d_source
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("input_file"))
            .map(|(_, value)| value.as_str())
            .expect("input_file string param");
        assert_eq!(
            input_file,
            deck_dir.join("stimulus.txt").to_string_lossy().as_ref()
        );

        let d_state = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("d_state1"))
            .expect("d_state model parsed");
        let state_file = d_state
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("state_file"))
            .map(|(_, value)| value.as_str())
            .expect("state_file string param");
        assert_eq!(
            state_file,
            deck_dir.join("state.tbl").to_string_lossy().as_ref()
        );
        assert_eq!(netlist.source_path.as_deref(), Some(deck_path.as_path()));
    }
}
