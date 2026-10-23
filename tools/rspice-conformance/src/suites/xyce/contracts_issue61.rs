use super::*;

const ISSUE61_LABEL: &str = "ISSUE_61 behavioral lead-current expected failure";

impl XyceTestRunner {
    pub(super) fn observe_issue61_failure(
        source: &str,
        deck_path: &Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let lines = source.lines().collect::<Vec<_>>();
        if lines.len() != 12
            || lines.get(2).copied() != Some("B1 1 0 I={1.0e-3}")
            || lines.get(6).copied() != Some("B2 2 0 I={I(B1)*20.0}")
            || lines.get(9).map(|line| line.trim()) != Some(".tran 1ns 1us")
            || lines.get(10).map(|line| line.trim()) != Some(".print tran v(1) v(2)")
            || deck_path.file_name().and_then(|name| name.to_str()) != Some("issue61.cir")
        {
            return Err(format!("{ISSUE61_LABEL} authored failure envelope changed"));
        }
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            ..NetlistParseOptions::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(source, deck_path, options, abort)
                .map_err(|error| format!("{ISSUE61_LABEL} no longer parses: {error:?}"))?;
        if netlist.elements.len() != 4
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !matches!(&netlist.elements[0].kind,
                ElementKind::BehavioralCurrent { expression, .. } if expression == "1.0e-3")
            || !matches!(&netlist.elements[1].kind,
                ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                    if value.to_bits() == 0.0f64.to_bits()
                        && instance_params == &[("__RSPICE_XYCE_DEFAULT_RESISTOR_VALUE".to_string(), 1.0)]
                        && deferred_params.is_empty())
            || !matches!(&netlist.elements[2].kind,
                ElementKind::BehavioralCurrent { expression, .. }
                    if expression.eq_ignore_ascii_case("I(B1)*20.0"))
            || !matches!(&netlist.elements[3].kind,
                ElementKind::Resistor { value, value_expr: None, model: None, instance_params, deferred_params }
                    if value.to_bits() == 0.0f64.to_bits()
                        && instance_params == &[("__RSPICE_XYCE_DEFAULT_RESISTOR_VALUE".to_string(), 1.0)]
                        && deferred_params.is_empty())
            || !matches!(&netlist.analyses[0],
                AnalysisCommand::Tran { step, stop, start: None, max_step: None, uic: false }
                    if step.to_bits() == 1e-9f64.to_bits() && stop.to_bits() == 1e-6f64.to_bits())
        {
            return Err(format!("{ISSUE61_LABEL} typed circuit envelope changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.dependencies.len() != 2
            || request
                .dependencies
                .iter()
                .any(|dependency| dependency.kind != OutputSymbolKind::Node)
            || request.dependencies[0].symbol != "1"
            || request.dependencies[1].symbol != "2"
        {
            return Err(format!("{ISSUE61_LABEL} typed output request changed"));
        }

        let error = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .build_circuit_with_abort(&netlist, abort)
            .expect_err("ISSUE_61 must reject I(B1) when B1 has no MNA branch");
        let SimulationError::BehavioralReference(error) = error else {
            return Err(format!(
                "{ISSUE61_LABEL} produced the wrong typed failure: {error:?}"
            ));
        };
        if error.owner_name != "B2"
            || error.canonical_owner_name != "B2"
            || error.dependency_name != "B1"
            || error.canonical_dependency_name != "B1"
            || error.reason
                != rspice_core::device::BehavioralReferenceReason::LeadCurrentNotSolutionVariable
            || !error
                .to_string()
                .starts_with(XYCE_ISSUE61_UPSTREAM_DIAGNOSTIC)
        {
            return Err(format!("{ISSUE61_LABEL} typed device identity changed"));
        }

        let repaired_source = source.replacen("B1 1 0 I={1.0e-3}", "B1 1 0 V={1.0e-3}", 1);
        let repaired = Netlist::parse_with_path_and_options_and_abort(
            &repaired_source,
            deck_path,
            options,
            abort,
        )
        .map_err(|error| format!("{ISSUE61_LABEL} repaired deck no longer parses: {error:?}"))?;
        Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
            .build_circuit_with_abort(&repaired, abort)
            .map_err(|error| format!("{ISSUE61_LABEL} repaired deck no longer builds: {error}"))?;
        Ok(())
    }

    pub(super) fn validate_issue61_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        if abort.is_aborted() {
            return Err(format!(
                "{ISSUE61_LABEL} exceeded its deadline before observation"
            ));
        }
        Self::observe_issue61_failure(&source, &deck.path, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{ISSUE61_LABEL} observation exceeded its deadline"));
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
            path: root.join(XYCE_ISSUE61_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_ISSUE61_PATH.to_string(),
        }
    }

    #[test]
    fn issue61_observation_preserves_typed_branch_failure() {
        let path = corpus_root().join(XYCE_ISSUE61_PATH);
        let source = fs::read_to_string(&path).expect("read ISSUE61 source");
        XyceTestRunner::observe_issue61_failure(
            &source,
            &path,
            &rspice_core::abort_signal::NoAbort,
        )
        .expect("ISSUE61 produces its exact typed branch failure");
        assert!(
            XyceTestRunner::observe_issue61_failure(
                &source.replacen("I(B1)*20.0", "20.0", 1),
                &path,
                &rspice_core::abort_signal::NoAbort,
            )
            .is_err()
        );
    }

    #[test]
    fn issue61_oracle_and_deadline_are_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let deck = canonical_deck(&root);
        runner
            .validate_issue61_oracle(&deck, Instant::now())
            .expect("canonical ISSUE61 oracle qualifies");
        assert!(
            runner
                .validate_issue61_oracle(
                    &deck,
                    Instant::now()
                        - Duration::from_millis(
                            u64::try_from(runner.config.max_time_per_test_ms + 1)
                                .expect("test deadline fits Duration"),
                        ),
                )
                .is_err()
        );
    }
}
