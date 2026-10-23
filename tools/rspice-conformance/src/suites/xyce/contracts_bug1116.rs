use super::*;

const BUG1116_LABEL: &str = "BUG_1116 missing-diode-model expected failure";

impl XyceTestRunner {
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
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG1116_LABEL} exceeded its deadline before observation"
            ));
        }
        Self::observe_bug1116_failure(&source, &deck.path, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG1116_LABEL} observation exceeded its deadline"));
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
}
