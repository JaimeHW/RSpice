//! Driving a single deck and a whole suite through the engine.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::contracts_bug307::Bug307Role;
use super::contracts_bug352::Bug352Role;
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
            return self.run_discovered_test_unqualified(deck);
        }
        let upstream_exclusion = match &self.upstream_exclusions {
            Ok(exclusions) => exclusions
                .get(&Self::normalize_manifest_key(&deck.relative_path))
                .cloned(),
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    "upstream_exclusion_manifest_error",
                    error.clone(),
                    Vec::new(),
                );
            }
        };
        let Some(upstream_exclusion) = upstream_exclusion else {
            return self.run_discovered_test_unqualified(deck);
        };
        match upstream_exclusion.disposition {
            XyceUpstreamExclusionDisposition::Excluded => {
                self.upstream_excluded_result(deck, start, &upstream_exclusion.source)
            }
            XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                expected_contract,
            } => {
                let mut result = self.run_discovered_test_unqualified(deck);
                let actual_contract = result.contract.clone();
                let actual_passed = result.passed;
                let actual_expected_unsupported = result.expected_unsupported;
                let promotion_is_valid = result.passed
                    && !result.expected_unsupported
                    && !result.upstream_excluded
                    && actual_contract == expected_contract;
                result.upstream_exclusion_source = Some(upstream_exclusion.source);
                if !promotion_is_valid {
                    let underlying_error = result.error.take();
                    result.passed = false;
                    result.expected_unsupported = false;
                    result.upstream_excluded = false;
                    result.contract = "upstream_exclusion_promotion_mismatch".to_string();
                    result.error = Some(format!(
                        "RSpice independent qualification expected contract {expected_contract:?}, but the deck produced contract {actual_contract:?} with passed={}, expected_unsupported={}{}",
                        actual_passed,
                        actual_expected_unsupported,
                        underlying_error
                            .as_deref()
                            .map(|error| format!("; underlying result: {error}"))
                            .unwrap_or_default()
                    ));
                }
                result
            }
        }
    }

    fn run_discovered_test_unqualified(&self, deck: &XyceDeck) -> XyceTestResult {
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

        if Self::normalize_manifest_key(&deck.relative_path) == XYCE_BUG864_RECORD {
            let result = match self.validate_bug864_oracle(deck, start) {
                Ok(()) => self.passed_result(deck, start, XYCE_BUG864_CONTRACT),
                Err(error) => {
                    self.failure_result(deck, start, XYCE_BUG864_CONTRACT, error, Vec::new())
                }
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

        if Self::normalize_manifest_key(&deck.relative_path) == XYCE_BUG48_RECORD {
            let result = match self.validate_bug48_oracle(deck, start) {
                Ok(()) => self.passed_result(deck, start, XYCE_BUG48_CONTRACT),
                Err(error) => {
                    self.failure_result(deck, start, XYCE_BUG48_CONTRACT, error, Vec::new())
                }
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

        if Self::normalize_manifest_key(&deck.relative_path) == XYCE_BUG159_OWNER_RECORD {
            let result = match self.validate_bug159_oracle(deck, start) {
                Ok(()) => self.passed_result(deck, start, XYCE_BUG159_CONTRACT),
                Err(error) => {
                    self.failure_result(deck, start, XYCE_BUG159_CONTRACT, error, Vec::new())
                }
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

        if Self::normalize_manifest_key(&deck.relative_path) == XYCE_BUG267_RECORD {
            let result = match self.validate_bug267_oracle(deck, start) {
                Ok(()) => self.passed_result(deck, start, XYCE_BUG267_CONTRACT),
                Err(error) => {
                    self.failure_result(deck, start, XYCE_BUG267_CONTRACT, error, Vec::new())
                }
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

        if let Some(role) = Bug352Role::for_record(&deck.relative_path) {
            let contract = role.contract();
            let result = match self.validate_bug352_oracle(deck, role, start) {
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

        if let Some(role) = Bug307Role::for_record(&deck.relative_path) {
            let contract = role.contract();
            let result = match self.validate_bug307_oracle(deck, role, start) {
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

        if let Some(contract) = self.bug546_temperature_rc_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_analytic_rc_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    XyceAnalyticRcKind::PassiveTemperature.result_contract(),
                    format!(
                        "BUG546 analytic passive-temperature RC qualification failed: {reason}"
                    ),
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

        if let Some(contract) = self.analytic_rc_wrapper_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_analytic_rc_contract(deck, contract, start),
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

        if let Some(contract) = self.analytic_int_floor_ceil_tran_wrapper_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_analytic_int_floor_ceil_tran_wrapper_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "analytic_int_floor_ceil_tran_wrapper",
                    format!("analytic INT/FLOOR/CEIL transient qualification failed: {reason}"),
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

        if let Some(contract) = self.classic_mos_dtemp_relational_contract(deck) {
            let role = XyceClassicMosDtempRole::for_record(&deck.relative_path)
                .expect("classic MOS DTEMP detector selected a recognized record")
                .1;
            let result = match contract {
                Ok(contract) => {
                    self.run_classic_mos_dtemp_relational_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!("classic MOS DTEMP relational qualification failed: {reason}"),
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

        if let Some(contract) = self.legacy_bjt_dtemp_relational_contract(deck) {
            let role = XyceLegacyBjtDtempRole::for_record(&deck.relative_path)
                .expect("legacy BJT DTEMP detector selected a recognized record")
                .1;
            let result = match contract {
                Ok(contract) => {
                    self.run_legacy_bjt_dtemp_relational_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!("legacy BJT DTEMP relational qualification failed: {reason}"),
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

        if let Some(contract) = self.xyce_sydney_level1_jfet_dtemp_relational_contract(deck) {
            let role = XyceSydneyLevel1JfetDtempRole::for_record(&deck.relative_path)
                .expect("Xyce Sydney level-1 JFET DTEMP detector selected a recognized record")
                .1;
            let result = match contract {
                Ok(contract) => self
                    .run_xyce_sydney_level1_jfet_dtemp_relational_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!(
                        "Xyce Sydney level-1 JFET DTEMP relational qualification failed: {reason}"
                    ),
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

        if let Some(contract) = self.level2_diode_dtemp_relational_contract(deck) {
            let role = XyceLevel2DiodeDtempRole::for_record(&deck.relative_path)
                .expect("Level-2 diode DTEMP detector selected a recognized record");
            let result = match contract {
                Ok(contract) => {
                    self.run_level2_diode_dtemp_relational_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!("Level-2 diode DTEMP relational qualification failed: {reason}"),
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

        if let Some(contract) = self.capacitor_dtemp_relational_contract(deck) {
            let role = XyceCapacitorDtempRole::for_record(&deck.relative_path)
                .expect("capacitor DTEMP detector selected a recognized record");
            let result = match contract {
                Ok(contract) => self.run_capacitor_dtemp_relational_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!("capacitor DTEMP relational qualification failed: {reason}"),
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

        if let Some(contract) = self.nonlinear_core_model_step_reference_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_nonlinear_core_model_step_reference_contract(deck, contract, start)
                }
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    if self.requires_upstream_wrapper(&deck.relative_path) {
                        XYCE_NONLINEAR_CORE_MODEL_STEP_WRAPPER_CONTRACT
                    } else {
                        XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT
                    },
                    format!("nonlinear CORE model-step family discovery failed: {reason}"),
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

        if let Some(contract) = self.bug1190_mutual_inductor_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_bug1190_mutual_inductor_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    if self.requires_upstream_wrapper(&deck.relative_path) {
                        XYCE_BUG1190_MUTUAL_INDUCTOR_WRAPPER_CONTRACT
                    } else {
                        XYCE_BUG1190_MUTUAL_INDUCTOR_BASELINE_CONTRACT
                    },
                    format!("BUG 1190 mutual-inductor family discovery failed: {reason}"),
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

        if let Some(contract) = self.vbic_dc_wrapper_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_vbic_dc_wrapper_family_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    "vbic_dc_wrapper_equivalence_family",
                    format!("VBIC DC wrapper family qualification failed: {reason}"),
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

        if let Some(contract) = self.params1_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_params1_family_contract(deck, contract, start),
                Err(reason) => {
                    let role = XyceParams1Role::for_record(&deck.relative_path).expect(
                        "PARAMS1 family detection must only select a recognized family record",
                    );
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!("PARAMS1 family provenance qualification failed: {reason}"),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.naked_algebra_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_naked_algebra_family_contract(deck, contract, start),
                Err(reason) => {
                    let role = XyceNakedAlgebraRole::for_record(&deck.relative_path).expect(
                        "nakedAlgebra family detection must only select a recognized family record",
                    );
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!("nakedAlgebra family provenance qualification failed: {reason}"),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.bug1826_thermal_parameter_family_contract(deck) {
            let result = match contract {
                Ok(contract) => {
                    self.run_bug1826_thermal_parameter_family_contract(deck, contract, start)
                }
                Err(reason) => {
                    let role = XyceBug1826ThermalParameterRole::for_record(&deck.relative_path)
                        .expect(
                            "BUG 1826 family detection must only select a recognized family record",
                        );
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!("BUG 1826 family provenance qualification failed: {reason}"),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.abm_lookup_order_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_abm_lookup_order_family_contract(deck, contract, start),
                Err(reason) => {
                    let (_, role) = XyceAbmLookupOrderRole::for_record(&deck.relative_path).expect(
                        "ABM_SPLINES lookup-order detection selects only recognized records",
                    );
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!(
                            "ABM_SPLINES lookup-order family provenance qualification failed: {reason}"
                        ),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.bug38_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_bug38_family_contract(deck, contract, start),
                Err(reason) => {
                    let role = XyceBug38Role::for_record(&deck.relative_path)
                        .expect("BUG_38_SON detection selects only recognized records");
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!(
                            "BUG_38_SON formal-parentheses family provenance qualification failed: {reason}"
                        ),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.bug39_gaussian_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_bug39_gaussian_contract(deck, contract, start),
                Err(reason) => {
                    let role = XyceBug39GaussianRole::for_record(&deck.relative_path)
                        .expect("BUG_39_SON detection selects only recognized records");
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!(
                            "BUG_39_SON generated Gaussian provenance qualification failed: {reason}"
                        ),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.bug39_deterministic_contract(deck) {
            let role = XyceBug39DeterministicRole::for_record(&deck.relative_path)
                .expect("BUG_39_SON deterministic detection selects only recognized records");
            let result = match contract {
                Ok(contract) => self.run_bug39_deterministic_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!(
                        "BUG_39_SON deterministic expression provenance qualification failed: {reason}"
                    ),
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

        if let Some(contract) = self.bug402_temperature_option_contract(deck) {
            let role = XyceBug402TemperatureRole::for_record(&deck.relative_path)
                .expect("BUG_402_SON detection selects only recognized records");
            let result = match contract {
                Ok(contract) => self.run_bug402_temperature_option_contract(deck, contract, start),
                Err(reason) => self.failure_result(
                    deck,
                    start,
                    role.result_contract(),
                    format!("BUG_402_SON temperature-option scope qualification failed: {reason}"),
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

        if let Some(contract) = self.source_multiplicity_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_source_multiplicity_family_contract(deck, contract, start),
                Err(reason) => {
                    let (_, role) = XyceSourceMultiplicityRole::for_record(&deck.relative_path)
                        .expect("source-multiplicity detection selects only recognized records");
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!(
                            "source-multiplicity family provenance qualification failed: {reason}"
                        ),
                        Vec::new(),
                    )
                }
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

        if let Some(contract) = self.abm_frequency_family_contract(deck) {
            let result = match contract {
                Ok(contract) => self.run_abm_frequency_family_contract(deck, contract, start),
                Err(reason) => {
                    let (_, role) = XyceAbmFrequencyRole::for_record(&deck.relative_path)
                        .expect("ABM_FREQ detection selects only recognized records");
                    self.failure_result(
                        deck,
                        start,
                        role.result_contract(),
                        format!("ABM_FREQ family provenance qualification failed: {reason}"),
                        Vec::new(),
                    )
                }
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
            return Some(
                match self.validate_expected_failure_oracle(deck, kind, start) {
                    Ok(()) => self.passed_result(deck, start, contract),
                    Err(error) => self.failure_result(deck, start, contract, error, Vec::new()),
                },
            );
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
        let reference_path = match contract
            .plan
            .require_waveform_reference_path("BUG702 positive execution")
        {
            Ok(path) => path,
            Err(error) => {
                return self.failure_result(deck, start, result_contract, error, Vec::new());
            }
        };
        let reference = match Self::parse_xyce_verify_tran_reference_file(reference_path) {
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
        startup_config.integration_method =
            rspice_core::numerics::integration::IntegrationMethod::BackwardEuler;
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
            let measurements = rspice_core::analysis::evaluate_ac_measurements(&netlist, &results);
            let continuous =
                rspice_core::analysis::evaluate_ac_continuous_measurements(&netlist, &results);
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
                    rspice_core::analysis::evaluate_ac_continuous_measurements(&netlist, &results);
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
            let scalar = rspice_core::analysis::evaluate_noise_measurements(&netlist, &results);
            let continuous =
                rspice_core::analysis::evaluate_noise_continuous_measurements(&netlist, &results);
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
                let measurements =
                    rspice_core::analysis::evaluate_noise_measurements(&run.netlist, &results);
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
                    let continuous = rspice_core::analysis::evaluate_noise_continuous_measurements(
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
            let continuous =
                rspice_core::analysis::evaluate_ac_continuous_measurements(&run.netlist, &results);
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
                rspice_core::analysis::evaluate_dc_measurements(&netlist, &measurement_sweep)
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

        let print = match plan.require_print("NOINDEX transient header execution") {
            Ok(print) => print,
            Err(error) => {
                return self.failure_result(deck, start, contract, error, Vec::new());
            }
        };
        let header = Self::transient_prn_header_columns(print, false).join("   ");
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
        if let Err(error) = plan.validate_executable_oracle_shape() {
            return self.failure_result(deck, start, contract, error, Vec::new());
        }
        if matches!(plan.contract, XyceStaticTranContract::WrapperNoIndexHeader) {
            return self.run_noindex_header_tran_wrapper_plan(deck, &plan, start);
        }
        let mut netlist = match Self::parse_xyce_netlist(&plan.source, &plan.deck_path) {
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

        if plan.is_scalar_measurement_only() {
            if let Err(error) = Self::normalize_scalar_tran_measurement_file_paths(&mut netlist) {
                return self.failure_result(deck, start, contract, error, Vec::new());
            }
            return self.run_static_scalar_tran_measurement_plan(deck, &plan, &netlist, start);
        }

        let reference_path =
            match plan.require_waveform_reference_path("static transient execution") {
                Ok(path) => path,
                Err(error) => {
                    return self.failure_result(deck, start, contract, error, Vec::new());
                }
            };
        let reference_result = match plan.comparison_mode {
            XyceStaticTranComparisonMode::Release710IntegratedRms { .. }
            | XyceStaticTranComparisonMode::Release710IntegratedRmsComp { .. } => {
                Self::parse_xyce_verify_tran_reference_file(reference_path)
            }
            XyceStaticTranComparisonMode::Pointwise => {
                Self::parse_tran_reference_file(plan.contract, reference_path)
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
        let print = match plan.require_print("static transient execution") {
            Ok(print) => print,
            Err(error) => {
                return self.failure_result(deck, start, contract, error, Vec::new());
            }
        };
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
        // A printed Xyce grid contains output breakpoints, not the solver's
        // accepted internal timestep history.  The intervals between PRN
        // rows may contain rejected candidates and hidden substeps, so they
        // must not be replayed as fixed companion-history deltas.  Keep the
        // grid as an output/comparison boundary and let the transient solver
        // determine its own accepted steps inside each interval.
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
        match engine.run_tran_with_startup_mode_and_abort(
            &netlist,
            tran.stop,
            max_step,
            rspice_core::engine::TransientStartupMode::from_uic(tran.uic),
            &abort,
        ) {
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
            Self::transient_print_requests_linear_capacitor_branch_quantity(&netlist, print);
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
        match locked_engine.run_tran_with_startup_mode_and_abort(
            &netlist,
            tran.stop,
            locked_max_step,
            rspice_core::engine::TransientStartupMode::from_uic(tran.uic),
            &abort,
        ) {
            Ok(locked_result) => {
                match self.compare_tran_prn_reference(
                    &reference,
                    print,
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
                .create_xyce_static_tran_engine_with_step_sizes_and_integration_method(
                    Some(reference_time_grid.clone()),
                    None,
                    initial_step,
                    rspice_core::numerics::integration::IntegrationMethod::BackwardEuler,
                );
            match backward_euler_engine.run_tran_with_startup_mode_and_abort(
                &netlist,
                tran.stop,
                locked_max_step,
                rspice_core::engine::TransientStartupMode::from_uic(tran.uic),
                &abort,
            ) {
                Ok(backward_euler_result) => {
                    match self.compare_tran_prn_reference(
                        &reference,
                        print,
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
            let gear12_engine = self
                .create_xyce_static_tran_engine_with_step_sizes_and_integration_method(
                    Some(reference_time_grid),
                    None,
                    initial_step,
                    rspice_core::numerics::integration::IntegrationMethod::Gear2,
                );
            match gear12_engine.run_tran_with_startup_mode_and_abort(
                &netlist,
                tran.stop,
                locked_max_step,
                rspice_core::engine::TransientStartupMode::from_uic(tran.uic),
                &abort,
            ) {
                Ok(gear12_result) => {
                    match self.compare_tran_prn_reference(
                        &reference,
                        print,
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

    pub(super) fn run_static_scalar_tran_measurement_plan(
        &self,
        deck: &XyceDeck,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        start: Instant,
    ) -> XyceTestResult {
        let contract = plan.result_contract();
        let Some((measurement_reference_paths, measurement_tolerance, measurement_input)) =
            plan.scalar_measurement_oracle()
        else {
            return self.failure_result(
                deck,
                start,
                contract,
                "scalar TRAN measurement execution received an invalid oracle shape".to_string(),
                Vec::new(),
            );
        };

        let measurements = match measurement_input {
            XyceScalarTranMeasurementInput::Simulation => {
                let transient = match self
                    .run_transient_family_netlist(plan, netlist, start, None, None)
                {
                    Ok(transient) => transient,
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
                    Err(error) if Self::is_expected_unsupported_runtime_error(&error) => {
                        return self.expected_unsupported_result(
                            deck,
                            start,
                            "unsupported_xyce_runtime",
                            &format!(
                                "RSpice runtime does not yet support this scalar TRAN measurement deck: {error}"
                            ),
                        );
                    }
                    Err(error) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("scalar TRAN measurement simulation error: {error}"),
                            Vec::new(),
                        );
                    }
                };
                if let Err(error) = Self::validate_transient_result_time_grid(&transient) {
                    return self.failure_result(
                        deck,
                        start,
                        contract,
                        format!("scalar TRAN measurement result grid is invalid: {error}"),
                        Vec::new(),
                    );
                }
                rspice_core::analysis::evaluate_tran_measurements(netlist, &transient)
            }
            XyceScalarTranMeasurementInput::Remeasure(input_path) => {
                let input = match Self::load_tran_remeasure_input(input_path) {
                    Ok(input) => input,
                    Err(error) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract,
                            format!("scalar TRAN remeasure input error: {error}"),
                            Vec::new(),
                        );
                    }
                };
                let signals = input.signal_slices();
                rspice_core::analysis::evaluate_tran_remeasurements(netlist, &input.time, &signals)
            }
        };

        // Keep input preparation or simulation and post-processing inside one
        // per-test budget. Reusing the original start instant avoids granting
        // measurement evaluation or file comparison a fresh deadline.
        let deadline = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if deadline.is_aborted() {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "scalar TRAN measurement shared deadline expired before evaluation ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                Vec::new(),
            );
        }
        let mismatches = match self.compare_analysis_measurement_outputs(
            measurement_reference_paths,
            &[],
            &measurements,
            &[],
            measurement_tolerance,
            netlist.options.measure_fail_output,
            netlist.options.measure_default_value,
            netlist.options.measure_use_cont_files(),
            &netlist.measurements,
            "TRAN",
            "TRAN_CONT",
        ) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    contract,
                    format!("scalar TRAN measurement reference comparison error: {error}"),
                    Vec::new(),
                );
            }
        };
        if deadline.is_aborted() {
            return self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "scalar TRAN measurement shared deadline expired after comparison ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                mismatches,
            );
        }
        if mismatches.is_empty() {
            self.passed_result(deck, start, contract)
        } else {
            self.failure_result(
                deck,
                start,
                contract,
                format!(
                    "{} Xyce scalar TRAN measurement mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
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

    pub(super) fn run_vbic_dc_wrapper_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceVbicDcWrapperFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let contract_name = contract.role.contract();
        let owner_plan =
            match self.static_dc_plan_for_path(&contract.owner_path, ExpressionDialect::Xyce) {
                Ok(plan) => plan,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "VBIC family '{}' owner no longer plans: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
        let (owner_netlist, owner_steps, owner_batches) =
            match self.run_vbic_dc_family_plan(&owner_plan, start) {
                Ok(run) => run,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "VBIC family '{}' owner execution failed: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
        let owner_table =
            match self.vbic_dc_result_batches_to_prn_table(&owner_plan, &owner_batches) {
                Ok(table) => table,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "VBIC family '{}' owner output conversion failed: {err}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };
        let gold = match Self::parse_dc_reference_file(
            XyceStaticDcContract::WrapperDefault,
            &contract.reference_path,
        ) {
            Ok(reference) => reference,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract_name,
                    format!(
                        "VBIC family '{}' gold PRN could not be parsed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        let mut mismatches = match self.compare_release_7_10_xyce_verify_dc_batches(
            &format!("VBIC family '{}' owner/gold", contract.family),
            &gold,
            &owner_table,
            &owner_batches,
            &owner_batches,
        ) {
            Ok(mismatches) => mismatches,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    contract_name,
                    format!(
                        "VBIC family '{}' owner/gold comparison failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };
        for mismatch in &mut mismatches {
            mismatch.probe = format!("gold {}", mismatch.probe);
        }
        if !owner_plan.steps.is_empty() {
            let step_reference = contract.owner_path.with_extension("cir.res.gs");
            if let Err(err) = self.compare_step_res_reference(
                &step_reference,
                &owner_netlist,
                &owner_plan.steps,
                &owner_steps,
            ) {
                return self.failure_result(
                    deck,
                    start,
                    contract_name,
                    format!(
                        "VBIC family '{}' STEP summary comparison failed: {err}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        }
        if !mismatches.is_empty() {
            mismatches.truncate(self.config.max_mismatches);
            return self.failure_result(
                deck,
                start,
                contract_name,
                format!(
                    "{} Xyce mismatch(es) between VBIC family '{}' owner and checked gold",
                    mismatches.len(),
                    contract.family
                ),
                mismatches,
            );
        }

        let targets = match contract.role {
            XyceVbicDcWrapperFamilyRole::Owner => {
                vec![contract.multiplicity_path, contract.polarity_path]
            }
            XyceVbicDcWrapperFamilyRole::MultiplicityControl => {
                vec![contract.multiplicity_path]
            }
            XyceVbicDcWrapperFamilyRole::PolarityControl => vec![contract.polarity_path],
        };
        let mut all_mismatches = Vec::new();
        for target_path in targets {
            let target_plan =
                match self.static_dc_plan_for_path(&target_path, ExpressionDialect::Xyce) {
                    Ok(plan) => plan,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract_name,
                            format!(
                                "VBIC family '{}' control '{}' no longer plans: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
            let (target_netlist, target_steps, target_batches) =
                match self.run_vbic_dc_family_plan(&target_plan, start) {
                    Ok(run) => run,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract_name,
                            format!(
                                "VBIC family '{}' control '{}' execution failed: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
            if !target_plan.steps.is_empty() {
                let step_reference = contract.owner_path.with_extension("cir.res.gs");
                if let Err(err) = self.compare_step_res_reference(
                    &step_reference,
                    &target_netlist,
                    &target_plan.steps,
                    &target_steps,
                ) {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "VBIC family '{}' control '{}' STEP summary comparison failed: {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            }
            let target_table =
                match self.vbic_dc_result_batches_to_prn_table(&target_plan, &target_batches) {
                    Ok(table) => table,
                    Err(err) => {
                        return self.failure_result(
                            deck,
                            start,
                            contract_name,
                            format!(
                                "VBIC family '{}' control '{}' output conversion failed: {err}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                };
            let mut target_mismatches = match self.compare_release_7_10_xyce_verify_dc_batches(
                &format!(
                    "VBIC family '{}' control '{}'",
                    contract.family,
                    self.display_path(&target_path)
                ),
                &owner_table,
                &target_table,
                &owner_batches,
                &target_batches,
            ) {
                Ok(mismatches) => mismatches,
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        contract_name,
                        format!(
                            "VBIC family '{}' owner/control comparison failed for '{}': {err}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut target_mismatches {
                mismatch.probe = format!("{} {}", self.display_path(&target_path), mismatch.probe);
            }
            all_mismatches.extend(target_mismatches);
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
                    "{} Xyce mismatch(es) in VBIC wrapper-equivalence family '{}'",
                    all_mismatches.len(),
                    contract.family
                ),
                all_mismatches,
            )
        }
    }

    fn run_vbic_dc_family_plan(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(Netlist, Vec<XyceStepRun>, Vec<XyceDcResultBatch>), SimulationError> {
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
        let step_runs = if plan.steps.is_empty() {
            vec![XyceStepRun {
                step_values: Vec::new(),
                netlist: netlist.clone(),
            }]
        } else {
            Self::nested_step_runs_for_commands_with_limits_and_abort(
                &engine,
                &netlist,
                &plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )?
        };
        let mut batches = Vec::with_capacity(step_runs.len());
        for run in &step_runs {
            let results = engine.run_dc_sweep2_spec_with_report_and_abort(
                &run.netlist,
                &plan.dc.source,
                &plan.dc.primary_spec(),
                plan.dc.sweep2.as_ref(),
                &abort,
            )?;
            batches.push(XyceDcResultBatch {
                netlist: run.netlist.clone(),
                results,
            });
        }
        Ok((netlist, step_runs, batches))
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

        if plan.contract.compares_step_res_reference()
            && let Some(res_reference_path) =
                Self::step_res_reference_path(&plan.deck_path, &plan.reference_path)
            && let Err(err) = self.compare_step_res_reference(
                &res_reference_path,
                &netlist,
                &plan.steps,
                &step_runs,
            )
        {
            return self.failure_result(
                deck,
                start,
                contract,
                format!("Xyce .STEP result summary comparison error: {err}"),
                Vec::new(),
            );
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
        let result_contract = contract.kind.dc_result_contract();
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

    pub(super) fn run_analytic_int_floor_ceil_tran_wrapper_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAnalyticIntFloorCeilTranContract,
        start: Instant,
    ) -> XyceTestResult {
        const RESULT_CONTRACT: &str = "analytic_int_floor_ceil_tran_wrapper";
        let (netlist, result) =
            match self.run_transient_family_plan(&contract.plan, start, None, None) {
                Ok(run) => run,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!(
                            "analytic INT/FLOOR/CEIL transient execution exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms
                        ),
                        Vec::new(),
                    );
                }
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!("analytic INT/FLOOR/CEIL transient execution failed: {error}"),
                        Vec::new(),
                    );
                }
            };
        let actual =
            match Self::transient_family_result_to_prn_table(&contract.plan, &netlist, &result) {
                Ok(table) => table,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        RESULT_CONTRACT,
                        format!(
                            "analytic INT/FLOOR/CEIL transient output conversion failed: {error}"
                        ),
                        Vec::new(),
                    );
                }
            };
        let mismatches = match self.compare_analytic_int_floor_ceil_tran_table(&actual) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    RESULT_CONTRACT,
                    format!("analytic INT/FLOOR/CEIL transient exact comparison failed: {error}"),
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
                    "{} analytic INT/FLOOR/CEIL transient mismatch(es)",
                    mismatches.len()
                ),
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

    pub(super) fn run_classic_mos_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceClassicMosDtempContract,
        start: Instant,
    ) -> XyceTestResult {
        const LABEL: &str = "classic level-1 MOS TEMP/DTEMP family";
        let result_contract = contract.role.result_contract();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_classic_mos_dtemp_provenance(&contract)?;
            let parse = |plan: &XyceStaticDcPlan, role: &str| {
                Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
                    &plan.source,
                    &plan.deck_path,
                    plan.expression_dialect,
                    plan.parameter_redefinition_policy,
                    plan.execution_dir.as_deref(),
                )
                .map_err(|error| format!("{LABEL} {role} parse failed: {error}"))
            };
            let owner_netlist = parse(&contract.owner_plan, "owner")?;
            let reference_netlist = parse(&contract.reference_plan, "reference")?;
            let owner_snapshot = Self::classic_mos_dtemp_snapshot(
                &contract.owner_plan,
                &owner_netlist,
                XyceClassicMosDtempRole::Owner,
            )?;
            let reference_snapshot = Self::classic_mos_dtemp_snapshot(
                &contract.reference_plan,
                &reference_netlist,
                XyceClassicMosDtempRole::Reference,
            )?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "{LABEL} base owner/reference topology or analysis semantics differ"
                ));
            }

            let expansion_engine = self.create_dc_engine();
            let owner_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &owner_netlist,
                &contract.owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} owner STEP expansion failed: {error}"))?;
            let reference_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &reference_netlist,
                &contract.reference_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} reference STEP expansion failed: {error}"))?;
            let owner_coordinates: [Value; 3] = [0.0, 10.0, 20.0];
            let reference_coordinates: [Value; 3] = [15.0, 25.0, 35.0];
            if owner_runs.len() != owner_coordinates.len()
                || reference_runs.len() != reference_coordinates.len()
            {
                return Err(format!(
                    "{LABEL} requires three owner and three reference materializations, found {}/{}",
                    owner_runs.len(),
                    reference_runs.len()
                ));
            }
            for (index, (((owner_run, reference_run), owner_coordinate), reference_coordinate)) in
                owner_runs
                    .iter()
                    .zip(&reference_runs)
                    .zip(owner_coordinates)
                    .zip(reference_coordinates)
                    .enumerate()
            {
                let [owner_step_value] = owner_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} owner materialization {index} lost its one STEP coordinate"
                    ));
                };
                let [reference_step_value] = reference_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} reference materialization {index} lost its one STEP coordinate"
                    ));
                };
                if owner_step_value.to_bits() != owner_coordinate.to_bits()
                    || reference_step_value.to_bits() != reference_coordinate.to_bits()
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed its ordered STEP coordinates"
                    ));
                }
                let normalized_owner = Self::normalize_classic_mos_dtemp_materialization(
                    &owner_run.netlist,
                    XyceClassicMosDtempRole::Owner,
                    owner_coordinate,
                    reference_coordinate,
                )?;
                let normalized_reference = Self::normalize_classic_mos_dtemp_materialization(
                    &reference_run.netlist,
                    XyceClassicMosDtempRole::Reference,
                    reference_coordinate,
                    reference_coordinate,
                )?;
                let materialized_owner_snapshot = Self::classic_mos_dtemp_snapshot(
                    &contract.owner_plan,
                    &normalized_owner,
                    XyceClassicMosDtempRole::Owner,
                )?;
                let materialized_reference_snapshot = Self::classic_mos_dtemp_snapshot(
                    &contract.reference_plan,
                    &normalized_reference,
                    XyceClassicMosDtempRole::Reference,
                )?;
                if materialized_owner_snapshot != owner_snapshot
                    || materialized_reference_snapshot != reference_snapshot
                    || materialized_owner_snapshot != materialized_reference_snapshot
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed non-temperature topology or analysis semantics"
                    ));
                }
            }
            Ok((owner_runs, reference_runs))
        })();
        let (owner_runs, reference_runs) = match qualification {
            Ok(runs) => runs,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "{LABEL} '{}' qualification failed: {reason}",
                        contract.family
                    ),
                    Vec::new(),
                );
            }
        };

        let simulate = |plan: &XyceStaticDcPlan,
                        runs: &[XyceStepRun],
                        role: &str|
         -> Result<Vec<XycePrnTable>, String> {
            let engine = self.create_dc_engine();
            let mut batch_plan = plan.clone();
            batch_plan.steps.clear();
            let expected_rows = if contract.family.starts_with("nmos") {
                361
            } else {
                6
            };
            let mut tables = Vec::with_capacity(runs.len());
            for (index, run) in runs.iter().enumerate() {
                let results = engine
                    .run_dc_sweep2_spec_with_report_and_abort(
                        &run.netlist,
                        &plan.dc.source,
                        &plan.dc.primary_spec(),
                        plan.dc.sweep2.as_ref(),
                        &abort,
                    )
                    .map_err(|error| {
                        format!("{LABEL} {role} step {index} simulation failed: {error}")
                    })?;
                let table = self
                    .dc_results_to_prn_table(&batch_plan, &run.netlist, &results)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {index} output failed: {error}")
                    })?;
                if table.rows.len() != expected_rows
                    || table.columns.len() != 5
                    || table.columns[0] != "Index"
                    || table.rows.iter().enumerate().any(|(row, values)| {
                        values.len() != table.columns.len()
                            || values[0].to_bits() != (row as Value).to_bits()
                    })
                {
                    return Err(format!(
                        "{LABEL} {role} step {index} did not produce its canonical {expected_rows}-row indexed PRN batch"
                    ));
                }
                tables.push(table);
            }
            Ok(tables)
        };

        // Preserve the wrapper's process order: the live reference deck runs
        // to completion before the DTEMP owner deck.
        let reference_tables =
            match simulate(&contract.reference_plan, &reference_runs, "reference") {
                Ok(tables) => tables,
                Err(reason) => {
                    return self.failure_result(deck, start, result_contract, reason, Vec::new());
                }
            };
        let owner_tables = match simulate(&contract.owner_plan, &owner_runs, "owner") {
            Ok(tables) => tables,
            Err(reason) => {
                return self.failure_result(deck, start, result_contract, reason, Vec::new());
            }
        };

        let mut causal = false;
        'step_pairs: for pair in owner_tables.windows(2) {
            for (left_row, right_row) in pair[0].rows.iter().zip(&pair[1].rows) {
                for (&left, &right) in left_row.iter().skip(1).zip(right_row.iter().skip(1)) {
                    let left = match Self::xyce_default_prn_text(left) {
                        Ok(value) => value,
                        Err(reason) => {
                            return self.failure_result(
                                deck,
                                start,
                                result_contract,
                                format!("{LABEL} causal output serialization failed: {reason}"),
                                Vec::new(),
                            );
                        }
                    };
                    let right = match Self::xyce_default_prn_text(right) {
                        Ok(value) => value,
                        Err(reason) => {
                            return self.failure_result(
                                deck,
                                start,
                                result_contract,
                                format!("{LABEL} causal output serialization failed: {reason}"),
                                Vec::new(),
                            );
                        }
                    };
                    if left != right {
                        causal = true;
                        break 'step_pairs;
                    }
                }
            }
        }
        if !causal {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "{LABEL} '{}' is temperature-invariant at default PRN precision",
                    contract.family
                ),
                Vec::new(),
            );
        }

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (index, (reference, owner)) in reference_tables.iter().zip(&owner_tables).enumerate() {
            let mut step_mismatches = match self
                .compare_serialized_default_prn_tables(reference, owner)
            {
                Ok(found) => found,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} exact historical diff adapter failed for step {index}: {reason}"
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {index}: {}", mismatch.probe);
            }
            row_offset += owner.rows.len();
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
                    "{} {LABEL} exact serialized PRN mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_legacy_bjt_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceLegacyBjtDtempContract,
        start: Instant,
    ) -> XyceTestResult {
        const LABEL: &str = "legacy Gummel-Poon BJT TEMP/DTEMP family";
        let result_contract = contract.role.result_contract();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_legacy_bjt_dtemp_provenance(&contract)?;
            let parse = |plan: &XyceStaticDcPlan, role: &str| {
                Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
                    &plan.source,
                    &plan.deck_path,
                    plan.expression_dialect,
                    plan.parameter_redefinition_policy,
                    plan.execution_dir.as_deref(),
                )
                .map_err(|error| format!("{LABEL} {role} parse failed: {error}"))
            };
            let owner_netlist = parse(&contract.owner_plan, "owner")?;
            let reference_netlist = parse(&contract.reference_plan, "reference")?;
            let owner_snapshot = Self::legacy_bjt_dtemp_snapshot(
                &contract.owner_plan,
                &owner_netlist,
                contract.family,
                XyceLegacyBjtDtempRole::Owner,
            )?;
            let reference_snapshot = Self::legacy_bjt_dtemp_snapshot(
                &contract.reference_plan,
                &reference_netlist,
                contract.family,
                XyceLegacyBjtDtempRole::Reference,
            )?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "{LABEL} {} base owner/reference topology or analysis semantics differ",
                    contract.family.label()
                ));
            }

            let expansion_engine = self.create_dc_engine();
            let owner_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &owner_netlist,
                &contract.owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} owner STEP expansion failed: {error}"))?;
            let reference_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &reference_netlist,
                &contract.reference_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} reference STEP expansion failed: {error}"))?;
            let owner_coordinates: [Value; 3] = [0.0, 10.0, 20.0];
            let reference_coordinates: [Value; 3] = [15.0, 25.0, 35.0];
            if owner_runs.len() != owner_coordinates.len()
                || reference_runs.len() != reference_coordinates.len()
            {
                return Err(format!(
                    "{LABEL} requires three owner and three reference materializations, found {}/{}",
                    owner_runs.len(),
                    reference_runs.len()
                ));
            }
            for (index, (((owner_run, reference_run), owner_coordinate), reference_coordinate)) in
                owner_runs
                    .iter()
                    .zip(&reference_runs)
                    .zip(owner_coordinates)
                    .zip(reference_coordinates)
                    .enumerate()
            {
                let [owner_step_value] = owner_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} owner materialization {index} lost its one STEP coordinate"
                    ));
                };
                let [reference_step_value] = reference_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} reference materialization {index} lost its one STEP coordinate"
                    ));
                };
                if owner_step_value.to_bits() != owner_coordinate.to_bits()
                    || reference_step_value.to_bits() != reference_coordinate.to_bits()
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed its ordered STEP coordinates"
                    ));
                }
                let normalized_owner = Self::normalize_legacy_bjt_dtemp_materialization(
                    &owner_run.netlist,
                    XyceLegacyBjtDtempRole::Owner,
                    owner_coordinate,
                    reference_coordinate,
                )?;
                let normalized_reference = Self::normalize_legacy_bjt_dtemp_materialization(
                    &reference_run.netlist,
                    XyceLegacyBjtDtempRole::Reference,
                    reference_coordinate,
                    reference_coordinate,
                )?;
                let materialized_owner_snapshot = Self::legacy_bjt_dtemp_snapshot(
                    &contract.owner_plan,
                    &normalized_owner,
                    contract.family,
                    XyceLegacyBjtDtempRole::Owner,
                )?;
                let materialized_reference_snapshot = Self::legacy_bjt_dtemp_snapshot(
                    &contract.reference_plan,
                    &normalized_reference,
                    contract.family,
                    XyceLegacyBjtDtempRole::Reference,
                )?;
                if materialized_owner_snapshot != owner_snapshot
                    || materialized_reference_snapshot != reference_snapshot
                    || materialized_owner_snapshot != materialized_reference_snapshot
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed non-temperature topology or analysis semantics"
                    ));
                }
            }
            Ok((owner_runs, reference_runs))
        })();
        let (owner_runs, reference_runs) = match qualification {
            Ok(runs) => runs,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "{LABEL} {} qualification failed: {reason}",
                        contract.family.label()
                    ),
                    Vec::new(),
                );
            }
        };

        let simulate = |plan: &XyceStaticDcPlan,
                        runs: &[XyceStepRun],
                        role: &str|
         -> Result<Vec<XycePrnTable>, String> {
            let engine = self.create_dc_engine();
            let mut batch_plan = plan.clone();
            batch_plan.steps.clear();
            let (expected_rows, expected_columns) = match contract.family {
                XyceLegacyBjtDtempFamily::Npn => (
                    13,
                    ["Index", "v(4)", "i(vmon1)", "i(vmon2)", "v(1)", "v(2)"],
                ),
                XyceLegacyBjtDtempFamily::Pnp => (
                    30,
                    ["Index", "v(1)", "v(6)", "i(vmon1)", "i(vmon2)", "i(vmon3)"],
                ),
            };
            let mut tables = Vec::with_capacity(runs.len());
            for (step_index, run) in runs.iter().enumerate() {
                let results = engine
                    .run_dc_sweep2_spec_with_report_and_abort(
                        &run.netlist,
                        &plan.dc.source,
                        &plan.dc.primary_spec(),
                        plan.dc.sweep2.as_ref(),
                        &abort,
                    )
                    .map_err(|error| {
                        format!("{LABEL} {role} step {step_index} simulation failed: {error}")
                    })?;
                let table = self
                    .dc_results_to_prn_table(&batch_plan, &run.netlist, &results)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {step_index} output failed: {error}")
                    })?;
                if table.columns != expected_columns || table.rows.len() != expected_rows {
                    return Err(format!(
                        "{LABEL} {role} step {step_index} produced schema {:?} with {} rows, expected {expected_columns:?} with {expected_rows}",
                        table.columns,
                        table.rows.len()
                    ));
                }
                for (row_index, values) in table.rows.iter().enumerate() {
                    if values.len() != table.columns.len()
                        || values[0].to_bits() != (row_index as Value).to_bits()
                        || values.iter().any(|value| !value.is_finite())
                    {
                        return Err(format!(
                            "{LABEL} {role} step {step_index} row {row_index} is malformed: {values:?}"
                        ));
                    }
                    let sweep_coordinates_match = match contract.family {
                        XyceLegacyBjtDtempFamily::Npn => {
                            values[1].to_bits() == (row_index as Value).to_bits()
                        }
                        XyceLegacyBjtDtempFamily::Pnp => {
                            let vpos = (row_index % 6) as Value;
                            let outer_index = row_index / 6;
                            let vbb = if outer_index == 0 {
                                0.0
                            } else {
                                -0.5 * outer_index as Value
                            };
                            values[1].to_bits() == vpos.to_bits()
                                && values[2].to_bits() == vbb.to_bits()
                        }
                    };
                    if !sweep_coordinates_match {
                        return Err(format!(
                            "{LABEL} {role} step {step_index} row {row_index} lost canonical DC traversal: {values:?}"
                        ));
                    }
                }
                tables.push(table);
            }
            Ok(tables)
        };

        // Preserve the historical wrapper's process order: the live reference
        // deck runs to completion before the DTEMP owner deck.
        let reference_tables =
            match simulate(&contract.reference_plan, &reference_runs, "reference") {
                Ok(tables) => tables,
                Err(reason) => {
                    return self.failure_result(deck, start, result_contract, reason, Vec::new());
                }
            };
        let owner_tables = match simulate(&contract.owner_plan, &owner_runs, "owner") {
            Ok(tables) => tables,
            Err(reason) => {
                return self.failure_result(deck, start, result_contract, reason, Vec::new());
            }
        };

        // Lock default Xyce 7.10 PRN anchors so the owner/reference equality
        // check cannot pass vacuously and the Xyce-dialect solver contract is
        // exercised independently of the relational comparison.
        let (dependent_columns, expected_anchors): (&[usize], Vec<Vec<Value>>) =
            match contract.family {
                XyceLegacyBjtDtempFamily::Npn => (
                    &[2, 3, 4, 5],
                    vec![
                        vec![2.96613536e-5, 2.96613590e-3, 8.17669684e-1, 6.06772820],
                        vec![2.96951607e-5, 2.96951661e-3, 8.04924401e-1, 6.06096678],
                        vec![2.97291922e-5, 2.97291976e-3, 7.92094526e-1, 6.05416048],
                    ],
                ),
                XyceLegacyBjtDtempFamily::Pnp => (
                    &[3, 4, 5],
                    vec![
                        vec![2.04352027e-5, 1.22611228e-3, 1.24654748e-3],
                        vec![2.04974145e-5, 1.22984499e-3, 1.25034241e-3],
                        vec![2.05598872e-5, 1.23359337e-3, 1.25415326e-3],
                    ],
                ),
            };
        for &column in dependent_columns {
            let causal = (|| -> Result<bool, String> {
                for pair in owner_tables.windows(2) {
                    for (left, right) in pair[0].rows.iter().zip(&pair[1].rows) {
                        let left = Self::xyce_default_prn_text(left[column])?;
                        let right = Self::xyce_default_prn_text(right[column])?;
                        if left != right {
                            return Ok(true);
                        }
                    }
                }
                Ok(false)
            })();
            let causal = match causal {
                Ok(causal) => causal,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} {} dependent-column serialization failed: {reason}",
                            contract.family.label()
                        ),
                        Vec::new(),
                    );
                }
            };
            if !causal {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "{LABEL} {} dependent column '{}' is temperature-invariant at default PRN precision",
                        contract.family.label(),
                        owner_tables[0].columns[column]
                    ),
                    Vec::new(),
                );
            }
        }
        for (step_index, (table, expected)) in
            owner_tables.iter().zip(&expected_anchors).enumerate()
        {
            let Some(anchor) = table.rows.last() else {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{LABEL} owner step {step_index} has no maximum-bias row"),
                    Vec::new(),
                );
            };
            for (&column, &expected_value) in dependent_columns.iter().zip(expected) {
                let actual_text = match Self::xyce_default_prn_text(anchor[column]) {
                    Ok(value) => value,
                    Err(reason) => {
                        return self.failure_result(
                            deck,
                            start,
                            result_contract,
                            format!("{LABEL} anchor serialization failed: {reason}"),
                            Vec::new(),
                        );
                    }
                };
                let expected_text = Self::xyce_default_prn_text(expected_value)
                    .expect("finite locked Xyce BJT anchor");
                if actual_text != expected_text {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} {} step {step_index} maximum-bias '{}' is {actual_text}, expected Xyce 7.10 {expected_text}",
                            contract.family.label(),
                            table.columns[column]
                        ),
                        Vec::new(),
                    );
                }
            }
        }

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (step_index, (reference, owner)) in
            reference_tables.iter().zip(&owner_tables).enumerate()
        {
            let mut step_mismatches = match self
                .compare_serialized_default_prn_tables(reference, owner)
            {
                Ok(found) => found,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} exact historical diff adapter failed for step {step_index}: {reason}"
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {step_index}: {}", mismatch.probe);
            }
            row_offset += owner.rows.len();
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
                    "{} {LABEL} exact serialized PRN mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn run_xyce_sydney_level1_jfet_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceSydneyLevel1JfetDtempContract,
        start: Instant,
    ) -> XyceTestResult {
        const LABEL: &str = "Xyce Sydney level-1 JFET TEMP/DTEMP family";
        let result_contract = contract.role.result_contract();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_xyce_sydney_level1_jfet_dtemp_provenance(&contract)?;
            let parse = |plan: &XyceStaticDcPlan, role: &str| {
                Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
                    &plan.source,
                    &plan.deck_path,
                    plan.expression_dialect,
                    plan.parameter_redefinition_policy,
                    plan.execution_dir.as_deref(),
                )
                .map_err(|error| format!("{LABEL} {role} parse failed: {error}"))
            };
            let owner_netlist = parse(&contract.owner_plan, "owner")?;
            let reference_netlist = parse(&contract.reference_plan, "reference")?;
            let owner_snapshot = Self::xyce_sydney_level1_jfet_dtemp_snapshot(
                &contract.owner_plan,
                &owner_netlist,
                contract.family,
                XyceSydneyLevel1JfetDtempRole::Owner,
            )?;
            let reference_snapshot = Self::xyce_sydney_level1_jfet_dtemp_snapshot(
                &contract.reference_plan,
                &reference_netlist,
                contract.family,
                XyceSydneyLevel1JfetDtempRole::Reference,
            )?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "{LABEL} {} base owner/reference topology or analysis semantics differ",
                    contract.family.label()
                ));
            }

            let expansion_engine = self.create_dc_engine();
            let owner_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &owner_netlist,
                &contract.owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} owner STEP expansion failed: {error}"))?;
            let reference_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &reference_netlist,
                &contract.reference_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} reference STEP expansion failed: {error}"))?;
            let owner_coordinates: [Value; 3] = [-10.0, 0.0, 10.0];
            let reference_coordinates: [Value; 3] = [15.0, 25.0, 35.0];
            if owner_runs.len() != owner_coordinates.len()
                || reference_runs.len() != reference_coordinates.len()
            {
                return Err(format!(
                    "{LABEL} requires three owner and three reference materializations, found {}/{}",
                    owner_runs.len(),
                    reference_runs.len()
                ));
            }
            for (index, (((owner_run, reference_run), owner_coordinate), reference_coordinate)) in
                owner_runs
                    .iter()
                    .zip(&reference_runs)
                    .zip(owner_coordinates)
                    .zip(reference_coordinates)
                    .enumerate()
            {
                let [owner_step_value] = owner_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} owner materialization {index} lost its one STEP coordinate"
                    ));
                };
                let [reference_step_value] = reference_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} reference materialization {index} lost its one STEP coordinate"
                    ));
                };
                if owner_step_value.to_bits() != owner_coordinate.to_bits()
                    || reference_step_value.to_bits() != reference_coordinate.to_bits()
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed its ordered STEP coordinates"
                    ));
                }
                let normalized_owner =
                    Self::normalize_xyce_sydney_level1_jfet_dtemp_materialization(
                        &owner_run.netlist,
                        contract.family,
                        XyceSydneyLevel1JfetDtempRole::Owner,
                        owner_coordinate,
                        reference_coordinate,
                    )?;
                let normalized_reference =
                    Self::normalize_xyce_sydney_level1_jfet_dtemp_materialization(
                        &reference_run.netlist,
                        contract.family,
                        XyceSydneyLevel1JfetDtempRole::Reference,
                        reference_coordinate,
                        reference_coordinate,
                    )?;
                let materialized_owner_snapshot = Self::xyce_sydney_level1_jfet_dtemp_snapshot(
                    &contract.owner_plan,
                    &normalized_owner,
                    contract.family,
                    XyceSydneyLevel1JfetDtempRole::Owner,
                )?;
                let materialized_reference_snapshot = Self::xyce_sydney_level1_jfet_dtemp_snapshot(
                    &contract.reference_plan,
                    &normalized_reference,
                    contract.family,
                    XyceSydneyLevel1JfetDtempRole::Reference,
                )?;
                if materialized_owner_snapshot != owner_snapshot
                    || materialized_reference_snapshot != reference_snapshot
                    || materialized_owner_snapshot != materialized_reference_snapshot
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed non-temperature topology or analysis semantics"
                    ));
                }
            }
            Ok((owner_runs, reference_runs))
        })();
        let (owner_runs, reference_runs) = match qualification {
            Ok(runs) => runs,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "{LABEL} {} qualification failed: {reason}",
                        contract.family.label()
                    ),
                    Vec::new(),
                );
            }
        };

        let simulate = |plan: &XyceStaticDcPlan,
                        runs: &[XyceStepRun],
                        role: &str|
         -> Result<Vec<XycePrnTable>, String> {
            let engine = self.create_dc_engine();
            let mut batch_plan = plan.clone();
            batch_plan.steps.clear();
            let expected_columns = ["Index", "v(1)", "v(2)", "i(vidmon)"];
            let mut tables = Vec::with_capacity(runs.len());
            for (step_index, run) in runs.iter().enumerate() {
                let results = engine
                    .run_dc_sweep2_spec_with_report_and_abort(
                        &run.netlist,
                        &plan.dc.source,
                        &plan.dc.primary_spec(),
                        plan.dc.sweep2.as_ref(),
                        &abort,
                    )
                    .map_err(|error| {
                        format!("{LABEL} {role} step {step_index} simulation failed: {error}")
                    })?;
                let table = self
                    .dc_results_to_prn_table(&batch_plan, &run.netlist, &results)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {step_index} output failed: {error}")
                    })?;
                if table.columns != expected_columns || table.rows.len() != 64 {
                    return Err(format!(
                        "{LABEL} {role} step {step_index} produced schema {:?} with {} rows, expected {expected_columns:?} with 64",
                        table.columns,
                        table.rows.len()
                    ));
                }
                for (row_index, values) in table.rows.iter().enumerate() {
                    if values.len() != table.columns.len()
                        || values[0].to_bits() != (row_index as Value).to_bits()
                        || values.iter().any(|value| !value.is_finite())
                    {
                        return Err(format!(
                            "{LABEL} {role} step {step_index} row {row_index} is malformed: {values:?}"
                        ));
                    }
                    let primary_index = row_index % 4;
                    let secondary_index = row_index / 4;
                    let (expected_vds, expected_vgs) = match contract.family {
                        XyceSydneyLevel1JfetDtempFamily::Njf => (
                            secondary_index as Value,
                            [0.0, -0.625, -1.25, -1.875][primary_index],
                        ),
                        XyceSydneyLevel1JfetDtempFamily::Pjf => (
                            -15.0 + secondary_index as Value,
                            0.5 * primary_index as Value,
                        ),
                    };
                    if values[1].to_bits() != expected_vds.to_bits()
                        || values[2].to_bits() != expected_vgs.to_bits()
                    {
                        return Err(format!(
                            "{LABEL} {role} step {step_index} row {row_index} lost canonical primary-fast DC traversal: {values:?}"
                        ));
                    }
                }
                tables.push(table);
            }
            Ok(tables)
        };

        // Preserve the historical wrapper's process order: the live reference
        // deck runs to completion before the DTEMP owner deck. Each execution
        // emits three 64-row default-precision PRN batches.
        let reference_tables =
            match simulate(&contract.reference_plan, &reference_runs, "reference") {
                Ok(tables) => tables,
                Err(reason) => {
                    return self.failure_result(deck, start, result_contract, reason, Vec::new());
                }
            };
        let owner_tables = match simulate(&contract.owner_plan, &owner_runs, "owner") {
            Ok(tables) => tables,
            Err(reason) => {
                return self.failure_result(deck, start, result_contract, reason, Vec::new());
            }
        };

        let causal = (|| -> Result<bool, String> {
            for pair in owner_tables.windows(2) {
                for (left, right) in pair[0].rows.iter().zip(&pair[1].rows) {
                    if Self::xyce_default_prn_text(left[3])?
                        != Self::xyce_default_prn_text(right[3])?
                    {
                        return Ok(true);
                    }
                }
            }
            Ok(false)
        })();
        let causal = match causal {
            Ok(causal) => causal,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{LABEL} drain-current serialization failed: {reason}"),
                    Vec::new(),
                );
            }
        };
        if !causal {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!(
                    "{LABEL} {} drain current is temperature-invariant at default PRN precision",
                    contract.family.label()
                ),
                Vec::new(),
            );
        }

        // Independent Xyce 7.10 anchors prevent a mutually wrong
        // owner/reference pair from satisfying only the relational equality.
        // Row numbering follows Xyce's primary-sweep-fast traversal.
        let anchors: Vec<(usize, &'static str, [Value; 3])> = match contract.family {
            XyceSydneyLevel1JfetDtempFamily::Njf => vec![
                (
                    60,
                    "VDS=15,VGS=0",
                    [4.21529620e-4, 4.21709025e-4, 4.21893183e-4],
                ),
                (
                    63,
                    "VDS=15,VGS=-1.875",
                    [9.34645375e-5, 9.34898878e-5, 9.35161183e-5],
                ),
            ],
            XyceSydneyLevel1JfetDtempFamily::Pjf => vec![
                (
                    0,
                    "VDS=-15,VGS=0",
                    [-1.25565313e-3, -1.26233291e-3, -1.26923501e-3],
                ),
                (
                    63,
                    "VDS=0,VGS=1.5",
                    [-2.30892692e-11, -9.85396713e-11, -3.96063757e-10],
                ),
            ],
        };
        for (row_index, bias_label, expected_by_temperature) in anchors {
            for (step_index, (&expected, table)) in expected_by_temperature
                .iter()
                .zip(&owner_tables)
                .enumerate()
            {
                let actual_text = match Self::xyce_default_prn_text(table.rows[row_index][3]) {
                    Ok(value) => value,
                    Err(reason) => {
                        return self.failure_result(
                            deck,
                            start,
                            result_contract,
                            format!("{LABEL} anchor serialization failed: {reason}"),
                            Vec::new(),
                        );
                    }
                };
                let expected_text = Self::xyce_default_prn_text(expected)
                    .expect("finite locked Xyce Sydney level-1 JFET anchor");
                if actual_text != expected_text {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} {} step {step_index} {bias_label} i(vidmon) is {actual_text} (raw {:.17e}), expected Xyce 7.10 {expected_text}",
                            contract.family.label(),
                            table.rows[row_index][3],
                        ),
                        Vec::new(),
                    );
                }
            }
        }

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (step_index, (reference, owner)) in
            reference_tables.iter().zip(&owner_tables).enumerate()
        {
            let mut step_mismatches = match self
                .compare_serialized_default_prn_tables(reference, owner)
            {
                Ok(found) => found,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} exact historical diff adapter failed for step {step_index}: {reason}"
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {step_index}: {}", mismatch.probe);
            }
            row_offset += owner.rows.len();
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
                    "{} {LABEL} exact serialized PRN mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn level2_diode_dtemp_tables_are_temperature_causal(
        tables: &[XycePrnTable],
        stop: Value,
    ) -> Result<bool, String> {
        const LABEL: &str = "Level-2 diode TEMP/DTEMP family";
        if tables.len() != 3 {
            return Err(format!(
                "{LABEL} causality requires exactly three temperature tables, got {}",
                tables.len()
            ));
        }
        let expected_stop = Self::xyce_default_prn_text(stop)?;
        let mut final_values = Vec::with_capacity(tables.len());
        for (index, table) in tables.iter().enumerate() {
            if table.columns != ["Index", "TIME", "V(2)"] {
                return Err(format!(
                    "{LABEL} causal table {index} lost its Index/TIME/V(2) schema"
                ));
            }
            let final_row = table.rows.last().ok_or_else(|| {
                format!("{LABEL} causal table {index} has no final transient row")
            })?;
            if final_row.len() != table.columns.len() {
                return Err(format!(
                    "{LABEL} causal table {index} final row has the wrong width"
                ));
            }
            let final_time = Self::xyce_default_prn_text(final_row[1])?;
            if final_time != expected_stop {
                return Err(format!(
                    "{LABEL} causal table {index} ends at serialized TIME={final_time}, expected tstop={expected_stop}"
                ));
            }
            final_values.push(Self::xyce_default_prn_text(final_row[2])?);
        }
        Ok(final_values.windows(2).any(|pair| pair[0] != pair[1]))
    }

    pub(super) fn run_level2_diode_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceLevel2DiodeDtempContract,
        start: Instant,
    ) -> XyceTestResult {
        const LABEL: &str = "Level-2 diode TEMP/DTEMP family";
        let result_contract = contract.role.result_contract();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_level2_diode_dtemp_provenance(&contract)?;
            let owner_netlist = Self::parse_xyce_netlist(
                &contract.owner_plan.source,
                &contract.owner_plan.deck_path,
            )
            .map_err(|error| format!("{LABEL} owner parse failed: {error}"))?;
            let reference_netlist = Self::parse_xyce_netlist(
                &contract.reference_plan.source,
                &contract.reference_plan.deck_path,
            )
            .map_err(|error| format!("{LABEL} reference parse failed: {error}"))?;
            let owner_snapshot = Self::level2_diode_dtemp_snapshot(
                &contract.owner_plan,
                &owner_netlist,
                XyceLevel2DiodeDtempRole::Owner,
            )?;
            let reference_snapshot = Self::level2_diode_dtemp_snapshot(
                &contract.reference_plan,
                &reference_netlist,
                XyceLevel2DiodeDtempRole::Reference,
            )?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "{LABEL} base owner/reference topology or analysis semantics differ"
                ));
            }

            let expansion_engine = self.create_dc_engine();
            let owner_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &owner_netlist,
                &contract.owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} owner STEP expansion failed: {error}"))?;
            let reference_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &reference_netlist,
                &contract.reference_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} reference STEP expansion failed: {error}"))?;
            let owner_coordinates: [Value; 3] = [-82.0, -2.0, 45.0];
            let reference_coordinates: [Value; 3] = [-55.0, 25.0, 72.0];
            if owner_runs.len() != owner_coordinates.len()
                || reference_runs.len() != reference_coordinates.len()
            {
                return Err(format!(
                    "{LABEL} requires three owner and three reference materializations, found {}/{}",
                    owner_runs.len(),
                    reference_runs.len()
                ));
            }
            for (index, (((owner_run, reference_run), owner_coordinate), reference_coordinate)) in
                owner_runs
                    .iter()
                    .zip(&reference_runs)
                    .zip(owner_coordinates)
                    .zip(reference_coordinates)
                    .enumerate()
            {
                let [owner_step_value] = owner_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} owner materialization {index} lost its one STEP coordinate"
                    ));
                };
                let [reference_step_value] = reference_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} reference materialization {index} lost its one STEP coordinate"
                    ));
                };
                if owner_step_value.to_bits() != owner_coordinate.to_bits()
                    || reference_step_value.to_bits() != reference_coordinate.to_bits()
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed its ordered STEP coordinates"
                    ));
                }
                let normalized_owner = Self::normalize_level2_diode_dtemp_materialization(
                    &owner_run.netlist,
                    XyceLevel2DiodeDtempRole::Owner,
                    owner_coordinate,
                    reference_coordinate,
                )?;
                let normalized_reference = Self::normalize_level2_diode_dtemp_materialization(
                    &reference_run.netlist,
                    XyceLevel2DiodeDtempRole::Reference,
                    reference_coordinate,
                    reference_coordinate,
                )?;
                let materialized_owner_snapshot = Self::level2_diode_dtemp_snapshot(
                    &contract.owner_plan,
                    &normalized_owner,
                    XyceLevel2DiodeDtempRole::Owner,
                )?;
                let materialized_reference_snapshot = Self::level2_diode_dtemp_snapshot(
                    &contract.reference_plan,
                    &normalized_reference,
                    XyceLevel2DiodeDtempRole::Reference,
                )?;
                if materialized_owner_snapshot != owner_snapshot
                    || materialized_reference_snapshot != reference_snapshot
                    || materialized_owner_snapshot != materialized_reference_snapshot
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed non-temperature topology or analysis semantics"
                    ));
                }
            }
            Ok((owner_runs, reference_runs))
        })();
        let (owner_runs, reference_runs) = match qualification {
            Ok(runs) => runs,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{LABEL} qualification failed: {reason}"),
                    Vec::new(),
                );
            }
        };

        let simulate = |plan: &XyceStaticTranPlan,
                        runs: &[XyceStepRun],
                        role: &str|
         -> Result<Vec<XycePrnTable>, String> {
            let mut tables = Vec::with_capacity(runs.len());
            for (index, run) in runs.iter().enumerate() {
                let result = self
                    .run_transient_family_netlist(plan, &run.netlist, start, None, None)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {index} simulation failed: {error}")
                    })?;
                let table = Self::transient_family_result_to_prn_table(plan, &run.netlist, &result)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {index} output failed: {error}")
                    })?;
                if table.columns != ["Index", "TIME", "V(2)"]
                    || table.rows.len() < 2
                    || table.rows.first().is_none_or(|row| {
                        row.len() != table.columns.len()
                            || row[1].to_bits() != plan.tran.start.unwrap_or(0.0).to_bits()
                    })
                    || table.rows.last().is_none_or(|row| {
                        row.len() != table.columns.len()
                            || row[1].to_bits() != plan.tran.stop.to_bits()
                    })
                    || table.rows.iter().enumerate().any(|(row, values)| {
                        values.len() != table.columns.len()
                            || values[0].to_bits() != (row as Value).to_bits()
                            || values.iter().any(|value| !value.is_finite())
                            || values[1] < 0.0
                            || values[1] > 1.0
                    })
                    || table.rows.windows(2).any(|pair| pair[0][1] >= pair[1][1])
                {
                    return Err(format!(
                        "{LABEL} {role} step {index} did not produce a finite, time-ordered indexed Index/TIME/V(2) PRN batch spanning exact tstart through tstop"
                    ));
                }
                tables.push(table);
            }
            Ok(tables)
        };

        // Preserve the historical wrapper order: the live reference deck
        // runs to completion before the DTEMP owner deck.
        let reference_tables =
            match simulate(&contract.reference_plan, &reference_runs, "reference") {
                Ok(tables) => tables,
                Err(reason) => {
                    return self.failure_result(deck, start, result_contract, reason, Vec::new());
                }
            };
        let owner_tables = match simulate(&contract.owner_plan, &owner_runs, "owner") {
            Ok(tables) => tables,
            Err(reason) => {
                return self.failure_result(deck, start, result_contract, reason, Vec::new());
            }
        };

        // This gate proves that the paired TEMP/DTEMP controls are not both
        // ignored. It does not claim an independent absolute waveform oracle
        // for every temperature law on the locked model; the native diode's
        // TBV1/TBV2 formula and breakdown matching have direct device tests.
        let causal = match Self::level2_diode_dtemp_tables_are_temperature_causal(
            &owner_tables,
            contract.owner_plan.tran.stop,
        ) {
            Ok(causal) => causal,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{LABEL} causal output validation failed: {reason}"),
                    Vec::new(),
                );
            }
        };
        if !causal {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("{LABEL} is temperature-invariant at default PRN precision"),
                Vec::new(),
            );
        }

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (index, (reference, owner)) in reference_tables.iter().zip(&owner_tables).enumerate() {
            let mut step_mismatches = match self
                .compare_serialized_default_prn_tables(reference, owner)
            {
                Ok(found) => found,
                Err(reason) => {
                    return self.failure_result(
                            deck,
                            start,
                            result_contract,
                            format!(
                                "{LABEL} exact historical diff adapter failed for step {index}: {reason}"
                            ),
                            Vec::new(),
                        );
                }
            };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {index}: {}", mismatch.probe);
            }
            row_offset += owner.rows.len();
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
                    "{} {LABEL} exact serialized PRN mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    pub(super) fn capacitor_dtemp_tables_are_temperature_causal(
        tables: &[XycePrnTable],
        stop: Value,
    ) -> Result<bool, String> {
        const LABEL: &str = "capacitor TEMP/DTEMP family";
        if tables.len() != 3 {
            return Err(format!(
                "{LABEL} causality requires exactly three temperature tables, got {}",
                tables.len()
            ));
        }
        let expected_stop = Self::xyce_default_prn_text(stop)?;
        let mut final_values = Vec::with_capacity(tables.len());
        for (index, table) in tables.iter().enumerate() {
            if table.columns != ["Index", "TIME", "v(1)"] {
                return Err(format!(
                    "{LABEL} causal table {index} lost its Index/TIME/v(1) schema"
                ));
            }
            let final_row = table.rows.last().ok_or_else(|| {
                format!("{LABEL} causal table {index} has no final transient row")
            })?;
            if final_row.len() != table.columns.len() {
                return Err(format!(
                    "{LABEL} causal table {index} final row has the wrong width"
                ));
            }
            let final_time = Self::xyce_default_prn_text(final_row[1])?;
            if final_time != expected_stop {
                return Err(format!(
                    "{LABEL} causal table {index} ends at serialized TIME={final_time}, expected tstop={expected_stop}"
                ));
            }
            final_values.push(Self::xyce_default_prn_text(final_row[2])?);
        }
        Ok(final_values.windows(2).any(|pair| pair[0] != pair[1]))
    }

    pub(super) fn run_capacitor_dtemp_relational_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceCapacitorDtempContract,
        start: Instant,
    ) -> XyceTestResult {
        const LABEL: &str = "capacitor TEMP/DTEMP family";
        let result_contract = contract.role.result_contract();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_capacitor_dtemp_provenance(&contract)?;
            let owner_netlist = Self::parse_xyce_netlist(
                &contract.owner_plan.source,
                &contract.owner_plan.deck_path,
            )
            .map_err(|error| format!("{LABEL} owner parse failed: {error}"))?;
            let reference_netlist = Self::parse_xyce_netlist(
                &contract.reference_plan.source,
                &contract.reference_plan.deck_path,
            )
            .map_err(|error| format!("{LABEL} reference parse failed: {error}"))?;
            let owner_snapshot = Self::capacitor_dtemp_snapshot(
                &contract.owner_plan,
                &owner_netlist,
                XyceCapacitorDtempRole::Owner,
            )?;
            let reference_snapshot = Self::capacitor_dtemp_snapshot(
                &contract.reference_plan,
                &reference_netlist,
                XyceCapacitorDtempRole::Reference,
            )?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "{LABEL} base owner/reference topology or analysis semantics differ"
                ));
            }

            let expansion_engine = self.create_dc_engine();
            let owner_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &owner_netlist,
                &contract.owner_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} owner STEP expansion failed: {error}"))?;
            let reference_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &expansion_engine,
                &reference_netlist,
                &contract.reference_plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} reference STEP expansion failed: {error}"))?;
            let owner_coordinates: [Value; 3] = [600.0, 700.0, 800.0];
            let reference_coordinates: [Value; 3] = [627.0, 727.0, 827.0];
            let expected_capacitances: [Value; 3] = [1.8352e-6, 1.9303e-6, 2.0128e-6];
            if owner_runs.len() != owner_coordinates.len()
                || reference_runs.len() != reference_coordinates.len()
            {
                return Err(format!(
                    "{LABEL} requires three owner and three reference materializations, found {}/{}",
                    owner_runs.len(),
                    reference_runs.len()
                ));
            }
            let mut resolved_capacitances = Vec::with_capacity(owner_runs.len());
            for (
                index,
                ((((owner_run, reference_run), owner_coordinate), reference_coordinate), expected),
            ) in owner_runs
                .iter()
                .zip(&reference_runs)
                .zip(owner_coordinates)
                .zip(reference_coordinates)
                .zip(expected_capacitances)
                .enumerate()
            {
                let [owner_step_value] = owner_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} owner materialization {index} lost its one STEP coordinate"
                    ));
                };
                let [reference_step_value] = reference_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} reference materialization {index} lost its one STEP coordinate"
                    ));
                };
                if owner_step_value.to_bits() != owner_coordinate.to_bits()
                    || reference_step_value.to_bits() != reference_coordinate.to_bits()
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed its ordered STEP coordinates"
                    ));
                }

                let owner_capacitance = expansion_engine
                    .resolved_capacitor_value(&owner_run.netlist, "c1")
                    .map_err(|error| {
                        format!(
                            "{LABEL} owner materialization {index} capacitance resolution failed: {error}"
                        )
                    })?
                    .ok_or_else(|| {
                        format!(
                            "{LABEL} owner materialization {index} lost capacitor C1"
                        )
                    })?;
                let reference_capacitance = expansion_engine
                    .resolved_capacitor_value(&reference_run.netlist, "c1")
                    .map_err(|error| {
                        format!(
                            "{LABEL} reference materialization {index} capacitance resolution failed: {error}"
                        )
                    })?
                    .ok_or_else(|| {
                        format!(
                            "{LABEL} reference materialization {index} lost capacitor C1"
                        )
                    })?;
                if !owner_capacitance.is_finite()
                    || owner_capacitance <= 0.0
                    || owner_capacitance.to_bits() != reference_capacitance.to_bits()
                    || (owner_capacitance - expected).abs() >= 1.0e-18
                {
                    return Err(format!(
                        "{LABEL} materialization {index} resolved inconsistent capacitance: owner={owner_capacitance}, reference={reference_capacitance}, expected={expected}"
                    ));
                }
                resolved_capacitances.push(owner_capacitance);

                let normalized_owner = Self::normalize_capacitor_dtemp_materialization(
                    &owner_run.netlist,
                    XyceCapacitorDtempRole::Owner,
                    owner_coordinate,
                    reference_coordinate,
                )?;
                let normalized_reference = Self::normalize_capacitor_dtemp_materialization(
                    &reference_run.netlist,
                    XyceCapacitorDtempRole::Reference,
                    reference_coordinate,
                    reference_coordinate,
                )?;
                let materialized_owner_snapshot = Self::capacitor_dtemp_snapshot(
                    &contract.owner_plan,
                    &normalized_owner,
                    XyceCapacitorDtempRole::Owner,
                )?;
                let materialized_reference_snapshot = Self::capacitor_dtemp_snapshot(
                    &contract.reference_plan,
                    &normalized_reference,
                    XyceCapacitorDtempRole::Reference,
                )?;
                if materialized_owner_snapshot != owner_snapshot
                    || materialized_reference_snapshot != reference_snapshot
                    || materialized_owner_snapshot != materialized_reference_snapshot
                {
                    return Err(format!(
                        "{LABEL} materialization {index} changed non-temperature topology or analysis semantics"
                    ));
                }
            }
            if resolved_capacitances
                .windows(2)
                .any(|pair| pair[0].to_bits() == pair[1].to_bits())
            {
                return Err(format!(
                    "{LABEL} temperature polynomial produced repeated capacitance values"
                ));
            }
            Ok((owner_runs, reference_runs))
        })();
        let (owner_runs, reference_runs) = match qualification {
            Ok(runs) => runs,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{LABEL} qualification failed: {reason}"),
                    Vec::new(),
                );
            }
        };

        let simulate = |plan: &XyceStaticTranPlan,
                        runs: &[XyceStepRun],
                        role: &str|
         -> Result<Vec<XycePrnTable>, String> {
            let mut tables = Vec::with_capacity(runs.len());
            let expected_initial = Self::xyce_default_prn_text(1.0)?;
            for (index, run) in runs.iter().enumerate() {
                let result = self
                    .run_transient_family_netlist(plan, &run.netlist, start, None, None)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {index} simulation failed: {error}")
                    })?;
                let table = Self::transient_family_result_to_prn_table(plan, &run.netlist, &result)
                    .map_err(|error| {
                        format!("{LABEL} {role} step {index} output failed: {error}")
                    })?;
                let initial_value = table
                    .rows
                    .first()
                    .and_then(|row| row.get(2))
                    .copied()
                    .ok_or_else(|| {
                        format!("{LABEL} {role} step {index} has no initial V(1) sample")
                    })?;
                if table.columns != ["Index", "TIME", "v(1)"] {
                    return Err(format!(
                        "{LABEL} {role} step {index} produced unexpected PRN columns: {:?}",
                        table.columns
                    ));
                }
                if table.rows.len() < 2 {
                    return Err(format!(
                        "{LABEL} {role} step {index} produced only {} PRN row(s)",
                        table.rows.len()
                    ));
                }
                let first = &table.rows[0];
                let last = &table.rows[table.rows.len() - 1];
                let expected_start = plan.tran.start.unwrap_or(0.0);
                if first.len() != table.columns.len()
                    || first[1].to_bits() != expected_start.to_bits()
                {
                    return Err(format!(
                        "{LABEL} {role} step {index} PRN starts at {:?}, expected exact TIME={expected_start:e}",
                        first
                    ));
                }
                if last.len() != table.columns.len()
                    || last[1].to_bits() != plan.tran.stop.to_bits()
                {
                    return Err(format!(
                        "{LABEL} {role} step {index} PRN ends at {:?}, expected exact TIME={:e}",
                        last, plan.tran.stop
                    ));
                }
                let serialized_initial = Self::xyce_default_prn_text(initial_value)?;
                if serialized_initial != expected_initial {
                    return Err(format!(
                        "{LABEL} {role} step {index} PRN starts with V(1)={initial_value:e} ({serialized_initial}), expected IC=1 ({expected_initial})"
                    ));
                }
                for (row, values) in table.rows.iter().enumerate() {
                    if values.len() != table.columns.len()
                        || values[0].to_bits() != (row as Value).to_bits()
                        || values.iter().any(|value| !value.is_finite())
                        || values[1] < expected_start
                        || values[1] > plan.tran.stop
                    {
                        return Err(format!(
                            "{LABEL} {role} step {index} PRN row {row} is malformed or outside the transient interval: {values:?}"
                        ));
                    }
                }
                if let Some((row, pair)) = table
                    .rows
                    .windows(2)
                    .enumerate()
                    .find(|(_, pair)| pair[0][1] >= pair[1][1])
                {
                    return Err(format!(
                        "{LABEL} {role} step {index} PRN time is not strictly increasing between rows {row} and {}: {} then {}",
                        row + 1,
                        pair[0][1],
                        pair[1][1]
                    ));
                }
                tables.push(table);
            }
            Ok(tables)
        };

        // Preserve the historical wrapper order: the live reference deck
        // runs to completion before the DTEMP owner deck.
        let reference_tables =
            match simulate(&contract.reference_plan, &reference_runs, "reference") {
                Ok(tables) => tables,
                Err(reason) => {
                    return self.failure_result(deck, start, result_contract, reason, Vec::new());
                }
            };
        let owner_tables = match simulate(&contract.owner_plan, &owner_runs, "owner") {
            Ok(tables) => tables,
            Err(reason) => {
                return self.failure_result(deck, start, result_contract, reason, Vec::new());
            }
        };

        let causal = match Self::capacitor_dtemp_tables_are_temperature_causal(
            &owner_tables,
            contract.owner_plan.tran.stop,
        ) {
            Ok(causal) => causal,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{LABEL} causal output validation failed: {reason}"),
                    Vec::new(),
                );
            }
        };
        if !causal {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("{LABEL} is temperature-invariant at default PRN precision"),
                Vec::new(),
            );
        }

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (index, (reference, owner)) in reference_tables.iter().zip(&owner_tables).enumerate() {
            let mut step_mismatches = match self
                .compare_serialized_default_prn_tables(reference, owner)
            {
                Ok(found) => found,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} exact historical diff adapter failed for step {index}: {reason}"
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {index}: {}", mismatch.probe);
            }
            row_offset += owner.rows.len();
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
                    "{} {LABEL} exact serialized PRN mismatch(es)",
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

    pub(super) fn run_analytic_rc_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAnalyticRcContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.kind.result_contract();
        let label = contract.kind.label();
        let (netlist, result) =
            match self.run_transient_family_plan(&contract.plan, start, None, None) {
                Ok(result) => result,
                Err(SimulationError::Aborted) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{label} execution exceeded timeout ({}ms)",
                            self.config.max_time_per_test_ms,
                        ),
                        Vec::new(),
                    );
                }
                Err(err) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("{label} execution failed: {err}"),
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
                        result_contract,
                        format!("{label} output conversion failed: {err}"),
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
                result_contract,
                format!("{label} initial-condition validation failed: {err}"),
                Vec::new(),
            );
        }
        if let Err(err) =
            Self::validate_analytic_rc_complete_time_domain(&actual, contract.plan.tran.stop)
        {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("{label} output-domain validation failed: {err}"),
                Vec::new(),
            );
        }
        let reference = match Self::analytic_rc_reference_table(&actual, &contract.specification) {
            Ok(table) => table,
            Err(err) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("{label} reference generation failed: {err}"),
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
                    result_contract,
                    format!("{label} xyce_verify comparison failed: {err}"),
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
                format!("{} {label} xyce_verify mismatch(es)", mismatches.len(),),
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

    pub(super) fn run_nonlinear_core_model_step_reference_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceNonlinearCoreModelStepReferenceContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = if Self::same_path(&contract.target_path, &contract.owner_path) {
            XYCE_NONLINEAR_CORE_MODEL_STEP_WRAPPER_CONTRACT
        } else {
            XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT
        };
        let expansion_abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_nonlinear_core_model_step_provenance(&contract)?;
            let owner_plan = self.static_tran_family_plan_for_path(
                &contract.owner_path,
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            )?;
            if owner_plan.contract != XyceStaticTranContract::WrapperStatic
                || !matches!(&owner_plan.oracle, XyceStaticTranOracle::None)
                || owner_plan.output_override
                || owner_plan.timeint_conststep
                || owner_plan.wrapper_tolerance.is_some()
            {
                return Err(
                    "nonlinear CORE model-step owner requires one ordinary default PRN transient without a file oracle or wrapper tolerance"
                        .to_string(),
                );
            }
            Self::validate_nonlinear_core_model_step_source_directives(&owner_plan.source, true)?;
            if !owner_plan.tran.step.is_finite()
                || owner_plan.tran.step <= 0.0
                || !owner_plan.tran.stop.is_finite()
                || owner_plan.tran.stop <= owner_plan.tran.step
                || owner_plan.tran.start.is_some()
                || owner_plan.tran.max_step.is_some()
                || owner_plan.tran.uic
            {
                return Err(
                    "nonlinear CORE model-step owner requires one finite positive two-argument .TRAN without TSTART, DTMAX, or UIC"
                        .to_string(),
                );
            }
            let owner_scale = Self::tran_print_time_scale_factor(&owner_plan.source)?;
            if owner_scale.to_bits() != 1.0f64.to_bits() {
                return Err(
                    "nonlinear CORE model-step owner requires default transient output time units"
                        .to_string(),
                );
            }
            let owner_print = owner_plan.require_print("nonlinear CORE model-step owner")?;
            if owner_print.probes.len() != 5 {
                return Err(format!(
                    "nonlinear CORE model-step owner requires five ordered probes, found {}",
                    owner_print.probes.len()
                ));
            }
            if Self::logical_comp_directives(&owner_plan.source).len() != 3 {
                return Err(
                    "nonlinear CORE model-step owner requires exactly three historical *COMP directives"
                        .to_string(),
                );
            }
            let tolerances =
                Self::xyce_verify_comp_tolerances(&owner_plan.source, &owner_print.probes)?;
            let mut expected_tolerances =
                vec![XyceVerifyTransientTolerance::release_7_10_default(); 5];
            expected_tolerances[4].offset = 1.0e-3;
            if tolerances != expected_tolerances {
                return Err(format!(
                    "nonlinear CORE model-step owner changed the effective Release 7.10 *COMP policy: {tolerances:?}"
                ));
            }

            let [step] = owner_plan.steps.as_slice() else {
                return Err(format!(
                    "nonlinear CORE model-step owner requires exactly one .STEP command, found {}",
                    owner_plan.steps.len()
                ));
            };
            let Some(step_param) = step.param_name.as_deref() else {
                return Err(
                    "nonlinear CORE model-step owner requires an explicit model parameter"
                        .to_string(),
                );
            };
            if step.target != StepTarget::Device
                || !["area", "gap", "path"]
                    .iter()
                    .any(|name| step_param.eq_ignore_ascii_case(name))
                || !matches!(
                    step.sweep,
                    StepSweep::Linear {
                        start,
                        stop,
                        step,
                    } if start.is_finite()
                        && stop.is_finite()
                        && step.is_finite()
                        && start > 0.0
                        && stop > start
                        && step > 0.0
                )
            {
                return Err(
                    "nonlinear CORE model-step owner requires one finite positive linear AREA, GAP, or PATH device-target sweep"
                        .to_string(),
                );
            }
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_plan.deck_path)
                .map_err(|error| format!("owner netlist parse failed: {error}"))?;
            let owner_snapshot = Self::nonlinear_core_model_step_snapshot(&owner_netlist)?;
            if !step.name.eq_ignore_ascii_case(&owner_snapshot.model_name)
                || !owner_snapshot
                    .model_numeric_bits
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case(step_param))
            {
                return Err(format!(
                    "owner .STEP target '{}:{}' does not resolve to a declared geometry parameter on the unique CORE model '{}'",
                    step.name, step_param, owner_snapshot.model_name
                ));
            }
            let expected_probes = [
                format!("i({})", owner_snapshot.source_name),
                format!("{{v({})+0.2}}", owner_snapshot.inductor_signal_nodes[0]),
                format!("{{v({})+0.2}}", owner_snapshot.inductor_signal_nodes[1]),
                format!("i({})", owner_snapshot.inductor_names[0]),
                format!("i({})", owner_snapshot.inductor_names[1]),
            ]
            .map(|probe| Self::normalize_probe(&probe));
            let actual_probes = owner_print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .collect::<Vec<_>>();
            if actual_probes.as_slice() != expected_probes.as_slice() {
                return Err(format!(
                    "nonlinear CORE model-step owner changed its ordered source/offset-voltage/winding-current probes: {actual_probes:?}"
                ));
            }

            let step_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &self.create_xyce_engine(),
                &owner_netlist,
                &owner_plan.steps,
                xyce_step_plan_limits(),
                &expansion_abort,
            )
            .map_err(|error| match error {
                SimulationError::Aborted => format!(
                    "owner .STEP expansion exceeded timeout ({}ms)",
                    self.config.max_time_per_test_ms
                ),
                error => format!("owner .STEP expansion failed: {error}"),
            })?;
            if step_runs.len() != 3 || step_runs.len() != contract.member_paths.len() {
                return Err(format!(
                    "owner .STEP expands to {} run(s), but exactly three independent controls are required",
                    step_runs.len()
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
                if member_plan.contract != XyceStaticTranContract::PlainStatic
                    || !matches!(&member_plan.oracle, XyceStaticTranOracle::None)
                    || !member_plan.steps.is_empty()
                    || member_plan.output_override
                    || member_plan.timeint_conststep
                    || member_plan.wrapper_tolerance.is_some()
                {
                    return Err(format!(
                        "independent control {} must be one ordinary non-stepped default PRN transient without a file oracle",
                        self.display_path(member_path)
                    ));
                }
                Self::validate_nonlinear_core_model_step_source_directives(
                    &member_plan.source,
                    false,
                )?;
                let member_print =
                    member_plan.require_print("independent nonlinear CORE control")?;
                if member_print.probes != owner_print.probes {
                    return Err(format!(
                        "independent control {} changes the ordered .PRINT TRAN probes",
                        self.display_path(member_path)
                    ));
                }
                if !Self::tran_analyses_match_exactly(&owner_plan.tran, &member_plan.tran) {
                    return Err(format!(
                        "independent control {} changes the .TRAN analysis tuple",
                        self.display_path(member_path)
                    ));
                }
                if Self::tran_print_time_scale_factor(&member_plan.source)?.to_bits()
                    != owner_scale.to_bits()
                {
                    return Err(format!(
                        "independent control {} changes transient output time units",
                        self.display_path(member_path)
                    ));
                }
                if Self::logical_comp_directives(&member_plan.source).len() != 2 {
                    return Err(format!(
                        "independent control {} must retain exactly two valid but unreferenced historical voltage *COMP directives",
                        self.display_path(member_path)
                    ));
                }
                let comp_result =
                    Self::xyce_verify_comp_tolerances(&member_plan.source, &member_print.probes);
                match comp_result {
                    Err(error) if error == XYCE_VERIFY_COMP_NO_PRINTED_PROBE => {}
                    Err(error) => {
                        return Err(format!(
                            "independent control {} has invalid historical *COMP directives: {error}",
                            self.display_path(member_path)
                        ));
                    }
                    Ok(_) => {
                        return Err(format!(
                            "independent control {} changed a historical voltage *COMP directive to target a printed probe",
                            self.display_path(member_path)
                        ));
                    }
                }

                let member_netlist =
                    Self::parse_xyce_netlist(&member_plan.source, &member_plan.deck_path).map_err(
                        |error| {
                            format!(
                                "independent control {} parse failed: {error}",
                                self.display_path(member_path)
                            )
                        },
                    )?;
                let stepped_snapshot = Self::nonlinear_core_model_step_snapshot(&step_run.netlist)?;
                let member_snapshot = Self::nonlinear_core_model_step_snapshot(&member_netlist)?;
                if stepped_snapshot != member_snapshot {
                    return Err(format!(
                        "owner step {index} is not structurally and numerically identical to independent control {}",
                        self.display_path(member_path)
                    ));
                }
                let [step_value] = step_run.step_values.as_slice() else {
                    return Err(format!(
                        "owner step {index} did not retain exactly one swept value"
                    ));
                };
                let stepped_parameter_bits = stepped_snapshot
                    .model_numeric_bits
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(step_param))
                    .map(|(_, bits)| *bits)
                    .ok_or_else(|| {
                        format!("owner step {index} lost swept CORE parameter {step_param}")
                    })?;
                if stepped_parameter_bits != step_value.to_bits() {
                    return Err(format!(
                        "owner step {index} retained value {step_value}, but its materialized CORE parameter {step_param} has different bits"
                    ));
                }
                members.push((member_plan, member_netlist));
            }
            Ok((owner_plan, step_runs, members, tolerances))
        })();

        let (owner_plan, step_runs, members, tolerances) = match qualification {
            Ok(qualified) => qualified,
            Err(reason) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "nonlinear CORE model-step family '{}' qualification failed: {reason}",
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
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("independent control step {index} simulation failed: {error}"),
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
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("owner step {index} simulation failed: {error}"),
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
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("independent control step {index} output failed: {error}"),
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
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("owner step {index} output failed: {error}"),
                        Vec::new(),
                    );
                }
            };
            let mut step_mismatches = match self
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &baseline_table,
                    &stepped_table,
                    &tolerances,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                ) {
                Ok(found) => found,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "Release 7.10 xyce_verify comparison failed for step {index}: {error}"
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
                    "{} nonlinear CORE model-step Release 7.10 xyce_verify mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }

    fn bug1190_normalized_parameter_alias(expression: &str) -> String {
        let compact = expression
            .chars()
            .filter(|character| !character.is_ascii_whitespace())
            .collect::<String>();
        compact
            .strip_prefix('{')
            .and_then(|inner| inner.strip_suffix('}'))
            .unwrap_or(&compact)
            .to_ascii_lowercase()
    }

    fn validate_bug1190_parameter_alias_source(
        source: &str,
        expect_alias: bool,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG 1190 mutual-inductor parameter-alias family";
        let mut subcircuit_depth = 0usize;
        let mut aliases = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let fields = Self::split_grouped_whitespace_fields(stripped, LABEL)?;
            let Some(command) = fields.first() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".subckt") {
                subcircuit_depth = subcircuit_depth
                    .checked_add(1)
                    .ok_or_else(|| format!("{LABEL} subcircuit nesting depth overflow"))?;
                continue;
            }
            if command.eq_ignore_ascii_case(".ends") {
                subcircuit_depth = subcircuit_depth
                    .checked_sub(1)
                    .ok_or_else(|| format!("{LABEL} has .ENDS without .SUBCKT"))?;
                continue;
            }
            if subcircuit_depth != 0 || !command.eq_ignore_ascii_case(".param") {
                continue;
            }
            if fields.len() != 2 {
                return Err(format!(
                    "{LABEL} top-level .PARAM must contain exactly one assignment"
                ));
            }
            let (name, expression) = fields[1].split_once('=').ok_or_else(|| {
                format!("{LABEL} top-level .PARAM must use NAME=EXPRESSION syntax")
            })?;
            if !name.eq_ignore_ascii_case("p_scalefac")
                || Self::bug1190_normalized_parameter_alias(expression) != "scalefac"
            {
                return Err(format!(
                    "{LABEL} admits only the exact top-level `P_SCALEFAC={{SCALEFAC}}` alias"
                ));
            }
            aliases += 1;
        }
        if subcircuit_depth != 0 {
            return Err(format!("{LABEL} contains an unterminated .SUBCKT"));
        }
        let expected = usize::from(expect_alias);
        if aliases != expected {
            return Err(format!(
                "{LABEL} requires {expected} top-level P_SCALEFAC alias definition(s), found {aliases}"
            ));
        }
        Ok(())
    }

    fn validate_bug1190_prn_batch(
        table: &XycePrnTable,
        expected_columns: &[String],
        expected_stop: Value,
        role: &str,
    ) -> Result<(), String> {
        if table.columns != expected_columns {
            return Err(format!(
                "{role} changed the exact default-PRN columns: expected {expected_columns:?}, got {:?}",
                table.columns
            ));
        }
        if table.rows.len() < 2 {
            return Err(format!(
                "{role} default-PRN batch requires at least two rows, found {}",
                table.rows.len()
            ));
        }

        let mut previous_raw_time = None;
        let mut previous_printed_time = None;
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != expected_columns.len() {
                return Err(format!(
                    "{role} default-PRN row {row_index} has {} columns, expected {}",
                    row.len(),
                    expected_columns.len()
                ));
            }
            let serialized = row
                .iter()
                .enumerate()
                .map(|(column_index, value)| {
                    Self::xyce_default_prn_roundtrip(*value).map_err(|error| {
                        format!(
                            "{role} default-PRN row {row_index} column '{}' is not serializable: {error}",
                            expected_columns[column_index]
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            let expected_index = row_index as Value;
            if serialized[0].to_bits() != expected_index.to_bits() {
                return Err(format!(
                    "{role} default-PRN Index does not reset to zero and advance consecutively at row {row_index}: {}",
                    serialized[0]
                ));
            }
            let time = serialized[1];
            if row_index == 0 && time.to_bits() != 0.0f64.to_bits() {
                return Err(format!(
                    "{role} default-PRN batch must begin at TIME=0, got {time}"
                ));
            }
            if previous_raw_time.is_some_and(|previous| row[1] <= previous) {
                return Err(format!(
                    "{role} transient TIME does not increase strictly at row {row_index}: {}",
                    row[1]
                ));
            }
            if previous_printed_time.is_some_and(|previous| time < previous) {
                return Err(format!(
                    "{role} serialized default-PRN TIME decreases at row {row_index}: {time}"
                ));
            }
            previous_raw_time = Some(row[1]);
            previous_printed_time = Some(time);
        }
        let expected_stop = Self::xyce_default_prn_roundtrip(expected_stop)?;
        let actual_stop = previous_printed_time.expect("two-row table has a final TIME");
        if actual_stop.to_bits() != expected_stop.to_bits() {
            return Err(format!(
                "{role} default-PRN batch ends at TIME={actual_stop}, expected .TRAN stop {expected_stop}"
            ));
        }
        Ok(())
    }

    pub(super) fn run_bug1190_mutual_inductor_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug1190MutualInductorContract,
        start: Instant,
    ) -> XyceTestResult {
        const LABEL: &str = "BUG 1190 mutual-inductor parameter-alias family";
        let result_contract = if Self::same_path(&contract.target_path, &contract.owner_path) {
            XYCE_BUG1190_MUTUAL_INDUCTOR_WRAPPER_CONTRACT
        } else {
            XYCE_BUG1190_MUTUAL_INDUCTOR_BASELINE_CONTRACT
        };
        let expansion_abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let qualification = (|| {
            self.validate_bug1190_mutual_inductor_provenance(&contract)?;
            let owner_plan = self.static_tran_family_plan_for_path(
                &contract.owner_path,
                XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily,
            )?;
            let baseline_plan = self.static_tran_family_plan_for_path(
                &contract.baseline_path,
                XyceStaticTranPlanPurpose::RelationalFamily,
            )?;
            if owner_plan.contract != XyceStaticTranContract::WrapperStatic
                || baseline_plan.contract != XyceStaticTranContract::PlainStatic
                || !matches!(&owner_plan.oracle, XyceStaticTranOracle::None)
                || !matches!(&baseline_plan.oracle, XyceStaticTranOracle::None)
                || owner_plan.output_override
                || baseline_plan.output_override
                || owner_plan.timeint_conststep
                || baseline_plan.timeint_conststep
                || owner_plan.wrapper_tolerance.is_some()
                || baseline_plan.wrapper_tolerance.is_some()
            {
                return Err(format!(
                    "{LABEL} requires one wrapper owner and one plain baseline using ordinary default PRN output without file or tolerance overrides"
                ));
            }
            if !Self::tran_analyses_match_exactly(&owner_plan.tran, &baseline_plan.tran)
                || owner_plan.tran.step.to_bits() != 0.0f64.to_bits()
                || !owner_plan.tran.stop.is_finite()
                || owner_plan.tran.stop <= 0.0
                || owner_plan.tran.start.is_some()
                || owner_plan.tran.max_step.is_some()
                || owner_plan.tran.uic
            {
                return Err(format!(
                    "{LABEL} owner and baseline require the same finite positive `.TRAN 0 TSTOP` analysis without TSTART, DTMAX, or UIC"
                ));
            }
            for (role, source) in [
                ("owner", owner_plan.source.as_str()),
                ("baseline", baseline_plan.source.as_str()),
            ] {
                let time_scale = Self::tran_print_time_scale_factor(source)?;
                if time_scale.to_bits() != 1.0f64.to_bits() {
                    return Err(format!(
                        "{LABEL} {role} requires default transient output time units"
                    ));
                }
            }

            let owner_print = owner_plan.require_print("BUG 1190 owner")?;
            let baseline_print = baseline_plan.require_print("BUG 1190 baseline")?;
            if owner_print.probes != baseline_print.probes || owner_print.probes.len() != 2 {
                return Err(format!(
                    "{LABEL} owner and baseline require the same two ordered voltage probes"
                ));
            }
            let expected_columns = Self::transient_prn_header_columns(owner_print, true);

            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_plan.deck_path)
                .map_err(|error| format!("{LABEL} owner parse failed: {error}"))?;
            let baseline_netlist =
                Self::parse_xyce_netlist(&baseline_plan.source, &baseline_plan.deck_path)
                    .map_err(|error| format!("{LABEL} baseline parse failed: {error}"))?;
            let owner_snapshot = Self::bug1190_mutual_inductor_snapshot(&owner_netlist)?;
            let baseline_snapshot = Self::bug1190_mutual_inductor_snapshot(&baseline_netlist)?;
            if owner_snapshot != baseline_snapshot {
                return Err(format!(
                    "{LABEL} owner and baseline are not structurally and numerically identical at their nominal parameter values"
                ));
            }

            let (expected_steps, expected_stop, expected_probes, nominal_swept_bits): (
                Vec<Value>,
                Value,
                [&str; 2],
                Vec<u64>,
            ) = match owner_snapshot.kind {
                XyceBug1190MutualInductorKind::Linear => (
                    vec![0.5, 1.0, 2.0],
                    0.1e-3,
                    ["v(node1)", "v(node2)"],
                    vec![4.65e-6f64.to_bits()],
                ),
                XyceBug1190MutualInductorKind::NonlinearCore => (
                    vec![0.5, 0.75, 1.0],
                    10.0e-3,
                    ["v(node1)", "v(node6)"],
                    vec![100.0f64.to_bits(), 100.0f64.to_bits()],
                ),
            };
            if owner_plan.tran.stop.to_bits() != expected_stop.to_bits()
                || owner_snapshot.swept_inductor_bits != nominal_swept_bits
            {
                return Err(format!(
                    "{LABEL} changed its qualified transient domain or nominal swept winding values"
                ));
            }
            let actual_probes = owner_print
                .probes
                .iter()
                .map(|probe| Self::normalize_probe(probe))
                .collect::<Vec<_>>();
            let expected_probes = expected_probes.map(Self::normalize_probe);
            if actual_probes.as_slice() != expected_probes.as_slice() {
                return Err(format!(
                    "{LABEL} changed its ordered voltage probes: {actual_probes:?}"
                ));
            }

            let [owner_step] = owner_plan.steps.as_slice() else {
                return Err(format!(
                    "{LABEL} owner requires exactly one .STEP command, found {}",
                    owner_plan.steps.len()
                ));
            };
            let [baseline_step] = baseline_plan.steps.as_slice() else {
                return Err(format!(
                    "{LABEL} baseline requires exactly one .STEP command, found {}",
                    baseline_plan.steps.len()
                ));
            };
            let validate_step = |role: &str, step: &StepCommand| -> Result<(), String> {
                let StepSweep::List(values) = &step.sweep else {
                    return Err(format!("{LABEL} {role} requires a LIST parameter sweep"));
                };
                if step.target != StepTarget::Param
                    || !step.name.eq_ignore_ascii_case("scalefac")
                    || step.param_name.is_some()
                    || values.len() != expected_steps.len()
                    || values
                        .iter()
                        .zip(&expected_steps)
                        .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
                {
                    return Err(format!(
                        "{LABEL} {role} requires the exact ordered SCALEFAC LIST sweep {expected_steps:?}"
                    ));
                }
                Ok(())
            };
            validate_step("owner", owner_step)?;
            validate_step("baseline", baseline_step)?;

            Self::validate_bug1190_parameter_alias_source(&owner_plan.source, true)?;
            Self::validate_bug1190_parameter_alias_source(&baseline_plan.source, false)?;
            if !owner_netlist.params.has_parameter_binding("p_scalefac")
                || owner_netlist.params.get("p_scalefac").map(Value::to_bits)
                    != Some(1.0f64.to_bits())
                || baseline_netlist.params.has_parameter_binding("p_scalefac")
            {
                return Err(format!(
                    "{LABEL} requires the owner-only ordinary `P_SCALEFAC={{SCALEFAC}}` alias"
                ));
            }

            let engine = self.create_xyce_engine();
            let owner_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &engine,
                &owner_netlist,
                &owner_plan.steps,
                xyce_step_plan_limits(),
                &expansion_abort,
            )
            .map_err(|error| format!("{LABEL} owner .STEP expansion failed: {error}"))?;
            let baseline_runs = Self::nested_step_runs_for_commands_with_limits_and_abort(
                &engine,
                &baseline_netlist,
                &baseline_plan.steps,
                xyce_step_plan_limits(),
                &expansion_abort,
            )
            .map_err(|error| format!("{LABEL} baseline .STEP expansion failed: {error}"))?;
            if owner_runs.len() != expected_steps.len()
                || baseline_runs.len() != expected_steps.len()
            {
                return Err(format!(
                    "{LABEL} requires exactly {} owner and baseline materializations, found {}/{}",
                    expected_steps.len(),
                    owner_runs.len(),
                    baseline_runs.len()
                ));
            }

            let mut materialized_windings = BTreeSet::new();
            for (index, ((owner_run, baseline_run), expected_step)) in owner_runs
                .iter()
                .zip(&baseline_runs)
                .zip(&expected_steps)
                .enumerate()
            {
                let [owner_value] = owner_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} owner run {index} lost its step coordinate"
                    ));
                };
                let [baseline_value] = baseline_run.step_values.as_slice() else {
                    return Err(format!(
                        "{LABEL} baseline run {index} lost its step coordinate"
                    ));
                };
                if owner_value.to_bits() != expected_step.to_bits()
                    || baseline_value.to_bits() != expected_step.to_bits()
                    || owner_run.netlist.params.get("scalefac").map(Value::to_bits)
                        != Some(expected_step.to_bits())
                    || owner_run
                        .netlist
                        .params
                        .get("p_scalefac")
                        .map(Value::to_bits)
                        != Some(expected_step.to_bits())
                    || baseline_run
                        .netlist
                        .params
                        .get("scalefac")
                        .map(Value::to_bits)
                        != Some(expected_step.to_bits())
                    || baseline_run
                        .netlist
                        .params
                        .has_parameter_binding("p_scalefac")
                {
                    return Err(format!(
                        "{LABEL} run {index} did not preserve the exact SCALEFAC/P_SCALEFAC parameter semantics"
                    ));
                }

                let owner_materialized =
                    Self::bug1190_mutual_inductor_snapshot(&owner_run.netlist)?;
                let baseline_materialized =
                    Self::bug1190_mutual_inductor_snapshot(&baseline_run.netlist)?;
                if owner_materialized != baseline_materialized
                    || owner_materialized.kind != owner_snapshot.kind
                {
                    return Err(format!(
                        "{LABEL} owner and baseline materialization {index} are not exactly equivalent"
                    ));
                }
                let expected_windings = match owner_snapshot.kind {
                    XyceBug1190MutualInductorKind::Linear => {
                        vec![(4.65e-6 * expected_step).to_bits()]
                    }
                    XyceBug1190MutualInductorKind::NonlinearCore => vec![
                        (100.0 * expected_step).to_bits(),
                        (100.0 * expected_step).to_bits(),
                    ],
                };
                if owner_materialized.swept_inductor_bits != expected_windings {
                    return Err(format!(
                        "{LABEL} run {index} did not materialize the exact swept winding values"
                    ));
                }
                materialized_windings.insert(owner_materialized.swept_inductor_bits);
            }
            if materialized_windings.len() != expected_steps.len() {
                return Err(format!(
                    "{LABEL} .STEP coordinates do not cause three distinct winding configurations"
                ));
            }

            Ok((
                owner_plan,
                baseline_plan,
                owner_runs,
                baseline_runs,
                expected_columns,
                expected_stop,
            ))
        })();

        let (owner_plan, baseline_plan, owner_runs, baseline_runs, expected_columns, expected_stop) =
            match qualification {
                Ok(qualified) => qualified,
                Err(reason) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} '{}' qualification failed: {reason}",
                            contract.family
                        ),
                        Vec::new(),
                    );
                }
            };

        let mut mismatches = Vec::new();
        let mut row_offset = 0usize;
        for (index, (owner_run, baseline_run)) in owner_runs.iter().zip(&baseline_runs).enumerate()
        {
            let baseline_result = match self.run_transient_family_netlist(
                &baseline_plan,
                &baseline_run.netlist,
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
                        format!("{LABEL} baseline step {index} simulation failed: {error}"),
                        Vec::new(),
                    );
                }
            };
            let owner_result = match self.run_transient_family_netlist(
                &owner_plan,
                &owner_run.netlist,
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
                        format!("{LABEL} owner step {index} simulation failed: {error}"),
                        Vec::new(),
                    );
                }
            };
            let baseline_table = match Self::transient_family_result_to_prn_table(
                &baseline_plan,
                &baseline_run.netlist,
                &baseline_result,
            ) {
                Ok(table) => table,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("{LABEL} baseline step {index} output failed: {error}"),
                        Vec::new(),
                    );
                }
            };
            let owner_table = match Self::transient_family_result_to_prn_table(
                &owner_plan,
                &owner_run.netlist,
                &owner_result,
            ) {
                Ok(table) => table,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("{LABEL} owner step {index} output failed: {error}"),
                        Vec::new(),
                    );
                }
            };
            for (role, table) in [("baseline", &baseline_table), ("owner", &owner_table)] {
                if let Err(error) =
                    Self::validate_bug1190_prn_batch(table, &expected_columns, expected_stop, role)
                {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("{LABEL} step {index} output contract failed: {error}"),
                        Vec::new(),
                    );
                }
            }
            let mut step_mismatches = match self.compare_release_7_10_file_compare_tables(
                &baseline_table,
                &owner_table,
                XyceFileCompareTolerance::BUG1190_MUTUAL_INDUCTOR,
            ) {
                Ok(found) => found,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!(
                            "{LABEL} Release 7.10 file_compare failed for step {index}: {error}"
                        ),
                        Vec::new(),
                    );
                }
            };
            for mismatch in &mut step_mismatches {
                mismatch.row += row_offset;
                mismatch.probe = format!("step {index}: {}", mismatch.probe);
            }
            row_offset += owner_table.rows.len();
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
                    "{} {LABEL} Release 7.10 file_compare mismatch(es)",
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
                if member_plan
                    .require_print("independent stepped transient baseline")?
                    .probes
                    != owner_plan.require_print("stepped transient owner")?.probes
                {
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

    pub(super) fn run_params1_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceParams1FamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_params1_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("PARAMS1 family provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }

        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_params1_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("PARAMS1 family provenance changed during execution: {reason}"),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
        }
        result
    }

    pub(super) fn run_naked_algebra_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceNakedAlgebraFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_naked_algebra_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("nakedAlgebra family provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }

        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_naked_algebra_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("nakedAlgebra family provenance changed during execution: {reason}"),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
        }
        result
    }

    pub(super) fn run_bug1826_thermal_parameter_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug1826ThermalParameterFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_bug1826_thermal_parameter_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("BUG 1826 family provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }

        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_bug1826_thermal_parameter_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 1826 family provenance changed during execution: {reason}"),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
        }
        result
    }

    pub(super) fn run_source_multiplicity_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceSourceMultiplicityFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_source_multiplicity_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("source-multiplicity provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }
        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_source_multiplicity_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("source-multiplicity provenance changed during execution: {reason}"),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
        }
        result
    }

    pub(super) fn run_abm_lookup_order_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAbmLookupOrderFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_abm_lookup_order_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("ABM_SPLINES lookup-order provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }
        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_abm_lookup_order_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!(
                        "ABM_SPLINES lookup-order provenance changed during execution: {reason}"
                    ),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
        }
        result
    }

    pub(super) fn run_bug38_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug38FamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_bug38_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("BUG_38_SON provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }
        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_bug38_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG_38_SON provenance changed during execution: {reason}"),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
        }
        result
    }

    pub(super) fn run_abm_frequency_family_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceAbmFrequencyFamilyContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        if let Err(reason) = self.validate_abm_frequency_provenance(&contract) {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("ABM_FREQ provenance changed before execution: {reason}"),
                Vec::new(),
            );
        }
        let mut result =
            self.run_baseline_family_contract(deck, contract.relational.clone(), start);
        if result.passed && !result.expected_unsupported {
            if let Err(reason) = self.validate_abm_frequency_provenance(&contract) {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("ABM_FREQ provenance changed during execution: {reason}"),
                    Vec::new(),
                );
            }
            result.contract = result_contract.to_string();
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
        if matches!(
            contract.kind,
            XyceBaselineFamilyKind::AbmFrequency | XyceBaselineFamilyKind::AcAnalysisExpression
        ) && analysis != XyceBaselineFamilyAnalysis::Ac
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
                | XyceBaselineFamilyKind::AbmLookupOrder
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
                | XyceBaselineFamilyKind::Params1
                | XyceBaselineFamilyKind::NakedAlgebra
                | XyceBaselineFamilyKind::Bug1826ThermalParameter
                | XyceBaselineFamilyKind::SwitchStateCase
                | XyceBaselineFamilyKind::SinExpression
                | XyceBaselineFamilyKind::ParamExpression
                | XyceBaselineFamilyKind::PassiveCapPrimaryValue
                | XyceBaselineFamilyKind::PassiveTemperatureOverride
                | XyceBaselineFamilyKind::TransientAnalysisExpression
                | XyceBaselineFamilyKind::Bug38SubcktFormalParentheses
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
        if contract.kind == XyceBaselineFamilyKind::SourceMultiplicity
            && let Err(reason) = Self::validate_source_multiplicity_dc_plan(&baseline_plan)
        {
            return self.failure_result(
                deck,
                start,
                wrapper_contract,
                format!(
                    "{kind_name} family '{}' baseline DC qualification failed: {reason}",
                    contract.family
                ),
                Vec::new(),
            );
        }
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
        let baseline_source_multiplicity_snapshot = if contract.kind
            == XyceBaselineFamilyKind::SourceMultiplicity
        {
            match Self::source_multiplicity_family_snapshot(&baseline_netlist, &baseline_plan.print)
            {
                Ok(snapshot) => Some(snapshot),
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
            }
        } else {
            None
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
            if contract.kind == XyceBaselineFamilyKind::SourceMultiplicity
                && let Err(reason) = Self::validate_source_multiplicity_dc_plan(&target_plan)
            {
                return self.failure_result(
                    deck,
                    start,
                    wrapper_contract,
                    format!(
                        "{kind_name} family '{}' member {} DC qualification failed: {reason}",
                        contract.family,
                        self.display_path(&target_path)
                    ),
                    Vec::new(),
                );
            }
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

            if let Some(baseline_snapshot) = baseline_source_multiplicity_snapshot.as_ref() {
                let target_snapshot = match Self::source_multiplicity_family_snapshot(
                    &target_netlist,
                    &target_plan.print,
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
                    Self::compare_source_multiplicity_snapshots(baseline_snapshot, &target_snapshot)
                {
                    return self.failure_result(
                        deck,
                        start,
                        wrapper_contract,
                        format!(
                            "{kind_name} family '{}' member {} changes semantics outside source M composition: {reason}",
                            contract.family,
                            self.display_path(&target_path)
                        ),
                        Vec::new(),
                    );
                }
            }

            let target_table = if contract.kind == XyceBaselineFamilyKind::SourceMultiplicity {
                match self.dc_results_to_prn_table(&target_plan, &target_netlist, &target_results) {
                    Ok(table) => Some(table),
                    Err(error) => {
                        return self.failure_result(
                            deck,
                            start,
                            wrapper_contract,
                            format!(
                                "{kind_name} family '{}' member {} output conversion failed: {error}",
                                contract.family,
                                self.display_path(&target_path)
                            ),
                            Vec::new(),
                        );
                    }
                }
            } else {
                None
            };
            let comparison = if let Some(target_table) = target_table.as_ref() {
                // Every removed wrapper passed owner.prn as GOODFILE and
                // baseline.prn as TESTFILE. The normalized-RMS denominator
                // makes this ordering observable and therefore contractual.
                self.compare_release_7_10_xyce_verify_dc_tables(
                    "source multiplicity",
                    target_table,
                    &baseline_table,
                    &target_results,
                    &baseline_results,
                )
            } else {
                self.compare_dc_prn_reference(
                    &baseline_table,
                    &target_plan.print,
                    &target_netlist,
                    &baseline_plan.source,
                    &target_plan.dc,
                    &target_results,
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

    pub(super) fn simulate_ac_baseline_family_table(
        &self,
        kind: XyceBaselineFamilyKind,
        plan: &XyceRelationalAcPlan,
        netlist: &Netlist,
    ) -> Result<XycePrnTable, String> {
        let engine = self.create_xyce_engine();
        if plan.frequency_bound {
            if kind != XyceBaselineFamilyKind::AbmFrequency || plan.ac.data_points().is_some() {
                return Err(format!(
                    "family kind {} does not admit this frequency-bound relational AC plan",
                    kind.name()
                ));
            }
            let mut points = Vec::with_capacity(plan.ac.frequencies.len());
            for (row, frequency) in plan.ac.frequencies.iter().copied().enumerate() {
                let source = Self::source_with_ac_frequency_bindings(&plan.source, frequency);
                let point_netlist =
                    Self::parse_xyce_netlist(&source, &plan.deck_path).map_err(|err| {
                        format!(
                            "frequency-bound relational AC row {} parse failed: {err}",
                            row + 1
                        )
                    })?;
                let mut results = engine.run_ac(&point_netlist, &[frequency]).map_err(|err| {
                    format!(
                        "frequency-bound relational AC row {} solve failed: {err}",
                        row + 1
                    )
                })?;
                let result = results.pop().ok_or_else(|| {
                    format!(
                        "frequency-bound relational AC row {} produced no result",
                        row + 1
                    )
                })?;
                if !results.is_empty() {
                    return Err(format!(
                        "frequency-bound relational AC row {} produced multiple results",
                        row + 1
                    ));
                }
                points.push(XyceAcDataPointResult {
                    netlist: point_netlist,
                    result,
                });
            }
            return Self::ac_family_data_points_to_prn_table(&plan.print, &points);
        }

        if let Some(data_points) = plan.ac.data_points() {
            if kind != XyceBaselineFamilyKind::AbmFrequency {
                return Err(format!(
                    "family kind {} does not admit a DATA relational AC plan",
                    kind.name()
                ));
            }
            let mut points = Vec::with_capacity(data_points.len());
            for (row, point) in data_points.iter().enumerate() {
                let (point_netlist, _) =
                    Engine::create_perturbed_netlist_multi(netlist, &point.overrides).map_err(
                        |err| format!("DATA relational AC row {} override failed: {err}", row + 1),
                    )?;
                let mut results =
                    engine
                        .run_ac(&point_netlist, &[point.frequency])
                        .map_err(|err| {
                            format!("DATA relational AC row {} solve failed: {err}", row + 1)
                        })?;
                let result = results.pop().ok_or_else(|| {
                    format!("DATA relational AC row {} produced no result", row + 1)
                })?;
                if !results.is_empty() {
                    return Err(format!(
                        "DATA relational AC row {} produced multiple results",
                        row + 1
                    ));
                }
                points.push(XyceAcDataPointResult {
                    netlist: point_netlist,
                    result,
                });
            }
            return Self::ac_family_data_points_to_prn_table(&plan.print, &points);
        }

        let results = engine
            .run_ac(netlist, &plan.ac.frequencies)
            .map_err(|err| format!("relational AC solve failed: {err}"))?;
        Self::ac_family_result_to_prn_table(&plan.print, netlist, &results)
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
        let baseline_plan = match self
            .baseline_family_ac_plan_for_path(contract.kind, &contract.baseline_path)
        {
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
        if let Err(reason) = Self::validate_baseline_family_ac_plan(contract.kind, &baseline_plan) {
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
        let baseline_netlist = match Self::relational_ac_plan_netlist(&baseline_plan) {
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
            match Self::strict_ac_family_snapshot(contract.kind, &baseline_netlist, &baseline_plan)
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
        let baseline_table = match self.simulate_ac_baseline_family_table(
            contract.kind,
            &baseline_plan,
            &baseline_netlist,
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
            let target_plan = match self
                .baseline_family_ac_plan_for_path(contract.kind, &target_path)
            {
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
            if let Err(reason) = Self::validate_baseline_family_ac_plan(contract.kind, &target_plan)
            {
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
                || (contract.kind != XyceBaselineFamilyKind::AbmFrequency
                    && !target_plan
                        .ac
                        .frequencies
                        .iter()
                        .zip(&baseline_plan.ac.frequencies)
                        .all(|(target, baseline)| target.to_bits() == baseline.to_bits()))
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
            let target_netlist = match Self::relational_ac_plan_netlist(&target_plan) {
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
            if contract.kind != XyceBaselineFamilyKind::AbmFrequency
                && !Self::ac_analyses_match_exactly(&baseline_netlist, &target_netlist)
            {
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
            let target_table = match self.simulate_ac_baseline_family_table(
                contract.kind,
                &target_plan,
                &target_netlist,
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
            let (gold, test) = if contract.kind.ac_comparator_member_is_good_waveform() {
                (&target_table, &baseline_table)
            } else {
                (&baseline_table, &target_table)
            };
            let mut mismatches =
                match self.compare_ac_comparator_tables_with_tolerance(gold, test, tolerance) {
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
        let baseline_print = match baseline_plan.require_print("transient family baseline") {
            Ok(print) => print,
            Err(error) => {
                return self.baseline_family_qualification_result(
                    deck,
                    start,
                    wrapper_contract,
                    contract.comparison,
                    error,
                );
            }
        };
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
        if contract.kind == XyceBaselineFamilyKind::Params1
            && let Err(err) = Self::validate_params1_transient_plan(&baseline_plan)
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
        if contract.kind == XyceBaselineFamilyKind::NakedAlgebra
            && let Err(err) = Self::validate_naked_algebra_transient_plan(&baseline_plan)
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
        if contract.kind == XyceBaselineFamilyKind::Bug1826ThermalParameter
            && let Err(err) =
                Self::validate_bug1826_thermal_parameter_transient_plan(&baseline_plan)
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
        if contract.kind == XyceBaselineFamilyKind::SourceMultiplicity
            && let Err(err) = Self::validate_source_multiplicity_transient_plan(&baseline_plan)
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
        if contract.kind == XyceBaselineFamilyKind::Bug38SubcktFormalParentheses
            && let Err(err) = Self::validate_bug38_transient_plan(
                &baseline_plan,
                XyceBug38Role::ParenthesizedControl,
            )
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
                &baseline_plan,
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
            let target_print = match target_plan.require_print("transient family member") {
                Ok(print) => print,
                Err(error) => {
                    return self.baseline_family_qualification_result(
                        deck,
                        start,
                        wrapper_contract,
                        contract.comparison,
                        error,
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
            if contract.kind == XyceBaselineFamilyKind::Params1
                && let Err(err) = Self::validate_params1_transient_plan(&target_plan)
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
            if contract.kind == XyceBaselineFamilyKind::NakedAlgebra
                && let Err(err) = Self::validate_naked_algebra_transient_plan(&target_plan)
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
            if contract.kind == XyceBaselineFamilyKind::Bug1826ThermalParameter
                && let Err(err) =
                    Self::validate_bug1826_thermal_parameter_transient_plan(&target_plan)
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
            if contract.kind == XyceBaselineFamilyKind::SourceMultiplicity
                && let Err(err) = Self::validate_source_multiplicity_transient_plan(&target_plan)
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
            if contract.kind == XyceBaselineFamilyKind::Bug38SubcktFormalParentheses
                && let Err(err) =
                    Self::validate_bug38_transient_plan(&target_plan, XyceBug38Role::WrapperOwner)
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
                if target_print.probes != baseline_print.probes {
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
                    &target_plan,
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
                let comparison = if contract.comparison.compares_prn_case_insensitively() {
                    self.compare_serialized_default_prn_tables_case_insensitive(
                        &baseline_table,
                        &target_table,
                    )
                } else if contract.comparison.compares_serialized_prn_exactly() {
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
                match self.compare_baseline_family_xyce_verify_tables(
                    contract.kind,
                    &baseline_table,
                    &target_table,
                ) {
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
                    target_print,
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
                        target_print,
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
        engine.run_tran_with_startup_mode_and_abort(
            netlist,
            plan.tran.stop,
            max_step,
            rspice_core::engine::TransientStartupMode::from_uic(plan.tran.uic),
            &abort,
        )
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
