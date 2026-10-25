use super::*;

const LABEL: &str = "BUG_113 voltage-switch initial junction state";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_113/VSWITCH.cir";
const NOIC_PATH: &str = "Netlists/Certification_Tests/BUG_113/VSWITCH_NOIC.cir";
const EXPLICIT_ON_PATH: &str = "Netlists/Certification_Tests/BUG_113/VSWITCH_ON_OFF.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_113/vswitch.cir";
const NOIC_RECORD: &str = "netlists/certification_tests/bug_113/vswitch_noic.cir";
const EXPLICIT_ON_RECORD: &str = "netlists/certification_tests/bug_113/vswitch_on_off.cir";
const OWNER_CONTRACT: &str = "bug113_switch_initial_state_jacobian_wrapper_owner";
const NOIC_CONTRACT: &str = "bug113_switch_initial_state_noic_baseline";
const EXPLICIT_ON_CONTRACT: &str = "bug113_switch_initial_state_explicit_on_member";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug113Role {
    WrapperOwner,
    NoInitialState,
    ExplicitOn,
}

impl Bug113Role {
    const WORKERS: [Self; 2] = [Self::NoInitialState, Self::ExplicitOn];

    pub(super) fn for_record(relative_path: &str) -> Option<Self> {
        match XyceTestRunner::normalize_manifest_key(relative_path).as_str() {
            OWNER_RECORD => Some(Self::WrapperOwner),
            NOIC_RECORD => Some(Self::NoInitialState),
            EXPLICIT_ON_RECORD => Some(Self::ExplicitOn),
            _ => None,
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => OWNER_CONTRACT,
            Self::NoInitialState => NOIC_CONTRACT,
            Self::ExplicitOn => EXPLICIT_ON_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => OWNER_PATH,
            Self::NoInitialState => NOIC_PATH,
            Self::ExplicitOn => EXPLICIT_ON_PATH,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::WrapperOwner => "empty wrapper owner",
            Self::NoInitialState => "unmarked switch worker",
            Self::ExplicitOn => "explicit-ON switch worker",
        }
    }

    fn expected_initial_state(self) -> Option<rspice_core::netlist::SwitchState> {
        match self {
            Self::NoInitialState => None,
            Self::ExplicitOn => Some(rspice_core::netlist::SwitchState::On),
            Self::WrapperOwner => unreachable!("the empty owner is not executable"),
        }
    }

    fn expected_jacobians(self) -> usize {
        match self {
            Self::NoInitialState => 3,
            Self::ExplicitOn => 2,
            Self::WrapperOwner => unreachable!("the empty owner is not executable"),
        }
    }
}

