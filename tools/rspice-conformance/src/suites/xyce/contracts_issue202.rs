use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "ISSUE_202 redefined-parameter mode matrix";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Issue202Role {
    Owner,
    First,
    Last,
}

impl Issue202Role {
    const ALL: [Self; 3] = [Self::Owner, Self::First, Self::Last];
    const CONTROLS: [Self; 2] = [Self::First, Self::Last];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_ISSUE202_OWNER_CONTRACT,
            Self::First => XYCE_ISSUE202_FIRST_CONTRACT,
            Self::Last => XYCE_ISSUE202_LAST_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::Owner => XYCE_ISSUE202_OWNER_PATH,
            Self::First => XYCE_ISSUE202_FIRST_PATH,
            Self::Last => XYCE_ISSUE202_LAST_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Owner => XYCE_ISSUE202_OWNER_RECORD,
            Self::First => XYCE_ISSUE202_FIRST_RECORD,
            Self::Last => XYCE_ISSUE202_LAST_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("ISSUE202 path has name")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Issue202Mode {
    NoOption,
    Ignore,
    Warning,
    Warn,
    UseFirst,
    UseFirstWarn,
    UseLast,
    UseLastWarn,
}

impl Issue202Mode {
    const SUCCESSFUL: [Self; 8] = [
        Self::NoOption,
        Self::Ignore,
        Self::Warning,
        Self::Warn,
        Self::UseFirst,
        Self::UseFirstWarn,
        Self::UseLast,
        Self::UseLastWarn,
    ];

    fn label(self) -> &'static str {
        match self {
            Self::NoOption => "nooption",
            Self::Ignore => "ignore",
            Self::Warning => "warning",
            Self::Warn => "warn",
            Self::UseFirst => "usefirst",
            Self::UseFirstWarn => "usefirstwarn",
            Self::UseLast => "uselast",
            Self::UseLastWarn => "uselastwarn",
        }
    }

    fn policies(
        self,
    ) -> (
        ParameterRedefinitionPolicy,
        rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    ) {
        use rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy as Diagnostic;
        match self {
            Self::NoOption | Self::Ignore | Self::UseFirst => {
                (ParameterRedefinitionPolicy::UseFirst, Diagnostic::Silent)
            }
            Self::Warning | Self::Warn | Self::UseFirstWarn => {
                (ParameterRedefinitionPolicy::UseFirst, Diagnostic::Warning)
            }
            Self::UseLast => (ParameterRedefinitionPolicy::UseLast, Diagnostic::Silent),
            Self::UseLastWarn => (ParameterRedefinitionPolicy::UseLast, Diagnostic::Warning),
        }
    }

    fn control(self) -> Issue202Role {
        match self {
            Self::UseLast | Self::UseLastWarn => Issue202Role::Last,
            _ => Issue202Role::First,
        }
    }
}

