use super::*;
use crate::suites::execution::ExecutionOutcome;

impl XyceTestRunner {
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
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        if abort.is_aborted() {
            return Err(format!(
                "BUG_1025 exceeded the bounded {timeout_ms}ms contract before observation"
            ));
        }
        let (outcome, diagnostic) = Self::observe_bug1025_no_analysis(&source, &deck.path, &abort)?;
        if outcome != ExecutionOutcome::NoAnalysis
            || !diagnostic.contains(XYCE_BUG1025_UPSTREAM_DIAGNOSTIC)
        {
            return Err(format!(
                "BUG_1025 produced the wrong bounded observation: {outcome:?} / {diagnostic:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "BUG_1025 observation exceeded the bounded {timeout_ms}ms contract"
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
}
