use super::*;

impl XyceTestRunner {
    pub(super) fn bug402_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG402_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG402_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG402_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug402_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug402_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG402_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG402_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG402_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG402_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_402_SON Release-7.10 family/wrapper/Tools/xyce_verify provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        if UPSTREAM_EXCLUSIONS_SOURCE_COMMIT != XYCE_BUG402_PRETRIM_COMMIT
            || UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE
                != "3e34bfaafa890cb2e4457137b6a0e325c8c1e87d"
        {
            return Err("BUG_402_SON pre-trim manifest commit/tree provenance changed".into());
        }
        Ok(())
    }

    pub(super) fn bug402_temperature_option_contract(
        &self,
        deck: &XyceDeck,
    ) -> Option<Result<XyceBug402TemperatureContract, String>> {
        let role = XyceBug402TemperatureRole::for_record(&deck.relative_path)?;
        Some((|| {
            const LABEL: &str = "BUG_402_SON temperature-option scope family";
            if deck.section != XyceDeckSection::Netlists
                || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
                || !Self::same_path(&deck.path, &self.root.join(role.path()))
            {
                return Err(format!(
                    "recognized {LABEL} role {role:?} is not backed by its exact canonical Netlists path"
                ));
            }

            self.validate_bug402_temperature_option_provenance()?;

            let xyce_reference_path = self.root.join(XYCE_BUG402_XYCE_REFERENCE_PATH);
            let spice_member_path = self.root.join(XYCE_BUG402_SPICE_MEMBER_PATH);
            let xyce_reference_plan =
                self.static_dc_plan_for_path(&xyce_reference_path, ExpressionDialect::Xyce)?;
            let spice_member_plan =
                self.static_dc_plan_for_path(&spice_member_path, ExpressionDialect::Xyce)?;
            let xyce_netlist =
                Self::parse_xyce_netlist(&xyce_reference_plan.source, &xyce_reference_path)
                    .map_err(|error| {
                        format!("canonical DEVICE TEMP member parse failed: {error}")
                    })?;
            let spice_netlist =
                Self::parse_xyce_netlist(&spice_member_plan.source, &spice_member_path)
                    .map_err(|error| format!("legacy SPICE TEMP member parse failed: {error}"))?;
            let xyce_snapshot =
                Self::bug402_temperature_snapshot(&xyce_reference_plan, &xyce_netlist)?;
            let spice_snapshot =
                Self::bug402_temperature_snapshot(&spice_member_plan, &spice_netlist)?;
            if xyce_snapshot != spice_snapshot {
                return Err(format!(
                    "{LABEL} parsed or flattened semantics differ after TEMP-scope normalization: canonical={xyce_snapshot:?}, legacy={spice_snapshot:?}"
                ));
            }

            Ok(XyceBug402TemperatureContract {
                xyce_reference_plan,
                spice_member_plan,
                role,
            })
        })())
    }

    pub(super) fn validate_bug402_temperature_option_provenance(&self) -> Result<(), String> {
        const LABEL: &str = "BUG_402_SON temperature-option scope family";
        Self::validate_bug402_historical_oracle_provenance()?;

        let family_dir = self.root.join("Netlists/Certification_Tests/BUG_402_SON");
        let family_metadata = fs::symlink_metadata(&family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} directory: {error}"))?;
        if family_metadata.file_type().is_symlink() || !family_metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} requires a physical, non-symlinked family directory"
            ));
        }

        let expected_artifacts = XYCE_BUG402_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut physical_names = Vec::new();
        let mut retained_records = Vec::new();
        for entry in fs::read_dir(&family_dir)
            .map_err(|error| format!("failed to enumerate {LABEL}: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to read {LABEL} entry: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path).map_err(|error| {
                format!(
                    "failed to inspect {LABEL} entry '{}': {error}",
                    path.display()
                )
            })?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} entry '{}' must be a regular non-symlink file",
                    path.display()
                ));
            }
            let file_name = path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| format!("{LABEL} contains a non-UTF-8 file name"))?;
            let key = file_name.to_ascii_lowercase();
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected_artifacts.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} complete physical census has unexpected file '{file_name}'"
                ));
            };
            if file_name != expected_name {
                return Err(format!(
                    "{LABEL} retained artifact spelling changed: expected '{expected_name}', got '{file_name}'"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} artifact: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} retained artifact '{file_name}' identity changed: bytes={}/{expected_bytes}, sha256={sha256}, blake3={content_blake3}",
                    canonical.len()
                ));
            }
            physical_names.push(key);
            retained_records.push(format!(
                "{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
        }
        physical_names.sort();
        retained_records.sort_by_key(|record| record.to_ascii_lowercase());
        let expected_names = expected_artifacts.keys().cloned().collect::<Vec<_>>();
        let retained_stream = retained_records.join("\n");
        let retained_sha256 = format!("{:x}", Sha256::digest(retained_stream.as_bytes()));
        let retained_blake3 = blake3::hash(retained_stream.as_bytes())
            .to_hex()
            .to_string();
        if physical_names != expected_names
            || retained_records.len() != XYCE_BUG402_RETAINED_RECORD_COUNT
            || retained_stream.len() != XYCE_BUG402_RETAINED_RECORD_BYTES
            || retained_sha256 != XYCE_BUG402_RETAINED_RECORDS_SHA256
            || retained_blake3 != XYCE_BUG402_RETAINED_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} retained physical/content census changed: records={}/{}, sha256={retained_sha256}, blake3={retained_blake3}, files={physical_names:?}",
                retained_records.len(),
                retained_stream.len()
            ));
        }

        let owner_path = self.root.join(XYCE_BUG402_OWNER_PATH);
        let xyce_reference_path = self.root.join(XYCE_BUG402_XYCE_REFERENCE_PATH);
        let spice_member_path = self.root.join(XYCE_BUG402_SPICE_MEMBER_PATH);
        let wrapper_records = Self::load_upstream_wrapper_decks(&self.root);
        if !self.requires_upstream_wrapper(XYCE_BUG402_OWNER_RECORD)
            || !wrapper_records.contains(XYCE_BUG402_OWNER_RECORD)
            || self.requires_upstream_wrapper(XYCE_BUG402_XYCE_REFERENCE_RECORD)
            || self.requires_upstream_wrapper(XYCE_BUG402_SPICE_MEMBER_RECORD)
            || wrapper_records.contains(XYCE_BUG402_XYCE_REFERENCE_RECORD)
            || wrapper_records.contains(XYCE_BUG402_SPICE_MEMBER_RECORD)
        {
            return Err(format!(
                "{LABEL} requires exactly bug402son.cir to own the removed upstream wrapper"
            ));
        }

        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG402_OWNER_RECORD) {
            return Err(format!(
                "{LABEL} placeholder owner must not acquire an upstream-exclusion row"
            ));
        }
        for role in [
            XyceBug402TemperatureRole::XyceDeviceReference,
            XyceBug402TemperatureRole::SpiceCompatibilityMember,
        ] {
            let exclusion = exclusions.get(role.record()).ok_or_else(|| {
                format!(
                    "{LABEL} worker '{}' lost exclusion provenance",
                    role.record()
                )
            })?;
            if exclusion.source != XYCE_BUG402_HISTORICAL_EXCLUDE_PATH
                || !matches!(
                    &exclusion.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == role.result_contract()
                )
            {
                return Err(format!(
                    "{LABEL} worker '{}' lacks its exact independent qualification contract",
                    role.record()
                ));
            }
        }

        for path in [&owner_path, &xyce_reference_path, &spice_member_path] {
            self.reject_wrapper_output_artifacts(path)
                .map_err(|error| format!("{LABEL} '{}' {error}", path.display()))?;
        }
        let owner_source = fs::read_to_string(&owner_path)
            .map_err(|error| format!("failed to read {LABEL} owner: {error}"))?;
        if owner_source
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()
            != 4
            || owner_source
                .lines()
                .filter(|line| !line.trim().is_empty())
                .any(|line| !line.trim_start().starts_with('*'))
        {
            return Err(format!(
                "{LABEL} bug402son.cir must remain the four-comment, non-simulated wrapper placeholder"
            ));
        }

        let xyce_source = fs::read_to_string(&xyce_reference_path)
            .map_err(|error| format!("failed to read {LABEL} canonical member: {error}"))?;
        let spice_source = fs::read_to_string(&spice_member_path)
            .map_err(|error| format!("failed to read {LABEL} legacy member: {error}"))?;
        Self::validate_bug402_temperature_source_pair(&xyce_source, &spice_source)
    }

    pub(super) fn validate_bug402_temperature_source_pair(
        xyce_source: &str,
        spice_source: &str,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_402_SON temperature-option scope family";
        const XYCE_CARD: &str = ".OPTIONS DEVICE TEMP=35";
        const SPICE_CARD: &str = ".OPTIONS  TEMP=35";
        const NORMALIZED_CARD: &str = ".OPTIONS <TEMP-SCOPE> TEMP=35";

        if xyce_source.matches(XYCE_CARD).count() != 1
            || xyce_source.contains(SPICE_CARD)
            || spice_source.matches(SPICE_CARD).count() != 1
            || spice_source.contains(XYCE_CARD)
        {
            return Err(format!(
                "{LABEL} requires one canonical DEVICE TEMP card and one legacy unscoped TEMP card"
            ));
        }
        let normalized_xyce = xyce_source.replacen(XYCE_CARD, NORMALIZED_CARD, 1);
        let normalized_spice = spice_source.replacen(SPICE_CARD, NORMALIZED_CARD, 1);
        if normalized_xyce != normalized_spice {
            return Err(format!(
                "{LABEL} worker sources differ outside the single TEMP package selector"
            ));
        }
        Ok(())
    }

    fn bug402_temperature_snapshot(
        plan: &XyceStaticDcPlan,
        netlist: &Netlist,
    ) -> Result<XyceBug402TemperatureSnapshot, String> {
        const LABEL: &str = "BUG_402_SON temperature-option scope family";
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.start.to_bits() != 0.2f64.to_bits()
            || plan.dc.stop.to_bits() != 1.001f64.to_bits()
            || plan.dc.step.to_bits() != 0.016f64.to_bits()
            || probes != ["v(20)", "i(vmon)"]
        {
            return Err(format!(
                "{LABEL} requires the exact VIN 0.2:0.016:1.001 sweep and V(20), I(VMON) print plan"
            ));
        }
        let grid = plan.dc.primary_spec().points();
        if grid.len() != XYCE_BUG402_DC_POINT_COUNT
            || grid
                .first()
                .is_none_or(|value| (value - 0.2).abs() > 64.0 * f64::EPSILON)
            || grid
                .last()
                .is_none_or(|value| (value - 1.0).abs() > 64.0 * f64::EPSILON)
        {
            return Err(format!(
                "{LABEL} DC grid changed: expected 51 points from 0.2 through 1.0, got {}",
                grid.len()
            ));
        }
        if netlist.options.temp.map(Value::to_bits) != Some(35.0f64.to_bits())
            || netlist.options.gmin.map(Value::to_bits) != Some(1.0e-15f64.to_bits())
            || netlist.options.tnom.is_some()
            || netlist.analyses.len() != 1
            || !matches!(netlist.analyses[0], AnalysisCommand::Dc { .. })
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
        {
            return Err(format!(
                "{LABEL} must resolve one 35 C global device temperature, GMIN=1e-15, default TNOM, and one ordinary DC analysis"
            ));
        }
        let mut probe_index = Self::dc_probe_index(netlist);
        for probe in &plan.print.probes {
            Self::validate_dc_probe_with_index(probe, netlist, &mut probe_index)
                .map_err(|error| format!("{LABEL} probe '{probe}' is not executable: {error}"))?;
        }
        validate_output_symbols(netlist)
            .map_err(|error| format!("{LABEL} has an unresolved output dependency: {error}"))?;

        let top_level_elements = Self::bug402_element_fingerprints(&netlist.elements)?;
        if top_level_elements
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            != ["vin", "vmon", "x1"]
        {
            return Err(format!(
                "{LABEL} top level requires exactly VIN, VMON, and X1, got {:?}",
                top_level_elements.keys().collect::<Vec<_>>()
            ));
        }
        let [subcircuit] = netlist.subcircuits.as_slice() else {
            return Err(format!("{LABEL} requires exactly one MMSZ5229 subcircuit"));
        };
        if !subcircuit.name.eq_ignore_ascii_case("MMSZ5229")
            || subcircuit.ports != ["1", "2"]
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || !subcircuit.nested_subcircuits.is_empty()
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
        {
            return Err(format!("{LABEL} MMSZ5229 hierarchy or local state changed"));
        }
        let subcircuit_elements = Self::bug402_element_fingerprints(&subcircuit.elements)?;
        if subcircuit_elements
            .keys()
            .map(String::as_str)
            .collect::<Vec<_>>()
            != ["d1", "d2", "d3", "d4", "ic", "rc"]
        {
            return Err(format!(
                "{LABEL} subcircuit requires exactly D1-D4, IC, and RC"
            ));
        }

        let flattened = flatten_netlist_with_models(netlist)
            .map_err(|error| format!("{LABEL} flattening failed: {error}"))?;
        if !flattened.scoped_initial_conditions.is_empty()
            || !flattened.scoped_node_sets.is_empty()
            || !flattened.scoped_startup_directives.is_empty()
            || !flattened.xspice_auto_bridge_node_hints.is_empty()
        {
            return Err(format!(
                "{LABEL} flattening introduced startup state or XSPICE bridge hints"
            ));
        }
        let flattened_elements = Self::bug402_element_fingerprints(&flattened.elements)?;
        if flattened_elements.len() != 8 {
            return Err(format!(
                "{LABEL} flattening must produce two top-level sources and six subcircuit devices, got {}",
                flattened_elements.len()
            ));
        }
        let models = Self::bug402_model_fingerprints(&netlist.models)?;
        let flattened_models = Self::bug402_model_fingerprints(&flattened.scoped_models)?;
        if models.len() != 5 || !flattened_models.is_empty() {
            return Err(format!(
                "{LABEL} requires five globally qualified models and no parameter-derived instance-scoped models, got {}/{}",
                models.len(),
                flattened_models.len()
            ));
        }
        let diode_model_count = models
            .values()
            .filter(|model| model.model_type == "d")
            .count();
        let resistor_model_count = models
            .values()
            .filter(|model| model.model_type == "r")
            .count();
        if diode_model_count != 4 || resistor_model_count != 1 {
            return Err(format!(
                "{LABEL} requires four diode models and one resistor temperature model"
            ));
        }

        Ok(XyceBug402TemperatureSnapshot {
            temperature_bits: 35.0f64.to_bits(),
            gmin_bits: 1.0e-15f64.to_bits(),
            sweep_source: plan.dc.source.to_ascii_lowercase(),
            sweep_start_bits: plan.dc.start.to_bits(),
            sweep_stop_bits: plan.dc.stop.to_bits(),
            sweep_step_bits: plan.dc.step.to_bits(),
            probes,
            top_level_elements,
            subcircuit_name: subcircuit.name.to_ascii_lowercase(),
            subcircuit_ports: subcircuit
                .ports
                .iter()
                .map(|port| port.to_ascii_lowercase())
                .collect(),
            subcircuit_elements,
            flattened_elements,
            models,
            flattened_models,
        })
    }

    fn bug402_element_fingerprints(
        elements: &[rspice_core::netlist::Element],
    ) -> Result<BTreeMap<String, XyceRelationalElementFingerprint>, String> {
        let mut fingerprints = BTreeMap::new();
        for element in elements {
            if !matches!(element.provenance, ElementProvenance::Authored) {
                return Err(format!(
                    "BUG_402_SON contains generated element '{}' in its authored topology",
                    element.name
                ));
            }
            let name = element.name.trim().to_ascii_lowercase();
            let nodes = element
                .nodes
                .iter()
                .map(|node| node.trim().to_ascii_lowercase())
                .collect::<Vec<_>>();
            let fingerprint = match &element.kind {
                ElementKind::VoltageSource(source) => {
                    let (waveform, numeric_bits) = Self::scoped_model_source_fingerprint(source)?;
                    XyceRelationalElementFingerprint {
                        kind: format!("V:{waveform}"),
                        nodes,
                        numeric_bits,
                        text: Vec::new(),
                    }
                }
                ElementKind::CurrentSource(source) => {
                    let (waveform, numeric_bits) = Self::scoped_model_source_fingerprint(source)?;
                    XyceRelationalElementFingerprint {
                        kind: format!("I:{waveform}"),
                        nodes,
                        numeric_bits,
                        text: Vec::new(),
                    }
                }
                ElementKind::VoltageSourceDeferred(expression) => {
                    XyceRelationalElementFingerprint {
                        kind: "V:DEFERRED".into(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec![expression.trim().to_ascii_lowercase()],
                    }
                }
                ElementKind::CurrentSourceDeferred(expression) => {
                    XyceRelationalElementFingerprint {
                        kind: "I:DEFERRED".into(),
                        nodes,
                        numeric_bits: Vec::new(),
                        text: vec![expression.trim().to_ascii_lowercase()],
                    }
                }
                ElementKind::Resistor {
                    value,
                    value_expr,
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    let params = Self::bug402_numeric_params(instance_params)?;
                    let mut deferred = deferred_params
                        .iter()
                        .map(|(key, value)| {
                            format!(
                                "{}={}",
                                key.trim().to_ascii_lowercase(),
                                value.trim().to_ascii_lowercase()
                            )
                        })
                        .collect::<Vec<_>>();
                    deferred.sort();
                    XyceRelationalElementFingerprint {
                        kind: "R".into(),
                        nodes,
                        numeric_bits: std::iter::once(value.to_bits())
                            .chain(params.iter().map(|(_, bits)| *bits))
                            .collect(),
                        text: std::iter::once(
                            model.as_deref().unwrap_or("").trim().to_ascii_lowercase(),
                        )
                        .chain(std::iter::once(
                            value_expr
                                .as_deref()
                                .unwrap_or("")
                                .trim()
                                .to_ascii_lowercase(),
                        ))
                        .chain(params.into_iter().map(|(key, _)| key))
                        .chain(deferred)
                        .collect(),
                    }
                }
                ElementKind::Diode {
                    model,
                    instance_params,
                    deferred_params,
                } => {
                    let params = Self::bug402_numeric_params(instance_params)?;
                    let mut deferred = deferred_params
                        .iter()
                        .map(|(key, value)| {
                            format!(
                                "{}={}",
                                key.trim().to_ascii_lowercase(),
                                value.trim().to_ascii_lowercase()
                            )
                        })
                        .collect::<Vec<_>>();
                    deferred.sort();
                    XyceRelationalElementFingerprint {
                        kind: "D".into(),
                        nodes,
                        numeric_bits: params.iter().map(|(_, bits)| *bits).collect(),
                        text: std::iter::once(model.trim().to_ascii_lowercase())
                            .chain(params.into_iter().map(|(key, _)| key))
                            .chain(deferred)
                            .collect(),
                    }
                }
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } if params.is_empty() => XyceRelationalElementFingerprint {
                    kind: "X".into(),
                    nodes,
                    numeric_bits: Vec::new(),
                    text: vec![subckt_name.trim().to_ascii_lowercase()],
                },
                _ => {
                    return Err(format!(
                        "BUG_402_SON element '{}' is outside the exact V/I/R/D/X topology envelope",
                        element.name
                    ));
                }
            };
            if name.is_empty() || fingerprints.insert(name, fingerprint).is_some() {
                return Err("BUG_402_SON contains an empty or duplicate element name".into());
            }
        }
        Ok(fingerprints)
    }

    fn bug402_numeric_params(params: &[(String, Value)]) -> Result<Vec<(String, u64)>, String> {
        let mut normalized = params
            .iter()
            .map(|(key, value)| {
                if !value.is_finite() {
                    return Err(format!(
                        "BUG_402_SON parameter {key} has non-finite value {value}"
                    ));
                }
                Ok((key.trim().to_ascii_lowercase(), value.to_bits()))
            })
            .collect::<Result<Vec<_>, String>>()?;
        normalized.sort();
        if normalized.windows(2).any(|pair| pair[0].0 == pair[1].0) {
            return Err("BUG_402_SON contains duplicate numeric parameters".into());
        }
        Ok(normalized)
    }

    fn bug402_model_fingerprints(
        models: &[rspice_core::netlist::ModelDef],
    ) -> Result<BTreeMap<String, XyceBug402ModelFingerprint>, String> {
        let mut fingerprints = BTreeMap::new();
        for model in models {
            if !model.expr_params.is_empty()
                || !model.string_params.is_empty()
                || !model.string_vector_params.is_empty()
                || !model.real_vector_params.is_empty()
                || !model.real_vector_expr_params.is_empty()
                || !model.integer_vector_params.is_empty()
            {
                return Err(format!(
                    "BUG_402_SON model '{}' contains non-scalar-numeric parameters",
                    model.name
                ));
            }
            let name = model.name.trim().to_ascii_lowercase();
            let fingerprint = XyceBug402ModelFingerprint {
                model_type: model.model_type.trim().to_ascii_lowercase(),
                numeric_params: Self::bug402_numeric_params(&model.params)?,
            };
            if name.is_empty() || fingerprints.insert(name, fingerprint).is_some() {
                return Err("BUG_402_SON contains an empty or duplicate model name".into());
            }
        }
        Ok(fingerprints)
    }

    pub(super) fn run_bug402_temperature_option_contract(
        &self,
        deck: &XyceDeck,
        contract: XyceBug402TemperatureContract,
        start: Instant,
    ) -> XyceTestResult {
        let result_contract = contract.role.result_contract();
        let run = |plan: &XyceStaticDcPlan, label: &str| {
            let (netlist, results) = self
                .run_static_dc_results(plan, start)
                .map_err(|error| format!("{label} DC execution failed: {error}"))?;
            if results.len() != XYCE_BUG402_DC_POINT_COUNT {
                return Err(format!(
                    "{label} produced {} DC rows instead of {}",
                    results.len(),
                    XYCE_BUG402_DC_POINT_COUNT
                ));
            }
            let table = self.dc_results_to_prn_table(plan, &netlist, &results)?;
            Ok((results, table))
        };

        // Preserve the upstream direction: canonical Xyce DEVICE syntax is
        // GOODFILE and legacy SPICE syntax is TESTFILE.
        let (xyce_results, xyce_table) = match run(
            &contract.xyce_reference_plan,
            "canonical DEVICE TEMP reference",
        ) {
            Ok(result) => result,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 402 paired execution failed: {error}"),
                    Vec::new(),
                );
            }
        };
        let (spice_results, spice_table) =
            match run(&contract.spice_member_plan, "legacy SPICE TEMP member") {
                Ok(result) => result,
                Err(error) => {
                    return self.failure_result(
                        deck,
                        start,
                        result_contract,
                        format!("BUG 402 paired execution failed: {error}"),
                        Vec::new(),
                    );
                }
            };

        let intended_coordinate_contract = (|| {
            let expected_grid = contract.xyce_reference_plan.dc.primary_spec().points();
            if expected_grid.len() != XYCE_BUG402_DC_POINT_COUNT
                || contract.spice_member_plan.dc.primary_spec().points() != expected_grid
            {
                return Err("paired DC plans do not share the exact authored grid".to_string());
            }
            if xyce_table.columns.len() != 3
                || spice_table.columns.len() != 3
                || !xyce_table
                    .columns
                    .iter()
                    .zip(["Index", "V(20)", "I(VMON)"])
                    .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
                || !spice_table
                    .columns
                    .iter()
                    .zip(["Index", "V(20)", "I(VMON)"])
                    .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
            {
                return Err(format!(
                    "paired DC headers changed: canonical={:?}, legacy={:?}",
                    xyce_table.columns, spice_table.columns
                ));
            }
            for (row, expected) in expected_grid.iter().copied().enumerate() {
                let expected = Self::xyce_prn_scientific_roundtrip(
                    expected,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                for (label, results, table) in [
                    ("canonical", &xyce_results, &xyce_table),
                    ("legacy", &spice_results, &spice_table),
                ] {
                    let result = results
                        .get(row)
                        .ok_or_else(|| format!("{label} result row {row} is missing"))?;
                    let values = table
                        .rows
                        .get(row)
                        .ok_or_else(|| format!("{label} table row {row} is missing"))?;
                    if values.len() != 3 || values.iter().any(|value| !value.is_finite()) {
                        return Err(format!(
                            "{label} table row {row} is non-finite or has the wrong width"
                        ));
                    }
                    let result_axis = Self::xyce_prn_scientific_roundtrip(
                        result.sweep_value,
                        XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                    )?;
                    let printed_axis = Self::xyce_prn_scientific_roundtrip(
                        values[1],
                        XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                    )?;
                    if (result_axis - expected).abs()
                        > XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE
                        || (printed_axis - expected).abs()
                            > XYCE_VERIFY_DEFAULT_ABSOLUTE_DIFFERENCE_TOLERANCE
                    {
                        return Err(format!(
                            "{label} row {row} does not preserve the authored sweep coordinate: expected={expected}, result={result_axis}, printed={printed_axis}"
                        ));
                    }
                }
            }
            Ok(())
        })();
        if let Err(error) = intended_coordinate_contract {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("BUG 402 coordinate contract failed: {error}"),
                Vec::new(),
            );
        }

        if let Err(error) = self.validate_bug402_temperature_option_provenance() {
            return self.failure_result(
                deck,
                start,
                result_contract,
                format!("BUG 402 provenance changed during paired execution: {error}"),
                Vec::new(),
            );
        }

        let mismatches = match self.compare_release_7_10_xyce_verify_dc_tables(
            "BUG 402 temperature-option scope",
            &xyce_table,
            &spice_table,
            &xyce_results,
            &spice_results,
        ) {
            Ok(mismatches) => mismatches,
            Err(error) => {
                return self.failure_result(
                    deck,
                    start,
                    result_contract,
                    format!("BUG 402 xyce_verify adapter failed: {error}"),
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
                    "{} BUG 402 temperature-option mismatch(es)",
                    mismatches.len()
                ),
                mismatches,
            )
        }
    }
}
