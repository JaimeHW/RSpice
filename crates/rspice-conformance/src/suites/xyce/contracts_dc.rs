//! DC, sweep, and `.STEP` deck contracts.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn validate_measure_step_find_when_oracle(
        &self,
        deck: &XyceDeck,
        member: XyceMeasureStepFindWhenMember,
        start: Instant,
    ) -> Result<(), String> {
        let sources = self.validate_measure_step_find_when_provenance(deck, member)?;
        self.check_measure_cont_tran_deadline(start, "FIND/WHEN stepped provenance")?;

        let mut parsed = Vec::with_capacity(sources.len());
        for (candidate, bytes) in sources {
            let source = std::str::from_utf8(&bytes)
                .map_err(|error| format!("MEASURE STEP FIND/WHEN source is not UTF-8: {error}"))?
                .to_string();
            let path = self.root.join(candidate.source_relative_path());
            let netlist = Self::parse_xyce_netlist(&source, &path).map_err(|error| {
                format!(
                    "MEASURE STEP FIND/WHEN parse failed for {}: {error}",
                    candidate.source_relative_path()
                )
            })?;
            let output = Self::single_tran_print_output_request(&source)?;
            if output.file.is_some()
                || output
                    .format
                    .as_deref()
                    .is_some_and(|format| !Self::tran_print_format_is_prn_compatible(format))
            {
                return Err(format!(
                    "{} requires one ordinary PRN-compatible .PRINT TRAN",
                    candidate.source_relative_path()
                ));
            }
            let print = XycePrintRequest {
                probes: output.probes,
            };
            let tran = Self::single_tran_analysis(&netlist)?;
            Self::validate_measure_step_find_when_plan(&netlist, &print, &tran, candidate)?;
            parsed.push((candidate, netlist, tran));
        }

        let owner_member = XyceMeasureStepFindWhenMember::owner();
        let owner_index = parsed
            .iter()
            .position(|(candidate, ..)| *candidate == owner_member)
            .ok_or_else(|| "MEASURE STEP FIND/WHEN family omitted its owner".to_string())?;
        let owner_measurements = format!("{:#?}", parsed[owner_index].1.measurements);
        for index in 0..4 {
            let control = XyceMeasureStepFindWhenMember::control(index)
                .ok_or_else(|| "invalid MEASURE STEP FIND/WHEN control index".to_string())?;
            let control_netlist = &parsed
                .iter()
                .find(|(candidate, ..)| *candidate == control)
                .ok_or_else(|| format!("MEASURE STEP FIND/WHEN omitted control {index}"))?
                .1;
            if format!("{:#?}", control_netlist.measurements) != owner_measurements {
                return Err(format!(
                    "MEASURE STEP FIND/WHEN control {index} measurement AST/order differs from its owner"
                ));
            }
        }

        let (_, owner_netlist, owner_tran) = &parsed[owner_index];
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(owner_tran));
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let steps = Self::step_commands(owner_netlist)?;
        let materialized = Self::nested_step_runs_for_commands_with_limits_and_abort(
            &engine,
            owner_netlist,
            &steps,
            xyce_step_plan_limits(),
            &abort,
        )
        .map_err(|error| format!("MEASURE STEP FIND/WHEN expansion failed: {error}"))?;
        let expected_step_values = [[1.0, -0.25], [2.0, -0.25], [1.0, -0.5], [2.0, -0.5]];
        if materialized.len() != expected_step_values.len()
            || materialized
                .iter()
                .zip(expected_step_values)
                .any(|(run, expected)| run.step_values != expected)
        {
            return Err(format!(
                "MEASURE STEP FIND/WHEN owner produced unexpected ordered step values: {:?}",
                materialized
                    .iter()
                    .map(|run| &run.step_values)
                    .collect::<Vec<_>>()
            ));
        }

        let mut owner_runs = Vec::with_capacity(4);
        for run in materialized {
            let tran = Self::single_tran_analysis(&run.netlist)?;
            owner_runs.push(self.evaluate_measure_cont_step_tran_run(
                run.netlist,
                run.step_values,
                &tran,
                start,
            )?);
        }

        for (index, (owner, expected)) in owner_runs.iter().zip(expected_step_values).enumerate() {
            let control_member = XyceMeasureStepFindWhenMember::control(index)
                .ok_or_else(|| "invalid MEASURE STEP FIND/WHEN control index".to_string())?;
            let (_, control_netlist, control_tran) = parsed
                .iter()
                .find(|(candidate, ..)| *candidate == control_member)
                .ok_or_else(|| format!("MEASURE STEP FIND/WHEN omitted control {index}"))?;
            let control = self.evaluate_measure_cont_step_tran_run(
                control_netlist.clone(),
                expected.to_vec(),
                control_tran,
                start,
            )?;
            Self::compare_measure_cont_step_waveforms(index, owner, &control)?;
            Self::compare_measure_cont_step_measurements(index, owner, &control)?;
            Self::validate_measure_cont_remeasure(
                &owner.netlist,
                &owner.transient,
                None,
                XyceFileCompareTolerance::MEASURE_CONT_STEP_REMEASURE,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
        }

        self.check_measure_cont_tran_deadline(
            start,
            "FIND/WHEN stepped execution, relational comparison, and remeasure",
        )
    }

    pub(super) fn validate_measure_step_find_when_plan(
        netlist: &Netlist,
        print: &XycePrintRequest,
        tran: &XyceTranAnalysis,
        member: XyceMeasureStepFindWhenMember,
    ) -> Result<(), String> {
        let steps = Self::step_commands(netlist)?;
        let exact_owner_steps = matches!(steps.as_slice(), [
            StepCommand {
                target: StepTarget::Device,
                name: first_name,
                param_name: Some(first_param),
                sweep: StepSweep::Linear {
                    start: first_start,
                    stop: first_stop,
                    step: first_step,
                },
            },
            StepCommand {
                target: StepTarget::Device,
                name: second_name,
                param_name: Some(second_param),
                sweep: StepSweep::Linear {
                    start: second_start,
                    stop: second_stop,
                    step: second_step,
                },
            },
        ] if first_name.eq_ignore_ascii_case("VS1")
            && first_param.eq_ignore_ascii_case("VA")
            && first_start.to_bits() == 1.0f64.to_bits()
            && first_stop.to_bits() == 2.0f64.to_bits()
            && first_step.to_bits() == 1.0f64.to_bits()
            && second_name.eq_ignore_ascii_case("VS4")
            && second_param.eq_ignore_ascii_case("V0")
            && second_start.to_bits() == (-0.25f64).to_bits()
            && second_stop.to_bits() == (-0.5f64).to_bits()
            && second_step.to_bits() == (-0.25f64).to_bits());
        if (member.is_owner() && !exact_owner_steps) || (!member.is_owner() && !steps.is_empty()) {
            return Err(format!(
                "MEASURE STEP FIND/WHEN exact owner/control .STEP contract changed: {steps:?}"
            ));
        }

        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        let expected_analyses = if member.is_owner() { 3 } else { 1 };
        if tran.step.to_bits() != 0.0f64.to_bits()
            || tran.stop.to_bits() != 0.005f64.to_bits()
            || tran.start.map(Value::to_bits) != Some(0.0f64.to_bits())
            || tran.max_step.map(Value::to_bits) != Some(1.0e-5f64.to_bits())
            || tran.uic
            || probes != ["v(1)", "v(2)", "v(3)", "v(4)", "v(5)"]
            || netlist.analyses.len() != expected_analyses
            || netlist.output_requests.len() != netlist.measurements.len() + 1
            || netlist.measurements.len() != 43
            || netlist.measurements.iter().any(|statement| {
                !statement.analysis.eq_ignore_ascii_case("TRAN")
                    || statement.print_policy != rspice_core::analysis::MeasurePrintPolicy::All
            })
            || !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || netlist.elements.len() != 10
        {
            return Err(format!(
                "MEASURE STEP FIND/WHEN exact TRAN/PRINT/topology contract changed for {}",
                member.source_relative_path()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_step_case_sensitive_census(&self) -> Result<(), String> {
        let base = self.root.join("Netlists/MEASURE_CONT/STEP");
        let mut names = Vec::new();
        let mut content = Vec::new();
        for entry in fs::read_dir(&base)
            .map_err(|error| format!("failed to read {}: {error}", base.display()))?
        {
            let path = entry
                .map_err(|error| format!("failed to inspect {}: {error}", base.display()))?
                .path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "MEASURE_CONT STEP member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| "MEASURE_CONT STEP member name is not UTF-8".to_string())?
                .to_string();
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(
                &format!("MEASURE_CONT STEP member {}", path.display()),
                &bytes,
            )?;
            names.push(name.clone());
            content.push(format!("{name}\0{}", blake3::hash(&canonical).to_hex()));
        }
        names.sort();
        content.sort();
        let names_hash = blake3::hash(names.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if names.len() != XYCE_MEASURE_CONT_STEP_TRAN_DIRECTORY_COUNT
            || names_hash != XYCE_MEASURE_CONT_STEP_TRAN_DIRECTORY_NAMES_BLAKE3
            || content_hash != XYCE_MEASURE_CONT_STEP_TRAN_DIRECTORY_CONTENT_BLAKE3
        {
            return Err(format!(
                "MEASURE_CONT STEP case-sensitive census changed: names={}/{names_hash}, content={content_hash}",
                names.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_cont_step_noise_deriv_plan(
        netlist: &Netlist,
        print: Option<&XycePrintRequest>,
        frequencies: &[Value],
    ) -> Result<(), String> {
        let steps = Self::step_commands(netlist)?;
        let exact_step = matches!(steps.as_slice(), [StepCommand {
            target: StepTarget::Device,
            name,
            param_name: None,
            sweep: StepSweep::Linear { start, stop, step },
        }] if name.eq_ignore_ascii_case("RL")
            && start.to_bits() == 1.0f64.to_bits()
            && stop.to_bits() == 1.5f64.to_bits()
            && step.to_bits() == 0.5f64.to_bits());
        let probes = print
            .map(|request| {
                request
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let expected_probes = [
            "vm(3)",
            "vm(b)",
            "vm(c)",
            "vm(d)",
            "vm(e)",
            "derivcrossconttest2",
            "derivcrossconttest3",
            "derivcrossconttest4",
            "derivcrossneg2",
            "derivcrosscontneg2",
            "derivcrossneg6",
            "derivcrosscontneg6",
        ];
        if !exact_step
            || probes != expected_probes
            || frequencies.len() != 61
            || frequencies.first().map(|value| value.to_bits()) != Some(1.0e-2f64.to_bits())
            || frequencies
                .last()
                .is_none_or(|value| (*value - 10.0).abs() > 1.0e-12)
            || netlist.options.measure_use_cont_files != Some(false)
            || netlist.analyses.len() != 2
            || netlist.output_requests.len() != netlist.measurements.len() + 1
            || !netlist.diagnostics.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || netlist.elements.len() != 13
        {
            return Err(format!(
                "MEASURE_CONT STEP NOISE exact analysis/step/print/topology contract changed: step={steps:?}, probes={probes:?}, frequencies={:?}/{:?}/{}, use_cont={:?}, analyses={}, outputs={}, measurements={}, diagnostics={}, models={}, subcircuits={}, data={}, elements={}",
                frequencies.first(),
                frequencies.last(),
                frequencies.len(),
                netlist.options.measure_use_cont_files,
                netlist.analyses.len(),
                netlist.output_requests.len(),
                netlist.measurements.len(),
                netlist.diagnostics.len(),
                netlist.models.len(),
                netlist.subcircuits.len(),
                netlist.data_tables.len(),
                netlist.elements.len()
            ));
        }
        let scalar = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("NOISE"))
            .count();
        let continuous = netlist
            .measurements
            .iter()
            .filter(|statement| statement.analysis.eq_ignore_ascii_case("NOISE_CONT"))
            .count();
        let derivative = netlist
            .measurements
            .iter()
            .filter(|statement| {
                matches!(
                    statement.measure_type,
                    rspice_core::analysis::MeasureType::Derivative { .. }
                )
            })
            .count();
        let last = netlist.measurements.last();
        if scalar != 13
            || continuous != 32
            || scalar + continuous != netlist.measurements.len()
            || derivative != 44
            || !last.is_some_and(|statement| {
                statement.analysis.eq_ignore_ascii_case("NOISE")
                    && statement.name.eq_ignore_ascii_case("lastMeasure")
                    && matches!(
                        statement.measure_type,
                        rspice_core::analysis::MeasureType::Max { .. }
                    )
            })
            || netlist.measurements.iter().any(|statement| {
                statement.print_policy != rspice_core::analysis::MeasurePrintPolicy::All
            })
        {
            return Err(format!(
                "MEASURE_CONT STEP NOISE measurement census changed: NOISE={scalar}/13, NOISE_CONT={continuous}/32, DERIV={derivative}/44"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_measure_noise_step_deriv_plan(
        netlist: &Netlist,
        print: Option<&XycePrintRequest>,
        frequencies: &[Value],
    ) -> Result<(), String> {
        let steps = Self::step_commands(netlist)?;
        let exact_step = matches!(steps.as_slice(), [StepCommand {
            target: StepTarget::Device,
            name,
            param_name: None,
            sweep: StepSweep::Linear { start, stop, step },
        }] if name.eq_ignore_ascii_case("V2")
            && start.to_bits() == 12.0f64.to_bits()
            && stop.to_bits() == 6.0f64.to_bits()
            && step.to_bits() == (-6.0f64).to_bits());
        let probes = print
            .map(|request| {
                request
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let expected_probes = [
            "vm(out)", "vr(out)", "vi(out)", "im(v1)", "inoise", "onoise",
        ];
        let derivative_count = netlist
            .measurements
            .iter()
            .filter(|measurement| {
                measurement.analysis.eq_ignore_ascii_case("NOISE")
                    && matches!(
                        measurement.measure_type,
                        rspice_core::analysis::MeasureType::Derivative { .. }
                    )
            })
            .count();
        if !exact_step
            || probes != expected_probes
            || frequencies.len() != 51
            || frequencies.first().map(|value| value.to_bits()) != Some(1.0e-2f64.to_bits())
            || frequencies
                .last()
                .is_none_or(|value| (*value - 1.0e3).abs() > 1.0e-9)
            || netlist.analyses.len() != 2
            || netlist.output_requests.len() != netlist.measurements.len() + 1
            || netlist.measurements.len() != 15
            || derivative_count != 14
            || !netlist.diagnostics.is_empty()
            || netlist.models.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || netlist.elements.len() != 10
        {
            return Err(format!(
                "MEASURE_NOISE STEP derivative exact contract changed: step={steps:?}, probes={probes:?}, frequencies={:?}/{:?}/{}, analyses={}, outputs={}, measurements={}/15, derivatives={derivative_count}/14, diagnostics={}, models={}, subcircuits={}, data={}, elements={}",
                frequencies.first(),
                frequencies.last(),
                frequencies.len(),
                netlist.analyses.len(),
                netlist.output_requests.len(),
                netlist.measurements.len(),
                netlist.diagnostics.len(),
                netlist.models.len(),
                netlist.subcircuits.len(),
                netlist.data_tables.len(),
                netlist.elements.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_plain_static_dc_prn_wrapper_source(source: &str) -> Result<(), String> {
        Self::validate_default_prn_wrapper_source_with_format_mode(source, true)?;

        let print = Self::single_dc_print_request(source)?;
        for probe in &print.probes {
            let normalized = Self::normalize_probe(probe);
            if normalized.contains('*') {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not cover wildcard probe '{probe}'"
                ));
            }
        }

        let mut dc_count = 0usize;
        let mut subckt_count = 0usize;
        let mut ends_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".dc" => dc_count += 1,
                ".model" => {
                    Self::validate_plain_static_dc_prn_wrapper_model_type(&trimmed)?;
                }
                ".subckt" => subckt_count += 1,
                ".ends" => ends_count += 1,
                ".print" | ".measure" | ".meas" | ".op" | ".step" | ".param" | ".global_param"
                | ".func" | ".options" | ".end" => {}
                other => {
                    return Err(format!(
                        "wrapper-origin plain static DC contract does not cover {other} directives"
                    ));
                }
            }
        }

        if dc_count == 0 {
            return Err(
                "wrapper-origin plain static DC contract requires at least one .DC statement, found none"
                    .to_string(),
            );
        }

        if subckt_count > 1 {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one .SUBCKT statement, found {subckt_count}"
            ));
        }
        if subckt_count != ends_count {
            return Err(format!(
                "wrapper-origin plain static DC contract requires balanced .SUBCKT/.ENDS statements, found {subckt_count}/{ends_count}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_plain_static_dc_prn_wrapper_model_type(
        model_line: &str,
    ) -> Result<(), String> {
        let Some(model_type) = model_line.split_whitespace().nth(2) else {
            return Err(
                "wrapper-origin plain static DC contract requires .MODEL statements to include a model type"
                    .to_string(),
            );
        };
        let normalized = model_type
            .trim_matches(|ch| matches!(ch, '(' | ')' | ','))
            .to_ascii_uppercase();
        if matches!(
            normalized.as_str(),
            "NMOS" | "PMOS" | "NPN" | "PNP" | "LPNP" | "D" | "DIODE" | "R" | "RES" | "RESISTOR"
        ) {
            return Ok(());
        }
        Err(format!(
            "wrapper-origin plain static DC contract does not yet cover .MODEL type {model_type}"
        ))
    }

    pub(super) fn validate_plain_static_dc_prn_wrapper_netlist(
        netlist: &Netlist,
    ) -> Result<(), String> {
        for model in &netlist.models {
            if !matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "NMOS" | "PMOS" | "NPN" | "PNP" | "LPNP" | "D" | "DIODE" | "R" | "RES" | "RESISTOR"
            ) {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not yet cover parsed model type {}",
                    model.model_type
                ));
            }
            if matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "NPN" | "PNP" | "LPNP"
            ) && !Self::model_is_native_legacy_bjt(model)
            {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not yet cover advanced BJT model type {}",
                    model.model_type
                ));
            }
            if matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "D" | "DIODE"
            ) && !Self::model_is_native_legacy_diode(model)
            {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not yet cover advanced diode model type {}",
                    model.model_type
                ));
            }
            if matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "R" | "RES" | "RESISTOR"
            ) && !Self::model_is_native_legacy_resistor(model)
            {
                return Err(format!(
                    "wrapper-origin plain static DC contract does not yet cover advanced resistor model type {}",
                    model.model_type
                ));
            }
        }
        if netlist.models.len() > 1
            && !Self::models_form_single_binned_native_mos_family(&netlist.models)
            && !netlist
                .models
                .iter()
                .all(Self::model_is_native_legacy_resistor)
        {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one parsed model unless all models form one binned native MOS model family, found {}",
                netlist.models.len()
            ));
        }
        if netlist.subcircuits.len() > 1 {
            return Err(format!(
                "wrapper-origin plain static DC contract currently covers at most one parsed subcircuit, found {}",
                netlist.subcircuits.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_scalar_dc_measurement_wrapper_source(
        source: &str,
    ) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        let mut dc_measurement_count = 0usize;
        let mut dc_continuous_measurement_count = 0usize;
        let mut use_continuous_files = None;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") {
                let tokens = Self::split_print_fields(&trimmed)?;
                let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
                if !Self::validate_default_prn_print_tokens(&token_refs, false)? {
                    primary_print_count += 1;
                }
                continue;
            }
            if command.eq_ignore_ascii_case(".measure") || command.eq_ignore_ascii_case(".meas") {
                let fields = Self::split_print_fields(&trimmed)?;
                let analysis = fields.get(1).map(String::as_str).unwrap_or_default();
                if !analysis.eq_ignore_ascii_case("DC") && !analysis.eq_ignore_ascii_case("DC_CONT")
                {
                    return Err(format!(
                        "scalar DC measurement artifact contract does not cover directive '{trimmed}'"
                    ));
                }
                if analysis.eq_ignore_ascii_case("DC") {
                    dc_measurement_count += 1;
                } else {
                    dc_continuous_measurement_count += 1;
                }
                continue;
            }
            if command.eq_ignore_ascii_case(".options") || command.eq_ignore_ascii_case(".option") {
                let fields = Self::split_print_fields(&trimmed)?;
                if fields
                    .get(1)
                    .is_some_and(|group| group.eq_ignore_ascii_case("MEASURE"))
                {
                    for (index, field) in fields.iter().enumerate().skip(2) {
                        let (name, value) = if let Some((name, value)) = field.split_once('=') {
                            (name, Some(value))
                        } else if field.eq_ignore_ascii_case("USE_CONT_FILES")
                            && fields.get(index + 1).is_some_and(|field| field == "=")
                        {
                            (field.as_str(), fields.get(index + 2).map(String::as_str))
                        } else {
                            (field.as_str(), None)
                        };
                        if name.eq_ignore_ascii_case("USE_CONT_FILES") {
                            use_continuous_files = match value {
                                Some("0") => Some(false),
                                Some("1") => Some(true),
                                _ => None,
                            };
                        }
                    }
                }
                continue;
            }
            // Xyce parses .FFT in every deck but activates it only for
            // transient analysis. The typed netlist parser below remains
            // responsible for validating the complete directive.
            if command.eq_ignore_ascii_case(".fft") {
                continue;
            }
            if Self::is_extra_wrapper_output_analysis_command(command) {
                return Err(format!(
                    "scalar DC measurement artifact contract does not cover {command} directives"
                ));
            }
        }
        if primary_print_count != 1 {
            return Err(format!(
                "scalar DC measurement artifact contract requires one primary .PRINT DC statement, found {primary_print_count}"
            ));
        }
        if dc_measurement_count == 0
            && !(dc_continuous_measurement_count > 0 && use_continuous_files == Some(false))
        {
            return Err(
                "DC measurement artifact contract requires a .MEASURE DC statement or an explicit USE_CONT_FILES=0 aggregate with at least one .MEASURE DC_CONT statement"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_no_output_dc_wrapper_source(source: &str) -> Result<(), String> {
        let mut has_dc_or_op = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            if trimmed.is_empty() {
                continue;
            }
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if command.eq_ignore_ascii_case(".print") || command.eq_ignore_ascii_case(".probe") {
                return Err(format!(
                    "wrapper-origin no-output DC contract does not cover {command} directives"
                ));
            }
            if command.eq_ignore_ascii_case(".dc") || command.eq_ignore_ascii_case(".op") {
                has_dc_or_op = true;
                continue;
            }
            if Self::is_extra_wrapper_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin no-output DC contract does not cover {command} directives"
                ));
            }
        }

        if has_dc_or_op {
            Ok(())
        } else {
            Err("wrapper-origin no-output DC contract requires a .DC or .OP analysis".to_string())
        }
    }

    pub(super) fn static_dc_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticDcContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::dc_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticDcContract::WrapperDefault
            } else {
                XyceStaticDcContract::PlainStatic
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticDcContract::WrapperCsv
            } else {
                XyceStaticDcContract::PlainCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticDcContract::WrapperCsd
            } else {
                XyceStaticDcContract::PlainCsd
            });
        }
        Err(format!(
            "native static .PRINT DC comparison does not cover FORMAT={normalized}"
        ))
    }

    pub(super) fn validate_exact_dc_family_plan(
        kind: XyceBaselineFamilyKind,
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        match kind {
            XyceBaselineFamilyKind::AbmLookupOrder => Self::validate_abm_lookup_order_dc_plan(plan),
            XyceBaselineFamilyKind::BjtExternalNode => {
                Self::validate_bjt_external_node_dc_plan(plan)
            }
            XyceBaselineFamilyKind::DcAnalysisExpression => {
                Self::validate_dc_analysis_expression_plan(plan)
            }
            XyceBaselineFamilyKind::DelimitedExpression => {
                Self::validate_delimited_expression_plan(plan)
            }
            XyceBaselineFamilyKind::PassiveResPrimaryValue => {
                Self::validate_passive_res_primary_dc_plan(plan)
            }
            XyceBaselineFamilyKind::SubcktParameterPrecedence => {
                Self::validate_subckt_parameter_precedence_dc_plan(plan)
            }
            XyceBaselineFamilyKind::SubcktParameterResolution => {
                Self::validate_subckt_parameter_resolution_dc_plan(plan)
            }
            XyceBaselineFamilyKind::NestedIncludeIdentity => {
                Self::validate_nested_include_identity_dc_plan(plan)
            }
            other => Err(format!(
                "family kind {} has no qualified exact-DC plan contract",
                other.name()
            )),
        }
    }

    pub(super) fn validate_abm_lookup_order_dc_plan(plan: &XyceStaticDcPlan) -> Result<(), String> {
        let exact_grid = matches!(
            &plan.dc.mode,
            DcSweepMode::List(values)
                if values.len() == XYCE_ABM_LOOKUP_ORDER_GRID.len()
                    && values
                        .iter()
                        .zip(XYCE_ABM_LOOKUP_ORDER_GRID)
                        .all(|(actual, expected)| actual.to_bits() == expected.to_bits())
        );
        if !plan.steps.is_empty()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || plan.dc.sweep2.is_some()
            || !plan.diagnostics.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VA")
            || !exact_grid
            || plan.print.probes != ["V(1)".to_string()]
        {
            return Err(
                "ABM_SPLINES lookup ordering requires one diagnostic-free, unstepped '.DC VA LIST 0.0 0.1 ... 1.0' analysis with sole ordered V(1) default-PRN probe"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_source_multiplicity_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if !plan.steps.is_empty()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.diagnostics.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.start.to_bits() != 1.0f64.to_bits()
            || plan.dc.stop.to_bits() != 12.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.print.probes != ["V(1)".to_string(), "V(2)".to_string(), "V(3)".to_string()]
        {
            return Err(
                "source multiplicity requires one diagnostic-free, unstepped '.DC VIN 1 12 1' analysis with ordered V(1), V(2), V(3) default-PRN probes"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_passive_res_primary_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if !plan.steps.is_empty()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
        {
            return Err(
                "resistor primary-value parity requires one unstepped linear default .prn DC sweep"
                    .to_string(),
            );
        }
        let span = plan.dc.stop - plan.dc.start;
        if !plan.dc.start.is_finite()
            || !plan.dc.stop.is_finite()
            || !plan.dc.step.is_finite()
            || plan.dc.step == 0.0
            || span == 0.0
            || span.signum() != plan.dc.step.signum()
            || (span / plan.dc.step).abs() < 1.0
        {
            return Err("resistor primary-value parity requires a finite directed DC sweep with at least two points".to_string());
        }
        if plan.print.probes.len() != 2 || !plan.diagnostics.is_empty() {
            return Err("resistor primary-value parity requires exactly two ordered probes and a diagnostic-free parse".to_string());
        }
        Self::validate_passive_primary_source_forms(
            &plan.source,
            XycePassivePrimaryKind::ResistorDc,
        )
    }

    pub(super) fn validate_subckt_parameter_precedence_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if !plan.steps.is_empty()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
            || !plan.diagnostics.is_empty()
        {
            return Err(
                "subcircuit-parameter precedence requires one diagnostic-free, unstepped, linear default .prn DC analysis"
                    .to_string(),
            );
        }
        if plan.dc.primary_spec().points().len() != 1 {
            return Err(
                "subcircuit-parameter precedence requires exactly one DC sweep point".to_string(),
            );
        }
        if plan.print.probes.len() != 2
            || plan
                .print
                .probes
                .iter()
                .any(|probe| Self::parse_voltage_probe(probe).is_none())
        {
            return Err(
                "subcircuit-parameter precedence requires exactly two ordered voltage probes"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_subckt_parameter_resolution_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "subcircuit-parameter resolution";
        if !matches!(plan.expression_dialect, ExpressionDialect::Xyce)
            || plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
        {
            return Err(format!(
                "{LABEL} requires one diagnostic-free, unstepped, one-dimensional linear .DC analysis with default indexed PRN output"
            ));
        }
        let span = plan.dc.stop - plan.dc.start;
        let estimated_intervals = span / plan.dc.step;
        if !plan.dc.start.is_finite()
            || !plan.dc.stop.is_finite()
            || !plan.dc.step.is_finite()
            || plan.dc.step == 0.0
            || !span.is_finite()
            || span == 0.0
            || span.signum() != plan.dc.step.signum()
            || !estimated_intervals.is_finite()
            || !(1.0..=1_000_000.0).contains(&estimated_intervals.abs())
        {
            return Err(format!(
                "{LABEL} requires finite directed sweep bounds with at most 1,000,001 points"
            ));
        }
        let points = plan.dc.primary_spec().points();
        if points.len() < 2 || points.iter().any(|point| !point.is_finite()) {
            return Err(format!(
                "{LABEL} requires a finite DC grid with at least two points"
            ));
        }
        let [voltage_text, current_text] = plan.print.probes.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one ordered voltage probe and one ordered branch-current probe"
            ));
        };
        let voltage = Self::parse_voltage_probe(voltage_text)
            .ok_or_else(|| format!("{LABEL} first output must be an ordinary voltage probe"))?;
        if voltage.accessor != XyceVoltageAccessor::Value
            || voltage.node_pos.trim().is_empty()
            || voltage.node_neg.is_some()
            || Self::parse_current_probe(current_text).is_none()
        {
            return Err(format!(
                "{LABEL} requires '.PRINT DC V(node) I(source)' in that order"
            ));
        }
        Self::validate_subckt_parameter_resolution_source_directives(&plan.source)
    }

    pub(super) fn validate_dc_analysis_expression_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "DC-analysis expression parity";
        if !matches!(plan.expression_dialect, ExpressionDialect::Xyce)
            || plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.print.probes.is_empty()
        {
            return Err(format!(
                "{LABEL} requires one diagnostic-free native Xyce .DC sweep with default indexed PRN output, no execution-directory override, no .DATA, and no .STEP"
            ));
        }
        if plan.dc.primary_spec().points().is_empty()
            || plan
                .dc
                .sweep2
                .as_ref()
                .is_some_and(|sweep| sweep.spec().points().is_empty())
        {
            return Err(format!("{LABEL} requires finite nonempty DC sweep grids"));
        }
        Self::dc_analysis_source_qualification(&plan.source).map(|_| ())
    }

    pub(super) fn validate_bjt_external_node_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if !plan.steps.is_empty() {
            return Err("exact BJT external-node DC does not admit .STEP".to_string());
        }
        if plan.dc_data.is_some() {
            return Err("exact BJT external-node DC does not admit .DC DATA".to_string());
        }
        if plan.print_format.is_some() {
            return Err(
                "exact BJT external-node DC requires ordinary primary .prn output without a format override"
                    .to_string(),
            );
        }
        if plan.dc.sweep2.is_some() {
            return Err("exact BJT external-node DC requires one sweep dimension".to_string());
        }
        if !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear) {
            return Err("exact BJT external-node DC requires a linear sweep".to_string());
        }
        if !plan.dc.start.is_finite()
            || !plan.dc.stop.is_finite()
            || !plan.dc.step.is_finite()
            || plan.dc.step <= 0.0
            || plan.dc.stop < plan.dc.start
        {
            return Err(format!(
                "exact BJT external-node DC requires a finite ascending sweep, got start={}, stop={}, step={}",
                plan.dc.start, plan.dc.stop, plan.dc.step
            ));
        }
        if plan.print.probes.is_empty() {
            return Err("exact BJT external-node DC requires at least one probe".to_string());
        }
        let mut probes = BTreeSet::new();
        for probe in &plan.print.probes {
            let normalized = Self::normalize_probe(probe);
            if !probes.insert(normalized) {
                return Err(format!(
                    "exact BJT external-node DC does not admit duplicate probe '{probe}'"
                ));
            }
        }
        if !plan.diagnostics.is_empty() {
            return Err(format!(
                "exact BJT external-node DC requires a diagnostic-free parse, found {} diagnostic(s)",
                plan.diagnostics.len()
            ));
        }

        let mut model_count = 0usize;
        let mut dc_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        for line in Self::logical_netlist_lines(&plan.source) {
            let stripped = Self::strip_netlist_comment(&line);
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".model" => model_count += 1,
                ".dc" => dc_count += 1,
                ".print" => print_count += 1,
                ".end" => end_count += 1,
                other => {
                    return Err(format!(
                        "exact BJT external-node DC does not admit directive '{other}'"
                    ));
                }
            }
        }
        if (model_count, dc_count, print_count, end_count) != (1, 1, 1, 1) {
            return Err(format!(
                "exact BJT external-node DC requires exactly one .MODEL, .DC, .PRINT, and .END; found ({model_count}, {dc_count}, {print_count}, {end_count})"
            ));
        }
        Self::validate_bjt_external_node_print_contract(&plan.source)?;
        Ok(())
    }

    pub(super) fn validate_bjt_external_node_dc_probes(
        print: &XycePrintRequest,
        netlist: &Netlist,
    ) -> Result<(), String> {
        for probe in &print.probes {
            if Self::parse_voltage_probe(probe)
                .is_some_and(|probe| probe.accessor == XyceVoltageAccessor::Value)
            {
                continue;
            }
            if Self::parse_current_probe(probe)
                .is_some_and(|source| Self::source_is_voltage_source(netlist, &source))
            {
                continue;
            }
            return Err(format!(
                "exact BJT external-node DC probe '{probe}' is not an atomic voltage or independent voltage-source current"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_tecplot_step_bindings(
        zones: &[XyceTecplotZone],
        steps: &[StepCommand],
        runs: &[XyceStepRun],
    ) -> Result<(), String> {
        if zones.len() != runs.len() {
            return Err(format!(
                "TECPLOT oracle has {} zones, but .STEP expansion produced {} runs",
                zones.len(),
                runs.len()
            ));
        }
        for (step_index, (zone, run)) in zones.iter().zip(runs).enumerate() {
            if run.step_values.len() != steps.len() {
                return Err(format!(
                    ".STEP run {} has {} values for {} commands",
                    step_index + 1,
                    run.step_values.len(),
                    steps.len()
                ));
            }
            if zone.auxdata.len() != steps.len() {
                return Err(format!(
                    "TECPLOT zone {} has {} AUXDATA bindings for {} .STEP commands",
                    step_index + 1,
                    zone.auxdata.len(),
                    steps.len()
                ));
            }
            for (command, actual) in steps.iter().zip(&run.step_values) {
                let expected = zone
                    .auxdata
                    .iter()
                    .find_map(|(name, binding)| {
                        name.eq_ignore_ascii_case(&command.name).then_some(*binding)
                    })
                    .ok_or_else(|| {
                        format!(
                            "TECPLOT zone {} has no AUXDATA binding for .STEP '{}'",
                            step_index + 1,
                            command.name
                        )
                    })?;
                if !Self::tecplot_binding_matches(expected, *actual) {
                    return Err(format!(
                        "TECPLOT zone {} binds {}={}, but expanded .STEP value is {}",
                        step_index + 1,
                        command.name,
                        expected.value,
                        actual
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_static_dc_contract(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        let dimensions = Self::dc_sweep_dimensions(netlist);
        if dimensions.is_empty() {
            Self::validate_dc_sweep_source(netlist, &dc.source)?;
            if let Some(sweep2) = &dc.sweep2 {
                Self::validate_dc_sweep_source(netlist, &sweep2.source)?;
            }
        } else {
            for dimension in &dimensions {
                Self::validate_dc_sweep_source(netlist, &dimension.source)?;
            }
        }

        let mut probe_index = Self::dc_probe_index(netlist);
        for probe in &print.probes {
            Self::validate_dc_probe_with_index(probe, netlist, &mut probe_index)?;
        }

        Self::reject_unsupported_static_dc_model_observables(netlist, print)?;
        Self::reject_unsupported_vbic_nested_current_source_sweeps(netlist, dc, print)?;

        Ok(())
    }

    pub(super) fn validate_static_dc_data_contract(
        netlist: &Netlist,
        dc_data: &XyceDcDataSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if dc_data.rows.is_empty() {
            return Err(".DC DATA sweep produced no table rows".to_string());
        }

        let mut probe_index = Self::dc_probe_index(netlist);
        for probe in &print.probes {
            Self::validate_dc_probe_with_index(probe, netlist, &mut probe_index)?;
        }
        Self::reject_unsupported_static_dc_model_observables(netlist, print)?;

        Ok(())
    }

    pub(super) fn validate_static_step_diode_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let element = netlist
            .elements
            .iter()
            .find(|element| Self::device_instance_names_match(&element.name, element_name))
            .ok_or_else(|| format!("diode '{}' not found", element_name))?;
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return Err(format!("element '{}' is not a diode", element_name));
        };

        if element.nodes.len() != 2 {
            return Err(format!(
                "native .STEP .PRINT TRAN comparison requires diode '{}' to have exactly two terminals",
                element_name
            ));
        }
        if !deferred_params.is_empty() {
            return Err(format!(
                "native .STEP .PRINT TRAN comparison does not support unresolved diode instance parameters on '{}'",
                element_name
            ));
        }
        for (name, value) in instance_params {
            if !Self::native_xyce_level2_diode_instance_param(name, *value) {
                return Err(format!(
                    "native .STEP .PRINT TRAN comparison does not support diode '{}' instance parameter {}={} in the native Level=2 temperature-breakdown envelope",
                    element_name, name, value
                ));
            }
        }
        if !Self::native_xyce_level2_diode_effective_temperature_is_valid(netlist, instance_params)
        {
            return Err(format!(
                "native .STEP .PRINT TRAN comparison requires diode '{}' to have one non-conflicting TEMP or DTEMP whose effective temperature remains above absolute zero",
                element_name
            ));
        }

        let model_def = Self::find_unique_model_in(&netlist.models, model).ok_or_else(|| {
            format!(
                "native .STEP .PRINT TRAN comparison requires diode '{}' to reference one unique model '{}', but none or multiple matching models were found",
                element_name, model
            )
        })?;
        if !Self::model_is_native_xyce_level2_tbv_diode(model_def) {
            return Err(format!(
                "native .STEP .PRINT TRAN comparison requires diode '{}' model '{}' to be a finite numeric Xyce/HSPICE LEVEL=2 D model with TBV1/TBV2 in the native subset",
                element_name, model
            ));
        }
        Ok(())
    }

    pub(super) fn validate_static_step_resistor_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let resistance = Self::effective_resistor_value(netlist, element_name)?.ok_or_else(|| {
            format!(
                "native .STEP .PRINT TRAN comparison could not resolve resistor '{}' to a static resistance",
                element_name
            )
        })?;
        if resistance.is_finite() || (resistance.is_infinite() && resistance.is_sign_positive()) {
            Ok(())
        } else {
            Err(format!(
                "native .STEP .PRINT TRAN comparison does not support resistor '{}' with invalid resistance {}",
                element_name, resistance
            ))
        }
    }

    pub(super) fn validate_static_step_capacitor_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        Self::validate_xyce_capacitor_contract_params(netlist, element_name)?;
        if Self::capacitor_uses_solution_dependent_value(netlist, element_name) {
            return Ok(());
        }
        let capacitance = Engine::new(SimulationConfig {
            spice_dialect: SpiceDialect::Xyce,
            ..SimulationConfig::default()
        })
        .resolved_capacitor_value(netlist, element_name)
        .map_err(|err| {
            format!(
                "native .PRINT TRAN comparison could not resolve capacitor '{}' to a static capacitance: {}",
                element_name, err
            )
        })?
        .ok_or_else(|| {
            format!(
                "native .PRINT TRAN comparison could not resolve capacitor '{}' to a static capacitance",
                element_name
            )
        })?;
        if capacitance.is_finite() && capacitance >= 0.0 {
            Ok(())
        } else {
            Err(format!(
                "native .PRINT TRAN comparison does not support capacitor '{}' with invalid capacitance {}",
                element_name, capacitance
            ))
        }
    }

    pub(super) fn validate_static_step_inductor_contract(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let inductance = Self::effective_inductor_value(netlist, element_name).ok_or_else(|| {
            format!(
                "native .STEP .PRINT TRAN comparison could not resolve inductor '{}' to a static inductance",
                element_name
            )
        })?;
        if inductance.is_finite() && inductance > 0.0 {
            Ok(())
        } else {
            Err(format!(
                "native .STEP .PRINT TRAN comparison does not support inductor '{}' with invalid inductance {}",
                element_name, inductance
            ))
        }
    }

    pub(super) fn validate_static_step_coupling_contract(
        netlist: &Netlist,
        element_name: &str,
        inductors: &[String],
        coefficient: Value,
        model: Option<&str>,
    ) -> Result<(), String> {
        if let Some(model_name) = model {
            if inductors.is_empty() {
                return Err(format!(
                    "native .PRINT TRAN comparison does not support nonlinear coupling '{}' without a winding",
                    element_name,
                ));
            }
            if !coefficient.is_finite() || !(0.0..=1.0).contains(&coefficient) {
                return Err(format!(
                    "native .PRINT TRAN comparison does not support nonlinear coupling '{}' with invalid coefficient {}",
                    element_name, coefficient
                ));
            }
            let Some(model_def) = Self::find_model(&netlist.models, model_name) else {
                return Err(format!(
                    "native .PRINT TRAN comparison could not resolve nonlinear Core model '{}' for coupling '{}'",
                    model_name, element_name
                ));
            };
            if !model_def.model_type.eq_ignore_ascii_case("CORE") {
                return Err(format!(
                    "native .PRINT TRAN comparison requires coupling '{}' model '{}' to be CORE",
                    element_name, model_name
                ));
            }
            for inductor_name in inductors {
                Self::validate_static_step_inductor_contract(netlist, inductor_name).map_err(
                    |err| {
                        format!(
                            "native .PRINT TRAN comparison does not support nonlinear coupling '{}' because referenced inductor '{}' is not a supported inductor: {}",
                            element_name, inductor_name, err
                        )
                    },
                )?;
            }
            return Ok(());
        }
        if inductors.len() < 2 {
            return Err(format!(
                "native .PRINT TRAN comparison does not support coupling '{}' with fewer than two inductors",
                element_name
            ));
        }
        if !coefficient.is_finite() || !(0.0..=1.0).contains(&coefficient) {
            return Err(format!(
                "native .PRINT TRAN comparison does not support coupling '{}' with invalid coefficient {}",
                element_name, coefficient
            ));
        }

        for inductor_name in inductors {
            Self::validate_static_step_inductor_contract(netlist, inductor_name).map_err(
                |err| {
                    format!(
                        "native .PRINT TRAN comparison does not support coupling '{}' because referenced inductor '{}' is not a supported linear inductor: {}",
                        element_name, inductor_name, err
                    )
                },
            )?;
        }
        Ok(())
    }

    pub(super) fn reject_unsupported_vbic_nested_current_source_sweeps(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        let Some(sweep2) = &dc.sweep2 else {
            return Ok(());
        };
        if !Self::netlist_uses_native_vbic_bjt(netlist) {
            return Ok(());
        }

        let bias_points = dc.primary_spec().points().len() * sweep2.spec().points().len();
        if bias_points <= 1000 {
            return Ok(());
        }

        for probe in &print.probes {
            if Self::dc_probe_references_current_source_current(probe, netlist)? {
                return Err(format!(
                    "native VBIC nested DC sweep with {bias_points} bias points and current-source branch-current probes exceeds the current Xyce harness execution envelope; keep this named unsupported until VBIC nested-sweep continuation/performance is production-ready"
                ));
            }
        }

        Ok(())
    }

    pub(super) fn reject_unsupported_static_dc_model_observables(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        if !Self::netlist_uses_ekv3_level301_mosfet(netlist) {
            return Ok(());
        }

        let mut has_voltage_source_current_probe = false;
        for probe in &print.probes {
            if Self::dc_probe_references_voltage_source_current(probe, netlist)? {
                has_voltage_source_current_probe = true;
                break;
            }
        }
        if !has_voltage_source_current_probe {
            return Ok(());
        }

        if Self::netlist_uses_unsupported_ekv3_level301_branch_current_model(netlist) {
            return Err(
                "EKV3 LEVEL=301 static .PRINT DC voltage-source branch-current probes require a native validated EKV3 150 nm model; unsupported EKV3 LEVEL=301 cards remain fail-closed"
                    .to_string(),
            );
        }

        Ok(())
    }

    pub(super) fn validate_dc_sweep_source(netlist: &Netlist, source: &str) -> Result<(), String> {
        if Self::source_is_independent_source(netlist, source)
            || source.eq_ignore_ascii_case("TEMP")
            || source.eq_ignore_ascii_case("TEMPER")
            || Self::scalar_parameter_sweep_source_is_supported(netlist, source)
            || Engine::canonical_device_parameter_sweep_source(netlist, source).is_some()
        {
            return Ok(());
        }
        Err(format!(
            "DC sweep source '{}' is not a supported top-level independent source, scalar parameter, passive device parameter, or TEMP sweep",
            source
        ))
    }

    #[cfg(test)]
    pub(super) fn validate_dc_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        let mut probe_index = Self::dc_probe_index(netlist);
        Self::validate_dc_probe_with_index(probe, netlist, &mut probe_index)
    }

    pub(super) fn validate_dc_probe_with_index(
        probe: &str,
        netlist: &Netlist,
        probe_index: &mut XyceDcProbeIndex,
    ) -> Result<(), String> {
        if Self::probe_names_live_measurement(probe, netlist, "DC", "DC_CONT") {
            return Ok(());
        }
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::validate_atomic_dc_probe_with_index(
                    &normalized_expression,
                    expression,
                    netlist,
                    probe_index,
                );
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::validate_dc_probe_expression_with_index(
                    expression,
                    netlist,
                    probe_index,
                );
            }
            let context = Self::print_eval_context(netlist, None, None);
            rspice_core::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!("unsupported .PRINT DC expression '{{{expression}}}': {err}")
            })?;
            return Ok(());
        }

        let normalized = Self::normalize_probe(probe);
        Self::validate_atomic_dc_probe_with_index(&normalized, probe, netlist, probe_index)
    }

    pub(super) fn validate_atomic_dc_probe(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let mut probe_index = Self::dc_probe_index(netlist);
        Self::validate_atomic_dc_probe_with_index(normalized, original, netlist, &mut probe_index)
    }

    pub(super) fn validate_atomic_dc_probe_with_index(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
        probe_index: &mut XyceDcProbeIndex,
    ) -> Result<(), String> {
        if let Some(voltage_probe) = Self::parse_tran_voltage_probe(normalized)
            && !voltage_probe.node_pos.is_empty()
            && voltage_probe
                .node_neg
                .as_deref()
                .is_none_or(|node| !node.is_empty())
        {
            return Ok(());
        }
        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
        {
            if parameter.eq_ignore_ascii_case("R")
                && Self::find_native_xyce_memristor_element(netlist, &element_name).is_some()
            {
                return Ok(());
            }
            if !Self::netlist_has_device_op_instance(netlist, &element_name) {
                return Err(format!(
                    "device operating-point probe '{}' targets an unknown reported device",
                    original
                ));
            }
            if Self::canonical_device_op_parameter(&parameter).is_some() {
                return Ok(());
            }
            return Err(format!(
                "device operating-point probe '{}' targets an unsupported operating-point parameter",
                original
            ));
        }
        if let Some(lead_current) = Self::parse_lead_current_probe(normalized) {
            if Self::lead_current_probe_is_omitted_empty_wildcard(netlist, &lead_current) {
                return Ok(());
            }
            if lead_current.element_name == "*" {
                return Err(format!(
                    "lead-current wildcard probe '{}' requires terminal expansion support",
                    original
                ));
            }
            if !Self::netlist_has_device_op_instance(netlist, &lead_current.element_name) {
                return Err(format!(
                    "lead-current probe '{}' targets an unknown reported device",
                    original
                ));
            }
            if Self::netlist_supports_lead_current_probe(netlist, &lead_current) {
                return Ok(());
            }
            return Err(format!(
                "lead-current probe '{}' targets unsupported {} terminal current",
                original,
                lead_current.terminal.function_name()
            ));
        }
        if Self::bare_device_parameter_probe_is_supported(netlist, normalized) {
            return Ok(());
        }
        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            if Self::semiconductor_instance_parameter_probe_is_supported(
                netlist,
                &element_name,
                &parameter,
            ) {
                return Ok(());
            }
            match parameter.as_str() {
                "dcv0" if Self::source_is_independent_source(netlist, &element_name) => {
                    return Ok(());
                }
                "r" => {
                    if Self::find_resistor_element(netlist, &element_name).is_some()
                        || Self::find_native_xyce_memristor_element(netlist, &element_name)
                            .is_some()
                    {
                        return Ok(());
                    }
                }
                "c" => {
                    if Self::find_capacitor_element(netlist, &element_name).is_some() {
                        return Ok(());
                    }
                }
                "l" => {
                    if Self::find_inductor_element(netlist, &element_name).is_some() {
                        return Ok(());
                    }
                    if Self::resistor_instance_parameter_probe_is_supported(
                        netlist,
                        &element_name,
                        &parameter,
                    ) {
                        return Ok(());
                    }
                }
                "temp" if Self::resistor_temperature_value(netlist, &element_name)?.is_some() => {
                    return Ok(());
                }
                _ => {
                    if Self::resistor_instance_parameter_probe_is_supported(
                        netlist,
                        &element_name,
                        &parameter,
                    ) {
                        return Ok(());
                    }
                }
            }
            if Self::model_parameter_probe_is_supported(netlist, &element_name, &parameter) {
                return Ok(());
            }
            return Err(format!(
                "device parameter probe '{}' targets an unsupported parameter",
                original
            ));
        }
        if let Some(parameter_name) = Self::parse_scalar_parameter_probe(normalized)
            && Self::scalar_parameter_probe_is_supported(netlist, &parameter_name)
        {
            return Ok(());
        }
        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if Self::netlist_has_recorded_branch_current_with_index(
                netlist,
                &element_name,
                probe_index,
            ) {
                return Ok(());
            }
            if Self::netlist_has_diode_instance_with_index(netlist, &element_name, probe_index) {
                return Ok(());
            }
            if Self::source_is_current_source(netlist, &element_name) {
                return Ok(());
            }
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name)? {
                if resistance.is_finite()
                    || (resistance.is_infinite() && resistance.is_sign_positive())
                {
                    return Ok(());
                }
                return Err(format!(
                    "current probe '{}' targets a resistor with invalid resistance {}",
                    original, resistance
                ));
            }
            if Self::find_resistor_element(netlist, &element_name).is_some() {
                // A solution-dependent resistor cannot have a scalar resistance
                // reconstructed during validation. Its constitutive law is
                // validated by the circuit builder and its accepted current is
                // exported through the result's typed device observables.
                return Ok(());
            }
            return Err(format!(
                "current probe '{}' targets an unsupported branch/device",
                original
            ));
        }
        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if Self::find_native_xyce_memristor_element(netlist, &element_name).is_some() {
                return Ok(());
            }
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name)? {
                if resistance.is_finite()
                    || (resistance.is_infinite() && resistance.is_sign_positive())
                {
                    return Ok(());
                }
                return Err(format!(
                    "power probe '{}' targets a resistor with invalid resistance {}",
                    original, resistance
                ));
            }
            if Self::find_resistor_element(netlist, &element_name).is_some() {
                return Ok(());
            }
            return Err(format!(
                "power probe '{}' targets an unsupported branch/device",
                original
            ));
        }
        if netlist.measurements.iter().any(|measurement| {
            measurement.analysis.eq_ignore_ascii_case("DC")
                && measurement.name.eq_ignore_ascii_case(original)
                && matches!(
                    measurement.measure_type,
                    rspice_core::analysis::MeasureType::Avg { .. }
                        | rspice_core::analysis::MeasureType::Rms { .. }
                        | rspice_core::analysis::MeasureType::Equation { .. }
                )
        }) {
            return Ok(());
        }
        Err(format!("unsupported .PRINT DC probe '{}'", original))
    }

    pub(super) fn validate_dc_probe_expression(
        expression: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let mut probe_index = Self::dc_probe_index(netlist);
        Self::validate_dc_probe_expression_with_index(expression, netlist, &mut probe_index)
    }

    pub(super) fn validate_dc_probe_expression_with_index(
        expression: &str,
        netlist: &Netlist,
        probe_index: &mut XyceDcProbeIndex,
    ) -> Result<(), String> {
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::validate_atomic_dc_probe_with_index(&normalized, call, netlist, probe_index)?;
            Ok(1.0)
        };
        let context = Self::print_eval_context(netlist, None, None);
        match Self::evaluate_print_expression_with_probe_calls(
            expression,
            context.clone(),
            &mut call_value,
        ) {
            Ok(_) => Ok(()),
            Err(err) if err.eq_ignore_ascii_case("division by zero") => {
                // Probe calls are replaced with placeholders while the static
                // contract is validated.  Giving every call the same value is
                // useful for catching malformed constant arithmetic, but it
                // can create a zero denominator for a valid expression such
                // as `(V(a)-V(b))/(V(c)-V(d))`.  Retry only that specific
                // arithmetic failure with stable, distinct values per probe.
                // Every retry still validates each probe through
                // `validate_atomic_dc_probe`, so unresolved or unsupported
                // probes cannot be hidden by the retry.
                let mut probe_values = BTreeMap::<String, Value>::new();
                let mut distinct_call_value = |call: &str| {
                    let normalized = Self::normalize_probe(call);
                    Self::validate_atomic_dc_probe_with_index(
                        &normalized,
                        call,
                        netlist,
                        probe_index,
                    )?;
                    let candidate = 1.0 + probe_values.len() as Value;
                    let value = *probe_values.entry(normalized).or_insert(candidate);
                    Ok(value)
                };
                match Self::evaluate_print_expression_with_probe_calls(
                    expression,
                    context,
                    &mut distinct_call_value,
                ) {
                    Ok(_) => Ok(()),
                    Err(_) => Err(format!(
                        "unsupported .PRINT DC expression '{{{expression}}}': {err}"
                    )),
                }
            }
            Err(err) => Err(format!(
                "unsupported .PRINT DC expression '{{{expression}}}': {err}"
            )),
        }
    }

    pub(super) fn validate_nested_include_identity_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if plan.execution_dir.is_some()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
        {
            return Err("nested-include identity requires one diagnostic-free, one-dimensional linear .DC analysis with ordinary default PRN output".to_string());
        }
        let points = plan.dc.primary_spec().points();
        if points.len() != 3
            || points.iter().any(|point| !point.is_finite())
            || !plan.dc.start.is_finite()
            || !plan.dc.stop.is_finite()
            || !plan.dc.step.is_finite()
            || plan.dc.step == 0.0
            || plan.print.probes.len() != 2
        {
            return Err("nested-include identity requires one finite three-point linear sweep and exactly two ordered DC probes".to_string());
        }
        for line in Self::logical_netlist_lines(&plan.source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            if stripped.is_empty() {
                continue;
            }
            let head = stripped.split_whitespace().next().unwrap_or("");
            if !(head.eq_ignore_ascii_case(".subckt")
                || head.eq_ignore_ascii_case(".ends")
                || head.eq_ignore_ascii_case(".include")
                || head.eq_ignore_ascii_case(".inc")
                || head.eq_ignore_ascii_case(".incl")
                || head.eq_ignore_ascii_case(".dc")
                || head.eq_ignore_ascii_case(".print")
                || head.eq_ignore_ascii_case(".end")
                || matches!(
                    head.chars().next().map(|ch| ch.to_ascii_uppercase()),
                    Some('R' | 'V' | 'X')
                ))
            {
                return Err(format!(
                    "nested-include identity does not admit source statement '{stripped}'"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn numbered_redefinition_dc_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceNumberedRedefinitionDcFamilyContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        if !relative.starts_with("netlists/") {
            return None;
        }
        let relative_parent = relative.rsplit_once('/')?.0;
        let deck_stem = deck.path.file_stem()?.to_str()?.to_ascii_lowercase();
        let manifest_anchors = self
            .upstream_wrapper_decks
            .iter()
            .filter_map(|path| {
                let (parent, file_name) = path.rsplit_once('/')?;
                if parent != relative_parent {
                    return None;
                }
                let stem = file_name.strip_suffix(".cir")?;
                Some((file_name, stem.to_ascii_lowercase()))
            })
            .collect::<Vec<_>>();
        let manifest_named_candidate = manifest_anchors.len() == 2
            && manifest_anchors.iter().any(|(_, anchor)| {
                deck_stem == *anchor
                    || deck_stem.strip_prefix(anchor).is_some_and(|suffix| {
                        suffix.len() == 1 && suffix.as_bytes()[0].is_ascii_digit()
                    })
            });
        if !manifest_named_candidate {
            return None;
        }
        let parent = deck.path.parent()?;
        let candidate_stems = fs::read_dir(parent)
            .ok()?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().ok().is_some_and(|kind| kind.is_file()))
            .filter_map(|entry| {
                let path = entry.path();
                path.extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                    .then(|| {
                        path.file_stem()
                            .and_then(|stem| stem.to_str())
                            .map(str::to_ascii_lowercase)
                    })
                    .flatten()
            })
            .collect::<Vec<_>>();
        let numbered_counts = manifest_anchors
            .iter()
            .map(|(_, anchor)| {
                candidate_stems
                    .iter()
                    .filter(|stem| {
                        stem.strip_prefix(anchor).is_some_and(|suffix| {
                            suffix.len() == 1 && suffix.as_bytes()[0].is_ascii_digit()
                        })
                    })
                    .count()
            })
            .collect::<Vec<_>>();
        if numbered_counts.len() != 2
            || numbered_counts.iter().any(|count| !(2..=3).contains(count))
        {
            return None;
        }

        Some((|| {
            let entries = fs::read_dir(parent)
                .map_err(|err| format!("failed to read candidate family directory: {err}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| format!("failed to enumerate candidate family directory: {err}"))?;
            let mut cir_paths = Vec::new();
            let mut source_artifacts = Vec::new();
            let mut folded_names = BTreeSet::new();
            for entry in entries {
                if !entry.file_type().map_err(|err| err.to_string())?.is_file() {
                    return Err("family directory may contain only regular files".to_string());
                }
                let path = entry.path();
                let name = path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .ok_or_else(|| "family directory contains a non-UTF-8 name".to_string())?;
                if !folded_names.insert(name.to_ascii_lowercase()) {
                    return Err("case-colliding family filenames are not admissible".to_string());
                }
                if path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                {
                    cir_paths.push(path);
                } else if name.to_ascii_lowercase().ends_with(".cir.res.gs") {
                    source_artifacts.push(path);
                } else {
                    return Err(format!("unexpected source-side family artifact '{}'", name));
                }
            }
            if cir_paths.len() != 11 || source_artifacts.len() != 1 {
                return Err(format!(
                    "family census requires eleven .cir files and one .cir.res.gs artifact, found {} and {}",
                    cir_paths.len(),
                    source_artifacts.len()
                ));
            }

            let mut anchors = Vec::new();
            for (file_name, _) in &manifest_anchors {
                let path = cir_paths
                    .iter()
                    .find(|path| {
                        path.file_name()
                            .and_then(|name| name.to_str())
                            .is_some_and(|name| name.eq_ignore_ascii_case(file_name))
                    })
                    .cloned()
                    .ok_or_else(|| format!("manifest anchor '{file_name}' is missing"))?;
                let source = fs::read_to_string(&path)
                    .map_err(|err| format!("failed to read manifest anchor: {err}"))?;
                if !source.trim().is_empty() {
                    return Err(format!(
                        "manifest anchor '{}' must contain only whitespace",
                        path.display()
                    ));
                }
                anchors.push(path);
            }
            anchors.sort_by_key(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().to_ascii_lowercase())
            });

            let mut claimed = BTreeSet::new();
            let mut groups = Vec::new();
            for owner_path in anchors {
                let owner_stem = owner_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| "family anchor has a non-UTF-8 stem".to_string())?;
                let folded_owner = owner_stem.to_ascii_lowercase();
                let mut numbered = BTreeMap::new();
                for path in &cir_paths {
                    if Self::same_path(path, &owner_path) {
                        continue;
                    }
                    let stem = path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .ok_or_else(|| "family member has a non-UTF-8 stem".to_string())?
                        .to_ascii_lowercase();
                    let Some(suffix) = stem.strip_prefix(&folded_owner) else {
                        continue;
                    };
                    if suffix.len() != 1 || !suffix.as_bytes()[0].is_ascii_digit() {
                        continue;
                    }
                    let index = usize::from(suffix.as_bytes()[0] - b'0');
                    if numbered.insert(index, path.clone()).is_some() {
                        return Err(format!(
                            "family '{owner_stem}' has duplicate member index {index}"
                        ));
                    }
                }
                if numbered.keys().copied().ne(0..3) {
                    return Err(format!(
                        "family '{owner_stem}' requires contiguous members 0, 1, and 2"
                    ));
                }
                let members = numbered.into_values().collect::<Vec<_>>();
                if !claimed.insert(owner_path.clone())
                    || members.iter().any(|path| !claimed.insert(path.clone()))
                {
                    return Err("a circuit is claimed by more than one numbered family".to_string());
                }
                groups.push((owner_path, members));
            }

            let standalone = cir_paths
                .iter()
                .filter(|path| !claimed.contains(*path))
                .cloned()
                .collect::<Vec<_>>();
            if standalone.len() != 3 || claimed.len() != 8 {
                return Err(
                    "directory must contain two four-record numbered families and three standalone oracle decks"
                        .to_string(),
                );
            }
            if standalone.iter().any(|path| {
                self.requires_upstream_wrapper(&self.relative_key(path))
                    || fs::read_to_string(path)
                        .ok()
                        .is_none_or(|source| source.trim().is_empty())
            }) {
                return Err(
                    "standalone oracle decks must be nonempty non-wrapper circuits".to_string(),
                );
            }
            let artifact_name = source_artifacts[0]
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| "source artifact has a non-UTF-8 name".to_string())?;
            if standalone
                .iter()
                .filter(|path| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| {
                            artifact_name.eq_ignore_ascii_case(&format!("{name}.res.gs"))
                        })
                })
                .count()
                != 1
            {
                return Err(
                    "the sole source-side .res.gs artifact must belong to exactly one standalone deck"
                        .to_string(),
                );
            }

            let first_reference = self
                .static_output_reference_path(&standalone[0], "prn")
                .ok_or_else(|| "cannot map standalone deck into OutputData".to_string())?;
            let output_parent = first_reference
                .parent()
                .ok_or_else(|| "OutputData mapping has no parent".to_string())?;
            let output_entries = fs::read_dir(output_parent)
                .map_err(|err| format!("failed to read family OutputData directory: {err}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| format!("failed to enumerate family OutputData: {err}"))?;
            let mut actual_outputs = BTreeSet::new();
            for entry in output_entries {
                if !entry.file_type().map_err(|err| err.to_string())?.is_file() {
                    return Err("family OutputData may contain only regular files".to_string());
                }
                let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
                if !name.ends_with(".cir.prn") || !actual_outputs.insert(name) {
                    return Err(
                        "family OutputData must contain unique default-PRN artifacts only"
                            .to_string(),
                    );
                }
            }
            let expected_outputs = standalone
                .iter()
                .map(|path| {
                    format!(
                        "{}.prn",
                        path.file_name()
                            .unwrap()
                            .to_string_lossy()
                            .to_ascii_lowercase()
                    )
                })
                .collect::<BTreeSet<_>>();
            if actual_outputs != expected_outputs {
                return Err(
                    "family OutputData census must contain exactly one .prn per standalone deck"
                        .to_string(),
                );
            }
            for path in &standalone {
                let plan = self.static_dc_plan_for_path(path, ExpressionDialect::Xyce)?;
                if plan.print.probes.is_empty() || !plan.diagnostics.is_empty() {
                    return Err(format!(
                        "standalone deck '{}' is not a diagnostic-free static DC plan",
                        path.display()
                    ));
                }
            }

            let mut selected_contract = None;
            let mut selected_policies = Vec::new();
            for (owner_path, member_paths) in groups {
                for path in &member_paths {
                    if self.requires_upstream_wrapper(&self.relative_key(path))
                        || fs::read_to_string(path)
                            .map_err(|err| format!("failed to read numbered member: {err}"))?
                            .trim()
                            .is_empty()
                    {
                        return Err(
                            "numbered members must be nonempty non-wrapper decks".to_string()
                        );
                    }
                    for extension in ["prn", "res"] {
                        if self
                            .static_output_reference_path(path, extension)
                            .is_some_and(|oracle| oracle.exists())
                        {
                            return Err(format!(
                                "numbered member '{}' must not own a .{extension} oracle",
                                path.display()
                            ));
                        }
                    }
                }

                let baseline_source = fs::read_to_string(&member_paths[0])
                    .map_err(|err| format!("failed to read numbered baseline: {err}"))?;
                if !Self::top_level_literal_parameter_definitions(&baseline_source)?.is_empty() {
                    return Err(
                        "numbered baseline must not define top-level literal parameters"
                            .to_string(),
                    );
                }
                let first_definitions = Self::top_level_literal_parameter_definitions(
                    &fs::read_to_string(&member_paths[1]).map_err(|err| err.to_string())?,
                )?;
                let second_definitions = Self::top_level_literal_parameter_definitions(
                    &fs::read_to_string(&member_paths[2]).map_err(|err| err.to_string())?,
                )?;
                if first_definitions != second_definitions
                    || first_definitions
                        .values()
                        .filter(|values| {
                            values.len() == 2 && values[0].to_bits() != values[1].to_bits()
                        })
                        .count()
                        != 1
                    || first_definitions
                        .values()
                        .any(|values| values.is_empty() || values.len() > 2)
                {
                    return Err(
                        "parameterized members must share one literal-definition map with exactly one distinct two-value duplicate"
                            .to_string(),
                    );
                }

                let mut matching_policies = Vec::new();
                for policy in [
                    ParameterRedefinitionPolicy::UseFirst,
                    ParameterRedefinitionPolicy::UseLast,
                ] {
                    let mut plans = Vec::new();
                    let mut snapshots = Vec::new();
                    let mut representations = Vec::new();
                    for path in &member_paths {
                        let plan = self.static_dc_plan_for_path_with_redefinition_policy(
                            path,
                            ExpressionDialect::Xyce,
                            policy,
                        )?;
                        Self::validate_numbered_redefinition_dc_plan(&plan)?;
                        let netlist =
                            Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
                                &plan.source,
                                path,
                                ExpressionDialect::Xyce,
                                policy,
                                None,
                            )
                            .map_err(|err| format!("failed to parse numbered member: {err}"))?;
                        representations.push(Self::numbered_redefinition_representation(&netlist)?);
                        snapshots.push(Self::numbered_redefinition_snapshot(
                            &netlist,
                            &plan.dc.source,
                        )?);
                        plans.push(plan);
                    }
                    if representations
                        != [
                            XyceNumberedRedefinitionRepresentation::LiteralBaseline,
                            XyceNumberedRedefinitionRepresentation::DependentFormalExpression,
                            XyceNumberedRedefinitionRepresentation::DependentInstanceExpression,
                        ]
                    {
                        return Err(
                            "numbered members must use baseline, dependent-formal, and dependent-instance representations in order"
                                .to_string(),
                        );
                    }
                    let baseline_plan = &plans[0];
                    if plans.iter().skip(1).any(|plan| {
                        plan.print.probes != baseline_plan.print.probes
                            || !Self::dc_sweeps_match_exactly(&plan.dc, &baseline_plan.dc)
                    }) {
                        return Err(
                            "numbered members must share identical PRINT and DC contracts"
                                .to_string(),
                        );
                    }
                    if snapshots
                        .iter()
                        .skip(1)
                        .all(|snapshot| snapshot == &snapshots[0])
                    {
                        matching_policies.push(policy);
                    }
                }
                let [policy] = matching_policies.as_slice() else {
                    return Err(
                        "numbered family must select exactly one first/last redefinition policy"
                            .to_string(),
                    );
                };
                selected_policies.push(*policy);

                if Self::same_path(&deck.path, &owner_path)
                    || member_paths
                        .iter()
                        .any(|path| Self::same_path(path, &deck.path))
                {
                    let role = member_paths
                        .iter()
                        .position(|path| Self::same_path(path, &deck.path))
                        .map_or(XyceNumberedRedefinitionDcFamilyRole::Owner, |index| {
                            if index == 0 {
                                XyceNumberedRedefinitionDcFamilyRole::Baseline
                            } else {
                                XyceNumberedRedefinitionDcFamilyRole::Member(index)
                            }
                        });
                    selected_contract = Some(XyceNumberedRedefinitionDcFamilyContract {
                        family: owner_path
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        owner_path,
                        baseline_path: member_paths[0].clone(),
                        member_paths,
                        parameter_redefinition_policy: *policy,
                        role,
                    });
                }
            }
            if selected_policies.len() != 2 || selected_policies[0] == selected_policies[1] {
                return Err(
                    "the two numbered families must select opposite redefinition policies"
                        .to_string(),
                );
            }
            selected_contract.ok_or_else(|| {
                "requested deck is not claimed by either numbered family".to_string()
            })
        })())
    }

    pub(super) fn validate_numbered_redefinition_dc_plan(
        plan: &XyceStaticDcPlan,
    ) -> Result<(), String> {
        if plan.expression_dialect != ExpressionDialect::Xyce
            || plan.execution_dir.is_some()
            || !plan.steps.is_empty()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, rspice_core::netlist::DcSweepMode::Linear)
            || !plan.diagnostics.is_empty()
            || plan.print.probes.is_empty()
        {
            return Err(
                "numbered redefinition family requires one diagnostic-free, unstepped, one-dimensional linear default-PRN DC plan"
                    .to_string(),
            );
        }
        let points = plan.dc.primary_spec().points();
        if points.len() != 1 || points.iter().any(|point| !point.is_finite()) {
            return Err(
                "numbered redefinition family requires exactly one finite DC sweep point"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn shared_stepped_dc_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceSharedSteppedDcFamilyContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        if !relative.starts_with("netlists/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let relative_parent = relative.rsplit_once('/')?.0;
        let deck_stem = deck
            .path
            .file_stem()
            .and_then(|stem| stem.to_str())?
            .to_ascii_lowercase();
        let manifest_anchor_files = self
            .upstream_wrapper_decks
            .iter()
            .filter_map(|path| {
                let (candidate_parent, file_name) = path.rsplit_once('/')?;
                (candidate_parent == relative_parent)
                    .then(|| file_name.strip_suffix(".cir").map(|stem| (file_name, stem)))
                    .flatten()
            })
            .collect::<Vec<_>>();
        let manifest_named_candidate = manifest_anchor_files.len() == 2
            && manifest_anchor_files.iter().any(|(_, anchor)| {
                deck_stem == *anchor
                    || deck_stem.strip_prefix(anchor).is_some_and(|suffix| {
                        suffix.len() == 1 && suffix.as_bytes()[0].is_ascii_digit()
                    })
            });
        let manifest_family_candidate = manifest_named_candidate
            && (|| {
                let expected = manifest_anchor_files
                    .iter()
                    .flat_map(|(file_name, _)| {
                        [
                            format!("{file_name}.prn").to_ascii_lowercase(),
                            format!("{file_name}.res").to_ascii_lowercase(),
                        ]
                    })
                    .collect::<BTreeSet<_>>();
                let output_subdirectory = Path::new(&deck.relative_path)
                    .parent()?
                    .strip_prefix("Netlists")
                    .ok()?;
                let output_parent = self.root.join("OutputData").join(output_subdirectory);
                let mut actual = BTreeSet::new();
                for entry in fs::read_dir(output_parent).ok()? {
                    let entry = entry.ok()?;
                    if !entry.file_type().ok()?.is_file() {
                        return None;
                    }
                    let name = entry.file_name().to_str()?.to_ascii_lowercase();
                    if !actual.insert(name) {
                        return None;
                    }
                }
                (actual == expected).then_some(())
            })()
            .is_some();
        let entries = match fs::read_dir(parent) {
            Ok(entries) => entries,
            Err(err) if manifest_family_candidate => {
                return Some(Err(format!(
                    "failed to read candidate shared stepped-DC family directory: {err}"
                )));
            }
            Err(_) => return None,
        };
        let mut cir_paths = Vec::new();
        let mut empty_cir_paths = BTreeSet::new();
        let mut folded_names = BTreeSet::new();
        let mut unexpected_entries = 0_usize;
        for entry in entries {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    unexpected_entries += 1;
                    continue;
                }
            };
            let path = entry.path();
            let is_circuit = path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"));
            let Ok(file_type) = entry.file_type() else {
                unexpected_entries += 1;
                continue;
            };
            if !file_type.is_file() || !is_circuit {
                unexpected_entries += 1;
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                unexpected_entries += 1;
                continue;
            };
            let metadata = match entry.metadata() {
                Ok(metadata) => metadata,
                Err(_) => {
                    unexpected_entries += 1;
                    continue;
                }
            };
            if !folded_names.insert(name.to_ascii_lowercase()) {
                return Some(Err(
                    "case-colliding .cir names are not admissible".to_string()
                ));
            }
            if metadata.len() == 0 {
                empty_cir_paths.insert(path.clone());
            }
            cir_paths.push(path);
        }
        if !cir_paths
            .iter()
            .any(|path| Self::same_path(path, &deck.path))
        {
            if manifest_family_candidate {
                return Some(Err(
                    "candidate shared stepped-DC family does not contain its requested physical record"
                        .to_string(),
                ));
            }
            return None;
        }
        let manifest_anchors = cir_paths
            .iter()
            .filter(|path| {
                empty_cir_paths.contains(*path)
                    && self.requires_upstream_wrapper(&self.relative_key(path))
            })
            .cloned()
            .collect::<Vec<_>>();
        if manifest_anchors.len() != 2 {
            if manifest_family_candidate {
                return Some(Err(
                    "candidate shared stepped-DC family must retain both zero-byte manifest anchors"
                        .to_string(),
                ));
            }
            return None;
        }
        let physical_family_candidate = cir_paths.len() == 8
            && (|| {
                let mut claimed = BTreeSet::new();
                let mut sizes = Vec::new();
                for owner_path in &manifest_anchors {
                    let owner_stem = owner_path.file_stem()?.to_str()?.to_ascii_lowercase();
                    let mut indices = BTreeSet::new();
                    claimed.insert(owner_path.clone());
                    for path in &cir_paths {
                        if Self::same_path(path, owner_path) {
                            continue;
                        }
                        let stem = path.file_stem()?.to_str()?.to_ascii_lowercase();
                        let Some(suffix) = stem.strip_prefix(&owner_stem) else {
                            continue;
                        };
                        if suffix.len() != 1 || !suffix.as_bytes()[0].is_ascii_digit() {
                            continue;
                        }
                        let index = usize::from(suffix.as_bytes()[0] - b'0');
                        if !indices.insert(index) || !claimed.insert(path.clone()) {
                            return None;
                        }
                    }
                    if !matches!(indices.len(), 2 | 4)
                        || indices.iter().copied().ne(0..indices.len())
                    {
                        return None;
                    }
                    sizes.push(indices.len());
                }
                sizes.sort_unstable();
                (sizes == [2, 4] && claimed.len() == cir_paths.len()).then_some(())
            })()
            .is_some();
        if !manifest_family_candidate && !physical_family_candidate {
            return None;
        }

        Some((|| {
            if unexpected_entries > 0 {
                return Err(format!(
                    "shared stepped-DC family directory contains {unexpected_entries} unexpected, unreadable, non-.cir, or non-regular entries"
                ));
            }
            if cir_paths.len() != 8 {
                return Err(format!(
                    "directory census requires exactly eight .cir files, found {}",
                    cir_paths.len()
                ));
            }
            if cir_paths.iter().any(|path| {
                let empty = empty_cir_paths.contains(path);
                let wrapper = self.requires_upstream_wrapper(&self.relative_key(path));
                empty != wrapper
            }) {
                return Err(
                    "only the two zero-byte family anchors may carry wrapper-manifest contracts"
                        .to_string(),
                );
            }

            let mut anchors = manifest_anchors;
            anchors.sort_by_key(|path| {
                path.file_name()
                    .map(|name| name.to_string_lossy().to_ascii_lowercase())
            });
            let mut groups = Vec::new();
            let mut claimed = BTreeSet::new();
            for owner_path in anchors {
                let owner_stem = owner_path
                    .file_stem()
                    .and_then(|stem| stem.to_str())
                    .ok_or_else(|| "family anchor has a non-UTF-8 stem".to_string())?;
                let owner_folded = owner_stem.to_ascii_lowercase();
                let mut numbered = BTreeMap::new();
                for path in &cir_paths {
                    if Self::same_path(path, &owner_path) {
                        continue;
                    }
                    let stem = path
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .ok_or_else(|| "family member has a non-UTF-8 stem".to_string())?;
                    let stem_folded = stem.to_ascii_lowercase();
                    let Some(suffix) = stem_folded.strip_prefix(&owner_folded) else {
                        continue;
                    };
                    if suffix.len() != 1 || !suffix.as_bytes()[0].is_ascii_digit() {
                        continue;
                    }
                    let index = usize::from(suffix.as_bytes()[0] - b'0');
                    if numbered.insert(index, path.clone()).is_some() {
                        return Err(format!(
                            "family '{}' has duplicate numeric member index {index}",
                            owner_stem
                        ));
                    }
                }
                if !matches!(numbered.len(), 2 | 4)
                    || numbered.keys().copied().ne(0..numbered.len())
                {
                    return Err(format!(
                        "family '{}' requires contiguous one-digit members 0..N with cardinality two or four",
                        owner_stem
                    ));
                }
                let members = numbered.into_values().collect::<Vec<_>>();
                if !claimed.insert(owner_path.clone())
                    || members.iter().any(|path| !claimed.insert(path.clone()))
                {
                    return Err("a .cir file is claimed by more than one family".to_string());
                }
                groups.push((owner_path, members));
            }
            if claimed.len() != cir_paths.len() {
                return Err("every .cir file must be claimed by exactly one anchor".to_string());
            }
            let mut sizes = groups
                .iter()
                .map(|(_, members)| members.len())
                .collect::<Vec<_>>();
            sizes.sort_unstable();
            if sizes != [2, 4] {
                return Err(
                    "directory must contain exactly one two-member and one four-member family"
                        .to_string(),
                );
            }

            let expected_artifacts = groups
                .iter()
                .flat_map(|(owner, _)| {
                    let name = owner.file_name().unwrap().to_string_lossy();
                    [
                        format!("{name}.prn").to_ascii_lowercase(),
                        format!("{name}.res").to_ascii_lowercase(),
                    ]
                })
                .collect::<BTreeSet<_>>();
            let first_output = self
                .static_output_reference_path(&groups[0].0, "prn")
                .ok_or_else(|| "cannot map anchor into OutputData".to_string())?;
            let output_parent = first_output
                .parent()
                .ok_or_else(|| "anchor OutputData path has no parent".to_string())?;
            let output_entries = fs::read_dir(output_parent)
                .map_err(|err| format!("failed to read shared OutputData directory: {err}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| format!("failed to enumerate shared OutputData directory: {err}"))?;
            let mut actual_artifacts = BTreeSet::new();
            for entry in output_entries {
                if !entry.file_type().map_err(|err| err.to_string())?.is_file() {
                    return Err(
                        "shared OutputData directory may contain only four anchor files"
                            .to_string(),
                    );
                }
                let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
                if !actual_artifacts.insert(name) {
                    return Err(
                        "case-colliding shared OutputData artifacts are not admissible".to_string(),
                    );
                }
            }
            if actual_artifacts != expected_artifacts {
                return Err(
                    "shared OutputData census must contain exactly each anchor's .prn and .res"
                        .to_string(),
                );
            }

            let mut selected = None;
            for (owner_path, member_paths) in groups {
                let belongs = Self::same_path(&deck.path, &owner_path)
                    || member_paths
                        .iter()
                        .any(|path| Self::same_path(path, &deck.path));
                let mut plans = Vec::with_capacity(member_paths.len());
                let mut netlists = Vec::with_capacity(member_paths.len());
                let mut topologies = Vec::with_capacity(member_paths.len());
                for member_path in &member_paths {
                    if fs::metadata(member_path)
                        .map_err(|err| format!("failed to inspect family member: {err}"))?
                        .len()
                        == 0
                        || self.requires_upstream_wrapper(&self.relative_key(member_path))
                    {
                        return Err("numbered family members must be nonempty non-wrapper decks"
                            .to_string());
                    }
                    for extension in ["prn", "res"] {
                        if self
                            .static_output_reference_path(member_path, extension)
                            .is_some_and(|path| path.exists())
                        {
                            return Err(format!(
                                "numbered member '{}' must not own a .{extension} oracle",
                                member_path.display()
                            ));
                        }
                    }
                    let plan =
                        self.static_dc_plan_for_path(member_path, ExpressionDialect::Xyce)?;
                    if plan.execution_dir.is_some()
                        || plan.print_format.is_some()
                        || plan.dc_data.is_some()
                        || plan.steps.is_empty()
                        || plan.print.probes.is_empty()
                        || !plan.diagnostics.is_empty()
                    {
                        return Err(format!(
                            "member '{}' is not a diagnostic-free default-PRN stepped-DC plan",
                            member_path.display()
                        ));
                    }
                    let netlist = Self::parse_xyce_netlist(&plan.source, member_path)
                        .map_err(|err| format!("failed to parse family member: {err}"))?;
                    topologies.push(Self::shared_stepped_dc_topology(&netlist)?);
                    netlists.push(netlist);
                    plans.push(plan);
                }
                let (representations, expected) = if member_paths.len() == 2 {
                    let direct =
                        Self::shared_stepped_dc_representation(&netlists[0], &plans[0], None)?;
                    let hierarchical =
                        Self::shared_stepped_dc_representation(&netlists[1], &plans[1], None)?;
                    (
                        vec![direct.0, hierarchical.0],
                        vec![
                            XyceSharedSteppedDcRepresentation::DirectIdentity,
                            XyceSharedSteppedDcRepresentation::HierarchicalIdentity,
                        ],
                    )
                } else {
                    let direct =
                        Self::shared_stepped_dc_representation(&netlists[0], &plans[0], None)?;
                    let transform = direct.1.as_deref().ok_or_else(|| {
                        "four-member family baseline does not define a non-identity transform"
                            .to_string()
                    })?;
                    let mut found = vec![direct.0];
                    for (netlist, plan) in netlists.iter().zip(&plans).skip(1) {
                        found.push(
                            Self::shared_stepped_dc_representation(netlist, plan, Some(transform))?
                                .0,
                        );
                    }
                    (
                        found,
                        vec![
                            XyceSharedSteppedDcRepresentation::DirectTransform,
                            XyceSharedSteppedDcRepresentation::TransformInSubcircuitBody,
                            XyceSharedSteppedDcRepresentation::FunctionCallInSubcircuitBody,
                            XyceSharedSteppedDcRepresentation::FunctionCallAtInstance,
                        ],
                    )
                };
                if representations != expected {
                    return Err(format!(
                        "family '{}' does not contain the required ordered semantic representations",
                        owner_path.display()
                    ));
                }
                if topologies.into_iter().collect::<BTreeSet<_>>().len() != 1 {
                    return Err(format!(
                        "family '{}' members do not share identical ordered source/load topology",
                        owner_path.display()
                    ));
                }
                let baseline_plan = &plans[0];
                if plans.iter().skip(1).any(|plan| {
                    plan.print.probes != baseline_plan.print.probes
                        || !Self::dc_sweeps_match_exactly(&plan.dc, &baseline_plan.dc)
                        || !Self::step_commands_match_exactly(&plan.steps, &baseline_plan.steps)
                }) {
                    return Err(format!(
                        "family '{}' members do not share identical PRINT, DC, and STEP policies",
                        owner_path.display()
                    ));
                }
                let global_bits = netlists
                    .iter()
                    .zip(&plans)
                    .map(|(netlist, plan)| {
                        let step_name = &plan.steps[0].name;
                        netlist
                            .params
                            .numeric_parameters()
                            .into_iter()
                            .find(|(name, _)| name.eq_ignore_ascii_case(step_name))
                            .map(|(_, value)| value.to_bits())
                    })
                    .collect::<Option<BTreeSet<_>>>()
                    .ok_or_else(|| {
                        "every member must retain its stepped global's initial value".to_string()
                    })?;
                if global_bits.len() != 1 {
                    return Err(
                        "all members must share one identical stepped-global initial value"
                            .to_string(),
                    );
                }
                let formal_default_bits = netlists
                    .iter()
                    .filter_map(|netlist| netlist.subcircuits.first())
                    .map(|subcircuit| subcircuit.params[0].1.to_bits())
                    .collect::<BTreeSet<_>>();
                if formal_default_bits.len() != 1 {
                    return Err(
                        "all hierarchical members must share one identical formal default"
                            .to_string(),
                    );
                }
                let source_bits = netlists
                    .iter()
                    .filter_map(|netlist| {
                        netlist
                            .elements
                            .iter()
                            .find_map(|element| match element.kind {
                                ElementKind::VoltageSource(
                                    rspice_core::netlist::SourceSpec::Dc(value),
                                ) => Some(value.to_bits()),
                                _ => None,
                            })
                    })
                    .collect::<BTreeSet<_>>();
                if source_bits.len() != 1 {
                    return Err(
                        "all members must share one identical finite DC source value".to_string(),
                    );
                }
                let prn_reference_path = self
                    .static_output_reference_path(&owner_path, "prn")
                    .filter(|path| path.is_file())
                    .ok_or_else(|| "family anchor is missing its shared .prn oracle".to_string())?;
                let res_reference_path = self
                    .static_output_reference_path(&owner_path, "res")
                    .filter(|path| path.is_file())
                    .ok_or_else(|| "family anchor is missing its shared .res oracle".to_string())?;
                if belongs {
                    let member_index = member_paths
                        .iter()
                        .position(|path| Self::same_path(path, &deck.path));
                    let role = match member_index {
                        None => XyceSharedSteppedDcFamilyRole::Owner,
                        Some(0) => XyceSharedSteppedDcFamilyRole::Baseline(representations[0]),
                        Some(index) => {
                            XyceSharedSteppedDcFamilyRole::Member(representations[index])
                        }
                    };
                    selected = Some(XyceSharedSteppedDcFamilyContract {
                        family: owner_path
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        baseline_path: member_paths[0].clone(),
                        owner_path,
                        member_paths,
                        prn_reference_path,
                        res_reference_path,
                        role,
                    });
                }
            }
            selected
                .ok_or_else(|| "deck is not claimed by the qualified directory census".to_string())
        })())
    }

    pub(super) fn stepped_ic_reference_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceSteppedIcReferenceContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/") {
            return None;
        }
        let parent = deck.path.parent()?;
        let stem = deck.path.file_stem()?.to_str()?;
        let stem_lower = stem.to_ascii_lowercase();
        let family = if let Some(family) = stem_lower.strip_suffix("_step") {
            (!family.is_empty()).then_some(family.to_string())?
        } else {
            let digit_start = stem_lower
                .char_indices()
                .rev()
                .find(|(_, ch)| !ch.is_ascii_digit())
                .map_or(0, |(index, ch)| index + ch.len_utf8());
            if digit_start == 0 || digit_start == stem_lower.len() {
                return None;
            }
            stem_lower[..digit_start].to_string()
        };
        let owner_stem = format!("{family}_step");
        let mut owner_path = None;
        let mut numbered = BTreeMap::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !entry.file_type().ok()?.is_file()
                || !path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            let candidate_stem = path.file_stem()?.to_str()?.to_ascii_lowercase();
            if candidate_stem == owner_stem {
                if owner_path.replace(path).is_some() {
                    return None;
                }
                continue;
            }
            let Some(index_text) = candidate_stem.strip_prefix(&family) else {
                continue;
            };
            if index_text.is_empty() || !index_text.bytes().all(|byte| byte.is_ascii_digit()) {
                continue;
            }
            let index = index_text.parse::<usize>().ok()?;
            if numbered.insert(index, path).is_some() {
                return None;
            }
        }
        let owner_path = owner_path?;
        if !self.requires_upstream_wrapper(&self.relative_key(&owner_path)) {
            return None;
        }
        if numbered.len() < 2 || numbered.keys().copied().ne(0..numbered.len()) {
            return None;
        }
        let member_paths = numbered.into_values().collect::<Vec<_>>();
        if member_paths.iter().any(|path| {
            self.requires_upstream_wrapper(&self.relative_key(path))
                || fs::metadata(path)
                    .ok()
                    .is_none_or(|metadata| !metadata.is_file() || metadata.len() == 0)
        }) || fs::metadata(&owner_path)
            .ok()
            .is_none_or(|metadata| !metadata.is_file() || metadata.len() == 0)
        {
            return None;
        }
        if !Self::same_path(&deck.path, &owner_path)
            && !member_paths
                .iter()
                .any(|path| Self::same_path(path, &deck.path))
        {
            return None;
        }

        Some(XyceSteppedIcReferenceContract {
            family,
            owner_path,
            member_paths,
            target_path: deck.path.clone(),
        })
    }

    pub(super) fn nonlinear_core_model_step_reference_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceNonlinearCoreModelStepReferenceContract, String>> {
        if deck.section != XyceDeckSection::Netlists {
            return None;
        }
        let parent = deck.path.parent()?;
        let exclusions = match &self.upstream_exclusions {
            Ok(exclusions) => exclusions,
            Err(error) => {
                return Some(Err(format!(
                    "nonlinear CORE model-step exclusion manifest is invalid: {error}"
                )));
            }
        };
        let entries = fs::read_dir(parent)
            .ok()?
            .collect::<Result<Vec<_>, _>>()
            .ok()?;
        let mut claims = Vec::new();
        for entry in &entries {
            let owner_path = entry.path();
            if !entry.file_type().ok()?.is_file()
                || !owner_path
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                || !self.requires_upstream_wrapper(&self.relative_key(&owner_path))
            {
                continue;
            }
            let owner_stem = owner_path.file_stem()?.to_str()?.to_string();
            let owner_stem_lower = owner_stem.to_ascii_lowercase();
            let mut numbered = Vec::new();
            for candidate in &entries {
                let path = candidate.path();
                if !candidate.file_type().ok()?.is_file()
                    || !path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                    || Self::same_path(&path, &owner_path)
                {
                    continue;
                }
                let stem = path.file_stem()?.to_str()?.to_ascii_lowercase();
                let Some(suffix) = stem.strip_prefix(&owner_stem_lower) else {
                    continue;
                };
                let (underscored, digits) = suffix
                    .strip_prefix('_')
                    .map_or((false, suffix), |digits| (true, digits));
                if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
                    continue;
                }
                numbered.push((underscored, digits.parse::<usize>().ok()?, path));
            }
            let candidate_shape = numbered.iter().any(|(_, _, path)| {
                let relative = Self::normalize_manifest_key(&self.relative_key(path));
                matches!(
                    exclusions.get(&relative).map(|record| &record.disposition),
                    Some(XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                        expected_contract,
                    }) if expected_contract == XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT
                )
            });
            if candidate_shape
                && (Self::same_path(&deck.path, &owner_path)
                    || numbered
                        .iter()
                        .any(|(_, _, path)| Self::same_path(&deck.path, path)))
            {
                claims.push((owner_path, owner_stem, numbered));
            }
        }
        let [(owner_path, family, numbered)] = claims.as_slice() else {
            return if claims.is_empty() {
                None
            } else {
                Some(Err(format!(
                    "nonlinear CORE model-step record belongs to {} wrapper families",
                    claims.len()
                )))
            };
        };

        Some((|| {
            let owner_path = owner_path.clone();
            let family = family.clone();
            let mut numbered = numbered.clone();
            numbered.sort_by_key(|(_, index, _)| *index);
            if numbered.len() != 3
                || numbered.iter().map(|(_, index, _)| *index).ne(1..=3)
                || numbered
                    .iter()
                    .map(|(underscored, _, _)| *underscored)
                    .collect::<BTreeSet<_>>()
                    .len()
                    != 1
            {
                return Err(format!(
                    "nonlinear CORE model-step family '{family}' requires exactly one consistent contiguous 1/2/3 baseline sequence, found {numbered:?}"
                ));
            }
            let member_paths = numbered
                .into_iter()
                .map(|(_, _, path)| path)
                .collect::<Vec<_>>();
            let mut all_paths = Vec::with_capacity(4);
            all_paths.push(owner_path.clone());
            all_paths.extend(member_paths.iter().cloned());
            for (index, path) in all_paths.iter().enumerate() {
                let metadata = fs::symlink_metadata(path).map_err(|error| {
                    format!("nonlinear CORE model-step member metadata failed: {error}")
                })?;
                if !metadata.file_type().is_file()
                    || metadata.file_type().is_symlink()
                    || metadata.len() == 0
                {
                    return Err(format!(
                        "nonlinear CORE model-step member '{}' must be a nonempty regular non-symlink file",
                        self.display_path(path)
                    ));
                }
                let relative = Self::normalize_manifest_key(&self.relative_key(path));
                if (index == 0) != self.requires_upstream_wrapper(&relative) {
                    return Err(format!(
                        "nonlinear CORE model-step family '{family}' requires owner-only wrapper provenance"
                    ));
                }
                self.reject_wrapper_output_artifacts(path)?;
            }
            let owner_relative = Self::normalize_manifest_key(&self.relative_key(&owner_path));
            if exclusions.contains_key(&owner_relative) {
                return Err(format!(
                    "nonlinear CORE model-step owner '{}' must not be upstream-excluded",
                    self.display_path(&owner_path)
                ));
            }
            for path in &member_paths {
                let relative = Self::normalize_manifest_key(&self.relative_key(path));
                let exclusion = exclusions.get(&relative).ok_or_else(|| {
                    format!(
                        "independent nonlinear CORE baseline '{}' lost upstream exclusion provenance",
                        self.display_path(path)
                    )
                })?;
                if !matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT
                ) {
                    return Err(format!(
                        "independent nonlinear CORE baseline '{}' is not promoted under contract '{}'",
                        self.display_path(path),
                        XYCE_NONLINEAR_CORE_MODEL_STEP_BASELINE_CONTRACT
                    ));
                }
            }
            if !all_paths
                .iter()
                .any(|path| Self::same_path(path, &deck.path))
                || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                    != Self::normalize_manifest_key(&deck.relative_path)
            {
                return Err(
                    "nonlinear CORE model-step request is not one canonical physical family member"
                        .to_string(),
                );
            }
            Ok(XyceNonlinearCoreModelStepReferenceContract {
                family,
                owner_path,
                member_paths,
                target_path: deck.path.clone(),
            })
        })())
    }

    pub(super) fn analytic_integer_dc_wrapper_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceAnalyticIntegerDcContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let kind = match relative.as_str() {
            XYCE_ANALYTIC_FMOD_DC_RECORD => XyceAnalyticIntegerKind::Fmod,
            XYCE_ANALYTIC_INT_FLOOR_CEIL_DC_RECORD => XyceAnalyticIntegerKind::IntFloorCeil,
            _ => return None,
        };
        Some((|| {
            if !self.requires_upstream_wrapper(&deck.relative_path) {
                return Err("record is not owned by the wrapper provenance manifest".to_string());
            }
            let metadata = fs::metadata(&deck.path)
                .map_err(|err| format!("could not inspect wrapper deck: {err}"))?;
            if !metadata.is_file() || metadata.len() == 0 {
                return Err("wrapper deck must be a nonempty regular file".to_string());
            }
            self.reject_wrapper_output_artifacts(&deck.path)?;
            let plan = self.static_dc_plan_for_path(&deck.path, ExpressionDialect::Xyce)?;
            let netlist =
                Self::parse_xyce_netlist(&plan.source, &plan.deck_path).map_err(|err| {
                    format!("netlist parser rejected analytic integer DC deck: {err}")
                })?;
            Self::validate_analytic_integer_dc_plan(&plan, &netlist, kind)?;
            Ok(XyceAnalyticIntegerDcContract { plan, kind })
        })())
    }

    pub(super) fn validate_analytic_integer_dc_plan(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        kind: XyceAnalyticIntegerKind,
    ) -> Result<(), String> {
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || plan.print_format.is_some()
            || !plan.diagnostics.is_empty()
        {
            return Err(
                "analytic integer DC wrapper requires default PRN output without execution overrides, DATA, STEP, or parser diagnostics"
                    .to_string(),
            );
        }
        if netlist.analyses.len() != 1
            || !matches!(netlist.analyses[0], AnalysisCommand::Dc { .. })
            || plan.dc.sweep2.is_some()
        {
            return Err(
                "analytic integer wrapper requires exactly one one-dimensional .DC analysis"
                    .to_string(),
            );
        }
        Self::validate_analytic_integer_dc_statement_envelope(&plan.source, kind)?;
        match kind {
            XyceAnalyticIntegerKind::Fmod => {
                Self::validate_analytic_fmod_dc_topology(plan, netlist)
            }
            XyceAnalyticIntegerKind::IntFloorCeil => {
                Self::validate_analytic_int_floor_ceil_dc_topology(plan, netlist)
            }
        }
    }

    pub(super) fn validate_analytic_integer_dc_statement_envelope(
        source: &str,
        kind: XyceAnalyticIntegerKind,
    ) -> Result<(), String> {
        let mut counts = BTreeMap::<char, usize>::new();
        // SPICE reserves the first physical record for the title, including
        // when that record begins with `*`. Strip it before applying the
        // ordinary logical-line/comment rules so both title forms preserve
        // the exact element census.
        let body = source.split_once('\n').map_or("", |(_, body)| body);
        for line in Self::logical_netlist_lines(body) {
            let statement = Self::strip_netlist_comment(&line).trim();
            if statement.is_empty() {
                continue;
            }
            let key = if statement.starts_with('.') {
                let directive = statement
                    .split_ascii_whitespace()
                    .next()
                    .unwrap_or_default()
                    .to_ascii_lowercase();
                match directive.as_str() {
                    ".dc" => 'd',
                    ".print" => 'p',
                    ".end" => 'e',
                    _ => {
                        return Err(format!(
                            "unrelated directive '{directive}' is outside the analytic integer DC envelope"
                        ));
                    }
                }
            } else {
                match statement.as_bytes().first().map(u8::to_ascii_lowercase) {
                    Some(b'r') => 'r',
                    Some(b'v') => 'v',
                    Some(b'b') => 'b',
                    _ => {
                        return Err(format!(
                            "unrelated element statement '{statement}' is outside the analytic integer DC envelope"
                        ));
                    }
                }
            };
            *counts.entry(key).or_default() += 1;
        }
        let expected = match kind {
            XyceAnalyticIntegerKind::Fmod => [('r', 1), ('v', 1), ('d', 1), ('p', 1), ('e', 1)],
            XyceAnalyticIntegerKind::IntFloorCeil => {
                [('r', 4), ('v', 1), ('d', 1), ('p', 1), ('e', 1)]
            }
        };
        for (key, count) in expected {
            if counts.remove(&key) != Some(count) {
                return Err(format!(
                    "analytic integer DC statement count for '{key}' must be {count}"
                ));
            }
        }
        if kind == XyceAnalyticIntegerKind::IntFloorCeil && counts.remove(&'b') != Some(3) {
            return Err("INT/FLOOR/CEIL wrapper requires exactly three B sources".to_string());
        }
        if !counts.is_empty() {
            return Err("analytic integer DC source contains extra statements".to_string());
        }
        Ok(())
    }

    pub(super) fn validate_analytic_fmod_dc_topology(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if plan.dc.start.to_bits() != 1.0f64.to_bits()
            || plan.dc.stop.to_bits() != 10.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.5f64.to_bits()
            || netlist.elements.len() != 2
            || plan.print.probes.len() != 2
        {
            return Err("FMOD wrapper requires the exact 1.0:0.5:10.0 sweep, two-element topology, and two probes".to_string());
        }
        let source = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .ok_or_else(|| "FMOD wrapper has no independent voltage source".to_string())?;
        let resistor = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::Resistor { .. }))
            .ok_or_else(|| "FMOD wrapper has no resistor".to_string())?;
        let source_nodes = source
            .nodes
            .iter()
            .map(|node| Self::canonical_param_expression_node_name(node))
            .collect::<Vec<_>>();
        let [signal, ground] = source_nodes.as_slice() else {
            return Err("FMOD source must be two-terminal".to_string());
        };
        if ground != "0"
            || !plan.dc.source.eq_ignore_ascii_case(&source.name)
            || !matches!(&source.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == 1.0f64.to_bits())
            || !Self::plain_unit_resistor_on_nodes(resistor, &[signal.clone(), ground.clone()])
        {
            return Err(
                "FMOD source/resistor topology or swept-source mapping changed".to_string(),
            );
        }
        let first = Self::normalize_probe(&plan.print.probes[0]);
        let second = Self::normalize_probe(&plan.print.probes[1]);
        if first != format!("v({signal})") || second != format!("{{fmod(99.5,v({signal}))}}") {
            return Err(
                "FMOD wrapper probes must be V(input) followed by fmod(99.5,V(input))".to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn validate_analytic_int_floor_ceil_dc_topology(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if plan.dc.start.to_bits() != (-1.0f64).to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.1f64.to_bits()
            || netlist.elements.len() != 8
            || plan.print.probes.len() != 4
        {
            return Err("INT/FLOOR/CEIL wrapper requires the exact -1:0.1:1 sweep, eight-element topology, and four probes".to_string());
        }
        let source = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .ok_or_else(|| {
                "INT/FLOOR/CEIL wrapper has no independent voltage source".to_string()
            })?;
        let source_nodes = source
            .nodes
            .iter()
            .map(|node| Self::canonical_param_expression_node_name(node))
            .collect::<Vec<_>>();
        let [input, ground] = source_nodes.as_slice() else {
            return Err("INT/FLOOR/CEIL input source must be two-terminal".to_string());
        };
        if ground != "0"
            || !plan.dc.source.eq_ignore_ascii_case(&source.name)
            || !matches!(&source.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == 1.0f64.to_bits())
        {
            return Err("INT/FLOOR/CEIL swept source mapping changed".to_string());
        }

        let mut behavioral_nodes = BTreeMap::new();
        for element in &netlist.elements {
            let ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            } = &element.kind
            else {
                continue;
            };
            let nodes = element
                .nodes
                .iter()
                .map(|node| Self::canonical_param_expression_node_name(node))
                .collect::<Vec<_>>();
            let [output, output_ground] = nodes.as_slice() else {
                return Err(format!(
                    "behavioral source '{}' must be two-terminal",
                    element.name
                ));
            };
            if output_ground != "0"
                || tc1.to_bits() != 0.0f64.to_bits()
                || tc2.to_bits() != 0.0f64.to_bits()
                || multiplicity.value.to_bits() != 1.0f64.to_bits()
                || multiplicity.value_expr.is_some()
                || multiplicity.given
            {
                return Err(format!(
                    "behavioral source '{}' changed topology or temperature coefficients",
                    element.name
                ));
            }
            let normalized = Self::normalize_probe(expression);
            let function = ["int", "floor", "ceil"]
                .into_iter()
                .find(|function| normalized == format!("{function}(v({input}))"))
                .ok_or_else(|| {
                    format!(
                        "behavioral source '{}' is not an exact INT/FLOOR/CEIL input mapping",
                        element.name
                    )
                })?;
            if behavioral_nodes.insert(function, output.clone()).is_some() {
                return Err(format!("duplicate {function} behavioral source"));
            }
        }
        if behavioral_nodes.len() != 3 {
            return Err(
                "INT/FLOOR/CEIL wrapper requires one behavioral source per function".to_string(),
            );
        }

        for node in std::iter::once(input.clone()).chain(behavioral_nodes.values().cloned()) {
            let expected_nodes = [node.clone(), "0".to_string()];
            let count = netlist
                .elements
                .iter()
                .filter(|element| Self::plain_unit_resistor_on_nodes(element, &expected_nodes))
                .count();
            if count != 1 {
                return Err(format!(
                    "node '{node}' must have exactly one unit resistor to literal ground"
                ));
            }
        }
        let expected_probes = [
            input.clone(),
            behavioral_nodes["int"].clone(),
            behavioral_nodes["floor"].clone(),
            behavioral_nodes["ceil"].clone(),
        ];
        for (probe, node) in plan.print.probes.iter().zip(expected_probes) {
            if Self::normalize_probe(probe) != format!("v({node})") {
                return Err(
                    "INT/FLOOR/CEIL print probes no longer map input/int/floor/ceil in order"
                        .to_string(),
                );
            }
        }
        Ok(())
    }

    pub(super) fn dc_analysis_expression_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        // Every member is classified as a baseline/wrapper pair, so a valid
        // family has an even number of circuit files and at least two pairs.
        // Check that shape before qualifying source text.
        let circuit_count = Self::circuit_file_count(parent)?;
        if circuit_count < 4 || circuit_count % 2 != 0 {
            return None;
        }
        let mut paths = Vec::new();
        let mut baseline_count = 0usize;
        let mut target_count = 0usize;
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let path = entry.path();
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            if !entry.file_type().ok()?.is_file()
                || fs::metadata(&path)
                    .ok()
                    .is_none_or(|metadata| metadata.len() == 0)
                || self
                    .static_prn_reference_path(&path)
                    .is_some_and(|reference| reference.is_file())
            {
                return None;
            }
            let member_relative = self.relative_key(&path);
            let wrapper = self.requires_upstream_wrapper(&member_relative);
            let source = fs::read_to_string(&path).ok()?;
            let (representation, _) = Self::dc_analysis_source_qualification(&source).ok()?;
            if wrapper != (representation == XyceDcAnalysisRepresentation::ParameterExpression) {
                return None;
            }
            if wrapper {
                target_count += 1;
            } else {
                baseline_count += 1;
            }
            paths.push(path);
        }
        if baseline_count < 2 || baseline_count != target_count || paths.len() != 2 * baseline_count
        {
            return None;
        }

        let mut pair_counts = BTreeMap::<(String, String), usize>::new();
        let mut selected = None;
        for path in &paths {
            let member = XyceDeck {
                path: path.clone(),
                relative_path: self.relative_key(path),
                section: XyceDeckSection::Netlists,
            };
            let contract = self.dc_analysis_expression_candidate_contract(&member)?;
            let pair = (
                Self::normalize_manifest_key(&self.relative_key(&contract.baseline_path)),
                Self::normalize_manifest_key(
                    &self.relative_key(
                        contract
                            .member_paths
                            .iter()
                            .find(|member| !Self::same_path(member, &contract.baseline_path))?,
                    ),
                ),
            );
            *pair_counts.entry(pair).or_default() += 1;
            if Self::same_path(path, &deck.path) {
                selected = Some(contract);
            }
        }
        if pair_counts.len() != baseline_count || pair_counts.values().any(|count| *count != 2) {
            return None;
        }
        selected
    }

    pub(super) fn dc_analysis_expression_candidate_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/")
            || self
                .static_prn_reference_path(&deck.path)
                .is_some_and(|reference| reference.is_file())
        {
            return None;
        }
        let parent = deck.path.parent()?;
        let source = fs::read_to_string(&deck.path).ok()?;
        let (representation, _) = Self::dc_analysis_source_qualification(&source).ok()?;
        let is_wrapper = self.requires_upstream_wrapper(&deck.relative_path);
        if is_wrapper != (representation == XyceDcAnalysisRepresentation::ParameterExpression) {
            return None;
        }
        let plan = self
            .static_dc_plan_for_path(&deck.path, ExpressionDialect::Xyce)
            .ok()?;
        Self::validate_dc_analysis_expression_plan(&plan).ok()?;
        let netlist = Self::parse_xyce_netlist(&plan.source, &deck.path).ok()?;
        let snapshot = Self::dc_analysis_expression_snapshot(&netlist).ok()?;

        let mut matches = Vec::new();
        for entry in fs::read_dir(parent).ok()? {
            let entry = entry.ok()?;
            let candidate = entry.path();
            if !entry.file_type().ok()?.is_file()
                || Self::same_path(&candidate, &deck.path)
                || !candidate
                    .extension()
                    .and_then(|extension| extension.to_str())
                    .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                || fs::metadata(&candidate)
                    .ok()
                    .is_none_or(|metadata| metadata.len() == 0)
                || self
                    .static_prn_reference_path(&candidate)
                    .is_some_and(|reference| reference.is_file())
            {
                continue;
            }
            let candidate_relative = self.relative_key(&candidate);
            let candidate_wrapper = self.requires_upstream_wrapper(&candidate_relative);
            if candidate_wrapper == is_wrapper {
                continue;
            }
            let candidate_source = fs::read_to_string(&candidate).ok()?;
            let Ok((candidate_representation, _)) =
                Self::dc_analysis_source_qualification(&candidate_source)
            else {
                continue;
            };
            if candidate_wrapper
                != (candidate_representation == XyceDcAnalysisRepresentation::ParameterExpression)
            {
                continue;
            }
            let Ok(candidate_plan) =
                self.static_dc_plan_for_path(&candidate, ExpressionDialect::Xyce)
            else {
                continue;
            };
            if Self::validate_dc_analysis_expression_plan(&candidate_plan).is_err()
                || plan.print.probes != candidate_plan.print.probes
                || plan.print_format != candidate_plan.print_format
                || !Self::dc_sweeps_match_exactly(&plan.dc, &candidate_plan.dc)
            {
                continue;
            }
            let Ok(candidate_netlist) =
                Self::parse_xyce_netlist(&candidate_plan.source, &candidate)
            else {
                continue;
            };
            let Ok(candidate_snapshot) = Self::dc_analysis_expression_snapshot(&candidate_netlist)
            else {
                continue;
            };
            let semantic_match = if is_wrapper {
                Self::compare_dc_analysis_expression_snapshots(&candidate_snapshot, &snapshot)
            } else {
                Self::compare_dc_analysis_expression_snapshots(&snapshot, &candidate_snapshot)
            };
            if semantic_match.is_ok() {
                matches.push(candidate);
            }
        }
        let [counterpart] = matches.as_slice() else {
            return None;
        };
        let (baseline_path, target_path) = if is_wrapper {
            (counterpart.clone(), deck.path.clone())
        } else {
            (deck.path.clone(), counterpart.clone())
        };
        let family = format!(
            "{}:{}",
            parent.file_name()?.to_str()?,
            baseline_path.file_stem()?.to_str()?
        );
        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::DcAnalysisExpression,
            // The canonical wrappers try a byte comparison before their DC
            // RMS fallback. These independently simulated representations are
            // deterministic and must serialize identically, so exact parity is
            // the intentional fail-closed refinement for this family.
            comparison: XyceBaselineFamilyComparison::ExactPrn,
            family,
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, target_path],
            target_path: Some(deck.path.clone()),
        })
    }

    pub(super) fn vbic_dc_wrapper_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceVbicDcWrapperFamilyContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let (relative_parent, file_name) = relative.rsplit_once('/')?;
        let exclusions = self.upstream_exclusions.as_ref().ok()?;
        let mut multiplicity_promotions = 0usize;
        let mut polarity_promotions = 0usize;
        let mut promoted_controls = 0usize;
        for (record, exclusion) in exclusions {
            let Some((record_parent, _)) = record.rsplit_once('/') else {
                continue;
            };
            if record_parent != relative_parent {
                continue;
            }
            let XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                expected_contract,
            } = &exclusion.disposition
            else {
                continue;
            };
            let role = if expected_contract
                == XyceVbicDcWrapperFamilyRole::MultiplicityControl.contract()
            {
                multiplicity_promotions += 1;
                true
            } else if expected_contract == XyceVbicDcWrapperFamilyRole::PolarityControl.contract() {
                polarity_promotions += 1;
                true
            } else {
                false
            };
            if role {
                promoted_controls += 1;
                if Self::normalize_manifest_key(&exclusion.source)
                    != format!("{relative_parent}/exclude")
                {
                    return Some(Err(format!(
                        "VBIC control promotion '{record}' does not retain the directory's upstream exclude provenance"
                    )));
                }
            }
        }
        if promoted_controls == 0 {
            return None;
        }
        if promoted_controls != 28 || multiplicity_promotions != 14 || polarity_promotions != 14 {
            return Some(Err(format!(
                "VBIC DC promotion provenance requires fourteen multiplicity and fourteen polarity controls, found {multiplicity_promotions}/{polarity_promotions}"
            )));
        }
        let stem = file_name.strip_suffix(".cir")?;
        let folded_stem = stem.to_ascii_lowercase();
        let candidate_owner_stem = folded_stem
            .strip_suffix("_noflip_p")
            .or_else(|| folded_stem.strip_suffix("_m"))
            .unwrap_or(&folded_stem);
        let owner_record = self.upstream_wrapper_decks.iter().find(|record| {
            record.rsplit_once('/').is_some_and(|(parent, name)| {
                parent == relative_parent
                    && name
                        .strip_suffix(".cir")
                        .is_some_and(|owner| owner.eq_ignore_ascii_case(candidate_owner_stem))
            })
        });
        let owner_record = match owner_record {
            Some(record) => record,
            None if exclusions.get(&relative).is_some_and(|exclusion| {
                matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                        expected_contract
                    } if expected_contract
                        == XyceVbicDcWrapperFamilyRole::MultiplicityControl.contract()
                        || expected_contract
                            == XyceVbicDcWrapperFamilyRole::PolarityControl.contract()
                )
            }) =>
            {
                return Some(Err(format!(
                    "promoted VBIC control '{relative}' has no matching wrapper-manifest owner"
                )));
            }
            None => return None,
        };
        let owner_name = owner_record.rsplit_once('/')?.1;
        let parent = deck.path.parent()?;
        let owner_path = Self::resolve_corpus_file(parent, owner_name);
        let owner_source = match fs::read_to_string(&owner_path) {
            Ok(source) => source,
            Err(err) => {
                return Some(Err(format!(
                    "failed to read recognized VBIC DC wrapper owner '{}': {err}",
                    owner_path.display()
                )));
            }
        };
        if !Self::source_has_analysis(&owner_source, "DC") {
            return None;
        }

        Some((|| {
            let mut groups = Vec::new();
            for record in self.upstream_wrapper_decks.iter().filter(|record| {
                record
                    .rsplit_once('/')
                    .is_some_and(|(record_parent, _)| record_parent == relative_parent)
            }) {
                let owner_name = record
                    .rsplit_once('/')
                    .map(|(_, name)| name)
                    .ok_or_else(|| "VBIC wrapper manifest record has no filename".to_string())?;
                let owner_path = Self::resolve_corpus_file(parent, owner_name);
                let owner_source = fs::read_to_string(&owner_path).map_err(|err| {
                    format!(
                        "failed to read VBIC wrapper owner '{}': {err}",
                        owner_path.display()
                    )
                })?;
                if !Self::source_has_analysis(&owner_source, "DC") {
                    continue;
                }
                let owner_stem = owner_path
                    .file_stem()
                    .and_then(|value| value.to_str())
                    .ok_or_else(|| "VBIC wrapper owner has a non-UTF-8 stem".to_string())?;
                let multiplicity_path =
                    Self::resolve_corpus_file(parent, &format!("{owner_stem}_m.cir"));
                let polarity_path =
                    Self::resolve_corpus_file(parent, &format!("{owner_stem}_noFlip_P.cir"));
                groups.push((owner_path, multiplicity_path, polarity_path));
            }
            groups.sort_by_key(|(owner, _, _)| {
                owner
                    .file_name()
                    .map(|name| name.to_string_lossy().to_ascii_lowercase())
            });
            if groups.len() != 14 {
                return Err(format!(
                    "VBIC DC wrapper provenance requires fourteen manifest-owned families, found {}",
                    groups.len()
                ));
            }

            let mut claimed = BTreeSet::new();
            let mut selected = None;
            for (owner_path, multiplicity_path, polarity_path) in groups {
                for path in [&owner_path, &multiplicity_path, &polarity_path] {
                    if !path.is_file() {
                        return Err(format!(
                            "VBIC DC wrapper family member is missing: {}",
                            path.display()
                        ));
                    }
                    let canonical = path.canonicalize().map_err(|err| {
                        format!("failed to canonicalize VBIC family member: {err}")
                    })?;
                    if !claimed.insert(canonical) {
                        return Err(
                            "a VBIC DC circuit is claimed by more than one family".to_string()
                        );
                    }
                }
                if !self.requires_upstream_wrapper(&self.relative_key(&owner_path))
                    || self.requires_upstream_wrapper(&self.relative_key(&multiplicity_path))
                    || self.requires_upstream_wrapper(&self.relative_key(&polarity_path))
                {
                    return Err(
                        "VBIC DC provenance requires one manifest owner and two non-wrapper controls"
                            .to_string(),
                    );
                }
                let exclusions = self
                    .upstream_exclusions
                    .as_ref()
                    .map_err(|err| format!("VBIC exclusion provenance is unavailable: {err}"))?;
                if exclusions.contains_key(&Self::normalize_manifest_key(
                    &self.relative_key(&owner_path),
                )) {
                    return Err(
                        "VBIC DC wrapper owners must remain manifest-owned rather than exclusion-manifest records"
                            .to_string(),
                    );
                }
                for (control, expected_contract) in [
                    (
                        &multiplicity_path,
                        XyceVbicDcWrapperFamilyRole::MultiplicityControl.contract(),
                    ),
                    (
                        &polarity_path,
                        XyceVbicDcWrapperFamilyRole::PolarityControl.contract(),
                    ),
                ] {
                    let record = self.relative_key(control);
                    let record_key = Self::normalize_manifest_key(&record);
                    let exclusion = exclusions.get(&record_key).ok_or_else(|| {
                        format!(
                            "VBIC control has no retained upstream exclusion provenance: {record}"
                        )
                    })?;
                    if Self::normalize_manifest_key(&exclusion.source)
                        != format!("{relative_parent}/exclude")
                        || !matches!(
                            &exclusion.disposition,
                            XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                                expected_contract: actual
                            } if actual == expected_contract
                        )
                    {
                        return Err(format!(
                            "VBIC control '{record}' must retain its upstream exclude source and independently qualified contract '{expected_contract}'"
                        ));
                    }
                }

                let owner_source = fs::read_to_string(&owner_path)
                    .map_err(|err| format!("failed to read VBIC owner: {err}"))?;
                let multiplicity_source = fs::read_to_string(&multiplicity_path)
                    .map_err(|err| format!("failed to read VBIC multiplicity control: {err}"))?;
                let polarity_source = fs::read_to_string(&polarity_path)
                    .map_err(|err| format!("failed to read VBIC polarity control: {err}"))?;
                let expected_multiplicity =
                    Self::vbic_dc_multiplicity_control_source(&owner_source)?;
                if Self::normalize_vbic_dc_source(&multiplicity_source) != expected_multiplicity {
                    return Err(format!(
                        "VBIC multiplicity control '{}' is not the canonical M=100/current-scale transform of its owner",
                        multiplicity_path.display()
                    ));
                }
                let expected_polarity = Self::vbic_dc_polarity_control_source(&owner_source)?;
                if Self::normalize_vbic_dc_source(&polarity_source) != expected_polarity {
                    return Err(format!(
                        "VBIC polarity control '{}' is not the canonical PNP/sign transform of its owner",
                        polarity_path.display()
                    ));
                }

                let owner_plan =
                    self.static_dc_plan_for_path(&owner_path, ExpressionDialect::Xyce)?;
                let multiplicity_plan =
                    self.static_dc_plan_for_path(&multiplicity_path, ExpressionDialect::Xyce)?;
                let polarity_plan =
                    self.static_dc_plan_for_path(&polarity_path, ExpressionDialect::Xyce)?;
                let owner_snapshot = Self::vbic_dc_family_plan_snapshot(&owner_plan)?;
                if Self::vbic_dc_family_plan_snapshot(&multiplicity_plan)? != owner_snapshot
                    || Self::vbic_dc_family_plan_snapshot(&polarity_plan)? != owner_snapshot
                {
                    return Err(format!(
                        "VBIC DC controls for '{}' do not preserve the owner's DC/STEP/PRINT semantics",
                        owner_path.display()
                    ));
                }

                let reference_path = self
                    .static_output_reference_path(&owner_path, "prn")
                    .ok_or_else(|| "cannot map VBIC owner to OutputData".to_string())?;
                if !reference_path.is_file() {
                    return Err(format!(
                        "VBIC DC wrapper owner has no checked-in gold PRN: {}",
                        reference_path.display()
                    ));
                }
                for control in [&multiplicity_path, &polarity_path] {
                    if self
                        .static_output_reference_path(control, "prn")
                        .is_some_and(|path| path.exists())
                    {
                        return Err(format!(
                            "VBIC relational control unexpectedly owns a direct gold PRN: {}",
                            control.display()
                        ));
                    }
                }
                let step_res = owner_path.with_extension("cir.res.gs");
                if owner_snapshot.steps.is_empty() == step_res.is_file() {
                    return Err(format!(
                        "VBIC owner '{}' must own a .res.gs artifact exactly when it has a .STEP sweep",
                        owner_path.display()
                    ));
                }

                let role = if Self::same_path(&deck.path, &owner_path) {
                    Some(XyceVbicDcWrapperFamilyRole::Owner)
                } else if Self::same_path(&deck.path, &multiplicity_path) {
                    Some(XyceVbicDcWrapperFamilyRole::MultiplicityControl)
                } else if Self::same_path(&deck.path, &polarity_path) {
                    Some(XyceVbicDcWrapperFamilyRole::PolarityControl)
                } else {
                    None
                };
                if let Some(role) = role {
                    selected = Some(XyceVbicDcWrapperFamilyContract {
                        family: owner_path
                            .file_stem()
                            .unwrap()
                            .to_string_lossy()
                            .to_string(),
                        owner_path,
                        multiplicity_path,
                        polarity_path,
                        reference_path,
                        role,
                    });
                }
            }
            if claimed.len() != 42 {
                return Err(format!(
                    "VBIC DC wrapper provenance must claim 42 distinct circuits, found {}",
                    claimed.len()
                ));
            }
            selected.ok_or_else(|| "requested deck is not a VBIC DC family member".to_string())
        })())
    }

    pub(super) fn normalize_vbic_dc_source(source: &str) -> String {
        let normalized = source.replace("\r\n", "\n").replace('\r', "\n");
        normalized.trim_end_matches('\n').to_string() + "\n"
    }

    fn replace_vbic_terminal_token(
        line: &str,
        expected: &str,
        replacement: &str,
    ) -> Option<String> {
        let trimmed = line.trim_end();
        let prefix = trimmed.strip_suffix(expected)?;
        prefix
            .chars()
            .next_back()
            .is_some_and(char::is_whitespace)
            .then(|| format!("{prefix}{replacement}"))
    }

    pub(super) fn vbic_dc_multiplicity_control_source(
        owner_source: &str,
    ) -> Result<String, String> {
        let owner = Self::normalize_vbic_dc_source(owner_source);
        let mut output = Vec::new();
        let mut cccs_count = 0usize;
        let mut multiplicity_count = 0usize;
        for line in owner.lines() {
            let trimmed = line.trim_start();
            if trimmed
                .as_bytes()
                .first()
                .is_some_and(|byte| byte.eq_ignore_ascii_case(&b'f'))
                && let Some(replaced) = Self::replace_vbic_terminal_token(line, "-1", "-0.01")
            {
                output.push(replaced);
                cccs_count += 1;
                continue;
            }
            if trimmed.to_ascii_lowercase().starts_with(".model ") {
                output.push("+ m=100".to_string());
                multiplicity_count += 1;
            }
            output.push(line.to_string());
        }
        if !(3..=4).contains(&cccs_count) || multiplicity_count != 1 {
            return Err(format!(
                "VBIC multiplicity transform requires three or four CCCS gains and one model boundary, found {cccs_count}/{multiplicity_count}"
            ));
        }
        Ok(output.join("\n") + "\n")
    }

    pub(super) fn vbic_dc_polarity_control_source(owner_source: &str) -> Result<String, String> {
        let owner = Self::normalize_vbic_dc_source(owner_source);
        let mut output = Vec::new();
        let mut inside_subcircuit = false;
        let mut vcvs_count = 0usize;
        let mut cccs_count = 0usize;
        let mut model_count = 0usize;
        for line in owner.lines() {
            let trimmed = line.trim_start();
            let folded = trimmed.to_ascii_lowercase();
            if folded.starts_with(".subckt ") {
                inside_subcircuit = true;
            } else if folded == ".ends" || folded.starts_with(".ends ") {
                inside_subcircuit = false;
            }
            if inside_subcircuit
                && trimmed
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| byte.eq_ignore_ascii_case(&b'e'))
                && let Some(replaced) = Self::replace_vbic_terminal_token(line, "1", "-1")
            {
                output.push(replaced);
                vcvs_count += 1;
                continue;
            }
            if inside_subcircuit
                && trimmed
                    .as_bytes()
                    .first()
                    .is_some_and(|byte| byte.eq_ignore_ascii_case(&b'f'))
                && let Some(replaced) = Self::replace_vbic_terminal_token(line, "-1", "1")
            {
                output.push(replaced);
                cccs_count += 1;
                continue;
            }
            if folded.starts_with(".model ") {
                let Some(index) = folded.find(" npn ") else {
                    return Err("VBIC polarity transform requires an NPN model card".to_string());
                };
                let mut replaced = line.to_string();
                replaced.replace_range(index + 1..index + 4, "pnp");
                output.push(replaced);
                model_count += 1;
                continue;
            }
            output.push(line.to_string());
        }
        if !(3..=4).contains(&vcvs_count) || vcvs_count != cccs_count || model_count != 1 {
            return Err(format!(
                "VBIC polarity transform requires matched three/four VCVS/CCCS signs and one NPN model, found {vcvs_count}/{cccs_count}/{model_count}"
            ));
        }
        Ok(output.join("\n") + "\n")
    }
}
