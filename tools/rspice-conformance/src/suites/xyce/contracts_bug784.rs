use super::*;

const BUG784_LABEL: &str = "BUG_784 archived duplicate .SUBCKT port failure";

impl XyceTestRunner {
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
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG784_LABEL} exceeded its deadline before observation"
            ));
        }
        Self::observe_bug784_failure(&source, &deck.path, &abort)?;
        if abort.is_aborted() {
            return Err(format!("{BUG784_LABEL} observation exceeded its deadline"));
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
}
