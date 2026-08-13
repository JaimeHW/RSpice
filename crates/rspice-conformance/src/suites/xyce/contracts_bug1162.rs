use super::*;

const BUG1162_LABEL: &str = "BUG_1162_SON inconsistent DC sweep";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1162Role {
    Owner,
    Baseline,
    Linear,
    Decade,
    Octave,
}

impl Bug1162Role {
    const ALL: [Self; 5] = [
        Self::Owner,
        Self::Baseline,
        Self::Linear,
        Self::Decade,
        Self::Octave,
    ];
    const WORKERS: [Self; 4] = [Self::Baseline, Self::Linear, Self::Decade, Self::Octave];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::Owner => XYCE_BUG1162_OWNER_PATH,
            Self::Baseline => XYCE_BUG1162_BASELINE_PATH,
            Self::Linear => XYCE_BUG1162_LINEAR_PATH,
            Self::Decade => XYCE_BUG1162_DECADE_PATH,
            Self::Octave => XYCE_BUG1162_OCTAVE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::Owner => XYCE_BUG1162_OWNER_RECORD,
            Self::Baseline => XYCE_BUG1162_BASELINE_RECORD,
            Self::Linear => XYCE_BUG1162_LINEAR_RECORD,
            Self::Decade => XYCE_BUG1162_DECADE_RECORD,
            Self::Octave => XYCE_BUG1162_OCTAVE_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::Owner => XYCE_BUG1162_OWNER_CONTRACT,
            Self::Baseline => XYCE_BUG1162_BASELINE_CONTRACT,
            Self::Linear => XYCE_BUG1162_LINEAR_CONTRACT,
            Self::Decade => XYCE_BUG1162_DECADE_CONTRACT,
            Self::Octave => XYCE_BUG1162_OCTAVE_CONTRACT,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::Owner => "bug_1162_son.cir",
            Self::Baseline => "baseline.cir",
            Self::Linear => "defective_lin.cir",
            Self::Decade => "defective_dec.cir",
            Self::Octave => "defective_oct.cir",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Owner => "wrapper owner",
            Self::Baseline => "one-point baseline",
            Self::Linear => "inconsistent linear control",
            Self::Decade => "inconsistent decade control",
            Self::Octave => "inconsistent octave control",
        }
    }

    fn expected_mode(self) -> DcSweepMode {
        match self {
            Self::Owner => unreachable!("the empty owner has no analysis"),
            Self::Baseline | Self::Linear => DcSweepMode::Linear,
            Self::Decade => DcSweepMode::Decade {
                points_per_decade: 4,
            },
            Self::Octave => DcSweepMode::Octave {
                points_per_octave: 4,
            },
        }
    }

    fn expected_warning_label(self) -> Option<&'static str> {
        match self {
            Self::Owner | Self::Baseline => None,
            Self::Linear => Some("Linear"),
            Self::Decade => Some("Decade"),
            Self::Octave => Some("Octave"),
        }
    }

    fn expected_warning_line(self) -> Option<usize> {
        match self {
            Self::Linear => Some(11),
            Self::Decade | Self::Octave => Some(12),
            Self::Owner | Self::Baseline => None,
        }
    }
}

