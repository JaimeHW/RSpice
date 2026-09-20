use super::*;

const BUG1040_LABEL: &str = "BUG_1040_SON NOOP/operating-point equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug1040Worker {
    OperatingPoint,
    Noop,
}

impl Bug1040Worker {
    const ALL: [Self; 2] = [Self::OperatingPoint, Self::Noop];

    fn path(self) -> &'static str {
        match self {
            Self::OperatingPoint => XYCE_BUG1040_OP_PATH,
            Self::Noop => XYCE_BUG1040_NOOP_PATH,
        }
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::OperatingPoint => "rc_discharge_diode_op.net",
            Self::Noop => "rc_discharge_diode_noop.net",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::OperatingPoint => "operating-point worker",
            Self::Noop => "NOOP worker",
        }
    }

    fn uses_noop(self) -> bool {
        matches!(self, Self::Noop)
    }
}

impl XyceTestRunner {
    pub(super) fn bug1040_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1040_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1040_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1040_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1040_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1040_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1040_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1040_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1040_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1040_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG1040_LABEL} Release-7.10 provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1040_source_directory(
        directory: &Path,
    ) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let metadata = fs::symlink_metadata(directory)
            .map_err(|error| format!("failed to inspect {BUG1040_LABEL} directory: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG1040_LABEL} source directory must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1040_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeMap::new();
        for entry in fs::read_dir(directory)
            .map_err(|error| format!("failed to read {BUG1040_LABEL} directory: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG1040_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG1040_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if observed.contains_key(&key) {
                return Err(format!("{BUG1040_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG1040_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG1040_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG1040_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG1040_LABEL} member {name:?} content changed"));
            }
            observed.insert(key, bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG1040_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        Ok(observed)
    }

    fn validate_bug1040_provenance(
        &self,
        deck: &XyceDeck,
    ) -> Result<BTreeMap<Bug1040Worker, Vec<u8>>, String> {
        Self::validate_bug1040_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG1040_OWNER_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path))
                != XYCE_BUG1040_OWNER_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG1040_OWNER_PATH))
        {
            return Err(format!("recognized {BUG1040_LABEL} owner is not canonical"));
        }
        let prefix = "netlists/certification_tests/bug_1040_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG1040_OWNER_RECORD]) {
            return Err(format!(
                "{BUG1040_LABEL} wrapper ownership changed: {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG1040_LABEL} exclusions invalid: {error}"))?;
        let family_exclusions = exclusions
            .keys()
            .filter(|record| record.starts_with(prefix))
            .collect::<Vec<_>>();
        if !family_exclusions.is_empty() {
            return Err(format!(
                "{BUG1040_LABEL} must not be excluded: {family_exclusions:?}"
            ));
        }
        let retained = Self::validate_bug1040_source_directory(
            &self.root.join("Netlists/Certification_Tests/BUG_1040_SON"),
        )?;
        let output_dir = self
            .root
            .join("OutputData/Certification_Tests/BUG_1040_SON");
        match fs::symlink_metadata(&output_dir) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!(
                    "failed to inspect {BUG1040_LABEL} OutputData: {error}"
                ));
            }
            Ok(_) => return Err(format!("{BUG1040_LABEL} must not acquire numerical gold")),
        }
        self.reject_wrapper_output_artifacts(&self.root.join(XYCE_BUG1040_OWNER_PATH))
            .map_err(|error| format!("{BUG1040_LABEL} owner {error}"))?;
        Bug1040Worker::ALL
            .into_iter()
            .map(|role| {
                self.reject_wrapper_output_artifacts(&self.root.join(role.path()))
                    .map_err(|error| format!("{BUG1040_LABEL} {} {error}", role.label()))?;
                retained
                    .get(&role.file_name().to_ascii_lowercase())
                    .cloned()
                    .map(|bytes| (role, bytes))
                    .ok_or_else(|| format!("{BUG1040_LABEL} lost {}", role.file_name()))
            })
            .collect()
    }

    fn bug1040_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug1040_model(model: &rspice_core::netlist::ModelDef) -> bool {
        let expected: [(&str, Value); 14] = [
            ("IS", 4.0e-10),
            ("RS", 0.105),
            ("N", 1.48),
            ("TT", 8.0e-7),
            ("CJO", 1.95e-11),
            ("VJ", 0.4),
            ("M", 0.38),
            ("EG", 1.36),
            ("XTI", -8.0),
            ("KF", 0.0),
            ("AF", 1.0),
            ("FC", 0.9),
            ("BV", 600.0),
            ("IBV", 1.0e-4),
        ];
        model.name.eq_ignore_ascii_case("D1N3940")
            && model.model_type.eq_ignore_ascii_case("D")
            && model.params.len() == expected.len()
            && expected.iter().all(|(expected_name, expected_value)| {
                model.params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == expected_value.to_bits()
                })
            })
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
    }

    fn validate_bug1040_worker(
        &self,
        role: Bug1040Worker,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        let expected_probes = ["V(1)", "V(2)"];
        if plan.deck_path != path
            || plan.source != source
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != expected_probes.len()
                    || print
                        .probes
                        .iter()
                        .zip(expected_probes)
                        .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            })
            || plan.tran.step.to_bits() != 0.0f64.to_bits()
            || plan.tran.stop.to_bits() != 2.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic != role.uses_noop()
        {
            return Err(format!(
                "{BUG1040_LABEL} {} plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG1040_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if netlist.elements.len() != 3
            || netlist.models.len() != 1
            || !Self::validate_bug1040_model(&netlist.models[0])
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{BUG1040_LABEL} {} typed envelope changed: elements={:?}, models={:?}, analyses={:?}, outputs={:?}, diagnostics={:?}",
                role.label(),
                netlist.elements,
                netlist.models,
                netlist.analyses,
                netlist.output_requests,
                netlist.diagnostics
            ));
        }
        let [resistor, diode, capacitor] = netlist.elements.as_slice() else {
            unreachable!("BUG1040 element count was checked")
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::bug1040_nodes_match(&resistor.nodes, &["1", "2"])
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
            || !diode.name.eq_ignore_ascii_case("D1")
            || !Self::bug1040_nodes_match(&diode.nodes, &["2", "0"])
            || diode.provenance != ElementProvenance::Authored
            || !matches!(&diode.kind, ElementKind::Diode {
                model,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("D1N3940")
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::bug1040_nodes_match(&capacitor.nodes, &["1", "0"])
            || capacitor.provenance != ElementProvenance::Authored
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: Some(initial_voltage),
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0e-6f64.to_bits()
                && initial_voltage.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG1040_LABEL} {} topology changed: {:?}",
                role.label(),
                netlist.elements
            ));
        }
        Ok(plan)
    }

    fn run_bug1040_worker(
        &self,
        role: Bug1040Worker,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| format!("{BUG1040_LABEL} {} failed: {error}", role.label()))?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{BUG1040_LABEL} {} PRN failed: {error}", role.label()))?;
        let max_voltage = table
            .rows
            .iter()
            .flat_map(|row| row.get(2..4).into_iter().flatten())
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        let final_voltage = table
            .rows
            .last()
            .and_then(|row| row.get(2))
            .copied()
            .unwrap_or(Value::NAN);
        if table.columns.len() != 4
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || !table.columns[3].eq_ignore_ascii_case("V(2)")
            || table.rows.len() < 3
            || table
                .rows
                .iter()
                .any(|row| row.len() != 4 || row.iter().any(|value| !value.is_finite()))
            || table.rows.first().is_none_or(|row| row[1].abs() > 1.0e-15)
            || table
                .rows
                .last()
                .is_none_or(|row| (row[1] - 2.0e-3).abs() > 1.0e-12)
            || max_voltage < 0.5
            || final_voltage.abs() >= max_voltage
        {
            return Err(format!(
                "{BUG1040_LABEL} {} produced an invalid or trivial table: columns={:?}, rows={}, max={max_voltage}, final={final_voltage}",
                role.label(),
                table.columns,
                table.rows.len()
            ));
        }
        Ok(table)
    }

    pub(super) fn validate_bug1040_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1040_LABEL} deadline expired before validation"
            ));
        }
        let sources = self.validate_bug1040_provenance(deck)?;
        let mut outputs = BTreeMap::new();
        for role in Bug1040Worker::ALL {
            let bytes = sources
                .get(&role)
                .ok_or_else(|| format!("{BUG1040_LABEL} lost {}", role.label()))?;
            let source = std::str::from_utf8(bytes).map_err(|error| {
                format!("{BUG1040_LABEL} {} is not UTF-8: {error}", role.label())
            })?;
            let path = self.root.join(role.path());
            let plan = self.validate_bug1040_worker(role, source, &path)?;
            outputs.insert(role, self.run_bug1040_worker(role, &plan, start)?);
        }
        let good = outputs
            .get(&Bug1040Worker::OperatingPoint)
            .expect("both BUG1040 workers ran");
        let test = outputs
            .get(&Bug1040Worker::Noop)
            .expect("both BUG1040 workers ran");
        let mismatches = self
            .compare_xyce_verify_transient_tables(good, test)
            .map_err(|error| format!("{BUG1040_LABEL} xyce_verify relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG1040_LABEL} NOOP differs from operating-point startup: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{BUG1040_LABEL} execution exceeded timeout"));
        }
        self.validate_bug1040_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG1040_LABEL} final provenance exceeded timeout"));
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

    fn bug1040_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1040-{label}-"))
            .tempdir()
            .expect("create BUG1040 fixture root");
        let root = temporary.path().to_path_buf();
        let family = root.join("Netlists/Certification_Tests/BUG_1040_SON");
        fs::create_dir_all(&family).expect("create BUG1040 source family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_1040_SON");
        for (name, ..) in XYCE_BUG1040_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name))
                .expect("copy canonical BUG1040 source member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1040_OWNER_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1040 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty exclusion manifest");
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1040_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1040_OWNER_PATH.to_string(),
        };
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1040_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1040_historical_oracle_provenance()
            .expect("Release-7.10 BUG1040 provenance remains exact");
    }

    #[test]
    fn bug1040_workers_preserve_the_exact_startup_delta() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1040Worker::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG1040 worker");
            runner
                .validate_bug1040_worker(role, &source, &path)
                .unwrap_or_else(|error| panic!("canonical {role:?} failed: {error}"));
            assert!(
                runner
                    .validate_bug1040_worker(
                        role,
                        &source.replacen("R1 1 2 1K", "R1 1 2 2K", 1),
                        &path
                    )
                    .is_err()
            );
        }
    }

    #[test]
    fn bug1040_oracle_runs_both_startup_modes() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1040_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1040_OWNER_PATH.to_string(),
        };
        runner
            .validate_bug1040_oracle(&deck, Instant::now())
            .expect("canonical BUG1040 relation passes");
    }

    #[test]
    fn bug1040_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let deck = XyceDeck {
            path: root.join(XYCE_BUG1040_OWNER_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1040_OWNER_PATH.to_string(),
        };
        assert!(
            runner
                .validate_bug1040_oracle(&deck, Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }

    #[test]
    fn bug1040_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (temporary, deck, runner) = bug1040_fixture("source");
        runner
            .validate_bug1040_provenance(&deck)
            .expect("canonical BUG1040 fixture passes");
        fs::write(temporary.path().join(XYCE_BUG1040_NOOP_PATH), "* mutated\n")
            .expect("mutate BUG1040 worker");
        assert!(runner.validate_bug1040_provenance(&deck).is_err());

        let (temporary, deck, _runner) = bug1040_fixture("manifest");
        fs::write(
            temporary.path().join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1040_NOOP_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("mutate BUG1040 wrapper owner");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(runner.validate_bug1040_provenance(&deck).is_err());

        let (temporary, deck, _) = bug1040_fixture("exclusion");
        fs::write(
            temporary.path().join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG1040_OWNER_PATH}\tNetlists/Certification_Tests/BUG_1040_SON/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG1040 exclusion");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(runner.validate_bug1040_provenance(&deck).is_err());

        let (temporary, deck, runner) = bug1040_fixture("output");
        fs::create_dir_all(
            temporary
                .path()
                .join("OutputData/Certification_Tests/BUG_1040_SON"),
        )
        .expect("create fabricated BUG1040 OutputData");
        assert!(runner.validate_bug1040_provenance(&deck).is_err());
    }
}