impl XyceTestRunner {
    fn validate_bug113_worker(
        &self,
        role: Bug113Role,
        source: &str,
        path: &Path,
    ) -> Result<Netlist, String> {
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
            || !plan.dc.source.eq_ignore_ascii_case("V1")
            || plan.dc.mode != DcSweepMode::Linear
            || plan.dc.start.to_bits() != 5.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || plan.dc.sweep2.is_some()
            || plan.print.probes.len() != 1
            || !plan.print.probes[0].eq_ignore_ascii_case("I(V1)")
        {
            return Err(format!(
                "{LABEL} {} static DC plan changed: {plan:?}",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        if !netlist.diagnostics.is_empty()
            || netlist.elements.len() != 5
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
        {
            return Err(format!("{LABEL} {} typed envelope changed", role.label()));
        }

        let exact_voltage_source = |element: &rspice_core::netlist::Element,
                                    name: &str,
                                    nodes: &[&str],
                                    expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && Self::nodes_match(&element.nodes, nodes)
                && element.provenance == ElementProvenance::Authored
                && matches!(
                    &element.kind,
                    ElementKind::VoltageSource(rspice_core::netlist::SourceSpec::Dc(value))
                        if value.to_bits() == expected.to_bits()
                )
        };
        let exact_resistor = |element: &rspice_core::netlist::Element,
                              name: &str,
                              nodes: &[&str],
                              expected: Value|
         -> bool {
            element.name.eq_ignore_ascii_case(name)
                && Self::nodes_match(&element.nodes, nodes)
                && element.provenance == ElementProvenance::Authored
                && matches!(
                    &element.kind,
                    ElementKind::Resistor {
                        value,
                        value_expr: None,
                        model: None,
                        instance_params,
                        deferred_params,
                    } if value.to_bits() == expected.to_bits()
                        && instance_params.is_empty()
                        && deferred_params.is_empty()
                )
        };
        if !exact_voltage_source(&netlist.elements[0], "V1", &["1", "0"], 5.0)
            || !netlist.elements[1].name.eq_ignore_ascii_case("S1")
            || !Self::nodes_match(&netlist.elements[1].nodes, &["1", "2"])
            || netlist.elements[1].provenance != ElementProvenance::Authored
            || !matches!(
                &netlist.elements[1].kind,
                ElementKind::VSwitch {
                    control_pos,
                    control_neg,
                    model,
                    initial_state,
                } if control_pos.eq_ignore_ascii_case("3")
                    && control_neg.eq_ignore_ascii_case("0")
                    && model.eq_ignore_ascii_case("SW")
                    && *initial_state == role.expected_initial_state()
            )
            || !exact_resistor(&netlist.elements[2], "R1", &["2", "0"], 100.0)
            || !exact_voltage_source(&netlist.elements[3], "V2", &["3", "0"], 1.0)
            || !exact_resistor(&netlist.elements[4], "R2", &["3", "0"], 100.0)
        {
            return Err(format!("{LABEL} {} exact topology changed", role.label()));
        }

        let expected_model_params: [(&str, Value); 4] = [
            ("RON", 1.0e-6),
            ("ROFF", 1.0e6),
            ("VON", 1.0),
            ("VOFF", 0.0),
        ];
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("SW")
            || !model.model_type.eq_ignore_ascii_case("VSWITCH")
            || model.params.len() != expected_model_params.len()
            || expected_model_params
                .iter()
                .any(|(expected_name, expected_value)| {
                    !model.params.iter().any(|(name, value)| {
                        name.eq_ignore_ascii_case(expected_name)
                            && value.to_bits() == expected_value.to_bits()
                    })
                })
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} exact VSWITCH model changed"));
        }
        if !matches!(
            &netlist.analyses[0],
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode: DcSweepMode::Linear,
                sweep2: None,
                ..
            } if source.eq_ignore_ascii_case("V1")
                && start.to_bits() == 5.0f64.to_bits()
                && stop.to_bits() == 5.0f64.to_bits()
                && step.to_bits() == 1.0f64.to_bits()
        ) {
            return Err(format!("{LABEL} exact one-point DC command changed"));
        }
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.dependencies.len() != 1
        {
            return Err(format!("{LABEL} exact .PRINT DC request changed"));
        }
        Ok(netlist)
    }

    fn run_bug113_worker(
        &self,
        role: Bug113Role,
        netlist: &Netlist,
        start: Instant,
    ) -> Result<Value, String> {
        let engine = self.create_dc_engine();
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let results = engine
            .run_dc_sweep_with_abort(netlist, "V1", 5.0, 5.0, 1.0, &abort)
            .map_err(|error| format!("{LABEL} {} failed: {error}", role.label()))?;
        let [(sweep, result)] = results.as_slice() else {
            return Err(format!(
                "{LABEL} {} produced {} DC points instead of one",
                role.label(),
                results.len()
            ));
        };
        let current = result
            .branch_current_named("V1")
            .ok_or_else(|| format!("{LABEL} {} lost I(V1)", role.label()))?;
        let quality = engine.convergence_quality();
        if sweep.to_bits() != 5.0f64.to_bits()
            || !current.is_finite()
            || quality.total_iterations != role.expected_jacobians()
            || quality.gmin_stepping_count != 0
            || quality.source_stepping_count != 0
            || quality.force_accepted_points != 0
        {
            return Err(format!(
                "{LABEL} {} historical observation changed: sweep={sweep:.17e}, I(V1)={current:.17e}, quality={quality:?}",
                role.label()
            ));
        }
        Ok(current)
    }

    pub(super) fn validate_bug113_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let mut currents = BTreeMap::new();
        for worker in Bug113Role::WORKERS {
            let path = self.root.join(worker.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            let netlist = self.validate_bug113_worker(worker, &source, &path)?;
            currents.insert(worker, self.run_bug113_worker(worker, &netlist, start)?);
        }
        let noic = currents[&Bug113Role::NoInitialState];
        let explicit_on = currents[&Bug113Role::ExplicitOn];
        if (noic - explicit_on).abs() > 1.0e-15
            || (noic - -0.05).abs() > 1.0e-9
            || Bug113Role::NoInitialState.expected_jacobians()
                != Bug113Role::ExplicitOn.expected_jacobians() + 1
        {
            return Err(format!(
                "{LABEL} relational result changed: no-initial={noic:.17e}, explicit-ON={explicit_on:.17e}"
            ));
        }
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} execution exceeded timeout ({}ms)",
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
    fn bug113_workers_preserve_typed_initial_state_and_exact_jacobians() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let mut currents = BTreeMap::new();
        for role in Bug113Role::WORKERS {
            let path = root.join(role.path());
            let source = fs::read_to_string(&path).expect("read canonical BUG113 worker");
            let netlist = runner
                .validate_bug113_worker(role, &source, &path)
                .expect("canonical BUG113 worker remains typed");
            currents.insert(
                role,
                runner
                    .run_bug113_worker(role, &netlist, Instant::now())
                    .expect("canonical BUG113 worker reproduces Jacobian count"),
            );
        }
        assert!(
            (currents[&Bug113Role::NoInitialState] - currents[&Bug113Role::ExplicitOn]).abs()
                <= 1.0e-15
        );

        let path = root.join(EXPLICIT_ON_PATH);
        let source = fs::read_to_string(&path).expect("read explicit-ON worker");
        for mutation in [
            source.replace("SW  ON", "SW OFF"),
            source.replace("RON=1u", "RON=2u"),
            source.replace("i(v1)", "v(2)"),
        ] {
            assert!(
                runner
                    .validate_bug113_worker(Bug113Role::ExplicitOn, &mutation, &path)
                    .is_err(),
                "initial-state, model, and observation mutations must fail closed"
            );
        }
    }

    #[test]
    fn bug113_oracle_rejects_an_expired_deadline() {
        let mut runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner.config.max_time_per_test_ms = 1;
        let expired_start = Instant::now()
            .checked_sub(Duration::from_millis(2))
            .expect("construct expired BUG113 deadline");
        let error = runner
            .validate_bug113_oracle(expired_start)
            .expect_err("an expired BUG113 deadline must fail closed");
        assert!(error.contains("deadline"));
    }
}
