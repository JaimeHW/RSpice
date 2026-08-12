use super::*;

const BUG352_LABEL: &str = "BUG_352 diode model-expression equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug352Role {
    ExpressionOwner,
    LiteralControl,
}

impl Bug352Role {
    const ALL: [Self; 2] = [Self::ExpressionOwner, Self::LiteralControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::ExpressionOwner => XYCE_BUG352_OWNER_PATH,
            Self::LiteralControl => XYCE_BUG352_CONTROL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ExpressionOwner => XYCE_BUG352_OWNER_RECORD,
            Self::LiteralControl => XYCE_BUG352_CONTROL_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ExpressionOwner => XYCE_BUG352_OWNER_CONTRACT,
            Self::LiteralControl => XYCE_BUG352_CONTROL_CONTRACT,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::ExpressionOwner => "BUG_352a.cir",
            Self::LiteralControl => "BUG_352b.cir",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ExpressionOwner => "model-expression owner",
            Self::LiteralControl => "literal-model control",
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug352_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG352_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG352_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG352_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug352_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug352_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG352_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG352_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG352_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG352_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_352 Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug352_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug352Role,
    ) -> Result<BTreeMap<Bug352Role, Vec<u8>>, String> {
        Self::validate_bug352_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {BUG352_LABEL} {} is not backed by its canonical path",
                role.label()
            ));
        }
        let family_prefix = "netlists/certification_tests/bug_352/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG352_OWNER_RECORD {
            return Err(format!(
                "{BUG352_LABEL} requires the expression deck as its sole wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG352_LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG352_OWNER_RECORD) {
            return Err(format!("{BUG352_LABEL} owner must not be excluded"));
        }
        let family_exclusions = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if family_exclusions.len() != 1 || family_exclusions[0].0 != XYCE_BUG352_CONTROL_RECORD {
            return Err(format!(
                "{BUG352_LABEL} must retain exactly its literal control exclusion row"
            ));
        }
        let control = family_exclusions[0].1;
        if control.source != XYCE_BUG352_EXCLUSION_SOURCE
            || !matches!(&control.disposition,
                XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                    if expected_contract == XYCE_BUG352_CONTROL_CONTRACT)
        {
            return Err(format!(
                "{BUG352_LABEL} literal-control qualification changed: {control:?}"
            ));
        }

        let family_dir = self.root.join("Netlists/Certification_Tests/BUG_352");
        let metadata = fs::symlink_metadata(&family_dir)
            .map_err(|error| format!("failed to inspect {BUG352_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG352_LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG352_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(&family_dir)
            .map_err(|error| format!("failed to read {BUG352_LABEL} family: {error}"))?
        {
            let entry = entry
                .map_err(|error| format!("failed to inspect {BUG352_LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG352_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG352_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!(
                    "{BUG352_LABEL} family contains case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG352_LABEL} family acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG352_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {BUG352_LABEL} member: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(BUG352_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{BUG352_LABEL} member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG352_LABEL} retained family census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        for member_role in Bug352Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member_role.path()))
                .map_err(|error| format!("{BUG352_LABEL} {} {error}", member_role.label()))?;
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_352");
        match fs::symlink_metadata(&output_family) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {BUG352_LABEL} OutputData absence: {error}"
                ));
            }
            Ok(_) => {
                return Err(format!(
                    "{BUG352_LABEL} must not acquire an invented numerical gold at {}",
                    output_family.display()
                ));
            }
        }
        Bug352Role::ALL
            .into_iter()
            .map(|member_role| {
                observed
                    .get(&member_role.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (member_role, bytes))
                    .ok_or_else(|| format!("{BUG352_LABEL} lost {}", member_role.file_name()))
            })
            .collect()
    }

    fn validate_bug352_worker_contract(
        &self,
        role: Bug352Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let authored_marker = match role {
            Bug352Role::ExpressionOwner => "is={foobar1*1.5}",
            Bug352Role::LiteralControl => "is=1.5",
        };
        if !source.to_ascii_lowercase().contains(authored_marker) {
            return Err(format!(
                "{BUG352_LABEL} {} lost its authored model representation",
                role.label()
            ));
        }
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.5f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != ["V(2)", "I(vamon1)"]
        {
            return Err(format!(
                "{BUG352_LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG352_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if netlist.title != "Test"
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 4
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG352_LABEL} {} typed envelope changed",
                role.label()
            ));
        }
        let params = netlist.params.numeric_parameters();
        match role {
            Bug352Role::ExpressionOwner
                if !matches!(params.as_slice(), [(name, value)]
                    if name.eq_ignore_ascii_case("FOOBAR1")
                        && value.to_bits() == 1.0f64.to_bits()) =>
            {
                return Err(format!("{BUG352_LABEL} owner FOOBAR1 binding changed"));
            }
            Bug352Role::LiteralControl if !params.is_empty() => {
                return Err(format!("{BUG352_LABEL} control acquired parameters"));
            }
            _ => {}
        }
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("JMOD1")
            || !model.model_type.eq_ignore_ascii_case("D")
            || !matches!(model.params.as_slice(), [(name, value)]
                if name.eq_ignore_ascii_case("IS")
                    && value.to_bits() == 1.5f64.to_bits())
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "{BUG352_LABEL} {} did not resolve its diode IS to exactly 1.5",
                role.label()
            ));
        }
        let diode = &netlist.elements[0];
        if !diode.name.eq_ignore_ascii_case("D1")
            || diode.nodes != ["1A", "0"]
            || diode.provenance != ElementProvenance::Authored
            || !matches!(&diode.kind, ElementKind::Diode { model, instance_params, deferred_params }
                if model.eq_ignore_ascii_case("JMOD1")
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG352_LABEL} {} diode changed: {diode:?}",
                role.label()
            ));
        }
        let resistor = &netlist.elements[1];
        if !resistor.name.eq_ignore_ascii_case("RS")
            || resistor.nodes != ["1", "1A"]
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{BUG352_LABEL} {} resistor changed", role.label()));
        }
        let exact_source = |element: &rspice_core::netlist::Element,
                            name: &str,
                            nodes: [&str; 2],
                            expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits())
        };
        if !exact_source(&netlist.elements[2], "VAMON1", ["1", "2"], 0.0)
            || !exact_source(&netlist.elements[3], "VIN", ["2", "0"], 5.0)
        {
            return Err(format!("{BUG352_LABEL} {} sources changed", role.label()));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
        } if source.eq_ignore_ascii_case("VIN")
            && start.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0f64.to_bits()
            && step.to_bits() == 0.5f64.to_bits())
        {
            return Err(format!(
                "{BUG352_LABEL} {} typed DC command changed",
                role.label()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 2
        {
            return Err(format!(
                "{BUG352_LABEL} {} typed .PRINT request changed",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug352_worker(
        &self,
        role: Bug352Role,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG352_LABEL} {} failed: {error}", role.label()))?;
        if results.len() != 11 {
            return Err(format!(
                "{BUG352_LABEL} {} produced {} points instead of 11",
                role.label(),
                results.len()
            ));
        }
        for (index, point) in results.iter().enumerate() {
            let expected = index as Value * 0.5;
            if point.sweep_value.to_bits() != expected.to_bits()
                || !point
                    .result
                    .node_voltages
                    .iter()
                    .chain(point.result.branch_currents.iter())
                    .all(|value| value.is_finite())
                || !point
                    .result
                    .dc_observables
                    .iter()
                    .all(|(_, value)| value.is_finite())
                || !point
                    .device_op_report
                    .entries
                    .iter()
                    .flat_map(|entry| entry.params.iter().map(|(_, value)| value))
                    .all(|value| value.is_finite())
                || !point.device_op_report.labels_resolve()
            {
                return Err(format!(
                    "{BUG352_LABEL} {} point {index} is not a finite exact-grid observation",
                    role.label()
                ));
            }
        }
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG352_LABEL} {} PRN failed: {error}", role.label()))?;
        let max_current = table
            .rows
            .iter()
            .filter_map(|row| row.get(2))
            .map(|value| value.abs())
            .fold(0.0f64, Value::max);
        if max_current <= 1.0e-6 {
            return Err(format!(
                "{BUG352_LABEL} {} current trace is trivially zero",
                role.label()
            ));
        }
        Ok(table)
    }

    pub(super) fn validate_bug352_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug352Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG352_LABEL} deadline expired before validation"));
        }
        let sources = self.validate_bug352_provenance(deck, role)?;
        let mut output = BTreeMap::new();
        for worker_role in Bug352Role::ALL {
            let bytes = sources
                .get(&worker_role)
                .ok_or_else(|| format!("{BUG352_LABEL} lost {}", worker_role.label()))?;
            let source = std::str::from_utf8(bytes).map_err(|error| {
                format!(
                    "{BUG352_LABEL} {} is not UTF-8: {error}",
                    worker_role.label()
                )
            })?;
            let path = self.root.join(worker_role.path());
            let plan = self.validate_bug352_worker_contract(worker_role, source, &path)?;
            output.insert(
                worker_role,
                self.run_bug352_worker(worker_role, &plan, start)?,
            );
        }
        let owner = output
            .get(&Bug352Role::ExpressionOwner)
            .expect("both BUG352 workers ran");
        let control = output
            .get(&Bug352Role::LiteralControl)
            .expect("both BUG352 workers ran");
        let mismatches = self
            .compare_serialized_default_prn_tables(owner, control)
            .map_err(|error| format!("{BUG352_LABEL} raw-PRN relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG352_LABEL} expression and literal PRNs differ: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{BUG352_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
            ));
        }
        self.validate_bug352_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG352_LABEL} final provenance exceeded timeout ({}ms)",
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

    fn bug352_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug352-{label}-"))
            .tempdir()
            .expect("create BUG352 fixture root");
        let root = temporary.path().to_path_buf();
        let family = root.join("Netlists/Certification_Tests/BUG_352");
        fs::create_dir_all(&family).expect("create BUG352 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_352");
        for (name, ..) in XYCE_BUG352_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy canonical BUG352 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG352_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG352 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG352_CONTROL_PATH}\t{XYCE_BUG352_EXCLUSION_SOURCE}\t{RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION}\t{XYCE_BUG352_CONTROL_CONTRACT}\n"
            ),
        )
        .expect("write BUG352 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG352_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG352_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug352_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug352_historical_oracle_provenance()
            .expect("Release-7.10 BUG352 provenance remains exact");
    }

    #[test]
    fn bug352_workers_resolve_expression_and_literal_to_the_same_model() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug352Role::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG352 worker");
            runner
                .validate_bug352_worker_contract(role, &source, &path)
                .expect("canonical BUG352 worker passes");
            let mutation = source.replace("1.5", "1.6");
            assert!(
                runner
                    .validate_bug352_worker_contract(role, &mutation, &path)
                    .is_err(),
                "model-value mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug352_oracle_runs_both_exact_roles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug352Role::ALL {
            let deck = XyceDeck {
                path: root.join(role.path()),
                section: XyceDeckSection::Netlists,
                relative_path: role.path().to_string(),
            };
            runner
                .validate_bug352_oracle(&deck, role, Instant::now())
                .expect("canonical BUG352 relational oracle passes");
        }
    }

    #[test]
    fn bug352_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(&root, config);
        let deck = XyceDeck {
            path: root.join(XYCE_BUG352_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG352_OWNER_PATH.to_string(),
        };
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired deadline");
        assert!(
            runner
                .validate_bug352_oracle(&deck, Bug352Role::ExpressionOwner, start)
                .is_err()
        );
    }

    #[test]
    fn bug352_provenance_rejects_source_output_and_qualification_drift() {
        let (_temporary, deck, runner) = bug352_fixture("source-drift");
        runner
            .validate_bug352_provenance(&deck, Bug352Role::ExpressionOwner)
            .expect("canonical BUG352 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG352 deck has parent")
                .join("unexpected.out"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG352 artifact");
        assert!(
            runner
                .validate_bug352_provenance(&deck, Bug352Role::ExpressionOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug352_fixture("output-drift");
        fs::create_dir_all(runner.root.join("OutputData/Certification_Tests/BUG_352"))
            .expect("create forbidden BUG352 output family");
        assert!(
            runner
                .validate_bug352_provenance(&deck, Bug352Role::ExpressionOwner)
                .is_err()
        );

        let (_temporary, deck, runner) = bug352_fixture("qualification-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG352_CONTROL_PATH}\t{XYCE_BUG352_EXCLUSION_SOURCE}\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("demote BUG352 control");
        let refreshed = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(
            refreshed
                .validate_bug352_provenance(&deck, Bug352Role::ExpressionOwner)
                .is_err()
        );
    }
}
