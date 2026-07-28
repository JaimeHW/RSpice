//! Finding decks in the vendored corpus and reading the manifest.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    /// Discover every `.cir` file under the vendored Xyce root.
    pub fn discover_tests(&self) -> Vec<XyceDeck> {
        let mut paths = Vec::new();
        Self::collect_circuit_files(&self.root, &mut paths);
        paths.sort();
        paths
            .into_iter()
            .map(|path| {
                let relative_path = self.relative_key(&path);
                let section = Self::section_for_relative_path(&relative_path);
                XyceDeck {
                    path,
                    relative_path,
                    section,
                }
            })
            .collect()
    }

    /// Discover simulator regression decks under `Netlists`.
    pub fn discover_netlist_tests(&self) -> Vec<XyceDeck> {
        self.discover_tests()
            .into_iter()
            .filter(|deck| deck.section == XyceDeckSection::Netlists)
            .collect()
    }

    pub(super) fn static_hb_plan_for_deck(
        &self,
        deck: &XyceDeck,
    ) -> Result<XyceStaticHbPlan, String> {
        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(
                "static HB currently implements the canonical three-file upstream-wrapper contract"
                    .to_string(),
            );
        }
        if Self::contains_control_block(&source) {
            return Err("HB deck uses a .control block; simulator scripting is not part of the static HB contract".to_string());
        }
        let requests = Self::print_output_requests(&source, "HB")?;
        if requests.len() != 1 {
            return Err(format!(
                "static HB requires exactly one .PRINT HB request, found {}",
                requests.len()
            ));
        }
        let request = requests.into_iter().next().expect("one HB print request");
        if request.file.is_some() {
            return Err("static HB does not combine the three canonical outputs with FILE= side destinations".to_string());
        }
        let output_format = XyceStaticHbOutputFormat::for_format(request.format.as_deref())
            .map_err(|err| format!("static HB output contract rejected the request: {err}"))?;
        if source.lines().any(|line| {
            Self::strip_netlist_comment(line)
                .trim_start()
                .to_ascii_lowercase()
                .starts_with(".fft")
        }) {
            return Err("static HB wrapper contract does not admit .FFT output".to_string());
        }

        let mut print_count = 0usize;
        let mut option_lines = Vec::new();
        for line in Self::logical_netlist_lines(&source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                print_count += 1;
            } else if command.eq_ignore_ascii_case(".probe") {
                return Err(
                    "static HB wrapper contract does not admit .PROBE directives".to_string(),
                );
            } else if command.eq_ignore_ascii_case(".options") {
                option_lines.push(trimmed);
            }
        }
        if print_count != 1 {
            return Err(format!(
                "static HB wrapper contract requires exactly one .PRINT directive, found {print_count}"
            ));
        }

        let netlist = Self::parse_xyce_netlist(&source, &deck.path)
            .map_err(|err| format!("HB netlist parse failed: {err}"))?;
        let [AnalysisCommand::Hb { frequencies }] = netlist.analyses.as_slice() else {
            return Err(
                "static HB requires exactly one .HB analysis and no other analysis cards"
                    .to_string(),
            );
        };
        let [frequency] = frequencies.as_slice() else {
            return Err(format!(
                "static HB currently supports one tone, found {}",
                frequencies.len()
            ));
        };
        if !frequency.is_finite() || *frequency <= 0.0 {
            return Err(format!(
                "static HB fundamental must be finite and positive, found {frequency}"
            ));
        }
        let [num_harmonics] = netlist.options.hb_num_frequencies.as_slice() else {
            return Err(format!(
                "static HB requires exactly one positive HBINT NUMFREQ value, found {}",
                netlist.options.hb_num_frequencies.len()
            ));
        };
        if *num_harmonics == 0 || *num_harmonics > 256 {
            return Err(format!(
                "static HB harmonic order must be in 1..=256, found {num_harmonics}"
            ));
        }
        let normalized_options = option_lines
            .iter()
            .map(|line| {
                line.chars()
                    .filter(|character| !character.is_whitespace())
                    .flat_map(char::to_lowercase)
                    .collect::<String>()
            })
            .collect::<Vec<_>>();
        let accepted_options = [
            format!(".optionshbintnumfreq={num_harmonics}"),
            format!(".optionshbintnumfreq[1]={num_harmonics}"),
        ];
        if normalized_options.len() != 1 || !accepted_options.contains(&normalized_options[0]) {
            return Err(
                "static HB wrapper contract currently admits only HBINT NUMFREQ; startup, output, and linear-solver options require distinct typed contracts"
                    .to_string(),
            );
        }

        for probe in &request.probes {
            if Self::static_hb_probe_is_unsupported(probe) {
                return Err(format!(
                    "static HB wrapper contract does not yet support branch, device-current, or device-parameter probe '{probe}'"
                ));
            }
        }

        if netlist.elements.iter().any(|element| {
            !matches!(
                &element.kind,
                rspice_core::netlist::ElementKind::Resistor { .. }
                    | rspice_core::netlist::ElementKind::Capacitor { .. }
                    | rspice_core::netlist::ElementKind::Diode { .. }
                    | rspice_core::netlist::ElementKind::VoltageSource(_)
            )
        }) {
            return Err(
                "static HB wrapper contract currently covers R/C/diode circuits driven by independent voltage sources"
                    .to_string(),
            );
        }

        let fd_reference_path = self
            .static_output_reference_path(&deck.path, output_format.reference_extension())
            .ok_or_else(|| "HB deck is not under tests/xyce/Netlists".to_string())?;
        let td_reference_path = self
            .static_output_reference_path(&deck.path, "HB.TD.prn")
            .ok_or_else(|| "HB deck is not under tests/xyce/Netlists".to_string())?;
        let ic_reference_path = self
            .static_output_reference_path(&deck.path, "hb_ic.prn")
            .ok_or_else(|| "HB deck is not under tests/xyce/Netlists".to_string())?;
        let startup_reference_path = self
            .static_output_reference_path(&deck.path, "startup.prn")
            .ok_or_else(|| "HB deck is not under tests/xyce/Netlists".to_string())?;
        if startup_reference_path.is_file() {
            return Err(
                "static HB wrapper contract does not yet implement the additional startup.prn artifact"
                    .to_string(),
            );
        }
        for (label, path) in [
            ("HB frequency-domain", &fd_reference_path),
            ("HB time-domain", &td_reference_path),
            ("HB startup transient", &ic_reference_path),
        ] {
            if !path.is_file() {
                return Err(format!(
                    "no checked-in {label} oracle at {}",
                    self.display_path(path)
                ));
            }
        }

        Ok(XyceStaticHbPlan {
            deck_path: deck.path.clone(),
            source,
            print: XycePrintRequest {
                probes: request.probes,
            },
            frequency: *frequency,
            num_harmonics: *num_harmonics,
            fd_reference_path,
            td_reference_path,
            ic_reference_path,
            output_format,
            wrapper: self.requires_upstream_wrapper(&deck.relative_path),
        })
    }

    pub(super) fn static_tran_plan_for_deck(
        &self,
        deck: &XyceDeck,
    ) -> Result<XyceStaticTranPlan, String> {
        self.static_tran_plan_for_deck_with_purpose(deck, XyceStaticTranPlanPurpose::AbsoluteOracle)
    }

    pub(super) fn static_tran_plan_for_deck_with_purpose(
        &self,
        deck: &XyceDeck,
        purpose: XyceStaticTranPlanPurpose,
    ) -> Result<XyceStaticTranPlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let analytic_wrapper = purpose == XyceStaticTranPlanPurpose::AnalyticOracle;
        let generated_reference_wrapper =
            purpose == XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily;
        if analytic_wrapper && !requires_wrapper {
            return Err(
                "analytic transient oracle purpose requires wrapper provenance".to_string(),
            );
        }
        if generated_reference_wrapper && !requires_wrapper {
            return Err(
                "generated-reference relational transient purpose requires wrapper provenance"
                    .to_string(),
            );
        }
        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;

        // Wrapper-only decks without a checked-in oracle must be classified
        // before expanding large continuation cards (for example, a massive
        // OUTPUTTIMEPOINTS list).  The raw-source shape is sufficient for
        // this fail-closed decision and avoids allocating parser-sized data
        // structures for a contract that cannot execute natively.
        let output_override = requires_wrapper
            && Self::is_native_output_override_wrapper_candidate_path(&deck.relative_path);
        let noindex_header_wrapper = requires_wrapper
            && Self::is_native_noindex_header_tran_wrapper_candidate(&deck.relative_path, &source);
        let has_static_tran_oracle = self.has_static_tran_reference_oracle(&deck.path);
        if requires_wrapper
            && !output_override
            && !noindex_header_wrapper
            && !has_static_tran_oracle
            && !Self::source_may_have_pwl_repeat_option(&source)
            && !analytic_wrapper
            && !generated_reference_wrapper
        {
            return Err(Self::upstream_wrapper_required_reason().to_string());
        }

        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;
        let source =
            Self::source_with_wrapper_paramfile_bindings(&source, &deck.path, requires_wrapper)?;

        let print_output = if output_override {
            Self::output_override_print_output_request(&source, "TRAN")?.ok_or_else(|| {
                "output override deck has no .PRINT TRAN statement with static columns".to_string()
            })?
        } else {
            Self::single_tran_print_output_request(&source)?
        };
        let print = XycePrintRequest {
            probes: print_output.probes.clone(),
        };
        let netlist = Self::parse_xyce_netlist(&source, &deck.path)
            .map_err(|err| format!("netlist parser does not yet accept this Xyce deck: {err}"))?;
        let tran = Self::single_tran_analysis(&netlist)?;
        let steps = Self::step_commands(&netlist)?;
        let has_prn_oracle = self
            .static_output_reference_path(
                &deck.path,
                XyceStaticTranContract::WrapperStatic.reference_extension(),
            )
            .is_some_and(|path| path.is_file());
        let native_static_prn_wrapper = if analytic_wrapper {
            None
        } else if requires_wrapper {
            if output_override {
                Self::native_output_override_prn_tran_wrapper_contract(&source)
                    .map(Some)
                    .map_err(|_| Self::upstream_wrapper_required_reason().to_string())?
            } else {
                Self::native_static_prn_tran_wrapper_contract(
                    &deck.path,
                    &deck.relative_path,
                    &source,
                    has_prn_oracle,
                )
            }
        } else {
            None
        };
        if steps.is_empty()
            && requires_wrapper
            && native_static_prn_wrapper.is_none()
            && !analytic_wrapper
            && !generated_reference_wrapper
        {
            return Err(Self::upstream_wrapper_required_reason().to_string());
        }
        let contract = if analytic_wrapper {
            XyceStaticTranContract::WrapperStatic
        } else if requires_wrapper {
            native_static_prn_wrapper.unwrap_or(XyceStaticTranContract::WrapperStatic)
        } else {
            Self::static_tran_contract_for_print_format(false, print_output.format.as_deref())?
        };
        let primary_reference_path = self
            .static_output_reference_path(&deck.path, contract.reference_extension())
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        let reference_path = if primary_reference_path.is_file() {
            primary_reference_path
        } else if matches!(contract, XyceStaticTranContract::WrapperCsd) {
            Self::tran_gsfile_reference_path(&deck.path).ok_or_else(|| {
                format!(
                    "no checked-in static .{} oracle at {}",
                    contract.reference_extension(),
                    self.display_path(&primary_reference_path)
                )
            })?
        } else {
            primary_reference_path
        };
        Self::validate_static_tran_reference_requirement(purpose, contract, &reference_path)?;
        if !steps.is_empty() {
            Self::validate_static_step_tran_contract(&netlist)?;
        }
        if requires_wrapper {
            if analytic_wrapper {
                Self::validate_native_static_prn_tran_wrapper_contract(&source)?;
            } else {
                match contract {
                    XyceStaticTranContract::WrapperCsd => {
                        Self::validate_native_static_csd_tran_wrapper_contract(&source)?;
                    }
                    XyceStaticTranContract::WrapperCsv => {
                        Self::validate_native_static_csv_tran_wrapper_contract(&source)?;
                    }
                    XyceStaticTranContract::WrapperStatic => {
                        if output_override {
                            Self::validate_native_output_override_prn_tran_wrapper_contract(
                                &source,
                            )?;
                        } else {
                            Self::validate_native_static_prn_tran_wrapper_contract_with_format_mode(
                            &source,
                            has_prn_oracle,
                        )?;
                        }
                    }
                    XyceStaticTranContract::WrapperStaticExpectedError => {
                        Self::validate_native_static_prn_tran_wrapper_contract(&source)?;
                    }
                    XyceStaticTranContract::WrapperNoIndexHeader => {
                        Self::validate_native_noindex_header_tran_wrapper_contract(&source)?;
                    }
                    _ => Self::validate_native_static_prn_tran_wrapper_contract(&source)?,
                }
            }
        }
        Self::validate_static_tran_analysis_contract(&netlist, &tran, &print)?;
        let timeint_conststep = Self::source_enables_constant_time_step_output(&source);
        let mut plan = XyceStaticTranPlan {
            deck_path: deck.path.clone(),
            reference_path,
            source,
            print,
            output_override,
            timeint_conststep,
            tran,
            steps,
            wrapper_tolerance: Self::native_default_prn_tran_wrapper_tolerance(&deck.relative_path),
            contract,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        plan.comparison_mode =
            Self::select_static_tran_comparison_mode(&plan, &netlist, purpose, requires_wrapper)?;
        let validation_purpose =
            if plan.steps.is_empty() && plan.comparison_mode.uses_integrated_rms_verifier() {
                XyceStaticTranPlanPurpose::DefaultLevel9XyceVerifyOracle
            } else {
                purpose
            };
        if validation_purpose.validates_absolute_device_contract()
            || matches!(
                validation_purpose,
                XyceStaticTranPlanPurpose::ScopedModelRelationalFamily
                    | XyceStaticTranPlanPurpose::AgeCapRelationalFamily
            )
        {
            Self::validate_native_transient_contract_for_purpose(&netlist, validation_purpose)?;
        } else {
            Self::validate_native_relational_transient_contract(&netlist)?;
        }

        Ok(plan)
    }

    pub(super) fn static_ac_plan_for_deck(
        &self,
        deck: &XyceDeck,
    ) -> Result<XyceStaticAcPlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let output_override = requires_wrapper
            && Self::is_native_output_override_wrapper_candidate_path(&deck.relative_path);
        let source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;
        if requires_wrapper {
            Self::validate_native_static_fd_ac_wrapper_contract(&source, output_override)?;
        }
        let sensitivity =
            self.static_ac_sensitivity_plan_for_source(&source, &deck.path, output_override)?;
        let engine_source = if sensitivity.is_some() {
            Self::source_without_xyce_sensitivity_directives(&source)
        } else {
            source.clone()
        };

        let primary_ac_output = if output_override {
            Self::output_override_print_output_request(&source, "AC")?
        } else {
            Self::canonical_print_output_request(&source, "AC", requires_wrapper)?
        };
        let primary_ac_ic_output = if output_override {
            Self::output_override_print_output_request(&source, "AC_IC")?
        } else {
            Self::canonical_print_output_request(&source, "AC_IC", requires_wrapper)?
        };
        if primary_ac_output.is_none() && primary_ac_ic_output.is_none() && sensitivity.is_none() {
            return Err(
                "deck has no primary .PRINT AC, .PRINT AC_IC, or qualified .PRINT SENS statement"
                    .to_string(),
            );
        }

        let (netlist, frequency_bound) = match Self::parse_xyce_netlist(&engine_source, &deck.path)
        {
            Ok(netlist) if Self::parsed_netlist_has_ac_frequency_dependent_global(&netlist) => {
                let frequency_bound_source =
                    Self::source_with_ac_frequency_bindings(&engine_source, 1.0);
                let netlist = Self::parse_xyce_netlist(&frequency_bound_source, &deck.path)
                    .map_err(|retry_err| {
                        format!(
                            "netlist parser does not accept this Xyce deck with AC frequency bindings: {retry_err}"
                        )
                    })?;
                (netlist, true)
            }
            Ok(netlist) => (netlist, false),
            Err(err)
                if Self::parse_error_is_unbound_ac_frequency_dependency(&engine_source, &err) =>
            {
                let frequency_bound_source =
                    Self::source_with_ac_frequency_bindings(&engine_source, 1.0);
                let netlist = Self::parse_xyce_netlist(&frequency_bound_source, &deck.path)
                    .map_err(|retry_err| {
                        format!(
                            "netlist parser does not yet accept this Xyce deck even with AC frequency bindings: {retry_err}"
                        )
                    })?;
                (netlist, true)
            }
            Err(err) => {
                return Err(format!(
                    "netlist parser does not yet accept this Xyce deck: {err}"
                ));
            }
        };
        let ac = Self::single_ac_analysis(&netlist)?;
        let steps = Self::step_commands(&netlist)?;
        if ac.data_points().is_some() && !steps.is_empty() {
            return Err(
                ".STEP combined with .AC DATA is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }
        if frequency_bound && !steps.is_empty() {
            return Err(
                ".STEP combined with AC frequency-dependent parameters is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }
        if frequency_bound && ac.data_points().is_some() {
            return Err(
                ".AC DATA combined with AC frequency-dependent parameters is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }

        let use_continuous_files = netlist.options.measure_use_cont_files();
        let measurement_reference_paths = if netlist.measurements.iter().any(|measurement| {
            measurement.print_policy == rspice_core::analysis::MeasurePrintPolicy::All
                && (measurement.analysis.eq_ignore_ascii_case("AC")
                    || (!use_continuous_files
                        && measurement.analysis.eq_ignore_ascii_case("AC_CONT")))
        }) {
            self.measurement_reference_paths(&deck.path, "ma")?
        } else {
            Vec::new()
        };
        let continuous_measurement_reference_paths = if use_continuous_files {
            self.continuous_measurement_reference_paths(&deck.path, &netlist, "AC_CONT", "ma")?
        } else {
            Vec::new()
        };
        let measurement_tolerance = if netlist.measurements.iter().any(|measurement| {
            measurement.analysis.eq_ignore_ascii_case("AC")
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Integ { .. }
                )
        }) {
            XyceFileCompareTolerance::MEASURE_COMMON_AC_INTEGRATION
        } else {
            XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT
        };

        let primary_ac_ic_file = primary_ac_ic_output
            .as_ref()
            .and_then(|request| request.file.clone());
        let (contract, reference_path, print, primary_ac_file) =
            if let Some(print_output) = primary_ac_output {
                let primary_ac_file = print_output.file.clone();
                let contract = Self::static_ac_contract_for_print_format(
                    requires_wrapper,
                    print_output.format.as_deref(),
                )?;
                let reference_path = self
                    .static_output_reference_path(&deck.path, contract.reference_extension())
                    .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
                let reference_exists = reference_path.is_file();
                if !reference_exists
                    && sensitivity.is_none()
                    && measurement_reference_paths.is_empty()
                    && continuous_measurement_reference_paths.is_empty()
                {
                    return Err(format!(
                        "no checked-in static .{} or AC measurement oracle at {}",
                        contract.reference_extension(),
                        self.display_path(&reference_path)
                    ));
                }
                let print = XycePrintRequest {
                    probes: print_output.probes,
                };
                Self::validate_static_ac_contract(&netlist, &ac, &print)?;
                let reference_path = reference_exists.then_some(reference_path);
                let print = if sensitivity.is_some() && !reference_exists {
                    None
                } else {
                    Some(print)
                };
                (contract, reference_path, print, primary_ac_file)
            } else if let Some(print_output) = primary_ac_ic_output {
                if !steps.is_empty() {
                    return Err(
                    ".STEP AC_IC-only comparison is not implemented for wrapper-origin AC output"
                        .to_string(),
                );
                }
                let contract = Self::static_ac_ic_contract_for_print_format(
                    requires_wrapper,
                    print_output.format.as_deref(),
                )?;
                let reference_path = self
                    .static_output_reference_path(&deck.path, contract.reference_extension())
                    .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
                if !reference_path.is_file() {
                    return Err(format!(
                        "no checked-in static .{} oracle at {}",
                        contract.reference_extension(),
                        self.display_path(&reference_path)
                    ));
                }
                (contract, None, None, None)
            } else if sensitivity.is_some() {
                (
                    if requires_wrapper {
                        XyceStaticAcContract::WrapperStatic
                    } else {
                        XyceStaticAcContract::PlainStatic
                    },
                    None,
                    None,
                    None,
                )
            } else {
                unreachable!("AC output request presence was checked before parsing");
            };

        Ok(XyceStaticAcPlan {
            deck_path: deck.path.clone(),
            reference_path,
            measurement_reference_paths,
            continuous_measurement_reference_paths,
            measurement_tolerance,
            source,
            print,
            primary_ac_file,
            primary_ac_ic_file,
            sensitivity,
            output_override,
            ac,
            frequency_bound,
            steps,
            contract,
        })
    }

    pub(super) fn static_noise_plan_for_deck(
        &self,
        deck: &XyceDeck,
    ) -> Result<XyceStaticNoisePlan, String> {
        let requires_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        let output_override = requires_wrapper
            && Self::is_native_output_override_wrapper_candidate_path(&deck.relative_path);
        let qualified_step_cont_derivative = Self::normalize_manifest_key(&deck.relative_path)
            == XYCE_MEASURE_CONT_STEP_NOISE_DERIV_RECORD;
        let qualified_step_scalar_derivative = Self::normalize_manifest_key(&deck.relative_path)
            == XYCE_MEASURE_NOISE_STEP_DERIV_RECORD;
        let source = if qualified_step_cont_derivative {
            let bytes = self.validate_measure_cont_step_noise_deriv_provenance(deck)?;
            String::from_utf8(bytes)
                .map_err(|error| format!("MEASURE_CONT STEP NOISE source is not UTF-8: {error}"))?
        } else if qualified_step_scalar_derivative {
            let bytes = self.validate_measure_noise_step_deriv_provenance(deck)?;
            String::from_utf8(bytes)
                .map_err(|error| format!("MEASURE_NOISE STEP source is not UTF-8: {error}"))?
        } else {
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?
        };
        // The static-contract probes run in sequence for every corpus deck.
        // Fail closed before parsing an unrelated deck: some upstream stress
        // fixtures intentionally describe enormous circuits and must only be
        // materialized by the contract that actually owns them.
        if !Self::source_has_analysis(&source, "NOISE") {
            return Err("deck has no .NOISE analysis".to_string());
        }
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;
        let print_output = if output_override {
            Self::output_override_print_output_request(&source, "NOISE")?
        } else {
            Self::canonical_print_output_request(&source, "NOISE", false)?
        };
        let print = print_output.as_ref().map(|request| XycePrintRequest {
            probes: request.probes.clone(),
        });
        let contract = if output_override {
            XyceStaticNoiseContract::StdPrn
        } else {
            XyceStaticNoiseContract::for_format(
                print_output
                    .as_ref()
                    .and_then(|request| request.format.as_deref()),
            )?
        };
        if output_override {
            Self::validate_native_output_override_prn_wrapper_contract(&source, "NOISE")?;
        } else if let Some(primary) = print_output.as_ref() {
            Self::validate_static_noise_output_destinations(&source, primary, contract)?;
        }

        let netlist = Self::parse_xyce_netlist(&source, &deck.path)
            .map_err(|err| format!("netlist parser does not yet accept this Xyce deck: {err}"))?;
        #[cfg(not(feature = "veriloga-builtins-base"))]
        if let Some(print) = print.as_ref() {
            if Self::noise_print_requires_generated_vbic_mechanisms(print, &netlist)? {
                return Err(
                    "NOISE output requests a named VBIC mechanism supplied by the canonical generated Verilog-A device; rebuild with the 'veriloga-builtins' feature"
                        .to_string(),
                );
            }
        }
        let noise_analysis = Self::noise_analysis_for_netlist(&netlist)?;
        let output_node = noise_analysis.output_node.clone();
        let reference_node = noise_analysis.reference_node.clone();
        let input_source = noise_analysis.input_source.clone();
        let frequencies = noise_analysis.frequencies.clone();
        let data_points = noise_analysis.data_points.clone();
        let data_table_name = noise_analysis.data_table_name.clone();
        let steps = Self::step_commands(&netlist)?;
        if data_points.is_some() && !steps.is_empty() {
            return Err(
                ".STEP combined with .NOISE DATA is not implemented in the native Xyce oracle"
                    .to_string(),
            );
        }
        let has_scalar_derivative = netlist.measurements.iter().any(|measurement| {
            measurement.analysis.eq_ignore_ascii_case("NOISE")
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Derivative { .. }
                )
        });
        let has_continuous_derivative = netlist.measurements.iter().any(|measurement| {
            measurement.analysis.eq_ignore_ascii_case("NOISE_CONT")
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Derivative { .. }
                )
        });
        if !steps.is_empty()
            && has_scalar_derivative
            && !has_continuous_derivative
            && !qualified_step_scalar_derivative
        {
            return Err(
                "native scalar-only .STEP NOISE DERIV measurements are not yet qualified to Xyce precision"
                    .to_string(),
            );
        }
        if qualified_step_cont_derivative {
            Self::validate_measure_cont_step_noise_deriv_plan(
                &netlist,
                print.as_ref(),
                &frequencies,
            )?;
        } else if qualified_step_scalar_derivative {
            Self::validate_measure_noise_step_deriv_plan(&netlist, print.as_ref(), &frequencies)?;
        }
        let use_continuous_files = netlist.options.measure_use_cont_files();
        let measurement_reference_paths = if netlist.measurements.iter().any(|measurement| {
            measurement.print_policy == rspice_core::analysis::MeasurePrintPolicy::All
                && (measurement.analysis.eq_ignore_ascii_case("NOISE")
                    || (!use_continuous_files
                        && measurement.analysis.eq_ignore_ascii_case("NOISE_CONT")))
        }) {
            self.measurement_reference_paths(&deck.path, "ma")?
        } else {
            Vec::new()
        };
        let continuous_measurement_reference_paths = if use_continuous_files {
            self.continuous_measurement_reference_paths(&deck.path, &netlist, "NOISE_CONT", "ma")?
        } else {
            Vec::new()
        };
        if !steps.is_empty() && !continuous_measurement_reference_paths.is_empty() {
            return Err(
                ".STEP NOISE_CONT comparison requires a stepped continuous sidecar contract"
                    .to_string(),
            );
        }
        let measurement_tolerance = if qualified_step_cont_derivative {
            // Both the aggregate measurement files and the captured stdout
            // were checked by Release 7.10's file_compare.pl contract.
            XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT
        } else if netlist.measurements.iter().any(|measurement| {
            (measurement.analysis.eq_ignore_ascii_case("NOISE")
                || measurement.analysis.eq_ignore_ascii_case("NOISE_CONT"))
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Integ { .. }
                )
        }) {
            XyceFileCompareTolerance::MEASURE_COMMON_AC_INTEGRATION
        } else if netlist.measurements.iter().any(|measurement| {
            (measurement.analysis.eq_ignore_ascii_case("NOISE")
                || measurement.analysis.eq_ignore_ascii_case("NOISE_CONT"))
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Derivative { .. }
                )
        }) {
            XyceFileCompareTolerance::MEASURE_COMMON_DERIVATIVE
        } else {
            XyceFileCompareTolerance::MEASURE_COMMON_DEFAULT
        };
        let reference_path = self
            .static_output_reference_path(&deck.path, contract.reference_extension())
            .filter(|path| path.is_file());
        let side_references = if output_override {
            Vec::new()
        } else {
            self.static_noise_side_references(&source, &deck.path)?
        };
        if reference_path.is_none()
            && side_references.is_empty()
            && measurement_reference_paths.is_empty()
            && continuous_measurement_reference_paths.is_empty()
        {
            return Err("deck has no checked-in NOISE waveform or measurement oracle".to_string());
        }
        if reference_path.is_some() && print.is_none() {
            return Err(
                "NOISE waveform oracle requires a primary .PRINT NOISE request".to_string(),
            );
        }
        Ok(XyceStaticNoisePlan {
            deck_path: deck.path.clone(),
            source,
            print,
            output_override,
            reference_path,
            side_references,
            measurement_reference_paths,
            continuous_measurement_reference_paths,
            gs_reference_path: qualified_step_cont_derivative.then(|| {
                self.root
                    .join("Netlists/MEASURE_CONT/STEP/DerivTestNoiseGSfile")
            }),
            measurement_tolerance,
            output_node,
            reference_node,
            input_source,
            frequencies,
            data_points,
            data_table_name,
            steps,
            contract,
        })
    }

    /// Parse the native static DC `.SENS` output contract.  The parser is
    /// deliberately independent of the ordinary DC contract so a deck that
    /// emits both `.PRINT DC` and `.PRINT SENS` is admitted by the sensitivity
    /// path before the generic wrapper classifier sees the extra table.
    pub(super) fn static_dc_sensitivity_plan_for_deck(
        &self,
        deck: &XyceDeck,
    ) -> Result<XyceStaticDcSensitivityPlan, String> {
        let authored_source =
            fs::read_to_string(&deck.path).map_err(|err| format!("failed to read deck: {err}"))?;
        let sensitivity_lines = Self::logical_netlist_lines(&authored_source)
            .into_iter()
            .filter(|line| {
                Self::strip_netlist_comment(line)
                    .split_whitespace()
                    .next()
                    .is_some_and(|command| command.eq_ignore_ascii_case(".sens"))
            })
            .collect::<Vec<_>>();
        if sensitivity_lines.is_empty() {
            return Err("deck has no .SENS directive".to_string());
        }
        if sensitivity_lines.len() != 1 {
            return Err(
                "native static DC sensitivity contract requires exactly one .SENS directive"
                    .to_string(),
            );
        }
        // Keep AC/TRAN/HB/NOISE sensitivity decks on their existing analysis
        // adapters.  `.SENS` is shared by DC, AC, and transient contracts, so
        // selecting the static DC path solely from the card would otherwise
        // mask the more specific family reason.
        for line in Self::logical_netlist_lines(&authored_source) {
            let command = Self::strip_netlist_comment(&line)
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .to_ascii_lowercase();
            if matches!(
                command.as_str(),
                ".ac" | ".tran" | ".noise" | ".hb" | ".sp" | ".stb" | ".disto" | ".pz" | ".tf"
            ) {
                return Err(format!(
                    "deck contains {command}; static DC sensitivity adapter only covers .DC/.OP"
                ));
            }
        }

        let print_requests = Self::aggregate_print_output_requests(
            Self::print_output_requests(&authored_source, "SENS")?,
            "SENS",
        )?;
        let primary_requests = print_requests
            .iter()
            .filter(|request| request.file.is_none())
            .cloned()
            .collect::<Vec<_>>();
        if primary_requests.len() > 1 {
            return Err(
                "native static DC sensitivity contract requires at most one primary .PRINT SENS statement"
                    .to_string(),
            );
        }
        let side_requests = print_requests
            .iter()
            .filter(|request| request.file.is_some())
            .cloned()
            .collect::<Vec<_>>();
        if primary_requests.is_empty() && side_requests.is_empty() {
            return Err(
                "native static DC sensitivity contract requires one .PRINT SENS output destination"
                    .to_string(),
            );
        }
        let canonical_request = primary_requests
            .first()
            .cloned()
            .or_else(|| side_requests.first().cloned())
            .expect("one primary or side .PRINT SENS request");
        let (reference_format, no_index) =
            Self::dc_sensitivity_output_schema(canonical_request.format.as_deref())?;

        let tokens = Self::split_print_fields(&sensitivity_lines[0])?;
        let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
        let mut objfunc = None;
        let mut parameters: Option<String> = None;
        let mut index = 1usize;
        while index < token_refs.len() {
            let Some((raw_key, raw_value, consumed)) =
                Self::print_option_assignment(&token_refs, index)
            else {
                if let Some(parameters) = parameters.as_mut()
                    && !token_refs[index].contains('=')
                    && token_refs[index].contains(':')
                {
                    if !parameters.ends_with(',') {
                        parameters.push(',');
                    }
                    parameters.push_str(token_refs[index].trim());
                    index += 1;
                    continue;
                }
                return Err(format!(
                    "Xyce .SENS directive contains an unsupported field '{}'",
                    token_refs[index]
                ));
            };
            let key = raw_key.trim().to_ascii_lowercase();
            let value = raw_value.trim();
            if value.is_empty() {
                return Err(format!("Xyce .SENS {key} assignment is missing a value"));
            }
            let destination = match key.as_str() {
                "objfunc" | "objvars" => &mut objfunc,
                "param" | "params" => &mut parameters,
                _ => {
                    return Err(format!(
                        "native static DC sensitivity contract does not cover .SENS field '{raw_key}'"
                    ));
                }
            };
            if destination.is_some() {
                return Err(format!("Xyce .SENS contains duplicate {key} assignments"));
            }
            *destination = Some(value.to_string());
            index += consumed;
        }

        let objectives =
            Self::parse_xyce_dc_sensitivity_objectives(objfunc.as_deref().ok_or_else(|| {
                "Xyce .SENS directive must provide OBJFUNC={output}[,...]".to_string()
            })?)?;
        let parameters =
            Self::parse_xyce_sensitivity_parameters(parameters.as_deref().ok_or_else(|| {
                "Xyce .SENS directive must provide PARAM=<device:param>[,<device:param>...]"
                    .to_string()
            })?)?;
        let (direct, adjoint) = Self::parse_xyce_sensitivity_flags(&authored_source)?;

        let reference_extension = match reference_format {
            XyceDcSensitivityReferenceFormat::Prn => "SENS.prn",
            XyceDcSensitivityReferenceFormat::Csv => "SENS.csv",
        };
        let reference_path = self
            .static_output_reference_path(&deck.path, reference_extension)
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !reference_path.is_file() {
            return Err(format!(
                "no checked-in static DC sensitivity oracle at {}",
                self.display_path(&reference_path)
            ));
        }

        let mut side_outputs = Vec::new();
        for request in side_requests {
            let file = request
                .file
                .clone()
                .expect("side sensitivity output has FILE= set");
            let (side_reference_format, side_no_index) =
                Self::dc_sensitivity_output_schema(request.format.as_deref())?;
            let candidate = Self::side_output_reference_candidate(&reference_path, &file)?;
            let side_reference_path = if candidate.is_file() {
                candidate
            } else if side_reference_format == XyceDcSensitivityReferenceFormat::Prn
                && !side_no_index
            {
                // GNUPLOT/SPLOT are PRN-compatible writers in Xyce.  When no
                // distinct side artifact is retained, compare the canonical
                // table under the same probe/schema contract.
                reference_path.clone()
            } else {
                return Err(format!(
                    "missing checked-in static DC sensitivity side-output oracle {}",
                    self.display_path(&candidate)
                ));
            };
            if side_reference_path == reference_path
                && (side_reference_format != reference_format || side_no_index != no_index)
            {
                return Err(format!(
                    "DC sensitivity side output '{file}' falls back to an incompatible canonical schema"
                ));
            }
            side_outputs.push(XyceStaticDcSensitivitySideOutput {
                file,
                reference_path: side_reference_path,
                reference_format: side_reference_format,
                print: XycePrintRequest {
                    probes: request.probes,
                },
                no_index: side_no_index,
            });
        }

        let stripped_source = Self::source_without_xyce_sensitivity_directives(&authored_source);
        let dc = self.static_dc_plan_for_source_with_execution_dir(
            &deck.path,
            stripped_source,
            ExpressionDialect::Xyce,
            None,
        )?;
        if dc.dc.sweep2.is_some() {
            return Err(
                "native static DC sensitivity contract does not yet cover a secondary .DC sweep"
                    .to_string(),
            );
        }
        let parsed_netlist = Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
            &dc.source,
            &dc.deck_path,
            dc.expression_dialect,
            dc.parameter_redefinition_policy,
            dc.execution_dir.as_deref(),
        )
        .map_err(|err| format!("netlist parser rejected static DC sensitivity deck: {err}"))?;
        if !parsed_netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Dc { .. }
                    | AnalysisCommand::Op
                    | AnalysisCommand::Sensitivity { .. }
                    | AnalysisCommand::Step(_)
            )
        }) {
            return Err(
                "native static DC sensitivity contract requires a .DC or .OP analysis".to_string(),
            );
        }
        if parsed_netlist.analyses.iter().any(|analysis| {
            !matches!(
                analysis,
                AnalysisCommand::Dc { .. }
                    | AnalysisCommand::Op
                    | AnalysisCommand::Sensitivity { .. }
                    | AnalysisCommand::Step(_)
            )
        }) {
            return Err(
                "native static DC sensitivity contract does not cover mixed AC/TRAN/HB analyses"
                    .to_string(),
            );
        }

        let add_stepnum_col = Self::source_requests_sensitivity_stepnum_column(&authored_source);
        Ok(XyceStaticDcSensitivityPlan {
            dc,
            reference_path,
            reference_format,
            print: XycePrintRequest {
                probes: canonical_request.probes,
            },
            objectives,
            parameters,
            direct,
            adjoint,
            no_index,
            add_stepnum_col,
            side_outputs,
        })
    }

    pub(super) fn top_level_execution_deck_path(deck_path: &Path) -> Result<PathBuf, String> {
        let file_name = deck_path
            .file_name()
            .ok_or_else(|| "wrapper deck has no filename".to_string())?;
        let parent = deck_path
            .parent()
            .ok_or_else(|| "wrapper deck has no parent directory".to_string())?;
        Ok(parent.join("top_level").join(file_name))
    }

    pub(super) fn is_expected_missing_inductor_value_error_deck(
        relative_path: &str,
        source: &str,
    ) -> bool {
        matches!(
            Self::normalize_manifest_key(relative_path).as_str(),
            "netlists/inductor/errormessagetest.cir"
        ) && source
            .to_ascii_lowercase()
            .contains("l value missing from instance line")
    }

    pub(super) fn collect_subcircuit_defs<'a>(
        subcircuit: &'a SubcircuitDef,
        defs: &mut BTreeMap<String, &'a SubcircuitDef>,
    ) {
        defs.insert(subcircuit.name.to_ascii_lowercase(), subcircuit);
        for nested in &subcircuit.nested_subcircuits {
            Self::collect_subcircuit_defs(nested, defs);
        }
    }

    pub(super) fn deck_sidecar_path(deck_path: &Path, extension: &str) -> PathBuf {
        let mut sidecar = deck_path.as_os_str().to_os_string();
        sidecar.push(".");
        sidecar.push(extension);
        PathBuf::from(sidecar)
    }

    pub(super) fn find_model<'a>(
        models: &'a [rspice_core::netlist::ModelDef],
        name: &str,
    ) -> Option<&'a rspice_core::netlist::ModelDef> {
        models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(name))
    }

    pub(super) fn find_model_or_binned<'a>(
        models: &'a [rspice_core::netlist::ModelDef],
        name: &str,
        instance_params: &[(String, Value)],
    ) -> Option<&'a rspice_core::netlist::ModelDef> {
        Self::find_model(models, name)
            .or_else(|| Self::find_binned_model(models, name, instance_params))
    }

    pub(super) fn find_binned_model<'a>(
        models: &'a [rspice_core::netlist::ModelDef],
        name: &str,
        instance_params: &[(String, Value)],
    ) -> Option<&'a rspice_core::netlist::ModelDef> {
        let prefix = format!("{name}.");
        models
            .iter()
            .filter(|model| {
                model.name.len() > prefix.len()
                    && model.name[..prefix.len()].eq_ignore_ascii_case(&prefix)
                    && Self::model_matches_geometry_bin(model, instance_params)
            })
            .min_by(|left, right| {
                Self::model_geometry_bin_range_size(left)
                    .partial_cmp(&Self::model_geometry_bin_range_size(right))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    pub(super) fn find_unique_model_in<'a>(
        models: impl IntoIterator<Item = &'a rspice_core::netlist::ModelDef>,
        name: &str,
    ) -> Option<&'a rspice_core::netlist::ModelDef> {
        let mut matches = models
            .into_iter()
            .filter(|model| model.name.eq_ignore_ascii_case(name));
        let model = matches.next()?;
        matches.next().is_none().then_some(model)
    }

    pub(super) fn find_semiconductor_model(
        netlist: &Netlist,
        model_name: &str,
    ) -> Option<rspice_core::netlist::ModelDef> {
        Self::find_model(&netlist.models, model_name)
            .cloned()
            .or_else(|| {
                rspice_core::netlist::flatten_netlist_with_models(netlist)
                    .ok()?
                    .scoped_models
                    .into_iter()
                    .find(|model| model.name.eq_ignore_ascii_case(model_name))
            })
    }

    pub(super) fn find_semiconductor_device_element(
        netlist: &Netlist,
        element_name: &str,
    ) -> Option<rspice_core::netlist::Element> {
        let is_semiconductor = |element: &rspice_core::netlist::Element| {
            matches!(
                element.kind,
                ElementKind::Diode { .. }
                    | ElementKind::Bjt { .. }
                    | ElementKind::Mosfet { .. }
                    | ElementKind::Jfet { .. }
                    | ElementKind::Mesfet { .. }
            )
        };
        netlist
            .elements
            .iter()
            .find(|element| {
                is_semiconductor(element)
                    && Self::device_instance_names_match(&element.name, element_name)
            })
            .cloned()
            .or_else(|| {
                rspice_core::netlist::flatten_netlist_with_models(netlist)
                    .ok()?
                    .elements
                    .into_iter()
                    .find(|element| {
                        is_semiconductor(element)
                            && Self::device_instance_names_match(&element.name, element_name)
                    })
            })
    }

    pub(super) fn find_recorded_two_terminal_branch_element(
        netlist: &Netlist,
        name: &str,
    ) -> Option<rspice_core::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && element.nodes.len() >= 2
                && Self::element_has_recorded_branch_current(&element.kind)
        }) {
            return Some(element.clone());
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && element.nodes.len() >= 2
                    && Self::element_has_recorded_branch_current(&element.kind)
            })
    }

    pub(super) fn find_resistor_element(
        netlist: &Netlist,
        name: &str,
    ) -> Option<rspice_core::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::Resistor { .. })
        }) {
            return Some(element.clone());
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::Resistor { .. })
            })
    }

    pub(super) fn find_native_xyce_memristor_element(
        netlist: &Netlist,
        name: &str,
    ) -> Option<rspice_core::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::XyceMemristor { .. })
                && Self::netlist_element_is_native_xyce_memristor(netlist, element)
        }) {
            return Some(element.clone());
        }

        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist).ok()?;
        let mut flat_netlist = netlist.clone();
        flat_netlist.elements = flattened.elements;
        flat_netlist.models.extend(flattened.scoped_models);
        flat_netlist.subcircuits.clear();
        flat_netlist
            .elements
            .iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::XyceMemristor { .. })
                    && Self::netlist_element_is_native_xyce_memristor(&flat_netlist, element)
            })
            .cloned()
    }

    pub(super) fn find_capacitor_element(
        netlist: &Netlist,
        name: &str,
    ) -> Option<rspice_core::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::Capacitor { .. })
        }) {
            return Some(element.clone());
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::Capacitor { .. })
            })
    }

    pub(super) fn find_inductor_element(
        netlist: &Netlist,
        name: &str,
    ) -> Option<rspice_core::netlist::Element> {
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, name)
                && matches!(&element.kind, ElementKind::Inductor { .. })
        }) {
            return Some(element.clone());
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, name)
                    && matches!(&element.kind, ElementKind::Inductor { .. })
            })
    }

    pub(super) fn find_bare_device_parameter_element(
        netlist: &Netlist,
        probe: &str,
    ) -> Option<rspice_core::netlist::Element> {
        let probe_name = Self::parse_bare_device_parameter_probe(probe)?;
        if let Some(element) = netlist.elements.iter().find(|element| {
            Self::device_instance_names_match(&element.name, &probe_name)
                && Self::element_has_bare_device_parameter(&element.kind)
        }) {
            return Some(element.clone());
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist)
            .ok()?
            .elements
            .into_iter()
            .find(|element| {
                Self::device_instance_names_match(&element.name, &probe_name)
                    && Self::element_has_bare_device_parameter(&element.kind)
            })
    }

    pub(super) fn deck_has_print_analysis(&self, deck: &XyceDeck, analysis: &str) -> bool {
        fs::read_to_string(&deck.path).is_ok_and(|source| {
            Self::print_output_requests(&source, analysis).is_ok_and(|requests| {
                requests
                    .into_iter()
                    .any(|request| request.file.is_none() && !request.probes.is_empty())
            })
        })
    }

    pub(super) fn collect_circuit_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                Self::collect_circuit_files(&path, out);
            } else if path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("cir"))
            {
                out.push(path);
            }
        }
    }

    pub(super) fn normalize_manifest_key(path: &str) -> String {
        path.trim().replace('\\', "/").to_ascii_lowercase()
    }

    pub(super) fn deck_name(path: &Path) -> String {
        path.file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string()
    }
}
