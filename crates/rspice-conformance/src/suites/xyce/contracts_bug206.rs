use super::*;

const BUG206_LABEL: &str = "BUG_206 undefined-subcircuit expected failure";

impl XyceTestRunner {
    pub(super) fn bug206_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG206_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG206_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG206_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug206_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug206_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG206_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG206_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG206_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG206_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG206_LABEL} Release-7.10 wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    fn validate_bug206_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug206_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG206_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG206_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG206_PATH))
        {
            return Err(format!(
                "recognized {BUG206_LABEL} record '{}' is not its canonical path",
                deck.relative_path
            ));
        }

        let prefix = "netlists/certification_tests/bug_206/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG206_RECORD]) {
            return Err(format!(
                "{BUG206_LABEL} requires its sole active wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG206_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG206_RECORD) {
            return Err(format!(
                "{BUG206_LABEL} must not acquire an upstream exclusion row"
            ));
        }

        let family = deck
            .path
            .parent()
            .ok_or_else(|| format!("{BUG206_LABEL} has no source family"))?;
        let metadata = fs::symlink_metadata(family)
            .map_err(|error| format!("failed to inspect {BUG206_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG206_LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG206_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family)
            .map_err(|error| format!("failed to read {BUG206_LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG206_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG206_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!("{BUG206_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG206_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG206_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG206_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG206_LABEL} member {name:?} content changed"));
            }
            if name == "bug_206.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG206_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_206");
        if output_family.exists() {
            return Err(format!("{BUG206_LABEL} acquired invented numerical output"));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG206_LABEL} {error}"))?;
        Ok(source.expect("exact retained family contains bug_206.cir"))
    }

    pub(super) fn observe_bug206_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if lines.len() != 10
            || lines.get(2).map(|line| line.trim_end()) != Some("X1 1 0")
            || deck_path.file_name().and_then(|name| name.to_str()) != Some("bug_206.cir")
        {
            return Err(format!(
                "{BUG206_LABEL} authored failure location or malformed instance changed"
            ));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let netlist =
            match Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort)
            {
                Ok(netlist) => netlist,
                Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                    return Err(format!(
                        "{BUG206_LABEL} parsing exceeded its bounded contract"
                    ));
                }
                Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => {
                    return Err(format!(
                        "{BUG206_LABEL} failed before hierarchy resolution: {error}"
                    ));
                }
            };

        if netlist.title != "Testing paramters"
            || netlist.elements.len() != 2
            || netlist.subcircuits.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
        {
            return Err(format!("{BUG206_LABEL} typed netlist envelope changed"));
        }
        let x1 = &netlist.elements[0];
        let ElementKind::Subcircuit {
            subckt_name,
            params,
        } = &x1.kind
        else {
            return Err(format!(
                "{BUG206_LABEL} X1 is no longer a subcircuit instance"
            ));
        };
        if x1.name != "X1" || x1.nodes != ["1"] || subckt_name != "0" || !params.is_empty() {
            return Err(format!("{BUG206_LABEL} malformed X1 identity changed"));
        }
        let v1 = &netlist.elements[1];
        if v1.name != "V1"
            || v1.nodes != ["1", "0"]
            || !matches!(v1.kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{BUG206_LABEL} voltage-source topology changed"));
        }
        let subckt = &netlist.subcircuits[0];
        if subckt.name != "foobie"
            || subckt.ports != ["1", "2"]
            || subckt.elements.len() != 2
            || !subckt.elements.iter().all(|element| {
                matches!(
                    &element.kind,
                    ElementKind::Resistor {
                        value,
                        value_expr: None,
                        model: None,
                        instance_params,
                        deferred_params,
                    } if value.to_bits() == 100.0f64.to_bits()
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                )
            })
        {
            return Err(format!("{BUG206_LABEL} valid FOOBIE definition changed"));
        }
        if !matches!(
            &netlist.analyses[0],
            AnalysisCommand::Dc { source, start, stop, step, mode: DcSweepMode::Linear, sweep2: None }
                if source.eq_ignore_ascii_case("V1")
                    && start.to_bits() == 1.0f64.to_bits()
                    && stop.to_bits() == 1.0f64.to_bits()
                    && step.to_bits() == 1.0f64.to_bits()
        ) {
            return Err(format!("{BUG206_LABEL} DC analysis changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Device
            || !request.dependencies[0].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("V1")
        {
            return Err(format!("{BUG206_LABEL} print request changed"));
        }

        let error = match flatten_netlist_with_models_with_abort(&netlist, abort) {
            Ok(_) => return Err(format!("{BUG206_LABEL} unexpectedly flattened")),
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                return Err(format!(
                    "{BUG206_LABEL} hierarchy resolution exceeded its bounded contract"
                ));
            }
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => error,
        };
        let ParseError::UndefinedSubcircuit(error) = error else {
            return Err(format!(
                "{BUG206_LABEL} produced the wrong typed failure: {error:?}"
            ));
        };
        if error.subcircuit_name != "0"
            || error.canonical_subcircuit_name != "0"
            || error.instance_name != "X1"
            || error.canonical_instance_name != "X1"
            || error.qualified_instance_name != "X1"
            || error.to_string() != XYCE_BUG206_UPSTREAM_DIAGNOSTIC_UNDEFINED
        {
            return Err(format!("{BUG206_LABEL} typed hierarchy identity changed"));
        }
        let diagnostics = format!("{XYCE_BUG206_UPSTREAM_DIAGNOSTIC_LOCATION}\n{}", error);
        let first = diagnostics
            .find(XYCE_BUG206_UPSTREAM_DIAGNOSTIC_LOCATION)
            .ok_or_else(|| format!("{BUG206_LABEL} lost the wrapper's source diagnostic"))?;
        let second = diagnostics
            .find(XYCE_BUG206_UPSTREAM_DIAGNOSTIC_UNDEFINED)
            .ok_or_else(|| format!("{BUG206_LABEL} lost the wrapper's hierarchy diagnostic"))?;
        if first >= second {
            return Err(format!(
                "{BUG206_LABEL} wrapper diagnostics are no longer ordered"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug206_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source_bytes = self.validate_bug206_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG206_LABEL} provenance exceeded its deadline"));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("{BUG206_LABEL} source is not UTF-8: {error}"))?;
        Self::observe_bug206_failure(source, &deck.path, &abort)?;
        self.validate_bug206_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG206_LABEL} post-observation provenance exceeded its deadline"
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
            path: root.join(XYCE_BUG206_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG206_PATH.to_string(),
        }
    }

    fn bug206_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug206-{label}-"))
            .tempdir()
            .expect("create BUG206 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_206");
        fs::create_dir_all(&family).expect("create BUG206 family");
        for name in ["README", "bug_206.cir"] {
            fs::copy(
                corpus_root()
                    .join("Netlists/Certification_Tests/BUG_206")
                    .join(name),
                family.join(name),
            )
            .expect("copy BUG206 family member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG206_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG206 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG206 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug206_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug206_historical_oracle_provenance()
            .expect("Release-7.10 BUG206 wrapper provenance remains exact");
    }

    #[test]
    fn bug206_observation_preserves_typed_hierarchy_failure() {
        let path = corpus_root().join(XYCE_BUG206_PATH);
        let source = fs::read_to_string(&path).expect("read BUG206 source");
        XyceTestRunner::observe_bug206_failure(&source, &path, &rspice_core::abort_signal::NoAbort)
            .expect("BUG206 produces its exact typed hierarchy failure");
        let corrected = source.replacen("X1 1 0 ", "X1 1 0 foobie", 1);
        assert!(
            XyceTestRunner::observe_bug206_failure(
                &corrected,
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn bug206_oracle_and_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_bug206_oracle(&deck, Instant::now())
            .expect("canonical BUG206 oracle qualifies");
        assert!(
            runner
                .validate_bug206_oracle(&deck, Instant::now() - Duration::from_secs(181))
                .is_err()
        );
    }

    #[test]
    fn bug206_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = bug206_fixture("source");
        runner
            .validate_bug206_provenance(&deck)
            .expect("canonical BUG206 fixture passes");
        fs::write(&deck.path, "* mutated\n").expect("mutate BUG206 source");
        assert!(runner.validate_bug206_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug206_fixture("manifest");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove BUG206 owner");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug206_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug206_fixture("exclusion");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG206_PATH}\tNetlists/Certification_Tests/BUG_206/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG206 exclusion");
        assert!(runner.validate_bug206_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug206_fixture("output");
        fs::create_dir_all(runner.root.join("OutputData/Certification_Tests/BUG_206"))
            .expect("create forbidden BUG206 output");
        assert!(runner.validate_bug206_provenance(&deck).is_err());
    }
}
