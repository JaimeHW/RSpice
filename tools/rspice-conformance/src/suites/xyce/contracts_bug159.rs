use super::*;

const BUG159_LABEL: &str = "BUG_159 BJT TNOM default equivalence";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug159WorkerRole {
    ExplicitTnom,
    ImplicitDefault,
}

impl Bug159WorkerRole {
    const ALL: [Self; 2] = [Self::ExplicitTnom, Self::ImplicitDefault];

    fn path(self) -> &'static str {
        match self {
            Self::ExplicitTnom => XYCE_BUG159_EXPLICIT_PATH,
            Self::ImplicitDefault => XYCE_BUG159_IMPLICIT_PATH,
        }
    }

    fn reference_file_name(self) -> &'static str {
        match self {
            Self::ExplicitTnom => "bug_159_1.cir.prn",
            Self::ImplicitDefault => "bug_159_2.cir.prn",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::ExplicitTnom => "explicit TNOM=27 worker",
            Self::ImplicitDefault => "implicit default-TNOM worker",
        }
    }
}

impl XyceTestRunner {
    fn validate_bug159_worker_contract(
        &self,
        role: Bug159WorkerRole,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticDcPlan, String> {
        let plan = self.static_dc_plan_for_source_with_execution_dir(
            path,
            source.to_string(),
            ExpressionDialect::Xyce,
            None,
        )?;
        let expected_probes = ["V(4)", "I(VMON1)", "I(VMON2)", "V(1)", "V(2)"];
        if plan.deck_path != path
            || plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || !plan.diagnostics.is_empty()
            || plan.print_format.is_some()
            || plan.dc_data.is_some()
            || !plan.steps.is_empty()
            || !plan.dc.source.eq_ignore_ascii_case("VCC")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 12.0f64.to_bits()
            || plan.dc.stop.to_bits() != 12.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes != expected_probes
        {
            return Err(format!(
                "{BUG159_LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path).map_err(|error| {
            format!("{BUG159_LABEL} {} no longer parses: {error}", role.label())
        })?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 6
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
            || !netlist.params.numeric_parameters().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || netlist.options.tnom.is_some()
        {
            return Err(format!(
                "{BUG159_LABEL} {} typed envelope changed",
                role.label()
            ));
        }

        let exact_voltage_source = |element: &rspice_core::netlist::Element,
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
        let exact_resistor = |element: &rspice_core::netlist::Element,
                              name: &str,
                              nodes: [&str; 2],
                              expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && element.nodes == nodes
                && element.provenance == ElementProvenance::Authored
                && matches!(&element.kind, ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == expected.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty())
        };
        if !exact_voltage_source(&netlist.elements[0], "VCC", ["4", "0"], 12.0)
            || !exact_resistor(&netlist.elements[1], "RC", ["3", "4"], 2_000.0)
            || !exact_resistor(&netlist.elements[2], "RB", ["4", "5"], 377_000.0)
            || !exact_voltage_source(&netlist.elements[3], "VMON1", ["5", "1"], 0.0)
            || !exact_voltage_source(&netlist.elements[4], "VMON2", ["3", "2"], 0.0)
        {
            return Err(format!(
                "{BUG159_LABEL} {} source/resistor topology changed",
                role.label()
            ));
        }
        let transistor = &netlist.elements[5];
        if !transistor.name.eq_ignore_ascii_case("Q")
            || transistor.nodes != ["2", "1", "0"]
            || transistor.provenance != ElementProvenance::Authored
            || !matches!(&transistor.kind, ElementKind::Bjt {
                model,
                bjt_type: rspice_core::netlist::BjtType::Npn,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("NBJT")
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!(
                "{BUG159_LABEL} {} exact three-terminal NPN instance changed",
                role.label()
            ));
        }

        const EXPLICIT_MODEL_PARAMS: [(&str, Value); 2] = [("BF", 100.0), ("TNOM", 27.0)];
        const IMPLICIT_MODEL_PARAMS: [(&str, Value); 1] = [("BF", 100.0)];
        let model = &netlist.models[0];
        let expected_params: &[(&str, Value)] = match role {
            Bug159WorkerRole::ExplicitTnom => &EXPLICIT_MODEL_PARAMS,
            Bug159WorkerRole::ImplicitDefault => &IMPLICIT_MODEL_PARAMS,
        };
        let params_match = model.params.len() == expected_params.len()
            && model.params.iter().zip(expected_params).all(
                |((name, value), (expected_name, expected_value))| {
                    name.eq_ignore_ascii_case(expected_name)
                        && value.to_bits() == (*expected_value).to_bits()
                },
            );
        if !model.name.eq_ignore_ascii_case("NBJT")
            || !model.model_type.eq_ignore_ascii_case("NPN")
            || !params_match
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!(
                "{BUG159_LABEL} {} exact NBJT model representation changed",
                role.label()
            ));
        }
        if !matches!(&netlist.analyses[0], AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
            mode: DcSweepMode::Linear,
            sweep2: None,
            ..
        } if source.eq_ignore_ascii_case("VCC")
            && start.to_bits() == 12.0f64.to_bits()
            && stop.to_bits() == 12.0f64.to_bits()
            && step.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!(
                "{BUG159_LABEL} {} typed one-point DC command changed",
                role.label()
            ));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 5
        {
            return Err(format!(
                "{BUG159_LABEL} {} typed .PRINT request changed",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn run_bug159_worker(
        &self,
        role: Bug159WorkerRole,
        plan: &XyceStaticDcPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, results) = self
            .run_static_dc_results(plan, start)
            .map_err(|error| format!("{BUG159_LABEL} {} failed: {error}", role.label()))?;
        let [point] = results.as_slice() else {
            return Err(format!(
                "{BUG159_LABEL} {} produced {} DC points instead of one",
                role.label(),
                results.len()
            ));
        };
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
        if point.sweep_value.to_bits() != 12.0f64.to_bits()
            || !finite
            || !point.device_op_report.labels_resolve()
        {
            return Err(format!(
                "{BUG159_LABEL} {} finite one-point observation changed",
                role.label()
            ));
        }
        self.dc_results_to_prn_table(plan, &netlist, &results)
            .map_err(|error| {
                format!(
                    "{BUG159_LABEL} {} default PRN generation failed: {error}",
                    role.label()
                )
            })
    }

    fn require_bug159_serialized_match(
        &self,
        expected_label: &str,
        expected: &XycePrnTable,
        actual_label: &str,
        actual: &XycePrnTable,
    ) -> Result<(), String> {
        let mismatches = self
            .compare_serialized_default_prn_tables(expected, actual)
            .map_err(|error| {
                format!(
                    "{BUG159_LABEL} {expected_label} versus {actual_label} layout comparison failed: {error}"
                )
            })?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{BUG159_LABEL} {expected_label} versus {actual_label} default PRN differs: {mismatches:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug159_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{BUG159_LABEL} deadline expired before execution"));
        }
        let output_dir = self.root.join("OutputData/Certification_Tests/BUG_159");
        let mut generated = BTreeMap::new();
        for role in Bug159WorkerRole::ALL {
            let path = self.root.join(role.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let plan = self.validate_bug159_worker_contract(role, &source, &path)?;
            let table = self.run_bug159_worker(role, &plan, start)?;
            let reference_path = output_dir.join(role.reference_file_name());
            let reference = Self::parse_prn_file(&reference_path).map_err(|error| {
                format!(
                    "{BUG159_LABEL} {} retained reference is invalid: {error}",
                    role.label()
                )
            })?;
            self.require_bug159_serialized_match(
                &format!("{} retained gold", role.label()),
                &reference,
                &format!("{} generated output", role.label()),
                &table,
            )?;
            generated.insert(role, table);
        }
        let explicit = generated
            .get(&Bug159WorkerRole::ExplicitTnom)
            .expect("both BUG159 worker roles are generated");
        let implicit = generated
            .get(&Bug159WorkerRole::ImplicitDefault)
            .expect("both BUG159 worker roles are generated");
        self.require_bug159_serialized_match(
            "explicit TNOM=27 worker",
            explicit,
            "implicit default-TNOM worker",
            implicit,
        )?;
        if abort.is_aborted() {
            return Err(format!(
                "{BUG159_LABEL} execution exceeded timeout ({}ms)",
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
    fn bug159_workers_preserve_exact_explicit_and_implicit_tnom_contracts() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug159WorkerRole::ALL {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read canonical BUG159 worker");
            runner
                .validate_bug159_worker_contract(role, &source, &path)
                .expect("canonical BUG159 worker contract passes");
        }

        let explicit_path = root.join(XYCE_BUG159_EXPLICIT_PATH);
        let explicit = fs::read_to_string(&explicit_path).expect("read explicit BUG159 worker");
        let explicit_mutation = explicit.replace("TNOM=27", "TNOM=28");
        assert!(
            runner
                .validate_bug159_worker_contract(
                    Bug159WorkerRole::ExplicitTnom,
                    &explicit_mutation,
                    &explicit_path,
                )
                .is_err(),
            "a non-default explicit TNOM must fail the BUG159 contract"
        );

        let implicit_path = root.join(XYCE_BUG159_IMPLICIT_PATH);
        let implicit = fs::read_to_string(&implicit_path).expect("read implicit BUG159 worker");
        for mutation in [
            implicit.replace("BF=100", "BF=101"),
            implicit.replace("V(4) I(VMON1)", "I(VMON1) V(4)"),
        ] {
            assert!(
                runner
                    .validate_bug159_worker_contract(
                        Bug159WorkerRole::ImplicitDefault,
                        &mutation,
                        &implicit_path,
                    )
                    .is_err(),
                "model and print-order mutations must fail the BUG159 contract"
            );
        }
    }

    #[test]
    fn bug159_oracle_rejects_an_expired_deadline() {
        let mut runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner.config.max_time_per_test_ms = 1;
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG159 deadline");
        let error = runner
            .validate_bug159_oracle(expired_start)
            .expect_err("an expired BUG159 deadline must fail closed");
        assert!(error.contains("deadline"));
    }
}
