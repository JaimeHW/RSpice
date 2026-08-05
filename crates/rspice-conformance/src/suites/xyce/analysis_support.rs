//! Analysis-directive and source helpers used across contracts.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn source_has_ac_frequency_dependent_parameter(source: &str) -> bool {
        Self::logical_netlist_lines(source).into_iter().any(|line| {
            let trimmed = Self::strip_netlist_comment(&line).trim().to_string();
            let command = trimmed.split_whitespace().next().unwrap_or_default();
            (command.eq_ignore_ascii_case(".PARAM")
                || command.eq_ignore_ascii_case(".CSPARAM")
                || command.eq_ignore_ascii_case(".GLOBAL_PARAM"))
                && (Self::text_contains_ascii_identifier_reference(&trimmed, "FREQ")
                    || Self::text_contains_ascii_identifier_reference(&trimmed, "HERTZ"))
        })
    }

    pub(super) fn source_with_ac_frequency_bindings(source: &str, frequency: Value) -> String {
        let mut lines = source.lines();
        let title = lines.next().unwrap_or("Untitled");
        let mut rebound = String::new();
        rebound.push_str(title);
        rebound.push('\n');
        rebound.push_str(&format!(
            ".PARAM FREQ={frequency:.17e} HERTZ={frequency:.17e}\n"
        ));
        for line in lines {
            rebound.push_str(line);
            rebound.push('\n');
        }
        rebound
    }

    pub(super) fn source_with_wrapper_paramfile_bindings(
        source: &str,
        deck_path: &Path,
        requires_wrapper: bool,
    ) -> Result<String, String> {
        if !requires_wrapper {
            return Ok(source.to_string());
        }
        let Some(parent) = deck_path.parent() else {
            return Ok(source.to_string());
        };
        let paramfile_path = parent.join("paramfile.txt");
        if !paramfile_path.is_file() {
            return Ok(source.to_string());
        }
        let content = fs::read_to_string(&paramfile_path).map_err(|err| {
            format!(
                "failed to read Xyce wrapper parameter file {}: {err}",
                paramfile_path.display()
            )
        })?;
        let bindings = Self::parse_xyce_paramfile_variables(&content, &paramfile_path)?;
        if bindings.is_empty() {
            return Ok(source.to_string());
        }
        Ok(Self::source_with_param_bindings(source, &bindings))
    }

    pub(super) fn source_with_static_dc_wrapper_bindings(
        source: &str,
        deck_path: &Path,
        requires_wrapper: bool,
    ) -> Result<String, String> {
        if !requires_wrapper {
            return Ok(source.to_string());
        }
        Self::source_with_absolute_inc_lib_wrapper_bindings(source, deck_path)
    }

    pub(super) fn source_with_absolute_inc_lib_wrapper_bindings(
        source: &str,
        deck_path: &Path,
    ) -> Result<String, String> {
        let blank_include_count = source
            .lines()
            .filter(|line| Self::is_blank_include_wrapper_directive(line))
            .count();
        let blank_lib_count = source
            .lines()
            .filter(|line| Self::is_blank_lib_wrapper_directive(line))
            .count();
        if blank_include_count == 0 && blank_lib_count == 0 {
            return Ok(source.to_string());
        }
        if blank_include_count != 1 || blank_lib_count != 1 {
            return Err(format!(
                "wrapper-origin absolute include/library contract requires exactly one blank .INC and one blank .LIB placeholder, found {blank_include_count}/{blank_lib_count}"
            ));
        }

        let parent = deck_path
            .parent()
            .ok_or_else(|| "wrapper deck has no parent directory".to_string())?;
        let include_path = parent.join("sub1").join("sub2").join("include2_abs_path");
        let library_path = parent.join("sub1").join("sub2").join("lib2_abs_path");
        if !include_path.is_file() {
            return Err(format!(
                "wrapper-origin absolute include placeholder target is missing: {}",
                include_path.display()
            ));
        }
        if !library_path.is_file() {
            return Err(format!(
                "wrapper-origin absolute library placeholder target is missing: {}",
                library_path.display()
            ));
        }

        let include_arg = Self::quote_spice_path(&include_path)?;
        let library_arg = Self::quote_spice_path(&library_path)?;
        let mut rebound = String::new();
        for line in source.lines() {
            if Self::is_blank_include_wrapper_directive(line) {
                rebound.push_str(".INC ");
                rebound.push_str(&include_arg);
                rebound.push('\n');
            } else if Self::is_blank_lib_wrapper_directive(line) {
                rebound.push_str(".LIB ");
                rebound.push_str(&library_arg);
                rebound.push_str(" LIB_ABS\n");
            } else {
                rebound.push_str(line);
                rebound.push('\n');
            }
        }
        Ok(rebound)
    }

    pub(super) fn source_with_param_bindings(source: &str, bindings: &[(String, Value)]) -> String {
        let mut lines = source.lines();
        let title = lines.next().unwrap_or("Untitled");
        let mut rebound = String::new();
        rebound.push_str(title);
        rebound.push('\n');
        for (name, value) in bindings {
            rebound.push_str(&format!(".PARAM {name}={value:.17e}\n"));
        }
        for line in lines {
            rebound.push_str(line);
            rebound.push('\n');
        }
        rebound
    }

    pub(super) fn evaluate_measure_cont_step_tran_run(
        &self,
        netlist: Netlist,
        step_values: Vec<Value>,
        tran: &XyceTranAnalysis,
        start: Instant,
    ) -> Result<XyceStepTranEvaluation, String> {
        let max_step = Self::transient_family_max_step(&netlist, tran)?;
        let engine =
            self.create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(tran));
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let transient = engine
            .run_tran_with_abort(&netlist, tran.stop, max_step, &abort)
            .map_err(|error| format!("MEASURE_CONT STEP transient execution failed: {error}"))?;
        Self::validate_transient_result_time_grid(&transient)?;
        let scalar = rspice_core::analysis::evaluate_tran_measurements(&netlist, &transient);
        let continuous =
            rspice_core::analysis::evaluate_tran_continuous_measurements(&netlist, &transient);
        for result in &continuous {
            result.validate_invariants().map_err(|error| {
                format!(
                    "MEASURE_CONT STEP continuous result '{}' is invalid: {error}",
                    result.name
                )
            })?;
        }
        Ok(XyceStepTranEvaluation {
            step_values,
            netlist,
            transient,
            scalar,
            continuous,
        })
    }

    pub(super) fn reject_startup_source_sidecars(deck_path: &Path) -> Result<(), String> {
        let family_dir = deck_path
            .parent()
            .ok_or_else(|| "startup-diagnostic source has no family directory".to_string())?;
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "startup-diagnostic source filename is not UTF-8".to_string())?;
        let prefix = format!("{deck_name}.").to_ascii_lowercase();
        let mut sidecars = Vec::new();
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to inspect source sidecars: {error}"))?
        {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to read source-sidecar entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.to_ascii_lowercase().starts_with(&prefix))
            {
                sidecars.push(entry.path());
            }
        }
        sidecars.sort();
        if !sidecars.is_empty() {
            return Err(format!(
                "startup-diagnostic source must not own source-directory sidecars: {sidecars:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn bug702_effective_canonical_source(
        kind: XyceBug702PositiveKind,
        source: &str,
    ) -> Result<String, String> {
        let source = source.to_string();
        match kind {
            XyceBug702PositiveKind::InlinedMultiple => Self::replace_bug702_block_once(
                "BUG702 inlined-multiple",
                source,
                "* Using multiple inlined initial conditions\r\n\
.INITCOND C1 IC=400 XNLR1:CABS IC=0\r\n\
C1         1 0  400uF  ",
                "C1         1 0  400uF IC=400V",
            ),
            XyceBug702PositiveKind::InlinedSingle => {
                let source = Self::replace_bug702_block_once(
                    "BUG702 inlined-single device",
                    source,
                    "* MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u ic=2,0 \r\n\
MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u \r\n",
                    "MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u ic=2,0 \r\n",
                )?;
                Self::replace_bug702_block_once(
                    "BUG702 inlined-single directive",
                    source,
                    "\r\n* NOTE the IC values are inlined here\r\n\
.initcond XiNv1:mn1 IC=2,0\r\n",
                    "",
                )
            }
            XyceBug702PositiveKind::External => {
                let source = Self::replace_bug702_block_once(
                    "BUG702 external representation",
                    source,
                    "* NOTE the IC values are retrieved from an external file\r\n\
.initCOND File \"initcond.dat\"",
                    "* NOTE the IC values are inlined here\r\n\
.initcond XiNv1:mn1 IC=2,0",
                )?;
                Self::bug702_effective_canonical_source(
                    XyceBug702PositiveKind::InlinedSingle,
                    &source,
                )
            }
            XyceBug702PositiveKind::Precedence => {
                let source = Self::replace_bug702_block_once(
                    "BUG702 precedence device",
                    source,
                    "*MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u ic=2000,1000 \r\n\
*NOTE the MN1 line has nonsensical IC= values (no dcop)\r\n\
MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u ic=20000,10000 \r\n",
                    "MN1 OUT IN GND GND GND CMOSN w=4u  l=0.15u  AS=6p AD=6p PS=7u PD=7u ic=2,0 \r\n",
                )?;
                Self::replace_bug702_block_once(
                    "BUG702 precedence directive",
                    source,
                    "\r\n* NOTE this initline overwrites the IC value above\r\n\
.INITCOND XINV1:MN1 IC=2,0 \r\n",
                    "",
                )
            }
        }
    }

    pub(super) fn source_has_comp_directive(source: &str) -> bool {
        !Self::logical_comp_directives(source).is_empty()
    }

    pub(super) fn static_ac_sensitivity_plan_for_source(
        &self,
        source: &str,
        deck_path: &Path,
        output_override: bool,
    ) -> Result<Option<XyceStaticAcSensitivityPlan>, String> {
        let sensitivity_lines = Self::logical_netlist_lines(source)
            .into_iter()
            .filter(|line| {
                Self::strip_netlist_comment(line)
                    .split_whitespace()
                    .next()
                    .is_some_and(|command| command.eq_ignore_ascii_case(".SENS"))
            })
            .collect::<Vec<_>>();
        let print_requests = Self::aggregate_print_output_requests(
            Self::print_output_requests(source, "SENS")?,
            "SENS",
        )?;
        if sensitivity_lines.is_empty() {
            if print_requests.is_empty() {
                return Ok(None);
            }
            return Err(
                "wrapper-origin .PRINT SENS output requires one Xyce .SENS OBJVARS/PARAM directive"
                    .to_string(),
            );
        }
        if sensitivity_lines.len() != 1 {
            return Err(
                "native Xyce AC sensitivity contract requires exactly one .SENS directive"
                    .to_string(),
            );
        }
        let primary_requests = print_requests
            .iter()
            .filter(|request| request.file.is_none())
            .cloned()
            .collect::<Vec<_>>();
        if primary_requests.len() > 1 {
            return Err(
                "native Xyce AC sensitivity contract requires at most one primary .PRINT SENS statement"
                    .to_string(),
            );
        }
        let mut side_requests = print_requests
            .iter()
            .filter(|request| request.file.is_some())
            .cloned()
            .collect::<Vec<_>>();
        if !output_override && primary_requests.is_empty() && side_requests.is_empty() {
            return Err(
                "native Xyce AC sensitivity contract requires one .PRINT SENS output destination"
                    .to_string(),
            );
        }
        // Xyce permits FILE= sensitivity outputs without a primary destination.
        // In that form, the first side table supplies the canonical schema and
        // every additional destination must be comparable to that same solve.
        let canonical_request = if output_override {
            // Xyce's `-o` command-line output overrides every FILE= destination
            // and emits the canonical STD sensitivity table alongside the
            // overridden AC table.  Keep the probes from all SENS requests,
            // but deliberately suppress side-output comparisons: those files
            // are not part of the command-line output contract.
            side_requests.clear();
            Self::output_override_print_output_request(source, "SENS")?.ok_or_else(|| {
                "frequency-domain output override requires one .PRINT SENS request with probes"
                    .to_string()
            })?
        } else {
            primary_requests
                .first()
                .cloned()
                .or_else(|| side_requests.first().cloned())
                .expect("one primary or side .PRINT SENS request")
        };
        let (reference_format, no_index) = if output_override {
            (XyceAcSensitivityReferenceFormat::Prn, false)
        } else {
            Self::ac_sensitivity_output_schema(canonical_request.format.as_deref())?
        };

        let tokens = Self::split_print_fields(&sensitivity_lines[0])?;
        let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
        let mut objvars = None;
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
                "objvars" => &mut objvars,
                "param" | "params" => &mut parameters,
                _ => {
                    return Err(format!(
                        "native Xyce AC sensitivity contract does not cover .SENS field '{raw_key}'"
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
            Self::parse_xyce_sensitivity_objectives(objvars.as_deref().ok_or_else(|| {
                "Xyce .SENS directive must provide OBJVARS=<output>[,<output>...]".to_string()
            })?)?;
        let parameters =
            Self::parse_xyce_sensitivity_parameters(parameters.as_deref().ok_or_else(|| {
                "Xyce .SENS directive must provide PARAM=<device:param>[,<device:param>...]"
                    .to_string()
            })?)?;
        let (direct, adjoint) = Self::parse_xyce_sensitivity_flags(source)?;
        let reference_extension = match reference_format {
            XyceAcSensitivityReferenceFormat::Prn => "FD.SENS.prn",
            XyceAcSensitivityReferenceFormat::Csv => "FD.SENS.csv",
        };
        let reference_path = self
            .static_output_reference_path(deck_path, reference_extension)
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !reference_path.is_file() {
            return Err(format!(
                "no checked-in static AC sensitivity {} oracle at {}",
                match reference_format {
                    XyceAcSensitivityReferenceFormat::Prn => "PRN",
                    XyceAcSensitivityReferenceFormat::Csv => "CSV",
                },
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
                Self::ac_sensitivity_output_schema(request.format.as_deref())?;
            let candidate = Self::side_output_reference_candidate(&reference_path, &file)?;
            let side_reference_path = if candidate.is_file() {
                candidate
            } else if side_reference_format == XyceAcSensitivityReferenceFormat::Prn
                && !side_no_index
            {
                // Xyce's AC sensitivity outputter falls back to standard PRN
                // for RAW/PROBE/Touchstone/GNUPLOT/SPLOT and unknown formats.
                // Those destinations therefore share the canonical FD.SENS.prn
                // table when no separately checked-in artifact exists.
                reference_path.clone()
            } else {
                return Err(format!(
                    "missing checked-in static AC sensitivity side-output oracle {}",
                    self.display_path(&candidate)
                ));
            };
            if side_reference_path == reference_path
                && (side_reference_format != reference_format || side_no_index != no_index)
            {
                return Err(format!(
                    "AC sensitivity side output '{}' falls back to a schema incompatible with the canonical FD.SENS oracle",
                    file
                ));
            }
            side_outputs.push(XyceStaticAcSensitivitySideOutput {
                file,
                reference_path: side_reference_path,
                reference_format: side_reference_format,
                print: XycePrintRequest {
                    probes: request.probes,
                },
                no_index: side_no_index,
            });
        }

        Ok(Some(XyceStaticAcSensitivityPlan {
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
            side_outputs,
        }))
    }

    pub(super) fn source_without_xyce_sensitivity_directives(source: &str) -> String {
        let mut output = String::new();
        let mut skipping = false;
        for line in source.lines() {
            let trimmed = Self::strip_netlist_comment(line).trim_start();
            if trimmed
                .split_whitespace()
                .next()
                .is_some_and(|command| command.eq_ignore_ascii_case(".sens"))
            {
                skipping = true;
                continue;
            }
            if skipping && trimmed.starts_with('+') {
                continue;
            }
            skipping = false;
            output.push_str(line);
            output.push('\n');
        }
        output
    }

    pub(super) fn source_requests_sensitivity_stepnum_column(source: &str) -> bool {
        Self::logical_netlist_lines(source).into_iter().any(|line| {
            let normalized = Self::strip_netlist_comment(&line)
                .chars()
                .filter(|character| !character.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            normalized.contains(".optionsoutputadd_stepnum_col=true")
                || normalized.contains(".optionsoutputadd_stepnum_col=1")
        })
    }

    pub(super) fn static_dc_plan_for_source_with_execution_dir(
        &self,
        deck_path: &Path,
        source: String,
        expression_dialect: ExpressionDialect,
        execution_dir: Option<&Path>,
    ) -> Result<XyceStaticDcPlan, String> {
        self.static_dc_plan_for_source_with_execution_dir_and_redefinition_policy(
            deck_path,
            source,
            expression_dialect,
            ParameterRedefinitionPolicy::UseLast,
            execution_dir,
        )
    }

    pub(super) fn static_dc_plan_for_source_with_execution_dir_and_redefinition_policy(
        &self,
        deck_path: &Path,
        source: String,
        expression_dialect: ExpressionDialect,
        parameter_redefinition_policy: ParameterRedefinitionPolicy,
        execution_dir: Option<&Path>,
    ) -> Result<XyceStaticDcPlan, String> {
        if Self::contains_control_block(&source) {
            return Err(
                "deck uses a .control block; Xyce adapter does not interpret simulator scripting"
                    .to_string(),
            );
        }
        Self::reject_unsupported_source_directives(&source)?;

        let print_output = Self::single_dc_or_file_output_request(&source).or_else(|err| {
            if Self::validate_no_output_dc_wrapper_source(&source).is_ok() {
                Ok(XycePrintOutputRequest {
                    format: None,
                    file: None,
                    probes: Vec::new(),
                })
            } else {
                Err(err)
            }
        })?;
        let print = XycePrintRequest {
            probes: print_output.probes,
        };
        let netlist = Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
            &source,
            deck_path,
            expression_dialect,
            parameter_redefinition_policy,
            execution_dir,
        )
        .map_err(|err| format!("netlist parser does not yet accept this Xyce deck: {err}"))?;
        let diagnostics = netlist.diagnostics.clone();
        let dc_data = Self::dc_data_sweep_for_source(&source, &netlist)?;
        let dc = match &dc_data {
            Some(dc_data) => Self::synthetic_dc_data_sweep(dc_data)?,
            None => Self::single_dc_sweep(&netlist)?,
        };
        let steps = Self::step_commands(&netlist)?;
        if let Some(dc_data) = &dc_data {
            Self::validate_static_dc_data_contract(&netlist, dc_data, &print)?;
        } else {
            Self::validate_static_dc_contract(&netlist, &dc, &print)?;
        }

        Ok(XyceStaticDcPlan {
            deck_path: deck_path.to_path_buf(),
            execution_dir: execution_dir.map(Path::to_path_buf),
            source,
            expression_dialect,
            parameter_redefinition_policy,
            print,
            print_format: print_output.format,
            dc,
            dc_data,
            steps,
            diagnostics,
        })
    }

    pub(super) fn source_has_absolute_inc_lib_wrapper_bindings(source: &str) -> bool {
        let mut has_absolute_include = false;
        let mut has_absolute_library = false;
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line);
            let trimmed = stripped.trim();
            if let Some(filename) = rspice_core::netlist::parse_include_directive(trimmed)
                && Path::new(&filename).is_absolute()
            {
                has_absolute_include = true;
            }
            if let Some((filename, section)) = rspice_core::netlist::parse_lib_directive(trimmed)
                && Path::new(&filename).is_absolute()
                && section.is_some_and(|section| section.eq_ignore_ascii_case("LIB_ABS"))
            {
                has_absolute_library = true;
            }
        }
        has_absolute_include && has_absolute_library
    }

    pub(super) fn source_may_have_ac_data_analysis_command(source: &str) -> bool {
        source.lines().any(|raw_line| {
            let line = Self::strip_netlist_comment(raw_line).trim_start();
            let mut fields = line.split_whitespace();
            let Some(command) = fields.next() else {
                return false;
            };
            command.eq_ignore_ascii_case(".ac")
                && fields.any(|field| {
                    field
                        .split_once('=')
                        .map_or(field, |(name, _)| name)
                        .eq_ignore_ascii_case("data")
                })
        })
    }

    pub(super) fn source_may_have_pwl_repeat_option(source: &str) -> bool {
        // Avoid rebuilding logical continuation lines for the common case
        // where the source contains no PWL token at all.  This is especially
        // important for wrapper-only decks that carry very large option
        // cards: the preflight only needs to recognize a possible PWL repeat
        // contract before deciding whether parser expansion is warranted.
        if !source
            .as_bytes()
            .windows(3)
            .any(|window| window.eq_ignore_ascii_case(b"pwl"))
        {
            return false;
        }

        Self::logical_netlist_lines(source).iter().any(|line| {
            let compact = Self::strip_netlist_comment(line)
                .chars()
                .filter(|ch| !ch.is_whitespace())
                .collect::<String>()
                .to_ascii_lowercase();
            compact.contains("pwl") && (compact.contains("r=") || compact.contains("repeat="))
        })
    }

    pub(super) fn transient_family_plan_purpose_for_path(
        &self,
        kind: XyceBaselineFamilyKind,
        path: &Path,
    ) -> XyceStaticTranPlanPurpose {
        if kind == XyceBaselineFamilyKind::TransientAnalysisExpression
            && self.requires_upstream_wrapper(&self.relative_key(path))
        {
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
        } else {
            kind.transient_plan_purpose()
        }
    }

    pub(super) fn ac_analyses_match_exactly(baseline: &Netlist, target: &Netlist) -> bool {
        match (baseline.analyses.as_slice(), target.analyses.as_slice()) {
            (
                [
                    AnalysisCommand::Ac {
                        variation: baseline_variation,
                        points: baseline_points,
                        start_freq: baseline_start,
                        stop_freq: baseline_stop,
                    },
                ],
                [
                    AnalysisCommand::Ac {
                        variation: target_variation,
                        points: target_points,
                        start_freq: target_start,
                        stop_freq: target_stop,
                    },
                ],
            ) => {
                baseline_variation == target_variation
                    && baseline_points == target_points
                    && baseline_start.is_finite()
                    && baseline_stop.is_finite()
                    && target_start.is_finite()
                    && target_stop.is_finite()
                    && baseline_start.to_bits() == target_start.to_bits()
                    && baseline_stop.to_bits() == target_stop.to_bits()
            }
            _ => false,
        }
    }

    pub(super) fn stepped_ic_option_signature(source: &str) -> Result<Vec<String>, String> {
        let statements = Self::logical_netlist_lines(source)
            .into_iter()
            .map(|line| Self::strip_netlist_comment(&line).trim().to_string())
            .filter(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
            })
            .map(|line| {
                line.chars()
                    .filter(|ch| !ch.is_whitespace())
                    .collect::<String>()
                    .to_ascii_lowercase()
            })
            .collect::<Vec<_>>();
        if statements.len() != 1 {
            return Err(format!(
                "stepped-IC family requires exactly one .OPTIONS statement, found {}",
                statements.len()
            ));
        }
        Ok(statements)
    }

    pub(super) fn stepped_ic_snapshots_match(
        stepped: &XyceSteppedIcSnapshot,
        independent: &XyceSteppedIcSnapshot,
    ) -> bool {
        if !stepped
            .capacitor_name
            .eq_ignore_ascii_case(&independent.capacitor_name)
            || stepped.initial_conditions != independent.initial_conditions
        {
            return false;
        }
        let capacitor_key = stepped.capacitor_name.trim().to_ascii_lowercase();
        let mut stepped_elements = stepped.elements.clone();
        let mut independent_elements = independent.elements.clone();
        let Some(mut stepped_capacitor) = stepped_elements.remove(&capacitor_key) else {
            return false;
        };
        let Some(mut independent_capacitor) = independent_elements.remove(&capacitor_key) else {
            return false;
        };
        let Some(stepped_bits) = stepped_capacitor.numeric_bits.first_mut() else {
            return false;
        };
        let Some(independent_bits) = independent_capacitor.numeric_bits.first_mut() else {
            return false;
        };
        *stepped_bits = 0.0f64.to_bits();
        *independent_bits = 0.0f64.to_bits();
        let stepped_value = Value::from_bits(stepped.capacitor_value_bits);
        let independent_value = Value::from_bits(independent.capacitor_value_bits);
        stepped_elements == independent_elements
            && stepped_capacitor == independent_capacitor
            && stepped_value.is_finite()
            && independent_value.is_finite()
            && (stepped_value - independent_value).abs() <= 1.0e-12
    }

    pub(super) fn transient_family_max_step(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Result<Value, String> {
        let solver_max_step = Self::transient_oracle_solver_max_step_for_netlist(netlist, tran);
        // A lossless transmission line already contributes its propagation
        // delay as an accepted-time breakpoint.  Capping the global ceiling
        // to an arbitrary number of samples across a source edge (for
        // example, 200 samples across a 100-ps PULSE edge) turns a perfectly
        // ordinary delayed-wave simulation into hundreds of thousands of
        // Newton solves.  Let the line's adaptive companion and breakpoint
        // handling resolve that edge instead; ordinary source-only decks keep
        // the conservative source-resolution ceiling below.
        let has_transmission_line = netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::TransmissionLine { .. }));
        let source_step = (!has_transmission_line)
            .then(|| Self::source_transient_max_step(netlist, tran))
            .flatten()
            .and_then(|step| Self::feasible_oracle_limited_step(tran, step));
        let max_step = [Some(solver_max_step), source_step]
            .into_iter()
            .flatten()
            .filter(|value| value.is_finite() && *value > 0.0)
            .reduce(Value::min)
            .unwrap_or(solver_max_step);
        if !max_step.is_finite() || max_step <= 0.0 {
            return Err(format!(
                "resolved transient family maximum step must be finite and positive, got {max_step}"
            ));
        }
        let estimated_steps = (tran.stop / max_step).ceil();
        if estimated_steps > MAX_NATIVE_TRAN_ORACLE_STEPS {
            return Err(format!(
                "transient family execution envelope supports at most {:.0} native step(s), but this deck requires about {:.0}",
                MAX_NATIVE_TRAN_ORACLE_STEPS, estimated_steps
            ));
        }
        Self::validate_transient_execution_envelope(netlist, estimated_steps)?;
        Ok(max_step)
    }

    /// Preserve Xyce's native transient solver ceiling for oracle runs.
    ///
    /// The reference time grid is an accepted-step/output contract, so adding
    /// a harness-only sampling ceiling changes the history path that produces
    /// stateful quantities (including trapezoidal ringing and behavioral
    /// trajectories).  Xyce's authored `DELMAX`, or its 10%-of-window default,
    /// must remain the sole global ceiling; source breakpoints and the
    /// reference grid still provide the required local resolution.
    pub(super) fn transient_oracle_solver_max_step(tran: &XyceTranAnalysis) -> Value {
        Self::xyce_transient_solver_max_step(tran)
    }

    pub(super) fn ac_analysis_value_fields(statement: &str) -> Result<Vec<String>, String> {
        const LABEL: &str = "AC-analysis expression parity";
        let fields = Self::split_grouped_whitespace_fields(statement, ".AC statement")?;
        if fields.len() != 5
            || fields
                .first()
                .is_none_or(|field| !field.eq_ignore_ascii_case(".AC"))
            || fields.get(1).is_none_or(|field| {
                !matches!(field.to_ascii_uppercase().as_str(), "DEC" | "OCT" | "LIN")
            })
        {
            return Err(format!(
                "{LABEL} requires '.AC DEC|OCT|LIN points start stop'"
            ));
        }
        Ok(fields[2..].to_vec())
    }

    pub(super) fn dc_analysis_value_fields(statement: &str) -> Result<Vec<String>, String> {
        const LABEL: &str = "DC-analysis expression parity";
        let fields = Self::split_grouped_whitespace_fields(statement, ".DC statement")?;
        if fields
            .first()
            .is_none_or(|field| !field.eq_ignore_ascii_case(".DC"))
        {
            return Err(format!("{LABEL} requires a .DC statement"));
        }
        if fields.get(1).is_some_and(|field| {
            field.eq_ignore_ascii_case("DEC") || field.eq_ignore_ascii_case("OCT")
        }) {
            if fields.len() != 6 || !Self::is_single_spice_identifier(&fields[2]) {
                return Err(format!(
                    "{LABEL} DEC/OCT sweeps require '.DC mode source start stop points'"
                ));
            }
            return Ok(fields[3..].to_vec());
        }
        if fields
            .get(2)
            .is_some_and(|field| field.eq_ignore_ascii_case("LIST"))
        {
            if fields.len() < 4 || !Self::is_single_spice_identifier(&fields[1]) {
                return Err(format!(
                    "{LABEL} LIST sweeps require '.DC source LIST value ...'"
                ));
            }
            return Ok(fields[3..].to_vec());
        }
        if !matches!(fields.len(), 5 | 9)
            || !Self::is_single_spice_identifier(&fields[1])
            || (fields.len() == 9 && !Self::is_single_spice_identifier(&fields[5]))
        {
            return Err(format!(
                "{LABEL} linear sweeps require one or two 'source start stop step' dimensions"
            ));
        }
        let mut values = fields[2..5].to_vec();
        if fields.len() == 9 {
            values.extend_from_slice(&fields[6..9]);
        }
        Ok(values)
    }

    pub(super) fn dc_sweep_modes_match_exactly(
        baseline: &rspice_core::netlist::DcSweepMode,
        target: &rspice_core::netlist::DcSweepMode,
    ) -> bool {
        match (baseline, target) {
            (
                rspice_core::netlist::DcSweepMode::Linear,
                rspice_core::netlist::DcSweepMode::Linear,
            ) => true,
            (
                rspice_core::netlist::DcSweepMode::List(baseline),
                rspice_core::netlist::DcSweepMode::List(target),
            ) => {
                baseline.len() == target.len()
                    && baseline.iter().all(|value| value.is_finite())
                    && target.iter().all(|value| value.is_finite())
                    && baseline
                        .iter()
                        .zip(target)
                        .all(|(baseline, target)| baseline.to_bits() == target.to_bits())
            }
            (
                rspice_core::netlist::DcSweepMode::Decade {
                    points_per_decade: baseline,
                },
                rspice_core::netlist::DcSweepMode::Decade {
                    points_per_decade: target,
                },
            ) => baseline == target,
            (
                rspice_core::netlist::DcSweepMode::Octave {
                    points_per_octave: baseline,
                },
                rspice_core::netlist::DcSweepMode::Octave {
                    points_per_octave: target,
                },
            ) => baseline == target,
            _ => false,
        }
    }

    pub(super) fn dc_second_sweeps_match_exactly(
        baseline: Option<&DcSecondSweep>,
        target: Option<&DcSecondSweep>,
    ) -> bool {
        match (baseline, target) {
            (None, None) => true,
            (Some(baseline), Some(target)) => {
                baseline.start.is_finite()
                    && baseline.stop.is_finite()
                    && baseline.step.is_finite()
                    && target.start.is_finite()
                    && target.stop.is_finite()
                    && target.step.is_finite()
                    && baseline.source == target.source
                    && baseline.start.to_bits() == target.start.to_bits()
                    && baseline.stop.to_bits() == target.stop.to_bits()
                    && baseline.step.to_bits() == target.step.to_bits()
                    && Self::dc_sweep_modes_match_exactly(&baseline.mode, &target.mode)
            }
            _ => false,
        }
    }

    pub(super) fn dc_sweeps_match_exactly(baseline: &XyceDcSweep, target: &XyceDcSweep) -> bool {
        baseline.start.is_finite()
            && baseline.stop.is_finite()
            && baseline.step.is_finite()
            && target.start.is_finite()
            && target.stop.is_finite()
            && target.step.is_finite()
            && baseline.source == target.source
            && baseline.start.to_bits() == target.start.to_bits()
            && baseline.stop.to_bits() == target.stop.to_bits()
            && baseline.step.to_bits() == target.step.to_bits()
            && Self::dc_sweep_modes_match_exactly(&baseline.mode, &target.mode)
            && Self::dc_second_sweeps_match_exactly(
                baseline.sweep2.as_ref(),
                target.sweep2.as_ref(),
            )
    }

    pub(super) fn transient_probe_requests_linear_capacitor_branch_quantity(
        netlist: &Netlist,
        probe: &str,
    ) -> bool {
        let normalized = Self::normalize_probe(probe);
        if Self::normalized_probe_requests_linear_capacitor_branch_quantity(netlist, &normalized) {
            return true;
        }

        let Some(expression) = Self::print_expression_inner(probe) else {
            return false;
        };
        let normalized_expression = Self::normalize_probe(expression);
        if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
            return Self::normalized_probe_requests_linear_capacitor_branch_quantity(
                netlist,
                &normalized_expression,
            );
        }

        let mut found = false;
        let mut call_value = |call: &str| {
            if Self::normalized_probe_requests_linear_capacitor_branch_quantity(
                netlist,
                &Self::normalize_probe(call),
            ) {
                found = true;
            }
            Ok(1.0)
        };
        let _ = Self::evaluate_print_expression_with_probe_calls(
            expression,
            netlist.params.clone(),
            &mut call_value,
        );
        found
    }

    pub(super) fn evaluate_dc_sensitivity_batches(
        &self,
        plan: &XyceStaticDcSensitivityPlan,
        batches: &[XyceDcResultBatch],
        start: Instant,
    ) -> Result<Vec<XyceDcSensitivityEvaluation>, String> {
        let engine = self.create_xyce_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let mut evaluations = Vec::new();
        for (step_index, batch) in batches.iter().enumerate() {
            for (local_index, point) in batch.results.iter().enumerate() {
                if abort.is_aborted() {
                    return Err(format!(
                        "DC sensitivity evaluation exceeded timeout ({}ms)",
                        self.config.max_time_per_test_ms
                    ));
                }
                let sweep_point = XyceDcSweepPoint {
                    primary: point.sweep_value,
                    secondary: None,
                };
                let point_netlist =
                    Self::dc_sensitivity_point_netlist(&batch.netlist, &plan.dc.dc, sweep_point)?;
                let mut objectives = Vec::with_capacity(plan.objectives.len());
                for objective in &plan.objectives {
                    let output =
                        Self::xyce_dc_sensitivity_output_from_spec(&objective.spec, &point.result)?;
                    let sensitivity = engine
                        .run_sensitivity_dc_complete_with_abort(
                            &point_netlist,
                            output,
                            &plan.parameters,
                            &abort,
                        )
                        .map_err(|error| error.to_string())?;
                    objectives.push(sensitivity);
                }
                evaluations.push(XyceDcSensitivityEvaluation {
                    netlist: point_netlist,
                    point: point.clone(),
                    objectives,
                    step_index,
                    local_index,
                });
            }
        }
        if evaluations.is_empty() {
            return Err("DC sensitivity simulation produced no result points".to_string());
        }
        Ok(evaluations)
    }

    pub(super) fn dc_sensitivity_nominal_column_name(objective_probe: &str) -> String {
        Self::normalize_probe(&format!("{{{objective_probe}}}"))
    }

    pub(super) fn dc_sensitivity_derivative_column_name(
        objective_probe: &str,
        parameter: &str,
        mode: &str,
    ) -> String {
        Self::normalize_probe(&format!("d_{{{objective_probe}}}/d_{parameter}_{mode}"))
    }

    pub(super) fn transient_probe_matches_within_time_quantization(
        &self,
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        expected: Value,
        actual: Value,
        tolerance: XyceComparisonTolerance,
        time_tolerance: Value,
    ) -> Result<bool, String> {
        let Some((&first_time, &last_time)) = result.time.first().zip(result.time.last()) else {
            return Ok(false);
        };
        let mut min_actual = actual;
        let mut max_actual = actual;
        for candidate_time in [time - time_tolerance, time + time_tolerance] {
            if candidate_time < first_time || candidate_time > last_time {
                continue;
            }
            let candidate = Self::evaluate_tran_probe(probe, netlist, result, candidate_time)?;
            if self
                .value_mismatch(expected, candidate, tolerance)
                .is_none()
            {
                return Ok(true);
            }
            if candidate.is_finite() {
                min_actual = min_actual.min(candidate);
                max_actual = max_actual.max(candidate);
            }
        }

        // A Xyce PRN timestamp is already rounded text. Around very steep
        // transitions, the oracle value can belong to the same printed-time
        // neighborhood while the closest simulator samples sit on adjacent
        // printed ticks. Bound the comparison with those immediate local
        // samples without accepting coarse-grid timing drift.
        let sample_window = time_tolerance * PRN_TIME_NEIGHBOR_HALF_ULPS;
        let lower_time = time - sample_window;
        let upper_time = time + sample_window;
        let first_sample = result.time.partition_point(|sample| *sample < lower_time);
        for &sample_time in result.time.iter().skip(first_sample) {
            if sample_time > upper_time {
                break;
            }
            let candidate = Self::evaluate_tran_probe(probe, netlist, result, sample_time)?;
            if self
                .value_mismatch(expected, candidate, tolerance)
                .is_none()
            {
                return Ok(true);
            }
            if candidate.is_finite() {
                min_actual = min_actual.min(candidate);
                max_actual = max_actual.max(candidate);
            }
        }

        Ok(expected.is_finite()
            && min_actual.is_finite()
            && max_actual.is_finite()
            && expected >= min_actual.min(max_actual)
            && expected <= min_actual.max(max_actual))
    }

    pub(super) fn transient_max_step_for_static_plan(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        reference: &XycePrnTable,
    ) -> Result<Value, String> {
        match plan.comparison_mode {
            XyceStaticTranComparisonMode::Release710IntegratedRms { .. }
            | XyceStaticTranComparisonMode::Release710IntegratedRmsComp { .. } => {
                let solver_max_step = if Self::netlist_is_native_transient_ekv26_pair(netlist) {
                    // The native EKV26 evaluator is reference-qualified on a
                    // fine oracle step envelope.  Keep the integrated verifier
                    // independent of the candidate output row count while
                    // retaining enough accepted states for the device's
                    // steep complementary transition.
                    Self::transient_oracle_solver_max_step(tran)
                } else {
                    Self::xyce_transient_solver_max_step(tran)
                };
                Self::transient_max_step_with_solver_ceiling(
                    netlist,
                    tran,
                    None,
                    solver_max_step,
                    false,
                )
            }
            XyceStaticTranComparisonMode::Pointwise => {
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
                if !has_solution_dependent_capacitor
                    && Self::transient_print_requests_linear_capacitor_branch_quantity(
                        netlist,
                        &plan.print,
                    )
                {
                    // Linear-capacitor lead currents are accepted-history
                    // quantities. A harness sampling cap introduced solely
                    // because the reference contains a tiny startup gap
                    // creates extra accepted states and changes those
                    // currents. Preserve Xyce's adaptive DELMAX contract for
                    // these outputs and interpolate only at comparison time.
                    return Self::transient_max_step_with_solver_ceiling(
                        netlist,
                        tran,
                        None,
                        Self::transient_oracle_solver_max_step_for_netlist(netlist, tran),
                        false,
                    );
                }
                Self::transient_max_step_for_reference(netlist, tran, reference)
            }
        }
    }

    pub(super) fn transient_max_step_with_solver_ceiling(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        reference_step: Option<Value>,
        solver_max_step: Value,
        include_harness_source_resolution: bool,
    ) -> Result<Value, String> {
        let source_step = if include_harness_source_resolution {
            Self::source_transient_max_step(netlist, tran)
                .and_then(|step| Self::feasible_oracle_limited_step(tran, step))
        } else {
            None
        };
        let reference_limited_step = Self::feasible_reference_limited_step(tran, reference_step);
        // Xyce's solver ceiling remains authoritative, but a pointwise oracle
        // still needs a bounded native sampling envelope when its first
        // printed interval is too fine to replay directly.  This is a
        // harness execution bound, not a reinterpretation of `.TRAN TSTEP` as
        // DELMAX; an authored DTMAX and an affordable reference cadence both
        // remain authoritative.
        let fallback_limit = (include_harness_source_resolution
            && tran.max_step.is_none()
            && reference_limited_step.is_none())
        .then(|| Self::transient_sampling_fallback_step(netlist, tran))
        .flatten();
        let mut max_step = [
            Some(solver_max_step),
            reference_limited_step,
            source_step,
            fallback_limit,
        ]
        .into_iter()
        .flatten()
        .filter(|value| value.is_finite() && *value > 0.0)
        .reduce(Value::min)
        .unwrap_or(solver_max_step);
        let unconstrained_estimated_steps = (tran.stop / max_step).ceil();
        if unconstrained_estimated_steps > MAX_NATIVE_TRAN_ORACLE_STEPS {
            return Err(format!(
                "transient harness execution envelope supports at most {:.0} native step(s), but this deck requires about {:.0}",
                MAX_NATIVE_TRAN_ORACLE_STEPS, unconstrained_estimated_steps
            ));
        }
        Self::validate_transient_execution_envelope(netlist, unconstrained_estimated_steps)?;
        if let Some(work_limited_step) = Self::compact_device_work_limited_step(netlist, tran)? {
            max_step = max_step.max(work_limited_step);
        }
        if !max_step.is_finite() || max_step <= 0.0 {
            return Err(format!(
                "resolved transient maximum step must be finite and positive, got {max_step}"
            ));
        }
        Ok(max_step)
    }

    pub(super) fn transient_sampling_fallback_step(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Option<Value> {
        let linear_passive = netlist.subcircuits.is_empty()
            && netlist.elements.iter().all(|element| match &element.kind {
                ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
                | ElementKind::VoltageSource(_)
                | ElementKind::CurrentSource(_) => true,
                ElementKind::Coupling { model, .. } => model.is_none(),
                _ => false,
            });
        linear_passive
            .then(|| ((tran.stop - tran.start.unwrap_or(0.0)) / 1000.0).max(f64::MIN_POSITIVE))
    }

    pub(super) fn transient_flattened_problem_size(
        netlist: &Netlist,
    ) -> Result<XyceTransientProblemSize, String> {
        let elements = if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            rspice_core::netlist::flatten_netlist_with_models(netlist)
                .map(|flattened| flattened.elements)
                .map_err(|err| {
                format!("transient harness execution envelope could not flatten subcircuits for native problem-size estimation: {err}")
            })?
        } else {
            netlist.elements.clone()
        };

        let mut nodes = BTreeSet::new();
        for element in &elements {
            for node in &element.nodes {
                if !Self::node_name_is_ground(node) {
                    nodes.insert(node.to_ascii_lowercase());
                }
            }
        }

        Ok(XyceTransientProblemSize {
            element_count: elements.len(),
            compact_device_count: Self::transient_compact_device_count(&elements),
            node_count: nodes.len(),
        })
    }

    pub(super) fn transient_hierarchy_problem_size_estimate(
        netlist: &Netlist,
    ) -> Result<XyceTransientProblemSize, String> {
        let mut subcircuits = BTreeMap::new();
        for subcircuit in &netlist.subcircuits {
            Self::collect_subcircuit_defs(subcircuit, &mut subcircuits);
        }

        let mut top_nodes = BTreeSet::new();
        let mut element_count = 0usize;
        let mut compact_device_count = 0usize;
        let mut internal_node_count = 0usize;
        let mut stack = BTreeSet::new();

        for element in &netlist.elements {
            for node in &element.nodes {
                if !Self::node_name_is_ground(node) {
                    top_nodes.insert(node.to_ascii_lowercase());
                }
            }
            if let ElementKind::Subcircuit { subckt_name, .. } = &element.kind {
                let subcircuit =
                    subcircuits
                        .get(&subckt_name.to_ascii_lowercase())
                        .ok_or_else(|| {
                            format!(
                                "transient harness execution envelope cannot estimate unresolved subcircuit '{}'",
                                subckt_name
                            )
                        })?;
                let size =
                    Self::subcircuit_problem_size_estimate(subcircuit, &subcircuits, &mut stack)?;
                element_count += size.element_count;
                compact_device_count += size.compact_device_count;
                internal_node_count += size.node_count;
            } else {
                element_count += 1;
                compact_device_count += Self::transient_element_compact_device_count(element);
            }
        }

        Ok(XyceTransientProblemSize {
            element_count,
            compact_device_count,
            node_count: top_nodes.len() + internal_node_count,
        })
    }

    pub(super) fn transient_compact_device_count(
        elements: &[rspice_core::netlist::Element],
    ) -> usize {
        elements
            .iter()
            .map(Self::transient_element_compact_device_count)
            .sum()
    }

    pub(super) fn transient_element_compact_device_count(
        element: &rspice_core::netlist::Element,
    ) -> usize {
        matches!(
            element.kind,
            ElementKind::Diode { .. }
                | ElementKind::Bjt { .. }
                | ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. }
        ) as usize
    }

    pub(super) fn source_transient_max_step(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Option<Value> {
        let independent_source_step = netlist.elements.iter().filter_map(|element| match &element
            .kind
        {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                Self::source_spec_transient_max_step(spec, tran)
            }
            _ => None,
        });
        independent_source_step
            .chain(netlist.elements.iter().filter_map(|element| {
                let (ElementKind::BehavioralVoltage { expression, .. }
                | ElementKind::BehavioralCurrent { expression, .. }) = &element.kind
                else {
                    return None;
                };
                // A direct behavioral SPICE_SIN is the expression form of the
                // same periodic source that receives the ordinary SIN source
                // envelope above. Keep the family execution grid identical
                // while leaving arbitrary behavioral expressions uncapped.
                let prepared = prepare_behavioral_expression(expression, &netlist.params).ok()?;
                let ast = parse_expression_strict(&prepared).ok()?;
                let Expr::Function {
                    func: rspice_core::expr::Function::SpiceSin,
                    args,
                } = ast
                else {
                    return None;
                };
                let [
                    Expr::Const(_offset),
                    Expr::Const(_amplitude),
                    Expr::Const(frequency),
                ] = args.as_slice()
                else {
                    return None;
                };
                Self::positive_frequency_step(Self::resolved_sin_frequency(*frequency, tran.stop))
            }))
            .filter(|step| step.is_finite() && *step > 0.0)
            .reduce(Value::min)
    }

    pub(super) fn source_spec_transient_max_step(
        spec: &rspice_core::netlist::SourceSpec,
        tran: &XyceTranAnalysis,
    ) -> Option<Value> {
        match spec {
            rspice_core::netlist::SourceSpec::Distortion { inner, .. } => {
                Self::source_spec_transient_max_step(inner, tran)
            }
            rspice_core::netlist::SourceSpec::RfPort { inner, .. } => {
                Self::source_spec_transient_max_step(inner, tran)
            }
            rspice_core::netlist::SourceSpec::Dc(_)
            | rspice_core::netlist::SourceSpec::Ac { .. }
            | rspice_core::netlist::SourceSpec::DcAc { .. }
            | rspice_core::netlist::SourceSpec::Pwl { .. }
            | rspice_core::netlist::SourceSpec::PwlFile { .. }
            | rspice_core::netlist::SourceSpec::TrNoise { .. } => None,
            rspice_core::netlist::SourceSpec::DcTransient { transient, .. }
            | rspice_core::netlist::SourceSpec::DcAcTransient { transient, .. } => {
                Self::source_spec_transient_max_step(transient, tran)
            }
            rspice_core::netlist::SourceSpec::Pulse {
                rise,
                fall,
                width,
                period,
                width_defaults_to_zero,
                ..
            } => {
                let tstep_hint = if tran.step.is_finite() && tran.step > 0.0 {
                    tran.step
                } else {
                    (tran.stop / 1000.0).max(f64::MIN_POSITIVE)
                };
                let (_delay, resolved_rise, resolved_fall, resolved_width, resolved_period) =
                    rspice_core::circuit::VoltageSources::resolve_pulse_timing_with_defaults(
                        0.0,
                        *rise,
                        *fall,
                        *width,
                        *period,
                        *width_defaults_to_zero,
                        tstep_hint,
                        tran.stop.max(f64::MIN_POSITIVE),
                        SpiceDialect::Xyce,
                    );
                [
                    Self::positive_duration_step(
                        resolved_rise,
                        TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION,
                    ),
                    Self::positive_duration_step(
                        resolved_fall,
                        TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION,
                    ),
                    Self::positive_duration_step(
                        resolved_width,
                        TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD,
                    ),
                    Self::positive_duration_step(
                        resolved_period,
                        TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD,
                    ),
                ]
                .into_iter()
                .flatten()
                .reduce(Value::min)
            }
            rspice_core::netlist::SourceSpec::Pat {
                rise, fall, sample, ..
            } => [
                Self::positive_duration_step(*rise, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*fall, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*sample, TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD),
            ]
            .into_iter()
            .flatten()
            .reduce(Value::min),
            rspice_core::netlist::SourceSpec::Exp {
                tau1,
                tau2,
                td1,
                td2,
                ..
            } => [
                Self::positive_duration_step(*tau1, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*tau2, TRAN_ORACLE_STEPS_PER_SOURCE_TRANSITION),
                Self::positive_duration_step(*td2 - *td1, TRAN_ORACLE_STEPS_PER_SOURCE_PERIOD),
            ]
            .into_iter()
            .flatten()
            .reduce(Value::min),
            rspice_core::netlist::SourceSpec::Sin { frequency, .. } => {
                Self::positive_frequency_step(Self::resolved_sin_frequency(*frequency, tran.stop))
            }
            rspice_core::netlist::SourceSpec::Sffm {
                carrier_freq,
                signal_freq,
                ..
            } => Self::positive_frequency_step(
                Self::resolved_modulated_frequency(*carrier_freq, 5.0, tran.stop).max(
                    Self::resolved_modulated_frequency(*signal_freq, 500.0, tran.stop),
                ),
            ),
            rspice_core::netlist::SourceSpec::Am {
                modulating_freq,
                carrier_freq,
                ..
            } => Self::positive_frequency_step(
                Self::resolved_modulated_frequency(*carrier_freq, 500.0, tran.stop).max(
                    Self::resolved_modulated_frequency(*modulating_freq, 5.0, tran.stop),
                ),
            ),
        }
    }

    pub(super) fn step_res_expected_columns(
        netlist: &Netlist,
        steps: &[StepCommand],
        step_runs: &[XyceStepRun],
    ) -> Result<Vec<(String, Vec<Value>)>, String> {
        let mut columns = Vec::new();
        for (step_index, step) in steps.iter().enumerate() {
            if step_runs
                .iter()
                .any(|run| run.step_values.len() <= step_index)
            {
                return Err(format!(
                    ".STEP run metadata is missing value {} for {}",
                    step_index,
                    Self::step_res_variable_name(step)
                ));
            }

            if let StepSweep::Data { table_name } = &step.sweep {
                let table = netlist
                    .data_tables
                    .iter()
                    .find(|table| table.name.eq_ignore_ascii_case(table_name))
                    .ok_or_else(|| format!(".STEP DATA table '{table_name}' not found"))?;
                if table.params.is_empty() {
                    return Err(format!(".STEP DATA table '{}' has no columns", table.name));
                }
                if table.rows.is_empty() {
                    return Err(format!(".STEP DATA table '{}' has no rows", table.name));
                }
                let first_new_column = columns.len();
                columns.extend(
                    table
                        .params
                        .iter()
                        .map(|param| (param.clone(), Vec::with_capacity(step_runs.len()))),
                );
                for run in step_runs {
                    let row_index = run.step_values[step_index];
                    if row_index.fract() != 0.0 || row_index < 0.0 {
                        return Err(format!(
                            ".STEP DATA table '{}' row selector {} is not a non-negative integer",
                            table.name, row_index
                        ));
                    }
                    let row_index = row_index as usize;
                    let row = table.rows.get(row_index).ok_or_else(|| {
                        format!(
                            ".STEP DATA table '{}' row selector {} is outside {} row(s)",
                            table.name,
                            row_index,
                            table.rows.len()
                        )
                    })?;
                    if row.len() != table.params.len() {
                        return Err(format!(
                            ".STEP DATA table '{}' row {} has {} value(s), expected {}",
                            table.name,
                            row_index,
                            row.len(),
                            table.params.len()
                        ));
                    }
                    for (column_index, value) in row.iter().copied().enumerate() {
                        columns[first_new_column + column_index].1.push(value);
                    }
                }
                continue;
            }

            columns.push((
                Self::step_res_variable_name(step),
                step_runs
                    .iter()
                    .map(|run| run.step_values[step_index])
                    .collect(),
            ));
        }

        Ok(columns)
    }

    pub(super) fn step_res_variable_name(step: &StepCommand) -> String {
        match step.target {
            StepTarget::Temp => "TEMP".to_string(),
            StepTarget::Param => step.name.clone(),
            StepTarget::Device | StepTarget::Model => match &step.param_name {
                Some(param_name) => format!("{}:{param_name}", step.name),
                None => step.name.clone(),
            },
        }
    }

    pub(super) fn dc_data_sweep_for_source(
        source: &str,
        netlist: &Netlist,
    ) -> Result<Option<XyceDcDataSweep>, String> {
        let table_names = Self::dc_data_table_names(source)?;
        if table_names.is_empty() {
            return Ok(None);
        }

        let mut rows: Option<Vec<XyceDcDataRow>> = None;
        let mut seen_columns = BTreeSet::new();
        for table_name in table_names {
            let table = netlist
                .data_tables
                .iter()
                .find(|table| table.name.eq_ignore_ascii_case(&table_name))
                .ok_or_else(|| format!(".DC DATA references unknown .DATA table '{table_name}'"))?;
            if table.params.is_empty() {
                return Err(format!(
                    ".DC DATA table '{}' has no parameter columns",
                    table.name
                ));
            }
            if table.rows.is_empty() {
                return Err(format!(".DC DATA table '{}' has no rows", table.name));
            }
            for (row_index, row) in table.rows.iter().enumerate() {
                if row.len() != table.params.len() {
                    return Err(format!(
                        ".DC DATA table '{}' row {} has {} value(s), expected {}",
                        table.name,
                        row_index + 1,
                        row.len(),
                        table.params.len()
                    ));
                }
            }

            match rows.as_ref() {
                Some(existing) if existing.len() != table.rows.len() => {
                    return Err(format!(
                        ".DC DATA table '{}' has {} row(s), expected {} to match the other TABLE-style .DC DATA sweeps",
                        table.name,
                        table.rows.len(),
                        existing.len()
                    ));
                }
                None => {
                    rows = Some(
                        (0..table.rows.len())
                            .map(|_| XyceDcDataRow {
                                overrides: Vec::new(),
                            })
                            .collect(),
                    );
                }
                Some(_) => {}
            }

            for (column_index, column_name) in table.params.iter().enumerate() {
                let column_key = Self::normalize_probe(column_name);
                if !seen_columns.insert(column_key) {
                    return Err(format!(
                        ".DC DATA column '{}' is specified more than once across the active data tables",
                        column_name
                    ));
                }

                let rows = rows.as_mut().expect("rows initialized from table length");
                for (row_index, row) in table.rows.iter().enumerate() {
                    let value = row[column_index];
                    if !value.is_finite() {
                        return Err(format!(
                            ".DC DATA table '{}' row {} column '{}' contains non-finite value {}",
                            table.name,
                            row_index + 1,
                            column_name,
                            value
                        ));
                    }
                    rows[row_index]
                        .overrides
                        .push(Self::dc_data_override_for_column(
                            netlist,
                            column_name,
                            value,
                        )?);
                }
            }
        }

        let rows = rows.unwrap_or_default();
        if rows.is_empty() {
            return Err(".DC DATA sweep produced no table rows".to_string());
        }
        Ok(Some(XyceDcDataSweep { rows }))
    }

    pub(super) fn dc_data_override_for_column(
        netlist: &Netlist,
        column_name: &str,
        value: Value,
    ) -> Result<XyceDcDataOverride, String> {
        if let Some((device_name, param_name)) = Self::parse_device_parameter_probe(column_name)
            && Self::netlist_has_top_level_element_named(netlist, &device_name)
        {
            return Ok(XyceDcDataOverride::Device {
                name: device_name,
                param_name: Some(param_name),
                value,
            });
        }

        if Self::netlist_has_top_level_element_named(netlist, column_name) {
            return Ok(XyceDcDataOverride::Device {
                name: column_name.to_string(),
                param_name: None,
                value,
            });
        }

        if Self::netlist_has_numeric_parameter(netlist, column_name) {
            return Ok(XyceDcDataOverride::Parameter {
                name: column_name.to_string(),
                value,
            });
        }

        Err(format!(
            ".DC DATA column '{}' does not resolve to a top-level device value, device parameter, or numeric parameter",
            column_name
        ))
    }

    pub(super) fn synthetic_dc_data_sweep(
        dc_data: &XyceDcDataSweep,
    ) -> Result<XyceDcSweep, String> {
        if dc_data.rows.is_empty() {
            return Err(".DC DATA sweep produced no table rows".to_string());
        }
        Ok(XyceDcSweep {
            source: "DATA".to_string(),
            start: 0.0,
            stop: (dc_data.rows.len() - 1) as Value,
            step: 1.0,
            mode: rspice_core::netlist::DcSweepMode::Linear,
            sweep2: None,
        })
    }

    pub(super) fn dc_data_table_names(source: &str) -> Result<Vec<String>, String> {
        let mut table_names = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line);
            let trimmed = stripped.trim();
            let Some(command) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".dc") {
                continue;
            }
            if let Some(table_name) = Self::assignment_value(trimmed, "data")? {
                table_names.push(table_name);
            }
        }
        Ok(table_names)
    }

    pub(super) fn source_spec_is_finite_dc_or_pulse(
        spec: &rspice_core::netlist::SourceSpec,
    ) -> bool {
        match spec {
            rspice_core::netlist::SourceSpec::Dc(value) => value.is_finite(),
            rspice_core::netlist::SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                phase,
                ..
            } => {
                [*v1, *v2, *delay, *rise, *fall, *width, *period, *phase]
                    .into_iter()
                    .all(Value::is_finite)
                    && *delay >= 0.0
                    && *rise >= 0.0
                    && *fall >= 0.0
                    && *width >= 0.0
                    && *period > 0.0
            }
            _ => false,
        }
    }

    /// Return whether an independent source is within the strict transient
    /// source envelope used by the Release 7.10 integrated-RMS LEVEL=9
    /// verifier. Xyce accepts an explicit DC operating value followed by a
    /// PULSE waveform (`DC x PULSE(...)`); the transient solve uses the
    /// waveform while the DC operating point uses `x`. Keep that combined
    /// form separate from the ordinary relational-source helper so a
    /// dedicated absolute oracle cannot silently broaden unrelated contracts.
    pub(super) fn source_spec_is_finite_dc_or_pulse_or_explicit_dc_transient(
        spec: &rspice_core::netlist::SourceSpec,
    ) -> bool {
        match spec {
            rspice_core::netlist::SourceSpec::DcTransient {
                dc_value,
                transient,
            } => dc_value.is_finite() && Self::source_spec_is_finite_dc_or_pulse(transient),
            _ => Self::source_spec_is_finite_dc_or_pulse(spec),
        }
    }

    pub(super) fn evaluate_dc_probe(
        probe: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
    ) -> Result<f64, String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::evaluate_atomic_dc_probe(
                    &normalized_expression,
                    netlist,
                    dc,
                    sweep_point,
                    result,
                    op_report,
                );
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::evaluate_dc_probe_expression(
                    expression,
                    netlist,
                    dc,
                    sweep_point,
                    result,
                    op_report,
                );
            }
            let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
            return rspice_core::netlist::expr::eval_expression(expression, &context).map_err(
                |err| format!("failed to evaluate .PRINT DC expression '{{{expression}}}': {err}"),
            );
        }

        let normalized = Self::normalize_probe(probe);
        Self::evaluate_atomic_dc_probe(&normalized, netlist, dc, sweep_point, result, op_report)
    }

    pub(super) fn dc_node_voltage_index(
        result: &rspice_core::SimulationResult,
    ) -> HashMap<String, Value> {
        result
            .node_names
            .iter()
            .enumerate()
            .filter_map(|(index, name)| {
                result
                    .try_voltage(index)
                    .map(|voltage| (name.to_ascii_lowercase(), voltage))
            })
            .collect()
    }

    pub(super) fn evaluate_dc_probe_with_node_voltage_index(
        probe: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
        node_voltages: &HashMap<String, Value>,
    ) -> Result<Value, String> {
        let atomic_probe = Self::print_expression_inner(probe).unwrap_or(probe);
        let normalized = Self::normalize_probe(atomic_probe);
        if let Some(voltage_probe) = Self::parse_voltage_probe(&normalized)
            && Self::probe_call_covers_entire_expression(&normalized)
        {
            let pos =
                Self::indexed_dc_voltage_named(node_voltages, netlist, &voltage_probe.node_pos)
                    .ok_or_else(|| {
                        format!("node '{}' not found in DC result", voltage_probe.node_pos)
                    })?;
            let neg = match voltage_probe.node_neg {
                Some(node) => Self::indexed_dc_voltage_named(node_voltages, netlist, &node)
                    .ok_or_else(|| format!("node '{}' not found in DC result", node))?,
                None => 0.0,
            };
            return Ok(voltage_probe.accessor.evaluate_dc(pos - neg));
        }
        Self::evaluate_dc_probe(probe, netlist, dc, sweep_point, result, op_report)
    }

    pub(super) fn evaluate_atomic_dc_probe(
        normalized: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
    ) -> Result<f64, String> {
        if let Some(voltage_probe) = Self::parse_voltage_probe(normalized) {
            let pos = Self::result_voltage_named(result, netlist, &voltage_probe.node_pos)
                .ok_or_else(|| {
                    format!("node '{}' not found in DC result", voltage_probe.node_pos)
                })?;
            let neg = match voltage_probe.node_neg {
                Some(node) => Self::result_voltage_named(result, netlist, &node)
                    .ok_or_else(|| format!("node '{}' not found in DC result", node))?,
                None => 0.0,
            };
            return Ok(voltage_probe.accessor.evaluate_dc(pos - neg));
        }

        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
        {
            if parameter.eq_ignore_ascii_case("R")
                && let Some(store_name) =
                    Self::xyce_memristor_resistance_store_name(netlist, &element_name)
            {
                let observable = format!("N({store_name})");
                return result.try_dc_observable_named(&observable).ok_or_else(|| {
                    format!("Xyce memristor resistance store '{store_name}' not found in DC result")
                });
            }
            if let Some(value) = result.try_dc_observable_named(normalized) {
                return Ok(value);
            }
            return Self::evaluate_device_operating_point_probe(
                op_report,
                &element_name,
                &parameter,
            );
        }

        if let Some(lead_current) = Self::parse_lead_current_probe(normalized) {
            return Self::evaluate_lead_current_probe(op_report, &lead_current);
        }

        if let Some(value) = Self::evaluate_bare_device_parameter_probe(
            netlist,
            dc,
            sweep_point,
            result,
            op_report,
            normalized,
        ) {
            return value;
        }

        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            if parameter.eq_ignore_ascii_case("R")
                && let Some(store_name) =
                    Self::xyce_memristor_resistance_store_name(netlist, &element_name)
            {
                return result
                    .try_dc_observable_named(&format!("N({store_name})"))
                    .ok_or_else(|| {
                        format!(
                            "Xyce memristor resistance store '{store_name}' not found in DC result"
                        )
                    });
            }
            return Self::evaluate_device_parameter_probe(
                netlist,
                dc,
                sweep_point,
                result,
                op_report,
                &element_name,
                &parameter,
            );
        }

        if let Some(parameter_name) = Self::parse_scalar_parameter_probe(normalized) {
            return Self::evaluate_scalar_parameter_probe(
                netlist,
                dc,
                sweep_point,
                &parameter_name,
            );
        }

        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if let Some(current) = result.try_dc_observable_named(normalized) {
                return Ok(current);
            }
            if let Some(current) = Self::result_branch_current_named(result, &element_name) {
                return Ok(current);
            }
            if let Some(current) =
                Self::evaluate_current_source_current(netlist, dc, sweep_point, &element_name)
            {
                return Ok(current);
            }
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name)? {
                return Self::evaluate_resistor_current(netlist, result, &element_name, resistance);
            }
        }

        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if let Some(power) = result.try_dc_observable_named(normalized) {
                return Ok(power);
            }
            if let Some(resistance) = Self::effective_resistor_value(netlist, &element_name)? {
                return Self::evaluate_resistor_power(netlist, result, &element_name, resistance);
            }
        }

        Err(format!("unsupported DC probe '{}'", normalized))
    }

    pub(super) fn evaluate_dc_probe_expression(
        expression: &str,
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
    ) -> Result<f64, String> {
        let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::evaluate_atomic_dc_probe(&normalized, netlist, dc, sweep_point, result, op_report)
        };
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| {
                format!("failed to evaluate .PRINT DC expression '{{{expression}}}': {err}")
            })
    }

    pub(super) fn evaluate_ac_probe(
        probe: &str,
        netlist: &Netlist,
        result: &AcResult,
        phase_output_radians: bool,
    ) -> Result<Value, String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_ac_probe(&normalized_expression, netlist) {
                return Self::evaluate_atomic_ac_probe(
                    &normalized_expression,
                    netlist,
                    result,
                    phase_output_radians,
                );
            }
            return Ok(Self::evaluate_ac_complex_expression(expression, netlist, result)?.re);
        }

        let normalized = Self::normalize_probe(probe);
        Self::evaluate_atomic_ac_probe(&normalized, netlist, result, phase_output_radians)
            .or_else(|_| Ok(Self::evaluate_ac_complex_expression(probe, netlist, result)?.re))
    }

    pub(super) fn evaluate_atomic_ac_probe(
        normalized: &str,
        netlist: &Netlist,
        result: &AcResult,
        phase_output_radians: bool,
    ) -> Result<Value, String> {
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(normalized) {
            let value = Self::evaluate_ac_voltage_probe(&voltage_probe, netlist, result)?;
            return voltage_probe
                .accessor
                .evaluate_ac_scalar(value, phase_output_radians)
                .ok_or_else(|| {
                    format!(
                        "AC probe '{}' is complex-valued; compare Re()/Im() columns or use VM/VP/VDB",
                        normalized
                    )
                });
        }

        if let Some(current_probe) = Self::parse_ac_current_probe(normalized) {
            let current = Self::ac_branch_current_named(result, &current_probe.element_name)
                .ok_or_else(|| {
                    format!(
                        "branch '{}' not found in AC result",
                        current_probe.element_name
                    )
                })?;
            return current_probe
                .accessor
                .evaluate_ac_scalar(current, phase_output_radians)
                .ok_or_else(|| {
                    format!(
                        "AC probe '{}' is complex-valued; compare Re()/Im() columns or use IR/II/IM/IP/IDB",
                        normalized
                    )
                });
        }

        if let Some(value) =
            Self::evaluate_static_frequency_device_parameter_probe("AC", netlist, normalized)
        {
            return value;
        }

        Err(format!("unsupported AC probe '{}'", normalized))
    }

    pub(super) fn evaluate_ac_complex_probe(
        probe: &str,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<Complex64, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            if voltage_probe.accessor != XyceVoltageAccessor::Value {
                return Err(format!(
                    "AC complex probe '{}' must use bare V(...) accessor",
                    probe.trim()
                ));
            }
            return Self::evaluate_ac_voltage_probe(&voltage_probe, netlist, result);
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            if current_probe.accessor != XyceCurrentAccessor::Value {
                return Err(format!(
                    "AC complex probe '{}' must use bare I(...) accessor",
                    probe.trim()
                ));
            }
            return Self::ac_branch_current_named(result, &current_probe.element_name).ok_or_else(
                || {
                    format!(
                        "branch '{}' not found in AC result",
                        current_probe.element_name
                    )
                },
            );
        }
        Self::evaluate_ac_complex_expression(probe, netlist, result)
    }

    pub(super) fn evaluate_ac_complex_expression(
        expression: &str,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<Complex64, String> {
        let context = Self::print_eval_context(netlist, None, None);
        let mut call_value =
            |call: &str| Self::evaluate_ac_expression_call_probe(call, netlist, result);
        let (rewritten, context) =
            Self::rewrite_ac_print_expression_complex(expression, context, &mut call_value)?;
        let value = rspice_core::netlist::expr::eval_expression_complex(&rewritten, &context)
            .map_err(|err| {
                format!("failed to evaluate .PRINT AC expression '{{{expression}}}': {err}")
            })?;
        Ok(Complex64::new(value.re, value.im))
    }

    pub(super) fn evaluate_ac_expression_call_probe(
        call: &str,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<ExprComplexValue, String> {
        let normalized = Self::normalize_probe(call);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            let value = Self::evaluate_ac_voltage_probe(&voltage_probe, netlist, result)?;
            return Ok(
                match voltage_probe
                    .accessor
                    .evaluate_ac_scalar(value, false)
                    .map(ExprComplexValue::real)
                {
                    Some(value) => value,
                    None => ExprComplexValue::new(value.re, value.im),
                },
            );
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            let current = Self::ac_branch_current_named(result, &current_probe.element_name)
                .ok_or_else(|| {
                    format!(
                        "branch '{}' not found in AC result",
                        current_probe.element_name
                    )
                })?;
            return Ok(
                match current_probe
                    .accessor
                    .evaluate_ac_scalar(current, false)
                    .map(ExprComplexValue::real)
                {
                    Some(value) => value,
                    None => ExprComplexValue::new(current.re, current.im),
                },
            );
        }
        let value = Self::evaluate_ac_complex_probe(call, netlist, result)?;
        Ok(ExprComplexValue::new(value.re, value.im))
    }

    pub(super) fn evaluate_ac_voltage_probe(
        probe: &XyceVoltageProbe,
        netlist: &Netlist,
        result: &AcResult,
    ) -> Result<Complex64, String> {
        let pos = Self::ac_node_voltage_named(result, netlist, &probe.node_pos)
            .ok_or_else(|| format!("node '{}' not found in AC result", probe.node_pos))?;
        let neg = match probe.node_neg.as_deref() {
            Some(node) => Self::ac_node_voltage_named(result, netlist, node)
                .ok_or_else(|| format!("node '{}' not found in AC result", node))?,
            None => Complex64::new(0.0, 0.0),
        };
        Ok(pos - neg)
    }

    pub(super) fn evaluate_static_frequency_device_parameter_probe(
        analysis: &str,
        netlist: &Netlist,
        normalized: &str,
    ) -> Option<Result<Value, String>> {
        let (element_name, parameter) = Self::parse_device_parameter_probe(normalized)?;
        Some(match parameter.as_str() {
            "acmag" => Self::independent_source_ac_terms(netlist, &element_name)
                .map(|(magnitude, _)| magnitude)
                .ok_or_else(|| {
                    format!(
                        "{analysis} device parameter probe '{element_name}:ACMAG' has no independent source"
                    )
                }),
            "acphase" => Self::independent_source_ac_terms(netlist, &element_name)
                .map(|(_, phase)| phase.to_degrees())
                .ok_or_else(|| {
                    format!(
                        "{analysis} device parameter probe '{element_name}:ACPHASE' has no independent source"
                    )
                }),
            "r" => Self::effective_resistor_value(netlist, &element_name).and_then(|value| {
                value.ok_or_else(|| {
                    format!(
                        "{analysis} device parameter probe '{element_name}:R' has no finite resistance"
                    )
                })
            }),
            "c" => Self::effective_capacitor_value(netlist, &element_name).ok_or_else(|| {
                format!(
                    "{analysis} device parameter probe '{element_name}:C' has no finite capacitance"
                )
            }),
            "l" => Self::effective_inductor_value(netlist, &element_name).ok_or_else(|| {
                format!(
                    "{analysis} device parameter probe '{element_name}:L' has no finite inductance"
                )
            }),
            _ => Err(format!(
                "{analysis} device parameter probe '{element_name}:{parameter}' is not supported"
            )),
        })
    }

    pub(super) fn evaluate_tran_probe(
        probe: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<f64, String> {
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_real_probe(&normalized_expression, netlist) {
                return Self::evaluate_atomic_tran_probe(
                    &normalized_expression,
                    netlist,
                    result,
                    time,
                );
            }
            if Self::print_expression_contains_probe_reference(expression) {
                return Self::evaluate_tran_probe_expression(expression, netlist, result, time);
            }
            let context = Self::print_tran_eval_context(netlist, time);
            return rspice_core::netlist::expr::eval_expression(expression, &context).map_err(
                |err| {
                    format!("failed to evaluate .PRINT TRAN expression '{{{expression}}}': {err}")
                },
            );
        }

        let normalized = Self::normalize_probe(probe);
        Self::evaluate_atomic_tran_probe(&normalized, netlist, result, time)
    }

    pub(super) fn evaluate_atomic_tran_probe(
        normalized: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<f64, String> {
        if normalized == "time" {
            return Ok(time);
        }
        if normalized.eq_ignore_ascii_case("TEMP") || normalized.eq_ignore_ascii_case("TEMPER") {
            return Ok(Self::netlist_temperature_c(netlist));
        }

        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
            && parameter.eq_ignore_ascii_case("R")
            && let Some(store_name) =
                Self::xyce_memristor_resistance_store_name(netlist, &element_name)
        {
            return Self::transient_store_named(result, &store_name, time);
        }

        if let Some((element_name, parameter)) =
            Self::parse_device_operating_point_probe(normalized)
            && element_name.to_ascii_lowercase().starts_with("ymin!")
        {
            let waveform = if let Some(waveform) =
                result.try_device_op_waveform_named(&element_name, &parameter)
            {
                waveform
            } else if let Some(winding_name) = parameter
                .strip_suffix("_branch")
                .filter(|name| !name.is_empty())
                .or_else(|| {
                    parameter
                        .eq_ignore_ascii_case("branch")
                        .then(|| {
                            element_name
                                .rsplit_once('_')
                                .filter(|(core_name, winding_name)| {
                                    core_name.starts_with("ymin!") && !winding_name.is_empty()
                                })
                                .map(|(_, winding_name)| winding_name)
                        })
                        .flatten()
                })
            {
                // Xyce exposes each winding's public MNA branch through the
                // shared YMIN namespace.  The Rust result already stores the
                // same accepted current under the authored winding name, so
                // resolve this internal alias without duplicating a dynamic
                // device-op parameter for every transient sample.
                result
                    .try_branch_current_waveform_named(winding_name)
                    .ok_or_else(|| {
                        format!(
                            "nonlinear-core operating-point waveform '{}:{}' is not present in the transient result",
                            element_name, parameter
                        )
                    })?
            } else {
                return Err(format!(
                    "nonlinear-core operating-point waveform '{}:{}' is not present in the transient result",
                    element_name, parameter
                ));
            };
            return Self::interpolate_transient_waveform_at(&result.time, waveform, time);
        }

        if let Some(voltage_probe) = Self::parse_tran_voltage_probe(normalized) {
            let pos =
                Self::transient_voltage_named(result, netlist, &voltage_probe.node_pos, time)?;
            let neg = match voltage_probe.node_neg {
                Some(node) => Self::transient_voltage_named(result, netlist, &node, time)?,
                None => 0.0,
            };
            return Ok(voltage_probe.accessor.evaluate_dc(pos - neg));
        }

        if let Some(element_name) = Self::parse_current_probe(normalized) {
            if let Some(current) = Self::transient_branch_current_named(result, &element_name, time)
            {
                return Ok(current);
            }
            if let Some(value) = Self::evaluate_independent_current_source_probe(
                netlist,
                result,
                &element_name,
                time,
            ) {
                return Ok(value);
            }
            return Err(format!(
                "branch current '{}' not found in transient result",
                element_name
            ));
        }

        if let Some(element_name) = Self::parse_power_probe(normalized) {
            if let Some(element) =
                Self::find_recorded_two_terminal_branch_element(netlist, &element_name)
            {
                return Self::evaluate_transient_two_terminal_branch_power(
                    netlist,
                    result,
                    time,
                    "device",
                    &element.name,
                    &element,
                );
            }
            return Err(format!(
                "transient power probe '{}' targets an unsupported branch/device",
                element_name
            ));
        }

        if let Some((element_name, parameter)) = Self::parse_device_parameter_probe(normalized) {
            return Self::evaluate_transient_device_parameter_probe(
                netlist,
                result,
                time,
                &element_name,
                &parameter,
            );
        }

        Err(format!("unsupported TRAN probe '{}'", normalized))
    }

    pub(super) fn evaluate_tran_probe_expression(
        expression: &str,
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
    ) -> Result<f64, String> {
        let context = Self::print_tran_eval_context(netlist, time);
        let mut call_value = |call: &str| {
            let normalized = Self::normalize_probe(call);
            Self::evaluate_atomic_tran_probe(&normalized, netlist, result, time)
        };
        Self::evaluate_print_expression_with_probe_calls(expression, context, &mut call_value)
            .map_err(|err| {
                format!("failed to evaluate .PRINT TRAN expression '{{{expression}}}': {err}")
            })
    }

    pub(super) fn transient_branch_current_named(
        result: &TransientResult,
        branch_name: &str,
        time: Value,
    ) -> Option<Value> {
        Self::transient_branch_current_waveform_named(result, branch_name).and_then(|waveform| {
            Self::interpolate_transient_waveform_at(&result.time, waveform, time).ok()
        })
    }

    pub(super) fn transient_store_named(
        result: &TransientResult,
        store_name: &str,
        time: Value,
    ) -> Result<Value, String> {
        let waveform = result
            .try_store_waveform_named(store_name)
            .ok_or_else(|| format!("device store '{store_name}' not found in transient result"))?;
        Self::interpolate_transient_waveform_at(&result.time, waveform, time)
    }

    pub(super) fn transient_branch_current_waveform_named<'a>(
        result: &'a TransientResult,
        branch_name: &str,
    ) -> Option<&'a [Value]> {
        result
            .try_branch_current_waveform_named(branch_name)
            .or_else(|| {
                let normalized = Self::normalize_device_instance_name(branch_name);
                (normalized != branch_name)
                    .then(|| result.try_branch_current_waveform_named(&normalized))?
            })
    }

    pub(super) fn evaluate_transient_two_terminal_branch_power(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        device_kind: &str,
        branch_name: &str,
        element: &rspice_core::netlist::Element,
    ) -> Result<Value, String> {
        let node_pos = element
            .nodes
            .first()
            .ok_or_else(|| format!("{device_kind} '{}' has no positive node", element.name))?;
        let node_neg = element
            .nodes
            .get(1)
            .ok_or_else(|| format!("{device_kind} '{}' has no negative node", element.name))?;
        let pos_index = rspice_core::Engine::node_lookup_candidates(netlist, node_pos)
            .into_iter()
            .find_map(|candidate| result.node_index_named(&candidate))
            .ok_or_else(|| format!("node '{}' not found in transient result", node_pos))?;
        let neg_index = rspice_core::Engine::node_lookup_candidates(netlist, node_neg)
            .into_iter()
            .find_map(|candidate| result.node_index_named(&candidate))
            .ok_or_else(|| format!("node '{}' not found in transient result", node_neg))?;
        let current = Self::transient_branch_current_waveform_named(result, branch_name)
            .ok_or_else(|| {
                format!(
                    "branch current '{}' not found in transient result",
                    branch_name
                )
            })?;

        if current.len() != result.time.len() {
            return Err(format!(
                "branch current '{}' has {} sample(s) for {} time point(s)",
                branch_name,
                current.len(),
                result.time.len()
            ));
        }

        let mut power = Vec::with_capacity(result.time.len());
        for (index, branch_current) in current.iter().enumerate().take(result.time.len()) {
            let v_pos = if pos_index == 0 {
                0.0
            } else {
                result.try_voltage_at(pos_index, index).ok_or_else(|| {
                    format!(
                        "node '{}' sample {} not found in transient result",
                        node_pos, index
                    )
                })?
            };
            let v_neg = if neg_index == 0 {
                0.0
            } else {
                result.try_voltage_at(neg_index, index).ok_or_else(|| {
                    format!(
                        "node '{}' sample {} not found in transient result",
                        node_neg, index
                    )
                })?
            };
            power.push((v_pos - v_neg) * branch_current);
        }

        Self::interpolate_transient_waveform_at(&result.time, &power, time)
    }

    pub(super) fn transient_voltage_named(
        result: &TransientResult,
        netlist: &Netlist,
        node_name: &str,
        time: Value,
    ) -> Result<Value, String> {
        let node = Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| result.node_index_named(&candidate))
            .ok_or_else(|| format!("node '{}' not found in transient result", node_name))?;
        if node == 0 {
            return Ok(0.0);
        }
        let waveform = result
            .try_voltage_waveform(node)
            .ok_or_else(|| format!("node '{}' not found in transient result", node_name))?;
        Self::interpolate_transient_waveform_at(&result.time, waveform, time)
    }

    pub(super) fn evaluate_device_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        if let Some(value) = Self::evaluate_semiconductor_instance_parameter_probe(
            netlist,
            dc,
            sweep_point,
            element_name,
            parameter,
        ) {
            return value;
        }

        match parameter {
            "dcv0" => {
                if element_name.eq_ignore_ascii_case(&dc.source) {
                    return Ok(sweep_point.primary);
                }
                if let Some(sweep2) = &dc.sweep2
                    && element_name.eq_ignore_ascii_case(&sweep2.source)
                {
                    return sweep_point.secondary.ok_or_else(|| {
                        format!(
                            "secondary sweep value for '{}' is unavailable",
                            element_name
                        )
                    });
                }
                Self::independent_source_dc_value(netlist, element_name).ok_or_else(|| {
                    format!(
                        "source parameter probe '{}:DCV0' targets an unknown independent source",
                        element_name
                    )
                })
            }
            "r" => Self::evaluate_resistor_parameter_r_value(
                netlist,
                dc,
                sweep_point,
                result,
                op_report,
                element_name,
            ),
            "c" => {
                Self::evaluate_capacitor_parameter_c_value(netlist, dc, sweep_point, element_name)
            }
            "l" => {
                if Self::find_inductor_element(netlist, element_name).is_some() {
                    return Self::evaluate_inductor_parameter_l_value(
                        netlist,
                        dc,
                        sweep_point,
                        element_name,
                    );
                }
                if Self::resistor_instance_parameter_probe_is_supported(
                    netlist,
                    element_name,
                    parameter,
                ) {
                    return Self::evaluate_resistor_instance_parameter_probe(
                        netlist,
                        element_name,
                        parameter,
                    );
                }
                Self::evaluate_model_parameter_probe(
                    netlist,
                    dc,
                    sweep_point,
                    element_name,
                    parameter,
                )
                .unwrap_or_else(|| {
                    Err(format!(
                        "device parameter probe '{}:{}' is not supported",
                        element_name, parameter
                    ))
                })
            }
            "temp" => Self::resistor_temperature_value(netlist, element_name)?.ok_or_else(|| {
                format!(
                    "resistor parameter probe '{}:TEMP' targets an unknown resistor",
                    element_name
                )
            }),
            _ => {
                if Self::resistor_instance_parameter_probe_is_supported(
                    netlist,
                    element_name,
                    parameter,
                ) {
                    return Self::evaluate_resistor_instance_parameter_probe(
                        netlist,
                        element_name,
                        parameter,
                    );
                }
                Self::evaluate_model_parameter_probe(
                    netlist,
                    dc,
                    sweep_point,
                    element_name,
                    parameter,
                )
                .unwrap_or_else(|| {
                    Err(format!(
                        "device parameter probe '{}:{}' is not supported",
                        element_name, parameter
                    ))
                })
            }
        }
    }

    pub(super) fn evaluate_bare_device_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
        probe: &str,
    ) -> Option<Result<f64, String>> {
        let probe_name = Self::parse_bare_device_parameter_probe(probe)?;
        let element = Self::find_bare_device_parameter_element(netlist, &probe_name)?;
        Some(match &element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => Ok(
                Self::source_dc_parameter_value(dc, sweep_point, &element.name, spec),
            ),
            ElementKind::Resistor { .. } => Self::evaluate_resistor_parameter_r_value(
                netlist,
                dc,
                sweep_point,
                result,
                op_report,
                &probe_name,
            ),
            ElementKind::Capacitor {
                value,
                value_expr,
                instance_params,
                ..
            } => Self::evaluate_static_passive_parameter_value(
                netlist,
                dc,
                sweep_point,
                "capacitor",
                &probe_name,
                "C",
                *value,
                value_expr.as_deref(),
                instance_params,
            ),
            ElementKind::Inductor {
                value,
                value_expr,
                instance_params,
                ..
            } => Self::evaluate_static_passive_parameter_value(
                netlist,
                dc,
                sweep_point,
                "inductor",
                &probe_name,
                "L",
                *value,
                value_expr.as_deref(),
                instance_params,
            ),
            _ => Err(format!(
                "bare device parameter probe '{}' targets an unsupported element kind",
                probe_name
            )),
        })
    }

    pub(super) fn source_dc_parameter_value(
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        element_name: &str,
        spec: &rspice_core::netlist::SourceSpec,
    ) -> Value {
        if Self::device_instance_names_match(element_name, &dc.source) {
            return sweep_point.primary;
        }
        if let Some(sweep2) = &dc.sweep2
            && Self::device_instance_names_match(element_name, &sweep2.source)
            && let Some(value) = sweep_point.secondary
        {
            return value;
        }
        extract_dc_value(spec)
    }

    pub(super) fn evaluate_static_passive_parameter_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        device_kind: &str,
        element_name: &str,
        parameter_name: &str,
        value: Value,
        value_expr: Option<&str>,
        instance_params: &[(String, Value)],
    ) -> Result<Value, String> {
        if let Some(instance_value) =
            Self::instance_param(instance_params, &[parameter_name, "VALUE"])
        {
            return Ok(instance_value);
        }
        if value.is_finite() {
            return Ok(value);
        }
        if let Some(expression) = value_expr {
            let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
            return rspice_core::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                format!(
                    "failed to evaluate {device_kind} parameter probe '{element_name}:{parameter_name}': {err}"
                )
            });
        }
        Err(format!(
            "{device_kind} parameter probe '{element_name}:{parameter_name}' could not resolve a value"
        ))
    }

    pub(super) fn evaluate_capacitor_parameter_c_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_capacitor_element(netlist, name).ok_or_else(|| {
            format!("capacitor parameter probe '{name}:C' targets an unknown capacitor")
        })?;
        let ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "capacitor parameter probe '{name}:C' targets a non-capacitor element"
            ));
        };
        if (model.is_some()
            || Self::capacitor_instance_params_affect_effective_value(instance_params))
            && let Some(capacitance) = Self::effective_capacitor_value(netlist, name)
        {
            return Ok(capacitance);
        }
        Self::evaluate_static_passive_parameter_value(
            netlist,
            dc,
            sweep_point,
            "capacitor",
            name,
            "C",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    pub(super) fn evaluate_inductor_parameter_l_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_inductor_element(netlist, name).ok_or_else(|| {
            format!("inductor parameter probe '{name}:L' targets an unknown inductor")
        })?;
        let ElementKind::Inductor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "inductor parameter probe '{name}:L' targets a non-inductor element"
            ));
        };
        if (model.is_some()
            || Self::inductor_instance_params_affect_effective_value(instance_params))
            && let Some(inductance) = Self::effective_inductor_value(netlist, name)
        {
            return Ok(inductance);
        }
        Self::evaluate_static_passive_parameter_value(
            netlist,
            dc,
            sweep_point,
            "inductor",
            name,
            "L",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    pub(super) fn evaluate_transient_device_parameter_probe(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        if parameter == "r"
            && let Some(store_name) =
                Self::xyce_memristor_resistance_store_name(netlist, element_name)
        {
            return Self::transient_store_named(result, &store_name, time);
        }
        match parameter {
            "r" => Self::evaluate_transient_resistor_parameter_r_value(
                netlist,
                result,
                time,
                element_name,
            ),
            "c" => Self::evaluate_transient_capacitor_parameter_c_value(
                netlist,
                result,
                time,
                element_name,
            ),
            "l" => Self::evaluate_transient_inductor_parameter_l_value(
                netlist,
                result,
                time,
                element_name,
            ),
            "temp" => {
                if let Some(waveform) = result.try_device_op_waveform_named(element_name, "temp") {
                    Self::interpolate_transient_waveform_at(&result.time, waveform, time)
                } else {
                    Self::resistor_temperature_value(netlist, element_name)?.ok_or_else(|| {
                        format!(
                            "resistor parameter probe '{}:TEMP' targets an unknown resistor",
                            element_name
                        )
                    })
                }
            }
            _ => Err(format!(
                "device parameter probe '{}:{}' is not supported in transient output",
                element_name, parameter
            )),
        }
    }

    pub(super) fn evaluate_transient_static_passive_parameter_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        device_kind: &str,
        element_name: &str,
        parameter_name: &str,
        value: Value,
        value_expr: Option<&str>,
        instance_params: &[(String, Value)],
    ) -> Result<Value, String> {
        if let Some(instance_value) =
            Self::instance_param(instance_params, &[parameter_name, "VALUE"])
        {
            return Ok(instance_value);
        }
        if value.is_finite() {
            return Ok(value);
        }
        if let Some(expression) = value_expr {
            let context = Self::print_tran_eval_context(netlist, time);
            let mut call_value = |call: &str| {
                let normalized = Self::normalize_probe(call);
                Self::evaluate_atomic_tran_probe(&normalized, netlist, result, time)
            };
            return Self::evaluate_print_expression_with_probe_calls(
                expression,
                context,
                &mut call_value,
            )
            .map_err(|err| {
                format!(
                    "failed to evaluate transient {device_kind} parameter probe '{element_name}:{parameter_name}': {err}"
                )
            });
        }
        Err(format!(
            "{device_kind} parameter probe '{element_name}:{parameter_name}' could not resolve a value"
        ))
    }

    pub(super) fn evaluate_transient_resistor_parameter_r_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        name: &str,
    ) -> Result<Value, String> {
        if let Some(waveform) = result.try_device_op_waveform_named(name, "r") {
            return Self::interpolate_transient_waveform_at(&result.time, waveform, time);
        }
        let element = Self::find_resistor_element(netlist, name).ok_or_else(|| {
            format!("resistor parameter probe '{name}:R' targets an unknown resistor")
        })?;
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "resistor parameter probe '{name}:R' targets a non-resistor element"
            ));
        };

        if Self::resistor_uses_xyce_default_marker(instance_params)
            && let Some(resistance) = Self::effective_resistor_value(netlist, name)?
        {
            return Ok(resistance);
        }
        let value = Self::evaluate_transient_static_passive_parameter_value(
            netlist,
            result,
            time,
            "resistor",
            name,
            "R",
            *value,
            value_expr.as_deref(),
            instance_params,
        );
        if value.is_ok() {
            return value;
        }
        if model.is_some()
            && let Some(resistance) = Self::resistor_parameter_r_value(netlist, name)?
        {
            return Ok(resistance);
        }
        value
    }

    pub(super) fn evaluate_transient_capacitor_parameter_c_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_capacitor_element(netlist, name).ok_or_else(|| {
            format!("capacitor parameter probe '{name}:C' targets an unknown capacitor")
        })?;
        let ElementKind::Capacitor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "capacitor parameter probe '{name}:C' targets a non-capacitor element"
            ));
        };
        if let Some(waveform) = result.try_device_op_waveform_named(name, "c") {
            return Self::interpolate_transient_waveform_at(&result.time, waveform, time);
        }
        if (model.is_some()
            || Self::capacitor_instance_params_affect_effective_value(instance_params))
            && let Some(capacitance) = Self::effective_capacitor_value(netlist, name)
        {
            return Ok(capacitance);
        }
        Self::evaluate_transient_static_passive_parameter_value(
            netlist,
            result,
            time,
            "capacitor",
            name,
            "C",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    pub(super) fn evaluate_transient_inductor_parameter_l_value(
        netlist: &Netlist,
        result: &TransientResult,
        time: Value,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_inductor_element(netlist, name).ok_or_else(|| {
            format!("inductor parameter probe '{name}:L' targets an unknown inductor")
        })?;
        let ElementKind::Inductor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "inductor parameter probe '{name}:L' targets a non-inductor element"
            ));
        };
        if (model.is_some()
            || Self::inductor_instance_params_affect_effective_value(instance_params))
            && let Some(inductance) = Self::effective_inductor_value(netlist, name)
        {
            return Ok(inductance);
        }
        Self::evaluate_transient_static_passive_parameter_value(
            netlist,
            result,
            time,
            "inductor",
            name,
            "L",
            *value,
            value_expr.as_deref(),
            instance_params,
        )
    }

    pub(super) fn evaluate_device_operating_point_probe(
        op_report: &rspice_core::circuit::DeviceOpReport,
        element_name: &str,
        parameter: &str,
    ) -> Result<f64, String> {
        let canonical_parameter =
            Self::canonical_device_op_parameter(parameter).ok_or_else(|| {
                format!(
                    "device operating-point probe 'N({element_name}:{parameter})' targets an unsupported operating-point parameter"
                )
            })?;

        for entry in &op_report.entries {
            if !Self::device_instance_names_match(&entry.name, element_name) {
                continue;
            }
            if let Some(value) = Self::xyce_device_operating_point_value(entry, canonical_parameter)
            {
                return Ok(value);
            }
            return Err(format!(
                "device operating-point probe 'N({element_name}:{parameter})' targets parameter '{}' that is not reported for {} '{}'",
                canonical_parameter, entry.device_kind, entry.name
            ));
        }

        Err(format!(
            "device operating-point probe 'N({element_name}:{parameter})' targets a device with no operating-point report"
        ))
    }

    pub(super) fn evaluate_lead_current_probe(
        op_report: &rspice_core::circuit::DeviceOpReport,
        probe: &XyceLeadCurrentProbe,
    ) -> Result<f64, String> {
        for entry in &op_report.entries {
            if !Self::device_instance_names_match(&entry.name, &probe.element_name) {
                continue;
            }
            let parameter = probe
                .terminal
                .op_parameter()
                .or_else(|| {
                    if entry.device_kind == "BJT" {
                        match probe.terminal {
                            XyceLeadCurrentTerminal::Bulk => Some("ib"),
                            XyceLeadCurrentTerminal::Collector => Some("ic"),
                            XyceLeadCurrentTerminal::Emitter => Some("ie"),
                            _ => None,
                        }
                    } else {
                        None
                    }
                })
                .ok_or_else(|| {
                    format!(
                        "lead-current probe '{}({})' targets unsupported terminal current",
                        probe.terminal.function_name(),
                        probe.element_name
                    )
                })?;
            if let Some(value) = Self::xyce_device_operating_point_value(entry, parameter) {
                return Ok(value);
            }
            return Err(format!(
                "lead-current probe '{}({})' targets parameter '{}' that is not reported for {} '{}'",
                probe.terminal.function_name(),
                probe.element_name,
                parameter,
                entry.device_kind,
                entry.name
            ));
        }

        Err(format!(
            "lead-current probe '{}({})' targets a device with no operating-point report",
            probe.terminal.function_name(),
            probe.element_name
        ))
    }

    pub(super) fn dc_probe_is_omitted_empty_wildcard(probe: &str, netlist: &Netlist) -> bool {
        Self::parse_lead_current_probe(probe).is_some_and(|probe| {
            Self::lead_current_probe_is_omitted_empty_wildcard(netlist, &probe)
        })
    }

    pub(super) fn native_transient_independent_source_spec(
        spec: &rspice_core::netlist::SourceSpec,
    ) -> bool {
        match spec {
            rspice_core::netlist::SourceSpec::Dc(value) => value.is_finite(),
            rspice_core::netlist::SourceSpec::Ac { magnitude, phase } => {
                magnitude.is_finite() && phase.is_finite()
            }
            rspice_core::netlist::SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } => dc_value.is_finite() && ac_magnitude.is_finite() && ac_phase.is_finite(),
            rspice_core::netlist::SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                phase,
                ..
            } => [*v1, *v2, *delay, *rise, *fall, *width, *period, *phase]
                .into_iter()
                .all(Value::is_finite),
            rspice_core::netlist::SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => [*offset, *amplitude, *frequency, *delay, *damping, *phase]
                .into_iter()
                .all(Value::is_finite),
            rspice_core::netlist::SourceSpec::Pwl { points, delay, .. } => {
                delay.is_finite()
                    && points
                        .iter()
                        .all(|(time, value)| time.is_finite() && value.is_finite())
            }
            rspice_core::netlist::SourceSpec::Pat {
                vhi,
                vlo,
                delay,
                rise,
                fall,
                sample,
                ..
            } => [*vhi, *vlo, *delay, *rise, *fall, *sample]
                .into_iter()
                .all(Value::is_finite),
            rspice_core::netlist::SourceSpec::DcTransient {
                dc_value,
                transient,
            }
            | rspice_core::netlist::SourceSpec::DcAcTransient {
                dc_value,
                transient,
                ..
            } => dc_value.is_finite() && Self::native_transient_independent_source_spec(transient),
            _ => false,
        }
    }

    pub(super) fn evaluate_scalar_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        parameter_name: &str,
    ) -> Result<f64, String> {
        if parameter_name.eq_ignore_ascii_case("TEMP")
            || parameter_name.eq_ignore_ascii_case("TEMPER")
        {
            return Ok(Self::active_temperature_c(
                netlist,
                Some(dc),
                Some(sweep_point),
            ));
        }
        if parameter_name.eq_ignore_ascii_case("VT") {
            return Ok(Self::thermal_voltage_celsius(Self::active_temperature_c(
                netlist,
                Some(dc),
                Some(sweep_point),
            )));
        }

        netlist
            .params
            .get(parameter_name)
            .map(Ok)
            .unwrap_or_else(|| {
                let expression = Self::scalar_parameter_expression(&netlist.params, parameter_name)
                    .ok_or_else(|| {
                        format!("scalar parameter probe '{}' is not defined", parameter_name)
                    })?;
                let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
                let prepared =
                    rspice_core::netlist::expr::prepare_behavioral_expression(expression, &context)
                        .map_err(|err| {
                            format!(
                                "scalar parameter probe '{}' could not be prepared: {}",
                                parameter_name, err
                            )
                        })?;
                rspice_core::netlist::expr::eval_expression(&prepared, &context).map_err(|err| {
                    format!(
                        "scalar parameter probe '{}' could not be evaluated: {}",
                        parameter_name, err
                    )
                })
            })
    }

    /// Xyce exposes an independent source's named DC value as a scalar in
    /// brace expressions (for example, `Vsrc/I(Vsrc)`).  Preserve explicit
    /// user parameters with the same spelling; otherwise bind the source's
    /// canonical operating-point value just as the native source evaluator
    /// does at TIME=0.
    pub(super) fn add_independent_source_parameter_bindings(
        netlist: &Netlist,
        context: &mut rspice_core::netlist::ParamContext,
    ) {
        for element in &netlist.elements {
            let (ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec)) =
                &element.kind
            else {
                continue;
            };
            if context.has_parameter_binding(&element.name)
                || context.get_global_expression(&element.name).is_some()
            {
                continue;
            }
            let value = Self::independent_source_dcv0(spec);
            if value.is_finite() {
                context.set(&element.name, value);
            }
        }
    }

    pub(super) fn independent_source_dcv0(spec: &rspice_core::netlist::SourceSpec) -> Value {
        match spec {
            rspice_core::netlist::SourceSpec::Distortion { inner, .. }
            | rspice_core::netlist::SourceSpec::RfPort { inner, .. } => {
                Self::independent_source_dcv0(inner)
            }
            rspice_core::netlist::SourceSpec::Dc(value)
            | rspice_core::netlist::SourceSpec::DcAc {
                dc_value: value, ..
            }
            | rspice_core::netlist::SourceSpec::DcTransient {
                dc_value: value, ..
            }
            | rspice_core::netlist::SourceSpec::DcAcTransient {
                dc_value: value, ..
            } => *value,
            _ => 0.0,
        }
    }

    pub(super) fn evaluate_resistor_current(
        netlist: &Netlist,
        result: &rspice_core::SimulationResult,
        resistor_name: &str,
        resistance: Value,
    ) -> Result<f64, String> {
        if Self::resistor_uses_branch_form(netlist, resistance) {
            return Err(format!(
                "missing solved branch current for zero/near-zero resistor '{}'",
                resistor_name
            ));
        }
        if resistance.is_infinite() && resistance.is_sign_positive() {
            return Ok(0.0);
        }
        Ok(Self::resistor_voltage_drop(netlist, result, resistor_name)? / resistance)
    }

    pub(super) fn evaluate_resistor_power(
        netlist: &Netlist,
        result: &rspice_core::SimulationResult,
        resistor_name: &str,
        resistance: Value,
    ) -> Result<f64, String> {
        let voltage_drop = Self::resistor_voltage_drop(netlist, result, resistor_name)?;
        let current =
            if let Some(current) = Self::result_branch_current_named(result, resistor_name) {
                current
            } else {
                Self::evaluate_resistor_current(netlist, result, resistor_name, resistance)?
            };
        Ok(voltage_drop * current)
    }

    pub(super) fn ac_node_voltage_named(
        result: &AcResult,
        netlist: &Netlist,
        node_name: &str,
    ) -> Option<Complex64> {
        if netlist.ground_policy().is_ground(node_name) {
            return Some(Complex64::new(0.0, 0.0));
        }
        Self::node_lookup_candidates(netlist, node_name)
            .into_iter()
            .find_map(|candidate| {
                result
                    .node_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case(&candidate))
                    .and_then(|index| result.voltages.get(index).copied())
            })
    }

    pub(super) fn ac_branch_current_named(
        result: &AcResult,
        branch_name: &str,
    ) -> Option<Complex64> {
        result
            .branch_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(branch_name))
            .and_then(|index| result.currents.get(index).copied())
            .or_else(|| {
                let normalized = Self::normalize_device_instance_name(branch_name);
                (normalized != branch_name).then(|| {
                    result
                        .branch_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case(&normalized))
                        .and_then(|index| result.currents.get(index).copied())
                })?
            })
    }

    pub(super) fn evaluate_semiconductor_instance_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        element_name: &str,
        parameter: &str,
    ) -> Option<Result<Value, String>> {
        let element = Self::find_semiconductor_device_element(netlist, element_name)?;
        if parameter.eq_ignore_ascii_case("TEMP") {
            let (_, instance_params) = Self::semiconductor_model_and_instance_params(&element)?;
            return Some(Ok(Self::instance_param(instance_params, &["TEMP"])
                .unwrap_or_else(|| {
                    Self::active_temperature_c(netlist, Some(dc), Some(sweep_point))
                })));
        }

        let (model_name, instance_params) =
            Self::semiconductor_model_and_instance_params(&element)?;
        if let Some(value) = Self::instance_param(instance_params, &[parameter]) {
            return Some(Ok(value));
        }
        Self::evaluate_semiconductor_model_parameter_probe(
            netlist,
            dc,
            sweep_point,
            model_name,
            parameter,
        )
    }

    pub(super) fn dc_probe_index(netlist: &Netlist) -> XyceDcProbeIndex {
        let mut index = XyceDcProbeIndex::default();
        for element in &netlist.elements {
            let name = Self::normalize_device_instance_name(&element.name);
            if matches!(element.kind, ElementKind::Diode { .. }) {
                index.diode_names.insert(name.clone());
            }
            if Self::element_has_recorded_branch_current(&element.kind) {
                index.recorded_branch_names.insert(name);
            }
        }
        index
    }

    pub(super) fn elements_have_independent_current_source(
        elements: &[rspice_core::netlist::Element],
        source: &str,
    ) -> bool {
        elements.iter().any(|element| {
            Self::device_instance_names_match(&element.name, source)
                && matches!(&element.kind, ElementKind::CurrentSource(_))
        })
    }

    pub(super) fn evaluate_independent_current_source_probe(
        netlist: &Netlist,
        result: &TransientResult,
        source: &str,
        time: Value,
    ) -> Option<Value> {
        if Self::tran_uses_uic(netlist)
            && Self::time_is_transient_initial_sample(result, time)
            && Self::netlist_has_independent_current_source(netlist, source)
        {
            return Some(0.0);
        }
        if let Some(value) = Self::evaluate_current_source_probe_from_elements(
            &netlist.elements,
            result,
            source,
            time,
        ) {
            return Some(value);
        }

        let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist).ok()?;
        Self::evaluate_current_source_probe_from_elements(&flattened.elements, result, source, time)
    }

    pub(super) fn evaluate_current_source_probe_from_elements(
        elements: &[rspice_core::netlist::Element],
        result: &TransientResult,
        source: &str,
        time: Value,
    ) -> Option<Value> {
        let spec = elements.iter().find_map(|element| {
            if Self::device_instance_names_match(&element.name, source)
                && let ElementKind::CurrentSource(spec) = &element.kind
            {
                return Some(spec);
            }
            None
        })?;
        let (tstep, tstop) = Self::transient_result_source_context(result);
        Some(
            rspice_core::circuit::VoltageSources::evaluate_source_spec_at_time_with_dialect(
                spec,
                time,
                tstep,
                tstop,
                SpiceDialect::Xyce,
            ),
        )
    }

    pub(super) fn transient_result_source_context(result: &TransientResult) -> (Value, Value) {
        let tstop = result.time.last().copied().unwrap_or(1e99).max(1e-18);
        let mut previous: Option<Value> = None;
        let mut min_step: Option<Value> = None;
        for &sample in &result.time {
            if let Some(previous_sample) = previous {
                let step = sample - previous_sample;
                if step.is_finite() && step > 0.0 {
                    min_step = Some(min_step.map_or(step, |current| current.min(step)));
                }
            }
            previous = Some(sample);
        }
        (min_step.unwrap_or(1e-12), tstop)
    }

    pub(super) fn source_is_voltage_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(&element.kind, ElementKind::VoltageSource(_))
        })
    }

    pub(super) fn source_is_current_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(&element.kind, ElementKind::CurrentSource(_))
        })
    }

    pub(super) fn source_is_independent_source(netlist: &Netlist, source: &str) -> bool {
        netlist.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case(source)
                && matches!(
                    &element.kind,
                    ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
                )
        })
    }

    pub(super) fn independent_source_dc_value(netlist: &Netlist, source: &str) -> Option<Value> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))
            .and_then(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Some(extract_dc_value(spec))
                }
                _ => None,
            })
    }

    pub(super) fn independent_source_ac_terms(
        netlist: &Netlist,
        source: &str,
    ) -> Option<(Value, Value)> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))
            .and_then(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Some(extract_ac_value(spec))
                }
                _ => None,
            })
    }

    pub(super) fn evaluate_current_source_current(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        source: &str,
    ) -> Option<Value> {
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(source))?;
        let ElementKind::CurrentSource(spec) = &element.kind else {
            return None;
        };

        if source.eq_ignore_ascii_case(&dc.source) {
            return Some(sweep_point.primary);
        }
        if let Some(sweep2) = &dc.sweep2
            && source.eq_ignore_ascii_case(&sweep2.source)
        {
            return sweep_point.secondary;
        }

        Some(extract_dc_value(spec))
    }

    pub(super) fn scalar_parameter_sweep_source_is_supported(
        netlist: &Netlist,
        parameter_name: &str,
    ) -> bool {
        netlist
            .params
            .all_params()
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case(parameter_name))
    }

    pub(super) fn evaluate_resistor_parameter_r_value(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        result: &rspice_core::SimulationResult,
        op_report: &rspice_core::circuit::DeviceOpReport,
        name: &str,
    ) -> Result<Value, String> {
        let element = Self::find_resistor_element(netlist, name).ok_or_else(|| {
            format!("resistor parameter probe '{name}:R' targets an unknown resistor")
        })?;
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            ..
        } = &element.kind
        else {
            return Err(format!(
                "resistor parameter probe '{name}:R' targets a non-resistor element"
            ));
        };

        if Self::resistor_uses_xyce_default_marker(instance_params)
            && let Some(resistance) = Self::effective_resistor_value(netlist, name)?
        {
            return Ok(resistance);
        }
        if let Some(resistance) = Self::instance_param(instance_params, &["R", "VALUE"]) {
            return Ok(resistance);
        }
        if value.is_finite() {
            return Ok(*value);
        }
        if let Some(expression) = value_expr.as_deref() {
            let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
            let mut call_value = |call: &str| {
                let normalized = Self::normalize_probe(call);
                Self::evaluate_atomic_dc_probe(
                    &normalized,
                    netlist,
                    dc,
                    sweep_point,
                    result,
                    op_report,
                )
            };
            return Self::evaluate_print_expression_with_probe_calls(
                expression,
                context,
                &mut call_value,
            )
            .map_err(|err| {
                format!("failed to evaluate resistor parameter probe '{name}:R': {err}")
            });
        }
        if model.is_some()
            && let Some(resistance) = Self::resistor_parameter_r_value(netlist, name)?
        {
            return Ok(resistance);
        }

        Err(format!(
            "resistor parameter probe '{name}:R' could not resolve a resistance value"
        ))
    }

    pub(super) fn evaluate_resistor_instance_parameter_probe(
        netlist: &Netlist,
        name: &str,
        parameter: &str,
    ) -> Result<Value, String> {
        Self::resistor_instance_parameter_value(netlist, name, parameter)?.ok_or_else(|| {
            format!(
                "resistor parameter probe '{}:{}' targets an unknown or unset resistor instance parameter",
                name, parameter
            )
        })
    }

    pub(super) fn step_commands(netlist: &Netlist) -> Result<Vec<StepCommand>, String> {
        let step_commands = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .cloned()
            .collect::<Vec<_>>();

        for step in &step_commands {
            match &step.sweep {
                StepSweep::Data { table_name } => {
                    let table = netlist
                        .data_tables
                        .iter()
                        .find(|table| table.name.eq_ignore_ascii_case(table_name))
                        .ok_or_else(|| format!(".STEP DATA table '{table_name}' not found"))?;
                    if table.params.is_empty() {
                        return Err(format!(".STEP DATA table '{}' has no columns", table.name));
                    }
                    if table.rows.is_empty() {
                        return Err(format!(".STEP DATA table '{}' has no rows", table.name));
                    }
                    for (row_index, row) in table.rows.iter().enumerate() {
                        if row.len() != table.params.len() {
                            return Err(format!(
                                ".STEP DATA table '{}' row {} has {} value(s), expected {}",
                                table.name,
                                row_index,
                                row.len(),
                                table.params.len()
                            ));
                        }
                    }
                }
                _ if step.sweep.values().is_empty() => {
                    return Err("deck has invalid .STEP sweep bounds".to_string());
                }
                _ => {}
            }
        }

        Ok(step_commands)
    }

    pub(super) fn xyce_ac_sweep_frequencies(
        variation: rspice_core::netlist::FreqVariation,
        points: usize,
        fstart: Value,
        fstop: Value,
    ) -> Vec<Value> {
        if points == 0
            || points > i32::MAX as usize
            || !fstart.is_finite()
            || !fstop.is_finite()
            || fstart > fstop
        {
            return Vec::new();
        }
        match variation {
            rspice_core::netlist::FreqVariation::Dec | rspice_core::netlist::FreqVariation::Oct
                if fstart <= 0.0 =>
            {
                return Vec::new();
            }
            rspice_core::netlist::FreqVariation::Lin if fstart < 0.0 => return Vec::new(),
            _ => {}
        }

        let count = match variation {
            rspice_core::netlist::FreqVariation::Lin => points,
            rspice_core::netlist::FreqVariation::Dec => {
                let span = (fstart.log10() - fstop.log10()).abs();
                let count = (span * points as Value + 1.0).floor();
                if !count.is_finite() || count <= 0.0 || count > i32::MAX as Value {
                    return Vec::new();
                }
                count as usize
            }
            rspice_core::netlist::FreqVariation::Oct => {
                let span = (fstart.ln() - fstop.ln()).abs() / std::f64::consts::LN_2;
                let count = (span * points as Value + 1.0).floor();
                if !count.is_finite() || count <= 0.0 || count > i32::MAX as Value {
                    return Vec::new();
                }
                count as usize
            }
        };

        let step = match variation {
            rspice_core::netlist::FreqVariation::Lin => {
                if count <= 1 {
                    0.0
                } else {
                    (fstop - fstart) / (count - 1) as Value
                }
            }
            rspice_core::netlist::FreqVariation::Dec => {
                (1.0 / points as Value * std::f64::consts::LN_10).exp()
            }
            rspice_core::netlist::FreqVariation::Oct => {
                (1.0 / points as Value * std::f64::consts::LN_2).exp()
            }
        };

        let frequencies = (0..count)
            .map(|index| match variation {
                rspice_core::netlist::FreqVariation::Lin => fstart + index as Value * step,
                rspice_core::netlist::FreqVariation::Dec
                | rspice_core::netlist::FreqVariation::Oct => fstart * step.powf(index as Value),
            })
            .collect::<Vec<_>>();
        if frequencies.iter().all(|frequency| frequency.is_finite()) {
            frequencies
        } else {
            Default::default()
        }
    }

    pub(super) fn dc_sweep_dimensions(netlist: &Netlist) -> Vec<XyceDcSweepDimension> {
        let mut dimensions = Vec::new();
        for analysis in &netlist.analyses {
            let AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode,
                sweep2,
            } = analysis
            else {
                continue;
            };

            dimensions.push(XyceDcSweepDimension {
                source: source.clone(),
                start: *start,
                stop: *stop,
                step: *step,
                mode: mode.clone(),
            });
            if let Some(sweep2) = sweep2 {
                dimensions.push(XyceDcSweepDimension {
                    source: sweep2.source.clone(),
                    start: sweep2.start,
                    stop: sweep2.stop,
                    step: sweep2.step,
                    mode: sweep2.mode.clone(),
                });
            }
        }
        dimensions
    }

    pub(super) fn synthetic_op_dc_sweep(netlist: &Netlist) -> Result<XyceDcSweep, String> {
        for element in &netlist.elements {
            let dc_value = match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    extract_dc_value(spec)
                }
                _ => continue,
            };
            if !dc_value.is_finite() {
                return Err(format!(
                    ".OP source '{}' has non-finite DC value {}",
                    element.name, dc_value
                ));
            }
            return Ok(XyceDcSweep {
                source: element.name.clone(),
                start: dc_value,
                stop: dc_value,
                step: 1.0,
                mode: rspice_core::netlist::DcSweepMode::Linear,
                sweep2: None,
            });
        }

        Err(".OP static .PRINT DC output requires at least one independent source for the native one-point adapter".to_string())
    }

    pub(super) fn ac_csd_column_expansion(
        columns: &[String],
        complex_values: bool,
    ) -> Vec<XyceAcCsdColumnExpansion> {
        columns
            .iter()
            .map(|column| {
                if complex_values && Self::ac_csd_column_is_complex_probe(column) {
                    XyceAcCsdColumnExpansion::Complex
                } else {
                    XyceAcCsdColumnExpansion::Scalar
                }
            })
            .collect()
    }

    pub(super) fn ac_csd_column_is_complex_probe(column: &str) -> bool {
        let normalized = Self::normalize_probe(column);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            return voltage_probe.accessor == XyceVoltageAccessor::Value;
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            return current_probe.accessor == XyceCurrentAccessor::Value;
        }
        false
    }

    pub(super) fn step_commands_match_exactly(left: &[StepCommand], right: &[StepCommand]) -> bool {
        fn sweep_matches(left: &StepSweep, right: &StepSweep) -> bool {
            match (left, right) {
                (
                    StepSweep::Linear {
                        start: a,
                        stop: b,
                        step: c,
                    },
                    StepSweep::Linear {
                        start: d,
                        stop: e,
                        step: f,
                    },
                ) => [a, b, c]
                    .into_iter()
                    .zip([d, e, f])
                    .all(|(x, y)| x.to_bits() == y.to_bits()),
                (
                    StepSweep::Decade {
                        points_per_decade: a,
                        start: b,
                        stop: c,
                    },
                    StepSweep::Decade {
                        points_per_decade: d,
                        start: e,
                        stop: f,
                    },
                )
                | (
                    StepSweep::Octave {
                        points_per_octave: a,
                        start: b,
                        stop: c,
                    },
                    StepSweep::Octave {
                        points_per_octave: d,
                        start: e,
                        stop: f,
                    },
                ) => a == d && b.to_bits() == e.to_bits() && c.to_bits() == f.to_bits(),
                (StepSweep::List(a), StepSweep::List(b)) => {
                    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.to_bits() == y.to_bits())
                }
                (StepSweep::Data { table_name: a }, StepSweep::Data { table_name: b }) => {
                    a.eq_ignore_ascii_case(b)
                }
                _ => false,
            }
        }
        left.len() == right.len()
            && left.iter().zip(right).all(|(a, b)| {
                a.target == b.target
                    && a.name.eq_ignore_ascii_case(&b.name)
                    && a.param_name.as_deref().map(str::to_ascii_lowercase)
                        == b.param_name.as_deref().map(str::to_ascii_lowercase)
                    && sweep_matches(&a.sweep, &b.sweep)
            })
    }

    pub(super) fn source_has_bare_resistor_parameter_value(
        source: &str,
        resistor_name: &str,
        parameter_name: &str,
    ) -> bool {
        let mut matching_lines = source.lines().filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('*') {
                return None;
            }
            let fields = line.split_ascii_whitespace().collect::<Vec<_>>();
            fields
                .first()
                .is_some_and(|name| name.eq_ignore_ascii_case(resistor_name))
                .then_some(fields)
        });
        let Some(fields) = matching_lines.next() else {
            return false;
        };
        matching_lines.next().is_none() && {
            fields.len() == 4
                && fields[3]
                    .trim_matches(['{', '}'])
                    .eq_ignore_ascii_case(parameter_name)
        }
    }

    pub(super) fn source_has_sole_global_parameter_definition(
        source: &str,
        parameter_name: &str,
    ) -> bool {
        let mut definitions = source.lines().filter_map(|line| {
            let line = line.trim();
            if line.is_empty() || line.starts_with('*') {
                return None;
            }
            let compact = line
                .chars()
                .filter(|character| !character.is_ascii_whitespace())
                .collect::<String>();
            compact
                .get(..13)
                .is_some_and(|prefix| prefix.eq_ignore_ascii_case(".global_param"))
                .then_some(compact)
        });
        let Some(definition) = definitions.next() else {
            return false;
        };
        if definitions.next().is_some() {
            return false;
        }
        let Some(assignment) = definition.get(13..) else {
            return false;
        };
        let Some((name, expression)) = assignment.split_once('=') else {
            return false;
        };
        !expression.is_empty()
            && !expression.contains(',')
            && name.eq_ignore_ascii_case(parameter_name)
    }

    pub(super) fn analytic_generated_wrapper_source(&self, deck: &XyceDeck) -> Option<String> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/")
            || !self.requires_upstream_wrapper(&deck.relative_path)
            || fs::metadata(&deck.path)
                .ok()
                .is_none_or(|metadata| !metadata.is_file() || metadata.len() == 0)
            || self.has_static_tran_reference_oracle(&deck.path)
        {
            return None;
        }
        fs::read_to_string(&deck.path).ok()
    }

    pub(super) fn source_has_analysis(source: &str, analysis: &str) -> bool {
        let expected = format!(".{analysis}");
        Self::logical_netlist_lines(source).iter().any(|line| {
            let Some(command) = Self::strip_netlist_comment(line).split_whitespace().next() else {
                return false;
            };
            command.eq_ignore_ascii_case(&expected)
        })
    }

    pub(super) fn source_has_op_analysis(source: &str) -> bool {
        Self::source_has_analysis(source, "OP")
    }
}
