//! Netlist and model-card helpers used across contracts.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn parsed_netlist_has_ac_frequency_dependent_global(netlist: &Netlist) -> bool {
        netlist
            .params
            .all_global_expressions()
            .into_iter()
            .any(|(_, expression)| {
                prepare_behavioral_expression(&expression, &netlist.params)
                    .ok()
                    .is_some_and(|prepared| {
                        behavioral_expression_references_unbound_frequency(
                            &prepared,
                            &netlist.params,
                        )
                    })
            })
    }

    pub(super) fn is_blank_include_wrapper_directive(line: &str) -> bool {
        Self::blank_wrapper_directive_matches(line, &[".include", ".inc", ".incl"])
    }

    pub(super) fn is_blank_lib_wrapper_directive(line: &str) -> bool {
        Self::blank_wrapper_directive_matches(line, &[".lib"])
    }

    pub(super) fn strict_level9_xyce_verify_source_precision(
        source: &str,
        expected_model_count: usize,
    ) -> Result<Option<usize>, String> {
        let lines = Self::logical_netlist_lines(source);
        if lines.is_empty() {
            return Ok(None);
        }
        let mut model_count = 0usize;
        let mut tran_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        let mut scientific_precision = None;
        let mut saw_end = false;
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            if stripped.is_empty() {
                continue;
            }
            if saw_end {
                return Ok(None);
            }
            let fields = Self::split_print_fields(stripped)?;
            let Some(command) = fields.first() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".model" => model_count += 1,
                ".tran" => {
                    tran_count += 1;
                    if fields.len() != 3 {
                        return Ok(None);
                    }
                }
                ".print" => {
                    print_count += 1;
                    scientific_precision =
                        Some(Self::strict_level9_xyce_verify_print_precision(&fields)?);
                }
                ".end" => {
                    end_count += 1;
                    if fields.len() != 1 {
                        return Ok(None);
                    }
                    saw_end = true;
                }
                _ => return Ok(None),
            }
        }
        if model_count != expected_model_count
            || tran_count != 1
            || print_count != 1
            || end_count != 1
        {
            return Ok(None);
        }
        Ok(scientific_precision)
    }

    pub(super) fn strict_level9_xyce_verify_print_precision(
        fields: &[String],
    ) -> Result<usize, String> {
        if fields.len() < 3
            || !fields[0].eq_ignore_ascii_case(".PRINT")
            || !fields[1].eq_ignore_ascii_case("TRAN")
        {
            return Err(
                "integrated-RMS output requires one ordinary .PRINT TRAN statement".to_string(),
            );
        }
        let field_refs = fields.iter().map(String::as_str).collect::<Vec<_>>();
        let mut precision = None;
        let mut width = None;
        let mut format = None;
        let mut filter = None;
        let mut time_scale_factor = None;
        let mut saw_probe = false;
        let mut index = 2usize;
        while index < fields.len() {
            if let Some((raw_key, raw_value, consumed)) =
                Self::print_option_assignment(&field_refs, index)
            {
                if saw_probe {
                    return Err(format!(
                        "integrated-RMS .PRINT TRAN does not admit option assignment '{raw_key}' after the first probe"
                    ));
                }
                let key = raw_key.trim().to_ascii_lowercase();
                let verifier_value = raw_value.trim();
                if !verifier_value
                    .chars()
                    .next()
                    .is_some_and(|first| first.is_ascii_alphanumeric() || first == '_')
                {
                    return Err(format!(
                        "integrated-RMS .PRINT TRAN option '{raw_key}' is not consumable by Release 7.10 xyce_verify.pl: '{raw_value}'"
                    ));
                }
                let value = verifier_value;
                match key.as_str() {
                    "precision" => precision = Some(value.to_string()),
                    "width" => width = Some(value.to_string()),
                    "format" => {
                        format = Some(value.to_string());
                    }
                    "filter" => filter = Some(value.to_string()),
                    "timescalefactor" => time_scale_factor = Some(value.to_string()),
                    _ => {
                        return Err(format!(
                            "integrated-RMS .PRINT TRAN does not admit output option '{raw_key}'"
                        ));
                    }
                }
                index += consumed;
                continue;
            }

            let normalized = field_refs[index].to_ascii_lowercase();
            if Self::is_print_option_token(&normalized)
                || matches!(
                    normalized.as_str(),
                    "file"
                        | "format"
                        | "width"
                        | "precision"
                        | "delimiter"
                        | "noindex"
                        | "index"
                        | "filter"
                        | "timescalefactor"
                )
            {
                return Err(format!(
                    "integrated-RMS .PRINT TRAN does not admit output option '{}'",
                    fields[index]
                ));
            }
            saw_probe = true;
            index += 1;
        }

        if !saw_probe {
            return Err("integrated-RMS .PRINT TRAN contains no probes".to_string());
        }
        let parse_effective_integer_option = |name: &str,
                                              raw: Option<&str>|
         -> Result<Option<i32>, String> {
            raw.map(|value| {
                    let parsed = rspice_core::netlist::lexer::parse_spice_value(value).map_err(|err| {
                        format!(
                            "integrated-RMS .PRINT TRAN {name} must be numeric: '{value}': {err}"
                        )
                    })?;
                    if !parsed.is_finite()
                        || parsed < f64::from(i32::MIN)
                        || parsed > f64::from(i32::MAX)
                    {
                        return Err(format!(
                            "integrated-RMS .PRINT TRAN {name} must be a finite Xyce integer value in the i32 range: '{value}'"
                        ));
                    }
                    Ok(parsed as i32)
                })
                .transpose()
        };
        let effective_precision =
            parse_effective_integer_option("PRECISION", precision.as_deref())?;
        if effective_precision != Some(12) {
            return Err(format!(
                "integrated-RMS .PRINT TRAN requires explicit effective PRECISION=12, got {effective_precision:?}"
            ));
        }
        let _effective_width = parse_effective_integer_option("WIDTH", width.as_deref())?;
        if format
            .as_deref()
            .is_some_and(|value| !value.eq_ignore_ascii_case("STD"))
        {
            return Err(format!(
                "integrated-RMS .PRINT TRAN requires effective FORMAT=STD, got {format:?}"
            ));
        }
        let parse_finite_option =
            |name: &str, raw: Option<&str>| -> Result<Option<Value>, String> {
                raw.map(|value| {
                    let parsed =
                    rspice_core::netlist::lexer::parse_spice_value(value).map_err(|err| {
                        format!(
                            "integrated-RMS .PRINT TRAN {name} must be numeric: '{value}': {err}"
                        )
                    })?;
                    if !parsed.is_finite() {
                        return Err(format!("integrated-RMS .PRINT TRAN {name} must be finite"));
                    }
                    Ok(parsed)
                })
                .transpose()
            };
        let effective_filter = parse_finite_option("FILTER", filter.as_deref())?;
        if effective_filter.is_some_and(|value| value != 0.0) {
            return Err(format!(
                "integrated-RMS .PRINT TRAN requires effective FILTER=0, got {effective_filter:?}"
            ));
        }
        let effective_time_scale =
            parse_finite_option("TIMESCALEFACTOR", time_scale_factor.as_deref())?;
        if effective_time_scale.is_some_and(|value| value != 1.0) {
            return Err(format!(
                "integrated-RMS .PRINT TRAN requires effective TIMESCALEFACTOR=1, got {effective_time_scale:?}"
            ));
        }
        Ok(12)
    }

    pub(super) fn is_native_csv_dc_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        if !Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/") {
            return false;
        }
        Self::dc_print_output_requests(source).is_ok_and(|requests| {
            requests.into_iter().any(|request| {
                request.file.is_none()
                    && request
                        .format
                        .as_deref()
                        .is_some_and(|format| format.eq_ignore_ascii_case("CSV"))
            })
        })
    }

    pub(super) fn is_native_csd_dc_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        if !Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dc/") {
            return false;
        }
        Self::dc_print_output_requests(source).is_ok_and(|requests| {
            requests.into_iter().any(|request| {
                request.file.is_none()
                    && request
                        .format
                        .as_deref()
                        .is_some_and(|format| format.eq_ignore_ascii_case("PROBE"))
            })
        })
    }

    pub(super) fn is_native_no_output_dc_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        matches!(
            Self::normalize_manifest_key(relative_path).as_str(),
            "netlists/output/dc/dc-noprn.cir"
        ) && Self::validate_no_output_dc_wrapper_source(source).is_ok()
    }

    pub(super) fn is_native_raw_wrapper_candidate_path(relative_path: &str) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        relative_path.starts_with("netlists/output/dc/")
            && relative_path
                .rsplit('/')
                .next()
                .is_some_and(|file_name| file_name.contains("-raw"))
    }

    pub(super) fn is_native_output_override_wrapper_candidate_path(relative_path: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/dasho/")
    }

    pub(super) fn is_native_hspice_math_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        let normalized_source = source.to_ascii_lowercase();
        relative_path.starts_with("netlists/parser/")
            && normalized_source.contains("-hspice-ext math")
            && normalized_source.contains("-hspice-ext all")
    }

    pub(super) fn is_native_hspice_random_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        let normalized_source = source.to_ascii_lowercase();
        relative_path.starts_with("netlists/parser/")
            && ["agauss(", "gauss(", "aunif(", "unif(", "rand("]
                .iter()
                .any(|operator| normalized_source.contains(operator))
    }

    pub(super) fn is_native_resistor_default_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        let normalized_source = source.to_ascii_lowercase();
        relative_path.starts_with("netlists/resistor/")
            && normalized_source.contains("default to 1000")
            && normalized_source.contains("warning")
    }

    pub(super) fn is_native_resistor_temperature_step_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        if !relative_path.starts_with("netlists/resistor_td/") {
            return false;
        }

        let mut has_step_temp = false;
        let mut has_tce = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.starts_with(".step ") && trimmed.contains("temp") {
                has_step_temp = true;
            }
            if trimmed.contains("tce") {
                has_tce = true;
            }
        }
        has_step_temp && has_tce
    }

    pub(super) fn is_native_semiconductor_resistor_step_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        let relative_path = Self::normalize_manifest_key(relative_path);
        if relative_path != "netlists/semic_resistor/semic_resistor_step.cir" {
            return false;
        }

        let mut has_resistor_geometry_step = false;
        let mut has_resistor_default_step = false;
        let mut has_resistor_model = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.starts_with(".step ") {
                has_resistor_geometry_step |= trimmed.contains("r1:l") || trimmed.contains("r2:l");
                has_resistor_default_step |= trimmed
                    .split_whitespace()
                    .nth(1)
                    .is_some_and(|target| target.eq_ignore_ascii_case("r3"));
            }
            if trimmed.starts_with(".model ")
                && trimmed
                    .split_whitespace()
                    .nth(2)
                    .is_some_and(|model_type| model_type.eq_ignore_ascii_case("r"))
            {
                has_resistor_model = true;
            }
        }

        has_resistor_geometry_step && has_resistor_default_step && has_resistor_model
    }

    pub(super) fn is_native_top_level_execution_dir_wrapper_candidate(
        deck_path: &Path,
        source: &str,
    ) -> bool {
        Self::logical_netlist_lines(source)
            .iter()
            .all(|line| Self::strip_netlist_comment(line).trim().is_empty())
            && Self::top_level_execution_deck_path(deck_path).is_ok_and(|path| path.is_file())
    }

    /// Recover the wrapper owner for a retained `top_level/` worker deck.
    ///
    /// Some upstream Xyce regression wrappers keep a comments-only owner at
    /// the family root and execute a same-named deck below `top_level/` while
    /// retaining the family root as the process working directory.  The
    /// nested deck is also retained by the corpus census, so independently
    /// qualifying that record must preserve both the wrapper's execution
    /// directory and its owner-relative numerical oracle.  This relation is
    /// accepted only when the harness manifest authenticates the sibling
    /// owner; an arbitrary nested deck must never inherit a parent directory
    /// or a sibling oracle by filename coincidence alone.
    pub(super) fn top_level_execution_worker_owner(
        &self,
        deck: &XyceDeck,
    ) -> Result<Option<PathBuf>, String> {
        let Some(worker_directory) = deck.path.parent() else {
            return Ok(None);
        };
        if !worker_directory
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("top_level"))
        {
            return Ok(None);
        }
        let Some(family_directory) = worker_directory.parent() else {
            return Ok(None);
        };
        let Some(file_name) = deck.path.file_name() else {
            return Ok(None);
        };
        let owner = family_directory.join(file_name);
        let owner_relative = self.relative_key(&owner);
        if !self.requires_upstream_wrapper(&owner_relative) {
            return Ok(None);
        }

        let metadata = fs::symlink_metadata(&owner).map_err(|error| {
            format!(
                "top-level execution-directory wrapper owner {} is unavailable: {error}",
                self.display_path(&owner)
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "top-level execution-directory wrapper owner {} must be a regular non-symlink file",
                self.display_path(&owner)
            ));
        }
        if !Self::same_path(&Self::top_level_execution_deck_path(&owner)?, &deck.path) {
            return Err(format!(
                "top-level execution-directory worker {} does not match authenticated owner {}",
                self.display_path(&deck.path),
                self.display_path(&owner)
            ));
        }

        let owner_source = fs::read_to_string(&owner).map_err(|error| {
            format!(
                "failed to read top-level execution-directory wrapper owner {}: {error}",
                self.display_path(&owner)
            )
        })?;
        if !Self::logical_netlist_lines(&owner_source)
            .iter()
            .all(|line| Self::strip_netlist_comment(line).trim().is_empty())
        {
            return Err(format!(
                "top-level execution-directory wrapper owner {} must contain only comments and whitespace",
                self.display_path(&owner)
            ));
        }

        Ok(Some(owner))
    }

    pub(super) fn is_native_absolute_inc_lib_wrapper_candidate(
        deck_path: &Path,
        source: &str,
    ) -> bool {
        if !Self::source_has_absolute_inc_lib_wrapper_bindings(source) {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };
        let Ok(netlist) = Self::parse_xyce_netlist(source, deck_path) else {
            return false;
        };
        let Ok(dc) = Self::single_dc_sweep(&netlist) else {
            return false;
        };
        Self::validate_static_dc_contract(&netlist, &dc, &print).is_ok()
    }

    pub(super) fn is_native_step_data_wrapper_candidate(source: &str) -> bool {
        let mut has_data_table = false;
        let mut has_step_data = false;
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.starts_with(".data ") {
                has_data_table = true;
            }
            if trimmed.starts_with(".step ") && trimmed.contains("data") && trimmed.contains('=') {
                has_step_data = true;
            }
        }
        has_data_table && has_step_data
    }

    pub(super) fn is_native_dc_data_table_wrapper_candidate(source: &str) -> bool {
        Self::dc_data_table_names(source).is_ok_and(|names| !names.is_empty())
    }

    pub(super) fn is_native_gnuplot_splot_wrapper_candidate(source: &str) -> bool {
        if Self::wrapper_source_has_extra_output_analysis(source) {
            return false;
        }
        let Ok((primary, side)) = Self::gnuplot_splot_print_pair(source) else {
            return false;
        };
        primary.probes == side.probes
    }

    pub(super) fn is_native_empty_wildcard_lead_current_wrapper_candidate(
        deck_path: &Path,
        source: &str,
    ) -> bool {
        if Self::validate_default_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };
        let wildcard_probes = print
            .probes
            .iter()
            .filter_map(|probe| Self::parse_lead_current_probe(probe))
            .filter(|probe| probe.element_name == "*")
            .collect::<Vec<_>>();
        if wildcard_probes.is_empty() {
            return false;
        }
        let Ok(netlist) = Self::parse_xyce_netlist(source, deck_path) else {
            return false;
        };

        wildcard_probes
            .iter()
            .all(|probe| Self::lead_current_probe_is_omitted_empty_wildcard(&netlist, probe))
    }

    pub(super) fn is_native_subcircuit_node_probe_wrapper_candidate(source: &str) -> bool {
        if Self::validate_default_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };

        print
            .probes
            .iter()
            .any(|probe| Self::print_probe_contains_subcircuit_node_voltage_probe(probe))
    }

    pub(super) fn is_native_voltage_accessor_wrapper_candidate(source: &str) -> bool {
        if Self::validate_default_prn_wrapper_source(source).is_err() {
            return false;
        }
        let Ok(print) = Self::single_dc_print_request(source) else {
            return false;
        };

        print.probes.iter().any(|probe| {
            let expression = Self::print_expression_inner(probe).unwrap_or(probe);
            let normalized = Self::normalize_probe(expression);
            Self::parse_voltage_probe(&normalized)
                .is_some_and(|probe| probe.accessor != XyceVoltageAccessor::Value)
                || Self::print_expression_contains_voltage_accessor_call(expression)
        })
    }

    pub(super) fn models_form_single_binned_native_mos_family(
        models: &[rspice_core::netlist::ModelDef],
    ) -> bool {
        if models.len() < 2 {
            return false;
        }
        let Some(first_base) = Self::binned_model_base_name(&models[0].name) else {
            return false;
        };
        let first_type = models[0].model_type.to_ascii_uppercase();
        if !matches!(first_type.as_str(), "NMOS" | "PMOS") {
            return false;
        }
        models.iter().all(|model| {
            model.model_type.eq_ignore_ascii_case(&first_type)
                && Self::binned_model_base_name(&model.name)
                    .is_some_and(|base| base.eq_ignore_ascii_case(first_base))
                && Self::model_has_numeric_geometry_bin_range(model)
        })
    }

    pub(super) fn binned_model_base_name(name: &str) -> Option<&str> {
        let (base, suffix) = name.split_once('.')?;
        (!base.is_empty() && !suffix.is_empty()).then_some(base)
    }

    pub(super) fn model_has_numeric_geometry_bin_range(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        const BIN_PARAMS: [&str; 6] = ["LMIN", "LMAX", "WMIN", "WMAX", "NFINMIN", "NFINMAX"];
        model
            .params
            .iter()
            .any(|(name, _)| BIN_PARAMS.iter().any(|bin| name.eq_ignore_ascii_case(bin)))
    }

    pub(super) fn is_native_noindex_header_tran_wrapper_candidate(
        relative_path: &str,
        source: &str,
    ) -> bool {
        // Release 7.10's removed BUG_61 wrapper does not compare waveform
        // values. It runs this deck and inspects only the first default-PRN
        // header line for TIME without Index.
        Self::normalize_manifest_key(relative_path)
            == "netlists/certification_tests/bug_61/capacitor.cir"
            && Self::validate_native_noindex_header_tran_wrapper_contract(source).is_ok()
    }

    pub(super) fn is_native_csd_tran_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/tran/")
            && Self::validate_native_static_csd_tran_wrapper_contract(source).is_ok()
    }

    pub(super) fn is_native_csv_tran_wrapper_candidate(relative_path: &str, source: &str) -> bool {
        Self::normalize_manifest_key(relative_path).starts_with("netlists/output/tran/")
            && Self::validate_native_static_csv_tran_wrapper_contract(source).is_ok()
    }

    pub(super) fn is_native_output_initial_interval_tran_wrapper_candidate(source: &str) -> bool {
        Self::validate_native_output_initial_interval_tran_wrapper_contract(source).is_ok()
    }

    pub(super) fn is_extra_wrapper_ac_output_analysis_command(command: &str) -> bool {
        matches!(
            command.to_ascii_lowercase().as_str(),
            ".dc" | ".four" | ".hb" | ".lin" | ".noise" | ".probe" | ".save" | ".sens" | ".tran"
        )
    }

    pub(super) fn is_extra_wrapper_tran_output_analysis_command(command: &str) -> bool {
        matches!(
            command.to_ascii_lowercase().as_str(),
            ".ac"
                | ".dc"
                | ".four"
                | ".fft"
                | ".hb"
                | ".measure"
                | ".meas"
                | ".noise"
                | ".probe"
                | ".save"
                | ".sens"
        )
    }

    pub(super) fn is_tran_analysis_keyword(keyword: &str) -> bool {
        matches!(keyword.trim().to_ascii_uppercase().as_str(), "TR" | "TRAN")
    }

    pub(super) fn is_ignorable_wrapper_tran_measure_side_output(
        line: &str,
    ) -> Result<bool, String> {
        let tokens = Self::split_grouped_whitespace_fields(line, ".MEASURE statement")?;
        if tokens.len() < 5 {
            return Ok(false);
        }
        if !matches!(
            tokens[0].to_ascii_lowercase().as_str(),
            ".measure" | ".meas"
        ) || !tokens[1].eq_ignore_ascii_case("TRAN")
        {
            return Ok(false);
        }
        if !matches!(tokens[3].to_ascii_lowercase().as_str(), "max" | "min") {
            return Ok(false);
        }

        let token_refs = tokens.iter().map(String::as_str).collect::<Vec<_>>();
        let mut index = 4usize;
        while index < token_refs.len() {
            if Self::print_option_assignment(&token_refs, index).is_some() {
                return Ok(false);
            }
            let normalized = token_refs[index]
                .trim()
                .trim_matches(['"', '\''])
                .to_ascii_lowercase();
            if matches!(
                normalized.as_str(),
                "error"
                    | "four"
                    | "fft"
                    | "file"
                    | "failvalue"
                    | "comp_function"
                    | "indepvarcol"
                    | "depvarcol"
            ) {
                return Ok(false);
            }
            index += 1;
        }

        Ok(true)
    }

    pub(super) fn is_extra_wrapper_output_analysis_command(command: &str) -> bool {
        matches!(
            command.to_ascii_lowercase().as_str(),
            ".ac"
                | ".four"
                | ".fft"
                | ".hb"
                | ".measure"
                | ".meas"
                | ".noise"
                | ".probe"
                | ".save"
                | ".sens"
                | ".tran"
        )
    }

    pub(super) fn noise_analysis_for_netlist(
        netlist: &Netlist,
    ) -> Result<XyceNoiseAnalysis, String> {
        let analyses = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Noise {
                    output_node,
                    reference_node,
                    input_source,
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                } => Some(Ok(XyceNoiseAnalysis {
                    output_node: output_node.clone(),
                    reference_node: reference_node.clone(),
                    input_source: input_source.clone(),
                    frequencies: ac_sweep_frequencies(*variation, *points, *start_freq, *stop_freq),
                    data_points: None,
                    data_table_name: None,
                })),
                AnalysisCommand::NoiseData {
                    output_node,
                    reference_node,
                    input_source,
                    table_name,
                } => Some(
                    Self::frequency_data_table_points(netlist, table_name, ".NOISE DATA").map(
                        |data_points| XyceNoiseAnalysis {
                            output_node: output_node.clone(),
                            reference_node: reference_node.clone(),
                            input_source: input_source.clone(),
                            frequencies: data_points.iter().map(|point| point.frequency).collect(),
                            data_points: Some(data_points),
                            data_table_name: Some(table_name.clone()),
                        },
                    ),
                ),
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()?;
        let analysis = match analyses.as_slice() {
            [analysis] => analysis.clone(),
            [] => return Err("deck has no .NOISE analysis".to_string()),
            _ => return Err("deck has multiple .NOISE analyses".to_string()),
        };
        if analysis.frequencies.is_empty() {
            return Err(".NOISE analysis produced no frequency points".to_string());
        }
        Ok(analysis)
    }

    /// Preserve Xyce's authored DELMAX for native MOS3 locked candidates.
    /// Their nonlinear charge companions make the accepted-step history
    /// path-sensitive; replacing Xyce's ceiling with the harness sampling cap
    /// changes the waveform even when every requested output time is identical.
    pub(super) fn netlist_requires_xyce_locked_solver_ceiling(netlist: &Netlist) -> bool {
        if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            let Ok(flattened) = flatten_netlist_with_models(netlist) else {
                return false;
            };
            let mut flat_netlist = netlist.clone();
            flat_netlist.elements = flattened.elements;
            flat_netlist.models.extend(flattened.scoped_models);
            flat_netlist.subcircuits.clear();
            return Self::netlist_requires_xyce_locked_solver_ceiling(&flat_netlist);
        }

        Self::netlist_is_native_transient_level3_mosfet_network(netlist)
    }

    /// Resolve the solver ceiling for a structurally qualified legacy MOS2
    /// transient.  The Berkeley MOS2 evaluator couples depletion charge,
    /// surface-state charge, and the VMAX channel-shortening derivatives;
    /// using the native 10%-of-window ceiling lets a single accepted step
    /// span that coupled transition and changes the companion history.
    /// The fine envelope is therefore part of the absolute MOS2 qualification
    /// contract, not a deck-specific exception.  Authored `DTMAX` remains
    /// authoritative for every model family.
    pub(super) fn transient_oracle_solver_max_step_for_netlist(
        netlist: &Netlist,
        tran: &XyceTranAnalysis,
    ) -> Value {
        let solver_max_step = Self::transient_oracle_solver_max_step(tran);
        if tran.max_step.is_none()
            && Self::netlist_is_native_transient_level2_mosfet_network(netlist)
        {
            let window = (tran.stop - tran.start.unwrap_or(0.0)).max(f64::MIN_POSITIVE);
            solver_max_step.min((window / 1000.0).max(f64::MIN_POSITIVE))
        } else {
            solver_max_step
        }
    }

    pub(super) fn scoped_model_source_fingerprint(
        spec: &rspice_core::netlist::SourceSpec,
    ) -> Result<(String, Vec<u64>), String> {
        match spec {
            rspice_core::netlist::SourceSpec::Dc(value) => {
                Ok(("DC".to_string(), vec![value.to_bits()]))
            }
            rspice_core::netlist::SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero,
            } => Ok((
                "PULSE".to_string(),
                vec![
                    v1.to_bits(),
                    v2.to_bits(),
                    delay.to_bits(),
                    rise.to_bits(),
                    fall.to_bits(),
                    width.to_bits(),
                    period.to_bits(),
                    pulse_count.to_bits(),
                    u64::from(*width_defaults_to_zero),
                ],
            )),
            _ => Err("unqualified scoped-model source waveform".to_string()),
        }
    }

    pub(super) fn scoped_model_element_fingerprint(
        element: &rspice_core::netlist::Element,
        params: &rspice_core::netlist::expr::ParamContext,
    ) -> Result<XyceRelationalElementFingerprint, String> {
        let nodes = element
            .nodes
            .iter()
            .map(|node| node.trim().to_ascii_lowercase())
            .collect::<Vec<_>>();
        let (kind, numeric_bits, text) = match &element.kind {
            ElementKind::Resistor { value, .. } => {
                ("R".to_string(), vec![value.to_bits()], Vec::new())
            }
            ElementKind::Capacitor { value, .. } => {
                ("C".to_string(), vec![value.to_bits()], Vec::new())
            }
            ElementKind::VoltageSource(spec) => {
                let (waveform, numeric_bits) = Self::scoped_model_source_fingerprint(spec)?;
                (format!("V:{waveform}"), numeric_bits, Vec::new())
            }
            ElementKind::CurrentSource(spec) => {
                let (waveform, numeric_bits) = Self::scoped_model_source_fingerprint(spec)?;
                (format!("I:{waveform}"), numeric_bits, Vec::new())
            }
            ElementKind::Vccs {
                transconductance,
                control_nodes,
                ..
            } => (
                "G".to_string(),
                vec![transconductance.to_bits()],
                vec![
                    control_nodes.0.trim().to_ascii_lowercase(),
                    control_nodes.1.trim().to_ascii_lowercase(),
                ],
            ),
            ElementKind::BehavioralVoltage {
                expression,
                tc1,
                tc2,
                multiplicity,
            }
            | ElementKind::BehavioralCurrent {
                expression,
                tc1,
                tc2,
                multiplicity,
            } => {
                let kind = if matches!(&element.kind, ElementKind::BehavioralVoltage { .. }) {
                    "BV"
                } else {
                    "BI"
                };
                let prepared =
                    rspice_core::netlist::expr::prepare_behavioral_expression(expression, params)
                        .map_err(|err| {
                        format!(
                            "could not canonicalize behavioral expression for '{}': {err}",
                            element.name
                        )
                    })?;
                (
                    kind.to_string(),
                    vec![
                        tc1.to_bits(),
                        tc2.to_bits(),
                        multiplicity.value.to_bits(),
                        u64::from(multiplicity.given),
                    ],
                    vec![
                        prepared.trim().to_ascii_lowercase(),
                        multiplicity
                            .value_expr
                            .as_deref()
                            .unwrap_or("")
                            .trim()
                            .to_ascii_lowercase(),
                    ],
                )
            }
            ElementKind::Bjt { .. } => ("Q".to_string(), Vec::new(), Vec::new()),
            ElementKind::Diode { .. } => ("D".to_string(), Vec::new(), Vec::new()),
            _ => {
                return Err(format!(
                    "flattened scoped-model family contains unqualified element '{}'",
                    element.name
                ));
            }
        };
        Ok(XyceRelationalElementFingerprint {
            kind,
            nodes,
            numeric_bits,
            text,
        })
    }

    pub(super) fn single_spice_numeric_literal_value(field: &str) -> Result<Value, String> {
        if !Self::is_single_spice_numeric_literal(field) {
            return Err(format!(
                "'{field}' is not one direct finite SPICE numeric literal"
            ));
        }
        rspice_core::netlist::lexer::parse_spice_value(field.trim())
            .map_err(|err| format!("could not parse direct numeric literal '{field}': {err}"))
    }

    pub(super) fn is_single_spice_identifier(field: &str) -> bool {
        let Ok(tokens) = rspice_core::netlist::lexer::tokenize(field.trim()) else {
            return false;
        };
        matches!(
            tokens
                .iter()
                .map(|token| &token.kind)
                .collect::<Vec<_>>()
                .as_slice(),
            [
                rspice_core::netlist::lexer::TokenKind::Ident(_),
                rspice_core::netlist::lexer::TokenKind::Eof
            ]
        )
    }

    pub(super) fn is_single_spice_node_token(field: &str) -> bool {
        let Ok(tokens) = rspice_core::netlist::lexer::tokenize(field.trim()) else {
            return false;
        };
        matches!(
            tokens
                .iter()
                .map(|token| &token.kind)
                .collect::<Vec<_>>()
                .as_slice(),
            [
                rspice_core::netlist::lexer::TokenKind::Ident(_)
                    | rspice_core::netlist::lexer::TokenKind::Number(_),
                rspice_core::netlist::lexer::TokenKind::Eof
            ]
        )
    }

    pub(super) fn validate_nonlinear_core_model_step_source_directives(
        source: &str,
        stepped_owner: bool,
    ) -> Result<(), String> {
        let mut model_count = 0usize;
        let mut step_count = 0usize;
        let mut tran_count = 0usize;
        let mut print_count = 0usize;
        let mut end_count = 0usize;
        for (index, logical) in Self::logical_netlist_lines(source).into_iter().enumerate() {
            if index == 0 {
                continue;
            }
            let stripped = Self::strip_netlist_comment(&logical);
            let fields = stripped.split_whitespace().collect::<Vec<_>>();
            let Some(command) = fields.first() else {
                continue;
            };
            if !command.starts_with('.') {
                continue;
            }
            match command.to_ascii_lowercase().as_str() {
                ".model" => model_count += 1,
                ".step" => step_count += 1,
                ".tran" => tran_count += 1,
                ".print" => print_count += 1,
                ".end" if fields.len() == 1 => end_count += 1,
                other => {
                    return Err(format!(
                        "nonlinear CORE model-step family contains unqualified directive '{other}'"
                    ));
                }
            }
        }
        let expected_steps = usize::from(stepped_owner);
        if model_count != 1
            || step_count != expected_steps
            || tran_count != 1
            || print_count != 1
            || end_count != 1
        {
            return Err(format!(
                "nonlinear CORE model-step directive census requires MODEL=1 STEP={expected_steps} TRAN=1 PRINT=1 END=1, found MODEL={model_count} STEP={step_count} TRAN={tran_count} PRINT={print_count} END={end_count}"
            ));
        }
        Ok(())
    }

    pub(super) fn is_analytic_rc_wrapper_candidate(source: &str) -> bool {
        let lines = Self::logical_netlist_lines(source);
        let Some(title) = lines.first() else {
            return false;
        };
        let title = Self::strip_netlist_comment(title).trim();
        if title.is_empty() || title.starts_with('.') {
            return false;
        }

        let mut capacitor_count = 0usize;
        let mut resistor_count = 0usize;
        let mut voltage_source_count = 0usize;
        let mut print_count = 0usize;
        let mut tran_count = 0usize;
        let mut options_count = 0usize;
        let mut end_count = 0usize;

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            // Analytic RC candidates admit only bounded TIMEINT/RELTOL/
            // ABSTOL(/NEwLTE) option cards.  Reject Xyce's OUTPUT option from
            // its first two raw tokens so a large OUTPUTTIMEPOINTS list is
            // never expanded by the grouped-field tokenizer.
            if stripped
                .split_whitespace()
                .next()
                .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
                && stripped
                    .split_whitespace()
                    .nth(1)
                    .is_some_and(|option| option.eq_ignore_ascii_case("output"))
            {
                return false;
            }
            let Ok(fields) = Self::split_grouped_whitespace_fields(
                stripped,
                "analytic first-order RC candidate statement",
            ) else {
                return false;
            };
            let Some(command) = fields.first() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".print"
                        if fields.len() == 3
                            && fields[1].eq_ignore_ascii_case("TRAN")
                            && Self::parse_voltage_probe(&fields[2]).is_some() =>
                    {
                        print_count += 1;
                    }
                    ".tran" if fields.len() == 3 => tran_count += 1,
                    ".options" if matches!(fields.len(), 4 | 5) => {
                        let keys = fields[1..]
                            .iter()
                            .map(|field| {
                                field
                                    .split_once('=')
                                    .map_or(field.as_str(), |(name, _)| name)
                                    .to_ascii_lowercase()
                            })
                            .collect::<BTreeSet<_>>();
                        let base_keys = ["timeint", "reltol", "abstol"]
                            .into_iter()
                            .map(str::to_string)
                            .collect::<BTreeSet<_>>();
                        let newlte_keys = ["timeint", "reltol", "abstol", "newlte"]
                            .into_iter()
                            .map(str::to_string)
                            .collect::<BTreeSet<_>>();
                        if keys != base_keys && keys != newlte_keys {
                            return false;
                        }
                        options_count += 1;
                    }
                    ".end" => end_count += 1,
                    _ => return false,
                }
                continue;
            }

            match command.chars().next().map(|ch| ch.to_ascii_uppercase()) {
                Some('C') => capacitor_count += 1,
                Some('R') => resistor_count += 1,
                Some('V') => voltage_source_count += 1,
                _ => return false,
            }
        }

        capacitor_count == 1
            && resistor_count == 1
            && voltage_source_count == 1
            && print_count == 1
            && tran_count == 1
            && options_count == 1
            && end_count == 1
    }

    pub(super) fn is_analytic_sinusoidal_rc_wrapper_candidate(source: &str) -> bool {
        let lines = Self::logical_netlist_lines(source);
        let Some(title) = lines.first() else {
            return false;
        };
        let title = Self::strip_netlist_comment(title).trim();
        if title.is_empty() || title.starts_with('.') {
            return false;
        }

        let mut capacitor_count = 0usize;
        let mut resistor_count = 0usize;
        let mut sinusoidal_voltage_count = 0usize;
        let mut print_count = 0usize;
        let mut tran_count = 0usize;
        let mut options_count = 0usize;
        let mut end_count = 0usize;
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            // Keep the candidate probe bounded for wrapper-only decks that
            // carry a massive OUTPUTTIMEPOINTS option card.
            if stripped
                .split_whitespace()
                .next()
                .is_some_and(|command| command.eq_ignore_ascii_case(".options"))
                && stripped
                    .split_whitespace()
                    .nth(1)
                    .is_some_and(|option| option.eq_ignore_ascii_case("output"))
            {
                return false;
            }
            let Ok(fields) = Self::split_grouped_whitespace_fields(
                stripped,
                "analytic sinusoidal RC candidate statement",
            ) else {
                return false;
            };
            let Some(command) = fields.first() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".print"
                        if fields.len() == 3
                            && fields[1].eq_ignore_ascii_case("TRAN")
                            && Self::print_expression_inner(&fields[2]).is_some() =>
                    {
                        print_count += 1;
                    }
                    ".tran" if fields.len() == 3 => tran_count += 1,
                    ".options" if fields.len() == 5 => {
                        let keys = fields[1..]
                            .iter()
                            .map(|field| {
                                field
                                    .split_once('=')
                                    .map_or(field.as_str(), |(name, _)| name)
                                    .to_ascii_lowercase()
                            })
                            .collect::<BTreeSet<_>>();
                        let expected = ["timeint", "reltol", "abstol", "method"]
                            .into_iter()
                            .map(str::to_string)
                            .collect::<BTreeSet<_>>();
                        if keys != expected {
                            return false;
                        }
                        options_count += 1;
                    }
                    ".end" if fields.len() == 1 => end_count += 1,
                    _ => return false,
                }
                continue;
            }

            match command.chars().next().map(|ch| ch.to_ascii_uppercase()) {
                Some('C') => capacitor_count += 1,
                Some('R') => resistor_count += 1,
                Some('V')
                    if fields
                        .get(3)
                        .is_some_and(|field| field.eq_ignore_ascii_case("SIN")) =>
                {
                    sinusoidal_voltage_count += 1;
                }
                _ => return false,
            }
        }

        capacitor_count == 1
            && resistor_count == 1
            && sinusoidal_voltage_count == 1
            && print_count == 1
            && tran_count == 1
            && options_count == 1
            && end_count == 1
    }

    pub(super) fn passive_model_has_no_deferred_state(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
    }

    pub(super) fn model_is_native_dc_analysis_expression_mos1(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut names = BTreeSet::new();
        model.params.iter().all(|(name, value)| {
            let normalized = name.to_ascii_uppercase();
            names.insert(normalized.clone())
                && Self::native_dc_analysis_expression_mos1_model_param(&normalized, *value)
        })
    }

    pub(super) fn native_dc_analysis_expression_mos1_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name {
            "LEVEL" => value.to_bits() == 1.0f64.to_bits(),
            "VTO" | "VT0" | "VTH0" | "XTI" => true,
            "KP" | "PHI" | "U0" | "UO" | "TOX" | "NSUB" | "L" | "W" | "PB" | "AF" => value > 0.0,
            "GAMMA" | "LAMBDA" | "NSS" | "NFS" | "IS" | "JS" | "LD" | "RD" | "RS" | "RSH"
            | "CBD" | "CAPBD" | "CBS" | "CAPBS" | "CJ" | "CJ0" | "CJSW" | "CGSO" | "CGDO"
            | "CGBO" | "KF" => value >= 0.0,
            "TPG" | "GATE" => matches!(value, -1.0 | 0.0 | 1.0),
            "MJ" | "MJSW" => (0.0..=1.0).contains(&value),
            "FC" => (0.0..1.0).contains(&value),
            "TNOM" => value > -273.15,
            _ => false,
        }
    }

    pub(super) fn diode_model_alias_comp_targets(source: &str) -> Result<Vec<String>, String> {
        const LABEL: &str = "native diode model-parameter alias equivalence";
        let mut targets = Vec::new();
        for line in source.lines() {
            let trimmed = line.trim_start();
            let Some(head) = trimmed.split_whitespace().next() else {
                continue;
            };
            if !head.eq_ignore_ascii_case("*COMP") {
                continue;
            }
            let fields = trimmed.split_whitespace().collect::<Vec<_>>();
            if fields.len() != 4 {
                return Err(format!("{LABEL} requires four fields on every *COMP line"));
            }
            let tolerance = |field: &str, expected: &str| -> Option<Value> {
                let (name, literal) = field.split_once('=')?;
                if !name.eq_ignore_ascii_case(expected) {
                    return None;
                }
                Self::single_spice_numeric_literal_value(literal)
                    .ok()
                    .filter(|value| value.is_finite() && *value > 0.0)
            };
            if tolerance(fields[2], "RELTOL").is_none() || tolerance(fields[3], "ABSTOL").is_none()
            {
                return Err(format!(
                    "{LABEL} requires finite positive RELTOL and ABSTOL on every *COMP line"
                ));
            }
            targets.push(Self::normalize_probe(fields[1]));
        }
        if targets.len() != 3 {
            return Err(format!("{LABEL} requires exactly three *COMP targets"));
        }
        Ok(targets)
    }

    pub(super) fn strict_param_expression_voltage_node_matches(
        expression: &Expr,
        expected: &str,
    ) -> bool {
        matches!(
            expression,
            Expr::NodeVoltage(node)
                if Self::canonical_param_expression_node_name(node) == expected
        )
    }

    pub(super) fn strict_param_expression_squared_voltage_difference_matches(
        expression: &Expr,
        positive_node: &str,
        negative_node: &str,
    ) -> bool {
        let Expr::Binary {
            op: rspice_core::expr::BinaryOp::Pow,
            left,
            right,
        } = expression
        else {
            return false;
        };
        let Expr::Const(exponent) = right.as_ref() else {
            return false;
        };
        let Expr::Binary {
            op: rspice_core::expr::BinaryOp::Sub,
            left,
            right,
        } = left.as_ref()
        else {
            return false;
        };
        exponent.to_bits() == 2.0f64.to_bits()
            && Self::strict_param_expression_voltage_node_matches(left, positive_node)
            && Self::strict_param_expression_voltage_node_matches(right, negative_node)
    }

    pub(super) fn is_single_spice_numeric_literal(field: &str) -> bool {
        let Ok(tokens) = rspice_core::netlist::lexer::tokenize(field) else {
            return false;
        };
        if !rspice_core::netlist::lexer::parse_spice_value(field.trim()).is_ok_and(Value::is_finite)
        {
            return false;
        }
        let kinds = tokens.iter().map(|token| &token.kind).collect::<Vec<_>>();
        matches!(
            kinds.as_slice(),
            [
                rspice_core::netlist::lexer::TokenKind::Number(_)
                    | rspice_core::netlist::lexer::TokenKind::Ident(_),
                rspice_core::netlist::lexer::TokenKind::Eof
            ] | [
                rspice_core::netlist::lexer::TokenKind::Plus
                    | rspice_core::netlist::lexer::TokenKind::Minus,
                rspice_core::netlist::lexer::TokenKind::Number(_)
                    | rspice_core::netlist::lexer::TokenKind::Ident(_),
                rspice_core::netlist::lexer::TokenKind::Eof
            ]
        )
    }

    pub(super) fn is_single_braced_voltage_assignment(field: &str) -> bool {
        let Some((key, expression)) = field.split_once('=') else {
            return false;
        };
        if !key.trim().eq_ignore_ascii_case("V") {
            return false;
        }
        let expression = expression.trim();
        let chars = expression.chars().collect::<Vec<_>>();
        if chars.first() != Some(&'{') || chars.last() != Some(&'}') {
            return false;
        }

        let mut depth = 0usize;
        for (index, ch) in chars.iter().enumerate() {
            match ch {
                '{' => depth += 1,
                '}' => {
                    let Some(next_depth) = depth.checked_sub(1) else {
                        return false;
                    };
                    depth = next_depth;
                    if depth == 0 && index + 1 != chars.len() {
                        return false;
                    }
                }
                _ => {}
            }
        }
        depth == 0
    }

    pub(super) fn model_is_native_bjt_external_node_level1_npn(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        model.model_type.eq_ignore_ascii_case("NPN")
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model.params.len() == 1
            && model.params[0].0.eq_ignore_ascii_case("BF")
            && model.params[0].1.is_finite()
            && model.params[0].1 > 0.0
    }

    pub(super) fn materialize_dc_data_row_netlist(
        engine: &Engine,
        base_netlist: &Netlist,
        row: &XyceDcDataRow,
    ) -> Result<Netlist, SimulationError> {
        let param_overrides = row
            .overrides
            .iter()
            .filter_map(|override_| match override_ {
                XyceDcDataOverride::Parameter { name, value } => Some((name.clone(), *value)),
                XyceDcDataOverride::Device { .. } => None,
            })
            .collect::<Vec<_>>();
        let (mut row_netlist, param_bindings) = if param_overrides.is_empty() {
            (base_netlist.clone(), 0)
        } else {
            Engine::create_perturbed_netlist_multi(base_netlist, &param_overrides)?
        };
        if !param_overrides.is_empty() && base_netlist.source_text.is_some() && param_bindings == 0
        {
            let names = param_overrides
                .iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(SimulationError::Circuit(format!(
                ".DC DATA parameter override(s) {names} are not bound to any netlist expression"
            )));
        }

        for override_ in &row.overrides {
            let XyceDcDataOverride::Device {
                name,
                param_name,
                value,
            } = override_
            else {
                continue;
            };
            let step = StepCommand {
                target: StepTarget::Device,
                name: name.clone(),
                param_name: param_name.clone(),
                sweep: StepSweep::List(vec![*value]),
            };
            let mut stepped = engine.step_netlists_for_command(&row_netlist, &step, &[*value])?;
            let Some((_, next_netlist)) = stepped.pop() else {
                return Err(SimulationError::Circuit(format!(
                    ".DC DATA device override '{}' produced no stepped netlist",
                    name
                )));
            };
            row_netlist = next_netlist;
        }

        Ok(row_netlist)
    }

    pub(super) fn dc_sensitivity_point_netlist(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
    ) -> Result<Netlist, String> {
        let dimension = XyceDcSweepDimension {
            source: dc.source.clone(),
            start: dc.start,
            stop: dc.stop,
            step: dc.step,
            mode: dc.mode.clone(),
        };
        let mut swept = netlist.clone();
        Self::apply_static_dc_dimension(&mut swept, &dimension, sweep_point.primary)
            .map_err(|error| error.to_string())?;
        if let Some(sweep2) = &dc.sweep2
            && let Some(secondary) = sweep_point.secondary
        {
            let dimension = XyceDcSweepDimension {
                source: sweep2.source.clone(),
                start: sweep2.start,
                stop: sweep2.stop,
                step: sweep2.step,
                mode: sweep2.mode.clone(),
            };
            Self::apply_static_dc_dimension(&mut swept, &dimension, secondary)
                .map_err(|error| error.to_string())?;
        }
        Ok(swept)
    }

    pub(super) fn is_ac_sensitivity_generated_column(column: &str) -> bool {
        let normalized = Self::normalize_probe(column);
        [
            "re({", "im({", "mag({", "ph({", "d_re({", "d_im({", "d_mag({", "d_ph({",
        ]
        .iter()
        .any(|prefix| normalized.starts_with(prefix))
    }

    pub(super) fn netlist_has_top_level_element_named(netlist: &Netlist, name: &str) -> bool {
        netlist
            .elements
            .iter()
            .any(|element| element.name.eq_ignore_ascii_case(name))
    }

    pub(super) fn netlist_has_numeric_parameter(netlist: &Netlist, name: &str) -> bool {
        netlist
            .params
            .all_params()
            .iter()
            .any(|(param_name, _)| param_name.eq_ignore_ascii_case(name))
    }

    pub(super) fn netlist_model_is_current_switch(netlist: &Netlist, model_name: &str) -> bool {
        Self::find_model(&netlist.models, model_name).is_some_and(|model| {
            matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "ISWITCH" | "ISW" | "CSW"
            )
        })
    }

    pub(super) fn netlist_uses_ekv3_level301_mosfet(netlist: &Netlist) -> bool {
        if Self::elements_use_ekv3_level301_mosfet(&netlist.elements, &netlist.models, &[]) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_use_ekv3_level301_mosfet(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
            )
        })
    }

    pub(super) fn netlist_uses_native_vbic_bjt(netlist: &Netlist) -> bool {
        if Self::elements_use_native_vbic_bjt(&netlist.elements, &netlist.models, &[]) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_use_native_vbic_bjt(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
            )
        })
    }

    pub(super) fn model_matches_geometry_bin(
        model: &rspice_core::netlist::ModelDef,
        instance_params: &[(String, Value)],
    ) -> bool {
        let lmin = Self::numeric_param_value(&model.params, "LMIN");
        let lmax = Self::numeric_param_value(&model.params, "LMAX");
        let wmin = Self::numeric_param_value(&model.params, "WMIN");
        let wmax = Self::numeric_param_value(&model.params, "WMAX");
        let nfinmin = Self::numeric_param_value(&model.params, "NFINMIN");
        let nfinmax = Self::numeric_param_value(&model.params, "NFINMAX");

        if lmin.is_none()
            && lmax.is_none()
            && wmin.is_none()
            && wmax.is_none()
            && nfinmin.is_none()
            && nfinmax.is_none()
        {
            return false;
        }

        let length = Self::numeric_param_value(instance_params, "L")
            .or_else(|| Self::numeric_param_value(instance_params, "LENGTH"));
        let width = Self::numeric_param_value(instance_params, "W")
            .or_else(|| Self::numeric_param_value(instance_params, "WIDTH"));
        let nfin = Self::numeric_param_value(instance_params, "NFIN");

        Self::bin_range_contains(length, lmin, lmax)
            && Self::bin_range_contains(width, wmin, wmax)
            && Self::bin_range_contains(nfin, nfinmin, nfinmax)
    }

    pub(super) fn model_geometry_bin_range_size(model: &rspice_core::netlist::ModelDef) -> Value {
        Self::bin_range_size(
            Self::numeric_param_value(&model.params, "LMIN"),
            Self::numeric_param_value(&model.params, "LMAX"),
        ) + Self::bin_range_size(
            Self::numeric_param_value(&model.params, "WMIN"),
            Self::numeric_param_value(&model.params, "WMAX"),
        ) + Self::bin_range_size(
            Self::numeric_param_value(&model.params, "NFINMIN"),
            Self::numeric_param_value(&model.params, "NFINMAX"),
        )
    }

    pub(super) fn model_is_ekv3_level301(model: &rspice_core::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) && model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
            .is_some_and(|(_, value)| (*value - 301.0).abs() <= 1.0e-9)
    }

    pub(super) fn model_is_ekv3_level301_native_150nm_branch_current(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) && Self::model_is_ekv3_level301(model)
    }

    pub(super) fn model_is_native_vbic_bjt(model: &rspice_core::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NPN" | "PNP"
        ) && model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
            .is_some_and(|(_, value)| {
                [4.0, 9.0, 11.0, 12.0, 13.0]
                    .iter()
                    .any(|level| (*value - *level).abs() <= 1.0e-9)
            })
    }

    pub(super) fn model_is_native_transient_vbic_level11(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NPN")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != 10
        {
            return false;
        }

        let mut names = BTreeSet::new();
        model.params.iter().all(|(name, value)| {
            let normalized = name.to_ascii_uppercase();
            let valid = value.is_finite()
                && match normalized.as_str() {
                    "LEVEL" => (*value - 11.0).abs() <= 1.0e-9,
                    "RCX" | "RCI" | "RBX" | "RBI" | "RE" | "RBP" | "RS" | "RTH" => *value > 0.0,
                    "IBEN" => *value > 0.0,
                    _ => false,
                };
            valid && names.insert(normalized)
        }) && names.len() == 10
    }

    pub(super) fn dc_sweep_point_netlist(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
    ) -> Result<Option<Netlist>, String> {
        let mut overrides = Vec::new();
        if Self::scalar_parameter_sweep_source_is_supported(netlist, &dc.source) {
            overrides.push((dc.source.clone(), sweep_point.primary));
        } else if let Some(device_parameter) =
            Engine::canonical_device_parameter_sweep_source(netlist, &dc.source)
        {
            overrides.push((device_parameter, sweep_point.primary));
        }
        if let Some(sweep2) = &dc.sweep2
            && let Some(secondary) = sweep_point.secondary
        {
            if Self::scalar_parameter_sweep_source_is_supported(netlist, &sweep2.source) {
                overrides.push((sweep2.source.clone(), secondary));
            } else if let Some(device_parameter) =
                Engine::canonical_device_parameter_sweep_source(netlist, &sweep2.source)
            {
                overrides.push((device_parameter, secondary));
            }
        }
        if overrides.is_empty() {
            return Ok(None);
        }

        Engine::create_perturbed_netlist_multi(netlist, &overrides)
            .map(|(netlist, _)| Some(netlist))
            .map_err(|err| {
                format!(
                    "failed to build Xyce DC parameter-sweep netlist for {:?}: {}",
                    overrides, err
                )
            })
    }

    pub(super) fn model_parameter_probe_is_supported(
        netlist: &Netlist,
        model_name: &str,
        parameter: &str,
    ) -> bool {
        Self::models_have_parameter_probe(&netlist.models, model_name, parameter)
            || rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
                Self::models_have_parameter_probe(&flattened.scoped_models, model_name, parameter)
            })
    }

    pub(super) fn models_have_parameter_probe(
        models: &[rspice_core::netlist::ModelDef],
        model_name: &str,
        parameter: &str,
    ) -> bool {
        Self::find_model(models, model_name).is_some_and(|model| {
            model
                .params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
                || model
                    .expr_params
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
        })
    }

    pub(super) fn evaluate_model_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        model_name: &str,
        parameter: &str,
    ) -> Option<Result<Value, String>> {
        Self::model_parameter_probe_value(
            &netlist.models,
            netlist,
            dc,
            sweep_point,
            model_name,
            parameter,
        )
        .or_else(|| {
            let flattened = rspice_core::netlist::flatten_netlist_with_models(netlist).ok()?;
            Self::model_parameter_probe_value(
                &flattened.scoped_models,
                netlist,
                dc,
                sweep_point,
                model_name,
                parameter,
            )
        })
    }

    pub(super) fn model_parameter_probe_value(
        models: &[rspice_core::netlist::ModelDef],
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        model_name: &str,
        parameter: &str,
    ) -> Option<Result<Value, String>> {
        let model = Self::find_model(models, model_name)?;
        if let Some((_, value)) = model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
        {
            return Some(Ok(*value));
        }

        model
            .expr_params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
            .map(|(_, expression)| {
                let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
                rspice_core::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                    format!(
                        "failed to evaluate model parameter probe '{}:{}': {err}",
                        model_name, parameter
                    )
                })
            })
    }

    pub(super) fn netlist_has_device_op_instance(netlist: &Netlist, instance_name: &str) -> bool {
        if Self::netlist_has_xyce_core_namespace(netlist, instance_name) {
            return true;
        }
        if netlist.elements.iter().any(|element| {
            Self::netlist_element_exports_device_op(element)
                && Self::device_instance_names_match(&element.name, instance_name)
        }) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            flattened.elements.iter().any(|element| {
                Self::netlist_element_exports_device_op(element)
                    && Self::device_instance_names_match(&element.name, instance_name)
            })
        })
    }

    pub(super) fn netlist_has_xyce_core_namespace(netlist: &Netlist, instance_name: &str) -> bool {
        let normalized = Self::normalize_probe(instance_name);
        let Some(core_name) = normalized.strip_prefix("ymin!") else {
            return false;
        };
        let matches = |elements: &[rspice_core::netlist::Element]| {
            elements.iter().any(|element| {
                element.name.eq_ignore_ascii_case(core_name)
                    && matches!(
                        &element.kind,
                        ElementKind::Coupling {
                            model: Some(_),
                            inductors,
                            ..
                        } if !inductors.is_empty()
                    )
            })
        };
        if matches(&netlist.elements) {
            return true;
        }
        rspice_core::netlist::flatten_netlist_with_models(netlist)
            .is_ok_and(|flattened| matches(&flattened.elements))
    }

    pub(super) fn netlist_supports_lead_current_probe(
        netlist: &Netlist,
        probe: &XyceLeadCurrentProbe,
    ) -> bool {
        match probe.terminal {
            XyceLeadCurrentTerminal::Drain => true,
            XyceLeadCurrentTerminal::Source => {
                Self::netlist_device_is_native_b3soi_mosfet(netlist, &probe.element_name)
            }
            XyceLeadCurrentTerminal::Gate => false,
            XyceLeadCurrentTerminal::Bulk => {
                Self::netlist_element_is_bjt(netlist, &probe.element_name)
            }
            XyceLeadCurrentTerminal::Collector | XyceLeadCurrentTerminal::Emitter => {
                Self::netlist_element_is_bjt(netlist, &probe.element_name)
            }
        }
    }

    pub(super) fn netlist_element_is_bjt(netlist: &Netlist, instance_name: &str) -> bool {
        if netlist.elements.iter().any(|element| {
            matches!(element.kind, ElementKind::Bjt { .. })
                && Self::device_instance_names_match(&element.name, instance_name)
        }) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            flattened.elements.iter().any(|element| {
                matches!(element.kind, ElementKind::Bjt { .. })
                    && Self::device_instance_names_match(&element.name, instance_name)
            })
        })
    }

    pub(super) fn netlist_has_lead_current_wildcard_match(
        netlist: &Netlist,
        terminal: XyceLeadCurrentTerminal,
    ) -> bool {
        if netlist
            .elements
            .iter()
            .any(|element| Self::element_matches_lead_current_wildcard(element, terminal))
        {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            flattened
                .elements
                .iter()
                .any(|element| Self::element_matches_lead_current_wildcard(element, terminal))
        })
    }

    pub(super) fn netlist_device_is_native_b3soi_mosfet(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        if Self::elements_device_is_native_b3soi_mosfet(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_b3soi_mosfet(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    pub(super) fn netlist_device_is_native_classic_jfet(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        if Self::elements_device_is_native_classic_jfet(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_classic_jfet(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    pub(super) fn netlist_xspice_model_is_native_transient_tff(
        netlist: &Netlist,
        model_name: &str,
    ) -> bool {
        model_name.eq_ignore_ascii_case("d_tff")
            || model_name.eq_ignore_ascii_case("xyce_d_tff")
            || Self::find_unique_model_in(&netlist.models, model_name).is_some_and(|model| {
                model.model_type.eq_ignore_ascii_case("d_tff")
                    || model.model_type.eq_ignore_ascii_case("xyce_d_tff")
            })
    }

    pub(super) fn netlist_xspice_model_is_native_transient_dig_gate(
        netlist: &Netlist,
        model_name: &str,
        timing: Option<&rspice_core::netlist::PspiceUTiming>,
    ) -> bool {
        let model = model_name.to_ascii_lowercase();
        let is_legacy = model.starts_with("xyce_legacy_d_");
        if !matches!(
            model.as_str(),
            "d_dff"
                | "d_dlatch"
                | "d_jkff"
                | "d_add"
                | "d_and"
                | "d_buffer"
                | "d_inverter"
                | "d_nand"
                | "d_nor"
                | "d_or"
                | "d_xnor"
                | "d_xor"
        ) && !matches!(
            model.as_str(),
            "xyce_legacy_d_and"
                | "xyce_legacy_d_dff"
                | "xyce_legacy_d_inverter"
                | "xyce_legacy_d_nand"
                | "xyce_legacy_d_nor"
                | "xyce_legacy_d_or"
                | "xyce_legacy_d_xnor"
                | "xyce_legacy_d_xor"
                | "xyce_d_dff"
                | "xyce_d_dlatch"
                | "xyce_d_jkff"
        ) {
            return false;
        }
        let Some(timing) = timing else {
            return false;
        };
        (is_legacy || timing.power_pins.is_some())
            && Self::find_unique_model_in(&netlist.models, &timing.timing_model)
                .is_some_and(|model| model.model_type.eq_ignore_ascii_case("DIG"))
    }

    pub(super) fn netlist_device_is_native_relational_legacy_diode(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        netlist.elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Diode {
                model,
                instance_params,
                deferred_params,
            } = &element.kind
            else {
                return false;
            };
            deferred_params.is_empty()
                && instance_params
                    .iter()
                    .all(|(name, value)| Self::native_relational_diode_instance_param(name, *value))
                && Self::find_model(&netlist.models, model)
                    .is_some_and(Self::model_is_native_relational_legacy_diode)
        })
    }

    pub(super) fn model_is_native_relational_legacy_diode(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        Self::model_is_native_legacy_diode(model)
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model
                .params
                .iter()
                .all(|(name, value)| Self::native_relational_diode_model_param(name, *value))
    }

    pub(super) fn native_relational_diode_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() || !Self::xyce_level2_native_diode_param(name) {
            return false;
        }
        match name.to_ascii_uppercase().as_str() {
            "LEVEL" => {
                (value - 0.0).abs() <= 1.0e-9
                    || (value - 1.0).abs() <= 1.0e-9
                    || (value - 2.0).abs() <= 1.0e-9
            }
            "IS" | "JS" | "RS" | "KF" | "IKF" | "IK" | "IKR" | "CJO" | "CJ0" | "CJ" | "TT"
            | "JSW" | "CJSW" | "CJP" | "ISR" => value >= 0.0,
            "N" | "AF" | "IBV" | "NR" | "NS" | "VJ" | "PHP" | "VJSW" | "EG" | "BV" | "VB"
            | "NBV" => value > 0.0,
            "M" | "MJSW" => (0.0..=1.0).contains(&value),
            "FC" | "FCS" => (0.0..1.0).contains(&value),
            "XTI" => true,
            "TNOM" => value > -273.15,
            "TBV1" | "TBV2" => true,
            _ => false,
        }
    }

    pub(super) fn netlist_device_is_native_relational_mos3(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        netlist.elements.iter().any(|element| {
            if !Self::device_instance_names_match(&element.name, instance_name) {
                return false;
            }
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return false;
            };
            if *compact_syntax
                || !deferred_params.is_empty()
                || !instance_params
                    .iter()
                    .all(|(name, value)| Self::native_relational_mos3_instance_param(name, *value))
            {
                return false;
            }
            let Some(model) = Self::find_model(&netlist.models, model) else {
                return false;
            };
            Self::model_is_native_relational_mos3(model)
                && Self::native_relational_mos3_effective_geometry_is_valid(model, instance_params)
        })
    }

    pub(super) fn model_is_native_relational_mos3(model: &rspice_core::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || Self::numeric_param_value(&model.params, "LEVEL")
            .is_none_or(|level| !level.is_finite() || (level - 3.0).abs() > 1.0e-9)
        {
            return false;
        }
        model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model
                .params
                .iter()
                .all(|(name, value)| Self::native_relational_mos3_model_param(name, *value))
    }

    pub(super) fn native_relational_mos3_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name.to_ascii_uppercase().as_str() {
            "LEVEL" => (value - 3.0).abs() <= 1.0e-9,
            "VTO" | "VT0" | "ETA" | "XL" | "XW" => true,
            "TPG" => matches!(value, -1.0 | 0.0 | 1.0),
            "KP" | "PHI" | "U0" | "UO" | "TOX" | "NSUB" | "L" | "W" | "PB" | "KAPPA" | "AF" => {
                value > 0.0
            }
            "GAMMA" | "LAMBDA" | "NSS" | "IS" | "JS" | "LD" | "RD" | "RS" | "CBD" | "CBS"
            | "CJ" | "CJ0" | "CJSW" | "CGSO" | "CGDO" | "CGBO" | "DELTA" | "THETA" | "NFS"
            | "VMAX" | "XJ" | "WD" | "KF" => value >= 0.0,
            // The classic MOS builder currently has two RSH lowering paths;
            // zero is semantically inert and therefore the only qualified
            // relational value until nonzero topology is single-stamped.
            "RSH" => value == 0.0,
            "MJ" | "MJSW" => (0.0..=1.0).contains(&value),
            "FC" => (0.0..1.0).contains(&value),
            "TNOM" => value > -273.15,
            _ => false,
        }
    }

    pub(super) fn netlist_is_native_classic_mos_parameter_alias_envelope(
        netlist: &Netlist,
    ) -> bool {
        if !netlist.subcircuits.is_empty()
            || netlist.models.len() != 2
            || netlist.elements.len() != 7
        {
            return false;
        }
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.len() != 2 {
            return false;
        }
        let mut levels = BTreeSet::new();
        let mut model_types = BTreeSet::new();
        for model in &netlist.models {
            let Some(level) = Self::numeric_param_value(&model.params, "LEVEL") else {
                return false;
            };
            if !matches!(level, 1.0 | 2.0 | 3.0 | 6.0)
                || !Self::model_is_native_classic_mos_parameter_alias(model, level)
            {
                return false;
            }
            levels.insert(level.to_bits());
            model_types.insert(model.model_type.to_ascii_uppercase());
        }
        if levels.len() != 1
            || model_types != BTreeSet::from(["NMOS".to_string(), "PMOS".to_string()])
        {
            return false;
        }
        let mut referenced_model_types = BTreeSet::new();
        let all_elements_supported = mosfets.into_iter().all(|element| {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                return false;
            };
            let names = instance_params
                .iter()
                .map(|(name, value)| (name.to_ascii_uppercase(), *value))
                .collect::<BTreeMap<_, _>>();
            let Some(model_def) = Self::find_unique_model_in(&netlist.models, model) else {
                return false;
            };
            referenced_model_types.insert(model_def.model_type.to_ascii_uppercase());
            element.nodes.len() == 4
                && !compact_syntax
                && deferred_params.is_empty()
                && instance_params.len() == 2
                && names.len() == 2
                && names
                    .get("L")
                    .is_some_and(|value| value.is_finite() && *value > 0.0)
                && names
                    .get("W")
                    .is_some_and(|value| value.is_finite() && *value > 0.0)
        });
        all_elements_supported
            && referenced_model_types == BTreeSet::from(["NMOS".to_string(), "PMOS".to_string()])
    }

    pub(super) fn netlist_element_is_native_classic_mos_parameter_alias(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        Self::netlist_is_native_classic_mos_parameter_alias_envelope(netlist)
            && matches!(element.kind, ElementKind::Mosfet { .. })
    }

    fn model_is_native_classic_mos_parameter_alias(
        model: &rspice_core::netlist::ModelDef,
        level: Value,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.iter().any(|(_, value)| !value.is_finite())
        {
            return false;
        }
        let names = model
            .params
            .iter()
            .map(|(name, _)| name.to_ascii_uppercase())
            .collect::<BTreeSet<_>>();
        if names.len() != model.params.len() {
            return false;
        }
        let one_mobility_alias = names.contains("UO") ^ names.contains("U0");
        let one_threshold_alias = names.contains("VTO") ^ names.contains("VT0");
        if !one_mobility_alias || !one_threshold_alias || names.contains("VTH0") {
            return false;
        }
        let mut expected = if level.to_bits() == 3.0f64.to_bits() {
            BTreeSet::from([
                "LEVEL", "U0", "VT0", "TOX", "NSUB", "NSS", "VMAX", "RS", "RD", "RSH", "IS", "XJ",
                "LD", "DELTA", "THETA", "ETA", "KAPPA", "KP", "L", "W", "GAMMA", "PHI", "NFS",
                "CBD", "CBS", "PB", "CGSO", "CGBO", "CGDO", "CJ", "MJ", "CJSW", "MJSW", "JS",
                "TPG", "KF", "AF", "FC", "TNOM",
            ])
        } else if matches!(level, 1.0 | 2.0 | 6.0) {
            BTreeSet::from([
                "LEVEL", "U0", "VT0", "TOX", "NSUB", "NSS", "RS", "RD", "RSH", "IS", "LD", "KP",
                "L", "W", "LAMBDA", "GAMMA", "PHI", "CBD", "CBS", "PB", "CGSO", "CGBO", "CGDO",
                "CJ", "MJ", "CJSW", "MJSW", "JS", "TPG", "KF", "AF", "FC", "TNOM",
            ])
        } else {
            return false;
        };
        if names.contains("UO") {
            expected.remove("U0");
            expected.insert("UO");
        }
        if names.contains("VTO") {
            expected.remove("VT0");
            expected.insert("VTO");
        }
        names.iter().map(String::as_str).collect::<BTreeSet<_>>() == expected
    }

    /// The native generated VBIC route is validated for one narrow absolute
    /// transient envelope: a single four-terminal LEVEL=11 NPN with the
    /// finite scalar card used by the Xyce BUG_1602 reference, driven by one
    /// PWL and three DC voltage sources and returning its fourth terminal to
    /// ground through one numeric resistor.  Other VBIC levels, parameter
    /// forms, hierarchies, and mixed-device transient networks remain
    /// fail-closed until each has an independent waveform oracle.
    pub(super) fn netlist_is_native_transient_vbic_level11_single_bjt(netlist: &Netlist) -> bool {
        if netlist.options.gmin.is_some()
            || !netlist.subcircuits.is_empty()
            || netlist.models.len() != 1
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { .. }])
            || !Self::native_transient_uses_standard_startup(netlist)
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return false;
        }

        let bjts = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Bjt { .. }))
            .collect::<Vec<_>>();
        if bjts.len() != 1 {
            return false;
        }
        let bjt = bjts[0];
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &bjt.kind
        else {
            return false;
        };
        if bjt.nodes.len() != 4
            || !bjt.nodes[2].eq_ignore_ascii_case("0")
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
            || !Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_transient_vbic_level11)
        {
            return false;
        }

        let mut dc_sources = 0usize;
        let mut pwl_sources = 0usize;
        let mut resistors = 0usize;
        let mut bulk_resistor_to_ground = false;
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::Bjt { .. } => {}
                ElementKind::VoltageSource(source) => {
                    if element.nodes.len() != 2 {
                        return false;
                    }
                    match source {
                        rspice_core::netlist::SourceSpec::Dc(value) if value.is_finite() => {
                            dc_sources += 1;
                        }
                        rspice_core::netlist::SourceSpec::Pwl {
                            points,
                            delay,
                            repeat_from,
                        } if Self::native_transient_vbic_pwl_is_valid(
                            points,
                            *delay,
                            *repeat_from,
                        ) =>
                        {
                            pwl_sources += 1;
                        }
                        _ => return false,
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    if element.nodes.len() != 2
                        || !value.is_finite()
                        || *value <= 0.0
                        || value_expr.is_some()
                        || model.is_some()
                        || !instance_params.is_empty()
                        || !deferred_params.is_empty()
                    {
                        return false;
                    }
                    resistors += 1;
                    bulk_resistor_to_ground = (element.nodes[0]
                        .eq_ignore_ascii_case(&bjt.nodes[3])
                        && element.nodes[1].eq_ignore_ascii_case("0"))
                        || (element.nodes[1].eq_ignore_ascii_case(&bjt.nodes[3])
                            && element.nodes[0].eq_ignore_ascii_case("0"));
                }
                _ => return false,
            }
        }

        dc_sources == 3 && pwl_sources == 1 && resistors == 1 && bulk_resistor_to_ground
    }

    pub(super) fn netlist_element_is_native_absolute_transient_level9_bsim3(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 4
            && !*compact_syntax
            && deferred_params.is_empty()
            && Self::native_absolute_transient_w_l_instance_params(instance_params)
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_absolute_transient_level9_bsim3)
    }

    /// The native BSIM4 v4.8 transient implementation has a narrowly
    /// qualified charge-integration envelope for the Xyce BUG_710_SON
    /// BSIM4-as-capacitor topology.  It is intentionally structural rather
    /// than path based: one four-terminal NMOS with D/S/B at ground, one
    /// positive DC current source injecting the gate, and an explicit zero
    /// gate initial condition.  Other BSIM4 transient networks remain
    /// fail-closed until independently reference-backed.
    pub(super) fn netlist_is_native_transient_bsim4_capacitor(netlist: &Netlist) -> bool {
        if !netlist.subcircuits.is_empty()
            || netlist.models.len() != 1
            || netlist.elements.len() != 2
            || netlist.initial_conditions.len() != 1
            || Self::tran_uses_uic(netlist)
            || !netlist.node_sets.is_empty()
            || netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Temp { .. }))
            || !netlist
                .options
                .temp
                .is_none_or(|temp| temp.is_finite() && (temp - 27.0).abs() <= 1.0e-9)
            || !netlist
                .options
                .tnom
                .is_none_or(|tnom| tnom.is_finite() && (tnom - 27.0).abs() <= 1.0e-9)
            || netlist.initial_conditions[0].voltage_expr.is_some()
            || !netlist.initial_conditions[0].voltage.is_finite()
            || netlist.initial_conditions[0].voltage != 0.0
        {
            return false;
        }

        let Some(mosfet) = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
        else {
            return false;
        };
        let Some(current_source) = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::CurrentSource(_)))
        else {
            return false;
        };
        if mosfet.nodes.len() != 4
            || !mosfet.nodes[0].eq_ignore_ascii_case("0")
            || !mosfet.nodes[2].eq_ignore_ascii_case("0")
            || !mosfet.nodes[3].eq_ignore_ascii_case("0")
            || !netlist.initial_conditions[0]
                .node
                .eq_ignore_ascii_case(&mosfet.nodes[1])
            || current_source.nodes.len() != 2
            || !current_source.nodes[0].eq_ignore_ascii_case("0")
            || !current_source.nodes[1].eq_ignore_ascii_case(&mosfet.nodes[1])
        {
            return false;
        }
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &mosfet.kind
        else {
            return false;
        };
        if *compact_syntax
            || !deferred_params.is_empty()
            || !Self::native_transient_bsim4_capacitor_instance_params_are_valid(instance_params)
            || !Self::find_unique_model_in(&netlist.models, model).is_some_and(|model| {
                model.model_type.eq_ignore_ascii_case("NMOS")
                    && Self::model_is_native_ac_supported_bsim4(model)
            })
        {
            return false;
        }
        let ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Dc(value)) =
            &current_source.kind
        else {
            return false;
        };
        value.is_finite() && *value > 0.0
    }

    /// The native BSIM3v3 transient implementation has a narrowly qualified
    /// charge-integration envelope for the Xyce BUG_710_SON BSIM3-as-capacitor
    /// topology.  This guard is structural: one four-terminal NMOS with
    /// D/S/B at ground, one positive DC current source injecting the gate, and
    /// an explicit zero gate initial condition.  Other BSIM3 transient
    /// networks remain fail-closed until independently reference-backed.
    pub(super) fn netlist_is_native_transient_bsim3_capacitor(netlist: &Netlist) -> bool {
        if !netlist.subcircuits.is_empty()
            || netlist.models.len() != 1
            || netlist.elements.len() != 2
            || netlist.initial_conditions.len() != 1
            || Self::tran_uses_uic(netlist)
            || !netlist.node_sets.is_empty()
            || netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Temp { .. }))
            || !netlist
                .options
                .temp
                .is_none_or(|temp| temp.is_finite() && (temp - 27.0).abs() <= 1.0e-9)
            || !netlist
                .options
                .tnom
                .is_none_or(|tnom| tnom.is_finite() && (tnom - 27.0).abs() <= 1.0e-9)
            || netlist.initial_conditions[0].voltage_expr.is_some()
            || !netlist.initial_conditions[0].voltage.is_finite()
            || netlist.initial_conditions[0].voltage != 0.0
        {
            return false;
        }
        let Some(mosfet) = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
        else {
            return false;
        };
        let Some(current_source) = netlist
            .elements
            .iter()
            .find(|element| matches!(element.kind, ElementKind::CurrentSource(_)))
        else {
            return false;
        };
        if mosfet.nodes.len() != 4
            || !mosfet.nodes[0].eq_ignore_ascii_case("0")
            || !mosfet.nodes[2].eq_ignore_ascii_case("0")
            || !mosfet.nodes[3].eq_ignore_ascii_case("0")
            || !netlist.initial_conditions[0]
                .node
                .eq_ignore_ascii_case(&mosfet.nodes[1])
            || current_source.nodes.len() != 2
            || !current_source.nodes[0].eq_ignore_ascii_case("0")
            || !current_source.nodes[1].eq_ignore_ascii_case(&mosfet.nodes[1])
        {
            return false;
        }
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &mosfet.kind
        else {
            return false;
        };
        if *compact_syntax
            || !deferred_params.is_empty()
            || !Self::native_transient_bsim3_capacitor_instance_params_are_valid(instance_params)
            || !Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_transient_bsim3_capacitor)
        {
            return false;
        }
        let ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::Dc(value)) =
            &current_source.kind
        else {
            return false;
        };
        value.is_finite() && *value > 0.0
    }

    pub(super) fn model_is_native_transient_bsim3_capacitor(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NMOS")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut names = BTreeSet::new();
        let mut level = false;
        let mut tox = false;
        model.params.iter().all(|(name, value)| {
            let key = name.to_ascii_uppercase();
            if !value.is_finite() || !names.insert(key.clone()) {
                return false;
            }
            match key.as_str() {
                "LEVEL" => {
                    level = (*value - 9.0).abs() <= 1.0e-9;
                    level
                }
                "TOX" => {
                    tox = *value > 0.0;
                    tox
                }
                _ => false,
            }
        }) && level
            && tox
    }

    pub(super) fn netlist_element_is_native_level9_xyce_verify_supported(
        element: &rspice_core::netlist::Element,
    ) -> bool {
        match &element.kind {
            ElementKind::Mosfet { .. } => true,
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                element.nodes.len() == 2
                    && Self::source_spec_is_finite_dc_or_pulse_or_explicit_dc_transient(spec)
            }
            ElementKind::Resistor {
                value,
                value_expr,
                model,
                instance_params,
                deferred_params,
            } => {
                element.nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            }
            ElementKind::Capacitor {
                value,
                value_expr,
                initial_voltage,
                model,
                instance_params,
                deferred_params,
            } => {
                element.nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && initial_voltage.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            }
            ElementKind::Inductor {
                value,
                value_expr,
                initial_current,
                model,
                instance_params,
                deferred_params,
            } => {
                element.nodes.len() == 2
                    && value.is_finite()
                    && *value > 0.0
                    && value_expr.is_none()
                    && initial_current.is_none()
                    && model.is_none()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            }
            _ => false,
        }
    }

    pub(super) fn netlist_is_native_level9_xyce_verify_envelope(netlist: &Netlist) -> bool {
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.is_empty()
            || mosfets.iter().any(|element| {
                !Self::netlist_element_is_native_absolute_transient_level9_bsim3(netlist, element)
            })
            || netlist.elements.iter().any(|element| {
                !Self::netlist_element_is_native_level9_xyce_verify_supported(element)
            })
        {
            return false;
        }

        let referenced_models = mosfets
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Mosfet { model, .. } => Some(model.to_ascii_lowercase()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        netlist.models.len() == referenced_models.len()
            && netlist.models.iter().all(|model| {
                referenced_models.contains(&model.name.to_ascii_lowercase())
                    && Self::model_is_native_absolute_transient_level9_bsim3(model)
            })
    }

    pub(super) fn netlist_element_is_native_bug1797_bsim3(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 4
            && !*compact_syntax
            && deferred_params.is_empty()
            && Self::native_absolute_transient_w_l_instance_params(instance_params)
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_bug1797_bsim3)
    }

    pub(super) fn netlist_is_native_bug1797_bsim3_envelope(netlist: &Netlist) -> bool {
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.len() != 17
            || mosfets
                .iter()
                .any(|element| !Self::netlist_element_is_native_bug1797_bsim3(netlist, element))
            || netlist.elements.iter().any(|element| {
                !Self::netlist_element_is_native_level9_xyce_verify_supported(element)
            })
        {
            return false;
        }
        let referenced_models = mosfets
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Mosfet { model, .. } => Some(model.to_ascii_lowercase()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        netlist.models.len() == 2
            && referenced_models.len() == 2
            && netlist.models.iter().all(|model| {
                referenced_models.contains(&model.name.to_ascii_lowercase())
                    && Self::model_is_native_bug1797_bsim3(model)
            })
    }

    pub(super) fn model_is_native_bug1797_bsim3(model: &rspice_core::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && matches!(model.params.as_slice(), [(name, level)]
                if name.eq_ignore_ascii_case("LEVEL")
                    && level.is_finite()
                    && matches!(level.to_bits(), bits
                        if bits == 9.0f64.to_bits() || bits == 49.0f64.to_bits()))
    }

    pub(super) fn model_is_native_absolute_transient_level9_bsim3(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && matches!(model.params.as_slice(), [(name, level)]
                if name.eq_ignore_ascii_case("LEVEL")
                    && level.is_finite()
                    && level.to_bits() == 9.0f64.to_bits())
    }

    pub(super) fn netlist_is_native_absolute_transient_vdmos_level18(netlist: &Netlist) -> bool {
        if netlist.options.gmin.is_some()
            || !netlist.subcircuits.is_empty()
            || netlist.models.len() != 1
        {
            return false;
        }

        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.len() != 1
            || netlist.elements.iter().any(|element| {
                matches!(
                    element.kind,
                    ElementKind::Bjt { .. }
                        | ElementKind::Diode { .. }
                        | ElementKind::Jfet { .. }
                        | ElementKind::XyceMemristor { .. }
                        | ElementKind::Subcircuit { .. }
                )
            })
        {
            return false;
        }

        Self::netlist_element_is_native_absolute_transient_vdmos_level18(netlist, mosfets[0])
    }

    pub(super) fn netlist_element_is_native_absolute_transient_vdmos_level18(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        netlist.options.gmin.is_none()
            && element.nodes.len() == 4
            && !compact_syntax
            && deferred_params.is_empty()
            && Self::native_absolute_transient_w_l_instance_params(instance_params)
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_absolute_transient_vdmos_level18)
    }

    pub(super) fn model_is_native_absolute_transient_vdmos_level18(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NMOS")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != 13
        {
            return false;
        }

        let mut names = BTreeSet::new();
        model.params.iter().all(|(name, value)| {
            let normalized = name.to_ascii_uppercase();
            let domain_ok = match normalized.as_str() {
                "LEVEL" => value.to_bits() == 18.0f64.to_bits(),
                // CV/CVE are Xyce's charge-model selectors.  The native
                // UCCM path implements the canonical selector pair only.
                "CV" | "CVE" => value.to_bits() == 1.0f64.to_bits(),
                "VTO" => value.is_finite() && *value > 0.0,
                "RD" | "RS" | "LAMBDA" | "SIGMA0" => value.is_finite() && *value >= 0.0,
                // The native Level=18 implementation consumes these physical
                // quantities directly and requires finite positive values.
                "UO" | "VMAX" | "TOX" => value.is_finite() && *value > 0.0,
                "DELTA" => value.is_finite() && *value >= 0.0,
                // Xyce's Level=18 M parameter is accepted only at the
                // canonical value used by the native UCCM envelope; other
                // values alter the compact model's knee behavior.
                "M" => value.to_bits() == 3.0f64.to_bits(),
                _ => false,
            };
            domain_ok && names.insert(normalized)
        }) && names.len() == 13
    }

    pub(super) fn netlist_element_is_native_scoped_model_relational_bjt(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 3
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_scoped_model_relational_bjt)
    }

    pub(super) fn netlist_element_is_native_static_ac_exact_bf_is_pnp(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 3
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_static_ac_exact_bf_is_pnp)
    }

    pub(super) fn netlist_element_is_native_static_ac_exact_bf_is_npn(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 3
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_static_ac_exact_bf_is_npn)
    }

    pub(super) fn model_is_native_static_ac_exact_bf_is_pnp(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        Self::model_is_native_static_ac_exact_bf_is_bjt(model)
            && matches!(
                model.model_type.to_ascii_uppercase().as_str(),
                "PNP" | "LPNP"
            )
    }

    pub(super) fn model_is_native_static_ac_exact_bf_is_npn(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        Self::model_is_native_static_ac_exact_bf_is_bjt(model)
            && model.model_type.eq_ignore_ascii_case("NPN")
    }

    pub(super) fn model_is_native_static_ac_exact_bf_is_bjt(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NPN" | "PNP" | "LPNP"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != 2
        {
            return false;
        }
        let bf_count = model
            .params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("BF"))
            .count();
        let is_count = model
            .params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("IS"))
            .count();
        bf_count == 1
            && is_count == 1
            && model.params.iter().all(|(name, value)| {
                value.is_finite()
                    && *value > 0.0
                    && matches!(name.to_ascii_uppercase().as_str(), "BF" | "IS")
            })
    }

    pub(super) fn model_is_native_scoped_model_relational_bjt(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NPN")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || model.params.len() != 2
        {
            return false;
        }
        let bf_count = model
            .params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("BF"))
            .count();
        let is_count = model
            .params
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("IS"))
            .count();
        bf_count == 1
            && is_count == 1
            && model.params.iter().all(|(name, value)| {
                value.is_finite()
                    && *value > 0.0
                    && matches!(name.to_ascii_uppercase().as_str(), "BF" | "IS")
            })
    }

    pub(super) fn netlist_element_is_native_exact_is_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 2
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_exact_is_diode)
    }

    pub(super) fn netlist_element_is_native_absolute_transient_exact_is_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        netlist.options.gmin.is_none()
            && Self::netlist_element_is_native_exact_is_diode(netlist, element)
    }

    pub(super) fn netlist_element_is_native_absolute_transient_tbv_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        netlist.options.gmin.is_none()
            && Self::netlist_element_is_native_xyce_level2_tbv_diode(netlist, element)
    }

    pub(super) fn netlist_element_is_native_relational_level2_tbv_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        netlist.options.gmin.is_none()
            && Self::netlist_element_is_native_xyce_level2_tbv_diode(netlist, element)
    }

    pub(super) fn netlist_element_is_native_absolute_transient_minimum_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        if netlist.options.gmin.is_some()
            || (netlist.options.device_min_resistance.is_none()
                && netlist.options.device_min_capacitance.is_none())
        {
            return false;
        }
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 2
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_relational_legacy_diode)
    }

    pub(super) fn netlist_element_is_native_absolute_transient_legacy_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        if netlist.options.gmin.is_some() {
            return false;
        }
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 2
            && instance_params.iter().all(|(name, value)| {
                Self::native_absolute_transient_legacy_diode_instance_param(name, *value)
            })
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_absolute_transient_legacy_diode)
    }

    pub(super) fn netlist_element_is_native_generated_cmc_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        if netlist.options.gmin.is_some() {
            return false;
        }
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 2
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_cmc_diode)
    }

    pub(super) fn model_is_native_cmc_diode(model: &rspice_core::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model.params.iter().all(|(name, value)| {
                name.eq_ignore_ascii_case("LEVEL")
                    && value.is_finite()
                    && (*value - 2002.0).abs() <= 1.0e-9
            })
            && model.params.len() == 1
    }

    pub(super) fn netlist_element_is_native_generated_juncap_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        if netlist.options.gmin.is_some() {
            return false;
        }
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 2
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_juncap_diode)
    }

    pub(super) fn model_is_native_juncap_diode(model: &rspice_core::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model.params.iter().all(|(name, value)| {
                name.eq_ignore_ascii_case("LEVEL")
                    && value.is_finite()
                    && (*value - 200.0).abs() <= 1.0e-9
            })
            && model.params.len() == 1
    }

    pub(super) fn model_is_native_absolute_transient_legacy_diode(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !Self::model_is_native_legacy_diode(model)
            || !model.real_vector_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
            || !model.string_params.is_empty()
        {
            return false;
        }

        // Keep the absolute envelope disjoint from the narrower IS-only
        // diode contract.  The legacy Xyce path admitted here is the
        // parameterized rectifier evaluator (N/RS are its required model
        // selectors); an IS-only card remains covered by its exact contract.
        let has_ideality = model
            .params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("N"));
        let has_series_resistance = model
            .params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("RS"));
        if !has_ideality || !has_series_resistance {
            // A bare `.MODEL name D` card is the canonical Xyce diode model:
            // all scalar parameters retain the documented SPICE defaults
            // (IS=1e-14, N=1, RS=0, CJO=0, TT=0, ...).  Keep this separate
            // from the parameterized rectifier envelope above so an
            // incomplete or ambiguous model cannot enter the absolute
            // transient contract merely by omitting N or RS.
            return model.params.is_empty() && model.expr_params.is_empty();
        }

        // The handwritten legacy evaluator consumes only this scalar subset.
        // Xyce's legacy model parser intentionally ignores unknown scalar
        // parameters, including values obtained by evaluating expressions
        // (BUG_45_SON exercises that behavior).  Permit those ignored values
        // only when they cannot alias a native parameter or the LEVEL selector
        // itself.
        model.params.iter().all(|(name, value)| {
            if Self::xyce_level2_native_diode_param(name) {
                Self::native_relational_diode_model_param(name, *value)
            } else {
                value.is_finite()
            }
        }) && model.expr_params.iter().all(|(name, _)| {
            !name.eq_ignore_ascii_case("LEVEL") && !Self::xyce_level2_native_diode_param(name)
        })
    }

    pub(super) fn netlist_element_is_native_xyce_level2_tbv_diode(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Diode {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 2
            && deferred_params.is_empty()
            && instance_params
                .iter()
                .all(|(name, value)| Self::native_xyce_level2_diode_instance_param(name, *value))
            && Self::native_xyce_level2_diode_effective_temperature_is_valid(
                netlist,
                instance_params,
            )
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_xyce_level2_tbv_diode)
    }

    pub(super) fn native_xyce_level2_diode_effective_temperature_is_valid(
        netlist: &Netlist,
        instance_params: &[(String, Value)],
    ) -> bool {
        let mut temp = None;
        let mut dtemp = None;
        for (name, value) in instance_params {
            if name.eq_ignore_ascii_case("TEMP") {
                if temp.replace(*value).is_some() {
                    return false;
                }
            } else if name.eq_ignore_ascii_case("DTEMP") && dtemp.replace(*value).is_some() {
                return false;
            }
        }
        if temp.is_some() && dtemp.is_some() {
            return false;
        }
        let effective_temp =
            temp.unwrap_or_else(|| netlist.options.temp.unwrap_or(27.0) + dtemp.unwrap_or(0.0));
        effective_temp.is_finite() && effective_temp > -273.15
    }

    pub(super) fn model_is_native_xyce_level2_tbv_diode(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) && (Self::numeric_param_value(&model.params, "LEVEL")
            .is_some_and(|level| (level - 2.0).abs() <= 1.0e-9))
            && model.params.iter().any(|(name, _)| {
                name.eq_ignore_ascii_case("TBV1") || name.eq_ignore_ascii_case("TBV2")
            })
            && Self::model_is_native_relational_legacy_diode(model)
    }

    pub(super) fn netlist_is_native_exact_is_diode_xyce_verify_envelope(netlist: &Netlist) -> bool {
        let diodes = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Diode { .. }))
            .collect::<Vec<_>>();
        if diodes.is_empty()
            || diodes.iter().any(|element| {
                !Self::netlist_element_is_native_absolute_transient_exact_is_diode(netlist, element)
            })
            || netlist.elements.iter().any(|element| {
                matches!(
                    element.kind,
                    ElementKind::Bjt { .. } | ElementKind::Mosfet { .. } | ElementKind::Jfet { .. }
                )
            })
        {
            return false;
        }

        let referenced_models = diodes
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Diode { model, .. } => Some(model.to_ascii_lowercase()),
                _ => None,
            })
            .collect::<BTreeSet<_>>();
        netlist.models.len() == referenced_models.len()
            && netlist.models.iter().all(|model| {
                referenced_models.contains(&model.name.to_ascii_lowercase())
                    && Self::model_is_native_exact_is_diode(model)
            })
    }

    pub(super) fn model_is_native_exact_is_diode(model: &rspice_core::netlist::ModelDef) -> bool {
        matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model.params.len() == 1
            && model.params[0].0.eq_ignore_ascii_case("IS")
            && model.params[0].1.is_finite()
            && model.params[0].1 > 0.0
    }

    /// Qualify the native legacy GP transient IRB/RBM path without admitting
    /// arbitrary BJT cards.  The envelope is intentionally structural: one
    /// bare NPN device, numeric-only supported GP model parameters, and only
    /// plain R/C passives plus independent sources around it.  Instance AREA,
    /// multiplicity, deferred bindings, parameter scopes, subcircuits, and
    /// model aliases remain outside this contract.
    pub(super) fn netlist_device_is_single_native_transient_level1_npn_irb(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        if !netlist.subcircuits.is_empty()
            || netlist.models.len() != 1
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return false;
        }

        let topology_is_qualified = match element.nodes.as_slice() {
            [_, _, _] => true,
            [_, _, _, substrate] => Self::node_name_is_ground(substrate),
            _ => false,
        };
        let bjt_elements = netlist
            .elements
            .iter()
            .filter(|candidate| matches!(candidate.kind, ElementKind::Bjt { .. }))
            .collect::<Vec<_>>();
        if bjt_elements.len() != 1
            || !std::ptr::eq(bjt_elements[0], element)
            || !topology_is_qualified
        {
            return false;
        }
        let ElementKind::Bjt {
            model,
            bjt_type: rspice_core::netlist::BjtType::Npn,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        if !instance_params.is_empty() || !deferred_params.is_empty() {
            return false;
        }
        let Some(model_def) = Self::find_model(&netlist.models, model) else {
            return false;
        };
        if !Self::model_is_native_transient_level1_npn_irb(model_def) {
            return false;
        }

        // Do not silently qualify a surrounding nonlinear or parameterized
        // element merely because the BJT itself matches the model envelope.
        netlist.elements.iter().all(|candidate| {
            if std::ptr::eq(candidate, element) {
                return true;
            }
            match &candidate.kind {
                ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } => {
                    value.is_finite()
                        && *value >= 0.0
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                }
                ElementKind::Capacitor {
                    value,
                    value_expr: None,
                    initial_voltage: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } => {
                    value.is_finite()
                        && *value >= 0.0
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                }
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    Self::native_transient_independent_source_spec(spec)
                }
                _ => false,
            }
        })
    }

    pub(super) fn model_is_native_transient_level1_npn_irb(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NPN")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        if Self::numeric_param_value(&model.params, "LEVEL")
            .is_some_and(|level| !level.is_finite() || (level - 1.0).abs() > 1.0e-9)
        {
            return false;
        }

        let mut seen = BTreeSet::new();
        for (name, value) in &model.params {
            let normalized = name.to_ascii_uppercase();
            if !seen.insert(normalized.clone())
                || !Self::native_transient_level1_npn_irb_model_param(&normalized, *value)
            {
                return false;
            }
        }
        let parameter = |name: &str| {
            model
                .params
                .iter()
                .find(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value)
        };
        let Some(rb) = parameter("RB") else {
            return false;
        };
        let Some(rbm) = parameter("RBM") else {
            return false;
        };
        let Some(irb) = parameter("IRB") else {
            return false;
        };
        rb.is_finite() && rb > rbm && rbm > 0.0 && irb.is_finite() && irb > 0.0
    }

    pub(super) fn native_transient_level1_npn_irb_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name {
            "LEVEL" => (value - 1.0).abs() <= 1.0e-9,
            "BF" | "BR" | "IS" | "NF" | "NE" | "NR" | "NC" | "VAF" | "VAR" | "VJE" | "VJC"
            | "VJS" | "EG" | "XTI" => value > 0.0,
            "IKF" | "ISE" | "IKR" | "ISC" | "RB" | "IRB" | "RBM" | "RE" | "RC" | "CJS" | "CJE"
            | "CJC" | "TF" | "TR" => value >= 0.0,
            "MJS" | "MJE" | "MJC" | "FC" => (0.0..1.0).contains(&value),
            "XTB" => true,
            _ => false,
        }
    }

    pub(super) fn netlist_element_is_native_transient_level1_npn(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        let topology_is_qualified = match element.nodes.as_slice() {
            [_, _, _] => true,
            [_, _, _, substrate] => Self::node_name_is_ground(substrate),
            _ => false,
        };
        topology_is_qualified
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_model(&netlist.models, model)
                .is_some_and(Self::model_is_native_transient_level1_npn)
    }

    pub(super) fn netlist_element_is_native_absolute_transient_level1_gp_npn(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        let topology_is_qualified = match element.nodes.as_slice() {
            [_, _, _] => true,
            [_, _, _, substrate] => Self::node_name_is_ground(substrate),
            _ => false,
        };
        topology_is_qualified
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_native_absolute_transient_level1_gp_npn)
    }

    pub(super) fn netlist_is_native_absolute_transient_level1_gp_npn(netlist: &Netlist) -> bool {
        let bjts = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Bjt { .. }))
            .collect::<Vec<_>>();
        bjts.len() == 1
            && bjts.iter().all(|element| {
                Self::netlist_element_is_native_absolute_transient_level1_gp_npn(netlist, element)
            })
    }

    pub(super) fn model_is_native_absolute_transient_level1_gp_npn(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NPN")
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }

        let mut names = BTreeSet::new();
        model.params.iter().all(|(name, value)| {
            let name = name.to_ascii_uppercase();
            names.insert(name.clone())
                && match name.as_str() {
                    "LEVEL" => value.is_finite() && (*value - 1.0).abs() <= 1.0e-9,
                    "TNOM" => value.is_finite() && *value > -273.15,
                    _ => Self::native_full_level1_gp_bjt_model_param(&name, *value),
                }
        })
    }

    pub(super) fn netlist_element_is_native_transient_level1_gp_bjt(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        let topology_is_qualified = match element.nodes.as_slice() {
            [_, _, _] => true,
            [_, _, _, substrate] => Self::node_name_is_ground(substrate),
            _ => false,
        };
        topology_is_qualified
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_model(&netlist.models, model)
                .is_some_and(Self::model_is_native_transient_level1_gp_bjt)
    }

    pub(super) fn netlist_is_native_transient_level1_gp_bjt_network(netlist: &Netlist) -> bool {
        let bjt_elements: Vec<_> = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Bjt { .. }))
            .collect();
        // The absolute transient envelope is deliberately bounded: the
        // native legacy GP equations are validated here only through the
        // four-device amplifier topology. Larger arbitrary BJT networks
        // remain fail-closed until their own transient oracle is qualified.
        !bjt_elements.is_empty()
            && bjt_elements.len() <= 4
            && bjt_elements.iter().all(|element| {
                Self::netlist_element_is_native_transient_level1_gp_bjt(netlist, element)
            })
    }

    pub(super) fn netlist_is_native_bug805_bjt_envelope(netlist: &Netlist) -> bool {
        netlist.elements.len() == 20
            && netlist
                .elements
                .iter()
                .filter(|element| matches!(element.kind, ElementKind::Bjt { .. }))
                .count()
                == 1
            && netlist.elements.iter().all(|element| {
                !matches!(element.kind, ElementKind::Bjt { .. })
                    || Self::netlist_element_is_native_bug805_bjt(netlist, element)
            })
    }

    pub(super) fn netlist_element_is_native_bug805_bjt(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Bjt {
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        let topology_is_qualified = match element.nodes.as_slice() {
            [_, _, _] => true,
            [_, _, _, substrate] => Self::node_name_is_ground(substrate),
            _ => false,
        };
        topology_is_qualified
            && instance_params.is_empty()
            && deferred_params.is_empty()
            && Self::find_model(&netlist.models, model)
                .is_some_and(Self::model_is_native_bug805_bjt)
    }

    fn model_is_native_bug805_bjt(model: &rspice_core::netlist::ModelDef) -> bool {
        model.model_type.eq_ignore_ascii_case("NPN")
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model
                .params
                .iter()
                .all(|(name, value)| Self::native_full_level1_gp_bjt_model_param(name, *value))
    }

    fn native_full_level1_gp_bjt_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name.to_ascii_uppercase().as_str() {
            "BF" | "BFM" | "BR" | "BRM" | "IS" | "NF" | "NR" | "NE" | "NLE" | "NC" | "VJE"
            | "PE" | "VJC" | "PC" | "VAF" | "VA" | "VBF" | "VAR" | "VB" | "VRB" | "VJS" | "PS"
            | "PSUB" | "EG" | "XTI" | "PT" | "AF" => value > 0.0,
            "IKF" | "IK" | "JBF" | "ISE" | "JLE" | "IKR" | "JBR" | "ISC" | "JLC" | "IRB"
            | "JRB" | "IOB" | "RB" | "RBM" | "RC" | "RE" | "CJS" | "CCS" | "CSUB" | "CJE"
            | "CJC" | "TF" | "TR" | "ITF" | "JTF" | "VTF" | "KF" => value >= 0.0,
            "MJE" | "ME" | "MJC" | "MC" | "MJS" | "MS" | "ESUB" | "FC" => {
                (0.0..1.0).contains(&value)
            }
            "XCJC" | "CDIS" => (0.0..=1.0).contains(&value),
            "XTF" | "PTF" | "XTB" | "TB" | "TCB" => true,
            _ => false,
        }
    }

    pub(super) fn model_is_native_transient_level1_gp_bjt(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !(model.model_type.eq_ignore_ascii_case("NPN")
            || model.model_type.eq_ignore_ascii_case("PNP"))
            || Self::numeric_param_value(&model.params, "LEVEL")
                .is_some_and(|level| !level.is_finite() || (level - 1.0).abs() > 1.0e-9)
        {
            return false;
        }
        model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model
                .params
                .iter()
                .all(|(name, value)| Self::native_transient_level1_gp_model_param(name, *value))
    }

    pub(super) fn native_transient_level1_gp_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name.to_ascii_uppercase().as_str() {
            "LEVEL" => (value - 1.0).abs() <= 1.0e-9,
            "BF" | "BR" | "IS" | "NF" | "NR" | "NE" | "NC" | "VJE" | "VJC" | "VAF" | "VAR"
            | "EG" | "XTI" => value > 0.0,
            "IKF" | "ISE" | "IKR" | "ISC" | "RB" | "RBM" | "RC" | "RE" | "CJS" | "CJE" | "CJC"
            | "TF" | "TR" => value >= 0.0,
            "MJE" | "MJC" | "FC" => (0.0..1.0).contains(&value),
            "XTB" | "TNOM" => true,
            _ => false,
        }
    }

    pub(super) fn model_is_native_transient_level1_npn(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !model.model_type.eq_ignore_ascii_case("NPN")
            || Self::numeric_param_value(&model.params, "LEVEL")
                .is_some_and(|level| !level.is_finite() || (level - 1.0).abs() > 1.0e-9)
        {
            return false;
        }
        model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model
                .params
                .iter()
                .all(|(name, value)| Self::native_transient_level1_npn_model_param(name, *value))
    }

    pub(super) fn native_transient_level1_npn_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name.to_ascii_uppercase().as_str() {
            "LEVEL" => (value - 1.0).abs() <= 1.0e-9,
            "BF" | "BR" | "IS" | "NE" | "NC" | "VJE" | "VJC" | "VAF" | "EG" => value > 0.0,
            "RB" | "RC" | "RE" | "CJS" | "CJE" | "CJC" | "TF" | "TR" => value >= 0.0,
            "MJE" | "MJC" => (0.0..1.0).contains(&value),
            _ => false,
        }
    }

    pub(super) fn netlist_element_is_native_transient_level1_mosfet(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        // The original absolute transient envelope validates one complementary
        // Level-1 pair.  Larger networks use the separately qualified CMOS
        // inverter-chain topology below; arbitrary multi-device networks stay
        // fail-closed until they have their own propagation oracle.
        if netlist
            .elements
            .iter()
            .filter(|candidate| matches!(candidate.kind, ElementKind::Mosfet { .. }))
            .count()
            > 2
        {
            return false;
        }
        Self::netlist_element_is_native_transient_level1_mosfet_unbounded(netlist, element)
    }

    pub(super) fn netlist_element_is_native_transient_level1_mosfet_unbounded(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        if element.nodes.len() != 4
            || *compact_syntax
            || !deferred_params.is_empty()
            || !Self::native_transient_level1_mos_instance_params_are_valid(instance_params)
        {
            return false;
        }
        Self::find_unique_model_in(&netlist.models, model)
            .is_some_and(Self::model_is_native_transient_level1_mosfet)
    }

    /// Validate the bounded native LEVEL=2 MOSFET envelope used by the
    /// absolute transient oracle.  The native evaluator follows Berkeley's
    /// MOS2 equations, including the VMAX/NEFF short-channel path, but the
    /// absolute corpus contract remains deliberately bounded until a larger
    /// multi-device propagation family has its own checked-in oracle.
    pub(super) fn netlist_is_native_transient_level2_mosfet_network(netlist: &Netlist) -> bool {
        if netlist.options.abstol.is_some() || netlist.options.timeint_abstol.is_some() {
            return false;
        }
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.is_empty() || mosfets.len() > 4 {
            return false;
        }
        mosfets.iter().all(|element| {
            Self::netlist_element_is_native_transient_level2_mosfet(netlist, element)
        })
    }

    pub(super) fn netlist_element_is_native_transient_level2_mosfet(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        if element.nodes.len() != 4
            || *compact_syntax
            || !deferred_params.is_empty()
            || !Self::native_transient_level1_mos_instance_params_are_valid(instance_params)
        {
            return false;
        }
        Self::find_unique_model_in(&netlist.models, model)
            .is_some_and(Self::model_is_native_transient_level2_mosfet)
    }

    pub(super) fn model_is_native_transient_level2_mosfet(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !Self::numeric_param_value(&model.params, "LEVEL")
            .is_some_and(|level| level.is_finite() && level.to_bits() == 2.0f64.to_bits())
        {
            return false;
        }
        if !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut names = BTreeSet::new();
        model.params.iter().all(|(name, value)| {
            let normalized = name.to_ascii_uppercase();
            names.insert(normalized.clone())
                && Self::native_transient_level2_mosfet_model_param(&normalized, *value)
        })
    }

    pub(super) fn native_transient_level2_mosfet_model_param(name: &str, value: Value) -> bool {
        if !value.is_finite() {
            return false;
        }
        match name {
            "LEVEL" => value.to_bits() == 2.0f64.to_bits(),
            "VTO" | "VT0" | "VTH0" | "LAMBDA" | "XTI" | "EG" => true,
            "KP" | "PHI" | "U0" | "UO" | "TOX" | "NSUB" | "L" | "W" | "PB" | "NEFF" | "UCRIT" => {
                value > 0.0
            }
            "GAMMA" | "NSS" | "NFS" | "IS" | "JS" | "LD" | "RD" | "RS" | "RSH" | "CBD"
            | "CAPBD" | "CBS" | "CAPBS" | "CJ" | "CJ0" | "CJSW" | "CGSO" | "CGDO" | "CGBO"
            | "DELTA" | "UEXP" | "VMAX" | "XJ" => value >= 0.0,
            "TPG" | "GATE" => matches!(value, -1.0 | 0.0 | 1.0),
            // Berkeley/Xyce MOS2 permits exponents above one (for example
            // MOSRECT's NMOS MJ=1.067); the evaluator consumes the numeric
            // value directly, so the structural contract only rejects
            // negative junction grading exponents.
            "MJ" | "MJSW" => value >= 0.0,
            "FC" => (0.0..1.0).contains(&value),
            "TNOM" => value > -273.15,
            _ => false,
        }
    }

    /// Validate the bounded native LEVEL=3 MOSFET envelope used by the
    /// absolute transient oracle.  MOS3 shares the classic evaluator with
    /// the already validated relational MOS3 family; this route admits only
    /// four-terminal, scalar-parameter devices and at most four flattened
    /// instances so the absolute waveform contract remains independently
    /// bounded.
    pub(super) fn netlist_is_native_transient_level3_mosfet_network(netlist: &Netlist) -> bool {
        if netlist.options.abstol.is_some() || netlist.options.timeint_abstol.is_some() {
            return false;
        }
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.is_empty() || mosfets.len() > 4 {
            return false;
        }
        mosfets.iter().all(|element| {
            Self::netlist_element_is_native_transient_level3_mosfet(netlist, element)
        })
    }

    pub(super) fn netlist_element_is_native_transient_level3_mosfet(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 4
            && !compact_syntax
            && deferred_params.is_empty()
            && Self::find_unique_model_in(&netlist.models, model).is_some()
            && Self::netlist_device_is_native_relational_mos3(netlist, &element.name)
    }

    /// Validate a strictly linear CMOS inverter chain built from the native
    /// Level-1 evaluator.  Every stage is one NMOS/PMOS pair sharing a drain
    /// and gate, with all NMOS source/bulk terminals at ground and all PMOS
    /// source/bulk terminals on one common non-ground rail.  The stage graph
    /// must be one acyclic path: exactly one input gate is not a prior output,
    /// exactly one final output is not consumed by a later gate, and every
    /// other stage output is consumed exactly once.
    pub(super) fn netlist_is_native_transient_level1_cmos_chain(netlist: &Netlist) -> bool {
        // The checked-in chain oracle is stable with the default convergence
        // policy and TIMEINT RELTOL-only overrides.  Absolute convergence
        // floors change switching-edge acceptance; keep those decks outside
        // this propagation envelope until they have their own oracle.
        if netlist.options.abstol.is_some() || netlist.options.timeint_abstol.is_some() {
            return false;
        }
        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.len() < 4 || mosfets.len() % 2 != 0 {
            return false;
        }

        let mut pairs = BTreeMap::<(String, String), (bool, bool)>::new();
        let mut nmos_models = BTreeSet::new();
        let mut pmos_models = BTreeSet::new();
        let mut pmos_rail = None::<String>;
        for element in &mosfets {
            if !Self::netlist_element_is_native_transient_level1_mosfet_unbounded(netlist, element)
            {
                return false;
            }
            let ElementKind::Mosfet { model, .. } = &element.kind else {
                return false;
            };
            let Some(model_def) = Self::find_unique_model_in(&netlist.models, model) else {
                return false;
            };
            let normalized_model = model_def.name.to_ascii_uppercase();
            let [drain, gate, source, bulk] = element.nodes.as_slice() else {
                return false;
            };
            let drain = drain.to_ascii_uppercase();
            let gate = gate.to_ascii_uppercase();
            let source = source.to_ascii_uppercase();
            let bulk = bulk.to_ascii_uppercase();
            let is_nmos = model_def.model_type.eq_ignore_ascii_case("NMOS");
            let is_pmos = model_def.model_type.eq_ignore_ascii_case("PMOS");
            if is_nmos == is_pmos {
                return false;
            }
            if is_nmos {
                if !Self::node_name_is_ground(&source) || !Self::node_name_is_ground(&bulk) {
                    return false;
                }
                nmos_models.insert(normalized_model);
            } else {
                if Self::node_name_is_ground(&source)
                    || source != bulk
                    || pmos_rail.as_ref().is_some_and(|rail| rail != &source)
                {
                    return false;
                }
                pmos_rail = Some(source);
                pmos_models.insert(normalized_model);
            }
            let pair = pairs.entry((drain, gate)).or_default();
            let slot = if is_nmos { &mut pair.0 } else { &mut pair.1 };
            if *slot {
                return false;
            }
            *slot = true;
        }

        let Some(pmos_rail) = pmos_rail else {
            return false;
        };
        if nmos_models.len() != 1 || pmos_models.len() != 1 {
            return false;
        }
        if !netlist.elements.iter().any(|element| {
            if !matches!(element.kind, ElementKind::VoltageSource(_)) || element.nodes.len() != 2 {
                return false;
            }
            let first = element.nodes[0].to_ascii_uppercase();
            let second = element.nodes[1].to_ascii_uppercase();
            (first == pmos_rail && Self::node_name_is_ground(&second))
                || (second == pmos_rail && Self::node_name_is_ground(&first))
        }) {
            return false;
        }
        if pairs
            .values()
            .any(|(has_nmos, has_pmos)| !has_nmos || !has_pmos)
        {
            return false;
        }

        let mut next_by_gate = BTreeMap::new();
        let mut gates = BTreeSet::new();
        let mut drains = BTreeSet::new();
        for (drain_gate, (has_nmos, has_pmos)) in pairs {
            debug_assert!(has_nmos && has_pmos);
            let (drain, gate) = drain_gate;
            if drain == gate || gates.contains(&gate) || !drains.insert(drain.clone()) {
                return false;
            }
            gates.insert(gate.clone());
            next_by_gate.insert(gate, drain);
        }

        let roots = gates.difference(&drains).cloned().collect::<Vec<_>>();
        let terminals = drains.difference(&gates).cloned().collect::<Vec<_>>();
        if roots.len() != 1 || terminals.len() != 1 {
            return false;
        }
        let mut visited = BTreeSet::new();
        let mut gate = roots[0].clone();
        loop {
            let Some(drain) = next_by_gate.get(&gate) else {
                return visited.len() == mosfets.len() / 2
                    && gate.eq_ignore_ascii_case(&terminals[0]);
            };
            if !visited.insert(gate.clone()) {
                return false;
            }
            gate = drain.clone();
        }
    }

    pub(super) fn model_is_native_transient_level1_mosfet(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        Self::model_is_native_dc_analysis_expression_mos1(model)
            && Self::numeric_param_value(&model.params, "LEVEL")
                .is_some_and(|level| level.to_bits() == 1.0f64.to_bits())
    }

    /// Return true for the canonical generated BSIM-SOI 4.6.1 LEVEL=70
    /// four-terminal surface. Circuit topology is intentionally unrestricted:
    /// the generated artifact, rather than a deck-specific conformance rule,
    /// owns the compact-model equations and parameter validation.
    pub(super) fn netlist_element_is_generated_bsimsoi461(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        if !cfg!(feature = "veriloga-model-bsimsoi-va") || netlist.options.gmin.is_some() {
            return false;
        }
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        element.nodes.len() == 4
            && !compact_syntax
            && deferred_params.is_empty()
            && instance_params
                .iter()
                .all(|(name, value)| !name.trim().is_empty() && value.is_finite())
            && Self::find_unique_model_in(&netlist.models, model)
                .is_some_and(Self::model_is_generated_bsimsoi461)
    }

    pub(super) fn model_is_generated_bsimsoi461(model: &rspice_core::netlist::ModelDef) -> bool {
        let expected_type: Value = if model.model_type.eq_ignore_ascii_case("NMOS") {
            1.0
        } else if model.model_type.eq_ignore_ascii_case("PMOS") {
            -1.0
        } else {
            return false;
        };
        if !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }

        let mut values = HashMap::with_capacity(model.params.len());
        for (name, value) in &model.params {
            if name.trim().is_empty()
                || !value.is_finite()
                || values.insert(name.to_ascii_uppercase(), *value).is_some()
            {
                return false;
            }
        }
        values
            .get("LEVEL")
            .is_some_and(|level| level.to_bits() == 70.0f64.to_bits())
            && values
                .get("VERSION")
                .is_none_or(|version| version.to_bits() == 4.6f64.to_bits())
            && values
                .get("TYPE")
                .is_none_or(|model_type| model_type.to_bits() == expected_type.to_bits())
    }

    /// Return true only for an element that is validated by the native EKV
    /// 2.6 evaluator used by the engine's LEVEL=260 route.
    ///
    /// The runner deliberately asks the canonical native constructor to
    /// validate the numeric model/instance domains instead of maintaining a
    /// second copy of EKV26's parameter equations here.  The surrounding
    /// structural checks keep generated Verilog-A model names, deferred
    /// expressions, compact syntax, and ambiguous geometry out of this
    /// absolute Xyce oracle contract.
    pub(super) fn netlist_element_is_native_transient_ekv26(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        if element.nodes.len() != 4
            || *compact_syntax
            || !deferred_params.is_empty()
            || !Self::native_transient_ekv26_instance_params_are_valid(instance_params)
        {
            return false;
        }
        let Some(model) = Self::find_unique_model_in(&netlist.models, model) else {
            return false;
        };
        let Some((mos_type, model_params)) = Self::native_transient_ekv26_model_params(model)
        else {
            return false;
        };
        rspice_core::device::EkvMosfet::from_params(
            "__xyce_ekv26_contract__".to_string(),
            0,
            0,
            0,
            0,
            mos_type,
            &model_params,
            instance_params,
            300.15,
        )
        .is_ok()
    }

    /// The validated absolute EKV26 transient envelope is the complementary
    /// two-device inverter topology reference-backed by Xyce 7.10.  Larger
    /// chains and arbitrary same-polarity networks need their own transient
    /// propagation oracle and remain fail-closed.
    pub(super) fn netlist_is_native_transient_ekv26_pair(netlist: &Netlist) -> bool {
        if netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }))
        {
            let Ok(flattened) = flatten_netlist_with_models(netlist) else {
                return false;
            };
            let mut flat_netlist = netlist.clone();
            flat_netlist.elements = flattened.elements;
            flat_netlist.models.extend(flattened.scoped_models);
            flat_netlist.subcircuits.clear();
            return Self::netlist_is_native_transient_ekv26_pair(&flat_netlist);
        }

        let mosfets = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mosfets.len() != 2
            || mosfets
                .iter()
                .any(|element| !Self::netlist_element_is_native_transient_ekv26(netlist, element))
        {
            return false;
        }

        let mut model_types = BTreeSet::new();
        for element in mosfets {
            let ElementKind::Mosfet { model, .. } = &element.kind else {
                return false;
            };
            let Some(model) = Self::find_unique_model_in(&netlist.models, model) else {
                return false;
            };
            if !Self::model_is_native_transient_ekv26(model) {
                return false;
            }
            model_types.insert(model.model_type.to_ascii_uppercase());
        }
        model_types.len() == 2 && model_types.contains("NMOS") && model_types.contains("PMOS")
    }

    pub(super) fn model_is_native_transient_ekv26(model: &rspice_core::netlist::ModelDef) -> bool {
        let Some((mos_type, model_params)) = Self::native_transient_ekv26_model_params(model)
        else {
            return false;
        };
        rspice_core::device::EkvMosfet::from_params(
            "__xyce_ekv26_contract__".to_string(),
            0,
            0,
            0,
            0,
            mos_type,
            &model_params,
            &[],
            300.15,
        )
        .is_ok()
    }

    pub(super) fn native_transient_ekv26_model_params(
        model: &rspice_core::netlist::ModelDef,
    ) -> Option<(rspice_core::device::MosType, HashMap<String, Value>)> {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return None;
        }

        let mut names = BTreeSet::new();
        let mut model_params = HashMap::with_capacity(model.params.len());
        for (name, value) in &model.params {
            let normalized = name.to_ascii_uppercase();
            if !names.insert(normalized.clone()) || !value.is_finite() {
                return None;
            }
            model_params.insert(normalized, *value);
        }
        if Self::numeric_param_value(&model.params, "LEVEL")
            .is_none_or(|level| level.to_bits() != 260.0f64.to_bits())
        {
            return None;
        }

        let mos_type = if model.model_type.eq_ignore_ascii_case("NMOS") {
            rspice_core::device::MosType::Nmos
        } else {
            rspice_core::device::MosType::Pmos
        };
        Some((mos_type, model_params))
    }

    pub(super) fn netlist_device_is_native_legacy_bjt(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        if Self::elements_device_is_native_legacy_bjt(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_legacy_bjt(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    pub(super) fn netlist_device_is_native_static_ac_legacy_npn_bjt(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        // The high-frequency GP companion is qualified only for a flat
        // netlist.  Flattened hierarchy has a separate exact BF/IS contract
        // with a 20 kHz limit; do not let the broader top-level model
        // predicate bypass that scoped-model boundary.
        if !netlist.subcircuits.is_empty() {
            return false;
        }
        if Self::elements_device_is_native_static_ac_legacy_npn_bjt(
            &netlist.elements,
            &netlist.models,
            &[],
            instance_name,
        ) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_device_is_native_static_ac_legacy_npn_bjt(
                &flattened.elements,
                &netlist.models,
                &flattened.scoped_models,
                instance_name,
            )
        })
    }

    pub(super) fn model_is_native_legacy_bjt(model: &rspice_core::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NPN" | "PNP" | "LPNP"
        ) {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL")
            .is_none_or(|level| level.is_finite() && (level - 1.0).abs() <= 1.0e-9)
    }

    pub(super) fn model_is_native_static_ac_legacy_npn_bjt(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        // The native AC companion uses the same scalar Level-1 Gummel-Poon
        // parameter subset as the validated transient path.  Keep expression,
        // string, and vector routes out of this high-frequency envelope; the
        // flat-netlist guard above keeps scoped hierarchy on its narrower
        // exact BF/IS contract.  Requiring one authored GP parameter beyond
        // LEVEL/IS/BF is the model-level discriminator that survives flattening.
        let mut parameter_names = BTreeSet::new();
        if model
            .params
            .iter()
            .any(|(name, _)| !parameter_names.insert(name.to_ascii_uppercase()))
        {
            return false;
        }
        let has_extended_gp_parameter = model
            .params
            .iter()
            .any(|(name, _)| !matches!(name.to_ascii_uppercase().as_str(), "LEVEL" | "IS" | "BF"));
        has_extended_gp_parameter && Self::model_is_native_transient_level1_npn(model)
    }

    pub(super) fn model_is_native_legacy_diode(model: &rspice_core::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "D" | "DIODE"
        ) {
            return false;
        }
        let Some(level) = Self::numeric_param_value(&model.params, "LEVEL") else {
            return true;
        };
        if !level.is_finite() {
            return false;
        }
        if (level - 0.0).abs() <= 1.0e-9 || (level - 1.0).abs() <= 1.0e-9 {
            return true;
        }
        (level - 2.0).abs() <= 1.0e-9 && Self::model_is_native_xyce_level2_diode_subset(model)
    }

    pub(super) fn model_is_native_legacy_resistor(model: &rspice_core::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "R" | "RES" | "RESISTOR"
        ) {
            return false;
        }
        if model
            .expr_params
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
            || model
                .string_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case("LEVEL"))
        {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL").is_none_or(|level| {
            level.is_finite() && ((level - 0.0).abs() <= 1.0e-9 || (level - 1.0).abs() <= 1.0e-9)
        })
    }

    pub(super) fn model_is_native_xyce_level2_diode_subset(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
            && model.params.iter().all(|(name, value)| {
                value.is_finite() && Self::xyce_level2_native_diode_param(name)
            })
    }

    pub(super) fn netlist_device_is_single_native_ac_supported_bulk_mosfet(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        let mut matched = false;
        let mut count = 0usize;
        for element in &netlist.elements {
            if !matches!(element.kind, ElementKind::Mosfet { .. }) {
                continue;
            }
            if !Self::netlist_element_is_native_ac_supported_bulk_mosfet(netlist, element) {
                return false;
            }
            count += 1;
            matched |= Self::device_instance_names_match(&element.name, instance_name);
        }
        matched && count == 1
    }

    /// The native BSIM3v3.3 port has a separately validated AC envelope for
    /// Xyce LEVEL=9 cards that use only the canonical L/W instance geometry
    /// and the model's LEVEL/VERSION selectors.  Keep this admission narrow:
    /// unsupported BSIM3 parameter surfaces must not silently fall through to
    /// a different compact-model implementation.
    pub(super) fn netlist_device_is_single_native_ac_supported_bsim3(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        let mut matched = false;
        let mut count = 0usize;
        for element in &netlist.elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            count += 1;
            matched |= Self::device_instance_names_match(&element.name, instance_name);
            if element.nodes.len() != 4
                || *compact_syntax
                || !deferred_params.is_empty()
                || !Self::native_ac_bsim3_instance_params_are_valid(instance_params)
                || !Self::find_unique_model_in(&netlist.models, model)
                    .is_some_and(Self::model_is_native_ac_supported_bsim3)
            {
                return false;
            }
        }
        matched && count == 1
    }

    /// The native BSIMSOI AC path is currently validated for the Xyce
    /// BSIMSOI3 dynamic-depletion route selected by an explicit LEVEL=10 /
    /// SOIMOD=1 model card.  Keep the admission structural and fail closed
    /// around compact/deferred geometry, instance SOIMOD overrides, and
    /// non-literal model parameters; the native B3SOIDD constructor remains
    /// the source of truth for the complete numeric model surface.
    pub(super) fn netlist_device_is_single_native_ac_supported_b3soi(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        let mut matched = false;
        let mut count = 0usize;
        for element in &netlist.elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            count += 1;
            matched |= Self::device_instance_names_match(&element.name, instance_name);
            if element.nodes.len() != 4
                || *compact_syntax
                || !deferred_params.is_empty()
                || !Self::native_ac_b3soi_instance_params_are_valid(instance_params)
                || !Self::find_unique_model_in(&netlist.models, model)
                    .is_some_and(Self::model_is_native_ac_supported_b3soi)
            {
                return false;
            }
        }
        matched && count == 1
    }

    pub(super) fn model_is_native_ac_supported_b3soi(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }

        let mut names = BTreeSet::new();
        let mut level = false;
        let mut soimod = false;
        for (name, value) in &model.params {
            let key = name.to_ascii_uppercase();
            if !value.is_finite() || !names.insert(key.clone()) {
                return false;
            }
            match key.as_str() {
                "LEVEL" if (*value - 10.0).abs() <= 1.0e-9 => level = true,
                "SOIMOD" if (*value - 1.0).abs() <= 1.0e-9 => soimod = true,
                _ => {}
            }
        }
        level && soimod
    }

    pub(super) fn model_is_native_ac_supported_bsim3(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut level = None;
        for (name, value) in &model.params {
            if !value.is_finite() {
                return false;
            }
            match name.to_ascii_uppercase().as_str() {
                "LEVEL" if level.is_none() && (*value - 9.0).abs() <= 1.0e-9 => level = Some(()),
                _ => return false,
            }
        }
        if level.is_none() {
            return false;
        }
        model.string_params.iter().all(|(name, value)| {
            name.eq_ignore_ascii_case("VERSION")
                && matches!(value.to_ascii_lowercase().as_str(), "3.2.2" | "3.3.0")
        })
    }

    /// The native BSIM4 v4.8 port has a separately validated AC envelope for
    /// Xyce LEVEL=14/54 cards.  Keep the runner admission fail-closed around
    /// the same canonical external geometry used by the native builder: one
    /// four-terminal MOSFET with literal positive L/W and a numeric BSIM4
    /// model card.  Model construction remains the source of truth for the
    /// full BSIM4 parameter/selectors surface; this guard only prevents
    /// compact/deferred cards and non-native model families from falling
    /// through to a simplified MOS implementation.
    pub(super) fn netlist_device_is_single_native_ac_supported_bsim4(
        netlist: &Netlist,
        instance_name: &str,
    ) -> bool {
        let mut matched = false;
        let mut count = 0usize;
        for element in &netlist.elements {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                continue;
            };
            count += 1;
            matched |= Self::device_instance_names_match(&element.name, instance_name);
            if element.nodes.len() != 4
                || *compact_syntax
                || !deferred_params.is_empty()
                || !Self::native_ac_bsim4_instance_params_are_valid(instance_params)
                || !Self::find_unique_model_in(&netlist.models, model)
                    .is_some_and(Self::model_is_native_ac_supported_bsim4)
            {
                return false;
            }
        }
        matched && count == 1
    }

    pub(super) fn model_is_native_ac_supported_bsim4(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut params = HashMap::with_capacity(model.params.len() + 1);
        let mut names = BTreeSet::new();
        let mut level = None;
        for (name, value) in &model.params {
            let key = name.to_ascii_uppercase();
            if !value.is_finite() || !names.insert(key.clone()) {
                return false;
            }
            if key == "LEVEL" {
                if ![14.0, 54.0].contains(value) {
                    return false;
                }
                level = Some(());
            }
            params.insert(key, *value);
        }
        if level.is_none() {
            return false;
        }
        for (name, value) in &model.string_params {
            if !name.eq_ignore_ascii_case("VERSION") {
                return false;
            }
            let version = value.trim().trim_matches(['"', '\'']);
            let mut parts = version.split('.');
            let Some(major) = parts.next() else {
                return false;
            };
            let Some(minor) = parts.next() else {
                return false;
            };
            if major.is_empty()
                || minor.is_empty()
                || !major.chars().all(|ch| ch.is_ascii_digit())
                || !minor.chars().all(|ch| ch.is_ascii_digit())
                || parts
                    .clone()
                    .any(|part| part.is_empty() || !part.chars().all(|ch| ch.is_ascii_digit()))
            {
                return false;
            }
            let patch = parts.collect::<Vec<_>>().join("");
            let Ok(version_value) = format!("{major}.{minor}{patch}").parse::<Value>() else {
                return false;
            };
            if !version_value.is_finite()
                || params
                    .insert("VERSION".to_string(), version_value)
                    .is_some()
            {
                return false;
            }
        }
        let Ok(native_model) = rspice_core::device::Bsim4v8Model::try_from_params(
            &params,
            model.model_type.eq_ignore_ascii_case("PMOS"),
            300.15,
        ) else {
            return false;
        };
        native_model.cvcharge_mod_supported_for_charges() || native_model.xpart < 0.0
    }

    pub(super) fn netlist_element_is_native_ac_supported_bulk_mosfet(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::Mosfet {
            model,
            compact_syntax,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return false;
        };
        if element.nodes.len() != 4
            || *compact_syntax
            || !deferred_params.is_empty()
            || !Self::native_ac_classic_mos_instance_params_are_valid(instance_params)
        {
            return false;
        }
        Self::find_unique_model_in(&netlist.models, model)
            .is_some_and(Self::model_is_native_ac_supported_bulk_mosfet)
    }

    pub(super) fn model_is_native_ac_supported_bulk_mosfet(
        model: &rspice_core::netlist::ModelDef,
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        let mut names = BTreeSet::new();
        model.params.iter().all(|(name, value)| {
            value.is_finite()
                && match name.to_ascii_uppercase().as_str() {
                    "LEVEL" => {
                        [1.0, 2.0, 3.0, 6.0].contains(value) && names.insert("LEVEL".to_string())
                    }
                    "TOX" => *value > 0.0 && names.insert("TOX".to_string()),
                    _ => false,
                }
        }) && names.contains("LEVEL")
    }

    pub(super) fn model_is_native_classic_jfet(model: &rspice_core::netlist::ModelDef) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NJF" | "PJF"
        ) {
            return false;
        }
        Self::numeric_param_value(&model.params, "LEVEL")
            .is_none_or(|level| level.is_finite() && (level - 1.0).abs() <= 1.0e-9)
    }

    pub(super) fn model_is_native_b3soi_mosfet(
        model: &rspice_core::netlist::ModelDef,
        instance_params: &[(String, Value)],
    ) -> bool {
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "NMOS" | "PMOS"
        ) {
            return false;
        }
        let Some(level) = Self::numeric_param_value(&model.params, "LEVEL") else {
            return false;
        };
        if (level - 10.0).abs() <= 1.0e-9 {
            return Self::numeric_param_value(instance_params, "SOIMOD")
                .or_else(|| Self::numeric_param_value(&model.params, "SOIMOD"))
                .is_none_or(|soi_mod| {
                    soi_mod.is_finite()
                        && (soi_mod - soi_mod.round()).abs() <= 1.0e-12
                        && matches!(soi_mod.round() as i32, 0..=3)
                });
        }
        [55.0, 56.0, 57.0]
            .iter()
            .any(|native_level| (level - native_level).abs() <= 1.0e-9)
    }

    pub(super) fn netlist_element_exports_device_op(
        element: &rspice_core::netlist::Element,
    ) -> bool {
        matches!(
            &element.kind,
            ElementKind::Diode { .. }
                | ElementKind::Bjt { .. }
                | ElementKind::Mosfet { .. }
                | ElementKind::Jfet { .. }
                | ElementKind::Mesfet { .. }
        ) || matches!(
            &element.kind,
            ElementKind::Coupling {
                model: Some(_),
                inductors,
                ..
            } if !inductors.is_empty()
        )
    }

    pub(super) fn strict_expression_contains_file_table(expression: &Expr) -> bool {
        match expression {
            Expr::Function { func, args } => {
                let file_function = matches!(
                    func,
                    rspice_core::expr::Function::Table
                        | rspice_core::expr::Function::TableFile
                        | rspice_core::expr::Function::FastTable
                        | rspice_core::expr::Function::FastTableFile
                        | rspice_core::expr::Function::Cubic
                        | rspice_core::expr::Function::CubicFile
                        | rspice_core::expr::Function::Akima
                        | rspice_core::expr::Function::AkimaFile
                        | rspice_core::expr::Function::Wodicka
                        | rspice_core::expr::Function::WodickaFile
                        | rspice_core::expr::Function::Barycentric
                        | rspice_core::expr::Function::BarycentricFile
                ) && matches!(args.first(), Some(Expr::StringLiteral(_)));
                file_function || args.iter().any(Self::strict_expression_contains_file_table)
            }
            Expr::Unary { operand, .. } => Self::strict_expression_contains_file_table(operand),
            Expr::Binary { left, right, .. } => {
                Self::strict_expression_contains_file_table(left)
                    || Self::strict_expression_contains_file_table(right)
            }
            Expr::LookupTable { input, .. } => Self::strict_expression_contains_file_table(input),
            Expr::Const(_)
            | Expr::NodeVoltage(_)
            | Expr::BranchCurrent(_)
            | Expr::StringLiteral(_)
            | Expr::Time
            | Expr::Frequency
            | Expr::Temperature
            | Expr::ThermalVoltage
            | Expr::Gmin => false,
        }
    }

    pub(super) fn netlist_temperature_c(netlist: &Netlist) -> Value {
        netlist
            .options
            .temp
            .or_else(|| netlist.params.get("TEMP"))
            .or_else(|| netlist.params.get("TEMPER"))
            .unwrap_or(27.0)
    }

    pub(super) fn is_temperature_name(name: &str) -> bool {
        name.eq_ignore_ascii_case("TEMP") || name.eq_ignore_ascii_case("TEMPER")
    }

    pub(super) fn netlist_has_recorded_branch_current(netlist: &Netlist, source: &str) -> bool {
        if Self::elements_have_recorded_branch_current(&netlist.elements, source) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_have_recorded_branch_current(&flattened.elements, source)
        })
    }

    pub(super) fn semiconductor_model_parameter_is_supported(
        netlist: &Netlist,
        model_name: &str,
        parameter: &str,
    ) -> bool {
        Self::find_semiconductor_model(netlist, model_name).is_some_and(|model| {
            model
                .params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
                || model
                    .expr_params
                    .iter()
                    .any(|(name, _)| name.eq_ignore_ascii_case(parameter))
        })
    }

    pub(super) fn evaluate_semiconductor_model_parameter_probe(
        netlist: &Netlist,
        dc: &XyceDcSweep,
        sweep_point: XyceDcSweepPoint,
        model_name: &str,
        parameter: &str,
    ) -> Option<Result<Value, String>> {
        let model = Self::find_semiconductor_model(netlist, model_name)?;
        if let Some((_, value)) = model
            .params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
        {
            return Some(Ok(*value));
        }
        model
            .expr_params
            .iter()
            .rev()
            .find(|(name, _)| name.eq_ignore_ascii_case(parameter))
            .map(|(_, expression)| {
                let context = Self::print_eval_context(netlist, Some(dc), Some(sweep_point));
                rspice_core::netlist::expr::eval_expression(expression, &context).map_err(|err| {
                    format!(
                        "failed to evaluate semiconductor model parameter probe '{}:{}': {err}",
                        model_name, parameter
                    )
                })
            })
    }

    pub(super) fn semiconductor_model_and_instance_params(
        element: &rspice_core::netlist::Element,
    ) -> Option<(&str, &[(String, Value)])> {
        match &element.kind {
            ElementKind::Diode {
                model,
                instance_params,
                ..
            }
            | ElementKind::Bjt {
                model,
                instance_params,
                ..
            }
            | ElementKind::Mosfet {
                model,
                instance_params,
                ..
            }
            | ElementKind::Jfet {
                model,
                instance_params,
                ..
            }
            | ElementKind::Mesfet {
                model,
                instance_params,
                ..
            } => Some((model.as_str(), instance_params.as_slice())),
            _ => None,
        }
    }

    pub(super) fn netlist_has_diode_instance_with_index(
        netlist: &Netlist,
        source: &str,
        index: &mut XyceDcProbeIndex,
    ) -> bool {
        let normalized = Self::normalize_device_instance_name(source);
        if index.diode_names.contains(&normalized) {
            return true;
        }

        Self::populate_flattened_dc_probe_index(netlist, index);

        index
            .flattened_diode_names
            .as_ref()
            .is_some_and(|names| names.contains(&normalized))
    }

    pub(super) fn netlist_has_recorded_branch_current_with_index(
        netlist: &Netlist,
        source: &str,
        index: &mut XyceDcProbeIndex,
    ) -> bool {
        let normalized = Self::normalize_device_instance_name(source);
        if index.recorded_branch_names.contains(&normalized) {
            return true;
        }

        Self::populate_flattened_dc_probe_index(netlist, index);

        index
            .flattened_recorded_branch_names
            .as_ref()
            .is_some_and(|names| names.contains(&normalized))
    }

    pub(super) fn netlist_has_independent_current_source(netlist: &Netlist, source: &str) -> bool {
        if Self::elements_have_independent_current_source(&netlist.elements, source) {
            return true;
        }

        rspice_core::netlist::flatten_netlist_with_models(netlist).is_ok_and(|flattened| {
            Self::elements_have_independent_current_source(&flattened.elements, source)
        })
    }

    /// Admit only the exact native Xyce memristor families that circuit
    /// construction can validate and instantiate. `YMEMRISTOR` is shared by
    /// several model levels, so syntax alone must never claim unsupported
    /// families.
    pub(super) fn netlist_element_is_native_xyce_memristor(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        let ElementKind::XyceMemristor {
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return false;
        };
        if !deferred_params.is_empty() {
            return false;
        }
        let Some(model_def) = Self::find_model(&netlist.models, model) else {
            return false;
        };
        rspice_core::engine::build_native_xyce_memristor(
            netlist,
            model_def,
            &element.name,
            model,
            instance_params,
            SimulationConfig::default().temperature,
        )
        .is_ok()
    }

    pub(super) fn single_tran_analysis(netlist: &Netlist) -> Result<XyceTranAnalysis, String> {
        let analyses = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic,
                } => Some(XyceTranAnalysis {
                    step: *step,
                    stop: *stop,
                    start: *start,
                    max_step: *max_step,
                    uic: *uic,
                }),
                _ => None,
            })
            .collect::<Vec<_>>();

        match analyses.len() {
            0 => Err("deck has no .TRAN analysis for static .PRINT TRAN output".to_string()),
            1 => Ok(analyses[0]),
            _ => Err(
                "deck has multiple .TRAN analyses; multi-analysis transient comparison is not implemented yet"
                    .to_string(),
            ),
        }
    }

    pub(super) fn single_ac_analysis(netlist: &Netlist) -> Result<XyceAcAnalysis, String> {
        let mut analyses = Vec::new();
        for analysis in &netlist.analyses {
            match analysis {
                AnalysisCommand::Ac {
                    variation,
                    points,
                    start_freq,
                    stop_freq,
                } => analyses.push(XyceAcAnalysis {
                    frequencies: Self::xyce_ac_sweep_frequencies(
                        *variation,
                        *points,
                        *start_freq,
                        *stop_freq,
                    ),
                    data_points: None,
                }),
                AnalysisCommand::AcData { table_name } => {
                    let data_points =
                        Self::frequency_data_table_points(netlist, table_name, ".AC DATA")?;
                    analyses.push(XyceAcAnalysis {
                        frequencies: data_points
                            .iter()
                            .map(|point| point.frequency)
                            .collect::<Vec<_>>(),
                        data_points: Some(data_points),
                    });
                }
                _ => {}
            }
        }

        match analyses.len() {
            0 => Err("deck has no .AC analysis for static .PRINT AC output".to_string()),
            1 => Ok(analyses[0].clone()),
            _ => Err(
                "deck has multiple .AC analyses; multi-analysis AC comparison is not implemented yet"
                    .to_string(),
            ),
        }
    }

    pub(super) fn single_dc_sweep(netlist: &Netlist) -> Result<XyceDcSweep, String> {
        let mut dimensions = Self::dc_sweep_dimensions(netlist);

        if dimensions.is_empty() {
            if netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Op))
            {
                return Self::synthetic_op_dc_sweep(netlist);
            }
            return Err("deck has no .DC or .OP analysis for static .PRINT DC output".to_string());
        }

        for (index, dimension) in dimensions.iter().enumerate() {
            if dimension.spec().points().is_empty() {
                if index == 0 {
                    return Err("deck has invalid .DC sweep bounds".to_string());
                }
                return Err(format!(
                    "deck has invalid .DC sweep bounds for dimension {}",
                    index + 1
                ));
            }
        }

        let primary = dimensions.remove(0);
        let sweep2 = dimensions
            .first()
            .cloned()
            .map(XyceDcSweepDimension::into_second_sweep);
        Ok(XyceDcSweep {
            source: primary.source,
            start: primary.start,
            stop: primary.stop,
            step: primary.step,
            mode: primary.mode,
            sweep2,
        })
    }

    pub(super) fn single_dc_print_request(source: &str) -> Result<XycePrintRequest, String> {
        let request = Self::single_dc_print_output_request(source)?;
        Ok(XycePrintRequest {
            probes: request.probes,
        })
    }

    pub(super) fn single_dc_print_output_request(
        source: &str,
    ) -> Result<XycePrintOutputRequest, String> {
        let requests = Self::print_output_requests(source, "DC")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .collect::<Vec<_>>();

        match requests.len() {
            0 => Err("deck has no .PRINT DC statement with static columns".to_string()),
            1 => Ok(requests.into_iter().next().expect("one request")),
            _ => Err("deck has multiple .PRINT DC statements; multi-table comparison is not implemented yet".to_string()),
        }
    }

    pub(super) fn single_dc_or_file_output_request(
        source: &str,
    ) -> Result<XycePrintOutputRequest, String> {
        match Self::single_dc_print_output_request(source) {
            Ok(request) => Ok(request),
            Err(primary_error) => {
                let mut side_outputs = Self::prn_compatible_side_output_requests(source)?
                    .into_iter()
                    .collect::<Vec<_>>();
                if side_outputs.is_empty() {
                    return Err(primary_error);
                }
                side_outputs.sort_by(|left, right| left.file.cmp(&right.file));
                Ok(side_outputs.remove(0))
            }
        }
    }

    pub(super) fn single_tran_print_output_request(
        source: &str,
    ) -> Result<XycePrintOutputRequest, String> {
        let requests = Self::print_output_requests(source, "TRAN")?
            .into_iter()
            .filter(|request| request.file.is_none())
            .collect::<Vec<_>>();

        match requests.len() {
            0 => Err("deck has no .PRINT TRAN statement with static columns".to_string()),
            1 => Ok(requests.into_iter().next().expect("one request")),
            _ => Err("deck has multiple .PRINT TRAN statements; multi-table comparison is not implemented yet".to_string()),
        }
    }

    #[cfg(test)]
    pub(super) fn single_ac_print_request(source: &str) -> Result<XycePrintRequest, String> {
        let request = Self::single_ac_print_output_request(source)?;
        Ok(XycePrintRequest {
            probes: request.probes,
        })
    }

    #[cfg(test)]
    pub(super) fn single_ac_print_output_request(
        source: &str,
    ) -> Result<XycePrintOutputRequest, String> {
        match Self::primary_print_output_request(source, "AC")? {
            Some(request) => Ok(request),
            None => Err("deck has no .PRINT AC statement with static columns".to_string()),
        }
    }

    pub(super) fn is_print_option_token(token: &str) -> bool {
        token.starts_with("format=")
            || token.starts_with("width=")
            || token.starts_with("precision=")
            || token.starts_with("delimiter=")
            || token.starts_with("noindex")
            || token.starts_with("index=")
    }

    pub(super) fn strict_nested_include_resistor_fingerprint(
        element: &rspice_core::netlist::Element,
    ) -> Result<XyceRelationalElementFingerprint, String> {
        let ElementKind::Resistor {
            value,
            value_expr,
            model,
            instance_params,
            deferred_params,
        } = &element.kind
        else {
            return Err(format!("element '{}' is not a resistor", element.name));
        };
        if element.nodes.len() != 2
            || !value.is_finite()
            || *value <= 0.0
            || value_expr.is_some()
            || model.is_some()
            || !instance_params.is_empty()
            || !deferred_params.is_empty()
        {
            return Err(format!(
                "resistor '{}' is not one direct finite positive two-terminal resistor",
                element.name
            ));
        }
        Ok(XyceRelationalElementFingerprint {
            kind: "R".to_string(),
            nodes: element
                .nodes
                .iter()
                .map(|node| node.to_ascii_lowercase())
                .collect(),
            numeric_bits: vec![value.to_bits()],
            text: Vec::new(),
        })
    }

    pub(super) fn logical_netlist_lines(source: &str) -> Vec<String> {
        let mut lines: Vec<String> = Vec::new();
        for raw in source.lines() {
            let line = raw
                .split_once(';')
                .map(|(head, _)| head)
                .unwrap_or(raw)
                .trim_end();
            if line.trim().is_empty() {
                continue;
            }
            if Self::strip_netlist_comment(line).trim().is_empty() {
                continue;
            }
            if line.trim_start().starts_with('+') {
                if let Some(previous) = lines.last_mut() {
                    previous.push(' ');
                    previous.push_str(line.trim_start().trim_start_matches('+').trim_start());
                } else {
                    lines.push(line.to_string());
                }
            } else {
                lines.push(line.to_string());
            }
        }
        lines
    }

    pub(super) fn strip_netlist_comment(line: &str) -> &str {
        let no_inline = line.split_once(';').map(|(head, _)| head).unwrap_or(line);
        let trimmed = no_inline.trim_start();
        if trimmed.starts_with('*') || trimmed.starts_with("//") {
            ""
        } else {
            no_inline
        }
    }
}
