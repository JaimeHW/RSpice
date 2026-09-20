use super::*;

const BUG267_LABEL: &str = "BUG_267 include/global-parameter success wrapper";

impl XyceTestRunner {
    pub(super) fn bug267_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG267_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG267_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG267_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug267_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug267_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG267_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG267_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG267_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG267_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_267 Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug267_retained_directory(
        directory: &Path,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {BUG267_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG267_LABEL} family {} must be a regular non-symlink directory",
                directory.display()
            ));
        }
        let expected = XYCE_BUG267_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {BUG267_LABEL} family: {error}"))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect {BUG267_LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG267_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG267_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!(
                    "{BUG267_LABEL} family contains case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG267_LABEL} family acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG267_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {BUG267_LABEL} member: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(BUG267_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{BUG267_LABEL} member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG267_LABEL} family census changed: expected {} members, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug267_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug267_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG267_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG267_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG267_PATH))
        {
            return Err(format!(
                "recognized {BUG267_LABEL} record '{}' is not backed by its exact canonical path",
                deck.relative_path
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!("{BUG267_LABEL} lost removed-wrapper ownership"));
        }
        let family_prefix = "netlists/certification_tests/bug_267/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG267_RECORD {
            return Err(format!(
                "{BUG267_LABEL} requires its exact sole wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG267_LABEL} exclusion manifest is invalid: {error}"))?;
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if !family_exclusions.is_empty() {
            return Err(format!(
                "{BUG267_LABEL} must not acquire upstream-exclusion records: {family_exclusions:?}"
            ));
        }

        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| format!("{BUG267_LABEL} record has no family directory"))?;
        let retained = Self::validate_bug267_retained_directory(family_dir)?;
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG267_LABEL} {error}"))?;
        let output_anchor = self
            .static_output_reference_path(&deck.path, "prn")
            .ok_or_else(|| format!("{BUG267_LABEL} cannot be mapped into OutputData"))?;
        let output_family = output_anchor
            .parent()
            .ok_or_else(|| format!("{BUG267_LABEL} OutputData anchor has no parent"))?;
        match fs::symlink_metadata(output_family) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to verify absence of {BUG267_LABEL} OutputData family: {error}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{BUG267_LABEL} must not acquire an invented numerical oracle at {}",
                    output_family.display()
                ));
            }
        }
        retained
            .get("bug267.cir")
            .cloned()
            .ok_or_else(|| format!("{BUG267_LABEL} lost its owner source"))
    }

    fn bug267_plan_and_netlist(
        &self,
        source: String,
        deck_path: &Path,
    ) -> Result<(XyceStaticDcPlan, Netlist), String> {
        let netlist = Self::parse_netlist_with_expression_dialect(
            &source,
            deck_path,
            ExpressionDialect::Xyce,
        )
        .map_err(|error| format!("{BUG267_LABEL} source no longer parses: {error}"))?;
        let dc = Self::single_dc_sweep(&netlist)?;
        let print = XycePrintRequest {
            probes: vec!["V(1)".into(), "I(V1)".into(), "bar".into()],
        };
        Self::validate_static_dc_contract(&netlist, &dc, &print)?;
        let steps = Self::step_commands(&netlist)?;
        let plan = XyceStaticDcPlan {
            deck_path: deck_path.to_path_buf(),
            execution_dir: None,
            source,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
            print,
            print_format: None,
            dc,
            dc_data: None,
            steps,
            diagnostics: netlist.diagnostics.clone(),
            sealed_sources: None,
        };
        Ok((plan, netlist))
    }

    fn validate_bug267_typed_contract(
        &self,
        source: String,
        deck_path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let (plan, netlist) = self.bug267_plan_and_netlist(source, deck_path)?;
        if plan.deck_path != deck_path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != ["V(1)", "I(V1)", "bar"]
        {
            return Err(format!("{BUG267_LABEL} static DC plan changed: {plan:?}"));
        }
        if netlist.title != "test to Test the Xyce Resistor Model"
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 2
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{BUG267_LABEL} typed netlist envelope changed"));
        }
        let resistor = &netlist.elements[0];
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1_000.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{BUG267_LABEL} exact R1 topology changed"));
        }
        let source = &netlist.elements[1];
        if !source.name.eq_ignore_ascii_case("V1")
            || source.nodes != ["1", "0"]
            || source.provenance != ElementProvenance::Authored
            || !matches!(&source.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{BUG267_LABEL} exact V1 topology changed"));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
        } if source.eq_ignore_ascii_case("V1")
            && start.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0f64.to_bits()
            && step.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{BUG267_LABEL} typed DC command changed"));
        }

        let expected_sidecar = deck_path
            .parent()
            .ok_or_else(|| format!("{BUG267_LABEL} deck path has no parent"))?
            .join("analysis.cmds");
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.origin.line != 2
            || request
                .origin
                .path
                .as_deref()
                .is_none_or(|path| !Self::same_path(path, &expected_sidecar))
            || request.dependencies.len() != 2
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || request.dependencies[0].expression
            || request.dependencies[1].kind != OutputSymbolKind::Device
            || !request.dependencies[1].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[1].symbol.eq_ignore_ascii_case("V1")
            || request.dependencies[1].expression
        {
            return Err(format!(
                "{BUG267_LABEL} included .PRINT request changed: {request:?}"
            ));
        }

        let mut ordinary_expressions = netlist.params.all_parameter_expressions();
        ordinary_expressions.sort();
        let mut global_expressions = netlist.params.all_global_expressions();
        global_expressions.sort();
        let numeric = netlist.params.numeric_parameters();
        // A fully static ordinary `.PARAM` is deliberately materialized into
        // the numeric namespace, while the dependent `.GLOBAL_PARAM`
        // expression remains authoritative. The exact retained source hash
        // above binds FOOBAR's authored `3*9` expression; these assertions
        // bind its typed projection and BAR's surviving dependency edge.
        if !ordinary_expressions.is_empty()
            || global_expressions != [("BAR".to_string(), "foobar".to_string())]
            || numeric.len() != 2
            || numeric
                .iter()
                .find(|(name, _)| name == "FOOBAR")
                .is_none_or(|(_, value)| value.to_bits() != 27.0f64.to_bits())
            || numeric
                .iter()
                .find(|(name, _)| name == "BAR")
                .is_none_or(|(_, value)| value.to_bits() != 27.0f64.to_bits())
            || netlist.params.get("FOOBAR").map(Value::to_bits) != Some(27.0f64.to_bits())
            || netlist.params.get("BAR").map(Value::to_bits) != Some(27.0f64.to_bits())
        {
            return Err(format!(
                "{BUG267_LABEL} ordinary/global parameter causality changed: ordinary={ordinary_expressions:?}, global={global_expressions:?}, numeric={numeric:?}"
            ));
        }
        Ok(plan)
    }

    fn run_bug267_typed_oracle(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(), String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG267_LABEL} DC execution failed: {error}"))?;
        if results.len() != 6 {
            return Err(format!(
                "{BUG267_LABEL} produced {} DC points instead of 6",
                results.len()
            ));
        }
        for (index, point) in results.iter().enumerate() {
            let expected = index as Value;
            let voltage = point
                .result
                .try_voltage_named("1")
                .ok_or_else(|| format!("{BUG267_LABEL} point {index} lost node 1"))?;
            let current = point
                .result
                .branch_current_named("V1")
                .ok_or_else(|| format!("{BUG267_LABEL} point {index} lost I(V1)"))?;
            let expected_current = -expected / 1_000.0;
            let finite = point.sweep_value.is_finite()
                && point
                    .result
                    .node_voltages
                    .iter()
                    .chain(point.result.branch_currents.iter())
                    .all(|value| value.is_finite())
                && point
                    .result
                    .dc_observables
                    .iter()
                    .all(|(_, value)| value.is_finite())
                && point
                    .device_op_report
                    .entries
                    .iter()
                    .flat_map(|entry| entry.params.iter().map(|(_, value)| value))
                    .all(|value| value.is_finite());
            if point.sweep_value.to_bits() != expected.to_bits()
                || (voltage - expected).abs() > 1.0e-12
                || (current - expected_current).abs() > 1.0e-12
                || !finite
                || !point.device_op_report.labels_resolve()
            {
                return Err(format!(
                    "{BUG267_LABEL} point {index} violates its analytic contract: sweep={}, V(1)={voltage}, I(V1)={current}",
                    point.sweep_value
                ));
            }
        }
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG267_LABEL} PRN materialization failed: {error}"))?;
        if table.columns != ["Index", "V(1)", "I(V1)", "bar"] || table.rows.len() != 6 {
            return Err(format!(
                "{BUG267_LABEL} default PRN layout changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            let expected = index as Value;
            if row.len() != 4
                || row[0].to_bits() != expected.to_bits()
                || (row[1] - expected).abs() > 1.0e-12
                || (row[2] + expected / 1_000.0).abs() > 1.0e-12
                || row[3].to_bits() != 27.0f64.to_bits()
                || !row.iter().all(|value| value.is_finite())
            {
                return Err(format!("{BUG267_LABEL} PRN row {index} changed: {row:?}"));
            }
        }
        let serialized_mismatches = self
            .compare_serialized_default_prn_tables(&table, &table)
            .map_err(|error| format!("{BUG267_LABEL} default PRN serialization failed: {error}"))?;
        if !serialized_mismatches.is_empty() {
            return Err(format!(
                "{BUG267_LABEL} default PRN serialization is not self-consistent: {serialized_mismatches:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug267_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG267_LABEL} deadline expired before validation"));
        }
        let source_bytes = self.validate_bug267_provenance(deck)?;
        let source = String::from_utf8(source_bytes)
            .map_err(|error| format!("{BUG267_LABEL} source is not UTF-8: {error}"))?;
        let plan = self.validate_bug267_typed_contract(source, &deck.path)?;
        self.run_bug267_typed_oracle(&plan, start)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG267_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug267_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG267_LABEL} final provenance exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
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

    fn canonical_deck(root: &Path) -> XyceDeck {
        XyceDeck {
            path: root.join(XYCE_BUG267_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG267_PATH.to_string(),
        }
    }

    fn bug267_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug267-{label}-"))
            .tempdir()
            .expect("create BUG267 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_267");
        fs::create_dir_all(&family).expect("create BUG267 fixture family");
        for name in ["analysis.cmds", "bug267.cir"] {
            fs::copy(
                source_root
                    .join("Netlists/Certification_Tests/BUG_267")
                    .join(name),
                family.join(name),
            )
            .expect("copy canonical BUG267 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG267_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG267 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG267 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug267_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug267_historical_oracle_provenance()
            .expect("Release-7.10 BUG267 provenance remains exact");
    }

    #[test]
    fn bug267_typed_contract_preserves_include_parameter_causality() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG267_PATH);
        let source = fs::read_to_string(&path).expect("read BUG267 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug267_typed_contract(source.clone(), &path)
            .expect("canonical BUG267 typed contract passes");

        for mutation in [
            source.replace(".global_param bar={foobar}", ".param bar={foobar}"),
            source.replace("R1 1 0 1K", "R1 1 0 2K"),
            source.replace("V1 1 0 5V", "V1 2 0 5V"),
            source.replace(".include analysis.cmds", ".include missing.cmds"),
        ] {
            assert!(
                runner
                    .validate_bug267_typed_contract(mutation, &path)
                    .is_err(),
                "BUG267 semantic mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug267_oracle_executes_all_six_analytic_rows() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug267_oracle(&canonical_deck(&root), Instant::now())
            .expect("canonical BUG267 oracle passes");
    }

    #[test]
    fn bug267_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired deadline");
        assert!(
            runner
                .validate_bug267_oracle(&canonical_deck(&root), start)
                .is_err()
        );
    }

    #[test]
    fn bug267_provenance_rejects_family_manifest_and_output_drift() {
        let (_temporary, deck, runner) = bug267_fixture("source-drift");
        let source = fs::read_to_string(&deck.path).expect("read BUG267 fixture source");
        fs::write(
            &deck.path,
            source.replace(".param foobar={3*9}", ".param foobar=27"),
        )
        .expect("mutate BUG267 authored parameter expression");
        assert!(runner.validate_bug267_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug267_fixture("family-drift");
        runner
            .validate_bug267_provenance(&deck)
            .expect("canonical BUG267 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG267 deck has parent")
                .join("unexpected.out"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG267 wrapper output");
        assert!(runner.validate_bug267_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug267_fixture("output-drift");
        let output = runner
            .static_output_reference_path(&deck.path, "prn")
            .expect("map BUG267 fixture to OutputData");
        fs::create_dir_all(output.parent().expect("BUG267 output has parent"))
            .expect("create forbidden BUG267 OutputData family");
        fs::write(output, "invented gold\n").expect("write forbidden BUG267 gold");
        assert!(runner.validate_bug267_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug267_fixture("manifest-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG267 wrapper ownership");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug267_provenance(&deck).is_err());
    }
}
