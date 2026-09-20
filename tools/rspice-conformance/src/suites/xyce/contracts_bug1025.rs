use super::*;
use crate::suites::execution::ExecutionOutcome;

impl XyceTestRunner {
    pub(super) fn bug1025_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1025_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1025_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1025_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1025_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1025_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1025_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1025_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1025_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1025_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_1025 Release-7.10 no-analysis wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug1025_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        const LABEL: &str = "BUG_1025 bounded no-analysis error";
        Self::validate_bug1025_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG1025_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG1025_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG1025_PATH))
        {
            return Err(format!(
                "recognized {LABEL} record '{}' is not backed by its exact canonical Netlists path",
                deck.relative_path
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!("{LABEL} lost removed-wrapper ownership"));
        }
        let family_prefix = "netlists/certification_tests/bug_1025/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG1025_RECORD {
            return Err(format!(
                "{LABEL} requires its exact single manifest owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1025_RECORD) {
            return Err(format!(
                "{LABEL} must not be classified by an upstream exclude sentinel"
            ));
        }

        let family_dir = deck
            .path
            .parent()
            .ok_or_else(|| format!("{LABEL} record has no family directory"))?;
        let family_metadata = fs::symlink_metadata(family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if !family_metadata.file_type().is_dir() || family_metadata.file_type().is_symlink() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1025_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family_dir)
            .map_err(|error| format!("failed to read {LABEL} family: {error}"))?
        {
            let entry =
                entry.map_err(|error| format!("failed to inspect {LABEL} member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if !metadata.file_type().is_file() || metadata.file_type().is_symlink() {
                return Err(format!(
                    "{LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!(
                    "{LABEL} family has a case-colliding member {name:?}"
                ));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{LABEL} family acquired unexpected member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {LABEL} member {name:?}: {error}"))?;
            let canonical = Self::canonical_lf_text_identity(LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!(
                    "{LABEL} retained member {name:?} changed: expected {expected_bytes}/{expected_sha256}/{expected_blake3}, got {}/{sha256}/{content_blake3}",
                    canonical.len()
                ));
            }
            if name == "null.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() || observed.len() != 3 {
            return Err(format!(
                "{LABEL} retained family census changed: expected 3 members, got {}",
                observed.len()
            ));
        }
        let canonical_options = Self::canonical_lf_text_identity(
            LABEL,
            &fs::read(family_dir.join("options"))
                .map_err(|error| format!("failed to re-read {LABEL} options: {error}"))?,
        )?;
        if canonical_options != b"timelimit=30\n" {
            return Err(format!(
                "{LABEL} options no longer binds the historical 30-second outer timeout"
            ));
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_1025");
        if output_family.exists() {
            return Err(format!(
                "{LABEL} acquired an invented numerical output family at {}",
                output_family.display()
            ));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{LABEL} {error}"))?;
        Ok(source.expect("exact retained family includes the source record"))
    }

    pub(super) fn observe_bug1025_no_analysis(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(ExecutionOutcome, &'static str), String> {
        const LABEL: &str = "BUG_1025 bounded no-analysis error";
        if !source.is_empty() {
            return Err(format!("{LABEL} source must remain exactly zero bytes"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort)
                .map_err(|error| match error {
                    rspice_core::netlist::ParseWithAbortError::Aborted => {
                        format!("{LABEL} parsing exceeded its bounded execution contract")
                    }
                    rspice_core::netlist::ParseWithAbortError::Parse(error) => {
                        format!("{LABEL} produced an unrelated parser failure: {error:?}")
                    }
                })?;
        if !netlist.title.is_empty()
            || !netlist.elements.is_empty()
            || !netlist.analyses.is_empty()
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.saves.signals.is_empty()
            || !netlist.output_requests.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} no longer parses to an exact empty no-analysis netlist"
            ));
        }
        let outcome = ExecutionOutcome::NoAnalysis;
        if outcome.summary() != "parsed, requests no analysis"
            || !"No analysis specified.".contains(XYCE_BUG1025_UPSTREAM_DIAGNOSTIC)
        {
            return Err(format!("{LABEL} canonical outcome/diagnostic changed"));
        }
        Ok((outcome, "No analysis specified."))
    }

    pub(super) fn validate_bug1025_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let timeout_ms = self
            .config
            .max_time_per_test_ms
            .clamp(1, XYCE_BUG1025_HISTORICAL_TIMEOUT_MS);
        let abort = DeadlineAbort::new(start, timeout_ms);
        let source_bytes = self.validate_bug1025_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "BUG_1025 provenance exceeded the bounded {timeout_ms}ms contract"
            ));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("BUG_1025 source is not UTF-8: {error}"))?;
        let (outcome, diagnostic) = Self::observe_bug1025_no_analysis(source, &deck.path, &abort)?;
        if outcome != ExecutionOutcome::NoAnalysis
            || !diagnostic.contains(XYCE_BUG1025_UPSTREAM_DIAGNOSTIC)
        {
            return Err(format!(
                "BUG_1025 produced the wrong bounded observation: {outcome:?} / {diagnostic:?}"
            ));
        }
        self.validate_bug1025_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "BUG_1025 post-observation provenance exceeded the bounded {timeout_ms}ms contract"
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
            path: root.join(XYCE_BUG1025_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1025_PATH.to_string(),
        }
    }

    fn bug1025_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let source_root = corpus_root();
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1025-{label}-"))
            .tempdir()
            .expect("create BUG1025 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_1025");
        fs::create_dir_all(&family).expect("create BUG1025 fixture family");
        for name in ["README", "null.cir", "options"] {
            fs::copy(
                source_root
                    .join("Netlists/Certification_Tests/BUG_1025")
                    .join(name),
                family.join(name),
            )
            .expect("copy canonical BUG1025 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1025_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1025 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG1025 exclusion manifest");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1025_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1025_historical_oracle_provenance()
            .expect("Release-7.10 BUG1025 provenance remains exact");
    }

    #[test]
    fn bug1025_observation_is_exactly_empty_no_analysis() {
        let path = corpus_root().join(XYCE_BUG1025_PATH);
        let (outcome, diagnostic) = XyceTestRunner::observe_bug1025_no_analysis(
            "",
            &path,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("empty BUG1025 deck produces the typed no-analysis outcome");
        assert_eq!(outcome, ExecutionOutcome::NoAnalysis);
        assert_eq!(diagnostic, "No analysis specified.");
        assert!(
            XyceTestRunner::observe_bug1025_no_analysis(
                "* still no analysis\n",
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err(),
            "a merely no-analysis deck must not substitute for the zero-byte historical record"
        );
    }

    #[test]
    fn bug1025_canonical_oracle_and_expired_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_bug1025_oracle(&deck, Instant::now())
            .expect("canonical BUG1025 oracle qualifies");
        let error = runner
            .validate_bug1025_oracle(&deck, Instant::now() - Duration::from_secs(31))
            .expect_err("an expired BUG1025 deadline must fail closed");
        assert!(
            error.contains("bounded"),
            "unexpected deadline error: {error}"
        );
    }

    #[test]
    fn bug1025_provenance_rejects_family_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = bug1025_fixture("source-drift");
        fs::write(&deck.path, "* no longer empty\n").expect("mutate BUG1025 source");
        assert!(runner.validate_bug1025_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1025_fixture("family-drift");
        runner
            .validate_bug1025_provenance(&deck)
            .expect("canonical BUG1025 fixture provenance passes");
        fs::write(
            deck.path
                .parent()
                .expect("BUG1025 deck has parent")
                .join("unexpected.err"),
            "stale wrapper output\n",
        )
        .expect("write stale BUG1025 wrapper output");
        assert!(runner.validate_bug1025_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1025_fixture("output-drift");
        let output = runner
            .static_output_reference_path(&deck.path, "prn")
            .expect("map BUG1025 fixture to OutputData");
        fs::create_dir_all(output.parent().expect("BUG1025 output has parent"))
            .expect("create forbidden BUG1025 OutputData family");
        fs::write(output, "invented gold\n").expect("write forbidden BUG1025 gold");
        assert!(runner.validate_bug1025_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1025_fixture("manifest-drift");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "")
            .expect("remove BUG1025 wrapper ownership");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug1025_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1025_fixture("exclusion-drift");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG1025_PATH}\tNetlists/Certification_Tests/BUG_1025/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG1025 exclusion");
        assert!(runner.validate_bug1025_provenance(&deck).is_err());
    }
}
