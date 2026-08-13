use super::*;

const BUG1116_LABEL: &str = "BUG_1116 missing-diode-model expected failure";

impl XyceTestRunner {
    pub(super) fn bug1116_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG1116_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG1116_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG1116_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug1116_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug1116_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG1116_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG1116_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG1116_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG1116_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG1116_LABEL} Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug1116_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug1116_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG1116_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG1116_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG1116_PATH))
        {
            return Err(format!(
                "recognized {BUG1116_LABEL} record '{}' is not its canonical path",
                deck.relative_path
            ));
        }

        let prefix = "netlists/certification_tests/bug_1116/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG1116_RECORD]) {
            return Err(format!(
                "{BUG1116_LABEL} requires its sole active wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG1116_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG1116_RECORD) {
            return Err(format!(
                "{BUG1116_LABEL} must not acquire an upstream exclusion row"
            ));
        }

        let family = deck
            .path
            .parent()
            .ok_or_else(|| format!("{BUG1116_LABEL} has no source family"))?;
        let metadata = fs::symlink_metadata(family)
            .map_err(|error| format!("failed to inspect {BUG1116_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG1116_LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG1116_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family)
            .map_err(|error| format!("failed to read {BUG1116_LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG1116_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG1116_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!("{BUG1116_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG1116_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG1116_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG1116_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG1116_LABEL} member {name:?} content changed"));
            }
            if name == "bug_1116.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG1116_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_1116");
        if output_family.exists() {
            return Err(format!(
                "{BUG1116_LABEL} acquired invented numerical output"
            ));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG1116_LABEL} {error}"))?;
        Ok(source.expect("exact retained family contains bug_1116.cir"))
    }

    pub(super) fn observe_bug1116_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if lines.len() != 10
            || lines.get(3).map(|line| line.trim()) != Some("D1 3 DMOD")
            || deck_path.file_name().and_then(|name| name.to_str()) != Some("bug_1116.cir")
        {
            return Err(format!(
                "{BUG1116_LABEL} authored failure location or malformed diode changed"
            ));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let error =
            match Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort)
            {
                Ok(_) => return Err(format!("{BUG1116_LABEL} unexpectedly parsed")),
                Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                    return Err(format!(
                        "{BUG1116_LABEL} parsing exceeded its bounded contract"
                    ));
                }
                Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => error,
            };
        let ParseError::MissingDeviceModel(error) = error else {
            return Err(format!(
                "{BUG1116_LABEL} produced the wrong typed failure: {error:?}"
            ));
        };
        if error.line != 4
            || error.device_name != "D1"
            || error.canonical_device_name != "D1"
            || error.device_type != "DIODE"
            || error.to_string() != XYCE_BUG1116_UPSTREAM_DIAGNOSTIC
        {
            return Err(format!("{BUG1116_LABEL} typed device identity changed"));
        }

        let repaired = source.replacen("D1 3 DMOD", "D1 3 0 DMOD", 1);
        let repaired = match Netlist::parse_with_path_and_options_and_abort(
            &repaired, deck_path, options, abort,
        ) {
            Ok(netlist) => netlist,
            Err(error) => {
                return Err(format!(
                    "{BUG1116_LABEL} repaired counterfactual no longer parses: {error:?}"
                ));
            }
        };
        if repaired.elements.len() != 4
            || repaired.models.len() != 1
            || repaired.analyses.len() != 1
            || repaired.output_requests.len() != 1
        {
            return Err(format!(
                "{BUG1116_LABEL} surrounding typed netlist envelope changed"
            ));
        }
        let diode = &repaired.elements[2];
        if diode.name != "D1"
            || diode.nodes != ["3", "0"]
            || !matches!(&diode.kind, ElementKind::Diode { model, instance_params, deferred_params }
                if model == "DMOD" && instance_params.is_empty() && deferred_params.is_empty())
            || repaired.models[0].name != "DMOD"
            || !repaired.models[0].model_type.eq_ignore_ascii_case("D")
            || !matches!(&repaired.analyses[0], AnalysisCommand::Tran { step, stop, start: None, max_step: None, uic: false }
                if step.to_bits() == 0.0f64.to_bits() && stop.to_bits() == 0.0005f64.to_bits())
        {
            return Err(format!(
                "{BUG1116_LABEL} repaired diode/model/transient semantics changed"
            ));
        }
        let request = &repaired.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.dependencies.len() != 4
            || request.dependencies[0].kind != OutputSymbolKind::Device
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("VMON")
        {
            return Err(format!("{BUG1116_LABEL} repaired print request changed"));
        }
        Ok(())
    }

    pub(super) fn validate_bug1116_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.clamp(1, 30_000));
        let source_bytes = self.validate_bug1116_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG1116_LABEL} provenance exceeded its deadline"));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("{BUG1116_LABEL} source is not UTF-8: {error}"))?;
        Self::observe_bug1116_failure(source, &deck.path, &abort)?;
        self.validate_bug1116_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1116_LABEL} post-observation provenance exceeded its deadline"
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
            path: root.join(XYCE_BUG1116_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG1116_PATH.to_string(),
        }
    }

    fn bug1116_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug1116-{label}-"))
            .tempdir()
            .expect("create BUG1116 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_1116");
        fs::create_dir_all(&family).expect("create BUG1116 family");
        for name in ["README", "bug_1116.cir", "options"] {
            fs::copy(
                corpus_root()
                    .join("Netlists/Certification_Tests/BUG_1116")
                    .join(name),
                family.join(name),
            )
            .expect("copy BUG1116 family member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG1116_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG1116 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG1116 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug1116_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug1116_historical_oracle_provenance()
            .expect("Release-7.10 BUG1116 wrapper provenance remains exact");
    }

    #[test]
    fn bug1116_observation_preserves_typed_model_failure() {
        let path = corpus_root().join(XYCE_BUG1116_PATH);
        let source = fs::read_to_string(&path).expect("read BUG1116 source");
        XyceTestRunner::observe_bug1116_failure(
            &source,
            &path,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("BUG1116 produces its exact typed model failure");
        let repaired = source.replacen("D1 3 DMOD", "D1 3 0 DMOD", 1);
        assert!(
            XyceTestRunner::observe_bug1116_failure(
                &repaired,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn bug1116_oracle_and_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_bug1116_oracle(&deck, Instant::now())
            .expect("canonical BUG1116 oracle qualifies");
        assert!(
            runner
                .validate_bug1116_oracle(&deck, Instant::now() - Duration::from_secs(31))
                .is_err()
        );
    }

    #[test]
    fn bug1116_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = bug1116_fixture("source");
        runner
            .validate_bug1116_provenance(&deck)
            .expect("canonical BUG1116 fixture passes");
        fs::write(&deck.path, "* mutated\n").expect("mutate BUG1116 source");
        assert!(runner.validate_bug1116_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1116_fixture("manifest");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove BUG1116 owner");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug1116_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1116_fixture("exclusion");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG1116_PATH}\tNetlists/Certification_Tests/BUG_1116/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG1116 exclusion");
        assert!(runner.validate_bug1116_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug1116_fixture("output");
        fs::create_dir_all(runner.root.join("OutputData/Certification_Tests/BUG_1116"))
            .expect("create forbidden BUG1116 output");
        assert!(runner.validate_bug1116_provenance(&deck).is_err());
    }
}