impl XyceTestRunner {
    pub(super) fn bug1162_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1162_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1162_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1162_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1162_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1162_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1162_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1162_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1162_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1162_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG1162_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1162_source_directory(
        directory: &Path,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {BUG1162_LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG1162_LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1162_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {BUG1162_LABEL} directory: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG1162_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG1162_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!("{BUG1162_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG1162_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG1162_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG1162_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG1162_LABEL} member {name:?} content changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG1162_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug1162_provenance(
        &self,
        deck: &XyceDeck,
        role: Bug1162Role,
    ) -> Result<BTreeMap<Bug1162Role, Vec<u8>>, String> {
        Self::validate_bug1162_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != role.record()
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != role.record()
            || !Self::same_path(&deck.path, &self.root.join(role.path()))
        {
            return Err(format!(
                "recognized {BUG1162_LABEL} {} is not canonical",
                role.label()
            ));
        }
        let prefix = "netlists/certification_tests/bug_1162_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG1162_OWNER_RECORD]) {
            return Err(format!(
                "{BUG1162_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG1162_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1162_OWNER_RECORD) {
            return Err(format!("{BUG1162_LABEL} owner must not be excluded"));
        }
        let family_count = exclusions
            .iter()
            .filter(|(record, _)| record.starts_with(prefix))
            .count();
        if family_count != Bug1162Role::WORKERS.len() {
            return Err(format!(
                "{BUG1162_LABEL} must retain exactly four worker exclusion rows"
            ));
        }
        for worker in Bug1162Role::WORKERS {
            let qualification = exclusions
                .get(worker.record())
                .ok_or_else(|| format!("{BUG1162_LABEL} lost {} exclusion", worker.label()))?;
            if qualification.source != XYCE_BUG1162_EXCLUSION_SOURCE
                || !matches!(&qualification.disposition,
                    XyceUpstreamExclusionDisposition::RspiceIndependentlyQualified { expected_contract }
                        if expected_contract == worker.contract())
            {
                return Err(format!(
                    "{BUG1162_LABEL} {} qualification changed: {qualification:?}",
                    worker.label()
                ));
            }
        }
        let retained = Self::validate_bug1162_source_directory(
            &self.root.join("Netlists/Certification_Tests/BUG_1162_SON"),
        )?;
        let output_dir = self
            .root
            .join("OutputData/Certification_Tests/BUG_1162_SON");
        match fs::symlink_metadata(&output_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {BUG1162_LABEL} OutputData: {error}"
                ));
            }
            Ok(_) => return Err(format!("{BUG1162_LABEL} must not acquire numerical gold")),
        }
        for member in Bug1162Role::ALL {
            self.reject_wrapper_output_artifacts(&self.root.join(member.path()))
                .map_err(|error| format!("{BUG1162_LABEL} {} {error}", member.label()))?;
        }
        Bug1162Role::WORKERS
            .into_iter()
            .map(|worker| {
                retained
                    .get(&worker.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (worker, bytes))
                    .ok_or_else(|| format!("{BUG1162_LABEL} lost {}", worker.file_name()))
            })
            .collect()
    }

