//! AC and noise deck contracts.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn validate_static_noise_output_destinations(
        source: &str,
        primary: &XycePrintOutputRequest,
        primary_contract: XyceStaticNoiseContract,
    ) -> Result<(), String> {
        let requests = Self::aggregate_print_output_requests(
            Self::print_output_requests(source, "NOISE")?,
            "NOISE",
        )?;
        let mut destinations = BTreeSet::new();
        for request in requests.iter().filter(|request| request.file.is_some()) {
            let file = request
                .file
                .as_deref()
                .expect("filtered NOISE side output has FILE");
            if !destinations.insert(file.to_ascii_lowercase()) {
                return Err(format!(
                    "native NOISE output contract has duplicate FILE= destination '{file}'"
                ));
            }
            let side_contract = XyceStaticNoiseContract::for_format(request.format.as_deref())?;
            if side_contract.output_family() != primary_contract.output_family() {
                return Err(format!(
                    "native NOISE output contract does not mix primary {:?} output with FILE='{file}' {:?} output",
                    primary_contract, side_contract
                ));
            }
            if request.probes.len() != primary.probes.len()
                || !request
                    .probes
                    .iter()
                    .zip(&primary.probes)
                    .all(|(left, right)| left.eq_ignore_ascii_case(right))
            {
                return Err(format!(
                    "native NOISE FILE='{file}' output must preserve the primary output probe schema"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_native_static_fd_ac_wrapper_contract(
        source: &str,
        output_override: bool,
    ) -> Result<(), String> {
        let mut primary_ac_print_count = 0usize;
        let mut side_ac_print_count = 0usize;
        let mut side_ac_print_formats = Vec::new();
        let mut primary_ac_ic_print_count = 0usize;
        let mut side_ac_ic_print_count = 0usize;
        let mut sensitivity_print_count = 0usize;
        let has_op_analysis = Self::source_has_op_analysis(source);
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
                let Some(analysis) = token_refs.get(1).copied() else {
                    return Err("wrapper-origin .PRINT statement has no analysis type".to_string());
                };
                if !analysis.eq_ignore_ascii_case("AC") {
                    if analysis.eq_ignore_ascii_case("SENS") {
                        sensitivity_print_count += 1;
                        continue;
                    }
                    if analysis.eq_ignore_ascii_case("AC_IC") {
                        if !has_op_analysis {
                            continue;
                        }
                        let mut index = 2usize;
                        let mut has_file_output = false;
                        let mut print_format = "STD".to_string();
                        while index < token_refs.len() {
                            if let Some((raw_key, raw_value, consumed)) =
                                Self::print_option_assignment(&token_refs, index)
                            {
                                let value = raw_value.trim().trim_matches(['"', '\'']);
                                match raw_key.trim().to_ascii_lowercase().as_str() {
                                    "file" => has_file_output = true,
                                    "format" => {
                                        print_format = value.to_string();
                                        if !Self::ac_ic_print_format_is_supported(value) {
                                            return Err(format!(
                                                "wrapper-origin frequency-domain static output contract does not cover .PRINT AC_IC FORMAT={value}"
                                            ));
                                        }
                                    }
                                    _ => {}
                                }
                                index += consumed;
                                continue;
                            }
                            index += 1;
                        }
                        if has_file_output {
                            side_ac_ic_print_count += 1;
                            if !Self::ac_ic_print_format_is_supported(&print_format) {
                                return Err(format!(
                                    "wrapper-origin frequency-domain static output contract does not cover AC_IC FILE= side output FORMAT={print_format}"
                                ));
                            }
                        } else {
                            primary_ac_ic_print_count += 1;
                        }
                        continue;
                    }
                    return Err(format!(
                        "wrapper-origin frequency-domain static output contract does not cover .PRINT {analysis}"
                    ));
                }
                let mut index = 2usize;
                let mut has_file_output = false;
                let mut print_format = "STD".to_string();
                while index < token_refs.len() {
                    if let Some((raw_key, raw_value, consumed)) =
                        Self::print_option_assignment(&token_refs, index)
                    {
                        let value = raw_value.trim().trim_matches(['"', '\'']);
                        match raw_key.trim().to_ascii_lowercase().as_str() {
                            "file" => has_file_output = true,
                            "format" => {
                                print_format = value.to_string();
                                let supported_format =
                                    Self::ac_print_format_is_prn_compatible(value)
                                        || value.eq_ignore_ascii_case("CSV")
                                        || value.eq_ignore_ascii_case("PROBE");
                                if !supported_format {
                                    return Err(format!(
                                        "wrapper-origin frequency-domain static output contract does not cover .PRINT AC FORMAT={value}"
                                    ));
                                }
                            }
                            _ => {}
                        }
                        index += consumed;
                        continue;
                    }
                    index += 1;
                }
                if has_file_output {
                    side_ac_print_count += 1;
                    side_ac_print_formats.push(print_format);
                } else {
                    primary_ac_print_count += 1;
                }
                continue;
            }
            if command.eq_ignore_ascii_case(".sens") {
                continue;
            }
            if command.eq_ignore_ascii_case(".lin") {
                if Self::lin_directive_is_ac_only(&trimmed)? {
                    continue;
                }
                return Err(
                    "wrapper-origin frequency-domain static output contract does not cover .LIN directives other than SPARCALC=0"
                        .to_string(),
                );
            }
            if Self::is_extra_wrapper_ac_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin frequency-domain static output contract does not cover {command} directives"
                ));
            }
        }

        if output_override {
            if primary_ac_print_count == 0
                && side_ac_print_count == 0
                && primary_ac_ic_print_count == 0
                && side_ac_ic_print_count == 0
            {
                return Err(
                    "wrapper-origin frequency-domain output override contract requires one .PRINT AC or .PRINT AC_IC statement"
                        .to_string(),
                );
            }
            return Ok(());
        }

        if primary_ac_print_count == 0 {
            if side_ac_print_count > 1 {
                return Err(
                    "wrapper-origin frequency-domain static output contract does not cover multiple .PRINT AC FILE= outputs without a primary .PRINT AC"
                        .to_string(),
                );
            }
            if let Some(format) = side_ac_print_formats.first() {
                let supported_format = Self::ac_print_format_is_prn_compatible(format)
                    || format.eq_ignore_ascii_case("CSV")
                    || format.eq_ignore_ascii_case("PROBE");
                if !supported_format {
                    return Err(format!(
                        "wrapper-origin frequency-domain static output contract does not cover primary FILE= .PRINT AC FORMAT={format}"
                    ));
                }
            }
            if primary_ac_ic_print_count == 0
                && side_ac_ic_print_count == 0
                && side_ac_print_count == 0
                && sensitivity_print_count == 0
            {
                return Err(
                    "wrapper-origin frequency-domain static output contract requires one primary .PRINT AC or .PRINT AC_IC statement"
                        .to_string(),
                );
            }
        } else {
            for format in side_ac_print_formats {
                if !Self::ac_print_format_is_prn_compatible(&format) {
                    return Err(format!(
                        "wrapper-origin frequency-domain static output contract does not cover FILE= side output FORMAT={format}"
                    ));
                }
            }
        }
        if primary_ac_ic_print_count == 0 && side_ac_ic_print_count > 1 {
            return Err(
                "wrapper-origin frequency-domain static output contract does not cover multiple .PRINT AC_IC FILE= outputs without a primary .PRINT AC_IC"
                    .to_string(),
            );
        }
        Ok(())
    }

    pub(super) fn expected_ac_data_analysis_init_failure(
        source: &str,
        deck_path: &Path,
    ) -> Result<Option<XyceAcDataAnalysisInitFailure>, String> {
        // Only an explicit `.AC ... DATA[=...]` command can enter this
        // contract.  Keep the generic expected-error dispatch cheap for
        // wrapper-only decks that contain large unrelated option cards.
        if !Self::source_may_have_ac_data_analysis_command(source) {
            return Ok(None);
        }

        let netlist = match Self::parse_xyce_netlist(source, deck_path) {
            Ok(netlist) => netlist,
            Err(_) => return Ok(None),
        };

        let mut table_name = None;
        for analysis in &netlist.analyses {
            let AnalysisCommand::AcData { table_name: name } = analysis else {
                return Ok(None);
            };
            if table_name.replace(name.as_str()).is_some() {
                return Ok(None);
            }
        }
        let Some(table_name) = table_name else {
            return Ok(None);
        };
        if !netlist.output_requests.iter().any(|request| {
            request.directive == OutputDirectiveKind::Print
                && request.analysis == Some(rspice_core::netlist::OutputAnalysisKind::Ac)
        }) {
            return Ok(None);
        }

        if netlist
            .data_tables
            .iter()
            .any(|table| table.name.eq_ignore_ascii_case(table_name))
        {
            return Ok(None);
        }

        // Xyce 7.10 emits one of the following ordered analysis-init
        // diagnostics, followed by `Invalid data=<name> parameter on .AC
        // line.`.  The table-state classification is structural and does not
        // depend on a deck path or filename.
        let failure = if netlist.data_tables.is_empty() {
            XyceAcDataAnalysisInitFailure::NoDataTables
        } else {
            XyceAcDataAnalysisInitFailure::UnknownTable
        };
        Ok(Some(failure))
    }

    pub(super) fn static_ac_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticAcContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::ac_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperStatic
            } else {
                XyceStaticAcContract::PlainStatic
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperCsv
            } else {
                XyceStaticAcContract::PlainCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperCsd
            } else {
                XyceStaticAcContract::PlainCsd
            });
        }
        Err(format!(
            "native static .PRINT AC comparison does not cover FORMAT={normalized}"
        ))
    }

    pub(super) fn static_ac_ic_contract_for_print_format(
        requires_wrapper: bool,
        format: Option<&str>,
    ) -> Result<XyceStaticAcContract, String> {
        let normalized = format.unwrap_or("STD").trim();
        if Self::ac_print_format_is_prn_compatible(normalized) {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperIcPrn
            } else {
                XyceStaticAcContract::PlainIcPrn
            });
        }
        if normalized.eq_ignore_ascii_case("CSV") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperIcCsv
            } else {
                XyceStaticAcContract::PlainIcCsv
            });
        }
        if normalized.eq_ignore_ascii_case("PROBE") {
            return Ok(if requires_wrapper {
                XyceStaticAcContract::WrapperIcCsd
            } else {
                XyceStaticAcContract::PlainIcCsd
            });
        }
        Err(format!(
            "native static .PRINT AC_IC comparison does not cover FORMAT={normalized}"
        ))
    }

    pub(super) fn validate_ac_analysis_expression_plan(
        plan: &XyceRelationalAcPlan,
    ) -> Result<(), String> {
        const LABEL: &str = "AC-analysis expression parity";
        if plan.print.probes.is_empty()
            || plan.ac.frequencies.is_empty()
            || plan
                .ac
                .frequencies
                .iter()
                .any(|frequency| !frequency.is_finite() || *frequency < 0.0)
            || plan.ac.data_points().is_some()
        {
            return Err(format!(
                "{LABEL} requires a finite nonempty ordinary AC sweep and at least one probe"
            ));
        }
        let netlist = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|err| format!("{LABEL} parse failed during plan validation: {err}"))?;
        Self::validate_ac_analysis_expression_probes(&netlist, &plan.print)?;
        Self::ac_analysis_source_qualification(&plan.source).map(|_| ())
    }

    pub(super) fn validate_ac_analysis_expression_probes(
        netlist: &Netlist,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        const LABEL: &str = "AC-analysis expression parity";
        let mut probes = BTreeSet::new();
        for probe in &print.probes {
            if Self::print_expression_inner(probe).is_some() {
                return Err(format!(
                    "{LABEL} admits only direct atomic AC voltage/current probes, not braced expressions"
                ));
            }
            let normalized = Self::normalize_probe(probe);
            let atomic = Self::parse_ac_voltage_probe(&normalized)
                .is_some_and(|_| Self::probe_call_covers_entire_expression(&normalized))
                || Self::parse_ac_current_probe(&normalized)
                    .is_some_and(|_| Self::probe_call_covers_entire_expression(&normalized));
            if !atomic || !probes.insert(normalized.clone()) {
                return Err(format!(
                    "{LABEL} probe '{probe}' is not a unique direct atomic AC voltage/current probe"
                ));
            }
            Self::validate_ac_probe(&normalized, netlist)?;
        }
        Ok(())
    }

    pub(super) fn validate_static_ac_contract(
        netlist: &Netlist,
        ac: &XyceAcAnalysis,
        print: &XycePrintRequest,
    ) -> Result<(), String> {
        let frequencies = ac.frequencies();
        if frequencies.is_empty() {
            return Err(".AC analysis produced no frequency points".to_string());
        }

        for measurement in &netlist.measurements {
            if !measurement.analysis.eq_ignore_ascii_case("AC")
                && !measurement.analysis.eq_ignore_ascii_case("AC_CONT")
            {
                return Err(format!(
                    "native static AC comparison does not cover .MEASURE {} '{}'",
                    measurement.analysis, measurement.name
                ));
            }
        }

        for probe in &print.probes {
            Self::validate_ac_probe(probe, netlist)?;
        }
        Self::validate_native_static_ac_contract(netlist, ac)?;

        Ok(())
    }

    pub(super) fn validate_native_static_ac_contract(
        netlist: &Netlist,
        ac: &XyceAcAnalysis,
    ) -> Result<(), String> {
        let flattened_hierarchy = netlist
            .elements
            .iter()
            .any(|element| matches!(element.kind, ElementKind::Subcircuit { .. }));
        let mut flattened_netlist;
        let netlist = if flattened_hierarchy {
            let flattened =
                rspice_core::netlist::flatten_netlist_with_models(netlist).map_err(|error| {
                    format!(
                        "native static .PRINT AC comparison could not flatten subcircuits: {error}"
                    )
                })?;
            flattened_netlist = netlist.clone();
            flattened_netlist.elements = flattened.elements;
            flattened_netlist.models.extend(flattened.scoped_models);
            flattened_netlist.subcircuits.clear();
            &flattened_netlist
        } else {
            netlist
        };

        let max_frequency = ac
            .frequencies
            .iter()
            .copied()
            .filter(|frequency| frequency.is_finite())
            .fold(0.0_f64, f64::max);
        let lin_ac_only = netlist.lin_analysis == Some(rspice_core::netlist::LinAnalysis::AcOnly);
        for element in &netlist.elements {
            match &element.kind {
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::RfPort { .. })
                | ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::RfPort { .. })
                    if lin_ac_only => {}
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::RfPort { .. })
                | ElementKind::CurrentSource(rspice_core::netlist::SourceSpec::RfPort { .. }) => {
                    return Err(format!(
                        "native static .PRINT AC comparison does not cover RF port source '{}' without executing its .LIN/S-parameter contract",
                        element.name
                    ));
                }
                ElementKind::VoltageSource(_)
                | ElementKind::CurrentSource(_)
                | ElementKind::Resistor { .. }
                | ElementKind::Inductor { .. } => {}
                ElementKind::Capacitor {
                    value, value_expr, ..
                } if value_expr.is_some() && !value.is_finite() => {
                    return Err(format!(
                        "native static .PRINT AC comparison does not support solution-dependent capacitor value expression on element '{}'",
                        element.name
                    ));
                }
                ElementKind::Capacitor { .. } => {}
                ElementKind::Coupling { coefficient, .. } => {
                    if !coefficient.is_finite() {
                        return Err(format!(
                            "native static .PRINT AC comparison does not support coupling '{}' with non-finite coefficient {}",
                            element.name, coefficient
                        ));
                    }
                }
                ElementKind::Vccs {
                    transconductance, ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCCS",
                        &element.name,
                        "transconductance",
                        *transconductance,
                    )?;
                }
                ElementKind::Vcvs { gain, .. } => {
                    Self::validate_finite_controlled_source_gain(
                        "VCVS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCCS",
                        &element.name,
                        "gain",
                        *gain,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        &netlist.elements,
                        "CCCS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                    ..
                } => {
                    Self::validate_finite_controlled_source_gain(
                        "CCVS",
                        &element.name,
                        "transresistance",
                        *transresistance,
                    )?;
                    Self::validate_current_controlled_source_probe(
                        &netlist.elements,
                        "CCVS",
                        &element.name,
                        control_element,
                    )?;
                }
                ElementKind::BehavioralVoltage {
                    expression,
                    tc1,
                    tc2,
                }
                | ElementKind::BehavioralCurrent {
                    expression,
                    tc1,
                    tc2,
                } if tc1.is_finite() && tc2.is_finite() => {
                    Self::validate_ac_behavioral_expression(
                        &element.name,
                        expression,
                        &netlist.params,
                    )?;
                }
                ElementKind::Mosfet { .. }
                    if Self::netlist_device_is_single_native_ac_supported_bulk_mosfet(
                        netlist,
                        &element.name,
                    ) || Self::netlist_device_is_single_native_ac_supported_bsim3(
                        netlist,
                        &element.name,
                    ) || Self::netlist_device_is_single_native_ac_supported_b3soi(
                        netlist,
                        &element.name,
                    ) || Self::netlist_device_is_single_native_ac_supported_bsim4(
                        netlist,
                        &element.name,
                    ) => {}
                ElementKind::Bjt { .. }
                    if max_frequency <= 100.0
                        && Self::netlist_device_is_native_legacy_bjt(netlist, &element.name) => {}
                ElementKind::Bjt { .. }
                    if max_frequency <= 10.0e9 + 1.0e-3
                        && !flattened_hierarchy
                        && Self::netlist_device_is_native_static_ac_legacy_npn_bjt(
                            netlist,
                            &element.name,
                        ) => {}
                ElementKind::Bjt { .. }
                    if Self::netlist_element_is_native_static_ac_exact_bf_is_pnp(
                        netlist, element,
                    ) => {}
                ElementKind::Bjt { .. }
                    if max_frequency <= 20_000.0 + 1.0e-9
                        && Self::netlist_element_is_native_static_ac_exact_bf_is_npn(
                            netlist, element,
                        ) => {}
                ElementKind::Diode { .. }
                    if Self::netlist_element_is_native_exact_is_diode(netlist, element) => {}
                ElementKind::Diode { .. }
                    if Self::netlist_element_is_native_generated_cmc_diode(netlist, element) => {}
                ElementKind::Diode { .. }
                    if Self::netlist_element_is_native_generated_juncap_diode(netlist, element) => {
                }
                _ => {
                    return Err(format!(
                        "native static .PRINT AC comparison currently supports flattened hierarchy containing non-RF independent sources, static R/L/C passives, mutual inductors, finite-gain linear controlled sources, time-independent behavioral sources, exact IS-only diodes, exact IS/BF PNPs at all frequencies, exact IS/BF NPNs through 20 kHz, strictly qualified single-device classic MOSFET LEVEL=1/2/3/6, validated single-device BSIM3 LEVEL=9 cards, validated single-device BSIMSOI LEVEL=10 cards with explicit SOIMOD=1, validated single-device BSIM4 LEVEL=14/54 cards, and validated native legacy level-1 NPN Gummel-Poon sweeps through 10 GHz; element '{}' requires a broader AC oracle contract",
                        element.name
                    ));
                }
            }
        }

        Ok(())
    }

    pub(super) fn validate_ac_behavioral_expression(
        element_name: &str,
        expression: &str,
        params: &rspice_core::netlist::ParamContext,
    ) -> Result<(), String> {
        let prepared = prepare_behavioral_expression(expression, params).map_err(|err| {
            format!(
                "native static .PRINT AC comparison could not prepare behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        let ast = parse_expression_strict(&prepared).map_err(|err| {
            format!(
                "native static .PRINT AC comparison does not yet support behavioral expression '{}' on element '{}': {err}",
                expression, element_name
            )
        })?;
        if Self::expression_depends_on_ac_runtime_quantity(&ast) {
            return Err(format!(
                "native static .PRINT AC comparison does not support behavioral expression '{}' on element '{}' because AC behavioral small-signal linearization has no transient time or frequency-domain expression variable",
                expression, element_name
            ));
        }
        Ok(())
    }

    pub(super) fn validate_ac_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        if Self::probe_names_live_measurement(probe, netlist, "AC", "AC_CONT") {
            return Ok(());
        }
        if let Some(expression) = Self::print_expression_inner(probe) {
            let normalized_expression = Self::normalize_probe(expression);
            if Self::braced_expression_is_atomic_ac_probe(&normalized_expression, netlist) {
                return Self::validate_atomic_ac_probe(&normalized_expression, expression, netlist);
            }
            return Self::validate_ac_expression_probe(expression, netlist);
        }

        let normalized = Self::normalize_probe(probe);
        Self::validate_atomic_ac_probe(&normalized, probe, netlist)
            .or_else(|_| Self::validate_ac_expression_probe(probe, netlist))
    }

    pub(super) fn validate_atomic_ac_probe(
        normalized: &str,
        original: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(normalized) {
            if !voltage_probe.node_pos.is_empty()
                && voltage_probe
                    .node_neg
                    .as_deref()
                    .is_none_or(|node| !node.is_empty())
            {
                return Ok(());
            }
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(normalized)
            && Self::netlist_has_recorded_branch_current(netlist, &current_probe.element_name)
        {
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
                "acmag" | "acphase"
                    if Self::source_is_independent_source(netlist, &element_name) =>
                {
                    return Ok(());
                }
                "r" if Self::find_resistor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "c" if Self::find_capacitor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                "l" if Self::find_inductor_element(netlist, &element_name).is_some() => {
                    return Ok(());
                }
                _ => {}
            }
        }
        if netlist.measurements.iter().any(|measurement| {
            measurement.analysis.eq_ignore_ascii_case("AC")
                && measurement.name.eq_ignore_ascii_case(original)
        }) {
            return Ok(());
        }
        Err(format!("unsupported .PRINT AC probe '{}'", original))
    }

    pub(super) fn validate_ac_complex_probe(probe: &str, netlist: &Netlist) -> Result<(), String> {
        let normalized = Self::normalize_probe(probe);
        if let Some(voltage_probe) = Self::parse_ac_voltage_probe(&normalized) {
            if voltage_probe.accessor != XyceVoltageAccessor::Value {
                return Err(format!(
                    ".PRINT AC complex expression expects a complex V(...) argument, got '{}'",
                    probe.trim()
                ));
            }
            return Self::validate_atomic_ac_probe(&normalized, probe, netlist);
        }
        if let Some(current_probe) = Self::parse_ac_current_probe(&normalized) {
            if current_probe.accessor != XyceCurrentAccessor::Value {
                return Err(format!(
                    ".PRINT AC complex expression expects a complex I(...) argument, got '{}'",
                    probe.trim()
                ));
            }
            return Self::validate_atomic_ac_probe(&normalized, probe, netlist);
        }
        Self::validate_ac_expression_probe(probe, netlist)
    }

    pub(super) fn validate_ac_expression_probe(
        expression: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let context = Self::print_eval_context(netlist, None, None);
        let mut call_value = |call: &str| {
            Self::validate_ac_expression_call_probe(call, netlist)?;
            Ok(ExprComplexValue::real(1.0))
        };
        let (rewritten, context) =
            Self::rewrite_ac_print_expression_complex(expression, context, &mut call_value)?;
        rspice_core::netlist::expr::eval_expression_complex(&rewritten, &context)
            .map_err(|err| format!("unsupported .PRINT AC expression '{{{expression}}}': {err}"))?;
        Ok(())
    }

    pub(super) fn validate_ac_expression_call_probe(
        call: &str,
        netlist: &Netlist,
    ) -> Result<(), String> {
        let normalized = Self::normalize_probe(call);
        if Self::parse_ac_voltage_probe(&normalized).is_some()
            || Self::parse_ac_current_probe(&normalized).is_some()
        {
            return Self::validate_atomic_ac_probe(&normalized, call, netlist);
        }
        Self::validate_ac_complex_probe(call, netlist)
    }

    pub(super) fn ac_analysis_expression_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        // AC expression qualification requires exactly one native baseline
        // and one wrapper representation.  The cardinality check keeps
        // unrelated wrapper-only directories out of the parser path.
        if Self::circuit_file_count(parent)? != 2 {
            return None;
        }
        let mut records = Vec::new();
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
                    .static_output_reference_path(&path, "FD.prn")
                    .is_some_and(|reference| reference.is_file())
            {
                return None;
            }
            let source = fs::read_to_string(&path).ok()?;
            let (representation, _) = Self::ac_analysis_source_qualification(&source).ok()?;
            let wrapper = self.requires_upstream_wrapper(&self.relative_key(&path));
            if wrapper != (representation == XyceAcAnalysisRepresentation::ParameterExpression) {
                return None;
            }
            let plan = self.relational_ac_plan_for_path(&path).ok()?;
            Self::validate_ac_analysis_expression_plan(&plan).ok()?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &path).ok()?;
            let snapshot = Self::ac_analysis_expression_snapshot(&netlist).ok()?;
            records.push((path, wrapper, plan, netlist, snapshot));
        }
        if records.len() != 2
            || !records
                .iter()
                .any(|(path, _, _, _, _)| Self::same_path(path, &deck.path))
        {
            return None;
        }
        let baseline_index = records.iter().position(|(_, wrapper, _, _, _)| !wrapper)?;
        let target_index = records.iter().position(|(_, wrapper, _, _, _)| *wrapper)?;
        if baseline_index == target_index
            || records
                .iter()
                .filter(|(_, wrapper, _, _, _)| !wrapper)
                .count()
                != 1
            || records
                .iter()
                .filter(|(_, wrapper, _, _, _)| *wrapper)
                .count()
                != 1
        {
            return None;
        }
        let (baseline_path, _, baseline_plan, baseline_netlist, baseline_snapshot) =
            &records[baseline_index];
        let (target_path, _, target_plan, target_netlist, target_snapshot) = &records[target_index];
        if baseline_plan.print.probes != target_plan.print.probes
            || baseline_plan.ac.frequencies.len() != target_plan.ac.frequencies.len()
            || !baseline_plan
                .ac
                .frequencies
                .iter()
                .zip(&target_plan.ac.frequencies)
                .all(|(baseline, target)| {
                    baseline.is_finite()
                        && target.is_finite()
                        && baseline.to_bits() == target.to_bits()
                })
            || !Self::ac_analyses_match_exactly(baseline_netlist, target_netlist)
            || Self::compare_ac_analysis_expression_snapshots(baseline_snapshot, target_snapshot)
                .is_err()
        {
            return None;
        }
        let tolerance = XyceAcComparatorTolerance::new(6.0e-5, 1.0e-4, 1.0e-6, 1.0e-6).ok()?;
        let family = format!(
            "{}:{}",
            parent.file_name()?.to_str()?,
            baseline_path.file_stem()?.to_str()?
        );
        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::AcAnalysisExpression,
            comparison: XyceBaselineFamilyComparison::AcComparator(tolerance),
            family,
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path.clone(), target_path.clone()],
            target_path: Some(deck.path.clone()),
        })
    }
}
