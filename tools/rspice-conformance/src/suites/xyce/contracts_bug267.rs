use super::*;

const BUG267_LABEL: &str = "BUG_267 include/global-parameter success wrapper";

impl XyceTestRunner {
    fn bug267_plan_and_netlist(
        &self,
        source: String,
        deck_path: &Path,
    ) -> Result<(XyceStaticDcPlan, Netlist), String> {
        let netlist = Self::parse_netlist_with_expression_dialect(
            &source,
            deck_path,
            ExpressionDialect::Xyce,
        )
        .map_err(|error| format!("{BUG267_LABEL} source no longer parses: {error}"))?;
        let dc = Self::single_dc_sweep(&netlist)?;
        let print = XycePrintRequest {
            probes: vec!["V(1)".into(), "I(V1)".into(), "bar".into()],
        };
        Self::validate_static_dc_contract(&netlist, &dc, &print)?;
        let steps = Self::step_commands(&netlist)?;
        let plan = XyceStaticDcPlan {
            deck_path: deck_path.to_path_buf(),
            execution_dir: None,
            source,
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent,
            print,
            print_format: None,
            dc,
            dc_data: None,
            steps,
            diagnostics: netlist.diagnostics.clone(),
            sealed_sources: None,
        };
        Ok((plan, netlist))
    }

