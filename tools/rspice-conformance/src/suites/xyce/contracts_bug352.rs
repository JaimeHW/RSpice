use super::*;

const BUG352_LABEL: &str = "BUG_352 diode model-expression equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug352Role {
    ExpressionOwner,
    LiteralControl,
}

impl Bug352Role {
    const ALL: [Self; 2] = [Self::ExpressionOwner, Self::LiteralControl];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn path(self) -> &'static str {
        match self {
            Self::ExpressionOwner => XYCE_BUG352_OWNER_PATH,
            Self::LiteralControl => XYCE_BUG352_CONTROL_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::ExpressionOwner => XYCE_BUG352_OWNER_RECORD,
            Self::LiteralControl => XYCE_BUG352_CONTROL_RECORD,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::ExpressionOwner => XYCE_BUG352_OWNER_CONTRACT,
            Self::LiteralControl => XYCE_BUG352_CONTROL_CONTRACT,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ExpressionOwner => "model-expression owner",
            Self::LiteralControl => "literal-model control",
        }
    }
}

impl XyceTestRunner {
    fn validate_bug352_worker_contract(
        &self,
        role: Bug352Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let authored_marker = match role {
            Bug352Role::ExpressionOwner => "is={foobar1*1.5}",
            Bug352Role::LiteralControl => "is=1.5",
        };
        if !source.to_ascii_lowercase().contains(authored_marker) {
            return Err(format!(
                "{BUG352_LABEL} {} lost its authored model representation",
                role.label()
            ));
        }
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VIN")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 0.5f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != ["V(2)", "I(vamon1)"]
        {
            return Err(format!(
                "{BUG352_LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }
        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG352_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if netlist.title != "Test"
            || !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 4
            || netlist.models.len() != 1
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || !netlist.node_sets.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
        {
            return Err(format!(
                "{BUG352_LABEL} {} typed envelope changed",
                role.label()
            ));
        }
        let params = netlist.params.numeric_parameters();
        match role {
            Bug352Role::ExpressionOwner
                if !matches!(params.as_slice(), [(name, value)]
                    if name.eq_ignore_ascii_case("FOOBAR1")
                        && value.to_bits() == 1.0f64.to_bits()) =>
            {
                return Err(format!("{BUG352_LABEL} owner FOOBAR1 binding changed"));
            }
            Bug352Role::LiteralControl if !params.is_empty() => {
                return Err(format!("{BUG352_LABEL} control acquired parameters"));
            }
            _ => {}
        }
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("JMOD1")
            || !model.model_type.eq_ignore_ascii_case("D")
            || !matches!(model.params.as_slice(), [(name, value)]
                if name.eq_ignore_ascii_case("IS")
                    && value.to_bits() == 1.5f64.to_bits())
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "{BUG352_LABEL} {} did not resolve its diode IS to exactly 1.5",
                role.label()
            ));
        }
        let diode = &netlist.elements[0];
        if !diode.name.eq_ignore_ascii_case("D1")
            || diode.nodes != ["1A", "0"]
            || diode.provenance != ElementProvenance::Authored
            || !matches!(&diode.kind, ElementKind::Diode { model, instance_params, deferred_params }
                if model.eq_ignore_ascii_case("JMOD1")
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG352_LABEL} {} diode changed: {diode:?}",
                role.label()
            ));
        }
        let resistor = &netlist.elements[1];
        if !resistor.name.eq_ignore_ascii_case("RS")
            || resistor.nodes != ["1", "1A"]
            || resistor.provenance != ElementProvenance::Authored
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{BUG352_LABEL} {} resistor changed", role.label()));
        }
        let exact_source = |element: &rspice_core::netlist::Element,
                            name: &str,
                            nodes: [&str; 2],
                            expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits())
        };
        if !exact_source(&netlist.elements[2], "VAMON1", ["1", "2"], 0.0)
            || !exact_source(&netlist.elements[3], "VIN", ["2", "0"], 5.0)
        {
            return Err(format!("{BUG352_LABEL} {} sources changed", role.label()));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
            ..
        } if source.eq_ignore_ascii_case("VIN")
            && start.to_bits() == 0.0f64.to_bits()
            && stop.to_bits() == 5.0f64.to_bits()
            && step.to_bits() == 0.5f64.to_bits())
        {
            return Err(format!(
                "{BUG352_LABEL} {} typed DC command changed",
                role.label()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 2
        {
            return Err(format!(
                "{BUG352_LABEL} {} typed .PRINT request changed",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug352_worker(
        &self,
        role: Bug352Role,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG352_LABEL} {} failed: {error}", role.label()))?;
        if results.len() != 11 {
            return Err(format!(
                "{BUG352_LABEL} {} produced {} points instead of 11",
                role.label(),
                results.len()
            ));
        }
        for (index, point) in results.iter().enumerate() {
            let expected = index as Value * 0.5;
            if point.sweep_value.to_bits() != expected.to_bits()
                || !point
                    .result
                    .node_voltages
                    .iter()
                    .chain(point.result.branch_currents.iter())
                    .all(|value| value.is_finite())
                || !point
                    .result
                    .dc_observables
                    .iter()
                    .all(|(_, value)| value.is_finite())
                || !point
                    .device_op_report
                    .entries
                    .iter()
                    .flat_map(|entry| entry.params.iter().map(|(_, value)| value))
                    .all(|value| value.is_finite())
                || !point.device_op_report.labels_resolve()
            {
                return Err(format!(
                    "{BUG352_LABEL} {} point {index} is not a finite exact-grid observation",
                    role.label()
                ));
            }
        }
        let table = self
            .dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| format!("{BUG352_LABEL} {} PRN failed: {error}", role.label()))?;
        let max_current = table
            .rows
            .iter()
            .filter_map(|row| row.get(2))
            .map(|value| value.abs())
            .fold(0.0f64, Value::max);
        if max_current <= 1.0e-6 {
            return Err(format!(
                "{BUG352_LABEL} {} current trace is trivially zero",
                role.label()
            ));
        }
        Ok(table)
    }

    pub(super) fn validate_bug352_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG352_LABEL} deadline expired before validation"));
        }
        let mut output = BTreeMap::new();
        for worker_role in Bug352Role::ALL {
            let path = self.root.join(worker_role.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let plan = self.validate_bug352_worker_contract(worker_role, &source, &path)?;
            output.insert(
                worker_role,
                self.run_bug352_worker(worker_role, &plan, start)?,
            );
        }
        let owner = output
            .get(&Bug352Role::ExpressionOwner)
            .expect("both BUG352 workers ran");
        let control = output
            .get(&Bug352Role::LiteralControl)
            .expect("both BUG352 workers ran");
        let mismatches = self
            .compare_serialized_default_prn_tables(owner, control)
            .map_err(|error| format!("{BUG352_LABEL} raw-PRN relation failed: {error}"))?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG352_LABEL} expression and literal PRNs differ: {mismatches:?}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{BUG352_LABEL} execution exceeded timeout ({}ms)",
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

    #[test]
    fn bug352_workers_resolve_expression_and_literal_to_the_same_model() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug352Role::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG352 worker");
            runner
                .validate_bug352_worker_contract(role, &source, &path)
                .expect("canonical BUG352 worker passes");
            let mutation = source.replace("1.5", "1.6");
            assert!(
                runner
                    .validate_bug352_worker_contract(role, &mutation, &path)
                    .is_err(),
                "model-value mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug352_oracle_runs_both_exact_roles() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_bug352_oracle(Instant::now())
            .expect("canonical BUG352 relational oracle passes");
    }

    #[test]
    fn bug352_oracle_rejects_an_expired_deadline() {
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(corpus_root(), config);
        let start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired deadline");
        assert!(runner.validate_bug352_oracle(start).is_err());
    }
}
