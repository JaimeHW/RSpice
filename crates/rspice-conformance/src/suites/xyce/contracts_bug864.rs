use super::*;

impl XyceTestRunner {
    pub(super) fn bug864_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG864_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG864_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG864_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug864_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug864_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG864_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG864_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG864_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG864_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_864_SON Release-7.10 wrapper/error-policy provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug864_provenance(&self, deck: &XyceDeck) -> Result<Vec<u8>, String> {
        const LABEL: &str = "BUG_864_SON unresolved subcircuit parameter";
        Self::validate_bug864_historical_oracle_provenance()?;
        if deck.section != XyceDeckSection::Netlists
            || Self::normalize_manifest_key(&deck.relative_path) != XYCE_BUG864_RECORD
            || Self::normalize_manifest_key(&self.relative_key(&deck.path)) != XYCE_BUG864_RECORD
            || !Self::same_path(&deck.path, &self.root.join(XYCE_BUG864_PATH))
        {
            return Err(format!(
                "recognized {LABEL} record '{}' is not backed by its exact canonical Netlists path",
                deck.relative_path
            ));
        }
        if !self.requires_upstream_wrapper(&deck.relative_path) {
            return Err(format!("{LABEL} lost removed-wrapper ownership"));
        }
        let family_prefix = "netlists/certification_tests/bug_864_son/";
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(family_prefix))
            .collect::<Vec<_>>();
        if owners.len() != 1 || owners[0].as_str() != XYCE_BUG864_RECORD {
            return Err(format!(
                "{LABEL} requires its exact single manifest owner, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if exclusions.contains_key(XYCE_BUG864_RECORD) {
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
        let expected = XYCE_BUG864_RETAINED_ARTIFACTS
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
            if name == "bug_864_son.cir" {
                source = Some(bytes);
            }
        }
        if observed.len() != expected.len() || observed.len() != 2 {
            return Err(format!(
                "{LABEL} retained family census changed: expected 2 members, got {}",
                observed.len()
            ));
        }
        let option_bytes = fs::read(family_dir.join("options"))
            .map_err(|error| format!("failed to re-read {LABEL} options: {error}"))?;
        let canonical_options = Self::canonical_lf_text_identity(LABEL, &option_bytes)?;
        if canonical_options != b"timelimit=30\n" {
            return Err(format!(
                "{LABEL} options no longer binds the historical 30-second outer timeout"
            ));
        }
        self.reject_wrapper_output_artifacts(&deck.path)
            .map_err(|error| format!("{LABEL} {error}"))?;
        Ok(source.expect("exact retained family includes the source record"))
    }

    pub(super) fn observe_bug864_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<UnresolvedSubcircuitParameterError, String> {
        const LABEL: &str = "BUG_864_SON unresolved subcircuit parameter";
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
                        format!("{LABEL} failed before hierarchy resolution: {error:?}")
                    }
                })?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 3
            || netlist.subcircuits.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
        {
            return Err(format!(
                "{LABEL} typed envelope changed: diagnostics={}, elements={}, subcircuits={}, analyses={}, outputs={}",
                netlist.diagnostics.len(),
                netlist.elements.len(),
                netlist.subcircuits.len(),
                netlist.analyses.len(),
                netlist.output_requests.len()
            ));
        }
        let subcircuit = &netlist.subcircuits[0];
        if !subcircuit.name.eq_ignore_ascii_case("MYMOS")
            || subcircuit.ports != ["1", "2", "3", "4"]
            || subcircuit.body_expr_params != [("FOO".to_string(), "(meh != 1)".to_string())]
            || subcircuit.elements.len() != 3
        {
            return Err(format!(
                "{LABEL} no longer retains the exact MYMOS local FOO=(MEH != 1) definition"
            ));
        }
        let invocation = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("XM1"))
            .ok_or_else(|| format!("{LABEL} lost the XM1 hierarchy invocation"))?;
        if invocation.nodes != ["D", "G", "0", "0"]
            || !matches!(&invocation.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("MYMOS") && params.len() == 2)
        {
            return Err(format!("{LABEL} XM1 invocation topology changed"));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc { source, start, stop, step, mode: DcSweepMode::Linear, sweep2: None }
            if source.eq_ignore_ascii_case("VG")
                && start.to_bits() == 0.0f64.to_bits()
                && stop.to_bits() == 1.8f64.to_bits()
                && step.to_bits() == 0.01f64.to_bits())
        {
            return Err(format!("{LABEL} exact DC tuple changed"));
        }
        let print = &netlist.output_requests[0];
        if print.directive != OutputDirectiveKind::Print
            || print.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || print.name.is_some()
            || !matches!(
                print.dependencies.as_slice(),
                [dependency]
                    if dependency.operator.eq_ignore_ascii_case("I")
                        && dependency.symbol.eq_ignore_ascii_case("VD")
                        && dependency.kind == OutputSymbolKind::Device
                        && !dependency.expression
            )
            || !matches!(
                netlist.saves.signals.as_slice(),
                [rspice_core::netlist::SaveSignal::Current(device)]
                    if device.eq_ignore_ascii_case("VD")
            )
        {
            return Err(format!("{LABEL} exact .PRINT DC I(VD) contract changed"));
        }

        let error = match flatten_netlist_with_models_with_abort(&netlist, abort) {
            Ok(_) => {
                return Err(format!(
                    "{LABEL} unexpectedly completed hierarchy expansion"
                ));
            }
            Err(rspice_core::netlist::ParseWithAbortError::Aborted) => {
                return Err(format!(
                    "{LABEL} hierarchy resolution exceeded its bounded execution contract"
                ));
            }
            Err(rspice_core::netlist::ParseWithAbortError::Parse(error)) => error,
        };
        let ParseError::UnresolvedSubcircuitParameter(error) = error else {
            return Err(format!(
                "{LABEL} produced the wrong typed hierarchy failure: {error:?}"
            ));
        };
        if !error.subcircuit_name.eq_ignore_ascii_case("MYMOS")
            || error.canonical_subcircuit_name != "MYMOS"
            || !error.instance_name.eq_ignore_ascii_case("XM1")
            || error.canonical_instance_name != "XM1"
            || !error.qualified_instance_name.eq_ignore_ascii_case("XM1")
            || error.parameter_name != "FOO"
            || error.canonical_parameter_name != "FOO"
            || error.expression != "(meh != 1)"
            || error.missing_dependency.as_deref() != Some("MEH")
            || !error.reason.contains("Undefined parameter: MEH")
            || !error.to_string().contains(XYCE_BUG864_UPSTREAM_DIAGNOSTIC)
        {
            return Err(format!("{LABEL} typed failure identity changed: {error:?}"));
        }
        Ok(*error)
    }

    pub(super) fn validate_bug864_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let timeout_ms = self
            .config
            .max_time_per_test_ms
            .clamp(1, XYCE_BUG864_HISTORICAL_TIMEOUT_MS);
        let abort = DeadlineAbort::new(start, timeout_ms);
        let source_bytes = self.validate_bug864_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "BUG_864_SON provenance exceeded the bounded {timeout_ms}ms contract"
            ));
        }
        let source = std::str::from_utf8(&source_bytes)
            .map_err(|error| format!("BUG_864_SON source is not UTF-8: {error}"))?;
        Self::observe_bug864_failure(source, &deck.path, &abort)?;
        self.validate_bug864_provenance(deck)?;
        if abort.is_aborted() {
            return Err(format!(
                "BUG_864_SON post-execution provenance exceeded the bounded {timeout_ms}ms contract"
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

    #[test]
    fn bug864_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug864_historical_oracle_provenance()
            .expect("Release-7.10 BUG864 provenance remains exact");
    }

    #[test]
    fn bug864_observation_preserves_failed_definition_and_missing_dependency() {
        let path = corpus_root().join(XYCE_BUG864_PATH);
        let source = fs::read_to_string(&path).expect("read canonical BUG864 source");
        let error = XyceTestRunner::observe_bug864_failure(
            &source,
            &path,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("BUG864 produces the typed bounded hierarchy failure");
        assert_eq!(error.canonical_parameter_name, "FOO");
        assert_eq!(error.missing_dependency.as_deref(), Some("MEH"));

        for mutated in [
            source.replace(".param foo = '(meh != 1)'", ".param foo = '(meh == 1)'"),
            source.replace(".param foo = '(meh != 1)'", ".param foo = 0"),
            source.replace("xM1 d g 0 0 mymos", "xM1 d g 0 1 mymos"),
        ] {
            assert!(
                XyceTestRunner::observe_bug864_failure(
                    &mutated,
                    &path,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err(),
                "semantic mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug864_oracle_rejects_an_expired_cooperative_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = XyceDeck {
            path: root.join(XYCE_BUG864_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG864_PATH.to_string(),
        };
        let expired_start = Instant::now() - Duration::from_secs(31);
        let error = runner
            .validate_bug864_oracle(&deck, expired_start)
            .expect_err("an expired BUG864 deadline must fail before hierarchy completion");
        assert!(
            error.contains("bounded"),
            "unexpected deadline error: {error}"
        );

        let source = fs::read_to_string(&deck.path).expect("read canonical BUG864 source");
        let error = XyceTestRunner::observe_bug864_failure(
            &source,
            &deck.path,
            &rspice_core::abort_signal::ImmediateAbort,
        )
        .expect_err("the BUG864 parser path must directly observe cancellation");
        assert!(error.contains("bounded"), "unexpected abort error: {error}");
    }
}
