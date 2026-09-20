use super::*;

const BUG636_LABEL: &str = "BUG_636_SON incomplete .TRAN expected failure";
const BUG636_PARSE_MESSAGE: &str =
    ".TRAN line has an unexpected number of fields\nUnrecognized dot line will be ignored";

impl XyceTestRunner {
    pub(super) fn bug636_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG636_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG636_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG636_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug636_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug636_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG636_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG636_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG636_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG636_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG636_LABEL} Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        if BUG636_PARSE_MESSAGE
            != format!(
                "{XYCE_BUG636_UPSTREAM_DIAGNOSTIC_ARITY}\n{XYCE_BUG636_UPSTREAM_DIAGNOSTIC_IGNORED}"
            )
        {
            return Err(format!(
                "{BUG636_LABEL} no longer preserves the wrapper's ordered diagnostic policy"
            ));
        }
        Ok(())
    }

    fn validate_bug636_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug636_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG636_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG636_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG636_PATH))
        {
            return Err(format!(
                "recognized {BUG636_LABEL} record '{}' is not its canonical path",
                deck.relative_path
            ));
        }

        let prefix = "netlists/certification_tests/bug_636_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG636_RECORD]) {
            return Err(format!(
                "{BUG636_LABEL} requires its sole active wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG636_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG636_RECORD) {
            return Err(format!(
                "{BUG636_LABEL} must not acquire an upstream exclusion row"
            ));
        }

        let family = deck
            .path
            .parent()
            .ok_or_else(|| format!("{BUG636_LABEL} has no source family"))?;
        let metadata = fs::symlink_metadata(family)
            .map_err(|error| format!("failed to inspect {BUG636_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG636_LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG636_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family)
            .map_err(|error| format!("failed to read {BUG636_LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG636_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG636_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!("{BUG636_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG636_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG636_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG636_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG636_LABEL} member {name:?} content changed"));
            }
            if name == "bug636.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG636_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_636_SON");
        if output_family.exists() {
            return Err(format!("{BUG636_LABEL} acquired invented numerical output"));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG636_LABEL} {error}"))?;
        Ok(source.expect("exact retained family contains bug636.cir"))
    }

    pub(super) fn observe_bug636_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let tran_line = source.lines().nth(9).map(str::trim);
        if source.lines().count() != 12
            || tran_line != Some(".tran 1u")
            || deck_path.file_name().and_then(|name| name.to_str()) != Some("bug636.cir")
        {
            return Err(format!(
                "{BUG636_LABEL} authored failure location or incomplete analysis changed"
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
                Ok(_) => return Err(format!("{BUG636_LABEL} unexpectedly parsed")),
                Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                    return Err(format!(
                        "{BUG636_LABEL} parsing exceeded its bounded contract"
                    ));
                }
                Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => error,
            };
        let ParseError::Syntax { line, message } = error else {
            return Err(format!(
                "{BUG636_LABEL} produced the wrong typed failure: {error:?}"
            ));
        };
        if line != 10 || message != BUG636_PARSE_MESSAGE {
            return Err(format!(
                "{BUG636_LABEL} parse identity changed: line={line}, message={message:?}"
            ));
        }
        let first = message
            .find(XYCE_BUG636_UPSTREAM_DIAGNOSTIC_ARITY)
            .ok_or_else(|| format!("{BUG636_LABEL} lost the wrapper's arity diagnostic"))?;
        let second = message
            .find(XYCE_BUG636_UPSTREAM_DIAGNOSTIC_IGNORED)
            .ok_or_else(|| format!("{BUG636_LABEL} lost the wrapper's ignored-line diagnostic"))?;
        if first >= second {
            return Err(format!(
                "{BUG636_LABEL} wrapper diagnostics are no longer ordered"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug636_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source_bytes = self.validate_bug636_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG636_LABEL} provenance exceeded its deadline"));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("{BUG636_LABEL} source is not UTF-8: {error}"))?;
        Self::observe_bug636_failure(source, &deck.path, &abort)?;
        self.validate_bug636_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG636_LABEL} post-observation provenance exceeded its deadline"
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
            path: root.join(XYCE_BUG636_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG636_PATH.to_string(),
        }
    }

    fn bug636_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug636-{label}-"))
            .tempdir()
            .expect("create BUG636 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_636_SON");
        fs::create_dir_all(&family).expect("create BUG636 family");
        fs::copy(
            corpus_root().join(XYCE_BUG636_PATH),
            family.join("bug636.cir"),
        )
        .expect("copy BUG636 deck");
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG636_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG636 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG636 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug636_historical_provenance_and_policy_are_exact() {
        XyceTestRunner::validate_bug636_historical_oracle_provenance()
            .expect("Release-7.10 BUG636 wrapper provenance remains exact");
        assert_eq!(
            BUG636_PARSE_MESSAGE,
            format!(
                "{XYCE_BUG636_UPSTREAM_DIAGNOSTIC_ARITY}\n{XYCE_BUG636_UPSTREAM_DIAGNOSTIC_IGNORED}"
            )
        );
    }

    #[test]
    fn bug636_observation_preserves_the_ordered_parse_failure() {
        let path = corpus_root().join(XYCE_BUG636_PATH);
        let source = fs::read_to_string(&path).expect("read BUG636 source");
        XyceTestRunner::observe_bug636_failure(&source, &path, &rspice_core::abort_signal::NoAbort)
            .expect("BUG636 produces its exact parse failure");
        for mutation in [
            source.replacen(".tran 1u ", ".tran 1u 1m", 1),
            source.replacen(".tran 1u ", ".tran 2u ", 1),
        ] {
            assert!(
                XyceTestRunner::observe_bug636_failure(
                    &mutation,
                    &path,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn bug636_oracle_and_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_bug636_oracle(&deck, Instant::now())
            .expect("canonical BUG636 oracle qualifies");
        assert!(
            runner
                .validate_bug636_oracle(&deck, Instant::now() - Duration::from_secs(181))
                .is_err()
        );
    }

    #[test]
    fn bug636_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = bug636_fixture("source");
        runner
            .validate_bug636_provenance(&deck)
            .expect("canonical BUG636 fixture passes");
        fs::write(&deck.path, "* mutated\n").expect("mutate BUG636 source");
        assert!(runner.validate_bug636_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug636_fixture("manifest");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove BUG636 owner");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug636_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug636_fixture("exclusion");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG636_PATH}\tNetlists/Certification_Tests/BUG_636_SON/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG636 exclusion");
        assert!(runner.validate_bug636_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug636_fixture("output");
        fs::create_dir_all(
            runner
                .root
                .join("OutputData/Certification_Tests/BUG_636_SON"),
        )
        .expect("create forbidden BUG636 output");
        assert!(runner.validate_bug636_provenance(&deck).is_err());
    }
}
