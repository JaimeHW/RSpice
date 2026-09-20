use super::*;

const BUG784_LABEL: &str = "BUG_784 archived duplicate .SUBCKT port failure";

impl XyceTestRunner {
    pub(super) fn bug784_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG784_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG784_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG784_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug784_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug784_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG784_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG784_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG784_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG784_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "{BUG784_LABEL} Release-7.10 archived-wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        let tags = XYCE_BUG784_HISTORICAL_ARTIFACTS[5];
        if tags.0 != "Netlists/Certification_Tests/BUG_784/tags"
            || tags.1 != XYCE_BUG784_HISTORICAL_TAGS.len()
            || format!("{:x}", Sha256::digest(XYCE_BUG784_HISTORICAL_TAGS)) != tags.2
            || blake3::hash(XYCE_BUG784_HISTORICAL_TAGS).to_hex().as_str() != tags.3
        {
            return Err(format!(
                "{BUG784_LABEL} no longer proves the exact historical tags=exclude state"
            ));
        }
        Ok(())
    }

    fn validate_bug784_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        Self::validate_bug784_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG784_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG784_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG784_PATH))
        {
            return Err(format!(
                "recognized {BUG784_LABEL} record '{}' is not its canonical path",
                deck.relative_path
            ));
        }
        let prefix = "netlists/certification_tests/bug_784/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != BTreeSet::from([XYCE_BUG784_RECORD]) {
            return Err(format!(
                "{BUG784_LABEL} requires its sole archived wrapper owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{BUG784_LABEL} exclusions invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG784_RECORD) {
            return Err(format!(
                "{BUG784_LABEL} must not acquire an upstream exclude-sentinel row; its historical inactivity is pinned by tags=exclude"
            ));
        }

        let family = deck
            .path
            .parent()
            .ok_or_else(|| format!("{BUG784_LABEL} has no source family"))?;
        let metadata = fs::symlink_metadata(family)
            .map_err(|error| format!("failed to inspect {BUG784_LABEL} family: {error}"))?;
        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
            return Err(format!(
                "{BUG784_LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG784_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
        let mut source = None;
        for entry in fs::read_dir(family)
            .map_err(|error| format!("failed to read {BUG784_LABEL} family: {error}"))?
        {
            let entry = entry.map_err(|error| format!("failed to inspect member: {error}"))?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
            if metadata.file_type().is_symlink() || !metadata.file_type().is_file() {
                return Err(format!(
                    "{BUG784_LABEL} member {} must be a regular non-symlink file",
                    path.display()
                ));
            }
            let name = entry
                .file_name()
                .to_str()
                .ok_or_else(|| format!("{BUG784_LABEL} member name is not UTF-8"))?
                .to_string();
            let key = name.to_ascii_lowercase();
            if !observed.insert(key.clone()) {
                return Err(format!("{BUG784_LABEL} contains a case collision"));
            }
            let Some((expected_name, expected_bytes, expected_sha256, expected_blake3)) =
                expected.get(&key).copied()
            else {
                return Err(format!(
                    "{BUG784_LABEL} acquired unexpected source member {name:?}"
                ));
            };
            if name != expected_name {
                return Err(format!(
                    "{BUG784_LABEL} member case changed: expected {expected_name:?}, got {name:?}"
                ));
            }
            let bytes = fs::read(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let canonical = Self::canonical_lf_text_identity(BUG784_LABEL, &bytes)?;
            let sha256 = format!("{:x}", Sha256::digest(&canonical));
            let content_blake3 = blake3::hash(&canonical).to_hex().to_string();
            if canonical.len() != expected_bytes
                || sha256 != expected_sha256
                || content_blake3 != expected_blake3
            {
                return Err(format!("{BUG784_LABEL} member {name:?} content changed"));
            }
            if name == "bug_784.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() {
            return Err(format!(
                "{BUG784_LABEL} source census changed: expected {}, got {}",
                expected.len(),
                observed.len()
            ));
        }
        let output_family = self.root.join("OutputData/Certification_Tests/BUG_784");
        if output_family.exists() {
            return Err(format!("{BUG784_LABEL} acquired invented numerical output"));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{BUG784_LABEL} {error}"))?;
        Ok(source.expect("exact retained family contains bug_784.cir"))
    }

    pub(super) fn observe_bug784_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<DuplicateSubcircuitPortBindingError, String> {
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
                        format!("{BUG784_LABEL} parsing exceeded its bounded contract")
                    }
                    rspice_core::netlist::ParseWithAbortError::Parse(error) => {
                        format!("{BUG784_LABEL} failed before hierarchy validation: {error:?}")
                    }
                })?;
        if !netlist.title.is_empty()
            || netlist.elements.len() != 2
            || netlist.subcircuits.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
        {
            return Err(format!("{BUG784_LABEL} typed envelope changed"));
        }
        let [resistor, invocation] = netlist.elements.as_slice() else {
            unreachable!("BUG784 element count was checked")
        };
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || !invocation.name.eq_ignore_ascii_case("X1")
            || invocation.nodes != ["1", "2"]
            || !matches!(&invocation.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("SUBA") && params.is_empty())
        {
            return Err(format!("{BUG784_LABEL} top-level topology changed"));
        }
        let subcircuit = &netlist.subcircuits[0];
        if !subcircuit.name.eq_ignore_ascii_case("SUBA")
            || subcircuit.ports != ["B", "B"]
            || subcircuit.elements.len() != 1
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err(format!(
                "{BUG784_LABEL} duplicate formal definition changed"
            ));
        }
        let local = &subcircuit.elements[0];
        if !local.name.eq_ignore_ascii_case("R1")
            || local.nodes != ["B", "0"]
            || !matches!(&local.kind, ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                if value.to_bits() == 1.0f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        {
            return Err(format!("{BUG784_LABEL} subcircuit body changed"));
        }
        if !matches!(netlist.analyses.as_slice(),
            [AnalysisCommand::Tran { step, stop, start: None, max_step: None, uic: false }]
                if step.to_bits() == 1.0e-9f64.to_bits()
                    && stop.to_bits() == 1.0e-6f64.to_bits())
        {
            return Err(format!("{BUG784_LABEL} transient request changed"));
        }
        let print = &netlist.output_requests[0];
        if print.directive != OutputDirectiveKind::Print
            || print.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || print.name.is_some()
            || !print.expressions.is_empty()
            || !matches!(print.dependencies.as_slice(), [dependency]
                if dependency.kind == OutputSymbolKind::Node
                    && dependency.operator.eq_ignore_ascii_case("V")
                    && dependency.symbol.eq_ignore_ascii_case("1")
                    && !dependency.expression)
            || !matches!(netlist.saves.signals.as_slice(),
                [rspice_core::netlist::SaveSignal::Voltage(node)] if node == "1")
        {
            return Err(format!("{BUG784_LABEL} output request changed"));
        }
        let subcircuit_line = source
            .lines()
            .position(|line| line.trim().eq_ignore_ascii_case(".subckt suba  b  b"))
            .map(|index| index + 1);
        let file_name = deck_path.file_name().and_then(|name| name.to_str());
        if subcircuit_line != Some(7) || file_name != Some("bug_784.cir") {
            return Err(format!("{BUG784_LABEL} diagnostic source location changed"));
        }

        let error = match flatten_netlist_with_models_with_abort(&netlist, abort) {
            Ok(_) => return Err(format!("{BUG784_LABEL} unexpectedly flattened")),
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                return Err(format!(
                    "{BUG784_LABEL} hierarchy validation exceeded its bounded contract"
                ));
            }
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => error,
        };
        let ParseError::DuplicateSubcircuitPortBinding(error) = error else {
            return Err(format!(
                "{BUG784_LABEL} produced the wrong failure: {error:?}"
            ));
        };
        if !error.subcircuit_name.eq_ignore_ascii_case("SUBA")
            || error.canonical_subcircuit_name != "SUBA"
            || !error.instance_name.eq_ignore_ascii_case("X1")
            || error.canonical_instance_name != "X1"
            || !error.qualified_instance_name.eq_ignore_ascii_case("X1")
            || !error.formal_port.eq_ignore_ascii_case("B")
            || error.first_position != 1
            || error.conflicting_position != 2
            || error.first_actual_node != "1"
            || error.conflicting_actual_node != "2"
        {
            return Err(format!(
                "{BUG784_LABEL} typed failure identity changed: {error:?}"
            ));
        }
        let diagnostic_prefix = format!(
            "in file {} at or near line {}",
            file_name.expect("BUG784 file name was checked"),
            subcircuit_line.expect("BUG784 source line was checked")
        );
        let diagnostic = format!(
            "Duplicate node in .SUBCKT line: {}",
            error.formal_port.to_ascii_lowercase()
        );
        if diagnostic_prefix != XYCE_BUG784_UPSTREAM_DIAGNOSTIC_PREFIX
            || diagnostic != XYCE_BUG784_UPSTREAM_DIAGNOSTIC
        {
            return Err(format!(
                "{BUG784_LABEL} archived wrapper diagnostic projection changed: {diagnostic_prefix:?} / {diagnostic:?}"
            ));
        }
        Ok(*error)
    }

    pub(super) fn validate_bug784_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source_bytes = self.validate_bug784_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!("{BUG784_LABEL} provenance exceeded its deadline"));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("{BUG784_LABEL} source is not UTF-8: {error}"))?;
        Self::observe_bug784_failure(source, &deck.path, &abort)?;
        self.validate_bug784_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG784_LABEL} post-observation provenance exceeded its deadline"
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
            path: root.join(XYCE_BUG784_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG784_PATH.to_string(),
        }
    }

    fn bug784_fixture(label: &str) -> (tempfile::TempDir, XyceDeck, XyceTestRunner) {
        let temporary = tempfile::Builder::new()
            .prefix(&format!("rspice-xyce-bug784-{label}-"))
            .tempdir()
            .expect("create BUG784 fixture root");
        let root = temporary.path();
        let family = root.join("Netlists/Certification_Tests/BUG_784");
        fs::create_dir_all(&family).expect("create BUG784 family");
        let canonical = corpus_root().join("Netlists/Certification_Tests/BUG_784");
        for (name, ..) in XYCE_BUG784_RETAINED_ARTIFACTS {
            fs::copy(canonical.join(name), family.join(name)).expect("copy BUG784 member");
        }
        fs::write(
            root.join(HARNESS_MANIFEST_FILE),
            format!("{XYCE_BUG784_PATH}\t{REQUIRES_UPSTREAM_WRAPPER_CONTRACT}\n"),
        )
        .expect("write BUG784 wrapper manifest");
        fs::write(
            root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n"
            ),
        )
        .expect("write empty BUG784 exclusions");
        let deck = canonical_deck(root);
        let runner = XyceTestRunner::new(root, XyceRunnerConfig::default());
        (temporary, deck, runner)
    }

    #[test]
    fn bug784_historical_provenance_is_exact_and_inactive() {
        XyceTestRunner::validate_bug784_historical_oracle_provenance()
            .expect("Release-7.10 archived BUG784 provenance remains exact");
        assert_eq!(XYCE_BUG784_HISTORICAL_TAGS, b"exclude\n");
    }

    #[test]
    fn bug784_observation_preserves_the_typed_port_conflict() {
        let path = corpus_root().join(XYCE_BUG784_PATH);
        let source = fs::read_to_string(&path).expect("read BUG784 source");
        let error = XyceTestRunner::observe_bug784_failure(
            &source,
            &path,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("BUG784 produces its typed hierarchy failure");
        assert_eq!(error.canonical_subcircuit_name, "SUBA");
        assert_eq!(error.first_actual_node, "1");
        assert_eq!(error.conflicting_actual_node, "2");
        for mutation in [
            source.replacen("suba  b  b", "suba  b  c", 1),
            source.replacen("X1  1  2", "X1  1  1", 1),
            source.replacen(".tran 1ns 1us", ".tran 2ns 1us", 1),
        ] {
            assert!(
                XyceTestRunner::observe_bug784_failure(
                    &mutation,
                    &path,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn bug784_oracle_and_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_bug784_oracle(&deck, Instant::now())
            .expect("canonical BUG784 oracle qualifies");
        assert!(
            runner
                .validate_bug784_oracle(&deck, Instant::now() - Duration::from_secs(181))
                .is_err()
        );
    }

    #[test]
    fn bug784_provenance_rejects_source_manifest_exclusion_and_output_drift() {
        let (_temporary, deck, runner) = bug784_fixture("source");
        runner
            .validate_bug784_provenance(&deck)
            .expect("canonical BUG784 fixture passes");
        fs::write(&deck.path, "* mutated\n").expect("mutate BUG784 source");
        assert!(runner.validate_bug784_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug784_fixture("manifest");
        fs::write(runner.root.join(HARNESS_MANIFEST_FILE), "").expect("remove BUG784 owner");
        let rebuilt = XyceTestRunner::new(&runner.root, XyceRunnerConfig::default());
        assert!(rebuilt.validate_bug784_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug784_fixture("exclusion");
        fs::write(
            runner.root.join(UPSTREAM_EXCLUSIONS_MANIFEST_FILE),
            format!(
                "schema_version\t{UPSTREAM_EXCLUSIONS_SCHEMA_VERSION}\nsource_commit\t{UPSTREAM_EXCLUSIONS_SOURCE_COMMIT}\nsource_netlists_tree\t{UPSTREAM_EXCLUSIONS_SOURCE_NETLISTS_TREE}\n{XYCE_BUG784_PATH}\tNetlists/Certification_Tests/BUG_784/exclude\t{UPSTREAM_EXCLUDED_DISPOSITION}\n"
            ),
        )
        .expect("fabricate BUG784 exclusion");
        assert!(runner.validate_bug784_provenance(&deck).is_err());

        let (_temporary, deck, runner) = bug784_fixture("output");
        fs::create_dir_all(runner.root.join("OutputData/Certification_Tests/BUG_784"))
            .expect("create forbidden BUG784 output");
        assert!(runner.validate_bug784_provenance(&deck).is_err());
    }
}
