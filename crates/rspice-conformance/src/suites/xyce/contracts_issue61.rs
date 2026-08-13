use super::*;

const ISSUE61_LABEL: &str = "ISSUE_61 behavioral lead-current expected failure";

impl XyceTestRunner {
    pub(super) fn issue61_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_ISSUE61_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_ISSUE61_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_ISSUE61_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_issue61_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::issue61_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_ISSUE61_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_ISSUE61_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_ISSUE61_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_ISSUE61_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{ISSUE61_LABEL} Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_issue61_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_issue61_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_ISSUE61_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_ISSUE61_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_ISSUE61_PATH))
        {
            return Err(format!(
                "recognized {ISSUE61_LABEL} record '{}' is not its canonical path",
                deck.relative_path
            ));
        }

        let prefix = "netlists/certification_tests/issue_61/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_ISSUE61_RECORD]) {
            return Err(format!(
                "{ISSUE61_LABEL} requires its sole active wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{ISSUE61_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_ISSUE61_RECORD) {
            return Err(format!(
                "{ISSUE61_LABEL} must not acquire an upstream exclusion row"
            ));
        }

        let family = deck
            .path
            .parent()
            .ok_or_else(|| format!("{ISSUE61_LABEL} has no source family"))?;
        let metadata = fs::symlink_metadata(family)
            .map_err(|error| format!("failed to inspect {ISSUE61_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{ISSUE61_LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_ISSUE61_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family)
            .map_err(|error| format!("failed to read {ISSUE61_LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{ISSUE61_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{ISSUE61_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!("{ISSUE61_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{ISSUE61_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{ISSUE61_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(ISSUE61_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{ISSUE61_LABEL} member {name:?} content changed"));
            }
            source = Some(bytes);
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{ISSUE61_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        if self
            .root
            .join("OutputData/Certification_Tests/ISSUE_61")
            .exists()
        {
            return Err(format!(
                "{ISSUE61_LABEL} acquired invented numerical output"
            ));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{ISSUE61_LABEL} {error}"))?;
        Ok(source.expect("exact retained family contains issue61.cir"))
    }

    pub(super) fn observe_issue61_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if lines.len() != 12
            || lines.get(2).copied() != Some("B1 1 0 I={1.0e-3}")
            || lines.get(6).copied() != Some("B2 2 0 I={I(B1)*20.0}")
            || lines.get(9).map(|line| line.trim()) != Some(".tran 1ns 1us")
            || lines.get(10).map(|line| line.trim()) != Some(".print tran v(1) v(2)")
            || deck_path.file_name().and_then(|name| name.to_str()) != Some("issue61.cir")
        {
            return Err(format!("{ISSUE61_LABEL} authored failure envelope changed"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort)
                .map_err(|error| format!("{ISSUE61_LABEL} no longer parses: {error:?}"))?;
        if netlist.elements.len() != 4
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !matches!(&netlist.elements[0].kind,
                ElementKind::BehavioralCurrent { expression, .. } if expression == "1.0e-3")
            || !matches!(&netlist.elements[1].kind,
                ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                    if value.to_bits() == 0.0f64.to_bits()
                        && instance_params == &[("__RSPICE_XYCE_DEFAULT_RESISTOR_VALUE".to_string(), 1.0)]
                        && deferred_params.is_empty())
            || !matches!(&netlist.elements[2].kind,
                ElementKind::BehavioralCurrent { expression, .. }
                    if expression.eq_ignore_ascii_case("I(B1)*20.0"))
            || !matches!(&netlist.elements[3].kind,
                ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                    if value.to_bits() == 0.0f64.to_bits()
                        && instance_params == &[("__RSPICE_XYCE_DEFAULT_RESISTOR_VALUE".to_string(), 1.0)]
                        && deferred_params.is_empty())
            || !matches!(&netlist.analyses[0],
                AnalysisCommand::Tran { step, stop, start: None, max_step: None, uic: false }
                    if step.to_bits() == 1e-9f64.to_bits() && stop.to_bits() == 1e-6f64.to_bits())
        {
            return Err(format!("{ISSUE61_LABEL} typed circuit envelope changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .any(|dependency| dependency.kind != OutputSymbolKind::Node)
            || request.dependencies[0].symbol != "1"
            || request.dependencies[1].symbol != "2"
        {
            return Err(format!("{ISSUE61_LABEL} typed output request changed"));
        }

        let error = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .build_circuit_with_abort(&netlist, abort)
            .expect_err("ISSUE_61 must reject I(B1) when B1 has no MNA branch");
        let SimulationError::BehavioralReference(error) = error else {
            return Err(format!(
                "{ISSUE61_LABEL} produced the wrong typed failure: {error:?}"
            ));
        };
        if error.owner_name != "B2"
            || error.canonical_owner_name != "B2"
            || error.dependency_name != "B1"
            || error.canonical_dependency_name != "B1"
            || error.reason
                != rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable
            || !error
                .to_string()
                .starts_with(XYCE_ISSUE61_UPSTREAM_DIAGNOSTIC)
        {
            return Err(format!("{ISSUE61_LABEL} typed device identity changed"));
        }

        let repaired_source = source.replacen("B1 1 0 I={1.0e-3}", "B1 1 0 V={1.0e-3}", 1);
        let repaired = Netlist::parse_with_path_and_options_and_abort(
            &repaired_source,
            deck_path,
            options,
            abort,
        )
        .map_err(|error| format!("{ISSUE61_LABEL} repaired deck no longer parses: {error:?}"))?;
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .build_circuit_with_abort(&repaired, abort)
            .map_err(|error| format!("{ISSUE61_LABEL} repaired deck no longer builds: {error}"))?;
        Ok(())
    }

    pub(super) fn validate_issue61_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source_bytes = self.validate_issue61_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{ISSUE61_LABEL} provenance exceeded its deadline"));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("{ISSUE61_LABEL} source is not UTF-8: {error}"))?;
        Self::observe_issue61_failure(source, &deck.path, &abort)?;
        self.validate_issue61_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{ISSUE61_LABEL} post-observation provenance exceeded its deadline"
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
            path: root.join(XYCE_ISSUE61_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_ISSUE61_PATH.to_string(),
        }
    }

    fn issue61_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-issue61-{label}-"))
            .tempdir()
            .expect("create ISSUE61 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/ISSUE_61");
        fs::create_dir_all(&family).expect("create ISSUE61 family");
        fs::copy(
            corpus_root().join(XYCE_ISSUE61_PATH),
            family.join("issue61.cir"),
        )
        .expect("copy ISSUE61 source");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_ISSUE61_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write ISSUE61 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty ISSUE61 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn issue61_historical_provenance_is_exact() {
        XyceTestRunner::validate_issue61_historical_oracle_provenance()
            .expect("Release-7.10 ISSUE61 wrapper provenance remains exact");
    }

    #[test]
    fn issue61_observation_preserves_typed_branch_failure() {
        let path = corpus_root().join(XYCE_ISSUE61_PATH);
        let source = fs::read_to_string(&path).expect("read ISSUE61 source");
        XyceTestRunner::observe_issue61_failure(
            &source,
            &path,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("ISSUE61 produces its exact typed branch failure");
        assert!(
            XyceTestRunner::observe_issue61_failure(
                &source.replacen("I(B1)*20.0", "20.0", 1),
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn issue61_oracle_and_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_issue61_oracle(&deck, Instant::now())
            .expect("canonical ISSUE61 oracle qualifies");
        assert!(
            runner
                .validate_issue61_oracle(
                    &deck,
                    Instant::now()
                        - Duration::from_millis(
                            u64::try_from(runner.config.max_time_per_test_ms + 1)
                                .expect("test deadline fits Duration"),
                        ),
                )
                .is_err()
        );
    }

    #[test]
    fn issue61_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = issue61_fixture("source");
        runner
            .validate_issue61_provenance(&deck)
            .expect("canonical ISSUE61 fixture passes");
        fs::write(&deck.path, "* mutated\n").expect("mutate ISSUE61 source");
        assert!(runner.validate_issue61_provenance(&deck).is_err());

        let (_temporary, deck, runner) = issue61_fixture("manifest");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove ISSUE61 owner");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_issue61_provenance(&deck).is_err());

        let (_temporary, deck, runner) = issue61_fixture("exclusion");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_ISSUE61_PATH}\tNetlists/Certification_Tests/ISSUE_61/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate ISSUE61 exclusion");
        assert!(runner.validate_issue61_provenance(&deck).is_err());

        let (_temporary, deck, runner) = issue61_fixture("output");
        fs::create_dir_all(runner.root.join("OutputData/Certification_Tests/ISSUE_61"))
            .expect("create forbidden ISSUE61 output");
        assert!(runner.validate_issue61_provenance(&deck).is_err());
    }
}
