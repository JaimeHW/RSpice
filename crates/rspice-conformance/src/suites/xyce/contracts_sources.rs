//! Source- and parameter-driven deck contracts.
//!
//! Part of the Xyce suite's `XyceTestRunner`, split out of a single
//! 96,731-line file. Methods keep `impl XyceTestRunner` so call sites are
//! unchanged; private ones are `pub(super)` so siblings can reach them.

use super::*;

impl XyceTestRunner {
    pub(super) fn validate_startup_source_identity(
        kind: XyceStartupOracleKind,
        source_bytes: &[u8],
    ) -> Result<(), String> {
        let (expected_bytes, expected_hash) = kind.source_identity();
        let actual_hash = blake3::hash(source_bytes).to_hex().to_string();
        if source_bytes.len() != expected_bytes || actual_hash != expected_hash {
            return Err(format!(
                "startup-diagnostic record '{}' source identity changed: expected {expected_bytes} bytes / {expected_hash}, got {} bytes / {actual_hash}",
                kind.record(),
                source_bytes.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_startup_source_family_census(
        family_dir: &Path,
        expected_physical_count: usize,
        expected_physical_hash: &str,
        expected_complete_count: usize,
        expected_complete_hash: &str,
    ) -> Result<(), String> {
        let mut physical = BTreeSet::new();
        let mut complete = BTreeSet::new();
        for entry in fs::read_dir(family_dir).map_err(|error| {
            format!(
                "failed to inspect startup-diagnostic source family {}: {error}",
                family_dir.display()
            )
        })? {
            let entry = entry.map_err(|error| {
                format!(
                    "failed to read startup-diagnostic family entry in {}: {error}",
                    family_dir.display()
                )
            })?;
            let metadata = fs::symlink_metadata(entry.path()).map_err(|error| {
                format!(
                    "failed to inspect startup-diagnostic family member {}: {error}",
                    entry.path().display()
                )
            })?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "startup-diagnostic family member {} must be a regular non-symlink file",
                    entry.path().display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| {
                    format!(
                        "startup-diagnostic family filename in {} is not UTF-8",
                        family_dir.display()
                    )
                })?
                .to_ascii_lowercase();
            if !complete.insert(name.clone()) {
                return Err(format!(
                    "startup-diagnostic family contains case-colliding name {name:?}"
                ));
            }
            if name.ends_with(".cir") {
                physical.insert(name);
            }
        }
        let physical = physical.into_iter().collect::<Vec<_>>();
        let complete = complete.into_iter().collect::<Vec<_>>();
        let physical_hash = blake3::hash(physical.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let complete_hash = blake3::hash(complete.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if physical.len() != expected_physical_count || physical_hash != expected_physical_hash {
            return Err(format!(
                "startup-diagnostic physical .cir census changed: expected {expected_physical_count} / {expected_physical_hash}, got {} / {physical_hash}",
                physical.len()
            ));
        }
        if complete.len() != expected_complete_count || complete_hash != expected_complete_hash {
            return Err(format!(
                "startup-diagnostic complete source-directory census changed: expected {expected_complete_count} / {expected_complete_hash}, got {} / {complete_hash}",
                complete.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug702_resource(
        family_dir: &Path,
        file_name: &str,
        expected_bytes: usize,
        expected_blake3: &str,
    ) -> Result<PathBuf, String> {
        let path = family_dir.join(file_name);
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            format!(
                "failed to inspect BUG702 resource {}: {error}",
                path.display()
            )
        })?;
        if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
            return Err(format!(
                "BUG702 resource {} must be a regular non-symlink file",
                path.display()
            ));
        }
        let bytes = fs::read(&path).map_err(|error| {
            format!("failed to read BUG702 resource {}: {error}", path.display())
        })?;
        let digest = blake3::hash(&bytes).to_hex().to_string();
        if bytes.len() != expected_bytes || digest != expected_blake3 {
            return Err(format!(
                "BUG702 resource {file_name} changed: expected {expected_bytes} bytes / {expected_blake3}, got {} bytes / {digest}",
                bytes.len()
            ));
        }
        path.canonicalize().map_err(|error| {
            format!(
                "failed to canonicalize BUG702 resource {}: {error}",
                path.display()
            )
        })
    }

    pub(super) fn validate_hspice_math_wrapper_source(source: &str) -> Result<(), String> {
        Self::validate_default_prn_wrapper_source(source)?;
        let checks = [
            ("**", "HSPICE exponentiation operator '**'"),
            ("^", "HSPICE exponentiation operator '^'"),
            ("&&", "HSPICE logical AND operator '&&'"),
            ("||", "HSPICE logical OR operator '||'"),
            ("?", "ternary conditional operator '?'"),
            (":", "ternary conditional separator ':'"),
        ];
        for (needle, label) in checks {
            if !source.contains(needle) {
                return Err(format!(
                    "wrapper-origin HSPICE math contract requires {label}"
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_default_prn_wrapper_source(source: &str) -> Result<(), String> {
        Self::validate_default_prn_wrapper_source_with_format_mode(source, false)
    }

    pub(super) fn validate_default_prn_wrapper_source_with_format_mode(
        source: &str,
        allow_wrapper_probe_primary_prn: bool,
    ) -> Result<(), String> {
        let mut primary_print_count = 0usize;
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
                if !Self::validate_default_prn_print_tokens(
                    &token_refs,
                    allow_wrapper_probe_primary_prn,
                )? {
                    primary_print_count += 1;
                }
                continue;
            }
            if command.eq_ignore_ascii_case(".measure") || command.eq_ignore_ascii_case(".meas") {
                let fields = Self::split_print_fields(&trimmed)?;
                if fields
                    .get(1)
                    .is_some_and(|field| field.eq_ignore_ascii_case("DC"))
                    && fields.get(3).is_some_and(|field| {
                        field.eq_ignore_ascii_case("EQN")
                            || field.eq_ignore_ascii_case("PARAM")
                            || field.to_ascii_uppercase().starts_with("PARAM=")
                    })
                {
                    continue;
                }
                return Err(format!(
                    "wrapper-origin default .prn contract does not cover {command} directive '{trimmed}'"
                ));
            }
            if Self::is_extra_wrapper_output_analysis_command(command) {
                return Err(format!(
                    "wrapper-origin default .prn contract does not cover {command} directives"
                ));
            }
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin default .prn contract requires one primary .PRINT statement"
                    .to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin default .prn contract requires one primary .PRINT statement, found {primary_print_count}"
            )),
        }
    }

    pub(super) fn validate_csv_wrapper_source(source: &str) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for request in Self::dc_print_output_requests(source)? {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_some() {
                return Err(format!(
                    "wrapper-origin CSV contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
            if !format.eq_ignore_ascii_case("CSV") {
                return Err(format!(
                    "wrapper-origin CSV contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            primary_print_count += 1;
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin CSV contract requires one primary .PRINT DC statement".to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin CSV contract requires one primary .PRINT DC statement, found {primary_print_count}"
            )),
        }
    }

    pub(super) fn validate_csd_wrapper_source(source: &str) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for request in Self::dc_print_output_requests(source)? {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_some() {
                return Err(format!(
                    "wrapper-origin CSDF contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
            if !format.eq_ignore_ascii_case("PROBE") {
                return Err(format!(
                    "wrapper-origin CSDF contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            primary_print_count += 1;
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin CSDF contract requires one primary .PRINT DC statement".to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin CSDF contract requires one primary .PRINT DC statement, found {primary_print_count}"
            )),
        }
    }

    pub(super) fn validate_file_only_prn_wrapper_source(source: &str) -> Result<(), String> {
        let requests = Self::dc_print_output_requests(source)?;
        if requests.is_empty() {
            return Err(
                "wrapper-origin file-output contract requires at least one .PRINT DC statement"
                    .to_string(),
            );
        }

        for request in requests {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_none() {
                return Err(format!(
                    "wrapper-origin file-output contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            if !Self::dc_print_format_is_prn_compatible(format) {
                return Err(format!(
                    "wrapper-origin file-output contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
        }

        Ok(())
    }

    pub(super) fn validate_raw_wrapper_source(source: &str) -> Result<(), String> {
        let mut primary_print_count = 0usize;
        for request in Self::dc_print_output_requests(source)? {
            let format = request.format.as_deref().unwrap_or("STD");
            if request.file.is_some() {
                return Err(format!(
                    "wrapper-origin RAW contract does not cover FILE= side output with FORMAT={format}"
                ));
            }
            let supported_format =
                format.eq_ignore_ascii_case("RAW") || format.eq_ignore_ascii_case("STD");
            if !supported_format {
                return Err(format!(
                    "wrapper-origin RAW contract does not cover primary .PRINT DC FORMAT={format}"
                ));
            }
            primary_print_count += 1;
        }

        match primary_print_count {
            1 => Ok(()),
            0 => Err(
                "wrapper-origin RAW contract requires one primary .PRINT DC statement".to_string(),
            ),
            _ => Err(format!(
                "wrapper-origin RAW contract requires one primary .PRINT DC statement, found {primary_print_count}"
            )),
        }
    }

    pub(super) fn validate_expected_missing_inductor_value_error_source(
        source: &str,
        deck_path: &Path,
    ) -> Result<(), String> {
        match Self::parse_xyce_netlist(source, deck_path) {
            Ok(_) => Err(
                "expected missing-inductor-value deck parsed successfully; expected a fatal parser diagnostic"
                    .to_string(),
            ),
            Err(err) => {
                let message = err.to_string();
                if message.contains("Inductor requires either a value or a model") {
                    Ok(())
                } else {
                    Err(format!(
                        "expected missing-inductor-value diagnostic, got parser error: {message}"
                    ))
                }
            }
        }
    }

    pub(super) fn validate_expected_pwl_repeat_value_error_source(
        source: &str,
        deck_path: &Path,
    ) -> Result<(), String> {
        if !Self::source_may_have_pwl_repeat_option(source) {
            return Err(
                "expected PWL repeat-value error deck has no PWL repeat option".to_string(),
            );
        }

        match Self::parse_xyce_netlist(source, deck_path) {
            Ok(_) => Err(format!(
                "expected {XYCE_PWL_REPEAT_VALUE_ERROR}, but deck parsed successfully"
            )),
            Err(err) => {
                let message = err.to_string();
                if message.contains(XYCE_PWL_REPEAT_VALUE_ERROR) {
                    Ok(())
                } else {
                    Err(format!(
                        "expected {XYCE_PWL_REPEAT_VALUE_ERROR}, got {message}"
                    ))
                }
            }
        }
    }

    pub(super) fn analytic_rc_source_contract(
        source: &str,
    ) -> Result<XyceAnalyticRcSourceContract, String> {
        const LABEL: &str = "analytic first-order RC";
        if source.lines().any(|line| {
            line.split_whitespace()
                .next()
                .is_some_and(|field| field.eq_ignore_ascii_case("*COMP"))
        }) {
            return Err(format!(
                "{LABEL} uses the unmodified Release 7.10 xyce_verify defaults and does not admit *COMP directives"
            ));
        }

        let lines = Self::logical_netlist_lines(source);
        let Some(title) = lines.first() else {
            return Err(format!("{LABEL} requires a circuit title"));
        };
        let title = Self::strip_netlist_comment(title).trim();
        if title.is_empty() || title.starts_with('.') {
            return Err(format!("{LABEL} requires an ordinary circuit title"));
        }

        let mut capacitor = None;
        let mut resistor = None;
        let mut voltage_source = None;
        let mut print_probe = None;
        let mut tran_values = None;
        let mut option_values = None;
        let mut end_count = 0usize;
        let mut element_names = BTreeSet::new();

        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "analytic first-order RC source statement",
            )?;
            let Some(command) = fields.first() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".print" => {
                        if print_probe.is_some()
                            || fields.len() != 3
                            || !fields[1].eq_ignore_ascii_case("TRAN")
                        {
                            return Err(format!(
                                "{LABEL} requires one canonical '.PRINT TRAN V(node)' statement"
                            ));
                        }
                        let probe = Self::parse_voltage_probe(&fields[2]).ok_or_else(|| {
                            format!("{LABEL} print token '{}' is not a voltage probe", fields[2])
                        })?;
                        if probe.accessor != XyceVoltageAccessor::Value
                            || probe.node_neg.is_some()
                            || !Self::is_single_spice_node_token(&probe.node_pos)
                        {
                            return Err(format!(
                                "{LABEL} requires one ordinary single-ended voltage probe"
                            ));
                        }
                        print_probe =
                            Some(Self::canonical_passive_primary_node_name(&probe.node_pos));
                    }
                    ".tran" => {
                        if tran_values.is_some() || fields.len() != 3 {
                            return Err(format!(
                                "{LABEL} requires one canonical '.TRAN step stop' statement"
                            ));
                        }
                        let step = Self::single_spice_numeric_literal_value(&fields[1])?;
                        let stop = Self::single_spice_numeric_literal_value(&fields[2])?;
                        if step < 0.0 || stop <= 0.0 {
                            return Err(format!(
                                "{LABEL} requires a finite nonnegative print step and positive stop time"
                            ));
                        }
                        tran_values = Some((step.to_bits(), stop.to_bits()));
                    }
                    ".options" => {
                        if option_values.is_some() || !matches!(fields.len(), 4 | 5) {
                            return Err(format!(
                                "{LABEL} requires exactly '.OPTIONS TIMEINT RELTOL=<numeric> ABSTOL=<numeric> [NEWLTE=<0..3>]'"
                            ));
                        }
                        let mut timeint = false;
                        let mut reltol = None;
                        let mut abstol = None;
                        let mut transient_lte_reference = None;
                        for option in &fields[1..] {
                            if option.eq_ignore_ascii_case("TIMEINT") {
                                if timeint {
                                    return Err(format!(
                                        "{LABEL} contains duplicate TIMEINT options"
                                    ));
                                }
                                timeint = true;
                                continue;
                            }
                            let Some((name, value)) = option.split_once('=') else {
                                return Err(format!(
                                    "{LABEL} option '{option}' is not a direct assignment"
                                ));
                            };
                            if value.contains('=') {
                                return Err(format!(
                                    "{LABEL} option '{option}' contains multiple assignments"
                                ));
                            }
                            let value = Self::single_spice_numeric_literal_value(value)?;
                            if name.eq_ignore_ascii_case("RELTOL") {
                                if value <= 0.0 {
                                    return Err(format!(
                                        "{LABEL} option '{name}' must be finite and positive"
                                    ));
                                }
                                if reltol.replace(value.to_bits()).is_some() {
                                    return Err(format!(
                                        "{LABEL} contains duplicate RELTOL options"
                                    ));
                                }
                            } else if name.eq_ignore_ascii_case("ABSTOL") {
                                if value <= 0.0 {
                                    return Err(format!(
                                        "{LABEL} option '{name}' must be finite and positive"
                                    ));
                                }
                                if abstol.replace(value.to_bits()).is_some() {
                                    return Err(format!(
                                        "{LABEL} contains duplicate ABSTOL options"
                                    ));
                                }
                            } else if name.eq_ignore_ascii_case("NEWLTE") {
                                let rounded = value.round();
                                if value != rounded || !(0.0..=3.0).contains(&rounded) {
                                    return Err(format!(
                                        "{LABEL} NEWLTE must be an integer from 0 to 3, found {value}"
                                    ));
                                }
                                let reference =
                                    TransientLteReference::from_xyce_selector(rounded as u8)
                                        .expect("range was checked above");
                                if transient_lte_reference.replace(reference).is_some() {
                                    return Err(format!(
                                        "{LABEL} contains duplicate NEWLTE options"
                                    ));
                                }
                            } else {
                                return Err(format!(
                                    "{LABEL} does not admit option assignment '{name}'"
                                ));
                            }
                        }
                        if !timeint || reltol.is_none() || abstol.is_none() {
                            return Err(format!(
                                "{LABEL} requires TIMEINT plus one RELTOL and one ABSTOL assignment"
                            ));
                        }
                        option_values = Some((
                            reltol.expect("checked above"),
                            abstol.expect("checked above"),
                            transient_lte_reference,
                        ));
                    }
                    ".end" if fields.len() == 1 => end_count += 1,
                    other => {
                        return Err(format!("{LABEL} does not admit directive '{other}'"));
                    }
                }
                continue;
            }

            if fields.len() < 4
                || !Self::is_single_spice_identifier(&fields[0])
                || !Self::is_single_spice_node_token(&fields[1])
                || !Self::is_single_spice_node_token(&fields[2])
            {
                return Err(format!(
                    "{LABEL} element '{command}' does not use direct name/node fields"
                ));
            }
            let element_name = Self::normalize_device_instance_name(&fields[0]);
            if !element_names.insert(element_name.clone()) {
                return Err(format!(
                    "{LABEL} contains duplicate element name '{element_name}'"
                ));
            }
            let nodes = [
                Self::canonical_passive_primary_node_name(&fields[1]),
                Self::canonical_passive_primary_node_name(&fields[2]),
            ];
            match fields[0].chars().next().map(|ch| ch.to_ascii_uppercase()) {
                Some('C') => {
                    if capacitor.is_some() || fields.len() != 5 {
                        return Err(format!(
                            "{LABEL} requires one 'Cname n+ n- value IC=<numeric>' element"
                        ));
                    }
                    let capacitance = Self::single_spice_numeric_literal_value(&fields[3])?;
                    let Some((name, value)) = fields[4].split_once('=') else {
                        return Err(format!("{LABEL} capacitor must contain IC=<numeric>"));
                    };
                    if value.contains('=') || !name.eq_ignore_ascii_case("IC") {
                        return Err(format!("{LABEL} capacitor must contain only IC=<numeric>"));
                    }
                    let initial_voltage = Self::single_spice_numeric_literal_value(value)?;
                    if capacitance <= 0.0 {
                        return Err(format!("{LABEL} capacitance must be positive"));
                    }
                    capacitor = Some((
                        element_name,
                        nodes,
                        capacitance.to_bits(),
                        initial_voltage.to_bits(),
                    ));
                }
                Some('R') => {
                    if resistor.is_some() || fields.len() != 4 {
                        return Err(format!(
                            "{LABEL} requires one direct 'Rname n+ n- value' element"
                        ));
                    }
                    let resistance = Self::single_spice_numeric_literal_value(&fields[3])?;
                    if resistance <= 0.0 {
                        return Err(format!("{LABEL} resistance must be positive"));
                    }
                    resistor = Some((element_name, nodes, resistance.to_bits()));
                }
                Some('V') => {
                    if voltage_source.is_some() || fields.len() != 4 {
                        return Err(format!("{LABEL} requires one direct DC voltage source"));
                    }
                    let value = Self::single_spice_numeric_literal_value(&fields[3])?;
                    voltage_source = Some((element_name, nodes, value.to_bits()));
                }
                _ => {
                    return Err(format!("{LABEL} does not admit element '{command}'"));
                }
            }
        }

        if end_count != 1 {
            return Err(format!(
                "{LABEL} requires exactly one .END statement, found {end_count}"
            ));
        }
        let (capacitor_name, capacitor_nodes, capacitance_bits, initial_voltage_bits) =
            capacitor.ok_or_else(|| format!("{LABEL} contains no qualified capacitor"))?;
        let (resistor_name, resistor_nodes, resistance_bits) =
            resistor.ok_or_else(|| format!("{LABEL} contains no qualified resistor"))?;
        let (source_name, source_nodes, source_value_bits) = voltage_source
            .ok_or_else(|| format!("{LABEL} contains no qualified voltage source"))?;
        let probe_node =
            print_probe.ok_or_else(|| format!("{LABEL} contains no qualified print probe"))?;
        let (tran_step_bits, tran_stop_bits) =
            tran_values.ok_or_else(|| format!("{LABEL} contains no qualified .TRAN"))?;
        let (reltol_bits, abstol_bits, transient_lte_reference) =
            option_values.ok_or_else(|| format!("{LABEL} contains no qualified .OPTIONS"))?;

        Ok(XyceAnalyticRcSourceContract {
            capacitor_name,
            capacitor_nodes,
            capacitance_bits,
            initial_voltage_bits,
            resistor_name,
            resistor_nodes,
            resistance_bits,
            source_name,
            source_nodes,
            source_value_bits,
            probe_node,
            tran_step_bits,
            tran_stop_bits,
            reltol_bits,
            abstol_bits,
            transient_lte_reference,
        })
    }

    pub(super) fn analytic_sinusoidal_rc_source_contract(
        source: &str,
    ) -> Result<XyceAnalyticSinusoidalRcSourceContract, String> {
        const LABEL: &str = "analytic sinusoidal first-order RC";

        let comp_lines = source
            .lines()
            .filter(|line| {
                line.split_whitespace()
                    .next()
                    .is_some_and(|token| token.eq_ignore_ascii_case("*COMP"))
            })
            .collect::<Vec<_>>();
        let [comp_line] = comp_lines.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one *COMP directive, found {}",
                comp_lines.len()
            ));
        };
        let comp_fields = Self::split_grouped_whitespace_fields(
            comp_line.trim(),
            "analytic sinusoidal RC *COMP directive",
        )?;
        let [
            comp_command,
            comp_probe,
            first_comp_option,
            second_comp_option,
        ] = comp_fields.as_slice()
        else {
            return Err(format!(
                "{LABEL} requires exactly '*COMP {{expression}} RELTOL=<numeric> ABSTOL=<numeric>'"
            ));
        };
        if !comp_command.eq_ignore_ascii_case("*COMP") {
            return Err(format!("{LABEL} comparison directive opcode changed"));
        }
        let mut verify_reltol = None;
        let mut verify_abstol = None;
        for option in [first_comp_option, second_comp_option] {
            let Some((name, value)) = option.split_once('=') else {
                return Err(format!(
                    "{LABEL} *COMP option '{option}' is not an assignment"
                ));
            };
            if value.contains('=') {
                return Err(format!("{LABEL} *COMP option '{option}' is malformed"));
            }
            let value = Self::single_spice_numeric_literal_value(value)?;
            if value <= 0.0 {
                return Err(format!("{LABEL} *COMP {name} must be finite and positive"));
            }
            if name.eq_ignore_ascii_case("RELTOL") {
                if verify_reltol.replace(value.to_bits()).is_some() {
                    return Err(format!("{LABEL} contains duplicate *COMP RELTOL"));
                }
            } else if name.eq_ignore_ascii_case("ABSTOL") {
                if verify_abstol.replace(value.to_bits()).is_some() {
                    return Err(format!("{LABEL} contains duplicate *COMP ABSTOL"));
                }
            } else {
                return Err(format!("{LABEL} does not admit *COMP option '{name}'"));
            }
        }
        let verify_reltol_bits =
            verify_reltol.ok_or_else(|| format!("{LABEL} *COMP has no RELTOL"))?;
        let verify_abstol_bits =
            verify_abstol.ok_or_else(|| format!("{LABEL} *COMP has no ABSTOL"))?;

        let lines = Self::logical_netlist_lines(source);
        let Some(title) = lines.first() else {
            return Err(format!("{LABEL} requires a circuit title"));
        };
        let title = Self::strip_netlist_comment(title).trim();
        if title.is_empty() || title.starts_with('.') {
            return Err(format!("{LABEL} requires an ordinary circuit title"));
        }

        let mut capacitor = None;
        let mut resistor = None;
        let mut voltage_source = None;
        let mut print_probe = None;
        let mut tran_values = None;
        let mut option_values = None;
        let mut end_count = 0usize;
        let mut element_names = BTreeSet::new();
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "analytic sinusoidal RC source statement",
            )?;
            let Some(command) = fields.first() else {
                continue;
            };
            if command.starts_with('.') {
                match command.to_ascii_lowercase().as_str() {
                    ".print" => {
                        if print_probe.is_some()
                            || fields.len() != 3
                            || !fields[1].eq_ignore_ascii_case("TRAN")
                        {
                            return Err(format!(
                                "{LABEL} requires one canonical '.PRINT TRAN {{V(node)+offset}}'"
                            ));
                        }
                        let (node, offset) =
                            Self::analytic_sinusoidal_rc_print_expression(&fields[2])?;
                        print_probe = Some((node, fields[2].clone(), offset.to_bits()));
                    }
                    ".tran" => {
                        if tran_values.is_some() || fields.len() != 3 {
                            return Err(format!(
                                "{LABEL} requires one canonical '.TRAN step stop'"
                            ));
                        }
                        let step = Self::single_spice_numeric_literal_value(&fields[1])?;
                        let stop = Self::single_spice_numeric_literal_value(&fields[2])?;
                        if step < 0.0 || stop <= 0.0 {
                            return Err(format!(
                                "{LABEL} requires a nonnegative print step and positive stop time"
                            ));
                        }
                        tran_values = Some((step.to_bits(), stop.to_bits()));
                    }
                    ".options" => {
                        if option_values.is_some() || fields.len() != 5 {
                            return Err(format!(
                                "{LABEL} requires exactly '.OPTIONS TIMEINT RELTOL=<numeric> ABSTOL=<numeric> METHOD=7'"
                            ));
                        }
                        let mut timeint = false;
                        let mut reltol = None;
                        let mut abstol = None;
                        let mut method = None;
                        for option in &fields[1..] {
                            if option.eq_ignore_ascii_case("TIMEINT") {
                                if timeint {
                                    return Err(format!("{LABEL} contains duplicate TIMEINT"));
                                }
                                timeint = true;
                                continue;
                            }
                            let Some((name, value)) = option.split_once('=') else {
                                return Err(format!(
                                    "{LABEL} option '{option}' is not a direct assignment"
                                ));
                            };
                            if value.contains('=') {
                                return Err(format!("{LABEL} option '{option}' is malformed"));
                            }
                            let value = Self::single_spice_numeric_literal_value(value)?;
                            if name.eq_ignore_ascii_case("RELTOL") {
                                if value <= 0.0 || reltol.replace(value.to_bits()).is_some() {
                                    return Err(format!(
                                        "{LABEL} requires one positive TIMEINT RELTOL"
                                    ));
                                }
                            } else if name.eq_ignore_ascii_case("ABSTOL") {
                                if value <= 0.0 || abstol.replace(value.to_bits()).is_some() {
                                    return Err(format!(
                                        "{LABEL} requires one positive TIMEINT ABSTOL"
                                    ));
                                }
                            } else if name.eq_ignore_ascii_case("METHOD") {
                                if value != 7.0 || method.replace("7".to_string()).is_some() {
                                    return Err(format!(
                                        "{LABEL} requires the direct Xyce ONESTEP selector METHOD=7"
                                    ));
                                }
                            } else {
                                return Err(format!("{LABEL} does not admit option '{name}'"));
                            }
                        }
                        if !timeint || reltol.is_none() || abstol.is_none() || method.is_none() {
                            return Err(format!(
                                "{LABEL} requires TIMEINT, RELTOL, ABSTOL, and METHOD=7 exactly once"
                            ));
                        }
                        option_values = Some((
                            reltol.expect("checked above"),
                            abstol.expect("checked above"),
                            method.expect("checked above"),
                        ));
                    }
                    ".end" if fields.len() == 1 => end_count += 1,
                    other => return Err(format!("{LABEL} does not admit directive '{other}'")),
                }
                continue;
            }

            if fields.len() < 4
                || !Self::is_single_spice_identifier(&fields[0])
                || !Self::is_single_spice_node_token(&fields[1])
                || !Self::is_single_spice_node_token(&fields[2])
            {
                return Err(format!(
                    "{LABEL} element '{command}' does not use direct name/node fields"
                ));
            }
            let element_name = Self::normalize_device_instance_name(&fields[0]);
            if !element_names.insert(element_name.clone()) {
                return Err(format!(
                    "{LABEL} contains duplicate element '{element_name}'"
                ));
            }
            let nodes = [
                Self::canonical_passive_primary_node_name(&fields[1]),
                Self::canonical_passive_primary_node_name(&fields[2]),
            ];
            match fields[0].chars().next().map(|ch| ch.to_ascii_uppercase()) {
                Some('C') => {
                    if capacitor.is_some() || fields.len() != 4 {
                        return Err(format!(
                            "{LABEL} requires one direct capacitor without model or IC fields"
                        ));
                    }
                    let value = Self::single_spice_numeric_literal_value(&fields[3])?;
                    if value <= 0.0 {
                        return Err(format!("{LABEL} capacitance must be positive"));
                    }
                    capacitor = Some((element_name, nodes, value.to_bits()));
                }
                Some('R') => {
                    if resistor.is_some() || fields.len() != 4 {
                        return Err(format!("{LABEL} requires one direct resistor"));
                    }
                    let value = Self::single_spice_numeric_literal_value(&fields[3])?;
                    if value <= 0.0 {
                        return Err(format!("{LABEL} resistance must be positive"));
                    }
                    resistor = Some((element_name, nodes, value.to_bits()));
                }
                Some('V') => {
                    if voltage_source.is_some()
                        || fields.len() != 9
                        || !fields[3].eq_ignore_ascii_case("SIN")
                    {
                        return Err(format!(
                            "{LABEL} requires one direct 'Vname n+ n- SIN vo va freq td theta' source"
                        ));
                    }
                    let values = fields[4..]
                        .iter()
                        .map(|field| Self::single_spice_numeric_literal_value(field))
                        .collect::<Result<Vec<_>, _>>()?;
                    let [offset, amplitude, frequency, delay, damping] = values.as_slice() else {
                        unreachable!("source arity was checked");
                    };
                    if *amplitude == 0.0 || *frequency <= 0.0 || *delay < 0.0 {
                        return Err(format!(
                            "{LABEL} source requires nonzero amplitude, positive frequency, and nonnegative delay"
                        ));
                    }
                    voltage_source = Some((
                        element_name,
                        nodes,
                        offset.to_bits(),
                        amplitude.to_bits(),
                        frequency.to_bits(),
                        delay.to_bits(),
                        damping.to_bits(),
                    ));
                }
                _ => return Err(format!("{LABEL} does not admit element '{command}'")),
            }
        }

        if end_count != 1 {
            return Err(format!(
                "{LABEL} requires exactly one .END statement, found {end_count}"
            ));
        }
        let (capacitor_name, capacitor_nodes, capacitance_bits) =
            capacitor.ok_or_else(|| format!("{LABEL} contains no qualified capacitor"))?;
        let (resistor_name, resistor_nodes, resistance_bits) =
            resistor.ok_or_else(|| format!("{LABEL} contains no qualified resistor"))?;
        let (
            source_name,
            source_nodes,
            source_offset_bits,
            source_amplitude_bits,
            source_frequency_bits,
            source_delay_bits,
            source_damping_bits,
        ) = voltage_source.ok_or_else(|| format!("{LABEL} contains no qualified SIN source"))?;
        let (probe_node, print_expression, print_offset_bits) =
            print_probe.ok_or_else(|| format!("{LABEL} contains no qualified print expression"))?;
        let (tran_step_bits, tran_stop_bits) =
            tran_values.ok_or_else(|| format!("{LABEL} contains no qualified .TRAN"))?;
        let (timeint_reltol_bits, timeint_abstol_bits, method_selector) =
            option_values.ok_or_else(|| format!("{LABEL} contains no qualified .OPTIONS"))?;

        if Self::normalize_probe(comp_probe) != Self::normalize_probe(&print_expression) {
            return Err(format!(
                "{LABEL} *COMP probe must exactly match the printed expression"
            ));
        }
        let fixed_values = [
            (
                "resistance",
                resistance_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_RESISTANCE.to_bits(),
            ),
            (
                "capacitance",
                capacitance_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_CAPACITANCE.to_bits(),
            ),
            (
                "source offset",
                source_offset_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_SOURCE_OFFSET.to_bits(),
            ),
            (
                "source amplitude",
                source_amplitude_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_SOURCE_AMPLITUDE.to_bits(),
            ),
            (
                "source frequency",
                source_frequency_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_FREQUENCY.to_bits(),
            ),
            ("source delay", source_delay_bits, 0.0f64.to_bits()),
            ("source damping", source_damping_bits, 0.0f64.to_bits()),
            (
                "print offset",
                print_offset_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_PRINT_OFFSET.to_bits(),
            ),
            (".TRAN print step", tran_step_bits, 0.0f64.to_bits()),
            (
                ".TRAN stop",
                tran_stop_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_ORACLE_STOP.to_bits(),
            ),
            (
                "TIMEINT RELTOL",
                timeint_reltol_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_TIMEINT_TOLERANCE.to_bits(),
            ),
            (
                "TIMEINT ABSTOL",
                timeint_abstol_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_TIMEINT_TOLERANCE.to_bits(),
            ),
            (
                "*COMP RELTOL",
                verify_reltol_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_VERIFY_TOLERANCE.to_bits(),
            ),
            (
                "*COMP ABSTOL",
                verify_abstol_bits,
                XYCE_ANALYTIC_SINUSOIDAL_RC_VERIFY_TOLERANCE.to_bits(),
            ),
        ];
        if let Some((name, actual, expected)) = fixed_values
            .into_iter()
            .find(|(_, actual, expected)| actual != expected)
        {
            return Err(format!(
                "{LABEL} {name} differs from the fixed Release 7.10 generator: actual={}, expected={}",
                Value::from_bits(actual),
                Value::from_bits(expected)
            ));
        }

        Ok(XyceAnalyticSinusoidalRcSourceContract {
            capacitor_name,
            capacitor_nodes,
            capacitance_bits,
            resistor_name,
            resistor_nodes,
            resistance_bits,
            source_name,
            source_nodes,
            source_offset_bits,
            source_amplitude_bits,
            source_frequency_bits,
            source_delay_bits,
            source_damping_bits,
            probe_node,
            print_expression,
            print_offset_bits,
            tran_step_bits,
            tran_stop_bits,
            timeint_reltol_bits,
            timeint_abstol_bits,
            method_selector,
            verify_reltol_bits,
            verify_abstol_bits,
        })
    }

    pub(super) fn passive_primary_source_contract(
        source: &str,
        element_name: &str,
        model_name: &str,
        kind: XycePassivePrimaryKind,
    ) -> Result<(XycePassivePrimaryRepresentation, u64, Vec<String>), String> {
        let mut representation = None;
        let mut value_bits = None;
        let mut fingerprint = Vec::new();
        let lines = Self::logical_netlist_lines(source);
        let Some(title) = lines.first() else {
            return Err("passive primary-value parity requires a circuit title".to_string());
        };
        fingerprint.push(Self::strip_netlist_comment(title).trim().to_string());
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let mut fields = Self::split_grouped_whitespace_fields(
                stripped,
                "passive primary-value source statement",
            )?;
            if fields
                .first()
                .is_some_and(|name| name.eq_ignore_ascii_case(element_name))
            {
                if representation.is_some() {
                    return Err(format!(
                        "passive primary-value parity requires exactly one active statement for '{element_name}'"
                    ));
                }
                if fields.len() != 5
                    || !Self::is_single_spice_identifier(&fields[3])
                    || !fields[3].eq_ignore_ascii_case(model_name)
                {
                    return Err(format!(
                        "primary device '{element_name}' must use exactly 'name n+ n- model primary-value'"
                    ));
                }
                let token = &fields[4];
                let (form, numeric) = if let Some((parameter, numeric)) = token.split_once('=') {
                    if numeric.contains('=')
                        || !parameter.eq_ignore_ascii_case(kind.primary_parameter())
                    {
                        return Err(format!(
                            "named primary token for '{element_name}' must be exactly {}=<numeric>",
                            kind.primary_parameter()
                        ));
                    }
                    (XycePassivePrimaryRepresentation::Named, numeric)
                } else {
                    (XycePassivePrimaryRepresentation::Positional, token.as_str())
                };
                let value = Self::single_spice_numeric_literal_value(numeric)?;
                representation = Some(form);
                value_bits = Some(value.to_bits());
                fields[4] = "<PRIMARY_VALUE>".to_string();
            }
            fingerprint.push(fields.join("\u{1f}"));
        }
        Ok((
            representation.ok_or_else(|| {
                format!("no active primary device statement found for '{element_name}'")
            })?,
            value_bits.expect("representation and value bits are set together"),
            fingerprint,
        ))
    }

    pub(super) fn validate_passive_primary_source_forms(
        source: &str,
        kind: XycePassivePrimaryKind,
    ) -> Result<(), String> {
        let lines = Self::logical_netlist_lines(source);
        let Some(title) = lines.first() else {
            return Err("passive primary-value parity requires a circuit title".to_string());
        };
        let title = Self::strip_netlist_comment(title).trim();
        if title.is_empty() || title.starts_with('.') {
            return Err(
                "passive primary-value parity requires an ordinary circuit title".to_string(),
            );
        }

        let mut element_counts = BTreeMap::<char, usize>::new();
        let mut directive_counts = BTreeMap::<String, usize>::new();
        let mut pulse_count = 0usize;
        let mut direct_voltage_count = 0usize;
        for line in lines.iter().skip(1) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "passive primary-value source statement",
            )?;
            let Some(command) = fields.first() else {
                continue;
            };
            if command.starts_with('.') {
                let directive = command.to_ascii_lowercase();
                *directive_counts.entry(directive.clone()).or_default() += 1;
                match (kind, directive.as_str()) {
                    (XycePassivePrimaryKind::CapacitorTran, ".model") => {
                        if fields.len() != 4
                            || !Self::is_single_spice_identifier(&fields[1])
                            || !fields[2].eq_ignore_ascii_case("C")
                            || fields[3] != "()"
                        {
                            return Err("capacitor primary-value parity requires exactly '.MODEL name C ()'".to_string());
                        }
                    }
                    (XycePassivePrimaryKind::ResistorDc, ".model") => {
                        if fields.len() != 4
                            || !Self::is_single_spice_identifier(&fields[1])
                            || !fields[2].eq_ignore_ascii_case("R")
                        {
                            return Err("resistor primary-value parity requires exactly '.MODEL name R (RSH=<numeric>)'".to_string());
                        }
                        let parameter = fields[3]
                            .strip_prefix('(')
                            .and_then(|inner| inner.strip_suffix(')'))
                            .ok_or_else(|| "resistor model parameters must use one parenthesized RSH assignment".to_string())?;
                        let Some((name, value)) = parameter.split_once('=') else {
                            return Err("resistor model must contain one RSH=<numeric> assignment"
                                .to_string());
                        };
                        if value.contains('=')
                            || !name.eq_ignore_ascii_case("RSH")
                            || Self::single_spice_numeric_literal_value(value)? <= 0.0
                        {
                            return Err("resistor model must contain exactly one finite positive RSH=<numeric> assignment".to_string());
                        }
                    }
                    (XycePassivePrimaryKind::CapacitorTran, ".tran") => {
                        if fields.len() != 3 {
                            return Err(
                                "capacitor primary-value parity requires '.TRAN step stop'"
                                    .to_string(),
                            );
                        }
                        Self::single_spice_numeric_literal_value(&fields[1])?;
                        Self::single_spice_numeric_literal_value(&fields[2])?;
                    }
                    (XycePassivePrimaryKind::ResistorDc, ".dc") => {
                        if fields.len() != 5 || !Self::is_single_spice_identifier(&fields[1]) {
                            return Err("resistor primary-value parity requires '.DC source start stop step'".to_string());
                        }
                        for value in &fields[2..] {
                            Self::single_spice_numeric_literal_value(value)?;
                        }
                    }
                    (XycePassivePrimaryKind::CapacitorTran, ".print") => {
                        if fields.len() != 4 || !fields[1].eq_ignore_ascii_case("TRAN") {
                            return Err("capacitor primary-value parity requires one canonical two-probe '.PRINT TRAN' statement".to_string());
                        }
                    }
                    (XycePassivePrimaryKind::ResistorDc, ".print") => {
                        if fields.len() != 4 || !fields[1].eq_ignore_ascii_case("DC") {
                            return Err("resistor primary-value parity requires one canonical two-probe '.PRINT DC' statement".to_string());
                        }
                    }
                    (_, ".end") if fields.len() == 1 => {}
                    _ => {
                        return Err(format!(
                            "passive primary-value parity does not admit directive '{command}'"
                        ));
                    }
                }
                continue;
            }

            let designator = command
                .chars()
                .next()
                .map(|ch| ch.to_ascii_uppercase())
                .ok_or_else(|| {
                    "passive primary-value parity contains an empty element name".to_string()
                })?;
            *element_counts.entry(designator).or_default() += 1;
            match (kind, designator) {
                (XycePassivePrimaryKind::CapacitorTran, 'C')
                | (XycePassivePrimaryKind::ResistorDc, 'R') => {
                    if fields.len() != 5 {
                        return Err(format!(
                            "primary device '{command}' must contain exactly one model and one primary-value token"
                        ));
                    }
                }
                (XycePassivePrimaryKind::CapacitorTran, 'R') => {
                    if fields.len() != 4 {
                        return Err(
                            "capacitor parity resistor must use 'Rname n+ n- value'".to_string()
                        );
                    }
                    Self::single_spice_numeric_literal_value(&fields[3])?;
                }
                (_, 'V') => {
                    if fields.len() != 4 {
                        return Err("passive primary-value voltage sources require exactly one source field".to_string());
                    }
                    if kind == XycePassivePrimaryKind::CapacitorTran
                        && fields[3]
                            .get(..5)
                            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("PULSE"))
                    {
                        Self::direct_numeric_function_arguments(&fields[3], "PULSE", 6)?;
                        pulse_count += 1;
                    } else {
                        Self::single_spice_numeric_literal_value(&fields[3])?;
                        direct_voltage_count += 1;
                    }
                }
                _ => {
                    return Err(format!(
                        "passive primary-value parity does not admit element '{command}'"
                    ));
                }
            }
        }

        let model_count = directive_counts.get(".model").copied().unwrap_or(0);
        let print_count = directive_counts.get(".print").copied().unwrap_or(0);
        let end_count = directive_counts.get(".end").copied().unwrap_or(0);
        match kind {
            XycePassivePrimaryKind::CapacitorTran
                if element_counts.get(&'C') == Some(&1)
                    && element_counts.get(&'R') == Some(&1)
                    && element_counts.get(&'V') == Some(&2)
                    && pulse_count == 1
                    && direct_voltage_count == 1
                    && model_count == 1
                    && directive_counts.get(".tran") == Some(&1)
                    && print_count == 1
                    && end_count == 1
                    && directive_counts.len() == 4 => {}
            XycePassivePrimaryKind::ResistorDc
                if element_counts.get(&'R') == Some(&1)
                    && element_counts.get(&'V') == Some(&2)
                    && direct_voltage_count == 2
                    && model_count == 1
                    && directive_counts.get(".dc") == Some(&1)
                    && print_count == 1
                    && end_count == 1
                    && directive_counts.len() == 4 => {}
            _ => {
                return Err(format!(
                    "passive primary-value {:?} source does not have the required bounded element and directive inventory",
                    kind
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_subckt_parameter_resolution_source_directives(
        source: &str,
    ) -> Result<(), String> {
        const LABEL: &str = "subcircuit-parameter resolution";
        if source.trim().is_empty()
            || Self::source_has_comp_directive(source)
            || source
                .lines()
                .find(|line| !line.trim().is_empty())
                .is_none_or(|line| !line.trim_start().starts_with('*'))
        {
            return Err(format!(
                "{LABEL} requires a nonempty comment-titled source without *COMP"
            ));
        }
        let mut directive_counts = BTreeMap::<String, usize>::new();
        let mut element_counts = BTreeMap::<char, usize>::new();
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            let command = stripped.split_whitespace().next().unwrap_or_default();
            if command.starts_with('.') {
                let normalized = command.to_ascii_lowercase();
                if !matches!(
                    normalized.as_str(),
                    ".param" | ".dc" | ".print" | ".subckt" | ".ends" | ".end"
                ) {
                    return Err(format!("{LABEL} does not admit directive '{command}'"));
                }
                *directive_counts.entry(normalized).or_default() += 1;
                continue;
            }
            let designator = command
                .chars()
                .next()
                .map(|value| value.to_ascii_uppercase())
                .ok_or_else(|| format!("{LABEL} contains an empty statement"))?;
            if !matches!(designator, 'X' | 'V' | 'R') {
                return Err(format!(
                    "{LABEL} admits only one subcircuit instance, one independent voltage source, and one resistor body; got '{command}'"
                ));
            }
            *element_counts.entry(designator).or_default() += 1;
        }
        for directive in [".dc", ".print", ".subckt", ".ends", ".end"] {
            if directive_counts.get(directive).copied() != Some(1) {
                return Err(format!(
                    "{LABEL} requires exactly one {directive} statement"
                ));
            }
        }
        if directive_counts.get(".param").copied().unwrap_or(0) > 1
            || element_counts.get(&'X').copied() != Some(1)
            || element_counts.get(&'V').copied() != Some(1)
            || element_counts.get(&'R').copied() != Some(1)
            || element_counts.len() != 3
        {
            return Err(format!(
                "{LABEL} requires at most one global .PARAM and exactly one X/V/R statement"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_param_expression_direct_source_forms(
        source: &str,
    ) -> Result<(), String> {
        const LABEL: &str = "parameter-expression parity";
        let lines = Self::logical_netlist_lines(source);
        let Some(title) = source.lines().find(|line| !line.trim().is_empty()) else {
            return Err(format!("{LABEL} requires a circuit title"));
        };
        let title = title
            .split_once(';')
            .map(|(head, _)| head)
            .unwrap_or(title)
            .trim();
        if title.is_empty() || title.starts_with('.') {
            return Err(format!("{LABEL} requires an ordinary circuit title"));
        }
        let logical_title_count = usize::from(
            !Self::strip_netlist_comment(title).trim().is_empty()
                && lines.first().is_some_and(|line| line.trim() == title),
        );

        let mut behavioral_count = 0usize;
        let mut resistor_count = 0usize;
        let mut voltage_count = 0usize;
        let mut instance_count = 0usize;
        for line in lines.iter().skip(logical_title_count) {
            let stripped = Self::strip_netlist_comment(line).trim();
            let Some(command) = stripped.split_whitespace().next() else {
                continue;
            };
            if command.starts_with('.') {
                if command.eq_ignore_ascii_case(".TRAN") {
                    let fields = Self::split_grouped_whitespace_fields(
                        stripped,
                        "parameter-expression .TRAN statement",
                    )?;
                    if fields.len() != 3
                        || !Self::is_single_spice_numeric_literal(&fields[1])
                        || !Self::is_single_spice_numeric_literal(&fields[2])
                    {
                        return Err(format!(
                            "{LABEL} requires direct numeric '.TRAN step stop' tokens"
                        ));
                    }
                }
                continue;
            }

            let fields = Self::split_grouped_whitespace_fields(
                stripped,
                "parameter-expression element statement",
            )?;
            let Some(designator) = fields[0].chars().next().map(|ch| ch.to_ascii_uppercase())
            else {
                return Err(format!("{LABEL} contains an empty element name"));
            };
            match designator {
                'B' => {
                    behavioral_count += 1;
                    if fields.len() != 4 || !Self::is_single_braced_voltage_assignment(&fields[3]) {
                        return Err(format!(
                            "{LABEL} behavioral source must use exactly 'Bname n+ n- V={{expression}}'"
                        ));
                    }
                }
                'R' => {
                    resistor_count += 1;
                    if fields.len() != 4 || !Self::is_single_spice_numeric_literal(&fields[3]) {
                        return Err(format!(
                            "{LABEL} resistors must use direct numeric 'Rname n+ n- value' form"
                        ));
                    }
                }
                'V' => {
                    voltage_count += 1;
                    if fields.len() != 4 || !Self::is_single_spice_numeric_literal(&fields[3]) {
                        return Err(format!(
                            "{LABEL} voltage source must use direct numeric 'Vname n+ n- value' form"
                        ));
                    }
                }
                'X' => {
                    instance_count += 1;
                    if fields.len() != 8 {
                        return Err(format!(
                            "{LABEL} subcircuit instance must bind exactly six nodes without parameter fields"
                        ));
                    }
                }
                _ => {
                    return Err(format!(
                        "{LABEL} does not admit element statement '{command}'"
                    ));
                }
            }
        }
        if (
            behavioral_count,
            resistor_count,
            voltage_count,
            instance_count,
        ) != (1, 2, 1, 1)
        {
            return Err(format!(
                "{LABEL} requires exactly one behavioral source, two resistors, one direct DC source, and one subcircuit instance; found ({behavioral_count}, {resistor_count}, {voltage_count}, {instance_count})"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_sin_expression_source_form(
        netlist: &Netlist,
        source_name: &str,
        representation: XyceSinExpressionRepresentation,
    ) -> Result<(), String> {
        let source = netlist.source_text.as_deref().ok_or_else(|| {
            "exact SIN/SPICE_SIN parity requires original source text for arity qualification"
                .to_string()
        })?;
        let mut matching_lines = Vec::new();
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            if stripped
                .split_whitespace()
                .next()
                .is_some_and(|name| name.eq_ignore_ascii_case(source_name))
            {
                matching_lines.push(stripped.to_string());
            }
        }
        let [source_line] = matching_lines.as_slice() else {
            return Err(format!(
                "exact SIN/SPICE_SIN parity requires exactly one source line for '{source_name}', found {}",
                matching_lines.len()
            ));
        };
        let fields = Self::split_grouped_whitespace_fields(
            source_line,
            "exact SIN/SPICE_SIN source statement",
        )?;
        match representation {
            XyceSinExpressionRepresentation::IndependentSin => {
                if fields.len() != 7
                    || !fields[3].eq_ignore_ascii_case("SIN")
                    || !fields[4..]
                        .iter()
                        .all(|field| Self::is_single_spice_numeric_literal(field))
                {
                    return Err(format!(
                        "independent source '{source_name}' must use exactly three standalone numeric tokens in 'SIN offset amplitude frequency' form"
                    ));
                }
            }
            XyceSinExpressionRepresentation::BehavioralSpiceSin => {
                if fields.len() != 4 || !Self::is_single_braced_voltage_assignment(&fields[3]) {
                    return Err(format!(
                        "behavioral source '{source_name}' must use one complete braced V={{SPICE_SIN(...)}} assignment without adjacent or trailing fields"
                    ));
                }
            }
        }
        Ok(())
    }

    pub(super) fn validate_xyce_capacitor_contract_params(
        netlist: &Netlist,
        element_name: &str,
    ) -> Result<(), String> {
        let element = Self::find_capacitor_element(netlist, element_name)
            .ok_or_else(|| format!("capacitor '{}' not found", element_name))?;
        let ElementKind::Capacitor {
            value,
            value_expr,
            initial_voltage,
            model,
            instance_params,
            deferred_params,
            ..
        } = &element.kind
        else {
            return Err(format!("element '{}' is not a capacitor", element_name));
        };

        if !value.is_finite() {
            if let Some(expression) = value_expr.as_deref() {
                if initial_voltage.is_some() {
                    return Err(format!(
                        "native static .PRINT TRAN comparison does not support solution-dependent capacitor '{}' with an explicit initial voltage",
                        element_name
                    ));
                }
                if !deferred_params.is_empty() {
                    return Err(format!(
                        "native static .PRINT TRAN comparison does not support unresolved instance parameters on solution-dependent capacitor '{}'",
                        element_name
                    ));
                }
                Self::validate_solution_dependent_capacitor_expression(
                    element_name,
                    expression,
                    &netlist.params,
                )?;
            } else if model.is_none() {
                return Err(format!(
                    "native static .PRINT TRAN comparison could not resolve capacitor value for element '{}'",
                    element_name
                ));
            }
        }

        const INSTANCE_PARAMS: &[&str] = &[
            "L", "W", "M", "MULT", "SCALE", "TEMP", "DTEMP", "TC1", "TC2", "AGE", "D",
        ];
        for (name, value) in instance_params {
            if !INSTANCE_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not yet support Xyce capacitor instance parameter {} on element '{}'",
                    name, element_name
                ));
            }
            if !value.is_finite() {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not support capacitor '{}' with non-finite instance parameter {}={}",
                    element_name, name, value
                ));
            }
            if (name.eq_ignore_ascii_case("M")
                || name.eq_ignore_ascii_case("MULT")
                || name.eq_ignore_ascii_case("SCALE"))
                && *value <= 0.0
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not support capacitor '{}' with non-positive instance parameter {}={}",
                    element_name, name, value
                ));
            }
            if name.eq_ignore_ascii_case("AGE") && *value < 0.0 {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not support capacitor '{}' with negative AGE={}",
                    element_name, value
                ));
            }
        }

        let Some(model_name) = model.as_deref() else {
            return Ok(());
        };
        let model = Self::find_model(&netlist.models, model_name).ok_or_else(|| {
            format!(
                "native static .PRINT TRAN comparison could not find capacitor '{}' model '{}'",
                element_name, model_name
            )
        })?;
        if !matches!(
            model.model_type.to_ascii_uppercase().as_str(),
            "C" | "CAP" | "CAPACITOR"
        ) {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support capacitor '{}' model '{}' of type '{}'",
                element_name, model_name, model.model_type
            ));
        }

        const MODEL_PARAMS: &[&str] = &["C", "CJ", "CJSW", "DEFW", "NARROW", "TC1", "TC2", "TNOM"];
        for (name, value) in &model.params {
            if !MODEL_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not yet support Xyce capacitor model parameter {} on model '{}'",
                    name, model_name
                ));
            }
            if !value.is_finite() {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not support capacitor model '{}' with non-finite parameter {}={}",
                    model_name, name, value
                ));
            }
        }
        for (name, expression) in &model.expr_params {
            if !MODEL_PARAMS
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
            {
                return Err(format!(
                    "native static .PRINT TRAN comparison does not yet support Xyce capacitor model expression parameter {} on model '{}'",
                    name, model_name
                ));
            }
            Self::validate_static_transient_passive_value_expression(
                "capacitor model parameter",
                name,
                expression,
                &netlist.params,
            )?;
        }
        if !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "native static .PRINT TRAN comparison does not support non-scalar capacitor model parameters on model '{}'",
                model_name
            ));
        }
        Ok(())
    }

    pub(super) fn validate_scoped_model_relational_source(
        source_name: &str,
        spec: &rspice_core::netlist::SourceSpec,
    ) -> Result<(), String> {
        if Self::source_spec_is_finite_dc_or_pulse(spec) {
            Ok(())
        } else {
            Err(format!(
                "native scoped-model relational source '{source_name}' must be finite DC or PULSE"
            ))
        }
    }

    pub(super) fn validate_finite_controlled_source_gain(
        source_kind: &str,
        element_name: &str,
        value_name: &str,
        value: Value,
    ) -> Result<(), String> {
        if value.is_finite() {
            Ok(())
        } else {
            Err(format!(
                "native static .PRINT TRAN comparison does not support {source_kind} element '{element_name}' with non-finite {value_name} {value}"
            ))
        }
    }

    pub(super) fn validate_current_controlled_source_probe(
        elements: &[rspice_core::netlist::Element],
        source_kind: &str,
        element_name: &str,
        control_element: &str,
    ) -> Result<(), String> {
        if Self::elements_have_recorded_branch_current(elements, control_element) {
            Ok(())
        } else {
            Err(format!(
                "native static .PRINT TRAN comparison does not support {source_kind} element '{element_name}' because controlling element '{control_element}' has no recorded branch current"
            ))
        }
    }

    pub(super) fn reject_unsupported_source_directives(source: &str) -> Result<(), String> {
        for line in Self::logical_netlist_lines(source) {
            let trimmed = Self::strip_netlist_comment(&line)
                .trim()
                .to_ascii_lowercase();
            if trimmed.is_empty() {
                continue;
            }
            if let Some(rest) = trimmed.strip_prefix(".options") {
                let option_tokens = rest
                    .split(|ch: char| ch.is_ascii_whitespace() || ch == ',')
                    .filter(|token| !token.is_empty())
                    .collect::<Vec<_>>();
                if option_tokens.contains(&"loca")
                    || (option_tokens.contains(&"nonlin")
                        && option_tokens
                            .iter()
                            .any(|token| token.starts_with("continuation")))
                {
                    return Err(
                        "deck requires Xyce LOCA continuation options; the native Xyce adapter does not yet implement continuation analysis semantics"
                            .to_string(),
                    );
                }
            }
        }
        Ok(())
    }

    pub(super) fn subckt_parameter_resolution_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceSubcktParameterResolutionFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }
        let parent = deck.path.parent()?;
        // This relational contract is defined by one empty wrapper anchor and
        // exactly six executable representations.  Reject other directory
        // shapes before reading/parsing member sources; wrapper-only folders
        // can contain very large cards that are not eligible for this family.
        if Self::circuit_file_count(parent)? != 7 {
            return None;
        }
        let mut anchor_path = None;
        let mut records = Vec::<(
            PathBuf,
            XyceSubcktParameterResolutionRepresentation,
            XyceStaticDcPlan,
            Option<XyceSubcktParameterResolutionSnapshot>,
            String,
        )>::new();
        let mut all_paths = Vec::new();
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
                || self
                    .static_prn_reference_path(&path)
                    .is_some_and(|reference| reference.is_file())
            {
                return None;
            }
            all_paths.push(path.clone());
            let member_relative = self.relative_key(&path);
            let wrapper = self.requires_upstream_wrapper(&member_relative);
            if fs::metadata(&path).ok()?.len() == 0 {
                if !wrapper || anchor_path.replace(path).is_some() {
                    return None;
                }
                continue;
            }
            let source = fs::read_to_string(&path).ok()?;
            Self::validate_subckt_parameter_resolution_source_directives(&source).ok()?;
            let plan = self
                .static_dc_plan_for_path(&path, ExpressionDialect::Xyce)
                .ok()?;
            Self::validate_subckt_parameter_resolution_dc_plan(&plan).ok()?;
            let netlist = Self::parse_xyce_netlist(&plan.source, &path).ok()?;
            let (representation, parameter_name, snapshot) =
                Self::subckt_parameter_resolution_qualification(
                    &netlist,
                    &plan.print,
                    &plan.dc.source,
                )
                .ok()?;
            if wrapper
                != (representation == XyceSubcktParameterResolutionRepresentation::UndefinedBinding)
            {
                return None;
            }
            records.push((path, representation, plan, snapshot, parameter_name));
        }
        let anchor_path = anchor_path?;
        if all_paths.len() != records.len() + 1 || records.len() != 6 {
            return None;
        }

        let mut representation_counts = BTreeMap::new();
        let parameter_names = records
            .iter()
            .map(|(_, _, _, _, name)| name.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        for (_, representation, _, _, _) in &records {
            *representation_counts.entry(*representation).or_default() += 1usize;
        }
        let required = [
            XyceSubcktParameterResolutionRepresentation::FormalDefaultAndInstanceOverride,
            XyceSubcktParameterResolutionRepresentation::ImplicitInstanceBinding,
            XyceSubcktParameterResolutionRepresentation::GlobalBinding,
            XyceSubcktParameterResolutionRepresentation::InstanceOverridesGlobal,
            XyceSubcktParameterResolutionRepresentation::UnusedInstanceBinding,
            XyceSubcktParameterResolutionRepresentation::UndefinedBinding,
        ];
        if parameter_names.len() != 1
            || required.iter().any(|representation| {
                representation_counts
                    .get(representation)
                    .copied()
                    .unwrap_or(0)
                    != 1
            })
        {
            return None;
        }

        let baseline_record = records.iter().find(|(_, representation, _, _, _)| {
            *representation
                == XyceSubcktParameterResolutionRepresentation::FormalDefaultAndInstanceOverride
        })?;
        let baseline_snapshot = baseline_record.3.as_ref()?;
        for (_, representation, plan, snapshot, _) in &records {
            if plan.print.probes != baseline_record.2.print.probes
                || !Self::dc_sweeps_match_exactly(&baseline_record.2.dc, &plan.dc)
            {
                return None;
            }
            if *representation == XyceSubcktParameterResolutionRepresentation::UndefinedBinding {
                if snapshot.is_some() {
                    return None;
                }
            } else if snapshot
                .as_ref()
                .map(|snapshot| &snapshot.flattened_elements)
                != Some(&baseline_snapshot.flattened_elements)
            {
                return None;
            }
        }

        let baseline_path = baseline_record.0.clone();
        let error_path = records
            .iter()
            .find(|(_, representation, _, _, _)| {
                *representation == XyceSubcktParameterResolutionRepresentation::UndefinedBinding
            })?
            .0
            .clone();
        if !all_paths
            .iter()
            .any(|path| Self::same_path(path, &deck.path))
        {
            return None;
        }
        let role = if Self::same_path(&deck.path, &anchor_path) {
            XyceSubcktParameterResolutionRole::Anchor
        } else if Self::same_path(&deck.path, &error_path) {
            XyceSubcktParameterResolutionRole::ExpectedError
        } else if Self::same_path(&deck.path, &baseline_path) {
            XyceSubcktParameterResolutionRole::Baseline
        } else {
            XyceSubcktParameterResolutionRole::Member
        };
        let valid_paths = records
            .into_iter()
            .filter_map(|(path, representation, _, _, _)| {
                (representation != XyceSubcktParameterResolutionRepresentation::UndefinedBinding)
                    .then_some(path)
            })
            .collect::<Vec<_>>();
        Some(XyceSubcktParameterResolutionFamilyContract {
            family: parent.file_name()?.to_str()?.to_string(),
            anchor_path,
            error_path,
            baseline_path,
            valid_paths,
            role,
            target_path: deck.path.clone(),
        })
    }

    pub(super) fn validate_nested_include_support_source(source: &str) -> Result<(), String> {
        let mut subckt_count = 0usize;
        let mut ends_count = 0usize;
        let mut resistor_count = 0usize;
        for line in Self::logical_netlist_lines(source) {
            let stripped = Self::strip_netlist_comment(&line).trim();
            if stripped.is_empty() {
                continue;
            }
            let head = stripped.split_whitespace().next().unwrap_or("");
            if head.eq_ignore_ascii_case(".subckt") {
                subckt_count += 1;
            } else if head.eq_ignore_ascii_case(".ends") {
                ends_count += 1;
            } else if head
                .chars()
                .next()
                .is_some_and(|character| character.eq_ignore_ascii_case(&'R'))
            {
                resistor_count += 1;
            } else {
                return Err(format!(
                    "nested-include support source contains unqualified statement '{stripped}'"
                ));
            }
        }
        if subckt_count != 1 || ends_count != 1 || resistor_count != 1 {
            return Err("nested-include support source must contain exactly one resistor-only subcircuit definition".to_string());
        }
        // Standalone include fragments intentionally begin with .SUBCKT and have no
        // circuit-title record. Supply a validation-only title so the ordinary
        // top-level parser does not consume the first directive as SPICE's title.
        let validation_source = format!("nested include support validation\n{source}");
        let parsed = Netlist::parse(&validation_source)
            .map_err(|err| format!("support source parse failed: {err}"))?;
        if parsed.subcircuits.len() != 1
            || parsed.subcircuits[0].ports.len() != 2
            || parsed.subcircuits[0].elements.len() != 1
        {
            return Err("support source is not one two-terminal local subcircuit".to_string());
        }
        Self::validate_nested_include_subcircuit_auxiliary_state(&parsed.subcircuits[0])?;
        Self::strict_nested_include_resistor_fingerprint(&parsed.subcircuits[0].elements[0])?;
        Ok(())
    }

    pub(super) fn bug754_global_parameter_relational_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug754GlobalParameterContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = match relative.as_str() {
            XYCE_BUG754_GLOBAL_PARAMETER_OWNER_RECORD => {
                XyceBug754GlobalParameterRole::GlobalParameterOwner
            }
            XYCE_BUG754_LITERAL_REFERENCE_RECORD => XyceBug754GlobalParameterRole::LiteralReference,
            _ => return None,
        };
        Some((|| {
            let parent = deck
                .path
                .parent()
                .ok_or_else(|| "BUG 754 record has no sibling directory".to_string())?;
            let owner_path = parent.join("dcsweep_globalpar.cir");
            let reference_path = parent.join("dcsweep_nopar.cir");
            if Self::normalize_manifest_key(&self.relative_key(&owner_path))
                != XYCE_BUG754_GLOBAL_PARAMETER_OWNER_RECORD
                || Self::normalize_manifest_key(&self.relative_key(&reference_path))
                    != XYCE_BUG754_LITERAL_REFERENCE_RECORD
            {
                return Err("owner/reference paths are not the exact BUG 754 sibling pair".into());
            }
            if !self.requires_upstream_wrapper(XYCE_BUG754_GLOBAL_PARAMETER_OWNER_RECORD)
                || self.requires_upstream_wrapper(XYCE_BUG754_LITERAL_REFERENCE_RECORD)
            {
                return Err(
                    "exactly dcsweep_globalpar.cir must own the removed upstream wrapper".into(),
                );
            }

            let mut family_members = BTreeSet::new();
            for entry in fs::read_dir(parent)
                .map_err(|err| format!("could not inspect BUG 754 directory: {err}"))?
            {
                let entry =
                    entry.map_err(|err| format!("could not inspect BUG 754 entry: {err}"))?;
                let path = entry.path();
                let file_name = entry.file_name();
                let name = file_name
                    .to_str()
                    .ok_or_else(|| "BUG 754 directory contains a non-Unicode entry".to_string())?;
                let normalized_name = name.to_ascii_lowercase();
                if normalized_name.starts_with("dcsweep_")
                    && path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                {
                    let metadata = fs::symlink_metadata(&path)
                        .map_err(|err| format!("could not inspect BUG 754 family member: {err}"))?;
                    if !metadata.file_type().is_file()
                        || metadata.file_type().is_symlink()
                        || metadata.len() == 0
                    {
                        return Err(format!(
                            "BUG 754 family member '{}' must be a nonempty regular non-symlink file",
                            path.display()
                        ));
                    }
                    Self::insert_bug754_family_member_name(&mut family_members, name)?;
                }
            }
            if family_members
                != BTreeSet::from([
                    "dcsweep_globalpar.cir".to_string(),
                    "dcsweep_nopar.cir".to_string(),
                ])
            {
                return Err(format!(
                    "BUG 754 dcsweep family must contain exactly its two records, got {family_members:?}"
                ));
            }
            for (member_role, path) in [
                ("global-parameter owner", &owner_path),
                ("literal reference", &reference_path),
            ] {
                self.reject_wrapper_output_artifacts(path)
                    .map_err(|err| format!("{member_role} {err}"))?;
            }

            let owner_source = fs::read_to_string(&owner_path)
                .map_err(|err| format!("failed to read BUG 754 owner: {err}"))?;
            let reference_source = fs::read_to_string(&reference_path)
                .map_err(|err| format!("failed to read BUG 754 reference: {err}"))?;
            Self::validate_bug754_source_pair(&owner_source, &reference_source)?;

            let owner_plan = self.static_dc_plan_for_path(&owner_path, ExpressionDialect::Xyce)?;
            let reference_plan =
                self.static_dc_plan_for_path(&reference_path, ExpressionDialect::Xyce)?;
            let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path)
                .map_err(|err| format!("BUG 754 owner parse failed: {err}"))?;
            let reference_netlist =
                Self::parse_xyce_netlist(&reference_plan.source, &reference_path)
                    .map_err(|err| format!("BUG 754 reference parse failed: {err}"))?;
            Self::validate_bug754_member(
                &owner_plan,
                &owner_netlist,
                XyceBug754GlobalParameterRole::GlobalParameterOwner,
            )?;
            Self::validate_bug754_member(
                &reference_plan,
                &reference_netlist,
                XyceBug754GlobalParameterRole::LiteralReference,
            )?;
            let owner_snapshot =
                Self::bug754_global_parameter_snapshot(&owner_plan, &owner_netlist)?;
            let reference_snapshot =
                Self::bug754_global_parameter_snapshot(&reference_plan, &reference_netlist)?;
            if owner_snapshot != reference_snapshot {
                return Err(format!(
                    "BUG 754 effective circuits differ after global-parameter normalization: owner={owner_snapshot:?} reference={reference_snapshot:?}"
                ));
            }

            Ok(XyceBug754GlobalParameterContract {
                owner_plan,
                reference_plan,
                role,
            })
        })())
    }

    pub(super) fn validate_bug754_source_pair(owner: &str, reference: &str) -> Result<(), String> {
        let statements = |source: &str| {
            Self::logical_netlist_lines(source)
                .into_iter()
                .filter_map(|line| {
                    let statement = Self::strip_netlist_comment(&line).trim();
                    (!statement.is_empty()).then(|| Self::normalize_probe(statement))
                })
                .collect::<Vec<_>>()
        };
        let owner_statements = statements(owner);
        let reference_statements = statements(reference);
        let expected_owner = [
            ".global_paramvgi=0.5",
            ".global_paramvdi=1.0",
            "m1draingatesource0mlev1",
            "vdraindrain0dc{vdi}",
            "vgategate0dc.5",
            "vsourcesource0dc{0}",
            ".dcvdrain010.001",
            ".printdcv(drain)v(gate)i(vdrain)",
            ".modelmlev1nmoslevel=1",
            ".end",
        ];
        let expected_reference = [
            "m1draingatesource0mlev1",
            "vdraindrain0dc1.0",
            "vgategate0dc.5",
            "vsourcesource0dc{0}",
            ".dcvdrain010.001",
            ".printdcv(drain)v(gate)i(vdrain)",
            ".modelmlev1nmoslevel=1",
            ".end",
        ];
        if owner_statements != expected_owner {
            return Err(format!(
                "BUG 754 owner executable statements changed: {owner_statements:?}"
            ));
        }
        if reference_statements != expected_reference {
            return Err(format!(
                "BUG 754 reference executable statements changed: {reference_statements:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug655_continuation_source_pair(
        owner_source: &str,
        reference_source: &str,
    ) -> Result<(), String> {
        let normalize = |source: &str, role: &str| {
            if !source.is_ascii() {
                return Err(format!("BUG 655 {role} source must be ASCII"));
            }
            let normalized = source.replace("\r\n", "\n");
            if normalized.contains('\r') {
                return Err(format!(
                    "BUG 655 {role} source contains a noncanonical bare carriage return"
                ));
            }
            Ok(normalized)
        };
        let owner = normalize(owner_source, "owner")?;
        let reference = normalize(reference_source, "reference")?;
        if owner != XYCE_BUG655_CANONICAL_OWNER_SOURCE {
            return Err("BUG 655 owner source is not the canonical continuation deck".into());
        }
        let mut expected_reference_lines = XYCE_BUG655_CANONICAL_OWNER_SOURCE
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>();
        for (line_index, spaces) in [(11usize, 2usize), (13, 1), (14, 3)] {
            expected_reference_lines[line_index] = format!(
                "{}{}",
                " ".repeat(spaces),
                expected_reference_lines[line_index]
            );
        }
        let expected_reference = format!("{}\n", expected_reference_lines.join("\n"));
        if reference != expected_reference {
            return Err(
                "BUG 655 reference source is not the canonical 2/1/3-space continuation variant"
                    .into(),
            );
        }

        let owner_lines = owner.lines().collect::<Vec<_>>();
        let reference_lines = reference.lines().collect::<Vec<_>>();
        if owner_lines.len() != 21 || reference_lines.len() != 21 {
            return Err(format!(
                "BUG 655 siblings must each contain exactly 21 physical lines, got owner={} reference={}",
                owner_lines.len(),
                reference_lines.len()
            ));
        }
        for line_index in 0..21 {
            if matches!(line_index, 11 | 13 | 14) {
                let expected_spaces = match line_index {
                    11 => 2,
                    13 => 1,
                    14 => 3,
                    _ => unreachable!(),
                };
                if !owner_lines[line_index].starts_with('+')
                    || reference_lines[line_index]
                        .len()
                        .saturating_sub(reference_lines[line_index].trim_start().len())
                        != expected_spaces
                    || reference_lines[line_index].trim_start() != owner_lines[line_index]
                {
                    return Err(format!(
                        "BUG 655 line {} does not preserve the exact leading-whitespace-only continuation difference",
                        line_index + 1
                    ));
                }
            } else if owner_lines[line_index] != reference_lines[line_index] {
                return Err(format!(
                    "BUG 655 siblings differ outside continuation lines 12, 14, and 15 at physical line {}",
                    line_index + 1
                ));
            }
        }
        Ok(())
    }

    pub(super) fn validate_bug662_header_source_pair(
        owner_source: &str,
        reference_source: &str,
    ) -> Result<(), String> {
        let (owner_header, owner_body) =
            Self::bug662_header_and_body(owner_source, XyceBug662HeaderRole::LongHeaderOwner)?;
        let (reference_header, reference_body) = Self::bug662_header_and_body(
            reference_source,
            XyceBug662HeaderRole::ShortHeaderReference,
        )?;
        if owner_body != reference_body {
            return Err(
                "BUG 662 siblings differ outside the title/header wrapping boundary".into(),
            );
        }
        let continuation = reference_header[1]
            .strip_prefix('*')
            .ok_or_else(|| "short-header continuation is not a SPICE comment".to_string())?;
        if owner_header[0] != format!("{}{continuation}", reference_header[0]) {
            return Err(
                "long title is not the exact reconstruction of the split short-header title".into(),
            );
        }
        if owner_header[0] != XYCE_BUG662_CANONICAL_LONG_TITLE {
            return Err("BUG 662 reconstructed title content is not canonical".into());
        }
        Ok(())
    }

    pub(super) fn params1_historical_oracle_provenance_records() -> Vec<String> {
        [
            (
                XYCE_PARAMS1_HISTORICAL_WRAPPER_PATH,
                XYCE_PARAMS1_HISTORICAL_WRAPPER_BYTES,
                XYCE_PARAMS1_HISTORICAL_WRAPPER_SHA256,
                XYCE_PARAMS1_HISTORICAL_WRAPPER_BLAKE3,
            ),
            (
                XYCE_RELEASE_710_XYCE_VERIFY_PATH,
                XYCE_RELEASE_710_XYCE_VERIFY_BYTES,
                XYCE_RELEASE_710_XYCE_VERIFY_SHA256,
                XYCE_RELEASE_710_XYCE_VERIFY_BLAKE3,
            ),
        ]
        .into_iter()
        .map(|(path, bytes, sha256, content_blake3)| {
            format!(
                "{XYCE_PARAMS1_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_PARAMS1_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
            )
        })
        .collect()
    }

    pub(super) fn validate_params1_historical_oracle_provenance() -> Result<(), String> {
        let mut records = Self::params1_historical_oracle_provenance_records();
        records.sort();
        let provenance_hash = blake3::hash(records.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if records.len() != XYCE_PARAMS1_HISTORICAL_ORACLE_RECORD_COUNT
            || provenance_hash != XYCE_PARAMS1_HISTORICAL_ORACLE_BLAKE3
        {
            return Err(format!(
                "PARAMS1 Release-7.10 wrapper/xyce_verify provenance changed: records={}/{provenance_hash}",
                records.len()
            ));
        }
        Ok(())
    }

    pub(super) fn params1_current_wrapper_manifest_rows(&self) -> Result<Vec<String>, String> {
        const LABEL: &str = "PARAMS1 wrapper manifest";
        let path = self.root.join(HARNESS_MANIFEST_FILE);
        let metadata = fs::symlink_metadata(&path)
            .map_err(|error| format!("failed to inspect {LABEL}: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
            return Err(format!(
                "{LABEL} '{}' must be a regular non-symlink file",
                self.display_path(&path)
            ));
        }
        let bytes = fs::read(&path).map_err(|error| format!("failed to read {LABEL}: {error}"))?;
        let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
        let source = std::str::from_utf8(&canonical)
            .map_err(|error| format!("{LABEL} is not UTF-8: {error}"))?;
        let expected =
            format!("Netlists/PARAMS1/params_a.cir\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}");
        let mut rows = Vec::new();
        for (line_index, line) in source.lines().enumerate() {
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            let Some((raw_path, _)) = line.split_once('\t') else {
                if line.to_ascii_lowercase().contains("params1") {
                    return Err(format!(
                        "{LABEL} line {} mentioning PARAMS1 is not tab-delimited",
                        line_index + 1
                    ));
                }
                continue;
            };
            if Self::normalize_manifest_key(raw_path).starts_with("netlists/params1/") {
                if line != expected {
                    return Err(format!(
                        "{LABEL} line {} is not the exact removed-wrapper owner row",
                        line_index + 1
                    ));
                }
                rows.push(line.to_string());
            }
        }
        if rows != [expected] {
            return Err(format!(
                "{LABEL} requires exactly the canonical params_a.cir owner row"
            ));
        }
        Ok(rows)
    }

    pub(super) fn params1_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceParams1FamilyContract, String>> {
        let relative = Self::normalize_manifest_key(&deck.relative_path);
        let role = XyceParams1Role::for_record(&relative)?;
        Some((|| {
            const LABEL: &str = "PARAMS1 parameter-equivalence family";
            if deck.section != XyceDeckSection::Netlists {
                return Err(format!(
                    "recognized {LABEL} record '{}' is not classified as a Netlists deck",
                    deck.relative_path
                ));
            }
            if Self::normalize_manifest_key(&self.relative_key(&deck.path)) != relative {
                return Err(format!(
                    "recognized {LABEL} record '{}' does not match its physical deck path",
                    deck.relative_path
                ));
            }

            let parent = deck
                .path
                .parent()
                .ok_or_else(|| format!("recognized {LABEL} record has no sibling directory"))?;
            let owner_path = parent.join("params_a.cir");
            let literal_baseline_path = parent.join("params_a0.cir");
            let parameterized_member_path = parent.join("params_a1.cir");
            let expected_target = match role {
                XyceParams1Role::WrapperOwner => &owner_path,
                XyceParams1Role::LiteralBaseline => &literal_baseline_path,
                XyceParams1Role::ParameterizedMember => &parameterized_member_path,
            };
            if !Self::same_path(&deck.path, expected_target) {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its exact canonical path"
                ));
            }

            let target_path = match role {
                XyceParams1Role::WrapperOwner => None,
                XyceParams1Role::LiteralBaseline => Some(literal_baseline_path.clone()),
                XyceParams1Role::ParameterizedMember => Some(parameterized_member_path.clone()),
            };
            let contract = XyceParams1FamilyContract {
                relational: XyceBaselineFamilyContract {
                    kind: XyceBaselineFamilyKind::Params1,
                    comparison: XyceBaselineFamilyComparison::TolerancedStrict,
                    family: "params_a".to_string(),
                    baseline_path: literal_baseline_path.clone(),
                    member_paths: vec![literal_baseline_path, parameterized_member_path],
                    target_path,
                },
                owner_path,
                role,
            };
            self.validate_params1_provenance(&contract)?;
            Ok(contract)
        })())
    }

    pub(super) fn validate_params1_provenance(
        &self,
        contract: &XyceParams1FamilyContract,
    ) -> Result<(), String> {
        const LABEL: &str = "PARAMS1 parameter-equivalence family";
        Self::validate_params1_historical_oracle_provenance()?;
        let parent = contract
            .owner_path
            .parent()
            .ok_or_else(|| format!("{LABEL} owner has no parent directory"))?;
        let parent_metadata = fs::symlink_metadata(parent)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if parent_metadata.file_type().is_symlink() || !parent_metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} directory '{}' must be a regular non-symlink directory",
                self.display_path(parent)
            ));
        }

        let literal_baseline_path = parent.join("params_a0.cir");
        let parameterized_member_path = parent.join("params_a1.cir");
        let expected_member_paths = [&literal_baseline_path, &parameterized_member_path];
        let member_paths_match = contract.relational.member_paths.len()
            == expected_member_paths.len()
            && contract
                .relational
                .member_paths
                .iter()
                .zip(expected_member_paths)
                .all(|(actual, expected)| Self::same_path(actual, expected));
        let target_matches = match (contract.relational.target_path.as_ref(), contract.role) {
            (None, XyceParams1Role::WrapperOwner) => true,
            (Some(actual), XyceParams1Role::LiteralBaseline) => {
                Self::same_path(actual, &literal_baseline_path)
            }
            (Some(actual), XyceParams1Role::ParameterizedMember) => {
                Self::same_path(actual, &parameterized_member_path)
            }
            _ => false,
        };
        if contract.relational.kind != XyceBaselineFamilyKind::Params1
            || contract.relational.comparison != XyceBaselineFamilyComparison::TolerancedStrict
            || contract.relational.family != "params_a"
            || !Self::same_path(&contract.relational.baseline_path, &literal_baseline_path)
            || !member_paths_match
            || !target_matches
        {
            return Err(format!(
                "{LABEL} contract is not the exact toleranced-strict a0/a1 relational contract"
            ));
        }

        let expected_records = [
            (&contract.owner_path, XYCE_PARAMS1_OWNER_RECORD),
            (&literal_baseline_path, XYCE_PARAMS1_LITERAL_BASELINE_RECORD),
            (
                &parameterized_member_path,
                XYCE_PARAMS1_PARAMETERIZED_MEMBER_RECORD,
            ),
        ];
        for (path, expected_record) in expected_records {
            if Self::normalize_manifest_key(&self.relative_key(path)) != expected_record {
                return Err(format!(
                    "{LABEL} path '{}' is not canonical record '{expected_record}'",
                    self.display_path(path)
                ));
            }
        }

        let expected_role_record = match contract.role {
            XyceParams1Role::WrapperOwner => XYCE_PARAMS1_OWNER_RECORD,
            XyceParams1Role::LiteralBaseline => XYCE_PARAMS1_LITERAL_BASELINE_RECORD,
            XyceParams1Role::ParameterizedMember => XYCE_PARAMS1_PARAMETERIZED_MEMBER_RECORD,
        };
        let actual_role_record = match contract.relational.target_path.as_ref() {
            Some(path) => Self::normalize_manifest_key(&self.relative_key(path)),
            None => Self::normalize_manifest_key(&self.relative_key(&contract.owner_path)),
        };
        if actual_role_record != expected_role_record {
            return Err(format!(
                "{LABEL} role {:?} is not bound to its exact canonical record",
                contract.role
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        let current_wrapper_rows = self.params1_current_wrapper_manifest_rows()?;
        let current_wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        let entries = fs::read_dir(parent)
            .map_err(|error| format!("failed to read {LABEL} directory: {error}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?;

        let mut directory_names = BTreeMap::<String, PathBuf>::new();
        let mut candidate_paths = BTreeMap::<String, PathBuf>::new();
        for entry in &entries {
            let path = entry.path();
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} directory contains a non-UTF-8 entry"))?
                .to_string();
            let normalized_name = name.to_ascii_lowercase();
            if directory_names
                .insert(normalized_name.clone(), path.clone())
                .is_some()
            {
                return Err(format!(
                    "{LABEL} directory contains a case-colliding entry '{name}'"
                ));
            }
            if !path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
            {
                continue;
            }
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {LABEL} candidate: {error}"))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} candidate '{}' must be a regular non-symlink file",
                    self.display_path(&path)
                ));
            }
            let relative = self.relative_key(&path);
            let key = Self::normalize_manifest_key(&relative);
            if candidate_paths.insert(key, path).is_some() {
                return Err(format!(
                    "{LABEL} candidate census contains a normalized-path collision"
                ));
            }
        }

        let expected_candidate_keys = BTreeSet::from([
            XYCE_PARAMS1_OWNER_RECORD.to_string(),
            XYCE_PARAMS1_LITERAL_BASELINE_RECORD.to_string(),
            XYCE_PARAMS1_PARAMETERIZED_MEMBER_RECORD.to_string(),
        ]);
        let actual_candidate_keys = candidate_paths.keys().cloned().collect::<BTreeSet<_>>();
        if actual_candidate_keys != expected_candidate_keys {
            return Err(format!(
                "{LABEL} must contain exactly its three canonical .cir records, got {actual_candidate_keys:?}"
            ));
        }

        let mut candidates = Vec::with_capacity(candidate_paths.len());
        let mut candidate_content = Vec::with_capacity(candidate_paths.len());
        let owner_manifest_rows = current_wrapper_rows;
        let mut historical_exclusion_rows = Vec::new();
        for (key, path) in &candidate_paths {
            let relative = self.relative_key(path);
            let role = XyceParams1Role::for_record(&relative)
                .ok_or_else(|| format!("{LABEL} candidate role became ambiguous"))?;
            let bytes = fs::read(path).map_err(|error| {
                format!("failed to read {LABEL} candidate '{relative}': {error}")
            })?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            if (role == XyceParams1Role::WrapperOwner) != canonical.is_empty() {
                return Err(format!(
                    "{LABEL} owner must be exactly empty and both executable members must be nonempty"
                ));
            }
            let content_hash = blake3::hash(&canonical).to_hex().to_string();
            let expected_content_hash = match role {
                XyceParams1Role::WrapperOwner => XYCE_PARAMS1_OWNER_CONTENT_BLAKE3,
                XyceParams1Role::LiteralBaseline => XYCE_PARAMS1_LITERAL_BASELINE_CONTENT_BLAKE3,
                XyceParams1Role::ParameterizedMember => {
                    XYCE_PARAMS1_PARAMETERIZED_MEMBER_CONTENT_BLAKE3
                }
            };
            if content_hash != expected_content_hash {
                return Err(format!(
                    "{LABEL} candidate '{relative}' canonical source identity changed: expected {expected_content_hash}, got {content_hash}"
                ));
            }
            candidates.push(relative.clone());
            candidate_content.push(format!("{relative}\t{content_hash}"));

            match role {
                XyceParams1Role::WrapperOwner => {
                    if !self.requires_upstream_wrapper(&relative)
                        || !current_wrapper_records.contains(key)
                        || exclusions.contains_key(key)
                    {
                        return Err(format!(
                            "{LABEL} owner '{relative}' lost its exclusive wrapper provenance"
                        ));
                    }
                }
                XyceParams1Role::LiteralBaseline | XyceParams1Role::ParameterizedMember => {
                    if self.requires_upstream_wrapper(&relative)
                        || current_wrapper_records.contains(key)
                    {
                        return Err(format!(
                            "{LABEL} executable member '{relative}' must not own the wrapper"
                        ));
                    }
                    let exclusion = exclusions.get(key).ok_or_else(|| {
                        format!(
                            "{LABEL} executable member '{relative}' lost its historical exclusion provenance"
                        )
                    })?;
                    if !matches!(
                        &exclusion.disposition,
                        XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified {
                            expected_contract,
                        } if expected_contract == role.result_contract()
                    ) {
                        return Err(format!(
                            "{LABEL} executable member '{relative}' does not carry its exact independent qualification contract"
                        ));
                    }
                    historical_exclusion_rows.push(format!(
                        "{relative}\t{}\t{UPSTREAM_EXCLUDED_DISPOSITION}",
                        exclusion.source
                    ));
                }
            }

            self.reject_wrapper_output_artifacts(path)
                .map_err(|error| format!("{LABEL} candidate '{relative}' {error}"))?;
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("{LABEL} candidate filename is not UTF-8"))?;
            for suffix in ["prn", "res", "prn.gs", "res.gs", "csv", "csd"] {
                let sidecar_name = format!("{file_name}.{suffix}").to_ascii_lowercase();
                if let Some(sidecar) = directory_names.get(&sidecar_name) {
                    return Err(format!(
                        "{LABEL} candidate must not have source-side output artifact '{}'",
                        self.display_path(sidecar)
                    ));
                }
            }
        }

        candidates.sort();
        candidate_content.sort();
        historical_exclusion_rows.sort();
        let candidate_hash = blake3::hash(candidates.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let content_hash = blake3::hash(candidate_content.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let owner_hash = blake3::hash(owner_manifest_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        let exclusion_hash = blake3::hash(historical_exclusion_rows.join("\n").as_bytes())
            .to_hex()
            .to_string();
        if candidates.len() != XYCE_PARAMS1_CANDIDATE_COUNT
            || candidate_hash != XYCE_PARAMS1_CANDIDATE_BLAKE3
            || content_hash != XYCE_PARAMS1_CANDIDATE_CONTENT_BLAKE3
            || owner_manifest_rows.len() != XYCE_PARAMS1_OWNER_COUNT
            || owner_hash != XYCE_PARAMS1_OWNER_MANIFEST_BLAKE3
            || historical_exclusion_rows.len() != XYCE_PARAMS1_EXCLUSION_COUNT
            || exclusion_hash != XYCE_PARAMS1_HISTORICAL_EXCLUSION_BLAKE3
        {
            return Err(format!(
                "{LABEL} provenance changed: candidates={}/{candidate_hash}/{content_hash}, owners={}/{owner_hash}, exclusions={}/{exclusion_hash}",
                candidates.len(),
                owner_manifest_rows.len(),
                historical_exclusion_rows.len()
            ));
        }
        Ok(())
    }

    pub(super) fn param_expression_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/certification_tests/") {
            return None;
        }

        let file_name = deck.path.file_name()?.to_str()?;
        let parent = deck.path.parent()?;
        if self.requires_upstream_wrapper(&deck.relative_path)
            && fs::metadata(&deck.path)
                .ok()
                .is_some_and(|metadata| metadata.len() == 0)
        {
            let family = file_name.strip_suffix(".cir")?;
            if family.is_empty()
                || !family
                    .chars()
                    .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
            {
                return None;
            }
            return self.param_expression_family_contract_for(parent, family, None);
        }

        let file_name_lower = file_name.to_ascii_lowercase();
        let suffix = if file_name_lower.ends_with("_2a.cir") {
            "_2a.cir"
        } else if file_name_lower.ends_with("_2.cir") {
            "_2.cir"
        } else {
            return None;
        };
        let family = file_name.get(..file_name.len().checked_sub(suffix.len())?)?;
        if family.is_empty()
            || !family
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || ch == '_')
        {
            return None;
        }
        self.param_expression_family_contract_for(parent, family, Some(deck.path.clone()))
    }

    pub(super) fn param_expression_family_contract_for(
        &self,
        parent: &Path,
        family: &str,
        target_path: Option<PathBuf>,
    ) -> Option<XyceBaselineFamilyContract> {
        let owner_path = parent.join(format!("{family}.cir"));
        let baseline_path = parent.join(format!("{family}_2.cir"));
        let literal_path = parent.join(format!("{family}_2a.cir"));
        let cir_stems = fs::read_dir(parent)
            .ok()?
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                if !path.is_file()
                    || !path
                        .extension()
                        .and_then(|extension| extension.to_str())
                        .is_some_and(|extension| extension.eq_ignore_ascii_case("cir"))
                {
                    return None;
                }
                path.file_stem()
                    .and_then(|stem| stem.to_str())
                    .map(str::to_ascii_lowercase)
            })
            .collect::<BTreeSet<_>>();
        let family_lower = family.to_ascii_lowercase();
        let required_stems = [
            family_lower.clone(),
            format!("{family_lower}_2"),
            format!("{family_lower}_2a"),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if cir_stems != required_stems
            || !owner_path.is_file()
            || !baseline_path.is_file()
            || !literal_path.is_file()
            || fs::metadata(&owner_path)
                .ok()
                .is_none_or(|metadata| metadata.len() != 0)
            || fs::metadata(&baseline_path)
                .ok()
                .is_none_or(|metadata| metadata.len() == 0)
            || fs::metadata(&literal_path)
                .ok()
                .is_none_or(|metadata| metadata.len() == 0)
        {
            return None;
        }

        let owner_relative = self.relative_key(&owner_path);
        let baseline_relative = self.relative_key(&baseline_path);
        let literal_relative = self.relative_key(&literal_path);
        if !self.requires_upstream_wrapper(&owner_relative)
            || self.requires_upstream_wrapper(&baseline_relative)
            || self.requires_upstream_wrapper(&literal_relative)
        {
            return None;
        }
        if let Some(target_path) = target_path.as_ref()
            && !Self::same_path(target_path, &baseline_path)
            && !Self::same_path(target_path, &literal_path)
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::ParamExpression,
            comparison: XyceBaselineFamilyComparison::TolerancedStrict,
            family: family.to_string(),
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, literal_path],
            target_path,
        })
    }

    pub(super) fn subckt_parameter_precedence_family_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<XyceBaselineFamilyContract> {
        let relative_path = Self::normalize_manifest_key(&deck.relative_path);
        if !relative_path.starts_with("netlists/") {
            return None;
        }
        let file_name = deck.path.file_name()?.to_str()?;
        let stem = file_name.strip_suffix(".cir")?;
        let parent = deck.path.parent()?;
        let (owner_name, baseline_name) = if stem
            .get(stem.len().saturating_sub(3)..)
            .is_some_and(|suffix| suffix.eq_ignore_ascii_case("ref"))
        {
            (
                format!("{}.cir", &stem[..stem.len() - 3]),
                file_name.to_string(),
            )
        } else {
            (file_name.to_string(), format!("{stem}Ref.cir"))
        };
        let owner_path = parent.join(&owner_name);
        let baseline_path = parent.join(&baseline_name);
        if !owner_path.is_file()
            || !baseline_path.is_file()
            || (!Self::same_path(&deck.path, &owner_path)
                && !Self::same_path(&deck.path, &baseline_path))
        {
            return None;
        }
        let owner_relative = self.relative_key(&owner_path);
        let baseline_relative = self.relative_key(&baseline_path);
        if !self.requires_upstream_wrapper(&owner_relative)
            || self.requires_upstream_wrapper(&baseline_relative)
            || self
                .static_prn_reference_path(&owner_path)
                .is_some_and(|path| path.is_file())
            || self
                .static_prn_reference_path(&baseline_path)
                .is_some_and(|path| path.is_file())
        {
            return None;
        }

        let owner_plan = self
            .static_dc_plan_for_path(&owner_path, ExpressionDialect::Xyce)
            .ok()?;
        let baseline_plan = self
            .static_dc_plan_for_path(&baseline_path, ExpressionDialect::Xyce)
            .ok()?;
        Self::validate_subckt_parameter_precedence_dc_plan(&owner_plan).ok()?;
        Self::validate_subckt_parameter_precedence_dc_plan(&baseline_plan).ok()?;
        if owner_plan.print.probes != baseline_plan.print.probes
            || !Self::dc_sweeps_match_exactly(&owner_plan.dc, &baseline_plan.dc)
        {
            return None;
        }

        let owner_netlist = Self::parse_xyce_netlist(&owner_plan.source, &owner_path).ok()?;
        let mut parameterized_instances_by_definition = BTreeMap::<String, usize>::new();
        for element in owner_netlist.elements.iter().chain(
            owner_netlist
                .subcircuits
                .iter()
                .flat_map(|subcircuit| subcircuit.elements.iter()),
        ) {
            if let ElementKind::Subcircuit {
                subckt_name,
                params,
            } = &element.kind
                && !params.is_empty()
            {
                *parameterized_instances_by_definition
                    .entry(subckt_name.to_ascii_uppercase())
                    .or_default() += 1;
            }
        }
        if !parameterized_instances_by_definition
            .values()
            .any(|count| *count >= 2)
        {
            return None;
        }

        Some(XyceBaselineFamilyContract {
            kind: XyceBaselineFamilyKind::SubcktParameterPrecedence,
            comparison: XyceBaselineFamilyComparison::Exact,
            family: owner_name.trim_end_matches(".cir").to_string(),
            baseline_path: baseline_path.clone(),
            member_paths: vec![baseline_path, owner_path],
            target_path: Some(deck.path.clone()),
        })
    }
}
