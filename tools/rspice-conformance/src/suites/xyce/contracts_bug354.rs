use super::*;

impl XyceTestRunner {
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
    fn bug354_expired_deadline_fails_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
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