impl XyceTestRunner {
    pub(super) fn issue202_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_ISSUE202_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, blake3)| {
                format!(
                    "{XYCE_ISSUE202_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_ISSUE202_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_issue202_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::issue202_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_ISSUE202_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_ISSUE202_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_ISSUE202_HISTORICAL_RECORDS_SHA256
            || blake3 != XYCE_ISSUE202_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{LABEL} Release provenance changed: records={}/{}, sha256={sha256}, blake3={blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_issue202_directory(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let directory = self.root.join("Netlists/Certification_Tests/ISSUE_202");
        let metadata = fs::symlink_metadata(&directory)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_ISSUE202_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        let mut records = Vec::new();
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect {LABEL}: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{LABEL} member {} is not a regular file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!("{LABEL} contains a case collision for {name:?}"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!("{LABEL} acquired unexpected member {name:?}"));
            };
            if name != expected_name {
                return Err(format!("{LABEL} member case changed: {name:?}"));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{LABEL} retained member {name:?} changed"));
            }
            records.push(format!(
                "{expected_name}\t{expected_bytes}\t{expected_sha256}\t{expected_blake3}"
            ));
            observed.insert(key, bytes);
        }
        records.sort();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_ISSUE202_RETAINED_RECORD_COUNT
            || stream.len() != XYCE_ISSUE202_RETAINED_RECORD_BYTES
            || sha256 != XYCE_ISSUE202_RETAINED_RECORDS_SHA256
            || content_blake3 != XYCE_ISSUE202_RETAINED_RECORDS_BLAKE3
        {
            return Err(format!("{LABEL} retained family census changed"));
        }
        Ok(observed)
    }

    fn validate_issue202_provenance(
        &self,
        deck: &XyceDeck,
        role: Issue202Role,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        Self::validate_issue202_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {LABEL} role is not at its canonical path"
            ));
        }
        let prefix = "netlists/certification_tests/issue_202/";
        let owners = Self::load_upstream_wrapper_decks(&self.root)
            .into_iter()
            .filter(|record| record.starts_with(prefix))
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_ISSUE202_OWNER_RECORD.to_string()]) {
            return Err(format!("{LABEL} wrapper ownership changed: {owners:?}"));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_ISSUE202_OWNER_RECORD) {
            return Err(format!("{LABEL} owner must not be excluded"));
        }
        let family = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .collect::<BTreeMap<_, _>>();
        if family.len() != 2 {
            return Err(format!("{LABEL} requires exactly two qualified controls"));
        }
        for control in Issue202Role::CONTROLS {
            let row = family
                .get(&control.record().to_string())
                .copied()
                .ok_or_else(|| format!("{LABEL} lost {}", control.file_name()))?;
            if row.source != XYCE_ISSUE202_EXCLUSION_SOURCE
                || !matches!(&row.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == control.contract())
            {
                return Err(format!(
                    "{LABEL} {} qualification changed",
                    control.file_name()
                ));
            }
        }
        let members = self.validate_issue202_directory()?;
        for role in Issue202Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(role.path()))
                .map_err(|error| format!("{LABEL} {} {error}", role.file_name()))?;
        }
        let output = self.root.join("OutputData/Certification_Tests/ISSUE_202");
        match fs::symlink_metadata(&output) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(format!("failed to inspect {LABEL} OutputData: {error}")),
            Ok(_) => return Err(format!("{LABEL} must not acquire invented numerical gold")),
        }
        Ok(members)
    }

    fn validate_issue202_plan(
        role: Issue202Role,
        plan: &XyceStaticDcPlan,
        expected_diagnostics: usize,
    ) -> Result<(), String> {
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
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.start.to_bits() != 1.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || probes != ["v(1)", "i(v1)", "r1:r"]
            || plan.diagnostics.len() != expected_diagnostics
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_issue202_netlist(
        role: Issue202Role,
        netlist: &Netlist,
        expected_resistance: Value,
        diagnostic_policy: rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    ) -> Result<(), String> {
        if !netlist.title.is_empty()
            || netlist.elements.len() != 2
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || netlist.params.get("FOOBAR").map(Value::to_bits)
                != Some(expected_resistance.to_bits())
            || netlist.params.all_params().len() != 1
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let [resistor, source] = netlist.elements.as_slice() else {
            unreachable!("ISSUE202 element count checked")
        };
        if resistor.provenance != ElementProvenance::Authored
            || !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == expected_resistance.to_bits()
                && value_expr.is_none()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || source.provenance != ElementProvenance::Authored
            || !source.name.eq_ignore_ascii_case("V1")
            || source.nodes != ["1", "0"]
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} topology changed: resistor={resistor:?}, source={source:?}",
                role.file_name()
            ));
        }
        let expected_warning_count = if role == Issue202Role::Owner
            && diagnostic_policy
                == rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Warning
        {
            5
        } else {
            0
        };
        if netlist.diagnostics.len() != expected_warning_count {
            return Err(format!(
                "{LABEL} {} diagnostic count changed for {diagnostic_policy:?}: expected {expected_warning_count}, actual {:?}",
                role.file_name(),
                netlist.diagnostics,
            ));
        }
        if expected_warning_count != 0 {
            let selected = match netlist.params.parameter_redefinition_policy() {
                ParameterRedefinitionPolicy::UseFirst => "first",
                ParameterRedefinitionPolicy::UseLast => "last",
            };
            for (offset, diagnostic) in netlist.diagnostics.iter().enumerate() {
                let expected_line = offset + 3;
                if diagnostic.code != "parameter-redefinition"
                    || diagnostic.severity != rspice_core::netlist::DiagnosticSeverity::Warning
                    || diagnostic.line != expected_line
                    || diagnostic.origin.as_ref().is_none_or(|origin| {
                        origin.line != expected_line
                            || origin
                                .path
                                .as_deref()
                                .is_none_or(|path| !path.ends_with(Path::new(role.path())))
                    })
                    || diagnostic.message
                        != format!("Parameter FOOBAR defined more than once. Using {selected} one.")
                {
                    return Err(format!(
                        "{LABEL} warning projection changed: {diagnostic:?}"
                    ));
                }
            }
        }
        Ok(())
    }

    fn validate_issue202_table(
        role: Issue202Role,
        table: &XycePrnTable,
        expected_resistance: Value,
    ) -> Result<(), String> {
        if table.columns.len() != 4
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || !table.columns[1].eq_ignore_ascii_case("V(1)")
            || !table.columns[2].eq_ignore_ascii_case("I(V1)")
            || !table.columns[3].eq_ignore_ascii_case("R1:R")
            || table.rows.len() != 1
            || table.rows[0].len() != 4
            || table.rows[0].iter().any(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        let row = &table.rows[0];
        let expected_current = -1.0 / expected_resistance;
        for (actual, expected, label) in [
            (row[0], 0.0, "Index"),
            (row[1], 1.0, "V(1)"),
            (row[2], expected_current, "I(V1)"),
            (row[3], expected_resistance, "R1:R"),
        ] {
            let actual = Self::xyce_default_prn_roundtrip(actual)?;
            let expected = Self::xyce_default_prn_roundtrip(expected)?;
            if actual != expected {
                return Err(format!(
                    "{LABEL} {} {label} analytic value changed: {actual} != {expected}",
                    role.file_name()
                ));
            }
        }
        Ok(())
    }

    fn issue202_plan(
        &self,
        role: Issue202Role,
        selection: ParameterRedefinitionPolicy,
        diagnostic: rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_path_with_redefinition_policies(
            &self.root.join(role.path()),
            ExpressionDialect::Xyce,
            selection,
            diagnostic,
        )?;
        Self::validate_issue202_plan(
            role,
            &plan,
            usize::from(
                role == Issue202Role::Owner
                    && diagnostic
                        == rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Warning,
            ) * 5,
        )?;
        Ok(plan)
    }

    pub(super) fn validate_issue202_oracle(
        &self,
        deck: &XyceDeck,
        role: Issue202Role,
        start: Instant,
    ) -> Result<(), String> {
        use rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy as Diagnostic;
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before provenance"
            ));
        }
        let members = self.validate_issue202_provenance(deck, role)?;

        let run = |plan: &XyceStaticDcPlan, expected: Value, member_role: Issue202Role| {
            let source = members
                .get(&member_role.file_name().to_ascii_lowercase())
                .ok_or_else(|| format!("{LABEL} lost {}", member_role.file_name()))?;
            if plan.source.as_bytes() != source.as_slice() {
                return Err(format!(
                    "{LABEL} {} source changed between reads",
                    member_role.file_name()
                ));
            }
            let (netlist, results) = self.run_static_dc_results(plan, start).map_err(|error| {
                format!(
                    "{LABEL} {} execution failed: {error}",
                    member_role.file_name()
                )
            })?;
            Self::validate_issue202_netlist(
                member_role,
                &netlist,
                expected,
                plan.parameter_redefinition_diagnostic_policy,
            )?;
            let table = self.dc_results_to_prn_table(plan, &netlist, &results)?;
            Self::validate_issue202_table(member_role, &table, expected)?;
            Ok::<_, String>((results, table))
        };

        let first_plan = self.issue202_plan(
            Issue202Role::First,
            ParameterRedefinitionPolicy::UseFirst,
            Diagnostic::Silent,
        )?;
        let last_plan = self.issue202_plan(
            Issue202Role::Last,
            ParameterRedefinitionPolicy::UseLast,
            Diagnostic::Silent,
        )?;
        let (first_results, first_table) = run(&first_plan, 1_000.0, Issue202Role::First)?;
        let (last_results, last_table) = run(&last_plan, 1.0, Issue202Role::Last)?;

        for mode in Issue202Mode::SUCCESSFUL {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} exceeded its shared deadline before {}",
                    mode.label()
                ));
            }
            let (selection, diagnostic) = mode.policies();
            let plan = self.issue202_plan(Issue202Role::Owner, selection, diagnostic)?;
            let control = mode.control();
            let expected = if control == Issue202Role::First {
                1_000.0
            } else {
                1.0
            };
            let (owner_results, owner_table) = run(&plan, expected, Issue202Role::Owner)?;
            let (good_results, good_table) = if control == Issue202Role::First {
                (&first_results, &first_table)
            } else {
                (&last_results, &last_table)
            };
            let mismatches = self.compare_release_7_10_xyce_verify_dc_tables(
                &format!("ISSUE_202 {}", mode.label()),
                good_table,
                &owner_table,
                good_results,
                &owner_results,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} produced {} mismatch(es): {mismatches:?}",
                    mode.label(),
                    mismatches.len()
                ));
            }
        }

        let owner_source = fs::read_to_string(self.root.join(XYCE_ISSUE202_OWNER_PATH))
            .map_err(|error| format!("failed to read {LABEL} owner: {error}"))?;
        if members
            .get(&Issue202Role::Owner.file_name().to_ascii_lowercase())
            .is_none_or(|source| source.as_slice() != owner_source.as_bytes())
        {
            return Err(format!(
                "{LABEL} owner source changed before error-mode validation"
            ));
        }
        let error = Self::parse_netlist_with_expression_dialect_policies_and_execution_dir(
            &owner_source,
            &self.root.join(XYCE_ISSUE202_OWNER_PATH),
            ExpressionDialect::Xyce,
            ParameterRedefinitionPolicy::UseFirst,
            Diagnostic::Error,
            None,
        )
        .expect_err("ISSUE202 error mode must reject a duplicate");
        let rspice_core::netlist::ParseError::ParameterRedefinition(error) = error else {
            return Err(format!(
                "{LABEL} error mode returned the wrong failure: {error}"
            ));
        };
        if error.canonical_name != "FOOBAR"
            || error.kind != rspice_core::netlist::ParameterDefinitionKind::Parameter
            || error.first_origin.line != 2
            || error.duplicate_origin.line != 3
            || !error
                .to_string()
                .contains("Parameter FOOBAR defined more than once")
        {
            return Err(format!("{LABEL} error-mode diagnostic changed: {error:?}"));
        }

        self.validate_issue202_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline"));
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

    fn fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-issue202-{label}-"))
            .tempdir()
            .expect("create ISSUE202 fixture");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/ISSUE_202");
        fs::create_dir_all(&family).expect("create ISSUE202 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/ISSUE_202");
        for (name, ..) in XYCE_ISSUE202_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy ISSUE202 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!(
                "{}\t{}\n",
                XYCE_ISSUE202_OWNER_PATH, REQUIRES_UPSTREAM_WRAPPER_CONTRACT
            ),
        )
        .expect("write wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n",
                XYCE_ISSUE202_FIRST_PATH,
                XYCE_ISSUE202_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_ISSUE202_FIRST_CONTRACT,
                XYCE_ISSUE202_LAST_PATH,
                XYCE_ISSUE202_EXCLUSION_SOURCE,
                RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                XYCE_ISSUE202_LAST_CONTRACT,
            ),
        )
        .expect("write exclusion manifest");
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_ISSUE202_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_ISSUE202_OWNER_PATH.to_string(),
        };
        (temporary, deck, runner)
    }

    #[test]
    fn issue202_historical_provenance_is_exact() {
        XyceTestRunner::validate_issue202_historical_oracle_provenance()
            .expect("ISSUE202 Release provenance remains exact");
    }

    #[test]
    fn issue202_all_roles_execute_the_complete_mode_matrix() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 30_000,
                ..XyceRunnerConfig::default()
            },
        );
        for role in Issue202Role::ALL {
            let deck = XyceDeck {
                section: XyceDeckSection::Netlists,
                path: root.join(role.path()),
                relative_path: role.path().to_string(),
            };
            runner
                .validate_issue202_oracle(&deck, role, Instant::now())
                .unwrap_or_else(|error| panic!("{} failed: {error}", role.file_name()));
        }
    }

    #[test]
    fn issue202_expired_deadline_fails_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            section: XyceDeckSection::Netlists,
            path: root.join(XYCE_ISSUE202_OWNER_PATH),
            relative_path: XYCE_ISSUE202_OWNER_PATH.to_string(),
        };
        assert!(
            runner
                .validate_issue202_oracle(
                    &deck,
                    Issue202Role::Owner,
                    Instant::now() - Duration::from_secs(1),
                )
                .is_err()
        );
    }

    #[test]
    fn issue202_relation_rejects_numeric_counterfactuals() {
        let first = XycePrnTable {
            columns: vec!["Index".into(), "V(1)".into(), "I(V1)".into(), "R1:R".into()],
            rows: vec![vec![0.0, 1.0, -1.0e-3, 1_000.0]],
        };
        XyceTestRunner::validate_issue202_table(Issue202Role::First, &first, 1_000.0)
            .expect("canonical first-definition table");
        let mut wrong_current = first.clone();
        wrong_current.rows[0][2] = -2.0e-3;
        assert!(
            XyceTestRunner::validate_issue202_table(Issue202Role::First, &wrong_current, 1_000.0,)
                .is_err()
        );
        let mut wrong_parameter_probe = first;
        wrong_parameter_probe.rows[0][3] = 1.0;
        assert!(
            XyceTestRunner::validate_issue202_table(
                Issue202Role::First,
                &wrong_parameter_probe,
                1_000.0,
            )
            .is_err()
        );
    }

    #[test]
    fn issue202_provenance_rejects_source_output_and_role_drift() {
        let (_temporary, deck, runner) = fixture("source");
        runner
            .validate_issue202_provenance(&deck, Issue202Role::Owner)
            .expect("canonical ISSUE202 fixture");
        fs::write(runner.root.join(XYCE_ISSUE202_FIRST_PATH), "* changed\n")
            .expect("mutate control");
        assert!(
            runner
                .validate_issue202_provenance(&deck, Issue202Role::Owner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("output");
        fs::write(
            runner
                .root
                .join("Netlists/Certification_Tests/ISSUE_202/first.cir.prn"),
            "invented\n",
        )
        .expect("add generated sidecar");
        assert!(
            runner
                .validate_issue202_provenance(&deck, Issue202Role::Owner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("qualification");
        let exclusions = runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&exclusions).expect("read exclusions");
        fs::write(
            &exclusions,
            text.replace(XYCE_ISSUE202_FIRST_CONTRACT, XYCE_ISSUE202_LAST_CONTRACT),
        )
        .expect("change qualification role");
        assert!(
            runner
                .validate_issue202_provenance(&deck, Issue202Role::Owner)
                .is_err()
        );

        let (_temporary, deck, runner) = fixture("owner");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove live wrapper owner");
        assert!(
            runner
                .validate_issue202_provenance(&deck, Issue202Role::Owner)
                .is_err(),
            "wrapper ownership must be reloaded during final provenance"
        );
    }
}
