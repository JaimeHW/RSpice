use super::*;

impl XyceTestRunner {
    pub(super) fn bug354_historical_oracle_provenance_records() -> Vec<String> {
        let mut records = XYCE_BUG354_HISTORICAL_ARTIFACTS
            .into_iter()
            .map(|(path, bytes, sha256, content_blake3)| {
                format!(
                    "{XYCE_BUG354_UPSTREAM_REGRESSION_COMMIT}\t{XYCE_BUG354_UPSTREAM_RELEASE_TAG}\t{path}\t{bytes}\t{sha256}\t{content_blake3}"
                )
            })
            .collect::<Vec<_>>();
        records.sort();
        records
    }

    pub(super) fn validate_bug354_historical_oracle_provenance() -> Result<(), String> {
        let records = Self::bug354_historical_oracle_provenance_records();
        let stream = records.join("\n");
        let sha256 = format!("{:x}", Sha256::digest(stream.as_bytes()));
        let content_blake3 = blake3::hash(stream.as_bytes()).to_hex().to_string();
        if records.len() != XYCE_BUG354_HISTORICAL_RECORD_COUNT
            || stream.len() != XYCE_BUG354_HISTORICAL_RECORD_BYTES
            || sha256 != XYCE_BUG354_HISTORICAL_RECORDS_SHA256
            || content_blake3 != XYCE_BUG354_HISTORICAL_RECORDS_BLAKE3
        {
            return Err(format!(
                "BUG_354_SON Release-7.10 nonzero-exit wrapper provenance changed: records={}/{}, sha256={sha256}, blake3={content_blake3}",
                records.len(),
                stream.len()
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug354_complete_family_provenance(
        &self,
        family_dir: &Path,
    ) -> Result<(), String> {
        const LABEL: &str = "BUG_354_SON expected output-validation failures";
        Self::validate_bug354_historical_oracle_provenance()?;
        let metadata = fs::symlink_metadata(family_dir)
            .map_err(|error| format!("failed to inspect {LABEL} family: {error}"))?;
        if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
            return Err(format!(
                "{LABEL} family must be a regular non-symlink directory"
            ));
        }
        let expected = XYCE_BUG354_RETAINED_ARTIFACTS
            .into_iter()
            .map(|artifact| (artifact.0.to_ascii_lowercase(), artifact))
            .collect::<BTreeMap<_, _>>();
        let mut observed = BTreeSet::new();
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
        }
        if observed.len() != expected.len() || observed.len() != 4 {
            return Err(format!(
                "{LABEL} retained family census changed: expected 4 members, got {}",
                observed.len()
            ));
        }
        let options = fs::read(family_dir.join("options"))
            .map_err(|error| format!("failed to re-read {LABEL} options: {error}"))?;
        if Self::canonical_lf_text_identity(LABEL, &options)? != b"timelimit=30\n" {
            return Err(format!(
                "{LABEL} options no longer binds the historical 30-second timeout"
            ));
        }

        let prefix = "netlists/certification_tests/bug_354_son/";
        let expected_owners = [
            XYCE_BUG354_FUNCTION_RECORD,
            XYCE_BUG354_LEAD_CURRENT_RECORD,
            XYCE_BUG354_PARAMETER_RECORD,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        let owners = self
            .upstream_wrapper_decks
            .iter()
            .filter(|record| record.starts_with(prefix))
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if owners != expected_owners {
            return Err(format!(
                "{LABEL} requires its exact three manifest owners, found {owners:?}"
            ));
        }
        let exclusions = Self::load_upstream_exclusions(&self.root)
            .map_err(|error| format!("{LABEL} exclusion manifest is invalid: {error}"))?;
        if expected_owners
            .iter()
            .any(|record| exclusions.contains_key(*record))
        {
            return Err(format!(
                "{LABEL} records must not be classified by an upstream exclude sentinel"
            ));
        }
        for path in [
            XYCE_BUG354_FUNCTION_PATH,
            XYCE_BUG354_LEAD_CURRENT_PATH,
            XYCE_BUG354_PARAMETER_PATH,
        ] {
            self.reject_wrapper_output_artifacts(&self.root.join(path))
                .map_err(|error| format!("{LABEL} {error}"))?;
        }
        Ok(())
    }

    pub(super) fn observe_bug354_output_validation_failure(
        source: &str,
        deck_path: &Path,
        kind: XyceExpectedFailureKind,
        abort: &dyn AbortSignal,
    ) -> Result<XyceExpectedFailureObservation, String> {
        const LABEL: &str = "BUG_354_SON expected output-validation failure";
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort)
                .map_err(|error| format!("{LABEL} failed before output validation: {error:?}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 3
            || !netlist.subcircuits.is_empty()
            || !netlist.models.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.measurements.is_empty()
            || !netlist.data_tables.is_empty()
        {
            return Err(format!("{LABEL} typed envelope changed"));
        }
        let expected_names = ["VA", "RB", "RC"];
        if !netlist
            .elements
            .iter()
            .zip(expected_names)
            .all(|(element, expected)| element.name.eq_ignore_ascii_case(expected))
            || netlist.elements[0].nodes != ["0", "1"]
            || netlist.elements[1].nodes != ["1", "2"]
            || netlist.elements[2].nodes != ["2", "0"]
            || !matches!(netlist.elements[0].kind, ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value)) if value.to_bits() == 5.0f64.to_bits())
            || !matches!(netlist.elements[1].kind, ElementKind::Resistor { value, .. } if value.to_bits() == 100.0f64.to_bits())
            || !matches!(netlist.elements[2].kind, ElementKind::Resistor { value, .. } if value.to_bits() == 100.0f64.to_bits())
        {
            return Err(format!("{LABEL} exact source topology changed"));
        }
        if !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Tran { step, stop, start: None, max_step: None, uic: false }] if step.to_bits() == 0.0f64.to_bits() && stop.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{LABEL} exact .TRAN 0 1 request changed"));
        }
        let request = &netlist.output_requests[0];
        let (expected_expression, expected_line, expected_issue, identifiers) = match kind {
            XyceExpectedFailureKind::Bug354BadFunction => (
                "fabs(v(1))",
                9,
                OutputExpressionIssue::UnknownFunction {
                    function: "FABS".to_string(),
                },
                vec!["FABS".to_string(), "bad_function.cir:9".to_string()],
            ),
            XyceExpectedFailureKind::Bug354BadLeadCurrent => (
                "iv(rb)",
                9,
                OutputExpressionIssue::UnknownFunction {
                    function: "IV".to_string(),
                },
                vec![
                    "IV".to_string(),
                    "RB".to_string(),
                    "bad_leadcurrent.cir:9".to_string(),
                ],
            ),
            XyceExpectedFailureKind::Bug354BadParameter => (
                "bar",
                10,
                OutputExpressionIssue::UnresolvedIdentifier {
                    identifier: "BAR".to_string(),
                },
                vec!["BAR".to_string(), "bad_parameter.cir:10".to_string()],
            ),
            _ => return Err(format!("{LABEL} received unrelated expected-failure kind")),
        };
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.origin.line != expected_line
            || request.expressions.len() != 1
            || request.expressions[0]
                .split_whitespace()
                .collect::<String>()
                != expected_expression
        {
            return Err(format!("{LABEL} exact authored PRINT expression changed"));
        }
        validate_output_symbols_with_abort(&netlist, abort).map_err(|error| {
            format!("{LABEL} failed ordinary output-symbol validation first: {error:?}")
        })?;
        let error = match validate_output_expressions_with_abort(&netlist, abort) {
            Ok(()) => return Err(format!("{LABEL} unexpectedly passed output validation")),
            Err(error) => error,
        };
        let rspice_core::netlist::ParseWithAbortError::Parse(
            ParseError::OutputExpressionValidation(error),
        ) = error
        else {
            return Err(format!(
                "{LABEL} produced the wrong typed failure: {error:?}"
            ));
        };
        if error.directive != OutputDirectiveKind::Print
            || error.origin.line != expected_line
            || error.expression.split_whitespace().collect::<String>() != expected_expression
            || error.issue != expected_issue
            || error
                .origin
                .path
                .as_ref()
                .and_then(|path| path.canonicalize().ok())
                != deck_path.canonicalize().ok()
        {
            return Err(format!("{LABEL} typed identity changed: {error:?}"));
        }
        Ok(XyceExpectedFailureObservation {
            stage: XyceExpectedFailureStage::OutputValidation,
            category: match kind {
                XyceExpectedFailureKind::Bug354BadFunction
                | XyceExpectedFailureKind::Bug354BadLeadCurrent => {
                    XyceExpectedFailureCategory::UnknownOutputFunction
                }
                XyceExpectedFailureKind::Bug354BadParameter => {
                    XyceExpectedFailureCategory::UnresolvedOutputIdentifier
                }
                _ => unreachable!(),
            },
            identifiers,
        })
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
    fn bug354_historical_provenance_is_exact() {
        XyceTestRunner::validate_bug354_historical_oracle_provenance()
            .expect("Release-7.10 BUG354 provenance remains exact");
    }

    #[test]
    fn bug354_three_typed_output_failures_are_exact_and_fail_closed() {
        let root = corpus_root();
        for (path, kind, correction) in [
            (
                XYCE_BUG354_FUNCTION_PATH,
                XyceExpectedFailureKind::Bug354BadFunction,
                ("fabs(v(1))", "abs(v(1))"),
            ),
            (
                XYCE_BUG354_LEAD_CURRENT_PATH,
                XyceExpectedFailureKind::Bug354BadLeadCurrent,
                ("iv(rb)", "i(rb)"),
            ),
            (
                XYCE_BUG354_PARAMETER_PATH,
                XyceExpectedFailureKind::Bug354BadParameter,
                ("{bar}", "{1}"),
            ),
        ] {
            let path = root.join(path);
            let source = fs::read_to_string(&path).expect("read retained BUG354 source");
            XyceTestRunner::observe_bug354_output_validation_failure(
                &source,
                &path,
                kind,
                &rspice_core::abort_signal::NoAbort,
            )
            .expect("retained BUG354 source produces exact typed failure");
            assert!(
                XyceTestRunner::observe_bug354_output_validation_failure(
                    &source.replace(correction.0, correction.1),
                    &path,
                    kind,
                    &rspice_core::abort_signal::NoAbort,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn bug354_family_provenance_and_deadline_are_enforced() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let family = root.join("Netlists/Certification_Tests/BUG_354_SON");
        runner
            .validate_bug354_complete_family_provenance(&family)
            .expect("retained BUG354 family provenance is exact");

        let deck = XyceDeck {
            path: root.join(XYCE_BUG354_FUNCTION_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG354_FUNCTION_PATH.to_string(),
        };
        let error = runner
            .validate_expected_failure_oracle(
                &deck,
                XyceExpectedFailureKind::Bug354BadFunction,
                Instant::now() - Duration::from_secs(31),
            )
            .expect_err("an expired historical BUG354 deadline must fail closed");
        assert!(
            error.contains("bounded"),
            "unexpected deadline error: {error}"
        );
    }
}
