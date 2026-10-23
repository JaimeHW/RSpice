use super::*;

const BUG206_LABEL: &str = "BUG_206 undefined-subcircuit expected failure";

impl XyceTestRunner {
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
            AnalysisCommand::Dc { source, start, stop, step, mode: DcSweepMode::Linear, sweep2: None, .. }
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
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG206_LABEL} exceeded its deadline before observation"
            ));
        }
        Self::observe_bug206_failure(&source, &deck.path, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG206_LABEL} observation exceeded its deadline"));
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
}
