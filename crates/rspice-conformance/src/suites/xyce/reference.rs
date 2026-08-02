//! Reading Xyce `.prn` and sidecar reference data.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn parse_xyce_netlist(
        source: &str,
        deck_path: &Path,
    ) -> Result<Netlist, rspice_core::netlist::ParseError> {
        Self::parse_netlist_with_expression_dialect(source, deck_path, ExpressionDialect::Xyce)
    }

    pub(super) fn parse_error_is_undefined_ac_frequency_symbol(
        err: &rspice_core::netlist::ParseError,
    ) -> bool {
        let message = err.to_string().to_ascii_uppercase();
        message.contains("UNDEFINED PARAMETER: FREQ")
            || message.contains("UNDEFINED PARAMETER: HERTZ")
    }

    pub(super) fn text_contains_ascii_identifier_reference(text: &str, identifier: &str) -> bool {
        let bytes = text.as_bytes();
        let mut index = 0usize;
        while index < bytes.len() {
            if !(bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_') {
                index += 1;
                continue;
            }
            let start = index;
            index += 1;
            while index < bytes.len()
                && (bytes[index].is_ascii_alphanumeric() || bytes[index] == b'_')
            {
                index += 1;
            }
            if !text[start..index].eq_ignore_ascii_case(identifier) {
                continue;
            }
            let mut following = index;
            while following < bytes.len() && bytes[following].is_ascii_whitespace() {
                following += 1;
            }
            if following >= bytes.len() || bytes[following] != b'=' {
                return true;
            }
        }
        false
    }

    pub(super) fn parse_error_is_unbound_ac_frequency_dependency(
        source: &str,
        err: &rspice_core::netlist::ParseError,
    ) -> bool {
        if Self::parse_error_is_undefined_ac_frequency_symbol(err) {
            return true;
        }
        if !err
            .to_string()
            .to_ascii_uppercase()
            .contains("UNDEFINED PARAMETER:")
        {
            return false;
        }

        Self::source_has_ac_frequency_dependent_parameter(source)
    }

    pub(super) fn parse_xyce_paramfile_variables(
        content: &str,
        paramfile_path: &Path,
    ) -> Result<Vec<(String, Value)>, String> {
        let mut lines = content.lines().enumerate().filter_map(|(index, raw)| {
            let trimmed = raw.trim();
            (!trimmed.is_empty()).then_some((index + 1, trimmed))
        });
        let Some((header_line, header)) = lines.next() else {
            return Ok(Vec::new());
        };
        let header_tokens = header.split_whitespace().collect::<Vec<_>>();
        if header_tokens.len() != 2 || !header_tokens[1].eq_ignore_ascii_case("variables") {
            return Err(format!(
                "unsupported Xyce wrapper parameter file header at {}:{}: expected '<count> variables'",
                paramfile_path.display(),
                header_line
            ));
        }
        let variable_count = header_tokens[0].parse::<usize>().map_err(|err| {
            format!(
                "invalid variable count '{}' in Xyce wrapper parameter file {}:{}: {err}",
                header_tokens[0],
                paramfile_path.display(),
                header_line
            )
        })?;

        let mut bindings = Vec::with_capacity(variable_count);
        let mut seen = BTreeSet::new();
        for variable_index in 0..variable_count {
            let Some((line_number, line)) = lines.next() else {
                return Err(format!(
                    "Xyce wrapper parameter file {} ended before variable row {} of {}",
                    paramfile_path.display(),
                    variable_index + 1,
                    variable_count
                ));
            };
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() != 2 {
                return Err(format!(
                    "invalid Xyce wrapper parameter row at {}:{}: expected '<value> <name>'",
                    paramfile_path.display(),
                    line_number
                ));
            }
            let value =
                rspice_core::netlist::lexer::parse_spice_value(fields[0]).map_err(|err| {
                    format!(
                        "invalid Xyce wrapper parameter value '{}' at {}:{}: {err}",
                        fields[0],
                        paramfile_path.display(),
                        line_number
                    )
                })?;
            if !value.is_finite() {
                return Err(format!(
                    "non-finite Xyce wrapper parameter value '{}' at {}:{}",
                    fields[0],
                    paramfile_path.display(),
                    line_number
                ));
            }
            let name = fields[1];
            if !Self::xyce_paramfile_parameter_name_is_supported(name) {
                return Err(format!(
                    "unsupported Xyce wrapper parameter name '{}' at {}:{}",
                    name,
                    paramfile_path.display(),
                    line_number
                ));
            }
            let key = name.to_ascii_lowercase();
            if !seen.insert(key) {
                return Err(format!(
                    "duplicate Xyce wrapper parameter '{}' at {}:{}",
                    name,
                    paramfile_path.display(),
                    line_number
                ));
            }
            bindings.push((name.to_string(), value));
        }
        Ok(bindings)
    }

    pub(super) fn parse_netlist_with_expression_dialect(
        source: &str,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
    ) -> Result<Netlist, rspice_core::netlist::ParseError> {
        Self::parse_netlist_with_expression_dialect_and_execution_dir(
            source,
            deck_path,
            expression_dialect,
            None,
        )
    }

    pub(super) fn parse_netlist_with_expression_dialect_and_execution_dir(
        source: &str,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
        execution_dir: Option<&Path>,
    ) -> Result<Netlist, rspice_core::netlist::ParseError> {
        Self::parse_netlist_with_expression_dialect_policy_and_execution_dir(
            source,
            deck_path,
            expression_dialect,
            ParameterRedefinitionPolicy::UseLast,
            execution_dir,
        )
    }

    pub(super) fn parse_netlist_with_expression_dialect_policy_and_execution_dir(
        source: &str,
        deck_path: &Path,
        expression_dialect: ExpressionDialect,
        parameter_redefinition_policy: ParameterRedefinitionPolicy,
        execution_dir: Option<&Path>,
    ) -> Result<Netlist, rspice_core::netlist::ParseError> {
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect,
            // Existing static contracts retain their historical parse
            // selection. Relational families that model Xyce's
            // `-redefined_params` modes select their policy explicitly in
            // their execution plan.
            parameter_redefinition_policy,
            ..NetlistParseOptions::default()
        };
        if let Some(execution_dir) = execution_dir {
            return Netlist::parse_with_path_and_execution_dir(
                source,
                deck_path,
                execution_dir,
                options,
            );
        }

        Netlist::parse_with_path_and_options(source, deck_path, options)
    }

    pub(super) fn measure_cont_prn_table_on_reference_grid(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
        reference: &XycePrnTable,
    ) -> Result<XycePrnTable, String> {
        let layout = Self::transient_reference_layout(reference)?;
        let time_scale = Self::tran_print_time_scale_factor(&plan.source)?;
        let measurement_traces = Self::measurement_output_traces(
            netlist,
            &result.time,
            plan.print.probes.iter().map(String::as_str),
            "TRAN",
            "TRAN_CONT",
            &[],
            |trace_netlist| {
                rspice_core::analysis::evaluate_tran_continuous_measurements(trace_netlist, result)
            },
        )?;
        let mut stateful = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::stateful_tran_print_expression(probe, netlist))
            .collect::<Result<Vec<_>, _>>()?;
        let mut rows = Vec::with_capacity(reference.rows.len());
        for (row_index, reference_row) in reference.rows.iter().enumerate() {
            let printed_time = *reference_row.get(layout.time_column).ok_or_else(|| {
                format!("MEASURE_CONT PRN reference row {row_index} has no TIME value")
            })?;
            let time = printed_time / time_scale;
            let mut row = vec![row_index as Value, printed_time];
            for (probe, stateful) in plan.print.probes.iter().zip(&mut stateful) {
                let value = if let Some(trace) = measurement_traces.get(&probe.to_ascii_uppercase())
                {
                    let tolerance = Self::default_prn_time_quantization_tolerance(time);
                    trace
                        .iter()
                        .filter(|(activation_index, _)| {
                            result
                                .time
                                .get(*activation_index)
                                .is_some_and(|activation_time| *activation_time <= time + tolerance)
                        })
                        .map(|(_, value)| *value)
                        .next_back()
                        .unwrap_or(0.0)
                } else {
                    match stateful {
                        Some(runtime) => Self::evaluate_stateful_tran_print_expression(
                            runtime, netlist, result, time,
                        )?,
                        None => Self::evaluate_tran_probe(probe, netlist, result, time)?,
                    }
                };
                if !value.is_finite() {
                    return Err(format!(
                        "MEASURE_CONT PRN probe '{probe}' is non-finite at time {time}"
                    ));
                }
                row.push(value);
            }
            if row.len() != reference.columns.len() {
                return Err(format!(
                    "MEASURE_CONT PRN row width {} does not match reference width {}",
                    row.len(),
                    reference.columns.len()
                ));
            }
            rows.push(row);
        }
        Ok(XycePrnTable {
            columns: reference.columns.clone(),
            rows,
        })
    }

    pub(super) fn parse_measure_cont_gs_file(
        path: &Path,
    ) -> Result<Vec<XyceMeasureContGsRow>, String> {
        let source = fs::read_to_string(path)
            .map_err(|error| format!("failed to read GS {}: {error}", path.display()))?;
        let mut rows = Vec::new();
        for (line_index, raw_line) in source.lines().enumerate() {
            let line_number = line_index + 1;
            let line = raw_line.trim();
            let Some((raw_name, raw_fields)) = line.split_once('=') else {
                continue;
            };
            let name = raw_name.trim();
            if name.is_empty()
                || name.chars().any(char::is_whitespace)
                || name.eq_ignore_ascii_case("targ")
                || name.eq_ignore_ascii_case("trig")
            {
                continue;
            }
            let fields = raw_fields.split_whitespace().collect::<Vec<_>>();
            if fields.is_empty() {
                return Err(format!(
                    "{}:{line_number}: empty GS measurement row",
                    path.display()
                ));
            }
            let failed = fields.contains(&"FAILED");
            let value = if failed {
                XyceMeasurementReferenceValue::Failed
            } else {
                Self::parse_measurement_reference_token(path, line_number, fields[0])?
            };
            let metadata = |label: &str| -> Result<Option<XyceMeasurementReferenceValue>, String> {
                let Some(index) = fields
                    .iter()
                    .position(|field| field.eq_ignore_ascii_case(label))
                else {
                    return Ok(None);
                };
                if fields.get(index + 1) != Some(&"=") {
                    return Err(format!(
                        "{}:{line_number}: GS {label} metadata has no '='",
                        path.display()
                    ));
                }
                let Some(raw) = fields.get(index + 2) else {
                    return Err(format!(
                        "{}:{line_number}: GS {label} metadata has no value",
                        path.display()
                    ));
                };
                if raw.eq_ignore_ascii_case("not")
                    && fields
                        .get(index + 3)
                        .is_some_and(|field| field.eq_ignore_ascii_case("found"))
                {
                    return Ok(Some(XyceMeasurementReferenceValue::Failed));
                }
                Self::parse_measurement_reference_token(path, line_number, raw).map(Some)
            };
            let trigger_axis = metadata("trig")?;
            let target_axis = metadata("targ")?;
            if trigger_axis.is_some() != target_axis.is_some() {
                return Err(format!(
                    "{}:{line_number}: GS trigger/target metadata is incomplete",
                    path.display()
                ));
            }
            let event_axis = fields
                .windows(3)
                .find(|window| {
                    (window[0].eq_ignore_ascii_case("time")
                        || window[0].eq_ignore_ascii_case("AT")
                        || window[0].eq_ignore_ascii_case("freq"))
                        && window[1] == "="
                })
                .map(|window| Self::parse_measurement_reference_token(path, line_number, window[2]))
                .transpose()?;
            rows.push(XyceMeasureContGsRow {
                mixed: XyceMixedMeasurementReferenceRow {
                    name: name.to_string(),
                    value,
                    trigger_axis,
                    target_axis,
                },
                event_axis,
            });
        }
        if rows.is_empty() {
            Err(format!(
                "{} contains no GS measurement rows",
                path.display()
            ))
        } else {
            Ok(rows)
        }
    }

    pub(super) fn parse_connectivity_diagnostic_reference(
        path: &Path,
    ) -> Result<Option<rspice_core::netlist::ConnectivityDiagnostics>, String> {
        const PREFIX: &str = "User warning: Voltage Node (";
        const ONE_TERMINAL: &str = "connected to only 1 device Terminal";
        const NO_DC_PATH: &str = "does not have a DC path to ground";

        let source = fs::read_to_string(path).map_err(|error| {
            format!(
                "failed to read topology diagnostic reference {}: {error}",
                path.display()
            )
        })?;
        let lines = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() || lines.iter().any(|line| !line.starts_with(PREFIX)) {
            return Ok(None);
        }

        let mut one_device_terminal_nodes = BTreeSet::new();
        let mut no_dc_path_nodes = BTreeSet::new();
        for line in lines {
            let remainder = line
                .strip_prefix(PREFIX)
                .expect("prefix was checked before parsing");
            let (node, message) = remainder
                .split_once(") ")
                .ok_or_else(|| format!("malformed topology diagnostic reference line '{line}'"))?;
            if node.is_empty() {
                return Err("topology diagnostic reference contains an empty node".to_string());
            }
            match message {
                ONE_TERMINAL => {
                    if !one_device_terminal_nodes.insert(node.to_string()) {
                        return Err(format!(
                            "duplicate one-device topology diagnostic for node '{node}'"
                        ));
                    }
                }
                NO_DC_PATH => {
                    if !no_dc_path_nodes.insert(node.to_string()) {
                        return Err(format!(
                            "duplicate no-DC-path topology diagnostic for node '{node}'"
                        ));
                    }
                }
                _ => return Ok(None),
            }
        }

        Ok(Some(rspice_core::netlist::ConnectivityDiagnostics {
            one_device_terminal_nodes: one_device_terminal_nodes.into_iter().collect(),
            no_dc_path_nodes: no_dc_path_nodes.into_iter().collect(),
        }))
    }

    pub(super) fn has_static_tran_reference_oracle(&self, deck_path: &Path) -> bool {
        [
            XyceStaticTranContract::WrapperStatic.reference_extension(),
            XyceStaticTranContract::WrapperCsv.reference_extension(),
            XyceStaticTranContract::WrapperCsd.reference_extension(),
        ]
        .into_iter()
        .any(|extension| {
            self.static_output_reference_path(deck_path, extension)
                .is_some_and(|path| path.is_file())
        })
    }

    pub(super) fn parse_xyce_sensitivity_objectives(
        value: &str,
    ) -> Result<Vec<XyceAcSensitivityObjective>, String> {
        let mut objectives = Vec::new();
        for raw in Self::split_xyce_sensitivity_list(value)? {
            let authored_name = raw.trim();
            if authored_name.is_empty() {
                return Err("Xyce .SENS OBJVARS contains an empty objective".to_string());
            }
            let normalized = Self::normalize_probe(authored_name);
            let spec = if let Some(inner) = normalized
                .strip_prefix("v(")
                .and_then(|inner| inner.strip_suffix(')'))
            {
                let (positive, negative) = if let Some((positive, negative)) = inner.split_once(',')
                {
                    (positive.trim(), Some(negative.trim()))
                } else {
                    (inner.trim(), None)
                };
                if positive.is_empty() || negative.is_some_and(str::is_empty) {
                    return Err(format!(
                        "Xyce .SENS voltage objective '{authored_name}' has an empty node"
                    ));
                }
                XyceAcSensitivityObjectiveSpec::Voltage {
                    positive: positive.to_string(),
                    negative: negative.map(str::to_string),
                }
            } else if let Some(element) = normalized
                .strip_prefix("i(")
                .and_then(|inner| inner.strip_suffix(')'))
            {
                if element.is_empty() {
                    return Err(format!(
                        "Xyce .SENS current objective '{authored_name}' has an empty element"
                    ));
                }
                XyceAcSensitivityObjectiveSpec::BranchCurrent(element.to_string())
            } else {
                if normalized.is_empty() || normalized.contains(['(', ')']) {
                    return Err(format!(
                        "Xyce .SENS OBJVARS objective '{authored_name}' is not a node or probe"
                    ));
                }
                XyceAcSensitivityObjectiveSpec::Voltage {
                    positive: normalized.clone(),
                    negative: None,
                }
            };
            if objectives
                .iter()
                .any(|existing: &XyceAcSensitivityObjective| {
                    existing.authored_name.eq_ignore_ascii_case(authored_name)
                })
            {
                return Err(format!(
                    "Xyce .SENS OBJVARS contains duplicate objective '{authored_name}'"
                ));
            }
            objectives.push(XyceAcSensitivityObjective {
                authored_name: authored_name.to_string(),
                spec,
            });
        }
        if objectives.is_empty() {
            return Err("Xyce .SENS OBJVARS contains no objectives".to_string());
        }
        Ok(objectives)
    }

    pub(super) fn parse_xyce_sensitivity_parameters(value: &str) -> Result<Vec<String>, String> {
        let mut parameters = Vec::new();
        let fields = Self::split_xyce_sensitivity_list(value)?;
        for (index, raw) in fields.iter().enumerate() {
            // Xyce permits a comma at the end of a continued PARAM line
            // before the next '+' line.  The logical-line normalizer keeps
            // that delimiter attached to the preceding token, so remove
            // only trailing commas here; commas inside grouped parameter
            // syntax are still handled by split_xyce_sensitivity_list.
            let parameter = raw.trim().trim_end_matches(',').trim();
            if parameter.is_empty() {
                if index + 1 == fields.len() && value.trim_end().ends_with(',') {
                    continue;
                }
                return Err("Xyce .SENS PARAM contains an empty parameter".to_string());
            }
            if parameter.contains(['(', ')', '{', '}', '=']) {
                return Err(format!(
                    "Xyce .SENS PARAM contains malformed parameter '{parameter}'"
                ));
            }
            if parameters
                .iter()
                .any(|existing: &String| existing.eq_ignore_ascii_case(parameter))
            {
                return Err(format!(
                    "Xyce .SENS PARAM contains duplicate parameter '{parameter}'"
                ));
            }
            parameters.push(parameter.to_string());
        }
        if parameters.is_empty() {
            return Err("Xyce .SENS PARAM contains no parameters".to_string());
        }
        Ok(parameters)
    }

    pub(super) fn parse_xyce_sensitivity_flags(source: &str) -> Result<(bool, bool), String> {
        let mut direct = None;
        let mut adjoint = None;
        for line in Self::logical_netlist_lines(source) {
            let tokens = Self::split_grouped_whitespace_fields(&line, ".OPTIONS statement")?;
            let Some(command) = tokens.first() else {
                continue;
            };
            if !command.eq_ignore_ascii_case(".options") {
                continue;
            }
            let Some(sensitivity_index) = tokens
                .iter()
                .position(|token| token.eq_ignore_ascii_case("sensitivity"))
            else {
                continue;
            };
            let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
            let mut index = sensitivity_index + 1;
            while index < token_refs.len() {
                let Some((raw_key, raw_value, consumed)) =
                    Self::print_option_assignment(&token_refs, index)
                else {
                    index += 1;
                    continue;
                };
                let key = raw_key.trim().to_ascii_lowercase();
                if !matches!(key.as_str(), "direct" | "adjoint") {
                    index += consumed;
                    continue;
                }
                let value = rspice_core::netlist::lexer::parse_spice_value(raw_value.trim())
                    .map_err(|error| {
                        format!(
                            "Xyce .OPTIONS SENSITIVITY {key} must be numeric: {raw_value}: {error}"
                        )
                    })?;
                if !value.is_finite() || (value != 0.0 && value != 1.0) {
                    return Err(format!(
                        "Xyce .OPTIONS SENSITIVITY {key} must be exactly 0 or 1, got {value}"
                    ));
                }
                let destination = if key == "direct" {
                    &mut direct
                } else {
                    &mut adjoint
                };
                if destination.is_some() {
                    return Err(format!(
                        "Xyce .OPTIONS SENSITIVITY contains duplicate {key}"
                    ));
                }
                *destination = Some(value == 1.0);
                index += consumed;
            }
        }
        let direct = direct.ok_or_else(|| {
            "native Xyce AC sensitivity contract requires explicit DIRECT=0|1".to_string()
        })?;
        let adjoint = adjoint.ok_or_else(|| {
            "native Xyce AC sensitivity contract requires explicit ADJOINT=0|1".to_string()
        })?;
        if !direct && !adjoint {
            return Err(
                "native Xyce AC sensitivity contract requires DIRECT or ADJOINT output".to_string(),
            );
        }
        Ok((direct, adjoint))
    }

    pub(super) fn static_noise_side_references(
        &self,
        source: &str,
        deck_path: &Path,
    ) -> Result<Vec<XyceStaticNoiseSideReference>, String> {
        let requests = Self::aggregate_print_output_requests(
            Self::print_output_requests(source, "NOISE")?,
            "NOISE",
        )?;
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        let mut references = Vec::new();
        for request in requests {
            let Some(file) = request.file.clone() else {
                continue;
            };
            let reference_path = Self::side_output_reference_candidate(&anchor, &file)?;
            // The vendored oracle defines the comparison surface. Some Xyce
            // fallback-format tests intentionally retain only the shared
            // primary PRN even though their deck declares additional FILE=
            // destinations. Preserve the already-validated declaration
            // schema, and compare every side artifact that OutputData retains.
            if !reference_path.is_file() {
                continue;
            }
            let contract = XyceStaticNoiseContract::for_format(request.format.as_deref())?;
            references.push(XyceStaticNoiseSideReference {
                file,
                print: XycePrintRequest {
                    probes: request.probes,
                },
                reference_path,
                contract,
            });
        }
        Ok(references)
    }

    pub(super) fn parse_xyce_dc_sensitivity_objectives(
        value: &str,
    ) -> Result<Vec<XyceAcSensitivityObjective>, String> {
        let mut objectives = Vec::new();
        for raw in Self::split_xyce_sensitivity_list(value)? {
            let mut objective = raw.trim();
            if objective.starts_with('{') && objective.ends_with('}') {
                objective = &objective[1..objective.len() - 1];
            }
            if objective.trim().is_empty() {
                return Err("Xyce .SENS OBJFUNC contains an empty objective".to_string());
            }
            let parsed = Self::parse_xyce_sensitivity_objectives(objective)?;
            objectives.extend(parsed);
        }
        let mut seen = BTreeSet::new();
        for objective in &objectives {
            if !seen.insert(Self::normalize_probe(&objective.authored_name)) {
                return Err(format!(
                    "Xyce .SENS OBJFUNC contains duplicate objective '{}'",
                    objective.authored_name
                ));
            }
        }
        if objectives.is_empty() {
            return Err("Xyce .SENS OBJFUNC contains no objectives".to_string());
        }
        Ok(objectives)
    }

    pub(super) fn is_native_default_prn_wrapper_candidate_path(relative_path: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/")
    }

    pub(super) fn is_native_file_only_prn_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        if !Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/") {
            return false;
        }
        Self::dc_print_output_requests(source).is_ok_and(|requests| {
            !requests.is_empty()
                && requests.iter().all(|request| {
                    request.file.is_some()
                        && Self::dc_print_format_is_prn_compatible(
                            request.format.as_deref().unwrap_or("STD"),
                        )
                })
        })
    }

    pub(super) fn is_native_multiplicity_static_prn_wrapper_candidate_path(
        relative_path: &str,
    ) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/multiplicity_factor/")
    }

    pub(super) fn is_native_plain_static_dc_prn_wrapper_candidate(
        deck_path: &Path,
        source: &str,
    ) -> bool {
        if Self::validate_plain_static_dc_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };
        let Ok(netlist) = Self::parse_xyce_netlist(source, deck_path) else {
            return false;
        };
        if Self::validate_plain_static_dc_prn_wrapper_netlist(&netlist).is_err() {
            return false;
        }
        let Ok(dc) = Self::single_dc_sweep(&netlist) else {
            return false;
        };
        Self::validate_static_dc_contract(&netlist, &dc, &print).is_ok()
    }

    pub(super) fn is_native_default_prn_tran_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        matches!(
            Self::normalize_manifest_key(relative_path).as_str(),
            "netlists/output/tran/op-prn.cir"
                | "netlists/output/tran/tran-gnuplot.cir"
                | "netlists/output/tran/tran-prn-comma.cir"
                | "netlists/output/tran/tran-prn.cir"
                | "netlists/output/tran/tran-prn-filter.cir"
                | "netlists/output/tran/tran-prn-noindex.cir"
                | "netlists/output/tran/tran-prn-precision.cir"
                | "netlists/output/tran/tran-prn-timescalefactor.cir"
                | "netlists/output/tran/tran-prn-width.cir"
                | "netlists/output/tran/tran-splot.cir"
                | "netlists/output/tran/tran-touchstone-defaults-to-prn.cir"
        ) && Self::validate_native_static_prn_tran_wrapper_contract(source).is_ok()
    }

    pub(super) fn is_native_output_other_prn_tran_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/other/")
            && Self::validate_native_static_prn_tran_wrapper_contract(source).is_ok()
    }

    pub(super) fn is_native_generic_static_prn_tran_wrapper_candidate(
        relative_path: &str,
        source: &str,
        has_prn_oracle: bool,
    ) -> bool {
        let normalized_path = Self::normalize_manifest_key(relative_path);
        if normalized_path.starts_with("netlists/output/") {
            return false;
        }
        Self::validate_native_static_prn_tran_wrapper_contract(source).is_ok()
            || (has_prn_oracle
                && Self::validate_native_static_prn_tran_wrapper_contract_with_format_mode(
                    source, true,
                )
                .is_ok())
    }

    pub(super) fn dc_print_format_is_prn_compatible(format: &str) -> bool {
        matches!(
            format.to_ascii_lowercase().as_str(),
            "std" | "tecplot" | "touchstone" | "touchstone2" | "noindex" | "gnuplot" | "splot"
        )
    }

    pub(super) fn tran_print_format_is_prn_compatible(format: &str) -> bool {
        matches!(
            format.to_ascii_lowercase().as_str(),
            "std" | "tecplot" | "touchstone" | "touchstone2" | "noindex" | "gnuplot" | "splot"
        )
    }

    pub(super) fn ac_print_format_is_prn_compatible(format: &str) -> bool {
        matches!(
            format.to_ascii_lowercase().as_str(),
            "std" | "tecplot" | "touchstone" | "touchstone2" | "noindex" | "gnuplot" | "splot"
        )
    }

    pub(super) fn ac_initial_condition_reference_extension(
        format: Option<&str>,
    ) -> Result<&'static str, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::ac_print_format_is_prn_compatible(normalized) {
            return Ok("TD.prn");
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok("TD.csv");
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok("TD.csd");
        }
        Err(format!(
            "native AC_IC comparison does not cover FORMAT={normalized}"
        ))
    }

    pub(super) fn ac_initial_condition_reference_path(
        &self,
        plan: &XyceStaticAcPlan,
        request: &XycePrintOutputRequest,
    ) -> Result<PathBuf, String> {
        if request.file.as_deref() == plan.primary_ac_ic_file.as_deref() {
            let extension =
                Self::ac_initial_condition_reference_extension(request.format.as_deref())?;
            let path = self
                .static_output_reference_path(&plan.deck_path, extension)
                .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
            if !path.is_file() {
                return Err(format!(
                    "missing checked-in AC_IC oracle {}",
                    self.display_path(&path)
                ));
            }
            return Ok(path);
        }
        if let Some(file) = request.file.as_deref() {
            return self.side_output_reference_path_for_deck(&plan.deck_path, file);
        }

        let extension = Self::ac_initial_condition_reference_extension(request.format.as_deref())?;
        let path = self
            .static_output_reference_path(&plan.deck_path, extension)
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        if !path.is_file() {
            return Err(format!(
                "missing checked-in AC_IC oracle {}",
                self.display_path(&path)
            ));
        }
        Ok(path)
    }

    pub(super) fn parse_ac_initial_condition_reference_file(
        request: &XycePrintOutputRequest,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match request.format.as_deref().unwrap_or("STD").trim() {
            format if format.eq_ignore_ascii_case("CSV") => Self::parse_csv_file(path),
            format if format.eq_ignore_ascii_case("PROBE") => Self::parse_tran_csd_file(path),
            _ => Self::parse_prn_file(path),
        }
    }

    pub(super) fn split_transient_step_reference(
        reference: &XycePrnTable,
        expected_steps: usize,
    ) -> Result<Vec<XycePrnTable>, String> {
        if expected_steps == 0 {
            return Err(".STEP expansion produced no runs".to_string());
        }
        if expected_steps == 1 {
            return Ok(vec![reference.clone()]);
        }
        let time_column = Self::reference_time_column_index(reference)
            .ok_or_else(|| "stepped transient reference table has no TIME column".to_string())?;
        let mut starts = vec![0usize];
        let mut previous_time = None;
        for (row_index, row) in reference.rows.iter().enumerate() {
            let time = *row.get(time_column).ok_or_else(|| {
                format!("row {row_index} has no TIME column at index {time_column}")
            })?;
            if !time.is_finite() {
                return Err(format!("row {row_index} has non-finite TIME value {time}"));
            }
            if let Some(previous) = previous_time
                && time < previous
            {
                starts.push(row_index);
            }
            previous_time = Some(time);
        }
        starts.push(reference.rows.len());
        let actual_steps = starts.len().saturating_sub(1);
        if actual_steps != expected_steps {
            return Err(format!(
                "reference contains {actual_steps} transient step table(s), but .STEP expansion produced {expected_steps} run(s)"
            ));
        }

        let mut references = Vec::with_capacity(expected_steps);
        for range in starts.windows(2) {
            let start = range[0];
            let end = range[1];
            if start == end {
                return Err("stepped transient reference contains an empty step table".to_string());
            }
            let reference = XycePrnTable {
                columns: reference.columns.clone(),
                rows: reference.rows[start..end].to_vec(),
            };
            Self::validate_transient_stepnum_column(&reference, references.len())?;
            references.push(reference);
        }
        Ok(references)
    }

    pub(super) fn split_ac_step_reference(
        reference: &XycePrnTable,
        expected_steps: usize,
        points_per_step: usize,
    ) -> Result<Vec<XycePrnTable>, String> {
        if expected_steps == 0 {
            return Err(".STEP expansion produced no runs".to_string());
        }
        if points_per_step == 0 {
            return Err("AC analysis produced no frequency points".to_string());
        }
        if expected_steps == 1 {
            return Ok(vec![reference.clone()]);
        }
        let expected_rows = expected_steps
            .checked_mul(points_per_step)
            .ok_or_else(|| "stepped AC row count overflow".to_string())?;
        if reference.rows.len() != expected_rows {
            return Err(format!(
                "reference contains {} AC row(s), but .STEP expansion produced {expected_steps} run(s) with {points_per_step} frequency point(s) each",
                reference.rows.len()
            ));
        }

        let mut references = Vec::with_capacity(expected_steps);
        for step_index in 0..expected_steps {
            let start = step_index * points_per_step;
            let end = start + points_per_step;
            references.push(XycePrnTable {
                columns: reference.columns.clone(),
                rows: reference.rows[start..end].to_vec(),
            });
        }
        Ok(references)
    }

    pub(super) fn prn_compatible_side_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Ok(Self::dc_print_output_requests(source)?
            .into_iter()
            .filter(|request| {
                request.file.is_some()
                    && Self::dc_print_format_is_prn_compatible(
                        request.format.as_deref().unwrap_or("STD"),
                    )
            })
            .collect())
    }

    pub(super) fn prn_compatible_tran_side_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Ok(Self::aggregate_print_output_requests(
            Self::print_output_requests(source, "TRAN")?,
            "TRAN",
        )?
        .into_iter()
        .filter(|request| {
            request.file.is_some()
                && Self::tran_print_format_is_prn_compatible(
                    request.format.as_deref().unwrap_or("STD"),
                )
        })
        .collect())
    }

    pub(super) fn prn_compatible_ac_side_output_requests(
        source: &str,
    ) -> Result<Vec<XycePrintOutputRequest>, String> {
        Ok(
            Self::aggregate_print_output_requests(
                Self::print_output_requests(source, "AC")?,
                "AC",
            )?
            .into_iter()
            .filter(|request| {
                request.file.is_some()
                    && Self::ac_print_format_is_prn_compatible(
                        request.format.as_deref().unwrap_or("STD"),
                    )
            })
            .collect(),
        )
    }

    pub(super) fn side_output_reference_path(
        reference_path: &Path,
        file: &str,
    ) -> Result<PathBuf, String> {
        let candidate = Self::side_output_reference_candidate(reference_path, file)?;
        if !candidate.is_file() {
            return Err(format!(
                "missing checked-in side-output oracle {}",
                candidate.display()
            ));
        }
        Ok(candidate)
    }

    pub(super) fn side_output_reference_path_for_deck(
        &self,
        deck_path: &Path,
        file: &str,
    ) -> Result<PathBuf, String> {
        let anchor = self
            .static_output_reference_path(deck_path, "anchor")
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        Self::side_output_reference_path(&anchor, file)
    }

    pub(super) fn ac_side_output_reference_path(
        reference_path: &Path,
        request: &XycePrintOutputRequest,
        file: &str,
    ) -> Result<PathBuf, String> {
        let candidate = Self::side_output_reference_candidate(reference_path, file)?;
        if candidate.is_file() {
            return Ok(candidate);
        }
        let format = request.format.as_deref().unwrap_or("STD");
        if format.eq_ignore_ascii_case("GNUPLOT") || format.eq_ignore_ascii_case("SPLOT") {
            return Ok(reference_path.to_path_buf());
        }
        Err(format!(
            "missing checked-in side-output oracle {}",
            candidate.display()
        ))
    }

    pub(super) fn side_output_reference_candidate(
        reference_path: &Path,
        file: &str,
    ) -> Result<PathBuf, String> {
        let side_path = Path::new(file);
        if side_path.is_absolute() {
            return Err(format!(
                "absolute FILE= side-output path '{}' cannot be mapped into the vendored OutputData tree",
                file
            ));
        }
        let parent = reference_path
            .parent()
            .ok_or_else(|| "primary reference path has no parent directory".to_string())?;
        Ok(parent.join(side_path))
    }

    pub(super) fn analytic_rc_reference_table(
        actual: &XycePrnTable,
        specification: &XyceAnalyticRcSpecification,
    ) -> Result<XycePrnTable, String> {
        const LABEL: &str = "analytic first-order RC";
        if !specification.source_value.is_finite()
            || !specification.initial_voltage.is_finite()
            || !specification.resistance.is_finite()
            || specification.resistance <= 0.0
            || !specification.capacitance.is_finite()
            || specification.capacitance <= 0.0
            || !specification.time_constant.is_finite()
            || specification.time_constant <= 0.0
            || (specification.resistance * specification.capacitance).to_bits()
                != specification.time_constant.to_bits()
        {
            return Err(format!(
                "{LABEL} specification is nonfinite or inconsistent"
            ));
        }
        if actual.columns.len() != 3
            || actual.columns[0] != "Index"
            || actual.columns[1] != "TIME"
            || Self::normalize_probe(&actual.columns[2])
                != format!("v({})", specification.output_node).to_ascii_lowercase()
        {
            return Err(format!(
                "{LABEL} generator requires Index, TIME, and V(output), got {:?}",
                actual.columns
            ));
        }
        if actual.rows.is_empty() {
            return Err(format!("{LABEL} simulator output contains no rows"));
        }

        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != actual.columns.len()
                || row[0].to_bits() != (row_index as Value).to_bits()
                || row.iter().any(|value| !value.is_finite())
            {
                return Err(format!(
                    "{LABEL} simulator row {row_index} is malformed, nonfinite, or has a noncanonical index"
                ));
            }
            // Release 7.10's Perl generator consumes the TIME token from the
            // already-written default PRN, not the simulator's raw f64 time.
            let printed_time = Self::xyce_default_prn_roundtrip(row[1])?;
            if printed_time < 0.0 {
                return Err(format!(
                    "{LABEL} simulator row {row_index} has negative printed time {printed_time}"
                ));
            }
            let decay = (-printed_time / specification.time_constant).exp();
            let analytic_value = specification.source_value
                + (specification.initial_voltage - specification.source_value) * decay;
            if !analytic_value.is_finite() {
                return Err(format!(
                    "{LABEL} produced nonfinite value at printed time {printed_time}"
                ));
            }
            rows.push(vec![row[0], printed_time, analytic_value]);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn bug647_default_prn_token_lines(
        table: &XycePrnTable,
    ) -> Result<Vec<Vec<String>>, String> {
        if table.columns.len() != 7 || table.rows.len() != 1_620 {
            return Err(
                "BUG 647 default stepped PRN requires seven columns and 1620 data rows".into(),
            );
        }
        let mut lines = Vec::with_capacity(1_622);
        lines.push(table.columns.clone());
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != 7 {
                return Err(format!("BUG 647 PRN row {row_index} is not seven fields"));
            }
            lines.push(
                row.iter()
                    .map(|value| Self::xyce_default_prn_text(*value))
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        lines.push(
            ["End", "of", "Xyce(TM)", "Parameter", "Sweep"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        );
        Ok(lines)
    }

    pub(super) fn bug655_default_prn_token_lines(
        table: &XycePrnTable,
    ) -> Result<Vec<Vec<String>>, String> {
        let expected_columns = ["Index", "I(I1)", "V(3)"];
        if table.columns.len() != expected_columns.len()
            || !table
                .columns
                .iter()
                .zip(expected_columns)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != 21
        {
            return Err(format!(
                "BUG 655 default PRN requires columns {expected_columns:?} and 21 rows, got {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut lines = Vec::with_capacity(23);
        lines.push(table.columns.clone());
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != 3 || row[0].to_bits() != (row_index as Value).to_bits() {
                return Err(format!(
                    "BUG 655 default PRN row {row_index} has a noncanonical Index or field count"
                ));
            }
            lines.push(
                row.iter()
                    .map(|value| Self::xyce_default_prn_text(*value))
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        lines.push(
            ["End", "of", "Xyce(TM)", "Simulation"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        );
        Ok(lines)
    }

    pub(super) fn bug662_default_prn_token_lines(
        table: &XycePrnTable,
    ) -> Result<Vec<Vec<String>>, String> {
        let expected_columns = ["Index", "TIME", "V(N14950)", "V(N15037)"];
        if table.columns != expected_columns || table.rows.len() < 2 {
            return Err(format!(
                "BUG 662 default PRN requires columns {expected_columns:?} and at least two transient rows, got {:?} and {} row(s)",
                table.columns,
                table.rows.len()
            ));
        }
        let mut lines = Vec::with_capacity(table.rows.len() + 2);
        lines.push(table.columns.clone());
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != expected_columns.len() {
                return Err(format!(
                    "BUG 662 default PRN row {row_index} has {} fields instead of {}",
                    row.len(),
                    expected_columns.len()
                ));
            }
            if row[0].to_bits() != (row_index as Value).to_bits() {
                return Err(format!(
                    "BUG 662 default PRN row {row_index} does not preserve the canonical Index sequence"
                ));
            }
            lines.push(
                row.iter()
                    .map(|value| Self::xyce_default_prn_text(*value))
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        lines.push(
            ["End", "of", "Xyce(TM)", "Simulation"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        );
        Ok(lines)
    }

    pub(super) fn bug754_default_prn_serialization_fingerprint(
        table: &XycePrnTable,
    ) -> Result<Vec<Vec<String>>, String> {
        const COLUMNS: [&str; 4] = ["Index", "v(drain)", "v(gate)", "I(vdrain)"];
        if table.columns != COLUMNS || table.rows.len() != 1_001 {
            return Err(format!(
                "BUG 754 default PRN requires exact columns {COLUMNS:?} and 1001 rows, got {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        // Release 7.10's default writer is deterministic: one fixed header
        // layout, one fixed whitespace layout per four-field row, precision-8
        // scientific formatting for every numeric token, and one fixed
        // simulation footer. Once exact columns, row count, Index sequence,
        // and every formatted token are equal, bytewise PRN equality follows;
        // there is no table-dependent separator or metadata left to vary.
        let mut lines = Vec::with_capacity(1_003);
        lines.push(table.columns.clone());
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != COLUMNS.len() || row[0].to_bits() != (row_index as Value).to_bits() {
                return Err(format!(
                    "BUG 754 default PRN row {row_index} has a noncanonical Index or field count"
                ));
            }
            lines.push(
                row.iter()
                    .map(|value| Self::xyce_default_prn_text(*value))
                    .collect::<Result<Vec<_>, _>>()?,
            );
        }
        lines.push(
            ["End", "of", "Xyce(TM)", "Simulation"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        );
        Ok(lines)
    }

    pub(super) fn analytic_sinusoidal_rc_reference_table(
        actual: &XycePrnTable,
        specification: &XyceAnalyticSinusoidalRcSpecification,
    ) -> Result<XycePrnTable, String> {
        const LABEL: &str = "analytic sinusoidal first-order RC";
        let (expression_node, expression_offset) =
            Self::analytic_sinusoidal_rc_print_expression(&specification.print_expression)?;
        if !specification.resistance.is_finite()
            || specification.resistance <= 0.0
            || !specification.capacitance.is_finite()
            || specification.capacitance <= 0.0
            || !specification.source_frequency.is_finite()
            || specification.source_frequency <= 0.0
            || !specification.print_offset.is_finite()
            || specification.resistance.to_bits()
                != XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_RESISTANCE.to_bits()
            || specification.capacitance.to_bits()
                != XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_CAPACITANCE.to_bits()
            || specification.source_frequency.to_bits()
                != XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_FREQUENCY.to_bits()
            || specification.print_offset.to_bits()
                != XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_PRINT_OFFSET.to_bits()
            || expression_node != specification.output_node
            || expression_offset.to_bits() != specification.print_offset.to_bits()
        {
            return Err(format!(
                "{LABEL} specification is nonfinite or differs from the fixed Release 7.10 generator"
            ));
        }
        if actual.columns.len() != 3
            || actual.columns[0] != "Index"
            || actual.columns[1] != "TIME"
            || Self::normalize_probe(&actual.columns[2])
                != Self::normalize_probe(&specification.print_expression)
        {
            return Err(format!(
                "{LABEL} generator requires Index, TIME, and the qualified expression, got {:?}",
                actual.columns
            ));
        }
        if actual.rows.is_empty() {
            return Err(format!("{LABEL} simulator output contains no rows"));
        }

        let a = -1.0 / (specification.resistance * specification.capacitance);
        let s = 2.0 * XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_PI * specification.source_frequency;
        let denominator = a * a + s * s;
        let initial_coefficient = -a * s / denominator;
        if !a.is_finite()
            || !s.is_finite()
            || !denominator.is_finite()
            || denominator <= 0.0
            || !initial_coefficient.is_finite()
        {
            return Err(format!("{LABEL} generator coefficients are invalid"));
        }

        let mut rows = Vec::with_capacity(actual.rows.len());
        for (row_index, row) in actual.rows.iter().enumerate() {
            if row.len() != actual.columns.len()
                || row[0].to_bits() != (row_index as Value).to_bits()
                || row.iter().any(|value| !value.is_finite())
            {
                return Err(format!(
                    "{LABEL} simulator row {row_index} is malformed, nonfinite, or has a noncanonical index"
                ));
            }
            // The Perl sidecar consumes the TIME token from the simulator's
            // already-written default PRN before evaluating its fixed formula.
            let printed_time = Self::xyce_default_prn_roundtrip(row[1])?;
            if printed_time < 0.0 {
                return Err(format!(
                    "{LABEL} simulator row {row_index} has negative printed time {printed_time}"
                ));
            }
            let analytic_value = specification.print_offset
                + initial_coefficient * (a * printed_time).exp()
                + a * (a * (s * printed_time).sin() + s * (s * printed_time).cos()) / denominator;
            if !analytic_value.is_finite() {
                return Err(format!(
                    "{LABEL} produced nonfinite value at printed time {printed_time}"
                ));
            }
            rows.push(vec![row[0], printed_time, analytic_value]);
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn ac_family_result_to_prn_table(
        print: &XycePrintRequest,
        netlist: &Netlist,
        results: &[AcResult],
    ) -> Result<XycePrnTable, String> {
        let first = results
            .first()
            .ok_or_else(|| "relational AC simulation produced no points".to_string())?;
        let mut expansions = Vec::with_capacity(print.probes.len());
        let mut columns = vec!["Index".to_string(), "FREQ".to_string()];
        for probe in &print.probes {
            let normalized = Self::normalize_ac_expression_probe_key(probe);
            let is_direct_complex = Self::parse_ac_voltage_probe(&normalized)
                .is_some_and(|probe| probe.accessor == XyceVoltageAccessor::Value)
                || Self::parse_ac_current_probe(&normalized)
                    .is_some_and(|probe| probe.accessor == XyceCurrentAccessor::Value);
            if is_direct_complex {
                let value = Self::evaluate_ac_complex_probe(probe, netlist, first)?;
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(format!(
                        "relational AC probe '{probe}' produced a non-finite complex value"
                    ));
                }
                expansions.push(XyceAcCsdColumnExpansion::Complex);
                columns.push(format!("Re({probe})"));
                columns.push(format!("Im({probe})"));
                continue;
            }
            match Self::evaluate_ac_probe(probe, netlist, first, false) {
                Ok(value) if value.is_finite() => {
                    expansions.push(XyceAcCsdColumnExpansion::Scalar);
                    columns.push(probe.clone());
                }
                Ok(value) => {
                    return Err(format!(
                        "relational AC probe '{probe}' produced non-finite scalar value {value}"
                    ));
                }
                Err(_) => {
                    let value = Self::evaluate_ac_complex_probe(probe, netlist, first)?;
                    if !value.re.is_finite() || !value.im.is_finite() {
                        return Err(format!(
                            "relational AC probe '{probe}' produced a non-finite complex value"
                        ));
                    }
                    expansions.push(XyceAcCsdColumnExpansion::Complex);
                    columns.push(format!("Re({probe})"));
                    columns.push(format!("Im({probe})"));
                }
            }
        }

        let mut rows = Vec::with_capacity(results.len());
        for (row_index, result) in results.iter().enumerate() {
            let mut row = Vec::with_capacity(columns.len());
            row.push(row_index as Value);
            row.push(
                Self::xyce_default_prn_roundtrip(result.frequency).map_err(|err| {
                    format!("could not serialize relational AC frequency at row {row_index}: {err}")
                })?,
            );
            for (probe, expansion) in print.probes.iter().zip(&expansions) {
                match expansion {
                    XyceAcCsdColumnExpansion::Scalar => {
                        let value = Self::evaluate_ac_probe(probe, netlist, result, false)?;
                        row.push(Self::xyce_default_prn_roundtrip(value).map_err(|err| {
                            format!(
                                "could not serialize relational AC probe '{probe}' at row {row_index}: {err}"
                            )
                        })?);
                    }
                    XyceAcCsdColumnExpansion::Complex => {
                        let value = Self::evaluate_ac_complex_probe(probe, netlist, result)?;
                        row.push(Self::xyce_default_prn_roundtrip(value.re).map_err(|err| {
                            format!(
                                "could not serialize real relational AC probe '{probe}' at row {row_index}: {err}"
                            )
                        })?);
                        row.push(Self::xyce_default_prn_roundtrip(value.im).map_err(|err| {
                            format!(
                                "could not serialize imaginary relational AC probe '{probe}' at row {row_index}: {err}"
                            )
                        })?);
                    }
                }
            }
            rows.push(row);
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn transient_family_result_to_prn_table(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<XycePrnTable, String> {
        Self::validate_transient_result_time_grid(result)?;
        let output_times = Self::xyce_verify_transient_output_times(plan, result)?;
        Self::transient_family_result_to_prn_table_at_times(plan, netlist, result, &output_times)
    }

    pub(super) fn xyce_verify_reference_time_grid(
        plan: &XyceStaticTranPlan,
        reference: &XycePrnTable,
        scientific_precision: usize,
    ) -> Result<Vec<Value>, String> {
        let layout = Self::transient_reference_layout(reference)?;
        let time_scale = Self::tran_print_time_scale_factor(&plan.source)?;
        if !time_scale.is_finite() || time_scale == 0.0 {
            return Err(format!(
                "transient print time scale must be finite and nonzero, got {time_scale}"
            ));
        }
        let mut output_times = Vec::with_capacity(reference.rows.len());
        for (row_index, row) in reference.rows.iter().enumerate() {
            let Some(&raw_printed_time) = row.get(layout.time_column) else {
                return Err(format!(
                    "Xyce integrated-RMS reference row {row_index} has no TIME column"
                ));
            };
            let serialized_time = Self::xyce_prn_scientific_roundtrip(
                raw_printed_time,
                scientific_precision,
            )
            .map_err(|err| {
                format!(
                    "could not serialize Xyce integrated-RMS reference TIME at row {row_index}: {err}"
                )
            })?;
            let printed_time = if serialized_time.abs() <= XYCE_VERIFY_DEFAULT_ZERO_TOLERANCE {
                0.0
            } else {
                serialized_time
            };
            let time = printed_time / time_scale;
            if !time.is_finite() {
                return Err(format!(
                    "Xyce integrated-RMS reference row {row_index} has invalid TIME {printed_time} after applying time scale {time_scale}"
                ));
            }
            if output_times
                .last()
                .is_some_and(|previous| *previous == time)
            {
                // Match Release 7.10 ReadDataFile: retain the first printed
                // row and discard immediately following duplicate times.
                continue;
            }
            if output_times.last().is_some_and(|previous| *previous > time) {
                return Err(format!(
                    "Xyce integrated-RMS reference times decrease at row {row_index}: {time}"
                ));
            }
            output_times.push(time);
        }
        if output_times.len() < 2 {
            return Err(format!(
                "Xyce integrated-RMS reference requires at least two output times, found {}",
                output_times.len()
            ));
        }
        Ok(output_times)
    }

    /// Serialize an adaptive native result on a checked-in reference grid for
    /// verifier diagnostics. This must never decide pass/fail: Release 7.10
    /// keeps the candidate's independently accepted grid and interpolates the
    /// reference onto it.
    #[cfg(test)]
    pub(super) fn transient_family_result_to_prn_table_on_reference_grid(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
        reference: &XycePrnTable,
        scientific_precision: usize,
    ) -> Result<XycePrnTable, String> {
        Self::validate_transient_result_time_grid(result)?;
        let output_times =
            Self::xyce_verify_reference_time_grid(plan, reference, scientific_precision)?;
        let result_first = result.time.first().copied().unwrap_or(Value::INFINITY);
        let result_last = result.time.last().copied().unwrap_or(Value::NEG_INFINITY);
        let time_tolerance = Self::default_prn_time_quantization_tolerance(
            output_times.last().copied().unwrap_or(0.0),
        );
        if output_times[0] < result_first - time_tolerance
            || output_times[output_times.len() - 1] > result_last + time_tolerance
        {
            return Err(format!(
                "native transient result [{result_first}, {result_last}] does not cover Xyce reference grid [{}, {}]",
                output_times[0],
                output_times[output_times.len() - 1]
            ));
        }
        Self::transient_family_result_to_prn_table_at_times(plan, netlist, result, &output_times)
    }

    pub(super) fn transient_family_result_to_prn_table_at_times(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        result: &TransientResult,
        output_times: &[Value],
    ) -> Result<XycePrnTable, String> {
        let time_scale = Self::tran_print_time_scale_factor(&plan.source)?;
        let columns = Self::transient_prn_header_columns(&plan.print, true);
        let mut stateful_expressions = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::stateful_tran_print_expression(probe, netlist))
            .collect::<Result<Vec<_>, _>>()?;
        let measurement_output_traces = Self::measurement_output_traces(
            netlist,
            &result.time,
            plan.print.probes.iter().map(String::as_str),
            "TRAN",
            "TRAN_CONT",
            &[],
            |trace_netlist| {
                rspice_core::analysis::evaluate_tran_continuous_measurements(trace_netlist, result)
            },
        )?;

        let mut rows = Vec::with_capacity(output_times.len());
        for (index, &time) in output_times.iter().enumerate() {
            let mut row = Vec::with_capacity(columns.len());
            row.push(index as Value);
            row.push(time * time_scale);
            for (probe, stateful) in plan.print.probes.iter().zip(&mut stateful_expressions) {
                let value = if let Some(trace) =
                    measurement_output_traces.get(&probe.to_ascii_uppercase())
                {
                    let tolerance = Self::default_prn_time_quantization_tolerance(time);
                    trace
                        .iter()
                        .filter(|(activation_index, _)| {
                            result
                                .time
                                .get(*activation_index)
                                .is_some_and(|activation_time| *activation_time <= time + tolerance)
                        })
                        .map(|(_, value)| *value)
                        .next_back()
                        .unwrap_or(0.0)
                } else {
                    match stateful {
                        Some(runtime) => Self::evaluate_stateful_tran_print_expression(
                            runtime, netlist, result, time,
                        )?,
                        None => Self::evaluate_tran_probe(probe, netlist, result, time)?,
                    }
                };
                if !value.is_finite() {
                    return Err(format!(
                        "baseline probe '{probe}' produced non-finite value {value} at time {time}"
                    ));
                }
                row.push(value);
            }
            rows.push(row);
        }

        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn transient_prn_header_columns(
        print: &XycePrintRequest,
        include_index: bool,
    ) -> Vec<String> {
        let mut columns = Vec::with_capacity(print.probes.len() + usize::from(include_index) + 1);
        if include_index {
            columns.push("Index".to_string());
        }
        columns.push("TIME".to_string());
        columns.extend(print.probes.iter().cloned());
        columns
    }

    pub(super) fn parse_expression_fingerprint(
        expression: &str,
    ) -> Result<XyceExpressionAstFingerprint, String> {
        let ast = rspice_core::netlist::expr::parse_expression(expression)
            .map_err(|err| format!("invalid delimited expression '{expression}': {err}"))?;
        Ok(Self::expression_ast_fingerprint(&ast))
    }

    pub(super) fn collect_analysis_parameter_references(
        expression: &rspice_core::netlist::expr::Expr,
        declared: &BTreeMap<String, u64>,
        references: &mut BTreeMap<String, usize>,
        directive: &str,
    ) -> Result<(), String> {
        use rspice_core::netlist::expr::{BinOpKind, Expr as NetExpr, UnaryOpKind};

        match expression {
            NetExpr::Number(value) if value.is_finite() => Ok(()),
            NetExpr::Param(name) => {
                let name = name.to_ascii_lowercase();
                if !declared.contains_key(&name) {
                    return Err(format!(
                        "undeclared or runtime identifier '{name}' in {directive} expression"
                    ));
                }
                *references.entry(name).or_default() += 1;
                Ok(())
            }
            NetExpr::BinOp {
                op:
                    BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Pow,
                left,
                right,
            } => {
                Self::collect_analysis_parameter_references(left, declared, references, directive)?;
                Self::collect_analysis_parameter_references(right, declared, references, directive)
            }
            NetExpr::UnaryOp {
                op: UnaryOpKind::Neg | UnaryOpKind::Pos,
                operand,
            } => Self::collect_analysis_parameter_references(
                operand, declared, references, directive,
            ),
            _ => Err(format!(
                "only finite real arithmetic over direct scalar parameters is admitted in {directive} expressions"
            )),
        }
    }

    pub(super) fn xyce_default_prn_roundtrip(value: Value) -> Result<Value, String> {
        Self::xyce_prn_scientific_roundtrip(value, XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION)
    }

    pub(super) fn xyce_prn_scientific_roundtrip(
        value: Value,
        scientific_precision: usize,
    ) -> Result<Value, String> {
        let printed = Self::xyce_prn_scientific_text(value, scientific_precision)?;
        printed
            .parse::<Value>()
            .map_err(|err| format!("could not parse scientific .prn value '{printed}': {err}"))
    }

    pub(super) fn xyce_default_prn_text(value: Value) -> Result<String, String> {
        Self::xyce_prn_scientific_text(value, XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION)
    }

    pub(super) fn xyce_prn_scientific_text(
        value: Value,
        scientific_precision: usize,
    ) -> Result<String, String> {
        if !value.is_finite() {
            return Err(format!("scientific .prn output cannot serialize {value}"));
        }
        if !(1..=XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION).contains(&scientific_precision) {
            return Err(format!(
                "scientific .prn precision must be between 1 and {XYCE_MAX_IEEE754_PRN_SCIENTIFIC_PRECISION}, got {scientific_precision}"
            ));
        }
        Ok(format!(
            "{value:.precision$e}",
            precision = scientific_precision
        ))
    }

    pub(super) fn dc_results_to_prn_table(
        &self,
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
        results: &[DcSweepPointResult],
    ) -> Result<XycePrnTable, String> {
        if !plan.steps.is_empty() {
            return Err(".STEP static DC results require the stepped .prn contract".to_string());
        }
        let mut columns = Vec::with_capacity(plan.print.probes.len() + 1);
        columns.push("Index".to_string());
        columns.extend(plan.print.probes.iter().cloned());

        let primary_points = plan.dc.primary_spec().points();
        if primary_points.is_empty() {
            return Err("primary DC sweep has no points".to_string());
        }
        let secondary_points = plan.dc.sweep2.as_ref().map(|sweep| sweep.spec().points());
        if secondary_points.as_ref().is_some_and(Vec::is_empty) {
            return Err("secondary DC sweep has no points".to_string());
        }

        let mut rows = Vec::with_capacity(results.len());
        for (row_index, point) in results.iter().enumerate() {
            let sweep_point = XyceDcSweepPoint {
                primary: point.sweep_value,
                secondary: if let Some(points) = secondary_points.as_ref() {
                    let outer_index = row_index / primary_points.len();
                    Some(*points.get(outer_index).ok_or_else(|| {
                        format!(
                            "row {row_index} maps outside secondary DC sweep point count ({})",
                            points.len()
                        )
                    })?)
                } else {
                    None
                },
            };

            let mut row = Vec::with_capacity(columns.len());
            row.push(row_index as f64);
            for probe in &plan.print.probes {
                row.push(Self::evaluate_dc_probe(
                    probe,
                    netlist,
                    &plan.dc,
                    sweep_point,
                    &point.result,
                    &point.device_op_report,
                )?);
            }
            rows.push(row);
        }

        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_headerless_ac_sensitivity_prn_file(
        plan: &XyceStaticAcSensitivityPlan,
    ) -> Result<XycePrnTable, String> {
        let content = fs::read_to_string(&plan.reference_path).map_err(|err| {
            format!(
                "failed to read headerless AC sensitivity oracle {}: {err}",
                plan.reference_path.display()
            )
        })?;
        let columns = Self::xyce_ac_sensitivity_reference_columns(plan);
        let mut rows = Vec::new();
        for (line_number, line) in content.lines().enumerate() {
            let line_number = line_number + 1;
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if line.to_ascii_lowercase().starts_with("end of xyce")
                || Self::is_prn_footer_line(line)
            {
                break;
            }
            let values = line
                .split_whitespace()
                .map(|token| {
                    Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid numeric token '{}' on headerless data line {}: {err}",
                            token, line_number
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != columns.len() {
                return Err(format!(
                    "headerless AC sensitivity data line {} has {} values, expected {}",
                    line_number,
                    values.len(),
                    columns.len()
                ));
            }
            rows.push(values);
        }
        if rows.is_empty() {
            return Err("headerless AC sensitivity oracle has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn xyce_ac_sensitivity_reference_columns(
        plan: &XyceStaticAcSensitivityPlan,
    ) -> Vec<String> {
        let mut columns = Vec::new();
        if !plan.no_index {
            columns.push("Index".to_string());
        }
        columns.push("FREQ".to_string());
        columns.extend(plan.print.probes.iter().cloned());
        for objective in &plan.objectives {
            let objective_probe = Self::xyce_sensitivity_objective_probe(&objective.spec);
            for component in ["re", "im", "mag", "ph"] {
                columns.push(Self::xyce_sensitivity_column_name(
                    component,
                    &objective_probe,
                    None,
                    None,
                ));
            }
            for mode in [plan.direct.then_some("dir"), plan.adjoint.then_some("adj")]
                .into_iter()
                .flatten()
            {
                for parameter in &plan.parameters {
                    for component in ["re", "im", "mag", "ph"] {
                        columns.push(Self::xyce_sensitivity_column_name(
                            component,
                            &objective_probe,
                            Some(parameter),
                            Some(mode),
                        ));
                    }
                }
            }
        }
        columns
    }

    pub(super) fn noise_reference_signal_probe(
        column: &XyceAcReferenceColumn,
    ) -> Result<String, String> {
        let probe = column.probe_name();
        let component_prefix = match column.component() {
            XyceAcProbeComponent::Scalar => return Ok(probe.to_string()),
            XyceAcProbeComponent::Real => 'R',
            XyceAcProbeComponent::Imaginary => 'I',
        };
        let normalized = Self::normalize_probe(probe);
        let Some(quantity) = normalized.chars().next() else {
            return Err("empty complex NOISE reference probe".to_string());
        };
        if !matches!(quantity, 'v' | 'i') || !normalized[quantity.len_utf8()..].starts_with('(') {
            return Err(format!(
                "complex NOISE reference probe '{probe}' is not a node voltage or branch current"
            ));
        }
        Ok(format!(
            "{}{component_prefix}{}",
            quantity.to_ascii_uppercase(),
            &normalized[quantity.len_utf8()..]
        ))
    }

    pub(super) fn noise_step_reference_batch(
        reference: &XycePrnTable,
        offset: usize,
        point_count: usize,
        step_index: usize,
    ) -> Result<(XycePrnTable, usize), String> {
        let end = offset.checked_add(point_count).ok_or_else(|| {
            format!(
                "stepped NOISE waveform row range overflowed in step {}",
                step_index + 1
            )
        })?;
        if end > reference.rows.len() {
            return Err(format!(
                "stepped NOISE waveform oracle ended before step {}: need rows {offset}..{end}, have {}",
                step_index + 1,
                reference.rows.len()
            ));
        }
        Ok((
            XycePrnTable {
                columns: reference.columns.clone(),
                rows: reference.rows[offset..end].to_vec(),
            },
            end,
        ))
    }

    pub(super) fn transient_probe_matches_reference_time_neighborhood(
        &self,
        reference: &XycePrnTable,
        time_column: usize,
        row_index: usize,
        data_column: usize,
        actual: Value,
        tolerance: XyceComparisonTolerance,
        time_tolerance: Value,
        time_scale_factor: Value,
    ) -> bool {
        if !actual.is_finite()
            || !time_tolerance.is_finite()
            || time_tolerance < 0.0
            || !time_scale_factor.is_finite()
            || time_scale_factor <= 0.0
        {
            return false;
        }
        let Some(row) = reference.rows.get(row_index) else {
            return false;
        };
        let Some(time) = row
            .get(time_column)
            .copied()
            .map(|reference_time| reference_time / time_scale_factor)
        else {
            return false;
        };
        if !time.is_finite() {
            return false;
        }

        let mut first_row = row_index;
        while first_row > 0
            && Self::reference_time_is_in_prn_neighborhood(
                time,
                reference.rows[first_row - 1]
                    .get(time_column)
                    .copied()
                    .map(|reference_time| reference_time / time_scale_factor),
                time_tolerance,
            )
        {
            first_row -= 1;
        }

        let mut last_row = row_index;
        while last_row + 1 < reference.rows.len()
            && Self::reference_time_is_in_prn_neighborhood(
                time,
                reference.rows[last_row + 1]
                    .get(time_column)
                    .copied()
                    .map(|reference_time| reference_time / time_scale_factor),
                time_tolerance,
            )
        {
            last_row += 1;
        }

        if first_row == last_row {
            return false;
        }

        let mut min_reference = Value::INFINITY;
        let mut max_reference = Value::NEG_INFINITY;
        let mut finite_values = 0usize;
        for row in &reference.rows[first_row..=last_row] {
            let Some(&value) = row.get(data_column) else {
                continue;
            };
            if value.is_finite() {
                finite_values += 1;
                min_reference = min_reference.min(value);
                max_reference = max_reference.max(value);
            }
        }

        finite_values >= 2
            && min_reference.is_finite()
            && max_reference.is_finite()
            && (actual >= min_reference && actual <= max_reference
                || self
                    .value_mismatch(min_reference, actual, tolerance)
                    .is_none()
                || self
                    .value_mismatch(max_reference, actual, tolerance)
                    .is_none())
    }

    pub(super) fn reference_time_is_in_prn_neighborhood(
        anchor_time: Value,
        candidate_time: Option<Value>,
        time_tolerance: Value,
    ) -> bool {
        let Some(candidate_time) = candidate_time else {
            return false;
        };
        if !anchor_time.is_finite() || !candidate_time.is_finite() {
            return false;
        }
        if candidate_time == anchor_time {
            return true;
        }
        let candidate_tolerance = Self::default_prn_time_quantization_tolerance(candidate_time);
        let neighborhood = time_tolerance.max(candidate_tolerance) * PRN_TIME_NEIGHBOR_HALF_ULPS;
        let binary_roundoff = Value::EPSILON * anchor_time.abs().max(candidate_time.abs());
        neighborhood > 0.0 && (candidate_time - anchor_time).abs() <= neighborhood + binary_roundoff
    }

    pub(super) fn reference_tran_data_columns(
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        first_data_column: usize,
    ) -> Result<Vec<String>, String> {
        let mut data_columns =
            Vec::with_capacity(reference.columns.len().saturating_sub(first_data_column));
        let mut probe_index = 0usize;
        for column in reference.columns.iter().skip(first_data_column) {
            if let Some(probe) = print.probes.get(probe_index)
                && Self::reference_column_matches_probe(column, probe, netlist.ground_policy())
            {
                data_columns.push(probe.clone());
                probe_index += 1;
                continue;
            }

            data_columns.push(column.clone());
        }
        if probe_index != print.probes.len() {
            return Err(format!(
                "reference table matched {} .PRINT TRAN probe(s), but deck requested {}",
                probe_index,
                print.probes.len()
            ));
        }
        Ok(data_columns)
    }

    pub(super) fn transient_max_step_for_reference(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        reference: &XycePrnTable,
    ) -> Result<Value, String> {
        // A nonlinear Xyce Core's PRN cadence is an output contract, not its
        // accepted solver cadence. The native MutIndNonLin device carries
        // hidden M/R history across the adaptive TrapGear path; constraining
        // DELMAX to the smallest printed interval changes that history and
        // produces a different physical trajectory. Keep the native solver
        // ceiling for these devices and interpolate the output at PRN times.
        if Self::netlist_has_xyce_nonlinear_core(netlist) {
            return Self::transient_max_step_with_solver_ceiling(
                netlist,
                tran,
                None,
                Self::transient_oracle_solver_max_step_for_netlist(netlist, tran),
                false,
            );
        }

        let reference_step = Self::reference_min_positive_time_step(reference)?;
        Self::transient_max_step_with_optional_reference(netlist, tran, reference_step)
    }

    pub(super) fn netlist_has_xyce_nonlinear_core(netlist: &Netlist) -> bool {
        let contains_core = |elements: &[rspice_core::netlist::Element]| {
            elements.iter().any(|element| match &element.kind {
                ElementKind::JilesAthertonInductor { .. } => true,
                ElementKind::Coupling {
                    model: Some(_),
                    inductors,
                    ..
                } => !inductors.is_empty(),
                _ => false,
            })
        };
        contains_core(&netlist.elements)
            || flatten_netlist_with_models(netlist)
                .is_ok_and(|flattened| contains_core(&flattened.elements))
    }

    pub(super) fn transient_max_step_with_optional_reference(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
        reference_step: Option<Value>,
    ) -> Result<Value, String> {
        Self::transient_max_step_with_solver_ceiling(
            netlist,
            tran,
            reference_step,
            Self::transient_oracle_solver_max_step_for_netlist(netlist, tran),
            true,
        )
    }

    pub(super) fn feasible_reference_limited_step(
        tran: &XyceTranAnalysis,
        reference_step: Option<Value>,
    ) -> Option<Value> {
        let reference_step =
            reference_step.filter(|step| step.is_finite() && *step > f64::MIN_POSITIVE)?;

        // The Xyce accepted cadence can be part of the oracle for dynamic
        // decks, so use the reference minimum spacing when it is affordable.
        // Some Xyce references contain a tiny adaptive gap in an otherwise
        // coarse table; those must fall back to source/requested/final-time
        // limits and be compared by interpolation instead of forcing millions
        // of native steps.
        Self::feasible_oracle_limited_step(tran, reference_step)
    }

    pub(super) fn reference_min_positive_time_step(
        reference: &XycePrnTable,
    ) -> Result<Option<Value>, String> {
        let time_column = Self::reference_time_column_index(reference)
            .ok_or_else(|| "reference table has no TIME column".to_string())?;
        let mut previous = None;
        let mut min_step: Option<Value> = None;
        for (row_index, row) in reference.rows.iter().enumerate() {
            let time = *row.get(time_column).ok_or_else(|| {
                format!("row {row_index} has no TIME column at index {time_column}")
            })?;
            if !time.is_finite() {
                return Err(format!("row {row_index} has non-finite TIME value {time}"));
            }
            if let Some(previous_time) = previous {
                let step = time - previous_time;
                if step < 0.0 {
                    return Err(format!(
                        "reference TIME column is not monotonic at row {row_index}"
                    ));
                }
                if step > 0.0 {
                    min_step = Some(min_step.map_or(step, |current| current.min(step)));
                }
            }
            previous = Some(time);
        }
        Ok(min_step)
    }

    pub(super) fn reference_time_grid(reference: &XycePrnTable) -> Result<Vec<Value>, String> {
        let time_column = Self::reference_time_column_index(reference)
            .ok_or_else(|| "reference table has no TIME column".to_string())?;
        let mut previous = None;
        let mut grid = Vec::with_capacity(reference.rows.len());
        for (row_index, row) in reference.rows.iter().enumerate() {
            let time = *row.get(time_column).ok_or_else(|| {
                format!("row {row_index} has no TIME column at index {time_column}")
            })?;
            if !time.is_finite() {
                return Err(format!("row {row_index} has non-finite TIME value {time}"));
            }
            if let Some(previous_time) = previous
                && time < previous_time
            {
                return Err(format!(
                    "reference TIME column is not monotonic at row {row_index}"
                ));
            }
            if previous.is_some_and(|previous_time| time == previous_time) {
                // Xyce's fixed-width PRN writer can round adjacent accepted
                // points to the same printed TIME.  The simulator retains
                // the first row when reconstructing a solver grid; preserve
                // that Release 7.10 behavior for the pointwise fallback too.
                continue;
            }
            grid.push(time);
            previous = Some(time);
        }
        Ok(grid)
    }

    pub(super) fn tran_analysis_for_reference_stop(
        contract: XyceStaticTranContract,
        tran: XyceTranAnalysis,
        reference_time_grid: &[Value],
    ) -> XyceTranAnalysis {
        if !contract.can_use_reference_stop() {
            return tran;
        }
        let Some(reference_stop) = reference_time_grid
            .last()
            .copied()
            .filter(|time| time.is_finite() && *time > tran.stop)
        else {
            return tran;
        };

        XyceTranAnalysis {
            stop: reference_stop,
            ..tran
        }
    }

    pub(super) fn reference_time_column_index(reference: &XycePrnTable) -> Option<usize> {
        Self::transient_reference_layout(reference)
            .ok()
            .map(|layout| layout.time_column)
    }

    pub(super) fn transient_reference_layout(
        reference: &XycePrnTable,
    ) -> Result<XyceTransientReferenceLayout, String> {
        if reference.columns.is_empty() {
            return Err("reference table has no columns".to_string());
        }

        let mut cursor = 0usize;
        let stepnum_column = reference
            .columns
            .get(cursor)
            .is_some_and(|column| column.eq_ignore_ascii_case("STEPNUM"))
            .then(|| {
                let column = cursor;
                cursor += 1;
                column
            });
        let index_column = reference
            .columns
            .get(cursor)
            .is_some_and(|column| column.eq_ignore_ascii_case("Index"))
            .then(|| {
                let column = cursor;
                cursor += 1;
                column
            });
        let time_column = cursor;
        if reference
            .columns
            .get(time_column)
            .is_none_or(|column| Self::normalize_probe(column) != "time")
        {
            return Err(format!(
                "expected Xyce transient .prn table to contain optional STEPNUM/Index metadata followed by TIME, got columns {:?}",
                reference.columns
            ));
        }

        Ok(XyceTransientReferenceLayout {
            stepnum_column,
            index_column,
            time_column,
            data_column_offset: time_column + 1,
        })
    }

    pub(super) fn step_res_reference_path(
        deck_path: &Path,
        reference_path: &Path,
    ) -> Option<PathBuf> {
        let output_res_path = reference_path.with_extension("res");
        if output_res_path.is_file() {
            return Some(output_res_path);
        }
        let deck_res_path = Self::deck_sidecar_path(deck_path, "res");
        if deck_res_path.is_file() {
            return Some(deck_res_path);
        }
        None
    }

    pub(super) fn parse_xyce_numeric_token(token: &str) -> Result<f64, std::num::ParseFloatError> {
        let normalized = token.trim_end_matches(',');
        normalized
            .parse::<f64>()
            .or_else(|_| normalized.replace(['D', 'd'], "e").parse::<f64>())
    }

    pub(super) fn reference_data_columns(
        &self,
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        first_data_column: usize,
        ordered_print_columns: bool,
    ) -> Result<Vec<XyceReferenceColumn>, String> {
        let mut data_columns =
            Vec::with_capacity(reference.columns.len().saturating_sub(first_data_column));
        let mut probe_index = 0usize;
        let mut used_probe_indices = BTreeSet::new();
        for column in reference.columns.iter().skip(first_data_column) {
            if Self::is_primary_dc_sweep_reference_column(column) {
                data_columns.push(XyceReferenceColumn::PrimarySweep {
                    name: column.clone(),
                });
                continue;
            }

            let (matched_index, probe) = if ordered_print_columns {
                let mut skipped_omitted_probes = false;
                let probe = loop {
                    let Some(probe) = print.probes.get(probe_index) else {
                        return Err(format!(
                            "reference column '{}' has no matching .PRINT DC probe",
                            column
                        ));
                    };
                    if Self::reference_column_matches_probe(column, probe, netlist.ground_policy())
                    {
                        break probe;
                    }
                    if Self::dc_probe_is_omitted_empty_wildcard(probe, netlist) {
                        probe_index += 1;
                        skipped_omitted_probes = true;
                        continue;
                    }
                    let prefix = if skipped_omitted_probes {
                        "after omitted empty wildcard probe(s), "
                    } else {
                        ""
                    };
                    return Err(format!(
                        "{prefix}reference column '{}' does not match .PRINT probe '{}'",
                        column, probe
                    ));
                };
                if probe_index >= print.probes.len() {
                    return Err(format!(
                        "reference column '{}' has no matching .PRINT DC probe",
                        column
                    ));
                }
                (probe_index, probe)
            } else {
                let Some((index, probe)) =
                    print.probes.iter().enumerate().find(|(index, probe)| {
                        !used_probe_indices.contains(index)
                            && Self::reference_column_matches_probe(
                                column,
                                probe,
                                netlist.ground_policy(),
                            )
                    })
                else {
                    return Err(format!(
                        "compact reference column '{}' has no matching .PRINT DC probe",
                        column
                    ));
                };
                (index, probe)
            };
            used_probe_indices.insert(matched_index);
            data_columns.push(XyceReferenceColumn::Probe {
                name: probe.clone(),
            });
            if ordered_print_columns {
                probe_index += 1;
            }
        }
        if ordered_print_columns {
            while let Some(probe) = print.probes.get(probe_index) {
                if !Self::dc_probe_is_omitted_empty_wildcard(probe, netlist) {
                    break;
                }
                probe_index += 1;
            }
        }
        if ordered_print_columns && probe_index != print.probes.len() {
            return Err(format!(
                "reference table matched {} .PRINT DC probe(s), but deck requested {}",
                probe_index,
                print.probes.len()
            ));
        }
        Ok(data_columns)
    }

    pub(super) fn is_primary_dc_sweep_reference_column(column: &str) -> bool {
        matches!(Self::normalize_probe(column).as_str(), "v-sweep" | "sweep")
    }

    pub(super) fn reference_column_probe_for_matching(column: &str) -> &str {
        let trimmed = column.trim();
        trimmed
            .strip_prefix('{')
            .and_then(|body| body.strip_suffix('}'))
            .map(str::trim)
            .unwrap_or(trimmed)
    }

    pub(super) fn canonical_reference_node_name(
        node: &str,
        ground_policy: rspice_core::netlist::GroundPolicy,
    ) -> &str {
        if ground_policy.is_ground(node) {
            "0"
        } else {
            node
        }
    }

    pub(super) fn reference_voltage_column_matches_probe(
        column: &str,
        probe: &str,
        ground_policy: rspice_core::netlist::GroundPolicy,
    ) -> bool {
        let Some(column_probe) = Self::parse_tran_voltage_probe(column) else {
            return false;
        };
        let Some(requested_probe) = Self::parse_tran_voltage_probe(probe) else {
            return false;
        };
        column_probe.accessor == requested_probe.accessor
            && Self::canonical_reference_node_name(&column_probe.node_pos, ground_policy)
                == Self::canonical_reference_node_name(&requested_probe.node_pos, ground_policy)
            && column_probe
                .node_neg
                .as_deref()
                .map(|node| Self::canonical_reference_node_name(node, ground_policy))
                == requested_probe
                    .node_neg
                    .as_deref()
                    .map(|node| Self::canonical_reference_node_name(node, ground_policy))
    }

    pub(super) fn reference_column_matches_probe(
        column: &str,
        probe: &str,
        ground_policy: rspice_core::netlist::GroundPolicy,
    ) -> bool {
        let normalized_column = Self::normalize_probe(column);
        let normalized_probe = Self::normalize_probe(probe);
        if normalized_column == normalized_probe {
            return true;
        }

        let normalized_column =
            Self::normalize_probe(Self::reference_column_probe_for_matching(column));
        let normalized_probe =
            Self::normalize_probe(Self::reference_column_probe_for_matching(probe));
        if normalized_column == normalized_probe {
            return true;
        }
        if Self::reference_voltage_column_matches_probe(
            &normalized_column,
            &normalized_probe,
            ground_policy,
        ) {
            return true;
        }
        if let Some(mapped_probe) = Self::compact_reference_probe_alias(&normalized_column) {
            return mapped_probe == normalized_probe;
        }
        if let Some(source_name) = Self::parse_current_probe(&normalized_probe) {
            return normalized_column == format!("{source_name}_branch")
                || normalized_column == format!("{source_name}#branch");
        }
        false
    }

    pub(super) fn reference_ac_data_columns(
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        data_column_offset: usize,
    ) -> Result<Vec<XyceAcReferenceColumn>, String> {
        let mut columns = Vec::new();
        for column in reference.columns.iter().skip(data_column_offset) {
            if Self::print_requests_scalar_ac_probe(print, column) {
                columns.push(XyceAcReferenceColumn::Probe {
                    name: column.clone(),
                    component: XyceAcProbeComponent::Scalar,
                });
                continue;
            }

            if let Some((component, probe)) = Self::parse_ac_component_reference_column(column) {
                if !Self::print_requests_complex_ac_probe(print, &probe) {
                    return Err(format!(
                        "AC reference column '{}' is not produced by the deck's .PRINT AC probes",
                        column
                    ));
                }
                columns.push(XyceAcReferenceColumn::Probe {
                    name: probe,
                    component,
                });
                continue;
            }

            return Err(format!(
                "AC reference column '{}' is not produced by the deck's .PRINT AC probes",
                column
            ));
        }
        Ok(columns)
    }

    pub(super) fn is_ac_frequency_reference_column(column: &str) -> bool {
        matches!(Self::normalize_probe(column).as_str(), "freq" | "frequency")
    }

    pub(super) fn parse_ac_component_reference_column(
        column: &str,
    ) -> Option<(XyceAcProbeComponent, String)> {
        let normalized = Self::normalize_probe(column);
        let (prefix, component) = if normalized.starts_with("re(") {
            ("re(", XyceAcProbeComponent::Real)
        } else if normalized.starts_with("im(") {
            ("im(", XyceAcProbeComponent::Imaginary)
        } else {
            return None;
        };
        if !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[prefix.len()..normalized.len() - 1];
        (!inner.is_empty()).then(|| (component, inner.to_string()))
    }

    pub(super) fn compact_reference_probe_alias(normalized_column: &str) -> Option<&'static str> {
        match normalized_column {
            "v(g)" => Some("v(g,ga)"),
            "v(d)" => Some("v(d,da)"),
            "v(s)" => Some("v(s,sa)"),
            "v(b)" => Some("v(b,ba)"),
            "i(d)" => Some("i(vdprobe)"),
            "i(g)" => Some("i(vgprobe)"),
            "i(s)" => Some("i(vsprobe)"),
            "i(b)" => Some("i(vbprobe)"),
            _ => None,
        }
    }

    pub(super) fn parse_comp_float(value: &str) -> Result<f64, String> {
        value
            .parse::<f64>()
            .map_err(|err| format!("invalid Xyce *COMP numeric value '{value}': {err}"))
    }

    pub(super) fn parse_transient_behavioral_expression(
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<Expr, String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison could not prepare behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT TRAN comparison does not yet support behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        Ok(ast)
    }

    pub(super) fn dc_probe_references_voltage_source_current(
        probe: &str,
        netlist: &Netlist,
    ) -> Result<bool, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(source_name) = Self::parse_current_probe(&normalized) {
            return Ok(Self::source_is_voltage_source(netlist, &source_name));
        }

        let Some(expression) = Self::print_expression_inner(&normalized) else {
            return Ok(false);
        };
        if !Self::print_expression_contains_probe_call(expression) {
            return Ok(false);
        }

        let mut references_voltage_source_current = false;
        Self::rewrite_print_expression_calls(expression, netlist.params.clone(), |call| {
            if let Some(source_name) = Self::parse_current_probe(call)
                && Self::source_is_voltage_source(netlist, &source_name)
            {
                references_voltage_source_current = true;
            }
            Ok(0.0)
        })?;
        Ok(references_voltage_source_current)
    }

    pub(super) fn dc_probe_references_current_source_current(
        probe: &str,
        netlist: &Netlist,
    ) -> Result<bool, String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(source_name) = Self::parse_current_probe(&normalized) {
            return Ok(Self::source_is_current_source(netlist, &source_name));
        }

        let Some(expression) = Self::print_expression_inner(&normalized) else {
            return Ok(false);
        };
        if !Self::print_expression_contains_probe_call(expression) {
            return Ok(false);
        }

        let mut references_current_source_current = false;
        Self::rewrite_print_expression_calls(expression, netlist.params.clone(), |call| {
            if let Some(source_name) = Self::parse_current_probe(call)
                && Self::source_is_current_source(netlist, &source_name)
            {
                references_current_source_current = true;
            }
            Ok(0.0)
        })?;
        Ok(references_current_source_current)
    }

    pub(super) fn evaluate_ac_reference_column(
        column: &XyceAcReferenceColumn,
        netlist: &Netlist,
        result: &AcResult,
        phase_output_radians: bool,
    ) -> Result<Value, String> {
        match column {
            XyceAcReferenceColumn::Probe { name, component } => match component {
                XyceAcProbeComponent::Scalar => {
                    Self::evaluate_ac_probe(name, netlist, result, phase_output_radians)
                }
                XyceAcProbeComponent::Real => {
                    Ok(Self::evaluate_ac_complex_probe(name, netlist, result)?.re)
                }
                XyceAcProbeComponent::Imaginary => {
                    Ok(Self::evaluate_ac_complex_probe(name, netlist, result)?.im)
                }
            },
        }
    }

    pub(super) fn hb_frequency_result_to_prn_table(
        reference: &XycePrnTable,
        print: &XycePrintRequest,
        netlist: &Netlist,
        result: &rspice_core::analysis::HbResult,
    ) -> Result<XycePrnTable, String> {
        if reference.columns.len() < 3
            || reference.columns.first().map(String::as_str) != Some("Index")
            || reference.columns.get(1).map(String::as_str) != Some("FREQ")
        {
            return Err(
                "HB.FD ACComparator table must begin with exact 'Index FREQ' columns".to_string(),
            );
        }
        let data_columns = Self::reference_ac_data_columns(reference, print, 2)?;
        let expected_rows = 2 * result.num_harmonics + 1;
        if reference.rows.len() != expected_rows {
            return Err(format!(
                "HB.FD oracle has {} rows, expected {expected_rows} for {} harmonics",
                reference.rows.len(),
                result.num_harmonics
            ));
        }

        let node_names = result
            .spectral_voltages
            .iter()
            .map(|node| node.node_name.clone())
            .collect::<Vec<_>>();
        let mut rows = Vec::with_capacity(expected_rows);
        for (row_index, signed_harmonic) in
            (-(result.num_harmonics as isize)..=result.num_harmonics as isize).enumerate()
        {
            let harmonic = signed_harmonic.unsigned_abs();
            let voltages = result
                .spectral_voltages
                .iter()
                .map(|node| {
                    let coefficient =
                        node.coefficients.get(harmonic).copied().ok_or_else(|| {
                            format!(
                                "HB node '{}' is missing harmonic {harmonic}",
                                node.node_name
                            )
                        })?;
                    let coefficient = if harmonic == 0 {
                        coefficient
                    } else {
                        coefficient / 2.0
                    };
                    Ok(if signed_harmonic < 0 {
                        coefficient.conj()
                    } else {
                        coefficient
                    })
                })
                .collect::<Result<Vec<_>, String>>()?;
            let frequency = signed_harmonic as Value * result.fundamental_freq;
            let ac_result = AcResult {
                frequency,
                node_names: node_names.clone(),
                branch_names: Vec::new(),
                voltages,
                currents: Vec::new(),
            };
            let mut row = Vec::with_capacity(reference.columns.len());
            row.push(row_index as Value);
            row.push(frequency);
            for column in &data_columns {
                row.push(Self::evaluate_ac_reference_column(
                    column, netlist, &ac_result, false,
                )?);
            }
            rows.push(row);
        }
        Ok(XycePrnTable {
            columns: reference.columns.clone(),
            rows,
        })
    }

    pub(super) fn hb_transient_result_to_prn_table(
        print: &XycePrintRequest,
        netlist: &Netlist,
        result: &TransientResult,
    ) -> Result<XycePrnTable, String> {
        Self::validate_transient_result_time_grid(result)?;
        let columns = Self::transient_prn_header_columns(print, true);
        let mut rows = Vec::with_capacity(result.time.len());
        for (index, time) in result.time.iter().copied().enumerate() {
            let mut row = Vec::with_capacity(columns.len());
            row.push(index as Value);
            row.push(time);
            for probe in &print.probes {
                row.push(Self::evaluate_tran_probe(probe, netlist, result, time)?);
            }
            rows.push(row);
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn print_expression_contains_probe_reference(expression: &str) -> bool {
        let mut index = 0usize;
        while index < expression.len() {
            if Self::print_probe_call_open_index(expression, index).is_some()
                || Self::print_device_parameter_token_at(expression, index).is_some()
            {
                return true;
            }
            let ch = expression[index..]
                .chars()
                .next()
                .expect("valid char boundary");
            index += ch.len_utf8();
        }
        false
    }

    pub(super) fn split_prn_header_fields(line: &str) -> Result<Vec<String>, String> {
        Self::split_grouped_whitespace_fields(line, "Xyce .prn header")
    }

    pub(super) fn parse_measurement_reference_file(
        path: &Path,
    ) -> Result<Vec<XyceMeasurementReference>, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut references = Vec::new();
        let mut names = BTreeSet::new();
        for (line_index, raw_line) in content.lines().enumerate() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('*') {
                continue;
            }
            let (raw_name, raw_value) = line.split_once('=').ok_or_else(|| {
                format!(
                    "{}:{}: expected '<measurement> = <value|FAILED>'",
                    path.display(),
                    line_index + 1
                )
            })?;
            let name = raw_name.trim();
            if name.is_empty() {
                return Err(format!(
                    "{}:{}: measurement name is empty",
                    path.display(),
                    line_index + 1
                ));
            }
            let normalized_name = name.to_ascii_uppercase();
            if !names.insert(normalized_name) {
                return Err(format!(
                    "{}:{}: duplicate measurement '{}'",
                    path.display(),
                    line_index + 1,
                    name
                ));
            }
            let raw_value = raw_value.trim();
            if raw_value.split_whitespace().count() != 1 {
                return Err(format!(
                    "{}:{}: measurement result must be one token",
                    path.display(),
                    line_index + 1
                ));
            }
            let value = if raw_value == "FAILED" {
                XyceMeasurementReferenceValue::Failed
            } else {
                let value =
                    rspice_core::netlist::lexer::parse_spice_value(raw_value).map_err(|err| {
                        format!(
                            "{}:{}: invalid measurement value '{}': {err}",
                            path.display(),
                            line_index + 1,
                            raw_value
                        )
                    })?;
                if !value.is_finite() {
                    return Err(format!(
                        "{}:{}: measurement value must be finite",
                        path.display(),
                        line_index + 1
                    ));
                }
                XyceMeasurementReferenceValue::Numeric {
                    value,
                    quantization: Self::measurement_literal_quantization(raw_value),
                }
            };
            references.push(XyceMeasurementReference {
                name: name.to_string(),
                value,
            });
        }
        if references.is_empty() {
            return Err(format!("{} has no measurement results", path.display()));
        }
        Ok(references)
    }

    pub(super) fn parse_continuous_measurement_reference_file(
        path: &Path,
    ) -> Result<XyceContinuousMeasurementReference, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut name: Option<String> = None;
        let mut records = Vec::new();
        for (line_index, raw_line) in content.lines().enumerate() {
            let line_number = line_index + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('*') {
                continue;
            }
            let (raw_name, raw_fields) = line.split_once('=').ok_or_else(|| {
                format!(
                    "{}:{line_number}: expected '<measurement> = <value|FAILED>'",
                    path.display()
                )
            })?;
            let row_name = raw_name.trim();
            if row_name.is_empty() {
                return Err(format!(
                    "{}:{line_number}: measurement name is empty",
                    path.display()
                ));
            }
            if let Some(expected_name) = name.as_deref() {
                if !row_name.eq_ignore_ascii_case(expected_name) {
                    return Err(format!(
                        "{}:{line_number}: measurement '{}' does not match first row '{}'",
                        path.display(),
                        row_name,
                        expected_name
                    ));
                }
            } else {
                name = Some(row_name.to_string());
            }

            let fields = raw_fields.split_whitespace().collect::<Vec<_>>();
            if fields.is_empty() {
                return Err(format!(
                    "{}:{line_number}: measurement result is empty",
                    path.display()
                ));
            }
            let value = Self::parse_measurement_reference_token(path, line_number, fields[0])?;
            let mut trigger_axis = None;
            let mut target_axis = None;
            let mut field_index = 1usize;
            while field_index < fields.len() {
                if field_index + 2 >= fields.len() || fields[field_index + 1] != "=" {
                    return Err(format!(
                        "{}:{line_number}: expected '<targ|trig> = <value>' after measurement value",
                        path.display()
                    ));
                }
                let label = fields[field_index];
                let parsed = Self::parse_measurement_reference_token(
                    path,
                    line_number,
                    fields[field_index + 2],
                )?;
                if matches!(parsed, XyceMeasurementReferenceValue::Failed) {
                    return Err(format!(
                        "{}:{line_number}: {label} metadata cannot be FAILED",
                        path.display()
                    ));
                }
                let slot = if label.eq_ignore_ascii_case("trig") {
                    &mut trigger_axis
                } else if label.eq_ignore_ascii_case("targ") {
                    &mut target_axis
                } else {
                    return Err(format!(
                        "{}:{line_number}: unknown continuous measurement metadata '{label}'",
                        path.display()
                    ));
                };
                if slot.replace(parsed).is_some() {
                    return Err(format!(
                        "{}:{line_number}: duplicate {label} metadata",
                        path.display()
                    ));
                }
                field_index += 3;
            }
            if matches!(value, XyceMeasurementReferenceValue::Failed)
                && (trigger_axis.is_some() || target_axis.is_some())
            {
                return Err(format!(
                    "{}:{line_number}: FAILED result cannot carry trigger or target metadata",
                    path.display()
                ));
            }
            if trigger_axis.is_some() != target_axis.is_some() {
                return Err(format!(
                    "{}:{line_number}: trigger/target metadata must be present as a pair",
                    path.display()
                ));
            }
            records.push(XyceContinuousMeasurementReferenceRecord {
                value,
                trigger_axis,
                target_axis,
            });
        }
        let name = name.ok_or_else(|| format!("{} has no measurement results", path.display()))?;
        Ok(XyceContinuousMeasurementReference { name, records })
    }

    pub(super) fn parse_measurement_reference_token(
        path: &Path,
        line_number: usize,
        raw_value: &str,
    ) -> Result<XyceMeasurementReferenceValue, String> {
        if raw_value == "FAILED" {
            return Ok(XyceMeasurementReferenceValue::Failed);
        }
        let value = rspice_core::netlist::lexer::parse_spice_value(raw_value).map_err(|err| {
            format!(
                "{}:{line_number}: invalid measurement value '{}': {err}",
                path.display(),
                raw_value
            )
        })?;
        if !value.is_finite() {
            return Err(format!(
                "{}:{line_number}: measurement value must be finite",
                path.display()
            ));
        }
        Ok(XyceMeasurementReferenceValue::Numeric {
            value,
            quantization: Self::measurement_literal_quantization(raw_value),
        })
    }

    pub(super) fn parse_mixed_measurement_reference_file(
        path: &Path,
    ) -> Result<Vec<XyceMixedMeasurementReferenceRow>, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        let mut rows = Vec::new();
        for (line_index, raw_line) in content.lines().enumerate() {
            let line_number = line_index + 1;
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('*') {
                continue;
            }
            let (raw_name, raw_fields) = line.split_once('=').ok_or_else(|| {
                format!(
                    "{}:{line_number}: expected '<measurement> = <value|FAILED>'",
                    path.display()
                )
            })?;
            let name = raw_name.trim();
            if name.is_empty() {
                return Err(format!(
                    "{}:{line_number}: measurement name is empty",
                    path.display()
                ));
            }
            let fields = raw_fields.split_whitespace().collect::<Vec<_>>();
            if fields.is_empty() {
                return Err(format!(
                    "{}:{line_number}: measurement result is empty",
                    path.display()
                ));
            }
            let value = Self::parse_measurement_reference_token(path, line_number, fields[0])?;
            let mut trigger_axis = None;
            let mut target_axis = None;
            let mut field_index = 1usize;
            while field_index < fields.len() {
                if field_index + 2 >= fields.len() || fields[field_index + 1] != "=" {
                    return Err(format!(
                        "{}:{line_number}: expected '<targ|trig> = <value|not found>' after measurement value",
                        path.display()
                    ));
                }
                let label = fields[field_index];
                let (parsed, consumed) = if fields[field_index + 2].eq_ignore_ascii_case("not") {
                    if fields
                        .get(field_index + 3)
                        .is_none_or(|field| !field.eq_ignore_ascii_case("found"))
                    {
                        return Err(format!(
                            "{}:{line_number}: expected 'not found' after {label} =",
                            path.display()
                        ));
                    }
                    (XyceMeasurementReferenceValue::Failed, 4)
                } else {
                    (
                        Self::parse_measurement_reference_token(
                            path,
                            line_number,
                            fields[field_index + 2],
                        )?,
                        3,
                    )
                };
                let slot = if label.eq_ignore_ascii_case("trig") {
                    &mut trigger_axis
                } else if label.eq_ignore_ascii_case("targ") {
                    &mut target_axis
                } else {
                    return Err(format!(
                        "{}:{line_number}: unknown mixed measurement metadata '{label}'",
                        path.display()
                    ));
                };
                if slot.replace(parsed).is_some() {
                    return Err(format!(
                        "{}:{line_number}: duplicate {label} metadata",
                        path.display()
                    ));
                }
                field_index += consumed;
            }
            if trigger_axis.is_some() != target_axis.is_some() {
                return Err(format!(
                    "{}:{line_number}: trigger/target metadata must be present as a pair",
                    path.display()
                ));
            }
            rows.push(XyceMixedMeasurementReferenceRow {
                name: name.to_string(),
                value,
                trigger_axis,
                target_axis,
            });
        }
        if rows.is_empty() {
            return Err(format!("{} has no measurement results", path.display()));
        }
        Ok(rows)
    }

    pub(super) fn parse_prn_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_prn_table(&content)
    }

    pub(super) fn parse_ac_comparator_prn_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        if content.is_empty() {
            return Err(format!("{} is empty", path.display()));
        }
        let last_nonempty = content.lines().rev().find(|line| !line.trim().is_empty());
        if last_nonempty.map(str::trim) != Some("End of Xyce(TM) Simulation") {
            return Err(format!(
                "{} has no exact ACComparator footer 'End of Xyce(TM) Simulation'",
                path.display()
            ));
        }
        Self::parse_prn_table(&content)
    }

    pub(super) fn parse_dc_reference_file(
        contract: XyceStaticDcContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticDcContract::WrapperRaw => Self::parse_raw_file(path),
            XyceStaticDcContract::PlainCsv | XyceStaticDcContract::WrapperCsv => {
                Self::parse_csv_file(path)
            }
            XyceStaticDcContract::PlainCsd | XyceStaticDcContract::WrapperCsd => {
                Self::parse_csd_file(path)
            }
            _ => Self::parse_prn_file(path),
        }
    }

    pub(super) fn parse_ac_reference_file(
        contract: XyceStaticAcContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticAcContract::PlainCsd | XyceStaticAcContract::WrapperCsd => {
                Self::parse_ac_csd_file(path)
            }
            _ => Self::parse_prn_file(path),
        }
    }

    pub(super) fn parse_noise_reference_file(
        contract: XyceStaticNoiseContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticNoiseContract::Csv => Self::parse_csv_file(path),
            XyceStaticNoiseContract::Tecplot => Self::parse_tecplot_file(path),
            _ => Self::parse_prn_file(path),
        }
    }

    pub(super) fn parse_tecplot_file(path: &Path) -> Result<XycePrnTable, String> {
        Ok(Self::parse_tecplot_reference_file(path)?.table)
    }

    pub(super) fn parse_tecplot_reference_file(
        path: &Path,
    ) -> Result<XyceTecplotReference, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_tecplot_reference_table(&content)
    }

    #[cfg(test)]
    pub(super) fn parse_tecplot_table(content: &str) -> Result<XycePrnTable, String> {
        Ok(Self::parse_tecplot_reference_table(content)?.table)
    }

    pub(super) fn parse_tecplot_reference_table(
        content: &str,
    ) -> Result<XyceTecplotReference, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(index, line)| (index + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        let Some(&(title_line_number, title)) = lines.first() else {
            return Err("empty Xyce TECPLOT table".to_string());
        };
        if !title.to_ascii_uppercase().starts_with("TITLE =") {
            return Err(format!(
                "Xyce TECPLOT table must begin with TITLE metadata at line {title_line_number}"
            ));
        }

        let variables_index = lines
            .iter()
            .position(|(_, line)| line.to_ascii_uppercase().starts_with("VARIABLES ="))
            .ok_or_else(|| "Xyce TECPLOT table has no VARIABLES declaration".to_string())?;
        let mut columns = Vec::new();
        let mut cursor = variables_index;
        while let Some(&(line_number, line)) = lines.get(cursor) {
            let variable_text = if cursor == variables_index {
                line.split_once('=')
                    .map(|(_, value)| value.trim())
                    .ok_or_else(|| {
                        format!("invalid Xyce TECPLOT VARIABLES declaration at line {line_number}")
                    })?
            } else if line.starts_with('"') {
                line
            } else {
                break;
            };
            let Some(quoted) = variable_text
                .strip_prefix('"')
                .and_then(|value| value.strip_suffix('"'))
            else {
                return Err(format!(
                    "Xyce TECPLOT variable at line {line_number} must be one quoted name"
                ));
            };
            let column = quoted.trim();
            if column.is_empty() {
                return Err(format!(
                    "Xyce TECPLOT variable at line {line_number} is empty"
                ));
            }
            columns.push(column.to_string());
            cursor += 1;
        }
        if columns.is_empty() {
            return Err("Xyce TECPLOT table has no variables".to_string());
        }

        let mut rows = Vec::new();
        let mut zones = Vec::new();
        let mut current_zone: Option<XyceTecplotZone> = None;
        let mut footer_seen = false;
        for (line_number, line) in lines.iter().skip(cursor).copied() {
            let upper = line.to_ascii_uppercase();
            if upper.starts_with("DATASETAUXDATA ") {
                continue;
            }
            if upper.starts_with("ZONE ") {
                if let Some(mut zone) = current_zone.take() {
                    zone.row_count = rows.len() - zone.row_start;
                    if zone.row_count == 0 {
                        return Err(format!(
                            "Xyce TECPLOT zone '{}' has no data rows",
                            zone.title
                        ));
                    }
                    zones.push(zone);
                }
                let title = Self::parse_tecplot_zone_title(line).map_err(|error| {
                    format!("invalid Xyce TECPLOT ZONE at line {line_number}: {error}")
                })?;
                current_zone = Some(XyceTecplotZone {
                    title,
                    auxdata: BTreeMap::new(),
                    row_start: rows.len(),
                    row_count: 0,
                });
                continue;
            }
            if upper.starts_with("AUXDATA ") {
                let Some(zone) = current_zone.as_mut() else {
                    return Err(format!(
                        "Xyce TECPLOT AUXDATA appears before the first ZONE at line {line_number}"
                    ));
                };
                if rows.len() != zone.row_start {
                    return Err(format!(
                        "Xyce TECPLOT AUXDATA appears after zone data at line {line_number}"
                    ));
                }
                let (name, value) = Self::parse_tecplot_auxdata(line).map_err(|error| {
                    format!("invalid Xyce TECPLOT AUXDATA at line {line_number}: {error}")
                })?;
                if zone.auxdata.insert(name.clone(), value).is_some() {
                    return Err(format!(
                        "Xyce TECPLOT zone '{}' repeats AUXDATA '{name}'",
                        zone.title
                    ));
                }
                continue;
            }
            if line == "End of Xyce(TM) Parameter Sweep" || line == "End of Xyce(TM) Simulation" {
                footer_seen = true;
                continue;
            }
            if footer_seen {
                return Err(format!(
                    "Xyce TECPLOT table has content after its completion footer at line {line_number}"
                ));
            }
            if current_zone.is_none() {
                return Err(format!(
                    "Xyce TECPLOT data appears before the first ZONE at line {line_number}"
                ));
            }
            let values = line
                .split_whitespace()
                .map(|token| {
                    Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid Xyce TECPLOT numeric token '{token}' at line {line_number}: {err}"
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != columns.len() {
                return Err(format!(
                    "Xyce TECPLOT data line {line_number} has {} values, expected {}",
                    values.len(),
                    columns.len()
                ));
            }
            rows.push(values);
        }
        if let Some(mut zone) = current_zone.take() {
            zone.row_count = rows.len() - zone.row_start;
            if zone.row_count == 0 {
                return Err(format!(
                    "Xyce TECPLOT zone '{}' has no data rows",
                    zone.title
                ));
            }
            zones.push(zone);
        }
        if zones.is_empty() {
            return Err("Xyce TECPLOT table has no ZONE".to_string());
        }
        if rows.is_empty() {
            return Err("Xyce TECPLOT table has no data rows".to_string());
        }
        if !footer_seen {
            return Err("Xyce TECPLOT table has no exact Xyce completion footer".to_string());
        }
        for zone in &zones {
            if zone.auxdata.is_empty() {
                return Err(format!(
                    "Xyce TECPLOT zone '{}' has no AUXDATA step bindings",
                    zone.title
                ));
            }
            for (name, expected) in &zone.auxdata {
                let actual =
                    Self::tecplot_zone_title_binding(&zone.title, name).ok_or_else(|| {
                        format!(
                            "Xyce TECPLOT zone title '{}' has no binding for AUXDATA '{name}'",
                            zone.title
                        )
                    })?;
                if !Self::tecplot_binding_matches(*expected, actual.value) {
                    return Err(format!(
                        "Xyce TECPLOT zone title '{}' binds {name}={}, but AUXDATA binds {}",
                        zone.title, actual.value, expected.value
                    ));
                }
            }
        }
        Ok(XyceTecplotReference {
            table: XycePrnTable { columns, rows },
            zones,
        })
    }

    pub(super) fn parse_tecplot_zone_title(line: &str) -> Result<String, String> {
        let upper = line.to_ascii_uppercase();
        let title_index = upper
            .find("T=")
            .ok_or_else(|| "ZONE has no T= title".to_string())?;
        let quoted = line[title_index + 2..].trim();
        let title = quoted
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .ok_or_else(|| "ZONE T= title must be quoted".to_string())?
            .trim();
        if title.is_empty() {
            return Err("ZONE T= title is empty".to_string());
        }
        Ok(title.to_string())
    }

    pub(super) fn parse_tecplot_auxdata(
        line: &str,
    ) -> Result<(String, XyceTecplotBinding), String> {
        let assignment = line
            .get("AUXDATA".len()..)
            .ok_or_else(|| "missing AUXDATA assignment".to_string())?
            .trim();
        let (name, raw_value) = assignment
            .split_once('=')
            .ok_or_else(|| "AUXDATA assignment has no '='".to_string())?;
        let name = name.trim();
        if name.is_empty() || name.chars().any(char::is_whitespace) {
            return Err("AUXDATA name must be one nonempty token".to_string());
        }
        let raw_value = raw_value.trim();
        let value = raw_value
            .strip_prefix('"')
            .and_then(|value| value.strip_suffix('"'))
            .ok_or_else(|| "AUXDATA value must be quoted".to_string())?
            .trim();
        let parsed = Self::parse_xyce_numeric_token(value)
            .map_err(|error| format!("AUXDATA value '{value}' is invalid: {error}"))?;
        Ok((
            name.to_string(),
            XyceTecplotBinding {
                value: parsed,
                quantization: Self::measurement_literal_quantization(value),
            },
        ))
    }

    pub(super) fn parse_tran_reference_file(
        contract: XyceStaticTranContract,
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        match contract {
            XyceStaticTranContract::PlainCsv | XyceStaticTranContract::WrapperCsv => {
                Self::parse_csv_file(path)
            }
            XyceStaticTranContract::PlainCsd | XyceStaticTranContract::WrapperCsd => {
                Self::parse_tran_csd_file(path)
            }
            _ => Self::parse_tran_prn_or_legacy_probe_file(path),
        }
    }

    pub(super) fn parse_xyce_verify_tran_reference_file(
        path: &Path,
    ) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_xyce_verify_tran_reference_table(&content)
    }

    pub(super) fn parse_xyce_verify_tran_reference_table(
        content: &str,
    ) -> Result<XycePrnTable, String> {
        let first_classified_line = content
            .lines()
            .find(|line| !line.is_empty())
            .ok_or_else(|| "empty Release 7.10 xyce_verify transient input".to_string())?;
        let first_header = first_classified_line.trim();
        if Self::prn_header_delimiter(first_header) != Some(XycePrnDelimiter::Whitespace) {
            return Err(
                "Release 7.10 xyce_verify transient input must begin with an indexed STD PRN header"
                    .to_string(),
            );
        }
        let first_columns = Self::parse_prn_columns(first_header, XycePrnDelimiter::Whitespace)?;
        if first_columns.first().map(String::as_str) != Some("Index") {
            return Err(
                "Release 7.10 xyce_verify transient input must begin with an indexed STD PRN header"
                    .to_string(),
            );
        }
        let table = Self::parse_prn_table(content)?;
        let header_index = content
            .lines()
            .position(|line| Self::is_prn_header_line(line.trim()))
            .expect("a parsed PRN table has a header");
        if !content
            .lines()
            .skip(header_index + 1)
            .any(|line| line.contains("End of Xyce"))
        {
            return Err(
                "Release 7.10 xyce_verify transient input has no normal 'End of Xyce' completion footer"
                    .to_string(),
            );
        }
        Ok(table)
    }

    pub(super) fn parse_tran_prn_or_legacy_probe_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_tran_prn_or_legacy_probe_table(&content)
    }

    pub(super) fn parse_tran_prn_or_legacy_probe_table(
        content: &str,
    ) -> Result<XycePrnTable, String> {
        let first_nonempty = content.lines().map(str::trim).find(|line| !line.is_empty());
        if first_nonempty.is_some_and(|line| line.eq_ignore_ascii_case("#H")) {
            return Self::parse_tran_csd_table(content);
        }
        Self::parse_prn_table(content)
    }

    pub(super) fn parse_csv_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_csv_table(&content)
    }

    pub(super) fn parse_csv_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(index, line)| (index + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        let Some(&(header_line_number, header_line)) = lines.first() else {
            return Err("empty Xyce CSV table".to_string());
        };
        let columns = Self::parse_csv_record(header_line).map_err(|error| {
            format!("invalid Xyce CSV header at line {header_line_number}: {error}")
        })?;
        if columns.is_empty() || columns.iter().any(|column| column.trim().is_empty()) {
            return Err("Xyce CSV header contains an empty column".to_string());
        }
        if !columns.first().is_some_and(|column| {
            Self::is_prn_metadata_header_token(column)
                || Self::looks_like_reference_probe_header(column)
        }) {
            return Err(format!(
                "Xyce CSV header begins with unsupported column '{}'",
                columns[0]
            ));
        }

        let mut rows = Vec::new();
        for &(line_number, line) in lines.iter().skip(1) {
            if line.to_ascii_lowercase().starts_with("end of xyce") {
                break;
            }
            let fields = Self::parse_csv_record(line).map_err(|error| {
                format!("invalid Xyce CSV record at line {line_number}: {error}")
            })?;
            if Self::same_prn_columns(&columns, &fields) {
                continue;
            }
            if fields.len() != columns.len() {
                return Err(format!(
                    "Xyce CSV data line {line_number} has {} values, expected {}",
                    fields.len(),
                    columns.len()
                ));
            }
            let row = fields
                .iter()
                .map(|field| {
                    Self::parse_xyce_numeric_token(field).map_err(|error| {
                        format!(
                            "invalid numeric token '{field}' on Xyce CSV data line {line_number}: {error}"
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            rows.push(row);
        }
        if rows.is_empty() {
            return Err("Xyce CSV table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_csv_record(line: &str) -> Result<Vec<String>, String> {
        let mut fields = Vec::new();
        let mut field = String::new();
        let mut chars = line.chars().peekable();
        let mut quoted = false;
        let mut quote_closed = false;

        while let Some(ch) = chars.next() {
            if quoted {
                if ch == '"' {
                    if chars.peek() == Some(&'"') {
                        chars.next();
                        field.push('"');
                    } else {
                        quoted = false;
                        quote_closed = true;
                    }
                } else {
                    field.push(ch);
                }
                continue;
            }
            match ch {
                '"' if field.trim().is_empty() && !quote_closed => {
                    field.clear();
                    quoted = true;
                }
                ',' => {
                    fields.push(field.trim().to_string());
                    field.clear();
                    quote_closed = false;
                }
                ch if quote_closed && ch.is_whitespace() => {}
                _ if quote_closed => {
                    return Err("characters follow a closed quoted field".to_string());
                }
                _ => field.push(ch),
            }
        }
        if quoted {
            return Err("unterminated quoted field".to_string());
        }
        fields.push(field.trim().to_string());
        Ok(fields)
    }

    pub(super) fn parse_raw_file(path: &Path) -> Result<XycePrnTable, String> {
        let bytes = fs::read(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_raw_table(&bytes)
    }

    pub(super) fn parse_csd_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_csd_table(&content)
    }

    pub(super) fn parse_ac_csd_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_ac_csd_table(&content)
    }

    pub(super) fn parse_tran_csd_file(path: &Path) -> Result<XycePrnTable, String> {
        let content =
            fs::read_to_string(path).map_err(|err| format!("{}: {err}", path.display()))?;
        Self::parse_tran_csd_table(&content)
    }

    pub(super) fn parse_tran_csd_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("empty Xyce transient CSDF table".to_string());
        }

        let mut columns: Option<Vec<String>> = None;
        let mut rows = Vec::new();
        let mut index = 0usize;
        while index < lines.len() {
            let (line_number, line) = lines[index];
            if !line.eq_ignore_ascii_case("#H") {
                return Err(format!(
                    "Xyce transient CSDF section must start with #H at line {line_number}, got '{line}'"
                ));
            }
            index += 1;

            let mut complex_values = false;
            let mut sweep_column = "TIME".to_string();
            while index < lines.len() {
                let (_, header_line) = lines[index];
                if header_line.eq_ignore_ascii_case("#N") {
                    break;
                }
                for (key, value) in Self::parse_csd_header_assignments(header_line) {
                    if key.eq_ignore_ascii_case("COMPLEXVALUES")
                        && value.eq_ignore_ascii_case("YES")
                    {
                        complex_values = true;
                    }
                    if key.eq_ignore_ascii_case("SWEEPVAR") {
                        sweep_column = value.to_ascii_uppercase();
                    }
                }
                index += 1;
            }
            if complex_values {
                return Err("Xyce transient CSDF COMPLEXVALUES=YES is not supported".to_string());
            }
            if index >= lines.len() {
                return Err("Xyce transient CSDF section has no #N column block".to_string());
            }

            index += 1;
            let Some((column_line_number, column_line)) = lines.get(index).copied() else {
                return Err("Xyce transient CSDF #N block has no column line".to_string());
            };
            let section_input_columns = Self::parse_csd_columns(column_line).map_err(|err| {
                format!("invalid Xyce transient CSDF column line {column_line_number}: {err}")
            })?;
            if section_input_columns.is_empty() {
                return Err(format!(
                    "Xyce transient CSDF column line {column_line_number} has no columns"
                ));
            }
            let mut section_columns = Vec::with_capacity(section_input_columns.len() + 1);
            section_columns.push(sweep_column);
            section_columns.extend(section_input_columns.iter().cloned());
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &section_columns) {
                    return Err(format!(
                        "Xyce transient CSDF section changes columns from {:?} to {:?}",
                        columns, section_columns
                    ));
                }
            } else {
                columns = Some(section_columns);
            }
            index += 1;

            while index < lines.len() {
                let (line_number, line) = lines[index];
                if line.eq_ignore_ascii_case("#;") {
                    index += 1;
                    break;
                }
                if line.eq_ignore_ascii_case("#H") {
                    break;
                }
                if !line.starts_with("#C") {
                    return Err(format!(
                        "expected Xyce transient CSDF #C row header at line {line_number}, got '{line}'"
                    ));
                }
                let (time, expected_count) =
                    Self::parse_csd_sweep_row_header(line).map_err(|err| {
                        format!(
                            "invalid Xyce transient CSDF #C row header at line {line_number}: {err}"
                        )
                    })?;
                index += 1;

                let mut row = Vec::with_capacity(expected_count + 1);
                row.push(time);
                while row.len() <= expected_count {
                    let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                        return Err(format!(
                            "Xyce transient CSDF row beginning at line {line_number} ended after {} value(s), expected {expected_count}",
                            row.len().saturating_sub(1)
                        ));
                    };
                    if data_line.starts_with('#') {
                        return Err(format!(
                            "Xyce transient CSDF row beginning at line {line_number} ended before {expected_count} value(s) at line {data_line_number}"
                        ));
                    }
                    for token in data_line.split_whitespace() {
                        if row.len() > expected_count {
                            return Err(format!(
                                "Xyce transient CSDF row beginning at line {line_number} has more than {expected_count} value(s)"
                            ));
                        }
                        let expected_position = row.len();
                        let value = Self::parse_csd_complex_value_token(token, expected_position)
                            .map_err(|err| {
                                format!(
                                    "invalid Xyce transient CSDF data token '{token}' at line {data_line_number}: {err}"
                                )
                            })?;
                        if value.im.abs() > f64::EPSILON {
                            return Err(format!(
                                "Xyce transient CSDF token '{token}' at line {data_line_number} has nonzero imaginary component {}",
                                value.im
                            ));
                        }
                        row.push(value.re);
                    }
                    index += 1;
                }
                if let Some(columns) = &columns
                    && row.len() != columns.len()
                {
                    return Err(format!(
                        "Xyce transient CSDF row beginning at line {line_number} has {} column value(s), expected {}",
                        row.len(),
                        columns.len()
                    ));
                }
                rows.push(row);
            }
        }

        let columns =
            columns.ok_or_else(|| "Xyce transient CSDF table has no columns".to_string())?;
        if rows.is_empty() {
            return Err("Xyce transient CSDF table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_ac_csd_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("empty Xyce AC CSDF table".to_string());
        }

        let mut columns: Option<Vec<String>> = None;
        let mut expansion: Option<Vec<XyceAcCsdColumnExpansion>> = None;
        let mut rows = Vec::new();
        let mut index = 0usize;
        while index < lines.len() {
            let (line_number, line) = lines[index];
            if !line.eq_ignore_ascii_case("#H") {
                return Err(format!(
                    "Xyce AC CSDF section must start with #H at line {line_number}, got '{line}'"
                ));
            }
            index += 1;

            let mut complex_values = false;
            let mut sweep_column = "FREQ".to_string();
            while index < lines.len() {
                let (_, header_line) = lines[index];
                if header_line.eq_ignore_ascii_case("#N") {
                    break;
                }
                for (key, value) in Self::parse_csd_header_assignments(header_line) {
                    if key.eq_ignore_ascii_case("COMPLEXVALUES")
                        && value.eq_ignore_ascii_case("YES")
                    {
                        complex_values = true;
                    }
                    if key.eq_ignore_ascii_case("SWEEPVAR") {
                        sweep_column = value.to_ascii_uppercase();
                    }
                }
                index += 1;
            }
            if index >= lines.len() {
                return Err("Xyce AC CSDF section has no #N column block".to_string());
            }

            index += 1;
            let Some((column_line_number, column_line)) = lines.get(index).copied() else {
                return Err("Xyce AC CSDF #N block has no column line".to_string());
            };
            let section_input_columns = Self::parse_csd_columns(column_line).map_err(|err| {
                format!("invalid Xyce AC CSDF column line {column_line_number}: {err}")
            })?;
            if section_input_columns.is_empty() {
                return Err(format!(
                    "Xyce AC CSDF column line {column_line_number} has no columns"
                ));
            }
            let section_expansion =
                Self::ac_csd_column_expansion(&section_input_columns, complex_values);
            let section_columns = Self::expanded_ac_csd_columns(
                &sweep_column,
                &section_input_columns,
                &section_expansion,
            );
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &section_columns) {
                    return Err(format!(
                        "Xyce AC CSDF section changes columns from {:?} to {:?}",
                        columns, section_columns
                    ));
                }
            } else {
                columns = Some(section_columns);
                expansion = Some(section_expansion);
            }
            let expansion = expansion
                .as_ref()
                .expect("AC CSDF expansion initialized with columns");
            index += 1;

            while index < lines.len() {
                let (line_number, line) = lines[index];
                if line.eq_ignore_ascii_case("#;") {
                    index += 1;
                    break;
                }
                if line.eq_ignore_ascii_case("#H") {
                    break;
                }
                if !line.starts_with("#C") {
                    return Err(format!(
                        "expected Xyce AC CSDF #C row header at line {line_number}, got '{line}'"
                    ));
                }
                let (frequency, expected_count) =
                    Self::parse_csd_sweep_row_header(line).map_err(|err| {
                        format!("invalid Xyce AC CSDF #C row header at line {line_number}: {err}")
                    })?;
                index += 1;

                let mut tokens = Vec::with_capacity(expected_count);
                while tokens.len() < expected_count {
                    let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                        return Err(format!(
                            "Xyce AC CSDF row beginning at line {line_number} ended after {} value(s), expected {expected_count}",
                            tokens.len()
                        ));
                    };
                    if data_line.starts_with('#') {
                        return Err(format!(
                            "Xyce AC CSDF row beginning at line {line_number} ended before {expected_count} value(s) at line {data_line_number}"
                        ));
                    }
                    for token in data_line.split_whitespace() {
                        if tokens.len() >= expected_count {
                            return Err(format!(
                                "Xyce AC CSDF row beginning at line {line_number} has more than {expected_count} value(s)"
                            ));
                        }
                        let expected_position = tokens.len() + 1;
                        tokens.push(
                            Self::parse_csd_complex_value_token(token, expected_position).map_err(
                                |err| {
                                    format!(
                                        "invalid Xyce AC CSDF data token '{token}' at line {data_line_number}: {err}"
                                    )
                                },
                            )?,
                        );
                    }
                    index += 1;
                }
                if tokens.len() != section_input_columns.len() {
                    return Err(format!(
                        "Xyce AC CSDF row beginning at line {line_number} has {} value(s), expected {} column(s)",
                        tokens.len(),
                        section_input_columns.len()
                    ));
                }

                let mut row = Vec::new();
                row.push(frequency);
                for (value, expansion) in tokens.into_iter().zip(expansion.iter()) {
                    match expansion {
                        XyceAcCsdColumnExpansion::Scalar => {
                            if value.im.abs() > f64::EPSILON {
                                return Err(format!(
                                    "Xyce AC CSDF scalar row beginning at line {line_number} has nonzero imaginary component {}",
                                    value.im
                                ));
                            }
                            row.push(value.re);
                        }
                        XyceAcCsdColumnExpansion::Complex => {
                            row.push(value.re);
                            row.push(value.im);
                        }
                    }
                }
                rows.push(row);
            }
        }

        let columns = columns.ok_or_else(|| "Xyce AC CSDF table has no columns".to_string())?;
        if rows.is_empty() {
            return Err("Xyce AC CSDF table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_csd_table(content: &str) -> Result<XycePrnTable, String> {
        let lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if lines.is_empty() {
            return Err("empty Xyce CSDF table".to_string());
        }

        let mut columns: Option<Vec<String>> = None;
        let mut rows = Vec::new();
        let mut index = 0usize;
        while index < lines.len() {
            let (line_number, line) = lines[index];
            if !line.eq_ignore_ascii_case("#H") {
                return Err(format!(
                    "Xyce CSDF section must start with #H at line {line_number}, got '{line}'"
                ));
            }
            index += 1;

            let mut complex_values = false;
            while index < lines.len() {
                let (_, header_line) = lines[index];
                if header_line.eq_ignore_ascii_case("#N") {
                    break;
                }
                for (key, value) in Self::parse_csd_header_assignments(header_line) {
                    if key.eq_ignore_ascii_case("COMPLEXVALUES")
                        && value.eq_ignore_ascii_case("YES")
                    {
                        complex_values = true;
                    }
                }
                index += 1;
            }
            if complex_values {
                return Err("Xyce CSDF COMPLEXVALUES=YES is not supported".to_string());
            }
            if index >= lines.len() {
                return Err("Xyce CSDF section has no #N column block".to_string());
            }

            index += 1;
            let Some((column_line_number, column_line)) = lines.get(index).copied() else {
                return Err("Xyce CSDF #N block has no column line".to_string());
            };
            let section_columns = Self::parse_csd_columns(column_line).map_err(|err| {
                format!("invalid Xyce CSDF column line {column_line_number}: {err}")
            })?;
            if section_columns.is_empty() {
                return Err(format!(
                    "Xyce CSDF column line {column_line_number} has no columns"
                ));
            }
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &section_columns) {
                    return Err(format!(
                        "Xyce CSDF section changes columns from {:?} to {:?}",
                        columns, section_columns
                    ));
                }
            } else {
                columns = Some(section_columns);
            }
            index += 1;

            while index < lines.len() {
                let (line_number, line) = lines[index];
                if line.eq_ignore_ascii_case("#;") {
                    index += 1;
                    break;
                }
                if line.eq_ignore_ascii_case("#H") {
                    break;
                }
                if !line.starts_with("#C") {
                    return Err(format!(
                        "expected Xyce CSDF #C row header at line {line_number}, got '{line}'"
                    ));
                }
                let (_, expected_count) =
                    Self::parse_csd_sweep_row_header(line).map_err(|err| {
                        format!("invalid Xyce CSDF #C row header at line {line_number}: {err}")
                    })?;
                index += 1;

                let mut row = Vec::with_capacity(expected_count);
                while row.len() < expected_count {
                    let Some((data_line_number, data_line)) = lines.get(index).copied() else {
                        return Err(format!(
                            "Xyce CSDF row beginning at line {line_number} ended after {} value(s), expected {expected_count}",
                            row.len()
                        ));
                    };
                    if data_line.starts_with('#') {
                        return Err(format!(
                            "Xyce CSDF row beginning at line {line_number} ended before {expected_count} value(s) at line {data_line_number}"
                        ));
                    }
                    for token in data_line.split_whitespace() {
                        if row.len() >= expected_count {
                            return Err(format!(
                                "Xyce CSDF row beginning at line {line_number} has more than {expected_count} value(s)"
                            ));
                        }
                        let expected_position = row.len() + 1;
                        let value = Self::parse_csd_complex_value_token(token, expected_position)
                            .map_err(|err| {
                                format!(
                                    "invalid Xyce CSDF data token '{token}' at line {data_line_number}: {err}"
                                )
                            })?;
                        if value.im.abs() > f64::EPSILON {
                            return Err(format!(
                                "Xyce CSDF real table token '{token}' at line {data_line_number} has nonzero imaginary component {}",
                                value.im
                            ));
                        }
                        row.push(value.re);
                    }
                    index += 1;
                }
                if let Some(columns) = &columns
                    && row.len() != columns.len()
                {
                    return Err(format!(
                        "Xyce CSDF row beginning at line {line_number} has {} value(s), expected {} column(s)",
                        row.len(),
                        columns.len()
                    ));
                }
                rows.push(row);
            }
        }

        let columns = columns.ok_or_else(|| "Xyce CSDF table has no columns".to_string())?;
        if rows.is_empty() {
            return Err("Xyce CSDF table has no data rows".to_string());
        }
        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_csd_header_assignments(line: &str) -> Vec<(&str, &str)> {
        line.split_whitespace()
            .filter_map(|field| {
                let (key, value) = field.split_once('=')?;
                Some((key.trim(), value.trim_matches(['"', '\''])))
            })
            .collect()
    }

    pub(super) fn parse_csd_columns(line: &str) -> Result<Vec<String>, String> {
        if !line.contains('\'') {
            return Ok(line.split_whitespace().map(str::to_string).collect());
        }

        let mut columns = Vec::new();
        let mut rest = line;
        loop {
            let Some(start) = rest.find('\'') else {
                if rest.trim().is_empty() {
                    return Ok(columns);
                }
                return Err(format!(
                    "unexpected unquoted text after CSDF column list: '{}'",
                    rest.trim()
                ));
            };
            if !rest[..start].trim().is_empty() {
                return Err(format!(
                    "unexpected text before quoted CSDF column: '{}'",
                    rest[..start].trim()
                ));
            }
            let after_start = &rest[start + 1..];
            let Some(end) = after_start.find('\'') else {
                return Err("unterminated quoted CSDF column".to_string());
            };
            columns.push(after_start[..end].to_string());
            rest = &after_start[end + 1..];
        }
    }

    pub(super) fn parse_csd_sweep_row_header(line: &str) -> Result<(f64, usize), String> {
        let fields = line.split_whitespace().collect::<Vec<_>>();
        if fields.len() != 3 || !fields[0].eq_ignore_ascii_case("#C") {
            return Err("expected '#C <sweep-value> <value-count>'".to_string());
        }
        let sweep_value = Self::parse_xyce_numeric_token(fields[1])
            .map_err(|err| format!("invalid sweep value '{}': {err}", fields[1]))?;
        let value_count = fields[2]
            .parse::<usize>()
            .map_err(|err| format!("invalid value count '{}': {err}", fields[2]))?;
        Ok((sweep_value, value_count))
    }

    pub(super) fn parse_csd_complex_value_token(
        token: &str,
        expected_position: usize,
    ) -> Result<Complex64, String> {
        let (value, position) = token
            .split_once(':')
            .ok_or_else(|| "expected '<value>:<position>'".to_string())?;
        let position = position
            .parse::<usize>()
            .map_err(|err| format!("invalid position '{}': {err}", position))?;
        if position != expected_position {
            return Err(format!(
                "position {position} does not match expected position {expected_position}"
            ));
        }
        let (real, imaginary) = value.split_once('/').unwrap_or((value, "0"));
        let real = Self::parse_xyce_numeric_token(real)
            .map_err(|err| format!("invalid real value '{}': {err}", real))?;
        let imaginary = Self::parse_xyce_numeric_token(imaginary)
            .map_err(|err| format!("invalid imaginary value '{}': {err}", imaginary))?;
        Ok(Complex64::new(real, imaginary))
    }

    pub(super) fn parse_raw_table(bytes: &[u8]) -> Result<XycePrnTable, String> {
        let mut offset = 0usize;
        let mut columns: Option<Vec<String>> = None;
        let mut rows = Vec::new();
        let mut plot_index = 0usize;

        while offset < bytes.len() {
            Self::skip_raw_blank_lines(bytes, &mut offset);
            if offset >= bytes.len() {
                break;
            }

            let (plot_columns, plot_rows) = Self::parse_raw_plot(bytes, &mut offset, plot_index)?;
            if let Some(columns) = &columns {
                if !Self::same_prn_columns(columns, &plot_columns) {
                    return Err(format!(
                        "Xyce RAW plot {} changes variables from {:?} to {:?}",
                        plot_index + 1,
                        columns,
                        plot_columns
                    ));
                }
            } else {
                columns = Some(plot_columns);
            }
            rows.extend(plot_rows);
            plot_index += 1;
        }

        let columns = columns.ok_or_else(|| "empty Xyce RAW table".to_string())?;
        if rows.is_empty() {
            return Err("Xyce RAW table has no data rows".to_string());
        }

        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_raw_plot(
        bytes: &[u8],
        offset: &mut usize,
        plot_index: usize,
    ) -> Result<(Vec<String>, Vec<Vec<f64>>), String> {
        let mut flags = None;
        let mut variable_count = None;
        let mut point_count = None;

        loop {
            let line = Self::read_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "Xyce RAW plot {} ended before Variables header",
                    plot_index + 1
                )
            })?;
            let trimmed = line.trim();
            if trimmed.is_empty()
                || trimmed.starts_with("Title:")
                || trimmed.starts_with("Date:")
                || trimmed.starts_with("Version:")
            {
                continue;
            }
            if trimmed.starts_with("Plotname:") {
                continue;
            }
            if let Some(value) = trimmed.strip_prefix("Flags:") {
                flags = Some(value.trim().to_string());
                continue;
            }
            if let Some(value) = trimmed.strip_prefix("No. Variables:") {
                variable_count = Some(value.trim().parse::<usize>().map_err(|err| {
                    format!(
                        "invalid RAW variable count '{}' in plot {}: {err}",
                        value.trim(),
                        plot_index + 1
                    )
                })?);
                continue;
            }
            if let Some(value) = trimmed.strip_prefix("No. Points:") {
                point_count = Some(value.trim().parse::<usize>().map_err(|err| {
                    format!(
                        "invalid RAW point count '{}' in plot {}: {err}",
                        value.trim(),
                        plot_index + 1
                    )
                })?);
                continue;
            }
            if trimmed.eq_ignore_ascii_case("Variables:") {
                break;
            }
            return Err(format!(
                "unexpected RAW header line in plot {}: '{}'",
                plot_index + 1,
                trimmed
            ));
        }

        let flags =
            flags.ok_or_else(|| format!("RAW plot {} has no Flags line", plot_index + 1))?;
        if !flags
            .split_whitespace()
            .any(|flag| flag.eq_ignore_ascii_case("real"))
        {
            return Err(format!(
                "RAW plot {} uses unsupported Flags: {flags}; only real-valued RAW is supported",
                plot_index + 1
            ));
        }
        if flags
            .split_whitespace()
            .any(|flag| flag.eq_ignore_ascii_case("complex"))
        {
            return Err(format!(
                "RAW plot {} is complex-valued; DC RAW comparison currently supports real data",
                plot_index + 1
            ));
        }
        let variable_count = variable_count
            .ok_or_else(|| format!("RAW plot {} has no variable count", plot_index + 1))?;
        let point_count =
            point_count.ok_or_else(|| format!("RAW plot {} has no point count", plot_index + 1))?;
        if variable_count == 0 {
            return Err(format!("RAW plot {} has no variables", plot_index + 1));
        }

        let mut columns = Vec::with_capacity(variable_count);
        for variable_index in 0..variable_count {
            let line = Self::read_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "RAW plot {} ended while reading variable {}",
                    plot_index + 1,
                    variable_index
                )
            })?;
            let fields = line.split_whitespace().collect::<Vec<_>>();
            if fields.len() < 3 {
                return Err(format!(
                    "invalid RAW variable line in plot {}: '{}'",
                    plot_index + 1,
                    line.trim()
                ));
            }
            let parsed_index = fields[0].parse::<usize>().map_err(|err| {
                format!(
                    "invalid RAW variable index '{}' in plot {}: {err}",
                    fields[0],
                    plot_index + 1
                )
            })?;
            if parsed_index != variable_index {
                return Err(format!(
                    "RAW variable index {} appears where {} was expected in plot {}",
                    parsed_index,
                    variable_index,
                    plot_index + 1
                ));
            }
            columns.push(fields[1].to_string());
        }

        let data_marker = loop {
            let line = Self::read_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "RAW plot {} ended before Values/Binary marker",
                    plot_index + 1
                )
            })?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            break trimmed.to_ascii_lowercase();
        };

        let rows = match data_marker.as_str() {
            "values:" => {
                Self::parse_ascii_raw_rows(bytes, offset, plot_index, point_count, variable_count)?
            }
            "binary:" => {
                Self::parse_binary_raw_rows(bytes, offset, plot_index, point_count, variable_count)?
            }
            other => {
                return Err(format!(
                    "RAW plot {} expected Values: or Binary:, got '{}'",
                    plot_index + 1,
                    other
                ));
            }
        };

        Ok((columns, rows))
    }

    pub(super) fn parse_ascii_raw_rows(
        bytes: &[u8],
        offset: &mut usize,
        plot_index: usize,
        point_count: usize,
        variable_count: usize,
    ) -> Result<Vec<Vec<f64>>, String> {
        let mut rows = Vec::with_capacity(point_count);
        for point_index in 0..point_count {
            let first_line = Self::read_next_nonempty_raw_line(bytes, offset).ok_or_else(|| {
                format!(
                    "RAW plot {} ended while reading point {}",
                    plot_index + 1,
                    point_index
                )
            })?;
            let first_fields = first_line.split_whitespace().collect::<Vec<_>>();
            if first_fields.is_empty() {
                return Err(format!(
                    "RAW plot {} has empty point line for point {}",
                    plot_index + 1,
                    point_index
                ));
            }
            let parsed_index = first_fields[0].parse::<usize>().map_err(|err| {
                format!(
                    "invalid RAW point index '{}' in plot {}: {err}",
                    first_fields[0],
                    plot_index + 1
                )
            })?;
            if parsed_index != point_index {
                return Err(format!(
                    "RAW point index {} appears where {} was expected in plot {}",
                    parsed_index,
                    point_index,
                    plot_index + 1
                ));
            }

            let mut row = first_fields
                .iter()
                .skip(1)
                .map(|token| {
                    Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid RAW numeric token '{}' at plot {}, point {}: {err}",
                            token,
                            plot_index + 1,
                            point_index
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            while row.len() < variable_count {
                let line = Self::read_next_nonempty_raw_line(bytes, offset).ok_or_else(|| {
                    format!(
                        "RAW plot {} ended while reading values for point {}",
                        plot_index + 1,
                        point_index
                    )
                })?;
                for token in line.split_whitespace() {
                    row.push(Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid RAW numeric token '{}' at plot {}, point {}: {err}",
                            token,
                            plot_index + 1,
                            point_index
                        )
                    })?);
                }
            }
            if row.len() != variable_count {
                return Err(format!(
                    "RAW plot {} point {} has {} values, expected {}",
                    plot_index + 1,
                    point_index,
                    row.len(),
                    variable_count
                ));
            }
            rows.push(row);
        }
        Ok(rows)
    }

    pub(super) fn parse_binary_raw_rows(
        bytes: &[u8],
        offset: &mut usize,
        plot_index: usize,
        point_count: usize,
        variable_count: usize,
    ) -> Result<Vec<Vec<f64>>, String> {
        let value_count = point_count
            .checked_mul(variable_count)
            .ok_or_else(|| format!("RAW plot {} point/variable count overflows", plot_index + 1))?;
        let byte_count = value_count
            .checked_mul(std::mem::size_of::<f64>())
            .ok_or_else(|| format!("RAW plot {} binary byte count overflows", plot_index + 1))?;
        if bytes.len().saturating_sub(*offset) < byte_count {
            return Err(format!(
                "RAW plot {} binary payload has {} byte(s), expected {}",
                plot_index + 1,
                bytes.len().saturating_sub(*offset),
                byte_count
            ));
        }

        let mut rows = Vec::with_capacity(point_count);
        for point_index in 0..point_count {
            let mut row = Vec::with_capacity(variable_count);
            for variable_index in 0..variable_count {
                let start = *offset
                    + (point_index * variable_count + variable_index) * std::mem::size_of::<f64>();
                let bytes: [u8; 8] = bytes[start..start + 8]
                    .try_into()
                    .expect("slice length checked");
                row.push(f64::from_le_bytes(bytes));
            }
            rows.push(row);
        }
        *offset += byte_count;
        Ok(rows)
    }

    pub(super) fn read_next_nonempty_raw_line(bytes: &[u8], offset: &mut usize) -> Option<String> {
        loop {
            let line = Self::read_raw_line(bytes, offset)?;
            if !line.trim().is_empty() {
                return Some(line);
            }
        }
    }

    pub(super) fn read_raw_line(bytes: &[u8], offset: &mut usize) -> Option<String> {
        if *offset >= bytes.len() {
            return None;
        }
        let start = *offset;
        let mut end = start;
        while end < bytes.len() && bytes[end] != b'\n' {
            end += 1;
        }
        *offset = end.saturating_add(usize::from(end < bytes.len()));
        let mut line = &bytes[start..end];
        if line.ends_with(b"\r") {
            line = &line[..line.len().saturating_sub(1)];
        }
        Some(String::from_utf8_lossy(line).into_owned())
    }

    pub(super) fn is_parameter_sweep_summary_reference(path: &Path) -> bool {
        let Ok(content) = fs::read_to_string(path) else {
            return false;
        };
        let mut lines = content
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty());
        let Some(header) = lines.next() else {
            return false;
        };
        header.to_ascii_uppercase().starts_with("STEP")
            && content
                .to_ascii_lowercase()
                .contains("end of xyce(tm) parameter sweep")
    }

    pub(super) fn parse_prn_table(content: &str) -> Result<XycePrnTable, String> {
        let nonempty_lines = content
            .lines()
            .enumerate()
            .map(|(line_number, line)| (line_number + 1, line.trim()))
            .filter(|(_, line)| !line.is_empty())
            .collect::<Vec<_>>();
        if nonempty_lines.is_empty() {
            return Err("empty Xyce .prn table".to_string());
        }

        let Some((header_index, (_header_line_number, header))) = nonempty_lines
            .iter()
            .enumerate()
            .find(|(_, (_, line))| Self::is_prn_header_line(line))
        else {
            return Err("Xyce .prn table has no header".to_string());
        };
        let delimiter = Self::prn_header_delimiter(header)
            .ok_or_else(|| format!("invalid Xyce .prn header '{}'", header))?;
        let columns = Self::parse_prn_columns(header, delimiter)?;
        if columns.is_empty() {
            return Err("Xyce .prn header has no columns".to_string());
        }

        let mut rows = Vec::new();
        for (line_number, line) in nonempty_lines.iter().skip(header_index + 1).copied() {
            if line.to_ascii_lowercase().starts_with("end of xyce") {
                break;
            }
            if !rows.is_empty() && Self::is_prn_footer_line(line) {
                break;
            }
            if Self::is_prn_separator_line(line) {
                continue;
            }
            if Self::is_prn_header_line(line) {
                let repeated_delimiter = Self::prn_header_delimiter(line)
                    .ok_or_else(|| format!("invalid repeated Xyce .prn header '{}'", line))?;
                let repeated_columns = Self::parse_prn_columns(line, repeated_delimiter)?;
                if Self::same_prn_columns(&columns, &repeated_columns) {
                    continue;
                }
                return Err(format!(
                    "Xyce .prn table changes columns at line {}; multi-table .prn output is not supported",
                    line_number
                ));
            }
            let values = Self::split_prn_fields(line, delimiter)
                .map(|token| {
                    Self::parse_xyce_numeric_token(token).map_err(|err| {
                        format!(
                            "invalid numeric token '{}' on data line {}: {err}",
                            token, line_number
                        )
                    })
                })
                .collect::<Result<Vec<_>, _>>()?;
            if values.len() != columns.len() {
                return Err(format!(
                    "data line {} has {} values, expected {}",
                    line_number,
                    values.len(),
                    columns.len()
                ));
            }
            rows.push(values);
        }

        if rows.is_empty() {
            return Err("Xyce .prn table has no data rows".to_string());
        }

        Ok(XycePrnTable { columns, rows })
    }

    pub(super) fn parse_prn_columns(
        line: &str,
        delimiter: XycePrnDelimiter,
    ) -> Result<Vec<String>, String> {
        match delimiter {
            XycePrnDelimiter::Whitespace => Self::split_prn_header_fields(line),
            XycePrnDelimiter::Comma => {
                Ok(line.split(',').map(str::trim).map(str::to_string).collect())
            }
        }
    }

    pub(super) fn is_prn_header_line(line: &str) -> bool {
        Self::prn_header_delimiter(line).is_some()
    }

    pub(super) fn prn_header_delimiter(line: &str) -> Option<XycePrnDelimiter> {
        if line
            .split(',')
            .next()
            .is_some_and(|token| Self::is_prn_metadata_header_token(token.trim()))
        {
            return Some(XycePrnDelimiter::Comma);
        }
        let whitespace_fields = Self::split_prn_header_fields(line).ok()?;
        if whitespace_fields
            .first()
            .is_some_and(|token| Self::is_prn_metadata_header_token(token))
        {
            return Some(XycePrnDelimiter::Whitespace);
        }
        let comma_fields = line.split(',').map(str::trim).collect::<Vec<_>>();
        if comma_fields.len() > 1
            && comma_fields
                .first()
                .is_some_and(|token| Self::looks_like_reference_probe_header(token))
        {
            return Some(XycePrnDelimiter::Comma);
        }
        if whitespace_fields.len() > 1
            && whitespace_fields
                .first()
                .is_some_and(|token| Self::looks_like_reference_probe_header(token))
        {
            return Some(XycePrnDelimiter::Whitespace);
        }
        None
    }

    pub(super) fn is_prn_metadata_header_token(token: &str) -> bool {
        token.eq_ignore_ascii_case("index") || token.eq_ignore_ascii_case("stepnum")
    }

    pub(super) fn reference_columns_are_compact_probe_table(reference: &XycePrnTable) -> bool {
        reference
            .columns
            .first()
            .is_some_and(|column| Self::looks_like_reference_probe_header(column))
    }

    pub(super) fn looks_like_reference_probe_header(token: &str) -> bool {
        let normalized = Self::normalize_probe(token);
        normalized == "temp"
            || normalized == "time"
            || normalized == "freq"
            || normalized == "frequency"
            || normalized == "sweep"
            || normalized.starts_with("v(")
            || normalized.starts_with("i(")
            || normalized.starts_with("n(")
            || Self::compact_reference_probe_alias(&normalized).is_some()
    }

    pub(super) fn split_prn_fields(
        line: &str,
        delimiter: XycePrnDelimiter,
    ) -> Box<dyn Iterator<Item = &str> + '_> {
        match delimiter {
            XycePrnDelimiter::Whitespace => Box::new(line.split_whitespace()),
            XycePrnDelimiter::Comma => Box::new(line.split(',').map(str::trim)),
        }
    }

    pub(super) fn same_prn_columns(left: &[String], right: &[String]) -> bool {
        left.len() == right.len()
            && left
                .iter()
                .zip(right.iter())
                .all(|(left, right)| left.eq_ignore_ascii_case(right))
    }

    pub(super) fn is_prn_separator_line(line: &str) -> bool {
        line.chars()
            .all(|ch| ch == '-' || ch == '=' || ch.is_whitespace())
    }

    pub(super) fn is_prn_footer_line(line: &str) -> bool {
        let normalized = line.to_ascii_lowercase();
        normalized.starts_with("cpu time")
            || normalized.starts_with("total cpu time")
            || normalized.starts_with("current dynamic memory usage")
            || normalized.starts_with("dynamic memory limit")
    }

    pub(super) fn static_prn_reference_path(&self, deck_path: &Path) -> Option<PathBuf> {
        self.static_output_reference_path(deck_path, "prn")
    }

    pub(super) fn measurement_reference_paths(
        &self,
        deck_path: &Path,
        analysis_prefix: &str,
    ) -> Result<Vec<PathBuf>, String> {
        let first = self
            .static_output_reference_path(deck_path, &format!("{analysis_prefix}0"))
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        let Some(parent) = first.parent() else {
            return Err("measurement artifact path has no parent directory".to_string());
        };
        if !parent.is_dir() {
            return Ok(Vec::new());
        }
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "deck filename is not valid UTF-8".to_string())?;
        let file_prefix = format!("{deck_name}.{analysis_prefix}");
        let mut indexed = BTreeMap::new();
        for entry in fs::read_dir(parent)
            .map_err(|err| format!("failed to scan {}: {err}", parent.display()))?
        {
            let entry =
                entry.map_err(|err| format!("failed to scan {}: {err}", parent.display()))?;
            let name = entry.file_name();
            let Some(name) = name.to_str() else {
                continue;
            };
            let Some(raw_index) = name.strip_prefix(&file_prefix) else {
                continue;
            };
            if raw_index.is_empty() || !raw_index.chars().all(|ch| ch.is_ascii_digit()) {
                continue;
            }
            let index = raw_index.parse::<usize>().map_err(|err| {
                format!("measurement artifact index '{raw_index}' is invalid: {err}")
            })?;
            if indexed.insert(index, entry.path()).is_some() {
                return Err(format!(
                    "duplicate measurement artifact index {index} for {}",
                    deck_path.display()
                ));
            }
        }
        for (expected, actual) in indexed.keys().copied().enumerate() {
            if expected != actual {
                return Err(format!(
                    "measurement artifacts for {} are not contiguous: expected index {expected}, found {actual}",
                    deck_path.display()
                ));
            }
        }
        Ok(indexed.into_values().collect())
    }

    pub(super) fn continuous_measurement_reference_paths(
        &self,
        deck_path: &Path,
        netlist: &Netlist,
        analysis: &str,
        artifact_prefix: &str,
    ) -> Result<Vec<PathBuf>, String> {
        let declarations = netlist
            .measurements
            .iter()
            .filter(|measurement| {
                measurement.analysis.eq_ignore_ascii_case(analysis)
                    && measurement.print_policy == rspice_core::analysis::MeasurePrintPolicy::All
            })
            .collect::<Vec<_>>();
        if declarations.is_empty() {
            return Ok(Vec::new());
        }

        let output_directory = self
            .static_output_reference_path(deck_path, &format!("{artifact_prefix}0"))
            .and_then(|path| path.parent().map(Path::to_path_buf))
            .ok_or_else(|| "deck is not under tests/xyce/Netlists".to_string())?;
        let deck_name = deck_path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "deck filename is not valid UTF-8".to_string())?;
        let prefix = format!("{deck_name}_");
        let suffix = format!(".{artifact_prefix}0");
        let mut available = BTreeMap::<String, PathBuf>::new();
        if output_directory.is_dir() {
            for entry in fs::read_dir(&output_directory)
                .map_err(|err| format!("failed to scan {}: {err}", output_directory.display()))?
            {
                let entry = entry.map_err(|err| {
                    format!("failed to scan {}: {err}", output_directory.display())
                })?;
                let Some(file_name) = entry.file_name().to_str().map(str::to_owned) else {
                    continue;
                };
                let Some(raw_measurement) = file_name
                    .strip_prefix(&prefix)
                    .and_then(|name| name.strip_suffix(&suffix))
                else {
                    continue;
                };
                if raw_measurement.is_empty() {
                    return Err(format!(
                        "continuous measurement artifact '{}' has an empty measurement name",
                        entry.path().display()
                    ));
                }
                let normalized = raw_measurement.to_ascii_uppercase();
                if available.insert(normalized.clone(), entry.path()).is_some() {
                    return Err(format!(
                        "duplicate continuous measurement artifact for '{normalized}'"
                    ));
                }
            }
        }

        let mut paths = Vec::with_capacity(declarations.len());
        for declaration in declarations {
            let normalized = declaration.name.to_ascii_uppercase();
            let path = available.remove(&normalized).ok_or_else(|| {
                format!(
                    "{analysis} measurement '{}' has no checked-in sidecar artifact",
                    declaration.name,
                )
            })?;
            paths.push(path);
        }
        if !available.is_empty() {
            return Err(format!(
                "unclaimed continuous measurement sidecar artifact(s): {}",
                available
                    .values()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        Ok(paths)
    }

    pub(super) fn tran_gsfile_reference_path(deck_path: &Path) -> Option<PathBuf> {
        let candidate = deck_path.with_extension("cir.GSfile");
        candidate.is_file().then_some(candidate)
    }

    pub(super) fn static_output_reference_path(
        &self,
        deck_path: &Path,
        extension: &str,
    ) -> Option<PathBuf> {
        let netlists_root = self.root.join("Netlists");
        let canonical = deck_path
            .canonicalize()
            .unwrap_or_else(|_| deck_path.to_path_buf());
        let relative = canonical
            .strip_prefix(&netlists_root)
            .or_else(|_| deck_path.strip_prefix(&netlists_root))
            .ok()?;
        let parent = relative.parent().unwrap_or_else(|| Path::new(""));
        let file_name = relative.file_name()?.to_string_lossy();
        Some(
            self.root
                .join("OutputData")
                .join(parent)
                .join(format!("{file_name}.{extension}")),
        )
    }

    pub(super) fn load_upstream_wrapper_decks(root: &Path) -> BTreeSet<String> {
        let manifest_path = root.join(HARNESS_MANIFEST_FILE);
        let Ok(content) = fs::read_to_string(manifest_path) else {
            return BTreeSet::new();
        };

        let mut decks = BTreeSet::new();
        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((path, contract)) = line.split_once('\t') else {
                continue;
            };
            if contract.trim() == REQUIRES_UPSTREAM_WRAPPER_CONTRACT {
                decks.insert(Self::normalize_manifest_key(path));
            }
        }
        decks
    }

    pub(super) fn parse_bjt_external_node_member_file_name(
        file_name: &str,
    ) -> Option<(String, u8)> {
        let stem = file_name.strip_suffix(".cir")?;
        let member_index = stem.chars().last()?.to_digit(10)? as u8;
        let family = &stem[..stem.len() - 1];
        if family.is_empty()
            || family.chars().last().is_some_and(|ch| ch.is_ascii_digit())
            || !matches!(member_index, 1 | 2)
        {
            return None;
        }
        Some((family.to_string(), member_index))
    }

    pub(super) fn parse_subckt_wrapper_file_name(file_name: &str) -> Option<String> {
        let stem = file_name.strip_suffix(".cir")?;
        let family = stem.strip_prefix("subckt_")?;
        if !family.is_empty()
            && family
                .chars()
                .all(|ch| ch.is_ascii_alphabetic() || ch == '_')
        {
            Some(family.to_string())
        } else {
            None
        }
    }

    pub(super) fn parse_subckt_family_member_file_name(file_name: &str) -> Option<String> {
        let stem = file_name.strip_suffix(".cir")?;
        let rest = stem.strip_prefix("subckt_")?;
        let digit_index = rest.find(|ch: char| ch.is_ascii_digit())?;
        let family = &rest[..digit_index];
        if family.is_empty() {
            return None;
        }
        let suffix = &rest[digit_index + 1..];
        if !matches!(suffix, "" | "_hs" | "_dup") {
            return None;
        }
        Some(family.to_string())
    }

    pub(super) fn parse_voltage_probe(probe: &str) -> Option<XyceVoltageProbe> {
        let normalized = Self::normalize_probe(probe);
        let open_index = normalized.find('(')?;
        if !normalized.ends_with(')') {
            return None;
        }
        let accessor = XyceVoltageAccessor::from_function_name(&normalized[..open_index])?;
        let inner = &normalized[open_index + 1..normalized.len() - 1];
        if inner.is_empty() {
            return None;
        }
        let (node_pos, node_neg) = if let Some((a, b)) = inner.split_once(',') {
            (a.to_string(), Some(b.to_string()))
        } else {
            (inner.to_string(), None)
        };
        Some(XyceVoltageProbe {
            accessor,
            node_pos,
            node_neg,
        })
    }

    pub(super) fn parse_tran_voltage_probe(probe: &str) -> Option<XyceVoltageProbe> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("n(") {
            return Self::parse_voltage_probe(&normalized);
        }
        if !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if inner.is_empty() || inner.contains(':') {
            return None;
        }
        let (node_pos, node_neg) = if let Some((a, b)) = inner.split_once(',') {
            (a.to_string(), Some(b.to_string()))
        } else {
            (inner.to_string(), None)
        };
        Some(XyceVoltageProbe {
            accessor: XyceVoltageAccessor::Value,
            node_pos,
            node_neg,
        })
    }

    pub(super) fn parse_ac_voltage_probe(probe: &str) -> Option<XyceVoltageProbe> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("n(") {
            return Self::parse_voltage_probe(&normalized);
        }
        if !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        if inner.is_empty() || inner.contains(':') {
            return None;
        }
        let (node_pos, node_neg) = if let Some((a, b)) = inner.split_once(',') {
            (a.to_string(), Some(b.to_string()))
        } else {
            (inner.to_string(), None)
        };
        Some(XyceVoltageProbe {
            accessor: XyceVoltageAccessor::Value,
            node_pos,
            node_neg,
        })
    }

    pub(super) fn parse_current_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("i(") || !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        (!inner.is_empty()).then(|| inner.to_string())
    }

    pub(super) fn parse_ac_current_probe(probe: &str) -> Option<XyceAcCurrentProbe> {
        let normalized = Self::normalize_probe(probe);
        let open_index = normalized.find('(')?;
        if !normalized.ends_with(')') {
            return None;
        }
        let accessor = XyceCurrentAccessor::from_function_name(&normalized[..open_index])?;
        let inner = &normalized[open_index + 1..normalized.len() - 1];
        (!inner.is_empty()).then(|| XyceAcCurrentProbe {
            accessor,
            element_name: inner.to_string(),
        })
    }

    pub(super) fn parse_power_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        if !(normalized.starts_with("p(") || normalized.starts_with("w("))
            || !normalized.ends_with(')')
        {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        (!inner.is_empty()).then(|| inner.to_string())
    }

    pub(super) fn parse_lead_current_probe(probe: &str) -> Option<XyceLeadCurrentProbe> {
        let normalized = Self::normalize_probe(probe);
        for function in ["id", "ig", "is", "ib", "ic", "ie"] {
            let prefix = format!("{function}(");
            if !normalized.starts_with(&prefix) || !normalized.ends_with(')') {
                continue;
            }
            let inner = &normalized[prefix.len()..normalized.len() - 1];
            if inner.is_empty() {
                return None;
            }
            return Some(XyceLeadCurrentProbe {
                terminal: XyceLeadCurrentTerminal::from_function_name(function)?,
                element_name: inner.to_string(),
            });
        }
        None
    }

    pub(super) fn parse_device_operating_point_probe(probe: &str) -> Option<(String, String)> {
        let normalized = Self::normalize_probe(probe);
        if !normalized.starts_with("n(") || !normalized.ends_with(')') {
            return None;
        }
        let inner = &normalized[2..normalized.len() - 1];
        let (element, parameter) = if let Some((element, parameter)) = inner.rsplit_once(':') {
            (element, parameter)
        } else {
            let (element, parameter) = inner.rsplit_once('_')?;
            if !element.to_ascii_lowercase().starts_with("ymin!") {
                return None;
            }
            (element, parameter)
        };
        if element.is_empty() || parameter.is_empty() {
            return None;
        }
        Some((element.to_string(), parameter.to_string()))
    }

    pub(super) fn parse_device_parameter_probe(probe: &str) -> Option<(String, String)> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        let (element, parameter) = unwrapped.rsplit_once(':')?;
        if element.is_empty() || parameter.is_empty() {
            return None;
        }
        if element
            .chars()
            .chain(parameter.chars())
            .any(|ch| matches!(ch, '(' | ')' | '+' | '-' | '*' | '/' | '^' | ','))
        {
            return None;
        }
        Some((element.to_string(), parameter.to_string()))
    }

    pub(super) fn parse_bare_device_parameter_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        if unwrapped.is_empty()
            || unwrapped.starts_with(':')
            || unwrapped.ends_with(':')
            || unwrapped.contains("::")
            || unwrapped
                .chars()
                .any(|ch| matches!(ch, '(' | ')' | '+' | '-' | '*' | '/' | '^' | ',' | '='))
        {
            return None;
        }
        let first = unwrapped.chars().next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        Some(unwrapped.to_string())
    }

    pub(super) fn parse_scalar_parameter_probe(probe: &str) -> Option<String> {
        let normalized = Self::normalize_probe(probe);
        let unwrapped = normalized
            .strip_prefix('{')
            .and_then(|value| value.strip_suffix('}'))
            .unwrap_or(&normalized);
        if unwrapped.is_empty()
            || unwrapped.contains(':')
            || unwrapped.contains('(')
            || unwrapped.contains(')')
            || unwrapped
                .chars()
                .any(|ch| matches!(ch, '+' | '-' | '*' | '/' | '^' | ','))
        {
            return None;
        }
        let mut chars = unwrapped.chars();
        let first = chars.next()?;
        if !(first.is_ascii_alphabetic() || first == '_') {
            return None;
        }
        if chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.') {
            Some(unwrapped.to_string())
        } else {
            None
        }
    }
}
