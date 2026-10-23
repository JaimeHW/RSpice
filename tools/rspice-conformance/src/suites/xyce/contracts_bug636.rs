use super::*;

const BUG636_LABEL: &str = "BUG_636_SON incomplete .TRAN expected failure";
const BUG636_PARSE_MESSAGE: &str =
    ".TRAN line has an unexpected number of fields\nUnrecognized dot line will be ignored";

impl XyceTestRunner {
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
        if line != 10
            || message != Self::located_analysis_card_message(deck_path, 10, BUG636_PARSE_MESSAGE)
        {
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
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG636_LABEL} exceeded its deadline before observation"
            ));
        }
        Self::observe_bug636_failure(&source, &deck.path, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG636_LABEL} observation exceeded its deadline"));
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
}