    fn validate_bug1162_worker(
        &self,
        role: Bug1162Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        if role == Bug1162Role::Owner {
            return Err(format!("{BUG1162_LABEL} empty owner is not a worker"));
        }
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let expected_stop: Value = if role == Bug1162Role::Baseline {
            100.0
        } else {
            1.0
        };
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.mode != role.expected_mode()
            || plan.dc.start.to_bits() != 100.0f64.to_bits()
            || plan.dc.stop.to_bits() != expected_stop.to_bits()
            || plan.dc.step.to_bits()
                != (if matches!(role, Bug1162Role::Decade | Bug1162Role::Octave) {
                    4.0f64
                } else {
                    1.0f64
                })
                .to_bits()
            || plan.dc.sweep2.is_some()
            || plan.dc.primary_spec().points() != [100.0]
            || plan.print.probes != ["V(1)", "I(v1)"]
        {
            return Err(format!(
                "{BUG1162_LABEL} {} plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG1162_LABEL} {} no longer parses: {error}", role.label())
        })?;
        match role.expected_warning_label() {
            None if !plan.diagnostics.is_empty() || !netlist.diagnostics.is_empty() => {
                return Err(format!("{BUG1162_LABEL} baseline acquired diagnostics"));
            }
            Some(label) => {
                let expected_prefix = format!("{label} DC or STEP parameters for sweep over V1");
                for diagnostics in [&plan.diagnostics, &netlist.diagnostics] {
                    let [diagnostic] = diagnostics.as_slice() else {
                        return Err(format!(
                            "{BUG1162_LABEL} {} must emit exactly one warning",
                            role.label()
                        ));
                    };
                    if diagnostic.code != "xyce-inconsistent-dc-sweep-direction"
                        || !diagnostic.message.starts_with(&expected_prefix)
                        || Some(diagnostic.line) != role.expected_warning_line()
                        || diagnostic.origin.as_ref().map(|origin| origin.line)
                            != role.expected_warning_line()
                    {
                        return Err(format!(
                            "{BUG1162_LABEL} {} warning changed: {diagnostic:?}",
                            role.label()
                        ));
                    }
                }
            }
            None => {}
        }
        if netlist.elements.len() != 2
            || !netlist.models.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG1162_LABEL} {} typed envelope changed",
                role.label()
            ));
        }
        let [resistor, voltage] = netlist.elements.as_slice() else {
            unreachable!("BUG1162 element count was checked")
        };
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
            || !voltage.name.eq_ignore_ascii_case("V1")
            || voltage.nodes != ["1", "0"]
            || voltage.provenance != ElementProvenance::Authored
            || !matches!(&voltage.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{BUG1162_LABEL} {} topology changed: {:?}",
                role.label(),
                netlist.elements
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 2
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || request.dependencies[0].expression
            || request.dependencies[1].kind != OutputSymbolKind::Device
            || !request.dependencies[1].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[1].symbol.eq_ignore_ascii_case("V1")
            || request.dependencies[1].expression
            || !request.expressions.is_empty()
        {
            return Err(format!(
                "{BUG1162_LABEL} {} output request changed: {request:?}",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug1162_worker(
        &self,
        role: Bug1162Role,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(XycePrnTable, Vec<DcSweepPointResult>), String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG1162_LABEL} {} failed: {error}", role.label()))?;
        let [point] = results.as_slice() else {
            return Err(format!(
                "{BUG1162_LABEL} {} produced {} points instead of one",
                role.label(),
                results.len()
            ));
        };
        if point.sweep_value.to_bits() != 100.0f64.to_bits()
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
            || !point.device_op_report.labels_resolve()
        {
            return Err(format!(
                "{BUG1162_LABEL} {} did not produce a finite 100 V observation",
                role.label()
            ));
        }
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG1162_LABEL} {} PRN failed: {error}", role.label()))?;
        let [row] = table.rows.as_slice() else {
            return Err(format!("{BUG1162_LABEL} {} PRN lost its row", role.label()));
        };
        if table.columns != ["Index", "V(1)", "I(v1)"]
            || row.len() != 3
            || row.iter().any(|value| !value.is_finite())
            || Self::xyce_default_prn_roundtrip(row[0])?.to_bits() != 0.0f64.to_bits()
            || Self::xyce_default_prn_roundtrip(row[1])?.to_bits() != 100.0f64.to_bits()
            || (Self::xyce_default_prn_roundtrip(row[2])? + 0.1).abs() > 1.0e-12
        {
            return Err(format!(
                "{BUG1162_LABEL} {} one-point physics changed: {:?}/{row:?}",
                role.label(),
                table.columns
            ));
        }
        Ok((table, results))
    }

    pub(super) fn validate_bug1162_oracle(
        &self,
        deck: &XyceDeck,
        role: Bug1162Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1162_LABEL} deadline expired before validation"
            ));
        }
        let sources = self.validate_bug1162_provenance(deck, role)?;
        let mut output = BTreeMap::new();
        for worker in Bug1162Role::WORKERS {
            let bytes = sources
                .get(&worker)
                .ok_or_else(|| format!("{BUG1162_LABEL} lost {}", worker.label()))?;
            let source = std::str::from_utf8(bytes).map_err(|error| {
                format!("{BUG1162_LABEL} {} is not UTF-8: {error}", worker.label())
            })?;
            let path = self.root.join(worker.path());
            let plan = self.validate_bug1162_worker(worker, source, &path)?;
            output.insert(worker, self.run_bug1162_worker(worker, &plan, start)?);
        }
        let (good, good_results) = output
            .get(&Bug1162Role::Baseline)
            .expect("BUG1162 baseline ran");
        for worker in [
            Bug1162Role::Linear,
            Bug1162Role::Decade,
            Bug1162Role::Octave,
        ] {
            let (test, test_results) = output.get(&worker).expect("BUG1162 control ran");
            let mismatches = self
                .compare_release_7_10_xyce_verify_dc_tables(
                    BUG1162_LABEL,
                    good,
                    test,
                    good_results,
                    test_results,
                )
                .map_err(|error| {
                    format!(
                        "{BUG1162_LABEL} {} comparison failed: {error}",
                        worker.label()
                    )
                })?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{BUG1162_LABEL} {} differs from baseline: {mismatches:?}",
                    worker.label()
                ));
            }
        }
        if abort.is_aborted() {
            return Err(format!("{BUG1162_LABEL} execution exceeded timeout"));
        }
        self.validate_bug1162_provenance(deck, role)?;
        if abort.is_aborted() {
            return Err(format!("{BUG1162_LABEL} final provenance exceeded timeout"));
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

    fn bug1162_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1162-{label}-"))
            .tempdir()
            .expect("create BUG1162 fixture root");
        let root = temporary.path().to_path_buf();
        let family = root.join("Netlists/Certification_Tests/BUG_1162_SON");
        fs::create_dir_all(&family).expect("create BUG1162 source family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_1162_SON");
        for (name, ..) in XYCE_BUG1162_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy canonical BUG1162 source member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1162_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1162 wrapper manifest");
        let mut qualifications = Bug1162Role::WORKERS
            .into_iter()
            .map(|role| {
                format!(
                    "{}\t{}\t{}\t{}",
                    role.path(),
                    XYCE_BUG1162_EXCLUSION_SOURCE,
                    RSPICE_INDEPENDENTLY_QUALIFIED_DISPOSITION,
                    role.contract()
                )
            })
            .collect::<Vec<_>>();
        qualifications.sort();
        let qualifications = qualifications.join("\n");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{qualifications}\n"
            ),
        )
        .expect("write BUG1162 exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1162_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1162_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1162_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1162_historical_oracle_provenance()
            .expect("Release-7.10 BUG1162 provenance remains exact");
    }

    #[test]
    fn bug1162_workers_preserve_one_point_and_typed_warnings() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1162Role::WORKERS {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1162 worker");
            runner
                .validate_bug1162_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            assert!(
                runner
                    .validate_bug1162_worker(
                        role,
                        &source.replacen("R1 1 0 1K", "R1 1 0 2K", 1),
                        &path
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn bug1162_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1162_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1162_OWNER_PATH.to_string(),
        };
        assert!(
            runner
                .validate_bug1162_oracle(
                    &deck,
                    Bug1162Role::Owner,
                    Instant::now() - Duration::from_secs(1)
                )
                .is_err()
        );
    }

    #[test]
    fn bug1162_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (temporary, deck, runner) = bug1162_fixture("source");
        runner
            .validate_bug1162_provenance(&deck, Bug1162Role::Owner)
            .expect("canonical BUG1162 fixture passes");
        fs::write(
            temporary.path().join(XYCE_BUG1162_LINEAR_PATH),
            "* mutated\n",
        )
        .expect("mutate BUG1162 worker");
        assert!(
            runner
                .validate_bug1162_provenance(&deck, Bug1162Role::Owner)
                .is_err()
        );

        let (temporary, deck, _) = bug1162_fixture("manifest");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1162_LINEAR_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("mutate BUG1162 wrapper owner");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug1162_provenance(&deck, Bug1162Role::Owner)
                .is_err()
        );

        let (temporary, deck, _) = bug1162_fixture("exclusion");
        let manifest = temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE);
        let text = fs::read_to_string(&manifest).expect("read BUG1162 exclusions");
        fs::write(
            &manifest,
            text.replace(XYCE_BUG1162_LINEAR_CONTRACT, XYCE_BUG1162_DECADE_CONTRACT),
        )
        .expect("mutate BUG1162 control contract");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug1162_provenance(&deck, Bug1162Role::Owner)
                .is_err()
        );

        let (temporary, deck, runner) = bug1162_fixture("output");
        fs::create_dir_all(
            temporary
                .path()
                .join("OutputData/Certification_Tests/BUG_1162_SON"),
        )
        .expect("create fabricated BUG1162 OutputData");
        assert!(
            runner
                .validate_bug1162_provenance(&deck, Bug1162Role::Owner)
                .is_err()
        );
    }
}
