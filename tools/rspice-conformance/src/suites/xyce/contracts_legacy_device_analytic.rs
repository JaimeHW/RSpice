use super::*;

type LegacyArtifactRecord<'a> = (&'a str, usize, &'a str, &'a str);

impl XyceTestRunner {
    fn legacy_device_analytic_historical_records(
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> Vec<String> {
        let (commit, tag, artifacts): (&str, &str, &[LegacyArtifactRecord<'_>]) = match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => (
                XYCE_BJT_ANALYTIC_UPSTREAM_REGRESSION_COMMIT,
                XYCE_BJT_ANALYTIC_UPSTREAM_RELEASE_TAG,
                &XYCE_BJT_ANALYTIC_HISTORICAL_ARTIFACTS,
            ),
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => (
                XYCE_NMOS_ANALYTIC_UPSTREAM_REGRESSION_COMMIT,
                XYCE_NMOS_ANALYTIC_UPSTREAM_RELEASE_TAG,
                &XYCE_NMOS_ANALYTIC_HISTORICAL_ARTIFACTS,
            ),
        };
        let mut records = artifacts
            .iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!("{commit}\t{tag}\t{path}\t{bytes}\t{sha256}\t{content_blake3}")
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    fn validate_legacy_device_analytic_historical_provenance(
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> Result<(), String> {
        let records = Self::legacy_device_analytic_historical_records(kind);
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        let (expected_count, expected_bytes, expected_sha256, expected_blake3) = match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => (
                XYCE_BJT_ANALYTIC_HISTORICAL_RECORD_COUNT,
                XYCE_BJT_ANALYTIC_HISTORICAL_RECORD_BYTES,
                XYCE_BJT_ANALYTIC_HISTORICAL_RECORDS_SHA256,
                XYCE_BJT_ANALYTIC_HISTORICAL_RECORDS_BLAKE3,
            ),
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => (
                XYCE_NMOS_ANALYTIC_HISTORICAL_RECORD_COUNT,
                XYCE_NMOS_ANALYTIC_HISTORICAL_RECORD_BYTES,
                XYCE_NMOS_ANALYTIC_HISTORICAL_RECORDS_SHA256,
                XYCE_NMOS_ANALYTIC_HISTORICAL_RECORDS_BLAKE3,
            ),
        };
        if records.len() != expected_count
            || stream.len() != expected_bytes
            || sha256 != expected_sha256
            || content_blake3 != expected_blake3
        {
            return Err(format!(
                "{} Release-7.10 generated-gold provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                kind.family(),
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn legacy_device_analytic_family_members(
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> &'static [LegacyArtifactRecord<'static>] {
        match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => {
                &XYCE_BJT_ANALYTIC_RETAINED_ARTIFACTS
            }
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => &XYCE_NMOS_ANALYTIC_RETAINED_ARTIFACTS,
        }
    }

    fn legacy_device_analytic_family_kinds(
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> &'static [XyceLegacyDeviceAnalyticKind] {
        const BJT: [XyceLegacyDeviceAnalyticKind; 2] = [
            XyceLegacyDeviceAnalyticKind::BjtRamp1,
            XyceLegacyDeviceAnalyticKind::BjtRamp2,
        ];
        const NMOS: [XyceLegacyDeviceAnalyticKind; 1] = [XyceLegacyDeviceAnalyticKind::NmosLevel1];
        match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => &BJT,
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => &NMOS,
        }
    }

    fn validate_legacy_device_analytic_provenance(
        &self,
        deck: &XyceDeck,
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> Result<Vec<u8>, String> {
        let label = kind.label();
        Self::validate_legacy_device_analytic_historical_provenance(kind)?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != kind.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != kind.record()
            || !Self::same_path(&deck.path, &self.root.join(kind.path()))
        {
            return Err(format!(
                "recognized {label} record '{}' is not backed by its exact canonical Netlists path",
                deck.relative_path
            ));
        }

        let family_kinds = Self::legacy_device_analytic_family_kinds(kind);
        let prefix = format!("netlists/{}/", kind.family().to_ascii_lowercase());
        let expected_owners = family_kinds
            .iter()
            .map(|member| member.record())
            .collect::<BTreeSet<_>>();
        let observed_owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(&prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if observed_owners != expected_owners {
            return Err(format!(
                "{} requires exactly its removed-wrapper owners, found {observed_owners:?}",
                kind.family()
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{} exclusion manifest is invalid: {error}", kind.family()))?;
        if family_kinds
            .iter()
            .any(|member| exclusions.contains_key(member.record()))
        {
            return Err(format!(
                "{} must not be classified by an upstream exclude sentinel",
                kind.family()
            ));
        }

        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| format!("{label} has no family directory"))?;
        let family_metadata = fs::symlink_metadata(family_dir)
            .map_err(|error| format!("failed to inspect {} family: {error}", kind.family()))?;
        if !family_metadata.file_type().is_dir() || family_metadata.file_type().is_symlink() {
            return Err(format!(
                "{} family must be a regular non-symlink directory",
                kind.family()
            ));
        }
        let expected = Self::legacy_device_analytic_family_members(kind)
            .iter()
            .copied()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut selected_source = None;
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to read {} family: {error}", kind.family()))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect {} member: {error}", kind.family()))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "{} member {} must be a regular non-symlink file",
                    kind.family(),
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{} member name is not UTF-8", kind.family()))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!(
                    "{} family has a case-colliding member {name:?}",
                    kind.family()
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{} family acquired unexpected member {name:?}",
                    kind.family()
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{} member case changed: expected {expected_name:?}, got {name:?}",
                    kind.family()
                ));
            }
            let bytes = fs::read(&path).map_err(|error| {
                format!("failed to read {} member {name}: {error}", kind.family())
            })?;
            let canonical = Self::canonical_lf_text_identity(label, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{} member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    kind.family(),
                    canonical.len()
                ));
            }
            if name == kind.file_name() {
                selected_source = Some(bytes);
            }
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{} retained family census changed: expected {}, got {}",
                kind.family(),
                expected.len(),
                observed.len()
            ));
        }
        let output_family = self.root.join(format!("OutputData/{}", kind.family()));
        if output_family.exists() {
            return Err(format!(
                "{} acquired an invented numerical output family at {}",
                kind.family(),
                output_family.display()
            ));
        }
        for member in family_kinds {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{} {error}", kind.family()))?;
        }
        selected_source.ok_or_else(|| format!("{label} retained source is missing"))
    }

    fn exact_legacy_device_analytic_params(
        actual: &[(String, Value)],
        expected: &[(&str, Value)],
    ) -> bool {
        let mut actual = actual
            .iter()
            .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
            .collect::<Vec<_>>();
        let mut expected = expected
            .iter()
            .map(|(name, value)| (name.to_ascii_uppercase(), value.to_bits()))
            .collect::<Vec<_>>();
        actual.sort();
        expected.sort();
        actual == expected
    }

    fn legacy_device_analytic_model_matches(
        model: &rspice_core::netlist::ModelDef,
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> bool {
        if !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return false;
        }
        match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 => {
                model.model_type.eq_ignore_ascii_case("NPN")
                    && Self::exact_legacy_device_analytic_params(
                        &model.params,
                        &[
                            ("LEVEL", 1.0),
                            ("ISE", 1.0e-12),
                            ("IS", 1.0e-10),
                            ("NE", 1.0),
                            ("IKF", 1.0),
                            ("CJE", 1.0e-8),
                        ],
                    )
            }
            XyceLegacyDeviceAnalyticKind::BjtRamp2 => {
                model.model_type.eq_ignore_ascii_case("NPN")
                    && Self::exact_legacy_device_analytic_params(
                        &model.params,
                        &[
                            ("LEVEL", 1.0),
                            ("ISE", 1.0e-12),
                            ("IS", 1.0e-10),
                            ("NE", 1.0),
                            ("VAR", 1.0),
                            ("TF", 0.02),
                        ],
                    )
            }
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => {
                model.model_type.eq_ignore_ascii_case("NMOS")
                    && Self::exact_legacy_device_analytic_params(
                        &model.params,
                        &[("LEVEL", 1.0), ("IS", 0.0), ("JS", 0.0), ("VTO", 1.0)],
                    )
            }
        }
    }

    pub(super) fn netlist_element_is_legacy_device_analytic_oracle_candidate(
        netlist: &Netlist,
        element: &rspice_core::netlist::Element,
    ) -> bool {
        match &element.kind {
            ElementKind::Bjt {
                model,
                bjt_type: rspice_core::netlist::BjtType::Npn,
                instance_params,
                deferred_params,
            } if element.nodes.len() == 3
                && instance_params.is_empty()
                && deferred_params.is_empty() =>
            {
                let Some(model) = Self::find_unique_model_in(&netlist.models, model) else {
                    return false;
                };
                Self::legacy_device_analytic_model_matches(
                    model,
                    XyceLegacyDeviceAnalyticKind::BjtRamp1,
                ) || Self::legacy_device_analytic_model_matches(
                    model,
                    XyceLegacyDeviceAnalyticKind::BjtRamp2,
                )
            }
            ElementKind::Mosfet {
                model,
                mos_type: rspice_core::netlist::MosType::Nmos,
                compact_syntax: false,
                instance_params,
                deferred_params,
            } if element.nodes.len() == 4
                && deferred_params.is_empty()
                && Self::exact_legacy_device_analytic_params(
                    instance_params,
                    &[
                        ("W", 1.0e-6),
                        ("L", 1.0e-6),
                        ("AD", 0.0),
                        ("AS", 0.0),
                        ("PD", 0.0),
                        ("PS", 0.0),
                    ],
                ) =>
            {
                Self::find_unique_model_in(&netlist.models, model).is_some_and(|model| {
                    Self::legacy_device_analytic_model_matches(
                        model,
                        XyceLegacyDeviceAnalyticKind::NmosLevel1,
                    )
                })
            }
            _ => false,
        }
    }

    fn legacy_device_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn legacy_device_pwl_ramp_source_matches(element: &rspice_core::netlist::Element) -> bool {
        element.name.eq_ignore_ascii_case("V1")
            && element.provenance == ElementProvenance::Authored
            && Self::legacy_device_nodes_match(&element.nodes, &["1", "0"])
            && matches!(&element.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::DcTransient {
                    dc_value,
                    transient,
                }) if dc_value.to_bits() == 0.0f64.to_bits()
                    && matches!(transient.as_ref(), rspice_core::netlist::SourceSpec::Pwl {
                        points,
                        delay,
                        repeat_from,
                    } if points.len() == 2
                        && points[0].0.to_bits() == 0.0f64.to_bits()
                        && points[0].1.to_bits() == 0.0f64.to_bits()
                        && points[1].0.to_bits() == 1.0f64.to_bits()
                        && points[1].1.to_bits() == 1.0f64.to_bits()
                        && delay.to_bits() == 0.0f64.to_bits()
                        && repeat_from.is_none()))
    }

    fn validate_legacy_device_analytic_typed_contract(
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> Result<(), String> {
        let (expected_step, expected_stop, expected_probes): (Value, Value, &[&str]) = match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 => (1.0e-6, 0.8, &["v(1)", "i(v1)"]),
            XyceLegacyDeviceAnalyticKind::BjtRamp2 => (1.0e-6, 0.1, &["v(1)", "i(v1)"]),
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => (1.0e-3, 10.0, &["v(2)"]),
        };
        let probes = plan
            .require_print(kind.label())?
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.tran.step.to_bits() != expected_step.to_bits()
            || plan.tran.stop.to_bits() != expected_stop.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::WrapperStatic
            || probes != expected_probes
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.measurements.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!(
                "{} exact TRAN/PRINT/semantic envelope changed",
                kind.label()
            ));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        }] if step.to_bits() == expected_step.to_bits() && stop.to_bits() == expected_stop.to_bits())
        {
            return Err(format!("{} authored TRAN command changed", kind.label()));
        }
        let output = &netlist.output_requests[0];
        let expected_dependencies: &[(&str, &str, OutputSymbolKind)] = match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => &[
                ("V", "1", OutputSymbolKind::Node),
                ("I", "V1", OutputSymbolKind::Device),
            ],
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => &[("V", "2", OutputSymbolKind::Node)],
        };
        if output.directive != OutputDirectiveKind::Print
            || output.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || output.name.is_some()
            || output.print_delimiter != Some(PrintDelimiter::Whitespace)
            || !output.expressions.is_empty()
            || output.dependencies.len() != expected_dependencies.len()
            || !output.dependencies.iter().zip(expected_dependencies).all(
                |(actual, (operator, symbol, symbol_kind))| {
                    actual.operator.eq_ignore_ascii_case(operator)
                        && actual.symbol.eq_ignore_ascii_case(symbol)
                        && actual.kind == *symbol_kind
                        && !actual.expression
                },
            )
        {
            return Err(format!("{} typed output request changed", kind.label()));
        }
        let (expected_reltol, expected_abstol): (Value, Value) = match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 => (1.0e-6, 1.0e-12),
            XyceLegacyDeviceAnalyticKind::BjtRamp2 => (1.0e-7, 1.0e-13),
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => (1.0e-7, 1.0e-8),
        };
        if netlist.options.timeint_reltol.map(Value::to_bits) != Some(expected_reltol.to_bits())
            || netlist.options.timeint_abstol.map(Value::to_bits) != Some(expected_abstol.to_bits())
            || netlist.options.gmin.map(Value::to_bits)
                != match kind {
                    XyceLegacyDeviceAnalyticKind::BjtRamp1
                    | XyceLegacyDeviceAnalyticKind::BjtRamp2 => Some(0.0f64.to_bits()),
                    XyceLegacyDeviceAnalyticKind::NmosLevel1 => None,
                }
            || netlist.options.device_voltage_limiting
                != match kind {
                    XyceLegacyDeviceAnalyticKind::BjtRamp1
                    | XyceLegacyDeviceAnalyticKind::BjtRamp2 => Some(false),
                    XyceLegacyDeviceAnalyticKind::NmosLevel1 => None,
                }
            || netlist.options.linsol_tr_partition
                != match kind {
                    XyceLegacyDeviceAnalyticKind::BjtRamp1
                    | XyceLegacyDeviceAnalyticKind::BjtRamp2 => Some(false),
                    XyceLegacyDeviceAnalyticKind::NmosLevel1 => None,
                }
        {
            return Err(format!("{} exact numerical options changed", kind.label()));
        }
        match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => {
                if netlist.models.len() != 1
                    || netlist.elements.len() != 2
                    || !Self::legacy_device_analytic_model_matches(&netlist.models[0], kind)
                    || !Self::legacy_device_pwl_ramp_source_matches(&netlist.elements[0])
                    || !netlist.elements[1].name.eq_ignore_ascii_case("Q1")
                    || netlist.elements[1].provenance != ElementProvenance::Authored
                    || !Self::legacy_device_nodes_match(
                        &netlist.elements[1].nodes,
                        &["1", "1", "0"],
                    )
                    || !matches!(&netlist.elements[1].kind, ElementKind::Bjt {
                        model,
                        bjt_type: rspice_core::netlist::BjtType::Npn,
                        instance_params,
                        deferred_params,
                    } if model.eq_ignore_ascii_case("QMOD")
                        && instance_params.is_empty()
                        && deferred_params.is_empty())
                {
                    return Err(format!("{} exact BJT topology/model changed", kind.label()));
                }
            }
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => {
                if netlist.models.len() != 1
                    || netlist.elements.len() != 4
                    || !Self::legacy_device_analytic_model_matches(&netlist.models[0], kind)
                {
                    return Err(format!(
                        "{} exact MOS topology/model count changed",
                        kind.label()
                    ));
                }
                let [gate, mosfet, capacitor, supply] = netlist.elements.as_slice() else {
                    unreachable!("element count checked above");
                };
                if !gate.name.eq_ignore_ascii_case("V1")
                    || gate.provenance != ElementProvenance::Authored
                    || !Self::legacy_device_nodes_match(&gate.nodes, &["1", "0"])
                    || !matches!(&gate.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == 2.0f64.to_bits())
                    || !mosfet.name.eq_ignore_ascii_case("M1")
                    || mosfet.provenance != ElementProvenance::Authored
                    || !Self::legacy_device_nodes_match(&mosfet.nodes, &["2", "1", "0", "0"])
                    || !Self::netlist_element_is_legacy_device_analytic_oracle_candidate(
                        netlist, mosfet,
                    )
                    || !capacitor.name.eq_ignore_ascii_case("C1")
                    || capacitor.provenance != ElementProvenance::Authored
                    || !Self::legacy_device_nodes_match(&capacitor.nodes, &["2", "3"])
                    || !matches!(&capacitor.kind, ElementKind::Capacitor {
                        value,
                        value_expr: None,
                        initial_voltage: Some(initial_voltage),
                        model: None,
                        instance_params,
                        deferred_params,
                    } if value.to_bits() == (10.0f64 * 1.0e-6).to_bits()
                        && initial_voltage.to_bits() == 0.0f64.to_bits()
                        && instance_params.is_empty()
                        && deferred_params.is_empty())
                    || !supply.name.eq_ignore_ascii_case("VCC")
                    || supply.provenance != ElementProvenance::Authored
                    || !Self::legacy_device_nodes_match(&supply.nodes, &["3", "0"])
                    || !matches!(&supply.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == 5.0f64.to_bits())
                {
                    return Err(format!("{} exact MOS topology changed", kind.label()));
                }
            }
        }
        Ok(())
    }

    fn validate_legacy_device_analytic_output_domain(
        table: &XycePrnTable,
        kind: XyceLegacyDeviceAnalyticKind,
        stop: Value,
    ) -> Result<(), String> {
        let expected_columns: &[&str] = match kind {
            XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => {
                &["Index", "TIME", "V(1)", "I(V1)"]
            }
            XyceLegacyDeviceAnalyticKind::NmosLevel1 => &["Index", "TIME", "v(2)"],
        };
        if table.columns.iter().map(String::as_str).collect::<Vec<_>>() != expected_columns
            || table.rows.len() < 2
        {
            return Err(format!(
                "{} output layout changed: columns={:?}, rows={}",
                kind.label(),
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected_columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous_time.is_some_and(|previous| row[1] <= previous)
            {
                return Err(format!(
                    "{} output row {index} is malformed: {row:?}",
                    kind.label()
                ));
            }
            if matches!(
                kind,
                XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2
            ) && Self::xyce_default_prn_roundtrip(row[1])?.to_bits()
                != Self::xyce_default_prn_roundtrip(row[2])?.to_bits()
            {
                return Err(format!(
                    "{} output row {index} no longer proves the authored PWL clamp V(1)=TIME: {row:?}",
                    kind.label()
                ));
            }
            previous_time = Some(row[1]);
        }
        let first_time = Self::xyce_default_prn_roundtrip(table.rows[0][1])?;
        let final_time = Self::xyce_default_prn_roundtrip(table.rows.last().expect("nonempty")[1])?;
        let expected_stop = Self::xyce_default_prn_roundtrip(stop)?;
        if first_time.to_bits() != 0.0f64.to_bits()
            || final_time.to_bits() != expected_stop.to_bits()
        {
            return Err(format!(
                "{} output domain changed: first={first_time}, last={final_time}, expected={expected_stop}",
                kind.label()
            ));
        }
        Ok(())
    }

    fn legacy_device_analytic_reference_table(
        actual: &XycePrnTable,
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> Result<XycePrnTable, String> {
        const VT: Value = 0.025_864_186;
        let mut rows = Vec::with_capacity(actual.rows.len());
        for row in &actual.rows {
            let index = row[0];
            let time = Self::xyce_default_prn_roundtrip(row[1])?;
            let reference = match kind {
                XyceLegacyDeviceAnalyticKind::BjtRamp1 => {
                    let isat: Value = 1.0e-10;
                    let ise: Value = 1.0e-12;
                    let bf: Value = 100.0;
                    let ikf: Value = 1.0;
                    let cje: Value = 1.0e-8;
                    let vj: Value = 0.75;
                    let fc: Value = 0.5;
                    let grading: Value = 0.33;
                    let a = isat * ((time / VT).exp() - 1.0);
                    let b = (0.25 + a / ikf).sqrt() + 0.5;
                    let transport = a / b + (1.0 / bf + ise / isat) * a;
                    let depletion = if time < fc * vj {
                        cje * (1.0 - time / vj).powf(-grading)
                    } else {
                        cje * (1.0 - fc).powf(-1.0 - grading)
                            * (1.0 - fc * (1.0 + grading) + grading / vj * time)
                    };
                    -transport - depletion
                }
                XyceLegacyDeviceAnalyticKind::BjtRamp2 => {
                    let isat: Value = 1.0e-10;
                    let ise: Value = 1.0e-12;
                    let bf: Value = 100.0;
                    let early_reverse: Value = 1.0;
                    let transit_time: Value = 0.02;
                    let exponential = (time / VT).exp();
                    let a = ((1.0 - time / early_reverse) * isat + isat / bf + ise
                        - transit_time * isat)
                        * (exponential - 1.0);
                    let b = (1.0 - time / early_reverse) * transit_time * isat / VT * exponential;
                    -a - b
                }
                XyceLegacyDeviceAnalyticKind::NmosLevel1 => {
                    let c: Value = 1.0e-5;
                    let threshold: Value = 1.0;
                    let gate: Value = 2.0;
                    let transconductance: Value = 2.0e-5;
                    let supply: Value = 5.0;
                    let overdrive = gate - threshold;
                    let alpha = transconductance / c * overdrive;
                    let transition =
                        2.0 * c / transconductance * (supply - overdrive) / overdrive.powi(2);
                    if time <= transition {
                        supply - alpha / 2.0 * overdrive * time
                    } else {
                        let decay = (-alpha * (time - transition)).exp();
                        2.0 * overdrive * decay / (1.0 + decay)
                    }
                }
            };
            if !reference.is_finite() {
                return Err(format!(
                    "{} analytic generator produced a nonfinite reference",
                    kind.label()
                ));
            }
            let reference = Self::xyce_default_prn_roundtrip(reference)?;
            rows.push(match kind {
                XyceLegacyDeviceAnalyticKind::BjtRamp1 | XyceLegacyDeviceAnalyticKind::BjtRamp2 => {
                    vec![
                        index,
                        time,
                        Self::xyce_default_prn_roundtrip(row[2])?,
                        reference,
                    ]
                }
                XyceLegacyDeviceAnalyticKind::NmosLevel1 => vec![index, time, reference],
            });
        }
        Ok(XycePrnTable {
            columns: actual.columns.clone(),
            rows,
        })
    }

    pub(super) fn validate_legacy_device_analytic_oracle(
        &self,
        deck: &XyceDeck,
        kind: XyceLegacyDeviceAnalyticKind,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source_bytes = self.validate_legacy_device_analytic_provenance(deck, kind)?;
        if abort.is_aborted() {
            return Err(format!(
                "{} provenance exceeded the shared deadline",
                kind.label()
            ));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            &deck.path,
            XyceStaticTranPlanPurpose::LegacyDeviceAnalyticOracle,
        )?;
        if Self::canonical_lf_text_identity(kind.label(), plan.source.as_bytes())?
            != Self::canonical_lf_text_identity(kind.label(), &source_bytes)?
        {
            return Err(format!(
                "{} plan did not preserve exact source",
                kind.label()
            ));
        }
        let parsed = Self::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .map_err(|error| format!("{} parse failed: {error}", kind.label()))?;
        Self::validate_legacy_device_analytic_typed_contract(&plan, &parsed, kind)?;
        if abort.is_aborted() {
            return Err(format!(
                "{} parse/typed validation exceeded the shared deadline",
                kind.label()
            ));
        }
        let (netlist, result) = self
            .run_transient_family_plan(&plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => {
                    format!("{} execution exceeded the shared deadline", kind.label())
                }
                other => format!("{} execution failed: {other}", kind.label()),
            })?;
        Self::validate_legacy_device_analytic_typed_contract(&plan, &netlist, kind)?;
        let actual = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
        Self::validate_legacy_device_analytic_output_domain(&actual, kind, plan.tran.stop)?;
        let reference = Self::legacy_device_analytic_reference_table(&actual, kind)?;
        let mismatches = self.compare_xyce_verify_transient_tables(&reference, &actual)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{} produced {} Release-7.10 xyce_verify mismatch(es): {mismatches:?}",
                kind.label(),
                mismatches.len()
            ));
        }
        self.validate_legacy_device_analytic_provenance(deck, kind)?;
        if abort.is_aborted() {
            return Err(format!(
                "{} post-execution provenance exceeded the shared deadline",
                kind.label()
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn corpus_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join("tests/xyce")
    }

    fn canonical_deck(root: &Path, kind: XyceLegacyDeviceAnalyticKind) -> XyceDeck {
        XyceDeck {
            path: root.join(kind.path()),
            section: XyceDeckSection::Netlists,
            relative_path: kind.path().to_string(),
        }
    }

    fn fixture(
        label: &str,
        kind: XyceLegacyDeviceAnalyticKind,
    ) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-legacy-analytic-{label}-"))
            .tempdir()
            .expect("create legacy analytic fixture root");
        let root = temporary.path();
        for member in XyceLegacyDeviceAnalyticKind::ALL {
            let destination = root.join(member.path());
            fs::create_dir_all(destination.parent().expect("family parent"))
                .expect("create fixture family");
            fs::copy(source_root.join(member.path()), destination)
                .expect("copy canonical legacy analytic member");
        }
        let owners = XyceLegacyDeviceAnalyticKind::ALL
            .into_iter()
            .map(|member| format!("{}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n", member.path()))
            .collect::<String>();
        fs::write(root.join(HARNESS_MANIFEST_FILE), owners)
            .expect("write legacy analytic wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "# rspice-upstream-exclusions-schema\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\n# source-commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\n# source-netlists-tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty legacy analytic exclusions");
        let deck = canonical_deck(root, kind);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn legacy_device_analytic_historical_provenance_is_exact() {
        assert_historical(XyceLegacyDeviceAnalyticKind::BjtRamp1);
        assert_historical(XyceLegacyDeviceAnalyticKind::NmosLevel1);
    }

    fn assert_historical(kind: XyceLegacyDeviceAnalyticKind) {
        XyceTestRunner::validate_legacy_device_analytic_historical_provenance(kind)
            .expect("Release-7.10 legacy analytic provenance remains exact");
    }

    #[test]
    fn legacy_device_analytic_typed_contracts_preserve_exact_device_laws() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceLegacyDeviceAnalyticKind::ALL {
            let deck = canonical_deck(&root, kind);
            runner
                .validate_legacy_device_analytic_provenance(&deck, kind)
                .expect("canonical legacy analytic provenance qualifies");
            let plan = runner
                .static_tran_plan_for_path_with_purpose(
                    &deck.path,
                    XyceStaticTranPlanPurpose::LegacyDeviceAnalyticOracle,
                )
                .expect("canonical legacy analytic plan builds");
            let parsed = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
                .expect("canonical legacy analytic deck parses");
            XyceTestRunner::validate_legacy_device_analytic_typed_contract(&plan, &parsed, kind)
                .expect("canonical legacy analytic typed contract qualifies");
        }
    }

    fn assert_typed_mutation_rejected(
        label: &str,
        kind: XyceLegacyDeviceAnalyticKind,
        original: &str,
        replacement: &str,
    ) {
        let (_temporary, deck, runner) = fixture(label, kind);
        let source = fs::read_to_string(&deck.path).expect("read analytic mutation fixture");
        assert!(
            source.contains(original),
            "mutation token {original:?} must exist in the canonical source"
        );
        fs::write(&deck.path, source.replacen(original, replacement, 1))
            .expect("write analytic semantic mutation");
        let result = runner
            .static_tran_plan_for_path_with_purpose(
                &deck.path,
                XyceStaticTranPlanPurpose::LegacyDeviceAnalyticOracle,
            )
            .and_then(|plan| {
                let parsed = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
                    .map_err(|error| error.to_string())?;
                XyceTestRunner::validate_legacy_device_analytic_typed_contract(&plan, &parsed, kind)
            });
        assert!(
            result.is_err(),
            "{label} semantic mutation must fail closed"
        );
    }

    #[test]
    fn legacy_device_analytic_typed_contracts_reject_semantic_mutations() {
        assert_typed_mutation_rejected(
            "bjt-model",
            XyceLegacyDeviceAnalyticKind::BjtRamp1,
            "IKF=1 CJE=10n",
            "IKF=2 CJE=10n",
        );
        assert_typed_mutation_rejected(
            "bjt-limiter",
            XyceLegacyDeviceAnalyticKind::BjtRamp2,
            "voltlim=0",
            "voltlim=1",
        );
        assert_typed_mutation_rejected(
            "bjt-probe-order",
            XyceLegacyDeviceAnalyticKind::BjtRamp1,
            "V(1) I(V1)",
            "I(V1) V(1)",
        );
        assert_typed_mutation_rejected(
            "mos-topology",
            XyceLegacyDeviceAnalyticKind::NmosLevel1,
            "M1 2 1 0 0 MOSMOD",
            "M1 2 1 0 3 MOSMOD",
        );
        assert_typed_mutation_rejected(
            "mos-capacitance",
            XyceLegacyDeviceAnalyticKind::NmosLevel1,
            "C1 2 3 10u IC=0",
            "C1 2 3 20u IC=0",
        );
        assert_typed_mutation_rejected(
            "mos-stop",
            XyceLegacyDeviceAnalyticKind::NmosLevel1,
            ".tran 1m 10s",
            ".tran 1m 9s",
        );
    }

    #[test]
    fn legacy_device_analytic_reference_formulas_cover_piecewise_boundaries() {
        let bjt = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into(), "I(V1)".into()],
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0],
                vec![1.0, 0.375, 0.375, 0.0],
                vec![2.0, 0.8, 0.8, 0.0],
            ],
        };
        let bjt_reference = XyceTestRunner::legacy_device_analytic_reference_table(
            &bjt,
            XyceLegacyDeviceAnalyticKind::BjtRamp1,
        )
        .expect("BJT analytic reference evaluates");
        assert!(bjt_reference.rows.iter().all(|row| row[3].is_finite()));
        assert_ne!(
            bjt_reference.rows[1][3].to_bits(),
            bjt_reference.rows[2][3].to_bits()
        );

        let mos = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(2)".into()],
            rows: vec![
                vec![0.0, 0.0, 0.0],
                vec![1.0, 4.0, 0.0],
                vec![2.0, 10.0, 0.0],
            ],
        };
        let mos_reference = XyceTestRunner::legacy_device_analytic_reference_table(
            &mos,
            XyceLegacyDeviceAnalyticKind::NmosLevel1,
        )
        .expect("MOS analytic reference evaluates");
        assert_eq!(mos_reference.rows[0][2].to_bits(), 5.0f64.to_bits());
        assert_eq!(mos_reference.rows[1][2].to_bits(), 1.0f64.to_bits());
        assert!(mos_reference.rows[2][2] < 1.0e-4);
    }

    #[test]
    fn legacy_device_analytic_oracles_execute_and_share_one_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for kind in XyceLegacyDeviceAnalyticKind::ALL {
            runner
                .validate_legacy_device_analytic_oracle(
                    &canonical_deck(&root, kind),
                    kind,
                    Instant::now(),
                )
                .unwrap_or_else(|error| panic!("{} failed: {error}", kind.label()));
        }
        let (_temporary, deck, runner) =
            fixture("deadline", XyceLegacyDeviceAnalyticKind::BjtRamp1);
        let expired = Instant::now() - Duration::from_secs(5);
        assert!(
            runner
                .validate_legacy_device_analytic_oracle(
                    &deck,
                    XyceLegacyDeviceAnalyticKind::BjtRamp1,
                    expired,
                )
                .is_err()
        );
    }

    #[test]
    fn legacy_device_analytic_provenance_rejects_source_output_and_role_drift() {
        let kind = XyceLegacyDeviceAnalyticKind::BjtRamp1;
        let (temporary, deck, runner) = fixture("source", kind);
        fs::write(&deck.path, "mutated\n").expect("mutate source");
        assert!(
            runner
                .validate_legacy_device_analytic_provenance(&deck, kind)
                .is_err()
        );
        drop(temporary);

        let (temporary, deck, runner) = fixture("output", kind);
        fs::create_dir_all(temporary.path().join("OutputData/BJT_ANALYTIC"))
            .expect("create invented output family");
        assert!(
            runner
                .validate_legacy_device_analytic_provenance(&deck, kind)
                .is_err()
        );
        drop(temporary);

        let (temporary, deck, _runner) = fixture("owner", kind);
        fs::write(temporary.path().join(HARNESS_MANIFEST_FILE), "").expect("remove wrapper owners");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_legacy_device_analytic_provenance(&deck, kind)
                .is_err()
        );
    }
}
