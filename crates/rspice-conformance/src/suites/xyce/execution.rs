//! Driving a single deck and a whole suite through the engine.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    /// Run every discovered vendored `.cir` through the Xyce corpus contract.
    pub fn run_all(&self) -> Vec<XyceTestResult> {
        self.discover_tests()
            .into_iter()
            .map(|deck| self.run_discovered_test(&deck))
            .collect()
    }

    /// Run every simulator regression deck under `Netlists`.
    pub fn run_netlist_suite(&self) -> Vec<XyceTestResult> {
        self.discover_netlist_tests()
            .into_iter()
            .map(|deck| self.run_discovered_test(&deck))
            .collect()
    }

    /// Run one deck path.
    pub fn run_test<P: AsRef<Path>>(&self, deck_path: P) -> XyceTestResult {
        let path = deck_path
            .as_ref()
            .canonicalize()
            .unwrap_or_else(|_| deck_path.as_ref().to_path_buf());
        let relative_path = self.relative_key(&path);
        let deck = XyceDeck {
            path,
            section: Self::section_for_relative_path(&relative_path),
            relative_path,
        };
        self.run_discovered_test(&deck)
    }

    pub(super) fn run_discovered_test(&self, deck: &XyceDeck) -> XyceTestResult {
        let start = Instant::now();
        if deck.section != XyceDeckSection::Netlists {
            return self.expected_unsupported_result(
                deck,
                start,
                "upstream_harness_fixture",
                "deck is part of upstream Xyce harness self-tests, not the simulator Netlists corpus",
            );
        }

        if let Some(kind) = XyceStartupOracleKind::for_record(&deck.relative_path) {
            let contract = kind.result_contract();
            let result = match self.validate_startup_diagnostic_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(kind) = XyceXdmReplaceGroundKind::for_record(&deck.relative_path) {
            let contract = "xdm_hspice_replaceground_dc_relational_wrapper";
            let result = match self.validate_xdm_replaceground_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(member) = XyceMeasureStepFindWhenMember::for_record(&deck.relative_path) {
            let contract = if member.is_owner() {
                "measure_step_find_when_relational_wrapper"
            } else {
                "measure_step_find_when_relational_control"
            };
            let result = match self.validate_measure_step_find_when_oracle(deck, member, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(member) = XyceMeasureContStepTranMember::for_record(&deck.relative_path) {
            let contract = match member.role {
                XyceMeasureContStepTranRole::Main => "measure_cont_step_tran_relational_wrapper",
                XyceMeasureContStepTranRole::Control0 | XyceMeasureContStepTranRole::Control1 => {
                    "measure_cont_step_tran_relational_control"
                }
            };
            let result = match self.validate_measure_cont_step_tran_oracle(deck, member, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(kind) = XyceMeasureContTranKind::for_record(&deck.relative_path) {
            let contract = "measure_cont_tran_removed_wrapper";
            let result = match self.validate_measure_cont_tran_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(kind) = XyceAbmTransientKind::for_record(&deck.relative_path) {
            let contract = "abm_generated_gold_transient_wrapper";
            let result = match self.validate_abm_transient_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(kind) = XyceAbmPowKind::for_record(&deck.relative_path) {
            let contract = "abm_pow_generated_gold_dc_wrapper";
            let result = match self.validate_abm_pow_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(kind) = XyceAddResistorsKind::for_record(&deck.relative_path) {
            let contract = "addresistors_generated_netlist_relational_wrapper";
            let result = match self.validate_addresistors_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(kind) = XyceRemoveUnusedKind::for_record(&deck.relative_path) {
            let contract = "removeunused_dynamic_gold_dc_wrapper";
            let result = match self.validate_removeunused_oracle(deck, kind, start) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(result) = self.run_expected_error_contract(deck, start) {
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.bug702_positive_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_bug702_positive_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    XyceBug702PositiveKind::for_record(&deck.relative_path)
                        .expect("recognized BUG702 positive record")
                        .result_contract(),
                    format!("BUG702 positive qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.passive_primary_value_composite_contract(deck) {
            let result = self.run_passive_primary_value_composite_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.analytic_rc_wrapper_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_analytic_rc_wrapper_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "analytic_first_order_rc_tran_wrapper",
                    format!("analytic first-order RC qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.analytic_sinusoidal_rc_wrapper_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_analytic_sinusoidal_rc_wrapper_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "analytic_sinusoidal_first_order_rc_tran_wrapper",
                    format!("analytic sinusoidal first-order RC qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.analytic_integer_dc_wrapper_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_analytic_integer_dc_wrapper_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "analytic_integer_dc_wrapper",
                    format!("analytic integer DC qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.resistor_dtemp_relational_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_resistor_dtemp_relational_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "resistor_dtemp_relational_wrapper",
                    format!("resistor DTEMP relational qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.bug647_resistor_relational_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_bug647_resistor_relational_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "bug647_resistor_relational_wrapper",
                    format!("BUG 647 resistor relational qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.bug655_continuation_relational_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_bug655_continuation_relational_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "bug655_continuation_relational_wrapper",
                    format!("BUG 655 continuation-line qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.bug662_long_header_relational_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_bug662_long_header_relational_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "bug662_long_header_relational_wrapper",
                    format!("BUG 662 long-header relational qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.bug667_nodeset_relational_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_bug667_nodeset_relational_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "bug667_nodeset_relational_wrapper",
                    format!("BUG 667 NODESET relational qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.bug754_global_parameter_relational_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_bug754_global_parameter_relational_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "bug754_global_parameter_dc_relational_wrapper",
                    format!("BUG 754 global-parameter DC qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.stepped_ic_reference_contract(deck) {
            let result = self.run_stepped_ic_reference_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.shared_stepped_dc_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_shared_stepped_dc_family_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "shared_stepped_dc_oracle_family",
                    format!("shared stepped-DC family qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.numbered_redefinition_dc_family_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_numbered_redefinition_dc_family_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "numbered_redefinition_dc_family",
                    format!("numbered redefinition DC family qualification failed: {reason}"),
                    Vec::new(),
                ),
            };
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.age_cap_family_contract(deck) {
            let result = self.run_age_cap_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.diode_model_alias_family_contract(deck) {
            let result = self.run_diode_model_alias_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.nested_include_identity_family_contract(deck) {
            let result = self.run_nested_include_identity_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.switch_state_case_family_contract(deck) {
            let result = self.run_switch_state_case_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.subckt_parameter_resolution_family_contract(deck) {
            let result =
                self.run_subckt_parameter_resolution_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        if let Some(contract) = self.baseline_family_contract(deck) {
            let result = self.run_baseline_family_contract(deck, contract, start);
            if self.config.verbose {
                println!(
                    "{} [{}] {}",
                    result.relative_path,
                    result.contract,
                    if result.passed { "PASS" } else { "FAIL" }
                );
            }
            return result;
        }

        let result = match self.static_dc_sensitivity_plan_for_deck(deck) {
            Ok(plan) => self.run_static_prn_dc_sensitivity_plan(deck, plan, start),
            Err(sensitivity_reason) => match self.static_hb_plan_for_deck(deck) {
                Ok(plan) => self.run_static_prn_hb_plan(deck, plan, start),
                Err(hb_reason) => match self.execution_plan(deck) {
                    Ok(plan) => self.run_static_prn_dc_plan(deck, plan, start),
                    Err(dc_reason) => match self.static_noise_plan_for_deck(deck) {
                        Ok(plan) => self.run_static_noise_plan(deck, plan, start),
                        Err(noise_reason)
                            if Self::normalize_manifest_key(&deck.relative_path)
                                == XYCE_MEASURE_CONT_STEP_NOISE_DERIV_RECORD =>
                        {
                            self.failure_result(
                                deck,
                                start,
                                "wrapper_scalar_measure_step_noise",
                                noise_reason,
                                Vec::new(),
                            )
                        }
                        Err(noise_reason) => match self.static_ac_plan_for_deck(deck) {
                            Ok(plan) => self.run_static_fd_prn_ac_plan(deck, plan, start),
                            Err(ac_reason) => match self.static_tran_plan_for_deck(deck) {
                                Ok(plan) => self.run_static_prn_tran_plan(deck, plan, start),
                                Err(tran_reason) => {
                                    let reason = if self.deck_has_print_analysis(deck, "SENS") {
                                        sensitivity_reason
                                    } else if self.deck_has_print_analysis(deck, "HB") {
                                        hb_reason
                                    } else if self.deck_has_print_analysis(deck, "NOISE") {
                                        noise_reason
                                    } else if self.deck_has_print_analysis(deck, "AC") {
                                        ac_reason
                                    } else if self.deck_has_print_analysis(deck, "TRAN") {
                                        tran_reason
                                    } else {
                                        dc_reason
                                    };
                                    return self.expected_unsupported_result(
                                        deck,
                                        start,
                                        "unsupported_xyce_contract",
                                        &reason,
                                    );
                                }
                            },
                        },
                    },
                },
            },
        };
        if self.config.verbose {
            println!(
                "{} [{}] {}",
                result.relative_path,
                result.contract,
                if result.passed { "PASS" } else { "FAIL" }
            );
        }
        result
    }

    pub(super) fn run_addresistors_dc_netlist(
        &self,
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        start: Instant,
    ) -> Result<Vec<DcSweepPointResult>, String> {
        if !plan.steps.is_empty() || plan.dc_data.is_some() {
            return Err("ADDRESISTORS bridge does not admit STEP/DATA state".to_string());
        }
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        engine
            .run_dc_sweep2_spec_with_report_and_abort(
                netlist,
                &plan.dc.source,
                &plan.dc.primary_spec(),
                plan.dc.sweep2.as_ref(),
                &abort,
            )
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "ADDRESISTORS bridge execution exceeded shared timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                other => format!("ADDRESISTORS bridge execution failed: {other}"),
            })
    }

    pub(super) fn run_expected_error_contract(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Option<XyceTestResult> {
        if let Some(kind) = XyceExpectedFailureKind::for_record(&deck.relative_path) {
            let contract = kind.result_contract();
            return Some(match self.validate_expected_failure_oracle(deck, kind) {
                Ok(()) => self.passed_result(deck, start, contract),
                Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
            });
        }

        let reference_path = self.static_prn_reference_path(&deck.path)?;
        if reference_path.is_file() {
            return None;
        }

        if let Some(result) = self.run_connectivity_diagnostic_contract(deck, start) {
            return Some(result);
        }

        let source = fs::read_to_string(&deck.path).ok()?;
        match Self::expected_ac_data_analysis_init_failure(&source, &deck.path) {
            Ok(Some(failure)) => {
                return Some(self.passed_result(deck, start, failure.result_contract()));
            }
            Ok(None) => {}
            Err(error) => {
                return Some(self.failure_result(
                    deck,
                    start,
                    "expected_error_ac_data_analysis_init",
                    error,
                    Vec::new(),
                ));
            }
        }

        if Self::is_expected_missing_inductor_value_error_deck(&deck.relative_path, &source) {
            let contract = "expected_error_missing_inductor_value";
            return Self::validate_expected_missing_inductor_value_error_source(
                &source, &deck.path,
            )
            .ok()
            .map(|()| self.passed_result(deck, start, contract));
        }

        if !Self::source_may_have_pwl_repeat_option(&source) {
            return None;
        }

        let contract = "expected_error_pwl_repeat_value";
        Self::validate_expected_pwl_repeat_value_error_source(&source, &deck.path)
            .ok()
            .map(|()| self.passed_result(deck, start, contract))
    }

    pub(super) fn run_bug702_positive_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug702PositiveContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.kind.result_contract();
        let reference =
            match Self::parse_xyce_verify_tran_reference_file(&contract.plan.reference_path) {
                Ok(reference) => reference,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("BUG702 alias reference parse failed: {error}"),
                        Vec::new(),
                    );
                }
            };
        let result = match self.run_transient_family_netlist(
            &contract.plan,
            &contract.netlist,
            start,
            None,
            None,
        ) {
            Ok(result) => result,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG702 positive simulation failed: {error}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_static_tran_primary_reference(
            &reference,
            &contract.plan,
            &contract.netlist,
            &result,
        ) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG702 alias reference comparison failed: {error}"),
                    Vec::new(),
                );
            }
        };
        if !mismatches.is_empty() {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG702 alias transient mismatch(es)", mismatches.len()),
                mismatches,
            );
        }
        self.passed_result(deck, start, result_contract)
    }

    pub(super) fn run_connectivity_diagnostic_contract(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Option<XyceTestResult> {
        const CONTRACT: &str = "wrapper_expected_topology_warnings";
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return None;
        }
        let reference_path = self.static_output_reference_path(&deck.path, "err")?;
        if !reference_path.is_file() {
            return None;
        }
        let expected = match Self::parse_connectivity_diagnostic_reference(&reference_path) {
            Ok(Some(expected)) => expected,
            Ok(None) => return None,
            Err(error) => {
                return Some(self.failure_result(deck, start, CONTRACT, error, Vec::new()));
            }
        };

        let outcome = (|| -> Result<(), String> {
            if expected.one_device_terminal_nodes.len() != 1 || expected.no_dc_path_nodes.len() != 1
            {
                return Err(
                    "topology-warning wrapper currently requires exactly one expected node per warning category"
                        .to_string(),
                );
            }
            let source = fs::read_to_string(&deck.path)
                .map_err(|error| format!("failed to read connectivity deck: {error}"))?;
            for line in Self::logical_netlist_lines(&source) {
                let normalized = Self::strip_netlist_comment(&line)
                    .chars()
                    .filter(|character| !character.is_whitespace())
                    .flat_map(char::to_lowercase)
                    .collect::<String>();
                if normalized.starts_with(".optionstopology")
                    && normalized.contains("check_connectivity")
                {
                    return Err(
                        "topology-warning wrapper does not admit an explicit CHECK_CONNECTIVITY override"
                            .to_string(),
                    );
                }
            }
            let netlist = Self::parse_xyce_netlist(&source, &deck.path)
                .map_err(|error| format!("connectivity deck parse failed: {error}"))?;
            let flattened = rspice_core::netlist::flatten_netlist_with_models(&netlist)
                .map_err(|error| format!("connectivity deck flattening failed: {error}"))?;
            let actual = rspice_core::netlist::analyze_xyce_connectivity(&flattened.elements)
                .map_err(|error| error.to_string())?;
            if actual != expected {
                return Err(format!(
                    "topology diagnostics differ: expected one-terminal {:?} and no-DC-path {:?}, found one-terminal {:?} and no-DC-path {:?}",
                    expected.one_device_terminal_nodes,
                    expected.no_dc_path_nodes,
                    actual.one_device_terminal_nodes,
                    actual.no_dc_path_nodes
                ));
            }
            Ok(())
        })();

        Some(match outcome {
            Ok(()) => self.passed_result(deck, start, CONTRACT),
            Err(error) => self.failure_result(deck, start, CONTRACT, error, Vec::new()),
        })
    }

    pub(super) fn run_static_prn_hb_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticHbPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.output_format.result_contract(plan.wrapper);
        let netlist = match Self::parse_xyce_netlist(&plan.source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after HB contract validation: {err}"),
                    Vec::new(),
                );
            }
        };
        let fd_reference_result = match plan.output_format {
            XyceStaticHbOutputFormat::StdPrn | XyceStaticHbOutputFormat::GnuplotPrn => {
                Self::parse_ac_comparator_prn_file(&plan.fd_reference_path)
            }
        };
        let fd_reference = match fd_reference_result {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("failed to parse HB.FD oracle: {err}"),
                    Vec::new(),
                );
            }
        };
        let td_reference =
            match Self::parse_xyce_verify_tran_reference_file(&plan.td_reference_path) {
                Ok(reference) => reference,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("failed to parse HB.TD oracle: {err}"),
                        Vec::new(),
                    );
                }
            };
        let ic_reference =
            match Self::parse_xyce_verify_tran_reference_file(&plan.ic_reference_path) {
                Ok(reference) => reference,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("failed to parse HB startup oracle: {err}"),
                        Vec::new(),
                    );
                }
            };

        let hb_config = rspice_core::analysis::HbConfig::new(plan.frequency)
            .with_harmonics(plan.num_harmonics)
            .with_collocation_points(2 * plan.num_harmonics + 1);
        let hb = match self.create_xyce_engine().run_hb(&netlist, hb_config) {
            Ok(result) => result,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("native HB simulation failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let fd_actual = match Self::hb_frequency_result_to_prn_table(
            &fd_reference,
            &plan.print,
            &netlist,
            &hb.result,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("HB.FD output construction failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mut mismatches = match self.compare_ac_comparator_tables(&fd_reference, &fd_actual) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("HB.FD ACComparator comparison failed: {err}"),
                    Vec::new(),
                );
            }
        };

        let td_result = Self::hb_result_to_transient_result(&hb.result);
        let td_actual =
            match Self::hb_transient_result_to_prn_table(&plan.print, &netlist, &td_result) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("HB.TD output construction failed: {err}"),
                        Vec::new(),
                    );
                }
            };
        match self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            &td_reference,
            &td_actual,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        ) {
            Ok(td_mismatches) => mismatches.extend(td_mismatches),
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("HB.TD xyce_verify comparison failed: {err}"),
                    Vec::new(),
                );
            }
        }

        let period = plan.frequency.recip();
        let mut startup_config = self.xyce_engine_config(None);
        startup_config.integration_method = rspice_core::analysis::IntegrationMethod::BackwardEuler;
        startup_config.transient_initial_timestep = Some(period / 1.0e6);
        let startup = match Engine::new(startup_config).run_tran(&netlist, period, period / 1000.0)
        {
            Ok(result) => result,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("HB startup transient failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let ic_actual =
            match Self::hb_transient_result_to_prn_table(&plan.print, &netlist, &startup) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("HB startup output construction failed: {err}"),
                        Vec::new(),
                    );
                }
            };
        match self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            &ic_reference,
            &ic_actual,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        ) {
            Ok(ic_mismatches) => mismatches.extend(ic_mismatches),
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("HB startup xyce_verify comparison failed: {err}"),
                    Vec::new(),
                );
            }
        }

        mismatches.truncate(self.config.max_mismatches);
        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} HB oracle mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_fd_prn_ac_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = if plan.reference_path.is_none()
            && (!plan.measurement_reference_paths.is_empty()
                || !plan.continuous_measurement_reference_paths.is_empty())
        {
            "wrapper_scalar_measure_ac"
        } else {
            plan.contract.result_contract(!plan.steps.is_empty())
        };
        let frequencies = plan.ac.frequencies();
        if frequencies.is_empty() {
            return self.failure_result(
                deck,
                start,
                contract,
                "AC analysis produced no frequency points".to_string(),
                Vec::new(),
            );
        }

        let frequency_bound_source;
        let engine_source = if plan.sensitivity.is_some() {
            Self::source_without_xyce_sensitivity_directives(&plan.source)
        } else {
            plan.source.clone()
        };
        let parse_source = if plan.frequency_bound {
            frequency_bound_source =
                Self::source_with_ac_frequency_bindings(&engine_source, frequencies[0]);
            frequency_bound_source.as_str()
        } else {
            engine_source.as_str()
        };
        let netlist = match Self::parse_xyce_netlist(parse_source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        if plan.reference_path.is_none() && plan.print.is_none() && plan.sensitivity.is_some() {
            let Some(sensitivity_plan) = plan.sensitivity.as_ref() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    "AC plan has neither a primary output nor a sensitivity request".to_string(),
                    Vec::new(),
                );
            };
            if !plan.steps.is_empty() {
                return self.run_static_step_ac_sensitivity_plan(
                    deck,
                    plan,
                    netlist,
                    frequencies,
                    start,
                );
            }
            if plan.ac.data_points().is_some() || plan.frequency_bound {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_contract",
                    "measurement-only AC sensitivity comparison currently requires an ordinary, unstepped frequency sweep",
                );
            }
            let engine = self.create_xyce_engine();
            let results = match engine.run_ac(&netlist, &frequencies) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this static AC sensitivity deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            };
            let mismatches = match self.compare_ac_sensitivity_outputs(
                sensitivity_plan,
                &netlist,
                &plan.source,
                &results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC sensitivity reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            return if mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce AC sensitivity reference mismatch(es)",
                        mismatches.len()
                    ),
                    mismatches,
                )
            };
        }

        if plan.reference_path.is_none() && plan.print.is_some() {
            if !plan.steps.is_empty() {
                return self.run_static_step_ac_measurement_plan(
                    deck,
                    plan,
                    netlist,
                    frequencies,
                    start,
                );
            }
            if plan.ac.data_points().is_some() || plan.frequency_bound {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_contract",
                    "measurement-only AC oracle comparison currently requires an ordinary, unstepped frequency sweep",
                );
            }
            let engine = self.create_xyce_engine();
            let results = match engine.run_ac(&netlist, &frequencies) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this static AC deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            };
            let measurements =
                rspice_core::analysis::evaluate_ac_measurements(&netlist, &results);
            let continuous = rspice_core::analysis::evaluate_ac_continuous_measurements(
                &netlist, &results,
            );
            let mismatches = match self.compare_analysis_measurement_outputs(
                &plan.measurement_reference_paths,
                &plan.continuous_measurement_reference_paths,
                &measurements,
                &continuous,
                plan.measurement_tolerance,
                netlist.options.measure_fail_output,
                netlist.options.measure_default_value,
                netlist.options.measure_use_cont_files(),
                &netlist.measurements,
                "AC",
                "AC_CONT",
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC measurement reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            return if mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC measurement mismatch(es)", mismatches.len()),
                    mismatches,
                )
            };
        }

        let Some(primary_reference_path) = plan.reference_path.as_ref() else {
            let ac_ic_mismatches = match self.compare_ac_initial_condition_outputs(&plan, &netlist)
            {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC initial-condition output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            return if ac_ic_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce AC initial-condition output mismatch(es)",
                        ac_ic_mismatches.len()
                    ),
                    ac_ic_mismatches,
                )
            };
        };
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                "AC plan has a primary reference without a primary print request".to_string(),
                Vec::new(),
            );
        };
        let reference = match Self::parse_ac_reference_file(plan.contract, primary_reference_path) {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("failed to parse Xyce AC oracle: {err}"),
                    Vec::new(),
                );
            }
        };

        if !plan.steps.is_empty() {
            return self.run_static_step_ac_plan(
                deck,
                plan,
                netlist,
                reference,
                frequencies,
                start,
            );
        }
        if plan.ac.data_points().is_some() {
            return self.run_static_ac_data_plan(deck, plan, netlist, reference, start);
        }
        if plan.frequency_bound {
            return self.run_static_frequency_bound_ac_plan(
                deck,
                plan,
                reference,
                frequencies,
                start,
            );
        }

        let engine = self.create_xyce_engine();
        let results = match engine.run_ac(&netlist, &frequencies) {
            Ok(results) => results,
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this static AC deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("simulation error: {err}"),
                    Vec::new(),
                );
            }
        };

        let mismatches = match self.compare_ac_prn_reference(
            &reference,
            primary_print,
            &netlist,
            &plan.source,
            &results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            if let Some(sensitivity_plan) = plan.sensitivity.as_ref() {
                let sensitivity_mismatches = match self.compare_ac_sensitivity_outputs(
                    sensitivity_plan,
                    &netlist,
                    &plan.source,
                    &results,
                ) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("AC sensitivity reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
                if !sensitivity_mismatches.is_empty() {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "{} Xyce AC sensitivity reference mismatch(es)",
                            sensitivity_mismatches.len()
                        ),
                        sensitivity_mismatches,
                    );
                }
            }
            if !plan.measurement_reference_paths.is_empty()
                || !plan.continuous_measurement_reference_paths.is_empty()
            {
                let measurements =
                    rspice_core::analysis::evaluate_ac_measurements(&netlist, &results);
                let continuous =
                    rspice_core::analysis::evaluate_ac_continuous_measurements(
                        &netlist, &results,
                    );
                let measurement_mismatches = match self.compare_analysis_measurement_outputs(
                    &plan.measurement_reference_paths,
                    &plan.continuous_measurement_reference_paths,
                    &measurements,
                    &continuous,
                    plan.measurement_tolerance,
                    netlist.options.measure_fail_output,
                    netlist.options.measure_default_value,
                    netlist.options.measure_use_cont_files(),
                    &netlist.measurements,
                    "AC",
                    "AC_CONT",
                ) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("AC measurement reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
                if !measurement_mismatches.is_empty() {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "{} Xyce AC measurement mismatch(es)",
                            measurement_mismatches.len()
                        ),
                        measurement_mismatches,
                    );
                }
            }
            let side_mismatches = match self.compare_ac_side_outputs(&plan, &netlist, &results) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC side-output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if side_mismatches.is_empty() {
                let ac_ic_mismatches =
                    match self.compare_ac_initial_condition_outputs(&plan, &netlist) {
                        Ok(mismatches) => mismatches,
                        Err(err) => {
                            return self.failure_result(
                                deck,
                                start,
                                contract,
                                format!("AC initial-condition output comparison error: {err}"),
                                Vec::new(),
                            );
                        }
                    };
                if ac_ic_mismatches.is_empty() {
                    self.passed_result(deck, start, contract)
                } else {
                    self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "{} Xyce AC initial-condition output mismatch(es)",
                            ac_ic_mismatches.len()
                        ),
                        ac_ic_mismatches,
                    )
                }
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC side-output mismatch(es)", side_mismatches.len()),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce AC reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_noise_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticNoisePlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = if plan.output_override {
            "wrapper_output_override_prn_noise"
        } else if plan.reference_path.is_some() || !plan.side_references.is_empty() {
            plan.contract.result_contract(false)
        } else {
            "wrapper_scalar_measure_noise"
        };
        let netlist = match Self::parse_xyce_netlist(&plan.source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after NOISE contract validation: {err}"),
                    Vec::new(),
                );
            }
        };
        if !plan.steps.is_empty() {
            return self.run_static_step_noise_measurement_plan(deck, plan, netlist, start);
        }
        let engine = self.create_xyce_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut data_row_netlists = Vec::new();
        let results = if let Some(table_name) = plan.data_table_name.as_deref() {
            match engine.run_noise_data_named_with_input_source_and_abort(
                &netlist,
                &plan.output_node,
                plan.reference_node.as_deref(),
                &plan.input_source,
                table_name,
                netlist.options.temp.unwrap_or(27.0) + 273.15,
                &abort,
            ) {
                Ok((row_netlists, results)) => {
                    data_row_netlists = row_netlists;
                    results
                }
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms) during .NOISE DATA",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this .NOISE DATA deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(".NOISE DATA simulation error: {err}"),
                        Vec::new(),
                    );
                }
            }
        } else {
            let temperature = netlist.options.temp.unwrap_or(27.0) + 273.15;
            if !temperature.is_finite() || temperature <= 0.0 {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("NOISE temperature must be positive Kelvin, got {temperature}"),
                    Vec::new(),
                );
            }
            match engine.run_noise_named_with_input_source_and_abort(
                &netlist,
                &plan.output_node,
                plan.reference_node.as_deref(),
                &plan.input_source,
                &plan.frequencies,
                temperature,
                &abort,
            ) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this NOISE deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("NOISE simulation error: {err}"),
                        Vec::new(),
                    );
                }
            }
        };

        let mut mismatches = Vec::new();
        if let Some(reference_path) = plan.reference_path.as_ref() {
            let Some(print) = plan.print.as_ref() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    "NOISE reference has no print request".to_string(),
                    Vec::new(),
                );
            };
            let reference = match Self::parse_noise_reference_file(plan.contract, reference_path) {
                Ok(reference) => reference,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("failed to parse Xyce NOISE oracle: {err}"),
                        Vec::new(),
                    );
                }
            };
            let comparison = if plan.data_points.is_some() {
                self.compare_noise_data_prn_reference(
                    &reference,
                    print,
                    &netlist,
                    &plan.source,
                    &results,
                    &data_row_netlists,
                )
            } else {
                self.compare_noise_prn_reference(
                    &reference,
                    print,
                    &netlist,
                    &plan.source,
                    &results,
                )
            };
            match comparison {
                Ok(waveform_mismatches) => mismatches.extend(waveform_mismatches),
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("NOISE waveform reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            }
        }
        for side in &plan.side_references {
            let reference =
                match Self::parse_noise_reference_file(side.contract, &side.reference_path) {
                    Ok(reference) => reference,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "failed to parse Xyce NOISE side-output oracle '{}': {err}",
                                side.file
                            ),
                            Vec::new(),
                        );
                    }
                };
            let comparison = if plan.data_points.is_some() {
                self.compare_noise_data_prn_reference(
                    &reference,
                    &side.print,
                    &netlist,
                    &plan.source,
                    &results,
                    &data_row_netlists,
                )
            } else {
                self.compare_noise_prn_reference(
                    &reference,
                    &side.print,
                    &netlist,
                    &plan.source,
                    &results,
                )
            };
            match comparison {
                Ok(mut side_mismatches) => {
                    for mismatch in &mut side_mismatches {
                        mismatch.probe = format!("{}:{}", side.file, mismatch.probe);
                    }
                    mismatches.extend(side_mismatches);
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "NOISE side-output '{}' reference comparison error: {err}",
                            side.file
                        ),
                        Vec::new(),
                    );
                }
            }
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        if mismatches.is_empty()
            && (!plan.measurement_reference_paths.is_empty()
                || !plan.continuous_measurement_reference_paths.is_empty())
        {
            let scalar =
                rspice_core::analysis::evaluate_noise_measurements(&netlist, &results);
            let continuous =
                rspice_core::analysis::evaluate_noise_continuous_measurements(
                    &netlist, &results,
                );
            match self.compare_analysis_measurement_outputs(
                &plan.measurement_reference_paths,
                &plan.continuous_measurement_reference_paths,
                &scalar,
                &continuous,
                plan.measurement_tolerance,
                netlist.options.measure_fail_output,
                netlist.options.measure_default_value,
                netlist.options.measure_use_cont_files(),
                &netlist.measurements,
                "NOISE",
                "NOISE_CONT",
            ) {
                Ok(measurement_mismatches) => mismatches.extend(measurement_mismatches),
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("NOISE measurement reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            }
        }
        mismatches.truncate(self.config.max_mismatches);
        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce NOISE mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_step_noise_measurement_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticNoisePlan,
        netlist: Netlist,
        start: Instant,
    ) -> XyceTestResult {
        let contract = if plan.output_override {
            "wrapper_output_override_prn_step_noise"
        } else if plan.reference_path.is_some() || !plan.side_references.is_empty() {
            plan.contract.result_contract(true)
        } else {
            "wrapper_scalar_measure_step_noise"
        };
        if plan.reference_path.is_none()
            && plan.side_references.is_empty()
            && plan.measurement_reference_paths.is_empty()
        {
            return self.failure_result(
                deck,
                start,
                contract,
                ".STEP NOISE comparison requires a waveform or contiguous scalar measurement artifact"
                    .to_string(),
                Vec::new(),
            );
        }

        let (waveform_reference, tecplot_zones) = match plan.reference_path.as_deref() {
            Some(path) if plan.contract == XyceStaticNoiseContract::Tecplot => {
                match Self::parse_tecplot_reference_file(path) {
                    Ok(reference) => (Some(reference.table), Some(reference.zones)),
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("failed to parse stepped Xyce NOISE oracle: {err}"),
                            Vec::new(),
                        );
                    }
                }
            }
            Some(path) => match Self::parse_noise_reference_file(plan.contract, path) {
                Ok(reference) => (Some(reference), None),
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("failed to parse stepped Xyce NOISE oracle: {err}"),
                        Vec::new(),
                    );
                }
            },
            None => (None, None),
        };
        let waveform_print = match (waveform_reference.as_ref(), plan.print.as_ref()) {
            (Some(_), Some(print)) => Some(print),
            (Some(_), None) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    "stepped NOISE waveform reference has no print request".to_string(),
                    Vec::new(),
                );
            }
            (None, _) => None,
        };
        let side_waveform_references = match plan
            .side_references
            .iter()
            .map(|side| {
                Self::parse_noise_reference_file(side.contract, &side.reference_path)
                    .map(|reference| (side, reference))
                    .map_err(|err| {
                        format!(
                            "failed to parse stepped Xyce NOISE side-output oracle '{}': {err}",
                            side.file
                        )
                    })
            })
            .collect::<Result<Vec<_>, _>>()
        {
            Ok(references) => references,
            Err(err) => {
                return self.failure_result(deck, start, contract, err, Vec::new());
            }
        };

        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let expansion_engine = self.create_xyce_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &expansion_engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP NOISE deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };
        if !plan.measurement_reference_paths.is_empty()
            && step_runs.len() != plan.measurement_reference_paths.len()
        {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    ".STEP expansion produced {} batches but {} contiguous measurement artifacts exist",
                    step_runs.len(),
                    plan.measurement_reference_paths.len()
                ),
                Vec::new(),
            );
        }
        if let Some(zones) = tecplot_zones.as_deref()
            && let Err(err) = Self::validate_tecplot_step_bindings(zones, &plan.steps, &step_runs)
        {
            return self.failure_result(deck, start, contract, err, Vec::new());
        }

        let gs_rows = match plan.gs_reference_path.as_deref() {
            Some(path) => match Self::parse_measure_cont_gs_file(path) {
                Ok(rows) => Some(rows),
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("stepped NOISE GS oracle is invalid: {error}"),
                        Vec::new(),
                    );
                }
            },
            None => None,
        };
        let mut gs_offset = 0usize;

        let engine = self.create_xyce_engine();
        let mut all_mismatches = Vec::new();
        let mut waveform_offset = 0usize;
        let mut side_waveform_offsets = vec![0usize; side_waveform_references.len()];
        for (step_index, run) in step_runs.iter().enumerate() {
            let temperature = run.netlist.options.temp.unwrap_or(27.0) + 273.15;
            if !temperature.is_finite() || temperature <= 0.0 {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "NOISE temperature in step {} must be positive Kelvin, got {temperature}",
                        step_index + 1
                    ),
                    Vec::new(),
                );
            }
            let analysis = match Self::noise_analysis_for_netlist(&run.netlist) {
                Ok(analysis) if analysis.data_points.is_none() => analysis,
                Ok(_) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        ".STEP NOISE execution does not admit DATA sweeps".to_string(),
                        Vec::new(),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "NOISE analysis in step {} is invalid: {err}",
                            step_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            let results = match engine.run_noise_named_with_input_source_and_abort(
                &run.netlist,
                &analysis.output_node,
                analysis.reference_node.as_deref(),
                &analysis.input_source,
                &analysis.frequencies,
                temperature,
                &abort,
            ) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms) in NOISE step {}",
                            self.config.max_time_per_test_ms,
                            step_index + 1
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this .STEP NOISE deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error in NOISE step {}: {err}", step_index + 1),
                        Vec::new(),
                    );
                }
            };
            if let Some(zones) = tecplot_zones.as_deref() {
                let zone = &zones[step_index];
                if zone.row_start != waveform_offset || zone.row_count != results.len() {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "TECPLOT zone {} row range is {}..{}, expected {}..{} for NOISE step {}",
                            step_index + 1,
                            zone.row_start,
                            zone.row_start + zone.row_count,
                            waveform_offset,
                            waveform_offset + results.len(),
                            step_index + 1
                        ),
                        Vec::new(),
                    );
                }
            }
            let mut mismatches = Vec::new();
            if let (Some(reference), Some(print)) = (waveform_reference.as_ref(), waveform_print) {
                let (step_reference, end) = match Self::noise_step_reference_batch(
                    reference,
                    waveform_offset,
                    results.len(),
                    step_index,
                ) {
                    Ok(batch) => batch,
                    Err(err) => {
                        return self.failure_result(deck, start, contract, err, Vec::new());
                    }
                };
                waveform_offset = end;
                match self.compare_noise_prn_reference_with_step(
                    &step_reference,
                    print,
                    &run.netlist,
                    &plan.source,
                    &results,
                    Some(step_index),
                ) {
                    Ok(waveform_mismatches) => mismatches.extend(waveform_mismatches),
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "NOISE waveform comparison error in step {}: {err}",
                                step_index + 1
                            ),
                            Vec::new(),
                        );
                    }
                }
            }

            for (side_index, (side, reference)) in side_waveform_references.iter().enumerate() {
                let (step_reference, end) = match Self::noise_step_reference_batch(
                    reference,
                    side_waveform_offsets[side_index],
                    results.len(),
                    step_index,
                ) {
                    Ok(batch) => batch,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("NOISE side-output '{}': {err}", side.file),
                            Vec::new(),
                        );
                    }
                };
                side_waveform_offsets[side_index] = end;
                match self.compare_noise_prn_reference_with_step(
                    &step_reference,
                    &side.print,
                    &run.netlist,
                    &plan.source,
                    &results,
                    Some(step_index),
                ) {
                    Ok(mut side_mismatches) => {
                        for mismatch in &mut side_mismatches {
                            mismatch.probe = format!("{}:{}", side.file, mismatch.probe);
                        }
                        mismatches.extend(side_mismatches);
                    }
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "NOISE side-output '{}' waveform comparison error in step {}: {err}",
                                side.file,
                                step_index + 1
                            ),
                            Vec::new(),
                        );
                    }
                }
            }

            if let Some(reference_path) = plan.measurement_reference_paths.get(step_index) {
                let measurements = rspice_core::analysis::evaluate_noise_measurements(
                    &run.netlist,
                    &results,
                );
                let comparison = if run.netlist.options.measure_use_cont_files() {
                    self.compare_measurement_references(
                        std::slice::from_ref(reference_path),
                        &measurements,
                        plan.measurement_tolerance,
                        run.netlist.options.measure_fail_output,
                        run.netlist.options.measure_default_value,
                        "NOISE",
                        &run.netlist.measurements,
                    )
                } else {
                    let continuous =
                        rspice_core::analysis::evaluate_noise_continuous_measurements(
                            &run.netlist,
                            &results,
                        );
                    let comparison = self.compare_mixed_measurement_references(
                        std::slice::from_ref(reference_path),
                        &measurements,
                        &continuous,
                        plan.measurement_tolerance,
                        &run.netlist.measurements,
                        "NOISE",
                        "NOISE_CONT",
                    );
                    if let (Ok(_), Some(gs)) = (&comparison, gs_rows.as_deref()) {
                        match Self::compare_noise_step_gs_semantics(
                            gs,
                            gs_offset,
                            &run.netlist,
                            &measurements,
                            &continuous,
                            plan.measurement_tolerance,
                        ) {
                            Ok(consumed) => gs_offset += consumed,
                            Err(error) => {
                                return self.failure_result(
                                    deck,
                                    start,
                                    contract,
                                    format!(
                                        "NOISE step {} GS comparison error: {error}",
                                        step_index + 1
                                    ),
                                    Vec::new(),
                                );
                            }
                        }
                    }
                    comparison
                };
                match comparison {
                    Ok(measurement_mismatches) => mismatches.extend(measurement_mismatches),
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "NOISE step {} measurement comparison error: {err}",
                                step_index + 1
                            ),
                            Vec::new(),
                        );
                    }
                }
            }
            for mismatch in &mut mismatches {
                mismatch.probe = format!("step {step_index}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if let Some(reference) = waveform_reference.as_ref()
            && waveform_offset != reference.rows.len()
        {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "stepped NOISE waveform oracle left rows unclaimed: consumed {waveform_offset}/{}",
                    reference.rows.len()
                ),
                Vec::new(),
            );
        }

        for ((side, reference), offset) in
            side_waveform_references.iter().zip(&side_waveform_offsets)
        {
            if *offset != reference.rows.len() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "stepped NOISE side-output '{}' oracle left rows unclaimed: consumed {offset}/{}",
                        side.file,
                        reference.rows.len()
                    ),
                    Vec::new(),
                );
            }
        }

        if let Some(gs) = gs_rows.as_deref()
            && gs_offset != gs.len()
        {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "stepped NOISE GS oracle left rows unclaimed: consumed {gs_offset}/{}",
                    gs.len()
                ),
                Vec::new(),
            );
        }

        if all_mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce stepped NOISE mismatch(es)", all_mismatches.len()),
                all_mismatches,
            )
        }
    }

    pub(super) fn run_static_frequency_bound_ac_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        reference: XycePrnTable,
        frequencies: Vec<Value>,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(false);
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                "frequency-bound AC comparison requires a primary .PRINT AC request".to_string(),
                Vec::new(),
            );
        };

        let engine = self.create_xyce_engine();
        let mut point_results = Vec::with_capacity(frequencies.len());
        for (row_index, frequency) in frequencies.iter().copied().enumerate() {
            let point_source = Self::source_with_ac_frequency_bindings(&plan.source, frequency);
            let point_netlist = match Self::parse_xyce_netlist(&point_source, &plan.deck_path) {
                Ok(netlist) => netlist,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "frequency-bound AC row {} parse error at FREQ={frequency}: {err}",
                            row_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            let mut results = match engine.run_ac(&point_netlist, &[frequency]) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this frequency-bound AC deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation error in frequency-bound AC row {}: {err}",
                            row_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            let Some(result) = results.pop() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "frequency-bound AC row {} produced no AC result",
                        row_index + 1
                    ),
                    Vec::new(),
                );
            };
            point_results.push(XyceAcDataPointResult {
                netlist: point_netlist,
                result,
            });
        }

        let mismatches = match self.compare_ac_data_prn_reference(
            &reference,
            primary_print,
            &plan.source,
            &point_results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            let side_mismatches = match self.compare_ac_data_side_outputs(&plan, &point_results) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC side-output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if side_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC side-output mismatch(es)", side_mismatches.len()),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce AC reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_ac_data_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(false);
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                ".AC DATA comparison requires a primary .PRINT AC request".to_string(),
                Vec::new(),
            );
        };
        let Some(data_points) = plan.ac.data_points() else {
            return self.failure_result(
                deck,
                start,
                contract,
                ".AC DATA comparison has no data rows".to_string(),
                Vec::new(),
            );
        };

        let engine = self.create_xyce_engine();
        let mut point_results = Vec::with_capacity(data_points.len());
        for (row_index, point) in data_points.iter().enumerate() {
            let row_netlist =
                match Engine::create_perturbed_netlist_multi(&netlist, &point.overrides) {
                    Ok((row_netlist, _)) => row_netlist,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                ".AC DATA row {} parameter override error: {err}",
                                row_index + 1
                            ),
                            Vec::new(),
                        );
                    }
                };
            let mut results = match engine.run_ac(&row_netlist, &[point.frequency]) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .AC DATA deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error in .AC DATA row {}: {err}", row_index + 1),
                        Vec::new(),
                    );
                }
            };
            let Some(result) = results.pop() else {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".AC DATA row {} produced no AC result", row_index + 1),
                    Vec::new(),
                );
            };
            point_results.push(XyceAcDataPointResult {
                netlist: row_netlist,
                result,
            });
        }

        let mismatches = match self.compare_ac_data_prn_reference(
            &reference,
            primary_print,
            &plan.source,
            &point_results,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            let side_mismatches = match self.compare_ac_data_side_outputs(&plan, &point_results) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC side-output comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if side_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce AC side-output mismatch(es)", side_mismatches.len()),
                    side_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce AC reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_step_ac_sensitivity_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        netlist: Netlist,
        frequencies: Vec<Value>,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(true);
        let Some(sensitivity_plan) = plan.sensitivity.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                "stepped AC sensitivity plan has no sensitivity output contract".to_string(),
                Vec::new(),
            );
        };
        if plan.ac.data_points().is_some() || plan.frequency_bound {
            return self.expected_unsupported_result(
                deck,
                start,
                "unsupported_xyce_contract",
                "stepped AC sensitivity comparison requires an ordinary frequency sweep",
            );
        }
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let expansion_engine = self.create_xyce_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &expansion_engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!(
                        "RSpice runtime does not yet support this .STEP AC sensitivity deck: {err}"
                    ),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };

        let engine = self.create_xyce_engine();
        let mut batches = Vec::with_capacity(step_runs.len());
        for (step_index, run) in step_runs.iter().enumerate() {
            let results = match engine.run_ac(&run.netlist, &frequencies) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP AC sensitivity deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation error in AC sensitivity step {}: {err}",
                            step_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            batches.push(XyceAcResultBatch {
                netlist: run.netlist.clone(),
                results,
            });
        }

        let mismatches = match self.compare_step_ac_sensitivity_outputs(
            sensitivity_plan,
            &plan.source,
            frequencies.len(),
            &batches,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("AC stepped sensitivity reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped AC sensitivity mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_step_ac_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        frequencies: Vec<Value>,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(true);
        let Some(primary_print) = plan.print.as_ref() else {
            return self.failure_result(
                deck,
                start,
                contract,
                ".STEP AC comparison requires a primary .PRINT AC request".to_string(),
                Vec::new(),
            );
        };
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let expansion_engine = self.create_xyce_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &expansion_engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP AC deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };

        let engine = self.create_xyce_engine();
        let mut batches = Vec::with_capacity(step_runs.len());
        for (step_index, run) in step_runs.iter().enumerate() {
            let results = match engine.run_ac(&run.netlist, &frequencies) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP AC deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error in AC step {}: {err}", step_index + 1),
                        Vec::new(),
                    );
                }
            };
            batches.push(XyceAcResultBatch {
                netlist: run.netlist.clone(),
                results,
            });
        }

        let mismatches = match self.compare_step_ac_reference_batches(
            &reference,
            primary_print,
            &plan.source,
            frequencies.len(),
            &batches,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };

        if mismatches.is_empty() {
            let side_mismatches =
                match self.compare_step_ac_side_outputs(&plan, frequencies.len(), &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("AC side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            let sensitivity_mismatches = if let Some(sensitivity_plan) = plan.sensitivity.as_ref() {
                match self.compare_step_ac_sensitivity_outputs(
                    sensitivity_plan,
                    &plan.source,
                    frequencies.len(),
                    &batches,
                ) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("AC stepped sensitivity reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                }
            } else {
                Vec::new()
            };
            if side_mismatches.is_empty() && sensitivity_mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                let mut all_mismatches = side_mismatches;
                all_mismatches.extend(sensitivity_mismatches);
                all_mismatches.truncate(self.config.max_mismatches);
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce stepped AC output mismatch(es)",
                        all_mismatches.len()
                    ),
                    all_mismatches,
                )
            }
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped AC reference mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_step_ac_measurement_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticAcPlan,
        netlist: Netlist,
        frequencies: Vec<Value>,
        start: Instant,
    ) -> XyceTestResult {
        let contract = "wrapper_scalar_measure_step_ac";
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let expansion_engine = self.create_xyce_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &expansion_engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP AC deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };
        if step_runs.len() != plan.measurement_reference_paths.len() {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    ".STEP expansion produced {} batches but {} contiguous measurement artifacts exist",
                    step_runs.len(),
                    plan.measurement_reference_paths.len()
                ),
                Vec::new(),
            );
        }

        let engine = self.create_xyce_engine();
        let mut all_mismatches = Vec::new();
        for (step_index, (run, reference_path)) in step_runs
            .iter()
            .zip(&plan.measurement_reference_paths)
            .enumerate()
        {
            let results = match engine.run_ac(&run.netlist, &frequencies) {
                Ok(results) => results,
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP AC deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error in AC step {}: {err}", step_index + 1),
                        Vec::new(),
                    );
                }
            };
            let measurements =
                rspice_core::analysis::evaluate_ac_measurements(&run.netlist, &results);
            let continuous = rspice_core::analysis::evaluate_ac_continuous_measurements(
                &run.netlist,
                &results,
            );
            let mut mismatches = match self.compare_analysis_measurement_outputs(
                std::slice::from_ref(reference_path),
                &[],
                &measurements,
                &continuous,
                plan.measurement_tolerance,
                run.netlist.options.measure_fail_output,
                run.netlist.options.measure_default_value,
                run.netlist.options.measure_use_cont_files(),
                &run.netlist.measurements,
                "AC",
                "AC_CONT",
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("AC step {step_index} measurement comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut mismatches {
                mismatch.probe = format!("step {step_index}:{}", mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if all_mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped AC measurement mismatch(es)",
                    all_mismatches.len()
                ),
                all_mismatches,
            )
        }
    }

    pub(super) fn run_static_prn_dc_sensitivity_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticDcSensitivityPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = if plan.dc.steps.is_empty() {
            "wrapper_static_sens_dc"
        } else {
            "wrapper_static_sens_step_dc"
        };
        if plan.dc.dc_data.is_some() {
            return self.expected_unsupported_result(
                deck,
                start,
                "unsupported_xyce_contract",
                "native static DC sensitivity does not yet cover .DC DATA tables",
            );
        }
        let netlist = match Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
            &plan.dc.source,
            &plan.dc.deck_path,
            plan.dc.expression_dialect,
            plan.dc.parameter_redefinition_policy,
            plan.dc.execution_dir.as_deref(),
        ) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after static DC sensitivity validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let batches = if plan.dc.steps.is_empty() {
            match self.run_static_dc_result_batches(&netlist, &plan.dc.dc, start) {
                Ok(batches) => batches,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this DC sensitivity deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            }
        } else {
            let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
            let engine = self.create_dc_engine();
            let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
                &engine,
                &netlist,
                &plan.dc.steps,
                xyce_step_plan_limits(),
                &abort,
            ) {
                Ok(runs) => runs,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            ".STEP expansion exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support this .STEP sensitivity deck: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(".STEP expansion error: {err}"),
                        Vec::new(),
                    );
                }
            };
            let mut batches = Vec::with_capacity(step_runs.len());
            for run in step_runs {
                let results = match engine.run_dc_sweep2_spec_with_report_and_abort(
                    &run.netlist,
                    &plan.dc.dc.source,
                    &plan.dc.dc.primary_spec(),
                    plan.dc.dc.sweep2.as_ref(),
                    &abort,
                ) {
                    Ok(results) => results,
                    Err(SimulationError::Aborted) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "simulation exceeded timeout ({}ms)",
                                self.config.max_time_per_test_ms
                            ),
                            Vec::new(),
                        );
                    }
                    Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                        return self.expected_unsupported_result(
                            deck,
                            start,
                            "unsupported_xyce_runtime",
                            &format!("RSpice runtime does not yet support this stepped sensitivity deck: {err}"),
                        );
                    }
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("simulation error: {err}"),
                            Vec::new(),
                        );
                    }
                };
                batches.push(XyceDcResultBatch {
                    netlist: run.netlist,
                    results,
                });
            }
            batches
        };

        let mismatches = match self.compare_dc_sensitivity_outputs(&plan, &batches, start) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("DC sensitivity reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce DC sensitivity mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_static_prn_dc_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceExecutionPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = if !plan.reference_path.is_file()
            && (!plan.measurement_reference_paths.is_empty()
                || !plan.continuous_measurement_reference_paths.is_empty())
        {
            "wrapper_scalar_measure_dc"
        } else {
            plan.contract.result_contract(false)
        };
        let netlist = match Self::parse_netlist_with_expression_dialect_and_execution_dir(
            &plan.source,
            &plan.deck_path,
            plan.expression_dialect,
            plan.execution_dir.as_deref(),
        ) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let reference = if !plan.reference_path.is_file()
            || matches!(
                plan.contract,
                XyceStaticDcContract::WrapperFilePrn | XyceStaticDcContract::WrapperNoOutput
            ) {
            None
        } else {
            match Self::parse_dc_reference_file(plan.contract, &plan.reference_path) {
                Ok(reference) => Some(reference),
                Err(err) if Self::is_parameter_sweep_summary_reference(&plan.reference_path) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_oracle",
                        &format!(
                            "checked-in Xyce sidecar is a parameter-sweep summary, not a numeric .PRINT table: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("failed to parse Xyce reference oracle: {err}"),
                        Vec::new(),
                    );
                }
            }
        };

        if !plan.steps.is_empty() {
            return if let Some(reference) = reference {
                self.run_static_prn_step_dc_plan(deck, plan, netlist, reference, start)
            } else if !plan.measurement_reference_paths.is_empty() {
                self.run_static_step_dc_measurement_plan(deck, plan, netlist, start)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    "file-output-only .STEP DC comparison is not implemented".to_string(),
                    Vec::new(),
                )
            };
        }

        if let Some(dc_data) = &plan.dc_data {
            let results = match self.run_static_dc_data_results(&netlist, dc_data, start) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            };
            let mut mismatches = if let Some(reference) = reference.as_ref() {
                match self.compare_dc_data_prn_reference(
                    reference,
                    &plan.print,
                    &plan.source,
                    &plan.dc,
                    &results,
                ) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                }
            } else {
                Vec::new()
            };
            if mismatches.is_empty() && !plan.measurement_reference_paths.is_empty() {
                let measurement_sweep = results
                    .iter()
                    .map(|row| (row.point.sweep_value, row.point.result.clone()))
                    .collect::<Vec<_>>();
                let point_params = results
                    .iter()
                    .map(|row| row.netlist.params.clone())
                    .collect::<Vec<_>>();
                let measurements =
                    rspice_core::analysis::evaluate_dc_measurements_with_parameter_contexts(
                        &netlist,
                        &measurement_sweep,
                        &point_params,
                    );
                match self.compare_measurement_references(
                    &plan.measurement_reference_paths,
                    &measurements,
                    plan.measurement_tolerance,
                    netlist.options.measure_fail_output,
                    netlist.options.measure_default_value,
                    "DC",
                    &netlist.measurements,
                ) {
                    Ok(measurement_mismatches) => mismatches.extend(measurement_mismatches),
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("DC DATA measurement reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                }
            }
            return if mismatches.is_empty() {
                self.passed_result(deck, start, contract)
            } else {
                self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("{} Xyce reference mismatch(es)", mismatches.len()),
                    mismatches,
                )
            };
        }

        let batches = match self.run_static_dc_result_batches(&netlist, &plan.dc, start) {
            Ok(batches) => batches,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("simulation error: {err}"),
                    Vec::new(),
                );
            }
        };

        let mismatches = if let Some(reference) = &reference {
            match self.compare_dc_prn_reference_batches(
                reference,
                &plan.print,
                &plan.source,
                &plan.dc,
                &batches,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            }
        } else {
            Vec::new()
        };

        let results = batches
            .iter()
            .flat_map(|batch| batch.results.iter().cloned())
            .collect::<Vec<_>>();

        if mismatches.is_empty()
            && (!plan.measurement_reference_paths.is_empty()
                || !plan.continuous_measurement_reference_paths.is_empty())
        {
            let measurement_sweep = results
                .iter()
                .map(|point| (point.sweep_value, point.result.clone()))
                .collect::<Vec<_>>();
            let measurements = if netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Dc { .. }))
            {
                rspice_core::analysis::evaluate_dc_measurements(
                    &netlist,
                    &measurement_sweep,
                )
            } else {
                rspice_core::analysis::unevaluated_measurements(
                    &netlist,
                    "DC",
                    "DC measurement requires an explicit .DC analysis",
                )
            };
            let continuous = rspice_core::analysis::evaluate_dc_continuous_measurements(
                &netlist,
                &measurement_sweep,
            );
            let measurement_mismatches = match self.compare_analysis_measurement_outputs(
                &plan.measurement_reference_paths,
                &plan.continuous_measurement_reference_paths,
                &measurements,
                &continuous,
                plan.measurement_tolerance,
                netlist.options.measure_fail_output,
                netlist.options.measure_default_value,
                netlist.options.measure_use_cont_files(),
                &netlist.measurements,
                "DC",
                "DC_CONT",
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("DC measurement reference comparison error: {err}"),
                        Vec::new(),
                    );
                }
            };
            if !measurement_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce DC measurement mismatch(es)",
                        measurement_mismatches.len()
                    ),
                    measurement_mismatches,
                );
            }
        }

        if mismatches.is_empty()
            && matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot)
        {
            let side_mismatches =
                match self.compare_gnuplot_splot_side_output_batches(&plan, &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("GNUPLOT/SPLOT side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if !side_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce GNUPLOT/SPLOT side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                );
            }
        }

        if mismatches.is_empty()
            && !matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot)
        {
            let side_mismatches =
                match self.compare_prn_compatible_side_output_batches(&plan, &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("PRN-compatible side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if !side_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce PRN-compatible side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                );
            }
        }

        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!("{} Xyce reference mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_noindex_header_tran_wrapper_plan(
        &self,
        deck: &XyceDeck,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.result_contract();
        if !plan.steps.is_empty() {
            return self.failure_result(
                deck,
                start,
                contract,
                "NOINDEX header-only transient wrapper contract does not cover .STEP output"
                    .to_string(),
                Vec::new(),
            );
        }

        // The authoritative wrapper still requires a successful simulator run,
        // but deliberately ignores every numeric output row.
        match self.run_transient_family_plan(plan, start, None, None) {
            Ok((_netlist, _result)) => {}
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this transient deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("simulation error: {err}"),
                    Vec::new(),
                );
            }
        }

        let header = Self::transient_prn_header_columns(&plan.print, false).join("   ");
        if let Err(err) = Self::validate_noindex_tran_prn_header(&header) {
            return self.failure_result(
                deck,
                start,
                contract,
                format!("NOINDEX transient output header error: {err}"),
                Vec::new(),
            );
        }

        self.passed_result(deck, start, contract)
    }

    pub(super) fn run_static_prn_tran_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticTranPlan,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.result_contract();
        if matches!(plan.contract, XyceStaticTranContract::WrapperNoIndexHeader) {
            return self.run_noindex_header_tran_wrapper_plan(deck, &plan, start);
        }
        let netlist = match Self::parse_xyce_netlist(&plan.source, &plan.deck_path) {
            Ok(netlist) => netlist,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("parse failed after contract validation: {err}"),
                    Vec::new(),
                );
            }
        };

        let reference_result = match plan.comparison_mode {
            XyceStaticTranComparisonMode::Release710IntegratedRms { .. }
            | XyceStaticTranComparisonMode::Release710IntegratedRmsComp { .. } => {
                Self::parse_xyce_verify_tran_reference_file(&plan.reference_path)
            }
            XyceStaticTranComparisonMode::Pointwise => {
                Self::parse_tran_reference_file(plan.contract, &plan.reference_path)
            }
        };
        let reference = match reference_result {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("failed to parse Xyce transient oracle: {err}"),
                    Vec::new(),
                );
            }
        };
        if !plan.steps.is_empty() {
            return self.run_static_prn_step_tran_plan(deck, plan, netlist, reference, start);
        }
        let reference_time_grid_result = match plan.comparison_mode {
            XyceStaticTranComparisonMode::Release710IntegratedRms {
                scientific_precision,
            }
            | XyceStaticTranComparisonMode::Release710IntegratedRmsComp {
                scientific_precision,
                ..
            } => Self::xyce_verify_reference_time_grid(&plan, &reference, scientific_precision),
            XyceStaticTranComparisonMode::Pointwise => Self::reference_time_grid(&reference),
        };
        let reference_time_grid = match reference_time_grid_result {
            Ok(grid) => grid,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference time-grid error: {err}"),
                    Vec::new(),
                );
            }
        };
        let tran = Self::tran_analysis_for_reference_stop(
            plan.contract,
            plan.tran,
            reference_time_grid.as_slice(),
        );

        let max_step =
            match Self::transient_max_step_for_static_plan(&plan, &netlist, &tran, &reference) {
                Ok(max_step) => max_step,
                Err(err)
                    if !plan.comparison_mode.uses_integrated_rms_verifier()
                        && err.contains("transient harness execution envelope") =>
                {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_contract",
                        &err,
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("transient maximum-step or execution-envelope error: {err}"),
                        Vec::new(),
                    );
                }
            };
        // The reference table supplies mandatory comparison boundaries, not
        // the solver's internal DELMAX.  Native MOS3 candidates retain
        // Xyce's DELMAX because their charge history is path-sensitive;
        // other candidates use the bounded oracle ceiling for precision.
        let locked_solver_max_step = if Self::netlist_requires_xyce_locked_solver_ceiling(&netlist)
        {
            Self::xyce_transient_solver_max_step(&tran)
        } else {
            Self::transient_oracle_solver_max_step_for_netlist(&netlist, &tran)
        };
        let locked_max_step = locked_solver_max_step.max(max_step);

        let initial_step = Self::xyce_initial_timestep_for_tran(&plan.tran);
        // The reference-backed EKV26 envelope is qualified at the Xyce
        // accepted-breakpoint grid.  Its explicit four-terminal pair and
        // canonical constructor guard make this deterministic grid an
        // execution contract, while other integrated-RMS decks retain their
        // independent adaptive candidate grid.
        let locked_time_grid = if Self::netlist_is_native_transient_ekv26_pair(&netlist)
            && plan.comparison_mode.uses_integrated_rms_verifier()
        {
            Some(reference_time_grid.clone())
        } else {
            None
        };
        let engine = self.create_xyce_static_tran_engine(locked_time_grid, initial_step);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut best_mismatches = None;
        let mut simulation_error = None;
        let mut fallback_errors = Vec::new();
        match engine.run_tran_with_abort(&netlist, tran.stop, max_step, &abort) {
            Ok(result) => {
                let mismatches = match self
                    .compare_static_tran_primary_reference(&reference, &plan, &netlist, &result)
                {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("reference comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
                if mismatches.is_empty() {
                    return self.passed_or_tran_side_output_failure(
                        deck, start, contract, &plan, &netlist, &result,
                    );
                }
                match self.pointwise_switch_transition_rms_fallback_passes(
                    &reference, &plan, &netlist, &result,
                ) {
                    Ok(true) => {
                        return self.passed_or_tran_side_output_failure(
                            deck, start, contract, &plan, &netlist, &result,
                        );
                    }
                    Err(err) => fallback_errors.push(format!(
                        "switch-transition integrated-RMS comparison error: {err}"
                    )),
                    Ok(false) => {}
                }

                best_mismatches = Some(mismatches);
            }
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err)
                if !plan.comparison_mode.uses_integrated_rms_verifier()
                    && Self::is_expected_unsupported_runtime_error(&err) =>
            {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this transient deck: {err}"),
                );
            }
            Err(err) => {
                simulation_error = Some(format!("simulation error: {err}"));
            }
        }

        if plan.comparison_mode.uses_integrated_rms_verifier() {
            if let Some(mismatches) = best_mismatches {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce integrated-RMS transient reference mismatch(es)",
                        mismatches.len()
                    ),
                    mismatches,
                );
            }
            return self.failure_result(
                deck,
                start,
                contract,
                simulation_error.unwrap_or_else(|| {
                    "integrated-RMS transient execution produced no comparable result".to_string()
                }),
                Vec::new(),
            );
        }

        let capacitor_branch_print =
            Self::transient_print_requests_linear_capacitor_branch_quantity(&netlist, &plan.print);
        let has_solution_dependent_capacitor = netlist.elements.iter().any(|element| {
            matches!(
                &element.kind,
                ElementKind::Capacitor {
                    value,
                    value_expr: Some(_),
                    ..
                } if !value.is_finite()
            )
        });

        let locked_engine =
            self.create_xyce_static_tran_engine(Some(reference_time_grid.clone()), initial_step);
        match locked_engine.run_tran_with_abort(&netlist, tran.stop, locked_max_step, &abort) {
            Ok(locked_result) => {
                match self.compare_tran_prn_reference(
                    &reference,
                    &plan.print,
                    &netlist,
                    &plan.source,
                    &locked_result,
                    plan.wrapper_tolerance,
                ) {
                    Ok(locked_mismatches) => {
                        if locked_mismatches.is_empty() {
                            return self.passed_or_tran_side_output_failure(
                                deck,
                                start,
                                contract,
                                &plan,
                                &netlist,
                                &locked_result,
                            );
                        }
                        match self.pointwise_switch_transition_rms_fallback_passes(
                            &reference,
                            &plan,
                            &netlist,
                            &locked_result,
                        ) {
                            Ok(true) => {
                                return self.passed_or_tran_side_output_failure(
                                    deck,
                                    start,
                                    contract,
                                    &plan,
                                    &netlist,
                                    &locked_result,
                                );
                            }
                            Err(err) => fallback_errors.push(format!(
                                "locked switch-transition integrated-RMS comparison error: {err}"
                            )),
                            Ok(false) => {}
                        }
                        if Self::candidate_mismatches_are_better(
                            best_mismatches.as_deref(),
                            &locked_mismatches,
                        ) {
                            best_mismatches = Some(locked_mismatches);
                        }
                    }
                    Err(err) => {
                        fallback_errors.push(format!(
                            "locked time-grid reference comparison error: {err}"
                        ));
                    }
                }
            }
            Err(SimulationError::Aborted) => {
                fallback_errors.push(format!(
                    "locked time-grid simulation exceeded timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ));
            }
            Err(err) => {
                fallback_errors.push(format!("locked time-grid simulation error: {err}"));
            }
        }

        if !capacitor_branch_print && !has_solution_dependent_capacitor {
            let backward_euler_engine = self
                .create_xyce_static_tran_engine_with_integration_method(
                    Some(reference_time_grid.clone()),
                    rspice_core::analysis::IntegrationMethod::BackwardEuler,
                    initial_step,
                );
            match backward_euler_engine.run_tran_with_abort(
                &netlist,
                tran.stop,
                locked_max_step,
                &abort,
            ) {
                Ok(backward_euler_result) => {
                    match self.compare_tran_prn_reference(
                        &reference,
                        &plan.print,
                        &netlist,
                        &plan.source,
                        &backward_euler_result,
                        plan.wrapper_tolerance,
                    ) {
                        Ok(backward_euler_mismatches) => {
                            if backward_euler_mismatches.is_empty() {
                                return self.passed_or_tran_side_output_failure(
                                    deck,
                                    start,
                                    contract,
                                    &plan,
                                    &netlist,
                                    &backward_euler_result,
                                );
                            }
                            match self.pointwise_switch_transition_rms_fallback_passes(
                                &reference,
                                &plan,
                                &netlist,
                                &backward_euler_result,
                            ) {
                                Ok(true) => {
                                    return self.passed_or_tran_side_output_failure(
                                        deck,
                                        start,
                                        contract,
                                        &plan,
                                        &netlist,
                                        &backward_euler_result,
                                    );
                                }
                                Err(err) => fallback_errors.push(format!(
                                    "backward-Euler switch-transition integrated-RMS comparison error: {err}"
                                )),
                                Ok(false) => {}
                            }
                            if Self::candidate_mismatches_are_better(
                                best_mismatches.as_deref(),
                                &backward_euler_mismatches,
                            ) {
                                best_mismatches = Some(backward_euler_mismatches);
                            }
                        }
                        Err(err) => {
                            fallback_errors
                                .push(format!("backward-Euler reference comparison error: {err}"));
                        }
                    }
                }
                Err(SimulationError::Aborted) => {
                    fallback_errors.push(format!(
                        "backward-Euler simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ));
                }
                Err(err) => {
                    fallback_errors.push(format!("backward-Euler simulation error: {err}"));
                }
            }
        }

        if plan.timeint_conststep && !capacitor_branch_print && !has_solution_dependent_capacitor {
            let gear12_engine = self.create_xyce_static_tran_engine_with_integration_method(
                Some(reference_time_grid),
                rspice_core::analysis::IntegrationMethod::Gear2,
                initial_step,
            );
            match gear12_engine.run_tran_with_abort(&netlist, tran.stop, locked_max_step, &abort) {
                Ok(gear12_result) => {
                    match self.compare_tran_prn_reference(
                        &reference,
                        &plan.print,
                        &netlist,
                        &plan.source,
                        &gear12_result,
                        plan.wrapper_tolerance,
                    ) {
                        Ok(gear12_mismatches) => {
                            if gear12_mismatches.is_empty() {
                                return self.passed_or_tran_side_output_failure(
                                    deck,
                                    start,
                                    contract,
                                    &plan,
                                    &netlist,
                                    &gear12_result,
                                );
                            }
                            if Self::candidate_mismatches_are_better(
                                best_mismatches.as_deref(),
                                &gear12_mismatches,
                            ) {
                                best_mismatches = Some(gear12_mismatches);
                            }
                        }
                        Err(err) => {
                            fallback_errors
                                .push(format!("Xyce Gear12 reference comparison error: {err}"));
                        }
                    }
                }
                Err(SimulationError::Aborted) => {
                    fallback_errors.push(format!(
                        "Xyce Gear12 simulation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ));
                }
                Err(err) => {
                    fallback_errors.push(format!("Xyce Gear12 simulation error: {err}"));
                }
            }
        }

        if let Some(best_mismatches) = best_mismatches {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce transient reference mismatch(es)",
                    best_mismatches.len()
                ),
                best_mismatches,
            )
        } else {
            let mut message = simulation_error.unwrap_or_else(|| {
                "transient simulation produced no comparable result".to_string()
            });
            if !fallback_errors.is_empty() {
                message.push_str("; ");
                message.push_str(&fallback_errors.join("; "));
            }
            self.failure_result(deck, start, contract, message, Vec::new())
        }
    }

    pub(super) fn run_static_prn_step_tran_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceStaticTranPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.result_contract();
        let uses_integrated_rms = plan.comparison_mode.uses_integrated_rms_verifier();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let expansion_engine = self.create_xyce_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &expansion_engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP TRAN deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };

        let step_references =
            match Self::split_transient_step_reference(&reference, step_runs.len()) {
                Ok(references) => references,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("stepped transient oracle error: {err}"),
                        Vec::new(),
                    );
                }
            };

        let mismatches =
            match self.compare_step_tran_runs(&plan, &step_runs, &step_references, &abort, false) {
                Ok(mismatches) => {
                    if mismatches.is_empty() {
                        return self.passed_or_step_tran_side_output_failure(
                            deck, start, contract, &plan, &step_runs, &abort, false,
                        );
                    }
                    if uses_integrated_rms {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "{} Xyce integrated-RMS stepped transient reference mismatch(es)",
                                mismatches.len()
                            ),
                            mismatches,
                        );
                    }
                    Some(mismatches)
                }
                Err(err) if err.starts_with("UNSUPPORTED:") => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        err.trim_start_matches("UNSUPPORTED:").trim(),
                    );
                }
                Err(err) => {
                    if uses_integrated_rms {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "integrated-RMS stepped transient reference comparison error: {err}"
                            ),
                            Vec::new(),
                        );
                    }
                    let locked_result = self.compare_step_tran_runs(
                        &plan,
                        &step_runs,
                        &step_references,
                        &abort,
                        true,
                    );
                    return match locked_result {
                        Ok(locked_mismatches) if locked_mismatches.is_empty() => self
                            .passed_or_step_tran_side_output_failure(
                                deck, start, contract, &plan, &step_runs, &abort, true,
                            ),
                        Ok(locked_mismatches) => self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "{} Xyce stepped transient reference mismatch(es)",
                                locked_mismatches.len()
                            ),
                            locked_mismatches,
                        ),
                        Err(locked_err) => self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("{err}; locked time-grid retry failed: {locked_err}"),
                            Vec::new(),
                        ),
                    };
                }
            };

        let mismatches = mismatches.expect("non-empty adaptive mismatches");

        if let Ok(locked_mismatches) =
            self.compare_step_tran_runs(&plan, &step_runs, &step_references, &abort, true)
        {
            if locked_mismatches.is_empty() {
                return self.passed_or_step_tran_side_output_failure(
                    deck, start, contract, &plan, &step_runs, &abort, true,
                );
            }
            if Self::candidate_mismatches_are_better(Some(&mismatches), &locked_mismatches) {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce stepped transient reference mismatch(es)",
                        locked_mismatches.len()
                    ),
                    locked_mismatches,
                );
            }
        }

        self.failure_result(
            deck,
            start,
            contract,
            format!(
                "{} Xyce stepped transient reference mismatch(es)",
                mismatches.len()
            ),
            mismatches,
        )
    }

    pub(super) fn run_numbered_redefinition_dc_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceNumberedRedefinitionDcFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let contract_name = contract.role.contract();
        if contract
            .member_paths
            .first()
            .is_none_or(|path| !Self::same_path(path, &contract.baseline_path))
            || contract.member_paths.len() != 3
            || !contract.owner_path.is_file()
        {
            return self.failure_result(
                deck,
                start,
                contract_name,
                "numbered redefinition family lost its qualified owner/baseline/member invariants"
                    .to_string(),
                Vec::new(),
            );
        }

        let baseline_plan = match self.static_dc_plan_for_path_with_redefinition_policy(
            &contract.baseline_path,
            ExpressionDialect::Xyce,
            contract.parameter_redefinition_policy,
        ) {
            Ok(plan) => plan,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract_name,
                    format!(
                        "family '{}' baseline no longer plans: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        let (baseline_netlist, baseline_results) =
            match self.run_static_dc_results(&baseline_plan, start) {
                Ok(run) => run,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "family '{}' baseline failed after qualification: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
        let baseline_table = match self.dc_results_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_results,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract_name,
                    format!(
                        "family '{}' baseline output conversion failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let targets = match contract.role {
            XyceNumberedRedefinitionDcFamilyRole::Owner
            | XyceNumberedRedefinitionDcFamilyRole::Baseline => {
                contract.member_paths.iter().skip(1).cloned().collect()
            }
            XyceNumberedRedefinitionDcFamilyRole::Member(index) => {
                let Some(path) = contract.member_paths.get(index) else {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        "qualified member index is outside the family".to_string(),
                        Vec::new(),
                    );
                };
                vec![path.clone()]
            }
        };
        let mut all_mismatches = Vec::new();
        for target_path in targets {
            let target_plan = match self.static_dc_plan_for_path_with_redefinition_policy(
                &target_path,
                ExpressionDialect::Xyce,
                contract.parameter_redefinition_policy,
            ) {
                Ok(plan) => plan,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "family '{}' member '{}' no longer plans: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            let (target_netlist, target_results) =
                match self.run_static_dc_results(&target_plan, start) {
                    Ok(run) => run,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract_name,
                            format!(
                                "family '{}' member '{}' failed after qualification: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
            let target_table = match self.dc_results_to_prn_table(
                &target_plan,
                &target_netlist,
                &target_results,
            ) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "family '{}' member '{}' output conversion failed: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            let mut mismatches =
                match self.compare_serialized_default_prn_tables(&baseline_table, &target_table) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract_name,
                            format!(
                                "family '{}' member '{}' exact comparison failed: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{} {}", self.display_path(&target_path), mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }
        if all_mismatches.is_empty() {
            self.passed_result(deck, start, contract_name)
        } else {
            self.failure_result(
                deck,
                start,
                contract_name,
                format!(
                    "{} exact default-PRN mismatch(es) in family '{}'",
                    all_mismatches.len(),
                    contract.family
                ),
                all_mismatches,
            )
        }
    }

    pub(super) fn run_shared_stepped_dc_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceSharedSteppedDcFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let contract_name = contract.role.contract();
        if contract
            .member_paths
            .first()
            .is_none_or(|path| !Self::same_path(path, &contract.baseline_path))
            || contract.prn_reference_path.with_extension("res") != contract.res_reference_path
            || !contract.owner_path.is_file()
        {
            return self.failure_result(
                deck,
                start,
                contract_name,
                "shared stepped-DC family contract lost its qualified owner/baseline/oracle invariants".to_string(),
                Vec::new(),
            );
        }
        let targets = if matches!(contract.role, XyceSharedSteppedDcFamilyRole::Owner) {
            contract.member_paths.clone()
        } else {
            vec![deck.path.clone()]
        };
        for target in targets {
            let static_plan = match self.static_dc_plan_for_path(&target, ExpressionDialect::Xyce) {
                Ok(plan) => plan,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "qualified family '{}' member '{}' no longer plans: {err}",
                            contract.family,
                            self.display_path(&target)
                        ),
                        Vec::new(),
                    );
                }
            };
            let plan = XyceExecutionPlan {
                deck_path: static_plan.deck_path,
                execution_dir: static_plan.execution_dir,
                reference_path: contract.prn_reference_path.clone(),
                measurement_reference_paths: Vec::new(),
                continuous_measurement_reference_paths: Vec::new(),
                measurement_tolerance: XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT,
                source: static_plan.source,
                expression_dialect: static_plan.expression_dialect,
                print: static_plan.print,
                dc: static_plan.dc,
                dc_data: static_plan.dc_data,
                steps: static_plan.steps,
                contract: XyceStaticDcContract::SharedStepOracle,
            };
            let mut result = self.run_static_prn_dc_plan(deck, plan, start);
            if !result.passed || result.expected_unsupported {
                result.passed = false;
                result.expected_unsupported = false;
                result.contract = contract_name.to_string();
                result.error = Some(format!(
                    "family '{}' member '{}' failed its direct shared .prn/.res oracle contract: {}",
                    contract.family,
                    self.display_path(&target),
                    result
                        .error
                        .as_deref()
                        .unwrap_or("unknown comparison failure")
                ));
                return result;
            }
        }
        self.passed_result(deck, start, contract_name)
    }

    pub(super) fn run_static_prn_step_dc_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceExecutionPlan,
        netlist: Netlist,
        reference: XycePrnTable,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.contract.result_contract(true);
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let engine = self.create_dc_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };

        if plan.contract.compares_step_res_reference() {
            if let Some(res_reference_path) =
                Self::step_res_reference_path(&plan.deck_path, &plan.reference_path)
            {
                if let Err(err) = self.compare_step_res_reference(
                    &res_reference_path,
                    &netlist,
                    &plan.steps,
                    &step_runs,
                ) {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("Xyce .STEP result summary comparison error: {err}"),
                        Vec::new(),
                    );
                }
            }
        }

        let mut batches = Vec::with_capacity(step_runs.len());
        for run in step_runs {
            let results = match engine.run_dc_sweep2_spec_with_report_and_abort(
                &run.netlist,
                &plan.dc.source,
                &plan.dc.primary_spec(),
                plan.dc.sweep2.as_ref(),
                &abort,
            ) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "simulation exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!("RSpice runtime does not yet support this .STEP deck: {err}"),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("simulation error: {err}"),
                        Vec::new(),
                    );
                }
            };
            batches.push(XyceDcResultBatch {
                netlist: run.netlist,
                results,
            });
        }

        let mismatches = match self.compare_dc_prn_reference_batches(
            &reference,
            &plan.print,
            &plan.source,
            &plan.dc,
            &batches,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("reference comparison error: {err}"),
                    Vec::new(),
                );
            }
        };

        if mismatches.is_empty()
            && matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot)
        {
            let side_mismatches =
                match self.compare_gnuplot_splot_side_output_batches(&plan, &batches) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("GNUPLOT/SPLOT side-output comparison error: {err}"),
                            Vec::new(),
                        );
                    }
                };
            if !side_mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        "{} Xyce GNUPLOT/SPLOT side-output mismatch(es)",
                        side_mismatches.len()
                    ),
                    side_mismatches,
                );
            }
        }

        if mismatches.is_empty() {
            if !matches!(plan.contract, XyceStaticDcContract::WrapperGnuplotSplot) {
                let side_mismatches =
                    match self.compare_prn_compatible_side_output_batches(&plan, &batches) {
                        Ok(mismatches) => mismatches,
                        Err(err) => {
                            return self.failure_result(
                                deck,
                                start,
                                contract,
                                format!("PRN-compatible side-output comparison error: {err}"),
                                Vec::new(),
                            );
                        }
                    };
                if !side_mismatches.is_empty() {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "{} Xyce PRN-compatible side-output mismatch(es)",
                            side_mismatches.len()
                        ),
                        side_mismatches,
                    );
                }
            }
            return self.passed_result(deck, start, contract);
        }

        self.failure_result(
            deck,
            start,
            contract,
            format!("{} Xyce reference mismatch(es)", mismatches.len()),
            mismatches,
        )
    }

    pub(super) fn run_static_step_dc_measurement_plan(
        &self,
        deck: &XyceDeck,
        plan: XyceExecutionPlan,
        netlist: Netlist,
        start: Instant,
    ) -> XyceTestResult {
        let contract = "wrapper_scalar_measure_step_dc";
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let engine = self.create_dc_engine();
        let step_runs = match Self::nested_step_runs_for_commands_with_limits_and_abort(
            &engine,
            &netlist,
            &plan.steps,
            xyce_step_plan_limits(),
            &abort,
        ) {
            Ok(runs) => runs,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(
                        ".STEP expansion exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    "unsupported_xyce_runtime",
                    &format!("RSpice runtime does not yet support this .STEP DC deck: {err}"),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!(".STEP expansion error: {err}"),
                    Vec::new(),
                );
            }
        };
        if step_runs.len() != plan.measurement_reference_paths.len() {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    ".STEP expansion produced {} batches but {} contiguous measurement artifacts exist",
                    step_runs.len(),
                    plan.measurement_reference_paths.len()
                ),
                Vec::new(),
            );
        }

        let mut all_mismatches = Vec::new();
        for (step_index, (run, reference_path)) in step_runs
            .iter()
            .zip(&plan.measurement_reference_paths)
            .enumerate()
        {
            let (measurement_sweep, point_params) = if let Some(dc_data) = &plan.dc_data {
                match self.run_static_dc_data_results(&run.netlist, dc_data, start) {
                    Ok(results) => (
                        results
                            .iter()
                            .map(|row| (row.point.sweep_value, row.point.result.clone()))
                            .collect::<Vec<_>>(),
                        results
                            .iter()
                            .map(|row| row.netlist.params.clone())
                            .collect::<Vec<_>>(),
                    ),
                    Err(SimulationError::Aborted) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "simulation exceeded timeout ({}ms)",
                                self.config.max_time_per_test_ms
                            ),
                            Vec::new(),
                        );
                    }
                    Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                        return self.expected_unsupported_result(
                            deck,
                            start,
                            "unsupported_xyce_runtime",
                            &format!(
                                "RSpice runtime does not yet support DC DATA step {}: {err}",
                                step_index + 1
                            ),
                        );
                    }
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("simulation error in DC DATA step {}: {err}", step_index + 1),
                            Vec::new(),
                        );
                    }
                }
            } else {
                match engine.run_dc_sweep2_spec_with_report_and_abort(
                    &run.netlist,
                    &plan.dc.source,
                    &plan.dc.primary_spec(),
                    plan.dc.sweep2.as_ref(),
                    &abort,
                ) {
                    Ok(results) => (
                        results
                            .iter()
                            .map(|point| (point.sweep_value, point.result.clone()))
                            .collect::<Vec<_>>(),
                        Vec::new(),
                    ),
                    Err(SimulationError::Aborted) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!(
                                "simulation exceeded timeout ({}ms)",
                                self.config.max_time_per_test_ms
                            ),
                            Vec::new(),
                        );
                    }
                    Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                        return self.expected_unsupported_result(
                            deck,
                            start,
                            "unsupported_xyce_runtime",
                            &format!(
                                "RSpice runtime does not yet support DC step {}: {err}",
                                step_index + 1
                            ),
                        );
                    }
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("simulation error in DC step {}: {err}", step_index + 1),
                            Vec::new(),
                        );
                    }
                }
            };
            let measurements = if plan.dc_data.is_some()
                || run
                    .netlist
                    .analyses
                    .iter()
                    .any(|analysis| matches!(analysis, AnalysisCommand::Dc { .. }))
            {
                if point_params.is_empty() {
                    rspice_core::analysis::evaluate_dc_measurements(
                        &run.netlist,
                        &measurement_sweep,
                    )
                } else {
                    rspice_core::analysis::evaluate_dc_measurements_with_parameter_contexts(
                        &run.netlist,
                        &measurement_sweep,
                        &point_params,
                    )
                }
            } else {
                rspice_core::analysis::unevaluated_measurements(
                    &run.netlist,
                    "DC",
                    "DC measurement requires an explicit .DC analysis",
                )
            };
            let continuous = if point_params.is_empty() {
                rspice_core::analysis::evaluate_dc_continuous_measurements(
                    &run.netlist,
                    &measurement_sweep,
                )
            } else {
                rspice_core::analysis::evaluate_dc_continuous_measurements_with_parameter_contexts(
                    &run.netlist,
                    &measurement_sweep,
                    &point_params,
                )
            };
            let mut mismatches = match self.compare_analysis_measurement_outputs(
                std::slice::from_ref(reference_path),
                &[],
                &measurements,
                &continuous,
                plan.measurement_tolerance,
                run.netlist.options.measure_fail_output,
                run.netlist.options.measure_default_value,
                run.netlist.options.measure_use_cont_files(),
                &run.netlist.measurements,
                "DC",
                "DC_CONT",
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!(
                            "DC measurement comparison error in step {}: {err}",
                            step_index + 1
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut mismatches {
                mismatch.probe = format!("step[{}]:{}", step_index + 1, mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if all_mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce stepped DC measurement mismatch(es)",
                    all_mismatches.len()
                ),
                all_mismatches,
            )
        }
    }

    pub(super) fn run_passive_primary_value_composite_contract(
        &self,
        deck: &XyceDeck,
        contract: XycePassivePrimaryCompositeContract,
        start: Instant,
    ) -> XyceTestResult {
        const COMPOSITE_CONTRACT: &str = "passive_primary_value_composite_wrapper";

        if let Some(target_path) = contract.target_path.as_ref() {
            let pair = if contract
                .capacitor_tran
                .member_paths
                .iter()
                .any(|path| Self::same_path(path, target_path))
            {
                contract.capacitor_tran
            } else if contract
                .resistor_dc
                .member_paths
                .iter()
                .any(|path| Self::same_path(path, target_path))
            {
                contract.resistor_dc
            } else {
                return self.failure_result(
                    deck,
                    start,
                    COMPOSITE_CONTRACT,
                    format!(
                        "passive primary-value family '{}' target {} is not one of its four qualified members",
                        contract.family,
                        self.display_path(target_path)
                    ),
                    Vec::new(),
                );
            };
            return self.run_baseline_family_contract(deck, pair, start);
        }

        if !Self::same_path(&deck.path, &contract.owner_path) {
            return self.failure_result(
                deck,
                start,
                COMPOSITE_CONTRACT,
                format!(
                    "passive primary-value family '{}' owner routing does not match {}",
                    contract.family,
                    self.display_path(&contract.owner_path)
                ),
                Vec::new(),
            );
        }

        for pair in [contract.capacitor_tran, contract.resistor_dc] {
            let result = self.run_baseline_family_contract(deck, pair, start);
            if result.expected_unsupported {
                return self.failure_result(
                    deck,
                    start,
                    COMPOSITE_CONTRACT,
                    format!(
                        "passive primary-value composite family '{}' did not execute a required pair: {}",
                        contract.family,
                        result
                            .error
                            .as_deref()
                            .unwrap_or("pair returned expected-unsupported without a reason")
                    ),
                    result.mismatches,
                );
            }
            if !result.passed {
                return XyceTestResult {
                    contract: COMPOSITE_CONTRACT.to_string(),
                    error: result.error.map(|error| {
                        format!(
                            "passive primary-value composite family '{}' failed: {error}",
                            contract.family
                        )
                    }),
                    ..result
                };
            }
        }

        self.passed_result(deck, start, COMPOSITE_CONTRACT)
    }

    pub(super) fn run_analytic_integer_dc_wrapper_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAnalyticIntegerDcContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.kind.result_contract();
        let (netlist, results) = match self.run_static_dc_results(&contract.plan, start) {
            Ok(run) => run,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "analytic integer DC execution exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("analytic integer DC execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let actual = match self.dc_results_to_prn_table(&contract.plan, &netlist, &results) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("analytic integer DC output conversion failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_analytic_integer_dc_table(&actual, contract.kind) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("analytic integer DC exact comparison failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} analytic integer DC mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_resistor_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceResistorDtempContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        // Preserve the upstream wrapper's execution order: res_ref.cir is the
        // independently simulated good data and res_dtemp.cir is the test.
        let reference = match self.simulate_resistor_dtemp_step_plan(
            &contract.reference_plan,
            start,
            "reference",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("resistor DTEMP reference execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let owner = match self.simulate_resistor_dtemp_step_plan(
            &contract.owner_plan,
            start,
            "DTEMP owner",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("resistor DTEMP owner execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_resistor_dtemp_tables(&reference, &owner) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("resistor DTEMP relational comparison failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "{} resistor DTEMP relational mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_bug647_resistor_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug647ResistorContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        let tables = self.simulate_bug647_resistor_pair(
            &contract.owner_plan,
            &contract.reference_plan,
            start,
        );
        let (owner, reference) = match tables {
            Ok(tables) => tables,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 647 resistor paired execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_bug647_resistor_tables(&reference, &owner) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 647 resistor file_compare adapter failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG 647 resistor mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_bug655_continuation_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug655ContinuationContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        let owner = match self.simulate_bug655_continuation_member(
            &contract.owner_plan,
            start,
            "column-zero continuation owner",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 655 owner execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let reference = match self.simulate_bug655_continuation_member(
            &contract.reference_plan,
            start,
            "leading-space continuation reference",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 655 spaced-reference execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_bug655_continuation_tables(&owner, &reference) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 655 Release 7.10 xyce_verify adapter failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG 655 DC mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_bug662_long_header_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug662HeaderContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        // Preserve the removed wrapper's order and oracle direction: the
        // short-header sibling is independently simulated as good data before
        // the long-header owner is independently simulated as test data.
        let reference = match self.simulate_bug662_header_member(
            &contract.reference_plan,
            start,
            "short-header reference",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 662 short-header execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let owner = match self.simulate_bug662_header_member(
            &contract.owner_plan,
            start,
            "long-header owner",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 662 long-header execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_bug662_header_tables(&reference, &owner) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 662 Release 7.10 xyce_verify adapter failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG 662 transient mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_bug667_nodeset_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug667NodesetContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        // Preserve the removed wrapper's execution order. Each sibling is
        // parsed and simulated independently, including its own adaptive
        // transient grid, before reproducing the wrapper's raw PRN diff.
        let owner = match self.simulate_bug667_nodeset_member(
            &contract.owner_plan,
            start,
            "subcircuit-scoped owner",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 667 scoped owner execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let reference = match self.simulate_bug667_nodeset_member(
            &contract.reference_plan,
            start,
            "explicit hierarchical reference",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 667 explicit reference execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_bug667_nodeset_tables(&owner, &reference) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 667 raw default-PRN diff adapter failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG 667 exact PRN mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_bug754_global_parameter_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug754GlobalParameterContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        // Preserve the removed wrapper's execution order: run the
        // global-parameter owner first, run the literal reference second, then
        // apply bytewise `diff` with dcsweep_nopar.cir as the good file.
        let owner = match self.simulate_bug754_global_parameter_member(
            &contract.owner_plan,
            start,
            "global-parameter owner",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 754 global-parameter execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let reference = match self.simulate_bug754_global_parameter_member(
            &contract.reference_plan,
            start,
            "literal reference",
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 754 literal-reference execution failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_bug754_global_parameter_tables(&reference, &owner) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 754 bytewise default-PRN adapter failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!("{} BUG 754 exact PRN mismatch(es)", mismatches.len()),
                mismatches,
            )
        }
    }

    pub(super) fn run_analytic_rc_wrapper_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAnalyticRcContract,
        start: Instant,
    ) -> XyceTestResult {
        const RESULT_CONTRACT: &str = "analytic_first_order_rc_tran_wrapper";
        let (netlist, result) =
            match self.run_transient_family_plan(&contract.plan, start, None, None) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!(
                            "analytic first-order RC execution exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!("analytic first-order RC execution failed: {err}"),
                        Vec::new(),
                    );
                }
            };
        let actual =
            match Self::transient_family_result_to_prn_table(&contract.plan, &netlist, &result) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!("analytic first-order RC output conversion failed: {err}"),
                        Vec::new(),
                    );
                }
            };
        if let Err(err) =
            Self::validate_analytic_rc_initial_sample(&actual, &contract.specification)
        {
            return self.failure_result(
                deck,
                start,
                RESULT_CONTRACT,
                format!("analytic first-order RC initial-condition validation failed: {err}"),
                Vec::new(),
            );
        }
        if let Err(err) =
            Self::validate_analytic_rc_complete_time_domain(&actual, contract.plan.tran.stop)
        {
            return self.failure_result(
                deck,
                start,
                RESULT_CONTRACT,
                format!("analytic first-order RC output-domain validation failed: {err}"),
                Vec::new(),
            );
        }
        let reference = match Self::analytic_rc_reference_table(&actual, &contract.specification) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    RESULT_CONTRACT,
                    format!("analytic first-order RC reference generation failed: {err}"),
                    Vec::new(),
                );
            }
        };
        let mismatches = match self.compare_xyce_verify_transient_tables(&reference, &actual) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    RESULT_CONTRACT,
                    format!("analytic first-order RC xyce_verify comparison failed: {err}"),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, RESULT_CONTRACT)
        } else {
            self.failure_result(
                deck,
                start,
                RESULT_CONTRACT,
                format!(
                    "{} analytic first-order RC xyce_verify mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_analytic_sinusoidal_rc_wrapper_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAnalyticSinusoidalRcContract,
        start: Instant,
    ) -> XyceTestResult {
        const RESULT_CONTRACT: &str = "analytic_sinusoidal_first_order_rc_tran_wrapper";
        let (netlist, result) =
            match self.run_transient_family_plan(&contract.plan, start, None, None) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!(
                            "analytic sinusoidal first-order RC execution exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!("analytic sinusoidal first-order RC execution failed: {err}"),
                        Vec::new(),
                    );
                }
            };
        let actual =
            match Self::transient_family_result_to_prn_table(&contract.plan, &netlist, &result) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!(
                            "analytic sinusoidal first-order RC output conversion failed: {err}"
                        ),
                        Vec::new(),
                    );
                }
            };
        if let Err(err) = Self::validate_analytic_sinusoidal_rc_output_domain(
            &actual,
            &contract.specification,
            contract.plan.tran.stop,
            contract.tolerance,
        ) {
            return self.failure_result(
                deck,
                start,
                RESULT_CONTRACT,
                format!(
                    "analytic sinusoidal first-order RC output-domain validation failed: {err}"
                ),
                Vec::new(),
            );
        }
        let reference =
            match Self::analytic_sinusoidal_rc_reference_table(&actual, &contract.specification) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!(
                            "analytic sinusoidal first-order RC reference generation failed: {err}"
                        ),
                        Vec::new(),
                    );
                }
            };
        let mismatches = match self.compare_xyce_verify_transient_tables_with_tolerance(
            &reference,
            &actual,
            contract.tolerance,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    RESULT_CONTRACT,
                    format!(
                        "analytic sinusoidal first-order RC xyce_verify comparison failed: {err}"
                    ),
                    Vec::new(),
                );
            }
        };
        if mismatches.is_empty() {
            self.passed_result(deck, start, RESULT_CONTRACT)
        } else {
            self.failure_result(
                deck,
                start,
                RESULT_CONTRACT,
                format!(
                    "{} analytic sinusoidal first-order RC xyce_verify mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_stepped_ic_reference_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceSteppedIcReferenceContract,
        start: Instant,
    ) -> XyceTestResult {
        const WRAPPER_CONTRACT: &str = "stepped_ic_reference_wrapper";
        const BASELINE_CONTRACT: &str = "stepped_ic_reference_baseline";

        let result_contract = if Self::same_path(&contract.target_path, &contract.owner_path) {
            WRAPPER_CONTRACT
        } else {
            BASELINE_CONTRACT
        };
        let expansion_abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            let owner_plan = self.static_tran_family_plan_for_path(
                &contract.owner_path,
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            )?;
            if owner_plan.output_override || owner_plan.timeint_conststep {
                return Err(
                    "stepped-IC generated-reference wrapper requires ordinary default transient output"
                        .to_string(),
                );
            }
            let [step] = owner_plan.steps.as_slice() else {
                return Err(format!(
                    "stepped-IC generated-reference wrapper requires exactly one .STEP command, found {}",
                    owner_plan.steps.len()
                ));
            };
            if step.target != StepTarget::Device
                || step
                    .param_name
                    .as_deref()
                    .is_none_or(|name| !name.eq_ignore_ascii_case("c"))
                || !matches!(
                    step.sweep,
                    StepSweep::Decade {
                        points_per_decade: 1..,
                        start,
                        stop,
                    } if start.is_finite() && stop.is_finite() && start > 0.0 && stop >= start
                )
            {
                return Err(
                    "stepped-IC generated-reference wrapper requires a finite positive DEC device-parameter sweep of capacitor parameter C"
                        .to_string(),
                );
            }
            let owner_options = Self::stepped_ic_option_signature(&owner_plan.source)?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_plan.deck_path)
                .map_err(|err| format!("owner netlist parse failed: {err}"))?;
            let step_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &self.create_xyce_engine(),
                &owner_netlist,
                &owner_plan.steps,
                xyce_step_plan_limits(),
                &expansion_abort,
            )
            .map_err(|err| match err {
                SimulationError::Aborted => format!(
                    "owner .STEP expansion exceeded timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                error => format!("owner .STEP expansion failed: {error}"),
            })?;
            if step_runs.len() != contract.member_paths.len() {
                return Err(format!(
                    "owner .STEP expands to {} run(s), but the family contains {} independent baseline deck(s)",
                    step_runs.len(),
                    contract.member_paths.len()
                ));
            }

            let mut members = Vec::with_capacity(contract.member_paths.len());
            for (index, (member_path, step_run)) in contract
                .member_paths
                .iter()
                .zip(step_runs.iter())
                .enumerate()
            {
                let member_plan = self.static_tran_family_plan_for_path(
                    member_path,
                    XyceStaticTranPlanPurpose::RelationalFamily,
                )?;
                if !member_plan.steps.is_empty()
                    || member_plan.output_override
                    || member_plan.timeint_conststep
                {
                    return Err(format!(
                        "independent baseline {} must use one ordinary non-stepped transient output",
                        self.display_path(member_path)
                    ));
                }
                if member_plan.print.probes != owner_plan.print.probes {
                    return Err(format!(
                        "independent baseline {} changes the ordered .PRINT TRAN probes",
                        self.display_path(member_path)
                    ));
                }
                if !Self::tran_analyses_match_exactly(&owner_plan.tran, &member_plan.tran) {
                    return Err(format!(
                        "independent baseline {} changes the .TRAN analysis tuple",
                        self.display_path(member_path)
                    ));
                }
                let member_options = Self::stepped_ic_option_signature(&member_plan.source)?;
                if member_options != owner_options {
                    return Err(format!(
                        "independent baseline {} changes the .OPTIONS contract",
                        self.display_path(member_path)
                    ));
                }
                let owner_scale = Self::tran_print_time_scale_factor(&owner_plan.source)?;
                let member_scale = Self::tran_print_time_scale_factor(&member_plan.source)?;
                if owner_scale.to_bits() != member_scale.to_bits() {
                    return Err(format!(
                        "independent baseline {} changes transient output time units",
                        self.display_path(member_path)
                    ));
                }
                let member_netlist =
                    Self::parse_xyce_netlist(&member_plan.source, &member_plan.deck_path).map_err(
                        |err| {
                            format!(
                                "independent baseline {} parse failed: {err}",
                                self.display_path(member_path)
                            )
                        },
                    )?;
                let stepped_snapshot = Self::stepped_ic_snapshot(&step_run.netlist)?;
                let member_snapshot = Self::stepped_ic_snapshot(&member_netlist)?;
                if !step
                    .name
                    .eq_ignore_ascii_case(&stepped_snapshot.capacitor_name)
                {
                    return Err(format!(
                        "owner step targets '{}', but the materialized circuit's qualified capacitor is '{}'",
                        step.name, stepped_snapshot.capacitor_name
                    ));
                }
                if !Self::stepped_ic_snapshots_match(&stepped_snapshot, &member_snapshot) {
                    return Err(format!(
                        "owner step {} is not structurally and numerically identical to independent baseline {}",
                        index,
                        self.display_path(member_path)
                    ));
                }
                let step_value = step_run.step_values.first().copied().ok_or_else(|| {
                    format!(
                        "owner step {index} did not retain its swept {}:C value",
                        step.name
                    )
                })?;
                let member_capacitance = Value::from_bits(member_snapshot.capacitor_value_bits);
                if step_run.step_values.len() != 1
                    || (step_value - member_capacitance).abs() > 1.0e-12
                {
                    return Err(format!(
                        "owner step {index} {}:C value {step_value} does not match independent baseline {} within the Release 7.10 sweep-result tolerance",
                        step.name,
                        self.display_path(member_path)
                    ));
                }
                if !step
                    .name
                    .eq_ignore_ascii_case(&member_snapshot.capacitor_name)
                {
                    return Err(format!(
                        "owner step targets '{}', but independent baseline {} names its qualified capacitor '{}'",
                        step.name,
                        self.display_path(member_path),
                        member_snapshot.capacitor_name
                    ));
                }
                members.push((member_plan, member_netlist));
            }
            Ok((owner_plan, step_runs, members))
        })();

        let (owner_plan, step_runs, members) = match qualification {
            Ok(qualified) => qualified,
            Err(reason) if reason.starts_with("owner .STEP expansion exceeded timeout") => {
                return self.failure_result(deck, start, result_contract, reason, Vec::new());
            }
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "stepped-IC generated-reference family '{}' qualification failed: {reason}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (index, ((member_plan, member_netlist), step_run)) in
            members.iter().zip(step_runs.iter()).enumerate()
        {
            let member_result = match self.run_transient_family_netlist(
                member_plan,
                member_netlist,
                start,
                None,
                None,
            ) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "independent baseline step {index} exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support stepped-IC baseline step {index}: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("independent baseline step {index} simulation failed: {err}"),
                        Vec::new(),
                    );
                }
            };
            let stepped_result = match self.run_transient_family_netlist(
                &owner_plan,
                &step_run.netlist,
                start,
                None,
                None,
            ) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "owner step {index} exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        "unsupported_xyce_runtime",
                        &format!(
                            "RSpice runtime does not yet support stepped-IC owner step {index}: {err}"
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("owner step {index} simulation failed: {err}"),
                        Vec::new(),
                    );
                }
            };
            let baseline_table = match Self::transient_family_result_to_prn_table(
                member_plan,
                member_netlist,
                &member_result,
            ) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("independent baseline step {index} output failed: {err}"),
                        Vec::new(),
                    );
                }
            };
            let stepped_table = match Self::transient_family_result_to_prn_table(
                &owner_plan,
                &step_run.netlist,
                &stepped_result,
            ) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("owner step {index} output failed: {err}"),
                        Vec::new(),
                    );
                }
            };
            let mut step_mismatches =
                match self.compare_xyce_verify_transient_tables(&baseline_table, &stepped_table) {
                    Ok(found) => found,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            result_contract,
                            format!(
                                "Release 7.10 xyce_verify comparison failed for step {index}: {err}"
                            ),
                            Vec::new(),
                        );
                    }
                };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {index}: {}", mismatch.probe);
            }
            row_offset += stepped_table.rows.len();
            mismatches.extend(step_mismatches);
            if mismatches.len() >= self.config.max_mismatches {
                mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if mismatches.is_empty() {
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "{} stepped-IC Release 7.10 xyce_verify mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_subckt_parameter_resolution_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceSubcktParameterResolutionFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        const MEMBER_CONTRACT: &str = "subckt_parameter_resolution_family_member";
        const ERROR_CONTRACT: &str = "subckt_parameter_resolution_expected_error";
        if !contract
            .valid_paths
            .iter()
            .any(|path| Self::same_path(path, &contract.baseline_path))
            || !contract.anchor_path.is_file()
            || !contract.error_path.is_file()
        {
            return self.failure_result(
                deck,
                start,
                XyceBaselineFamilyKind::SubcktParameterResolution.wrapper_contract(),
                format!(
                    "subcircuit-parameter resolution family '{}' lost a qualified anchor, error member, or baseline",
                    contract.family
                ),
                Vec::new(),
            );
        }
        let target_path = match contract.role {
            XyceSubcktParameterResolutionRole::Baseline
            | XyceSubcktParameterResolutionRole::Member => Some(contract.target_path.clone()),
            XyceSubcktParameterResolutionRole::Anchor
            | XyceSubcktParameterResolutionRole::ExpectedError => None,
        };
        let relational = XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::SubcktParameterResolution,
            comparison: XyceBaselineFamilyComparison::ExactPrn,
            family: contract.family,
            baseline_path: contract.baseline_path,
            member_paths: contract.valid_paths,
            target_path,
        };
        let mut result = self.run_baseline_family_contract(deck, relational, start);
        if result.passed && !result.expected_unsupported {
            result.contract = match contract.role {
                XyceSubcktParameterResolutionRole::Anchor => {
                    XyceBaselineFamilyKind::SubcktParameterResolution.wrapper_contract()
                }
                XyceSubcktParameterResolutionRole::Baseline => {
                    XyceBaselineFamilyKind::SubcktParameterResolution.baseline_contract()
                }
                XyceSubcktParameterResolutionRole::Member => MEMBER_CONTRACT,
                XyceSubcktParameterResolutionRole::ExpectedError => ERROR_CONTRACT,
            }
            .to_string();
        }
        result
    }

    pub(super) fn run_age_cap_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAgeCapFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let mut result = self.run_baseline_family_contract(deck, contract.relational, start);
        if result.passed && !result.expected_unsupported {
            result.contract = match contract.role {
                XyceAgeCapFamilyRole::Anchor => "age_cap_family_anchor",
                XyceAgeCapFamilyRole::AgedBaseline => "age_cap_family_aged_baseline",
                XyceAgeCapFamilyRole::EquivalentMember => "age_cap_family_equivalent_member",
            }
            .to_string();
        }
        result
    }

    pub(super) fn run_switch_state_case_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceSwitchStateCaseFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let mut result = self.run_baseline_family_contract(deck, contract.relational, start);
        if result.passed && !result.expected_unsupported {
            result.contract = match contract.role {
                XyceSwitchStateCaseFamilyRole::Anchor => "switch_state_case_family_anchor",
                XyceSwitchStateCaseFamilyRole::UppercaseBaseline => {
                    "switch_state_case_family_uppercase_baseline"
                }
                XyceSwitchStateCaseFamilyRole::LowercaseMember => {
                    "switch_state_case_family_lowercase_member"
                }
            }
            .to_string();
        }
        result
    }

    pub(super) fn run_diode_model_alias_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceDiodeModelAliasFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let mut result = self.run_baseline_family_contract(deck, contract.relational, start);
        if result.passed && !result.expected_unsupported {
            result.contract = match contract.role {
                XyceDiodeModelAliasFamilyRole::Anchor => "diode_model_alias_family_anchor",
                XyceDiodeModelAliasFamilyRole::CanonicalBaseline => {
                    "diode_model_alias_family_canonical_baseline"
                }
                XyceDiodeModelAliasFamilyRole::AliasMember => {
                    "diode_model_alias_family_alias_member"
                }
            }
            .to_string();
        }
        result
    }

    pub(super) fn run_nested_include_identity_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceNestedIncludeIdentityFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let mut result = self.run_baseline_family_contract(deck, contract.relational, start);
        if result.passed && !result.expected_unsupported {
            result.contract = match contract.role {
                XyceNestedIncludeIdentityFamilyRole::Anchor => {
                    "nested_include_identity_family_anchor"
                }
                XyceNestedIncludeIdentityFamilyRole::RepeatedTargetBaseline => {
                    "nested_include_identity_family_repeated_target_baseline"
                }
                XyceNestedIncludeIdentityFamilyRole::SplitIdenticalTargetsMember => {
                    "nested_include_identity_family_split_targets_member"
                }
            }
            .to_string();
        }
        result
    }

    pub(super) fn run_baseline_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBaselineFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let kind_name = contract.kind.name();
        let wrapper_contract = contract.kind.wrapper_contract();
        let baseline_contract = contract.kind.baseline_contract();
        let analysis = match self.baseline_family_analysis_for_path(&contract.baseline_path) {
            Ok(analysis) => analysis,
            Err(reason) => {
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    format!(
                        "{kind_name} family '{}' baseline analysis is ambiguous or unsupported: {reason}",
                        contract.family
                    ),
                );
            }
        };
        if contract.kind == XyceBaselineFamilyKind::AcAnalysisExpression
            && analysis != XyceBaselineFamilyAnalysis::Ac
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' requires an AC analysis",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if matches!(
            contract.kind,
            XyceBaselineFamilyKind::BjtExternalNode
                | XyceBaselineFamilyKind::DcAnalysisExpression
                | XyceBaselineFamilyKind::DelimitedExpression
                | XyceBaselineFamilyKind::PassiveResPrimaryValue
                | XyceBaselineFamilyKind::SubcktParameterResolution
                | XyceBaselineFamilyKind::NestedIncludeIdentity
        ) && analysis != XyceBaselineFamilyAnalysis::Dc
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' requires a DC analysis",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if matches!(
            contract.kind,
            XyceBaselineFamilyKind::AgeCap
                | XyceBaselineFamilyKind::DiodeModelAlias
                | XyceBaselineFamilyKind::SwitchStateCase
                | XyceBaselineFamilyKind::SinExpression
                | XyceBaselineFamilyKind::ParamExpression
                | XyceBaselineFamilyKind::PassiveCapPrimaryValue
                | XyceBaselineFamilyKind::PassiveTemperatureOverride
                | XyceBaselineFamilyKind::TransientAnalysisExpression
        ) && analysis != XyceBaselineFamilyAnalysis::Tran
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' requires a transient analysis",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if analysis == XyceBaselineFamilyAnalysis::Ac {
            return self.run_ac_baseline_family_contract(deck, contract, start);
        }
        if analysis == XyceBaselineFamilyAnalysis::Tran {
            let baseline_plan = match self.static_tran_family_plan_for_path(
                &contract.baseline_path,
                self.transient_family_plan_purpose_for_path(contract.kind, &contract.baseline_path),
            ) {
                Ok(plan) => plan,
                Err(reason) => {
                    return self.baseline_family_qualification_result(
                        deck,
                        start,
                        wrapper_contract,
                        contract.comparison,
                        format!(
                            "{kind_name} family '{}' baseline is not supported by the static TRAN adapter: {reason}",
                            contract.family
                        ),
                    );
                }
            };
            return self.run_transient_baseline_family_contract(
                deck,
                contract,
                baseline_plan,
                start,
            );
        }
        let baseline_plan = match self
            .static_dc_plan_for_path(&contract.baseline_path, ExpressionDialect::Xyce)
        {
            Ok(plan) => plan,
            Err(reason) => {
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    format!(
                        "{kind_name} family '{}' baseline is not supported by the static DC adapter: {reason}",
                        contract.family
                    ),
                );
            }
        };
        if contract.comparison.compares_waveforms_exactly() {
            return self.run_exact_dc_baseline_family_contract(
                deck,
                contract,
                baseline_plan,
                start,
            );
        }
        let baseline_run = self.run_static_dc_results(&baseline_plan, start);
        let (baseline_netlist, baseline_results) = match baseline_run {
            Ok(results) => results,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline exceeded timeout ({}ms)",
                        contract.family, self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.expected_unsupported_result(
                    deck,
                    start,
                    wrapper_contract,
                    &format!(
                        "{kind_name} family '{}' baseline is not supported by RSpice yet: {err}",
                        contract.family
                    ),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline error: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        if contract.kind.compares_baseline_oracle()
            && let Some(reference_path) = self.static_prn_reference_path(&contract.baseline_path)
            && reference_path.is_file()
        {
            let reference = match Self::parse_prn_file(&reference_path) {
                Ok(reference) => reference,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline oracle parse error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
            let mismatches = match self.compare_dc_prn_reference(
                &reference,
                &baseline_plan.print,
                &baseline_netlist,
                &baseline_plan.source,
                &baseline_plan.dc,
                &baseline_results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline oracle comparison error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
            if !mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{} {kind_name} family '{}' baseline oracle mismatch(es)",
                        mismatches.len(),
                        contract.family
                    ),
                    mismatches,
                );
            }
        }
        let baseline_table = match self.dc_results_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_results,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline output conversion failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let (targets, baseline_record) = Self::baseline_family_targets(&contract);
        if targets.is_empty() {
            return self.expected_unsupported_result(
                deck,
                start,
                wrapper_contract,
                &format!(
                    "{kind_name} family '{}' has no non-baseline member to compare",
                    contract.family
                ),
            );
        }

        let mut all_mismatches = Vec::new();
        for target_path in targets {
            let target_plan = match self
                .static_dc_plan_for_path(&target_path, ExpressionDialect::Xyce)
            {
                Ok(plan) => plan,
                Err(reason) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        wrapper_contract,
                        &format!(
                            "{kind_name} family '{}' member {} is not supported by the static DC adapter: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
            };
            let (target_netlist, target_results) = match self
                .run_static_dc_results(&target_plan, start)
            {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} exceeded timeout ({}ms)",
                            contract.family,
                            self.display_path(&target_path),
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.expected_unsupported_result(
                        deck,
                        start,
                        wrapper_contract,
                        &format!(
                            "{kind_name} family '{}' member {} is not supported by RSpice yet: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };

            let mut mismatches = match self.compare_dc_prn_reference(
                &baseline_table,
                &target_plan.print,
                &target_netlist,
                &baseline_plan.source,
                &target_plan.dc,
                &target_results,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} comparison error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{} {}", self.display_path(&target_path), mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if all_mismatches.is_empty() {
            let result_contract = if baseline_record {
                baseline_contract
            } else {
                wrapper_contract
            };
            self.passed_result(deck, start, result_contract)
        } else {
            self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{} {kind_name} family '{}' mismatch(es)",
                    all_mismatches.len(),
                    contract.family
                ),
                all_mismatches,
            )
        }
    }

    pub(super) fn run_ac_baseline_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBaselineFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let kind_name = contract.kind.name();
        let wrapper_contract = contract.kind.wrapper_contract();
        let baseline_contract = contract.kind.baseline_contract();
        let Some(tolerance) = contract.comparison.ac_comparator_tolerance() else {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' has no ACComparator tolerance contract",
                    contract.family
                ),
                Vec::new(),
            );
        };
        let baseline_plan = match self.relational_ac_plan_for_path(&contract.baseline_path) {
            Ok(plan) => plan,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline is not supported by the relational AC adapter: {reason}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        if let Err(reason) = Self::validate_ac_analysis_expression_plan(&baseline_plan) {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline AC qualification failed: {reason}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        let baseline_netlist =
            match Self::parse_xyce_netlist(&baseline_plan.source, &baseline_plan.deck_path) {
                Ok(netlist) => netlist,
                Err(err) => {
                    return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline parse failed after qualification: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
                }
            };
        let baseline_snapshot =
            match Self::strict_ac_family_snapshot(contract.kind, &baseline_netlist) {
                Ok(snapshot) => snapshot,
                Err(reason) => {
                    return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline semantic qualification failed: {reason}",
                        contract.family
                    ),
                    Vec::new(),
                );
                }
            };
        let engine = self.create_xyce_engine();
        let baseline_results = match engine.run_ac(&baseline_netlist, &baseline_plan.ac.frequencies)
        {
            Ok(results) => results,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline error: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        let baseline_table = match Self::ac_family_result_to_prn_table(
            &baseline_plan.print,
            &baseline_netlist,
            &baseline_results,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline output conversion failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let (targets, baseline_record) = Self::baseline_family_targets(&contract);
        if targets.is_empty() {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' has no non-baseline member to compare",
                    contract.family
                ),
                Vec::new(),
            );
        }
        let mut all_mismatches = Vec::new();
        for target_path in targets {
            let target_plan = match self.relational_ac_plan_for_path(&target_path) {
                Ok(plan) => plan,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} is not supported by the relational AC adapter: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            if let Err(reason) = Self::validate_ac_analysis_expression_plan(&target_plan) {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} AC qualification failed: {reason}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if target_plan.print.probes != baseline_plan.print.probes
                || target_plan.ac.frequencies.len() != baseline_plan.ac.frequencies.len()
                || !target_plan
                    .ac
                    .frequencies
                    .iter()
                    .zip(&baseline_plan.ac.frequencies)
                    .all(|(target, baseline)| target.to_bits() == baseline.to_bits())
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} changes the ordered .PRINT AC probes or resolved frequency grid",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            let target_netlist = match Self::parse_xyce_netlist(
                &target_plan.source,
                &target_plan.deck_path,
            ) {
                Ok(netlist) => netlist,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} parse failed after qualification: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            if !Self::ac_analyses_match_exactly(&baseline_netlist, &target_netlist) {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} changes the .AC analysis tuple",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            let target_snapshot = match Self::strict_ac_family_snapshot(
                contract.kind,
                &target_netlist,
            ) {
                Ok(snapshot) => snapshot,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} semantic qualification failed: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            if let Err(reason) =
                Self::compare_strict_ac_family_snapshots(&baseline_snapshot, &target_snapshot)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} changes semantics outside its qualified representation pair: {reason}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            let target_results = match engine.run_ac(&target_netlist, &target_plan.ac.frequencies) {
                Ok(results) => results,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            let target_table = match Self::ac_family_result_to_prn_table(
                &target_plan.print,
                &target_netlist,
                &target_results,
            ) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} output conversion failed: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            let mut mismatches = match self.compare_ac_comparator_tables_with_tolerance(
                &baseline_table,
                &target_table,
                tolerance,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} ACComparator error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut mismatches {
                mismatch.probe = format!("{} {}", self.display_path(&target_path), mismatch.probe);
            }
            all_mismatches.extend(mismatches);
            if all_mismatches.len() >= self.config.max_mismatches {
                all_mismatches.truncate(self.config.max_mismatches);
                break;
            }
        }

        if all_mismatches.is_empty() {
            self.passed_result(
                deck,
                start,
                if baseline_record {
                    baseline_contract
                } else {
                    wrapper_contract
                },
            )
        } else {
            self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{} {kind_name} family '{}' ACComparator mismatch(es)",
                    all_mismatches.len(),
                    contract.family
                ),
                all_mismatches,
            )
        }
    }

    pub(super) fn run_exact_dc_baseline_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBaselineFamilyContract,
        baseline_plan: XyceStaticDcPlan,
        start: Instant,
    ) -> XyceTestResult {
        let kind_name = contract.kind.name();
        let wrapper_contract = contract.kind.wrapper_contract();
        let baseline_contract = contract.kind.baseline_contract();
        if let Err(reason) = Self::validate_exact_dc_family_plan(contract.kind, &baseline_plan) {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline exact-DC qualification failed: {reason}",
                    contract.family
                ),
                Vec::new(),
            );
        }

        let (baseline_netlist, baseline_results) =
            match self.run_static_dc_results(&baseline_plan, start) {
                Ok(results) => results,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline exceeded timeout ({}ms)",
                            contract.family, self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
        let baseline_snapshot =
            match Self::strict_dc_family_snapshot(contract.kind, &baseline_netlist, &baseline_plan)
            {
                Ok(snapshot) => snapshot,
                Err(reason) => {
                    return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline semantic qualification failed: {reason}",
                        contract.family
                    ),
                    Vec::new(),
                );
                }
            };
        let baseline_table = match self.dc_results_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_results,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline exact output conversion failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        let baseline_sweep = baseline_results
            .iter()
            .map(|point| point.sweep_value)
            .collect::<Vec<_>>();

        let (targets, baseline_record) = Self::baseline_family_targets(&contract);
        if targets.is_empty() {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' has no non-baseline member to compare",
                    contract.family
                ),
                Vec::new(),
            );
        }

        for target_path in targets {
            let target_plan = match self
                .static_dc_plan_for_path(&target_path, ExpressionDialect::Xyce)
            {
                Ok(plan) => plan,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} is not supported by the static DC adapter: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            if let Err(reason) = Self::validate_exact_dc_family_plan(contract.kind, &target_plan) {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} exact-DC qualification failed: {reason}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if target_plan.print.probes != baseline_plan.print.probes {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' exact member {} changes ordered .PRINT DC probes",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if !Self::dc_sweeps_match_exactly(&baseline_plan.dc, &target_plan.dc) {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' exact member {} changes the .DC analysis tuple",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }

            let (target_netlist, target_results) =
                match self.run_static_dc_results(&target_plan, start) {
                    Ok(results) => results,
                    Err(SimulationError::Aborted) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} exceeded timeout ({}ms)",
                                contract.family,
                                self.display_path(&target_path),
                                self.config.max_time_per_test_ms
                            ),
                            Vec::new(),
                        );
                    }
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} error: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
            let target_snapshot = match Self::strict_dc_family_snapshot(
                contract.kind,
                &target_netlist,
                &target_plan,
            ) {
                Ok(snapshot) => snapshot,
                Err(reason) => {
                    return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} semantic qualification failed: {reason}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                }
            };
            if let Err(reason) =
                Self::compare_strict_dc_family_snapshots(&baseline_snapshot, &target_snapshot)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} changes semantics outside its qualified representation pair: {reason}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }

            let target_table = match self.dc_results_to_prn_table(
                &target_plan,
                &target_netlist,
                &target_results,
            ) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} exact output conversion failed: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            let target_sweep = target_results
                .iter()
                .map(|point| point.sweep_value)
                .collect::<Vec<_>>();
            let comparison = if contract.comparison.compares_prn_case_insensitively() {
                self.compare_serialized_default_prn_tables_case_insensitive(
                    &baseline_table,
                    &target_table,
                )
            } else if contract.comparison.compares_serialized_prn_exactly() {
                self.compare_serialized_default_prn_tables(&baseline_table, &target_table)
            } else {
                self.compare_exact_dc_prn_tables(
                    &baseline_table,
                    &target_table,
                    &baseline_sweep,
                    &target_sweep,
                )
            };
            let mut mismatches = match comparison {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} exact comparison error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            if !mismatches.is_empty() {
                for mismatch in &mut mismatches {
                    mismatch.probe =
                        format!("{} {}", self.display_path(&target_path), mismatch.probe);
                }
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{} {kind_name} family '{}' member {} exact DC mismatch(es)",
                        mismatches.len(),
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    mismatches,
                );
            }
        }

        let result_contract = if baseline_record {
            baseline_contract
        } else {
            wrapper_contract
        };
        self.passed_result(deck, start, result_contract)
    }

    pub(super) fn run_transient_baseline_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBaselineFamilyContract,
        baseline_plan: XyceStaticTranPlan,
        start: Instant,
    ) -> XyceTestResult {
        let kind_name = contract.kind.name();
        let wrapper_contract = contract.kind.wrapper_contract();
        let baseline_contract = contract.kind.baseline_contract();
        if !baseline_plan.steps.is_empty() {
            return self.baseline_family_qualification_result(
                deck,
                start,
                wrapper_contract,
                contract.comparison,
                format!(
                    "{kind_name} family '{}' transient relational contract does not yet support .STEP output",
                    contract.family
                ),
            );
        }
        if baseline_plan.output_override {
            return self.baseline_family_qualification_result(
                deck,
                start,
                wrapper_contract,
                contract.comparison,
                format!(
                    "{kind_name} family '{}' transient relational contract does not support wrapper output overrides",
                    contract.family
                ),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::SinExpression
            && let Err(err) = Self::validate_sin_expression_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::AgeCap
            && let Err(err) = Self::validate_age_cap_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::DiodeModelAlias
            && let Err(err) = Self::validate_diode_model_alias_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::SwitchStateCase
            && let Err(err) = Self::validate_switch_state_case_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::ParamExpression
            && let Err(err) = Self::validate_param_expression_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::PassiveCapPrimaryValue
            && let Err(err) = Self::validate_passive_cap_primary_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::PassiveTemperatureOverride
            && let Err(err) =
                Self::validate_passive_temperature_override_transient_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }
        if contract.kind == XyceBaselineFamilyKind::TransientAnalysisExpression
            && let Err(err) = Self::validate_transient_analysis_expression_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline qualification failed: {err}",
                    contract.family
                ),
                Vec::new(),
            );
        }

        let (baseline_netlist, baseline_result) = match self.run_transient_family_plan(
            &baseline_plan,
            start,
            None,
            None,
        ) {
            Ok(result) => result,
            Err(SimulationError::Aborted) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' transient baseline exceeded timeout ({}ms)",
                        contract.family, self.config.max_time_per_test_ms
                    ),
                    Vec::new(),
                );
            }
            Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    format!(
                        "{kind_name} family '{}' transient baseline is not supported by RSpice yet: {err}",
                        contract.family
                    ),
                );
            }
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' transient baseline error: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        let baseline_snapshot = if contract.comparison.requires_semantic_snapshot() {
            match Self::strict_transient_family_snapshot(
                &contract,
                &baseline_netlist,
                &baseline_plan.print,
            ) {
                Ok(snapshot) => Some(snapshot),
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline semantic qualification failed: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            }
        } else {
            None
        };
        let baseline_table = match Self::transient_family_result_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_result,
        ) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' transient baseline output conversion failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        if contract.kind.compares_transient_baseline_oracle() {
            let Some(reference_path) = self.static_prn_reference_path(&contract.baseline_path)
            else {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' baseline has no canonical .prn oracle path",
                        contract.family
                    ),
                    Vec::new(),
                );
            };
            if !reference_path.is_file() {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' canonical baseline oracle {} is missing",
                        contract.family,
                        self.display_path(&reference_path)
                    ),
                    Vec::new(),
                );
            }
            let reference = match Self::parse_prn_file(&reference_path) {
                Ok(reference) => reference,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline oracle parse error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
            let mismatches = match self.compare_static_tran_primary_reference(
                &reference,
                &baseline_plan,
                &baseline_netlist,
                &baseline_result,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' baseline oracle comparison error: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
            if !mismatches.is_empty() {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{} {kind_name} family '{}' baseline oracle mismatch(es)",
                        mismatches.len(),
                        contract.family
                    ),
                    mismatches,
                );
            }
        }
        let baseline_time_scale = match Self::tran_print_time_scale_factor(&baseline_plan.source) {
            Ok(scale) => scale,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' transient baseline time-scale error: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let (targets, baseline_record) = Self::baseline_family_targets(&contract);
        if targets.is_empty() {
            return self.baseline_family_qualification_result(
                deck,
                start,
                wrapper_contract,
                contract.comparison,
                format!(
                    "{kind_name} family '{}' has no non-baseline member to compare",
                    contract.family
                ),
            );
        }

        for target_path in targets {
            let target_plan = match self.static_tran_family_plan_for_path(
                &target_path,
                self.transient_family_plan_purpose_for_path(contract.kind, &target_path),
            ) {
                Ok(plan) => plan,
                Err(reason) => {
                    return self.baseline_family_qualification_result(
                        deck,
                        start,
                        wrapper_contract,
                        contract.comparison,
                        format!(
                            "{kind_name} family '{}' member {} is not supported by the static TRAN adapter: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
            };
            if !target_plan.steps.is_empty() || target_plan.output_override {
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    format!(
                        "{kind_name} family '{}' member {} uses stepped or overridden transient output, which the relational contract does not yet support",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::SinExpression
                && let Err(err) = Self::validate_sin_expression_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::AgeCap
                && let Err(err) = Self::validate_age_cap_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::DiodeModelAlias
                && let Err(err) = Self::validate_diode_model_alias_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::SwitchStateCase
                && let Err(err) = Self::validate_switch_state_case_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::ParamExpression
                && let Err(err) = Self::validate_param_expression_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::PassiveCapPrimaryValue
                && let Err(err) = Self::validate_passive_cap_primary_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::PassiveTemperatureOverride
                && let Err(err) =
                    Self::validate_passive_temperature_override_transient_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if contract.kind == XyceBaselineFamilyKind::TransientAnalysisExpression
                && let Err(err) = Self::validate_transient_analysis_expression_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} qualification failed: {err}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
            if !Self::baseline_family_tran_contracts_compatible(
                contract.kind,
                baseline_plan.contract,
                target_plan.contract,
            ) {
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    format!(
                        "{kind_name} family '{}' member {} uses {:?} transient output, but the baseline uses {:?}; relational formats must match",
                        contract.family,
                        self.display_path(&target_path),
                        target_plan.contract,
                        baseline_plan.contract
                    ),
                );
            }
            if contract.comparison.requires_exact_plan_equivalence() {
                if target_plan.print.probes != baseline_plan.print.probes {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' exact member {} changes ordered .PRINT TRAN probes",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
                if !Self::tran_analyses_match_exactly(&baseline_plan.tran, &target_plan.tran) {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' exact member {} changes the .TRAN analysis tuple",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
                if target_plan.timeint_conststep != baseline_plan.timeint_conststep {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' exact member {} changes constant-step output semantics",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            }
            let target_time_scale = match Self::tran_print_time_scale_factor(&target_plan.source) {
                Ok(scale) => scale,
                Err(err) => {
                    return self.baseline_family_qualification_result(
                        deck,
                        start,
                        wrapper_contract,
                        contract.comparison,
                        format!(
                            "{kind_name} family '{}' member {} has an unsupported transient time scale: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
            };
            let time_scale_differs = if contract.comparison.requires_exact_plan_equivalence() {
                target_time_scale.to_bits() != baseline_time_scale.to_bits()
            } else {
                let scale = baseline_time_scale
                    .abs()
                    .max(target_time_scale.abs())
                    .max(1.0);
                (target_time_scale - baseline_time_scale).abs() > Value::EPSILON * scale
            };
            if time_scale_differs {
                let reason = format!(
                    "{kind_name} family '{}' member {} uses transient time scale {}, but the baseline uses {}",
                    contract.family,
                    self.display_path(&target_path),
                    target_time_scale,
                    baseline_time_scale
                );
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    reason,
                );
            }

            // Exact relational families compare the independently integrated
            // target waveform against the baseline. Replaying the baseline's
            // accepted-step grid here is not semantics-preserving: companion
            // histories and nonlinear state evolve from the actual accepted
            // steps, so a forced grid can change target physics even when both
            // independent runs already produce the same grid. The exact
            // comparator below remains authoritative for rejecting any target
            // whose grid or serialized waveform is not identical.
            let (target_netlist, target_result) = match self.run_transient_family_plan(
                &target_plan,
                start,
                None,
                None,
            ) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} exceeded timeout ({}ms)",
                            contract.family,
                            self.display_path(&target_path),
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(err) if Self::is_expected_unsupported_runtime_error(&err) => {
                    return self.baseline_family_qualification_result(
                        deck,
                        start,
                        wrapper_contract,
                        contract.comparison,
                        format!(
                            "{kind_name} family '{}' member {} is not supported by RSpice yet: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} error: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            if let Some(baseline_snapshot) = baseline_snapshot.as_ref() {
                let target_snapshot = match Self::strict_transient_family_snapshot(
                    &contract,
                    &target_netlist,
                    &target_plan.print,
                ) {
                    Ok(snapshot) => snapshot,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} semantic qualification failed: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
                if let Err(err) = Self::compare_strict_transient_family_snapshots(
                    baseline_snapshot,
                    &target_snapshot,
                ) {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} changes qualified semantic state: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            }

            let mut mismatches = if contract.comparison.compares_waveforms_exactly() {
                let target_table = match Self::transient_family_result_to_prn_table(
                    &target_plan,
                    &target_netlist,
                    &target_result,
                ) {
                    Ok(table) => table,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} exact output conversion failed: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
                let comparison = if contract.comparison.compares_serialized_prn_exactly() {
                    self.compare_serialized_default_prn_tables(&baseline_table, &target_table)
                } else {
                    self.compare_exact_prn_tables(
                        &baseline_table,
                        &target_table,
                        &baseline_result.time,
                        &target_result.time,
                    )
                };
                match comparison {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} exact comparison error: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                }
            } else if contract.comparison.uses_xyce_verify_transient_oracle() {
                let target_table = match Self::transient_family_result_to_prn_table(
                    &target_plan,
                    &target_netlist,
                    &target_result,
                ) {
                    Ok(table) => table,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} xyce_verify output conversion failed: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
                match self.compare_xyce_verify_transient_tables(&baseline_table, &target_table) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} xyce_verify comparison error: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                }
            } else {
                match self.compare_tran_prn_reference(
                    &baseline_table,
                    &target_plan.print,
                    &target_netlist,
                    &baseline_plan.source,
                    &target_result,
                    None,
                ) {
                    Ok(mismatches) => mismatches,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} comparison error: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                }
            };
            if contract.comparison.permits_locked_time_retry() && !mismatches.is_empty() {
                let locked_run = self.run_transient_family_plan(
                    &target_plan,
                    start,
                    Some(baseline_result.time.clone()),
                    Some(baseline_result.step_sizes.clone()),
                );
                if let Ok((locked_netlist, locked_result)) = locked_run
                    && let Ok(locked_mismatches) = self.compare_tran_prn_reference(
                        &baseline_table,
                        &target_plan.print,
                        &locked_netlist,
                        &baseline_plan.source,
                        &locked_result,
                        None,
                    )
                    && Self::candidate_mismatches_are_better(Some(&mismatches), &locked_mismatches)
                {
                    mismatches = locked_mismatches;
                }
            }
            if !mismatches.is_empty() {
                for mismatch in &mut mismatches {
                    mismatch.probe =
                        format!("{} {}", self.display_path(&target_path), mismatch.probe);
                }
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{} {kind_name} family '{}' member {} transient baseline mismatch(es)",
                        mismatches.len(),
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    mismatches,
                );
            }
        }

        let result_contract = if baseline_record {
            baseline_contract
        } else {
            wrapper_contract
        };
        self.passed_result(deck, start, result_contract)
    }

    pub(super) fn run_transient_family_plan(
        &self,
        plan: &XyceStaticTranPlan,
        start: Instant,
        locked_time_grid: Option<Vec<Value>>,
        locked_time_step_sizes: Option<Vec<Value>>,
    ) -> Result<(Netlist, TransientResult), SimulationError> {
        let netlist = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|err| SimulationError::Netlist(format!("{err}")))?;
        let result = self.run_transient_family_netlist(
            plan,
            &netlist,
            start,
            locked_time_grid,
            locked_time_step_sizes,
        )?;
        Ok((netlist, result))
    }

    pub(super) fn run_transient_family_netlist(
        &self,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        start: Instant,
        locked_time_grid: Option<Vec<Value>>,
        locked_time_step_sizes: Option<Vec<Value>>,
    ) -> Result<TransientResult, SimulationError> {
        let max_step = Self::transient_family_max_step(netlist, &plan.tran)
            .map_err(SimulationError::Netlist)?;
        let initial_step = Self::xyce_initial_timestep_for_tran(&plan.tran);
        let engine = self.create_xyce_static_tran_engine_with_step_sizes(
            locked_time_grid,
            locked_time_step_sizes,
            initial_step,
        );
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        engine.run_tran_with_abort(netlist, plan.tran.stop, max_step, &abort)
    }

    pub(super) fn run_static_dc_results(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(Netlist, Vec<DcSweepPointResult>), SimulationError> {
        if !plan.steps.is_empty() {
            return Err(SimulationError::Netlist(
                ".STEP static DC execution requires the stepped .prn contract".to_string(),
            ));
        }
        let netlist = Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
            &plan.source,
            &plan.deck_path,
            plan.expression_dialect,
            plan.parameter_redefinition_policy,
            plan.execution_dir.as_deref(),
        )
        .map_err(|err| SimulationError::Netlist(format!("{err}")))?;
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = engine.run_dc_sweep2_spec_with_report_and_abort(
            &netlist,
            &plan.dc.source,
            &plan.dc.primary_spec(),
            plan.dc.sweep2.as_ref(),
            &abort,
        )?;
        Ok((netlist, results))
    }

    pub(super) fn run_static_dc_data_results(
        &self,
        base_netlist: &Netlist,
        dc_data: &XyceDcDataSweep,
        start: Instant,
    ) -> Result<Vec<XyceDcDataPointResult>, SimulationError> {
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut results = Vec::with_capacity(dc_data.rows.len());

        for (row_index, row) in dc_data.rows.iter().enumerate() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let row_netlist = Self::materialize_dc_data_row_netlist(&engine, base_netlist, row)?;
            let (result, device_op_report) =
                engine.run_dc_op_with_report(&row_netlist).map_err(|err| {
                    SimulationError::Circuit(format!(
                        ".DC DATA row {} operating point failed: {}",
                        row_index + 1,
                        err
                    ))
                })?;
            results.push(XyceDcDataPointResult {
                netlist: row_netlist,
                point: DcSweepPointResult {
                    // Xyce exposes .DC DATA's synthetic independent variable
                    // as the one-based table-row ordinal. This axis drives
                    // measurement windows and point-event interpolation.
                    sweep_value: (row_index + 1) as Value,
                    result,
                    device_op_report,
                },
            });
        }

        Ok(results)
    }

    pub(super) fn run_static_dc_result_batches(
        &self,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        start: Instant,
    ) -> Result<Vec<XyceDcResultBatch>, SimulationError> {
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let dimensions = Self::dc_sweep_dimensions(netlist);
        if dimensions.len() <= 2 {
            let results = engine.run_dc_sweep2_spec_with_report_and_abort(
                netlist,
                &dc.source,
                &dc.primary_spec(),
                dc.sweep2.as_ref(),
                &abort,
            )?;
            return Ok(vec![XyceDcResultBatch {
                netlist: netlist.clone(),
                results,
            }]);
        }

        let primary_points =
            rspice_core::engine::bounded_dc_sweep_points(&engine, &dimensions[0].spec(), &abort)?;
        let secondary_points =
            rspice_core::engine::bounded_dc_sweep_points(&engine, &dimensions[1].spec(), &abort)?;
        if primary_points.is_empty() || secondary_points.is_empty() {
            return Err(SimulationError::Circuit(
                "DC sweep has no points in its first two dimensions".to_string(),
            ));
        }

        let mut outer_points = Vec::with_capacity(dimensions.len() - 2);
        let mut batch_count = 1usize;
        for dimension in &dimensions[2..] {
            let points =
                rspice_core::engine::bounded_dc_sweep_points(&engine, &dimension.spec(), &abort)?;
            if points.is_empty() {
                return Err(SimulationError::Circuit(format!(
                    "DC sweep dimension '{}' has no points",
                    dimension.source
                )));
            }
            batch_count = batch_count.saturating_mul(points.len());
            outer_points.push(points);
        }

        let inner_count = primary_points.len().saturating_mul(secondary_points.len());
        engine.ensure_analysis_points(batch_count.saturating_mul(inner_count))?;
        engine.ensure_batch_runs(batch_count)?;

        let mut indices = vec![0usize; outer_points.len()];
        let mut batches = Vec::with_capacity(batch_count);
        loop {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }

            let mut swept = netlist.clone();
            // Xyce declares the first sweep dimension as the fastest-changing
            // coordinate.  The first two remain the engine's inner/secondary
            // sweep; additional dimensions are materialized as outer batches
            // in declaration order, with dimension three changing fastest.
            for (dimension, (points, index)) in dimensions[2..]
                .iter()
                .zip(outer_points.iter().zip(indices.iter().copied()))
                .filter(|(dimension, _)| {
                    !Self::is_temperature_name(&dimension.source)
                        && !Self::scalar_parameter_sweep_source_is_supported(
                            netlist,
                            &dimension.source,
                        )
                })
            {
                Self::apply_static_dc_dimension(&mut swept, dimension, points[index])?;
            }
            for (dimension, (points, index)) in dimensions[2..]
                .iter()
                .zip(outer_points.iter().zip(indices.iter().copied()))
                .filter(|(dimension, _)| {
                    Self::is_temperature_name(&dimension.source)
                        || Self::scalar_parameter_sweep_source_is_supported(
                            netlist,
                            &dimension.source,
                        )
                })
            {
                Self::apply_static_dc_dimension(&mut swept, dimension, points[index])?;
            }

            let secondary = dimensions[1].clone().into_second_sweep();
            let results = engine.run_dc_sweep2_spec_with_report_and_abort(
                &swept,
                &dimensions[0].source,
                &dimensions[0].spec(),
                Some(&secondary),
                &abort,
            )?;
            batches.push(XyceDcResultBatch {
                netlist: swept,
                results,
            });

            let mut position = 0usize;
            while position < indices.len() {
                indices[position] += 1;
                if indices[position] < outer_points[position].len() {
                    break;
                }
                indices[position] = 0;
                position += 1;
            }
            if position == indices.len() {
                break;
            }
        }

        Ok(batches)
    }
}