    fn validate_bug267_typed_contract(
        &self,
        source: String,
        deck_path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let (plan, netlist) = self.bug267_plan_and_netlist(source, deck_path)?;
        if plan.deck_path != deck_path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != ["V(1)", "I(V1)", "bar"]
        {
            return Err(format!("{BUG267_LABEL} static DC plan changed: {plan:?}"));
        }
        if netlist.title != "test to Test the Xyce Resistor Model"
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 2
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!("{BUG267_LABEL} typed netlist envelope changed"));
        }
        let resistor = &netlist.elements[0];
        if !resistor.name.eq_ignore_ascii_case("R1")
            || resistor.nodes != ["1", "0"]
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1_000.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{BUG267_LABEL} exact R1 topology changed"));
        }
        let source = &netlist.elements[1];
        if !source.name.eq_ignore_ascii_case("V1")
            || source.nodes != ["1", "0"]
            || source.provenance != ElementProvenance::Authored
            || !matches!(&source.kind,
                ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                    if value.to_bits() == 5.0f64.to_bits())
        {
            return Err(format!("{BUG267_LABEL} exact V1 topology changed"));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
            ..
        } if source.eq_ignore_ascii_case("V1")
            && start.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0f64.to_bits()
            && step.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{BUG267_LABEL} typed DC command changed"));
        }

        let expected_sidecar = deck_path
            .parent()
            .ok_or_else(|| format!("{BUG267_LABEL} deck path has no parent"))?
            .join("analysis.cmds");
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || !request.expressions.is_empty()
            || request.origin.line != 2
            || request
                .origin
                .path
                .as_deref()
                .is_none_or(|path| !Self::same_path(path, &expected_sidecar))
            || request.dependencies.len() != 2
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
            || request.dependencies[0].expression
            || request.dependencies[1].kind != OutputSymbolKind::Device
            || !request.dependencies[1].operator.eq_ignore_ascii_case("I")
            || !request.dependencies[1].symbol.eq_ignore_ascii_case("V1")
            || request.dependencies[1].expression
        {
            return Err(format!(
                "{BUG267_LABEL} included .PRINT request changed: {request:?}"
            ));
        }

        let mut ordinary_expressions = netlist.params.all_parameter_expressions();
        ordinary_expressions.sort();
        let mut global_expressions = netlist.params.all_global_expressions();
        global_expressions.sort();
        let numeric = netlist.params.numeric_parameters();
        // A fully static ordinary `.PARAM` is deliberately materialized into
        // the numeric namespace, while the dependent `.GLOBAL_PARAM`
        // expression remains authoritative. The exact retained source hash
        // above binds FOOBAR's authored `3*9` expression; these assertions
        // bind its typed projection and BAR's surviving dependency edge.
        if !ordinary_expressions.is_empty()
            || global_expressions != [("BAR".to_string(), "foobar".to_string())]
            || numeric.len() != 2
            || numeric
                .iter()
                .find(|(name, _)| name == "FOOBAR")
                .is_none_or(|(_, value)| value.to_bits() != 27.0f64.to_bits())
            || numeric
                .iter()
                .find(|(name, _)| name == "BAR")
                .is_none_or(|(_, value)| value.to_bits() != 27.0f64.to_bits())
            || netlist.params.get("FOOBAR").map(Value::to_bits) != Some(27.0f64.to_bits())
            || netlist.params.get("BAR").map(Value::to_bits) != Some(27.0f64.to_bits())
        {
            return Err(format!(
                "{BUG267_LABEL} ordinary/global parameter causality changed: ordinary={ordinary_expressions:?}, global={global_expressions:?}, numeric={numeric:?}"
            ));
        }
        Ok(plan)
    }

    fn run_bug267_typed_oracle(
        &self,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<(), String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG267_LABEL} DC execution failed: {error}"))?;
        if results.len() != 6 {
            return Err(format!(
                "{BUG267_LABEL} produced {} DC points instead of 6",
                results.len()
            ));
        }
        for (index, point) in results.iter().enumerate() {
            let expected = index as Value;
            let voltage = point
                .result
                .try_voltage_named("1")
                .ok_or_else(|| format!("{BUG267_LABEL} point {index} lost node 1"))?;
            let current = point
                .result
                .branch_current_named("V1")
                .ok_or_else(|| format!("{BUG267_LABEL} point {index} lost I(V1)"))?;
            let expected_current = -expected / 1_000.0;
            let finite = point.sweep_value.is_finite()
                && point
                    .result
                    .node_voltages
                    .iter()
                    .chain(point.result.branch_currents.iter())
                    .all(|value| value.is_finite())
                && point
                    .result
                    .dc_observables
                    .iter()
                    .all(|(_, value)| value.is_finite())
                && point
                    .device_op_report
                    .entries
                    .iter()
                    .flat_map(|entry| entry.params.iter().map(|(_, value)| value))
                    .all(|value| value.is_finite());
            if point.sweep_value.to_bits() != expected.to_bits()
                || (voltage - expected).abs() > 1.0e-12
                || (current - expected_current).abs() > 1.0e-12
                || !finite
                || !point.device_op_report.labels_resolve()
            {
                return Err(format!(
                    "{BUG267_LABEL} point {index} violates its analytic contract: sweep={}, V(1)={voltage}, I(V1)={current}",
                    point.sweep_value
                ));
            }
        }
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG267_LABEL} PRN materialization failed: {error}"))?;
        if table.columns != ["Index", "V(1)", "I(V1)", "bar"] || table.rows.len() != 6 {
            return Err(format!(
                "{BUG267_LABEL} default PRN layout changed: columns={:?}, rows={}",
                table.columns,
                table.rows.len()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            let expected = index as Value;
            if row.len() != 4
                || row[0].to_bits() != expected.to_bits()
                || (row[1] - expected).abs() > 1.0e-12
                || (row[2] + expected / 1_000.0).abs() > 1.0e-12
                || row[3].to_bits() != 27.0f64.to_bits()
                || !row.iter().all(|value| value.is_finite())
            {
                return Err(format!("{BUG267_LABEL} PRN row {index} changed: {row:?}"));
            }
        }
        let serialized_mismatches = self
            .compare_serialized_default_prn_tables(&table, &table)
            .map_err(|error| format!("{BUG267_LABEL} default PRN serialization failed: {error}"))?;
        if !serialized_mismatches.is_empty() {
            return Err(format!(
                "{BUG267_LABEL} default PRN serialization is not self-consistent: {serialized_mismatches:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug267_oracle(
        &self,
        deck: &XyceDeck,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG267_LABEL} deadline expired before validation"));
        }
        let source = fs::read_to_string(&deck.path)
            .map_err(|error| format!("failed to read {}: {error}", deck.path.display()))?;
        let plan = self.validate_bug267_typed_contract(source, &deck.path)?;
        self.run_bug267_typed_oracle(&plan, start)?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG267_LABEL} execution exceeded timeout ({}ms)",
                self.config.max_time_per_test_ms
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
            path: root.join(XYCE_BUG267_PATH),
            section: XyceDeckSection::Netlists,
            relative_path: XYCE_BUG267_PATH.to_string(),
        }
    }

    #[test]
    fn bug267_typed_contract_preserves_include_parameter_causality() {
        let root = corpus_root();
        let path = root.join(XYCE_BUG267_PATH);
        let source = fs::read_to_string(&path).expect("read BUG267 source");
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug267_typed_contract(source.clone(), &path)
            .expect("canonical BUG267 typed contract passes");

        for mutation in [
            source.replace(".global_param bar={foobar}", ".param bar={foobar}"),
            source.replace("R1 1 0 1K", "R1 1 0 2K"),
            source.replace("V1 1 0 5V", "V1 2 0 5V"),
            source.replace(".include analysis.cmds", ".include missing.cmds"),
        ] {
            assert!(
                runner
                    .validate_bug267_typed_contract(mutation, &path)
                    .is_err(),
                "BUG267 semantic mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug267_oracle_executes_all_six_analytic_rows() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug267_oracle(&canonical_deck(&root), Instant::now())
            .expect("canonical BUG267 oracle passes");
    }

    #[test]
    fn bug267_oracle_rejects_an_expired_deadline() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired deadline");
        assert!(
            runner
                .validate_bug267_oracle(&canonical_deck(&root), start)
                .is_err()
        );
    }
}
