use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "ISSUE_451 hierarchical-node operating-point relation";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Issue451Role {
    HierarchicalOwner,
    ExplicitReference,
}

impl Issue451Role {
    const ALL: [Self; 2] = [Self::HierarchicalOwner, Self::ExplicitReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => XYCE_ISSUE451_OWNER_CONTRACT,
            Self::ExplicitReference => XYCE_ISSUE451_REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => XYCE_ISSUE451_OWNER_PATH,
            Self::ExplicitReference => XYCE_ISSUE451_REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::HierarchicalOwner => XYCE_ISSUE451_OWNER_RECORD,
            Self::ExplicitReference => XYCE_ISSUE451_REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("ISSUE451 path has a file name")
    }
}

impl XyceTestRunner {
    fn validate_issue451_plan(role: Issue451Role, plan: &XyceStaticDcPlan) -> Result<(), String> {
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if plan.execution_dir.is_some()
            || plan.dc_data.is_some()
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || plan.dc.sweep2.is_some()
            || !matches!(plan.dc.mode, DcSweepMode::Linear)
            || !plan.dc.source.eq_ignore_ascii_case("Vin")
            || plan.dc.start.to_bits() != 1.0f64.to_bits()
            || plan.dc.stop.to_bits() != 1.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || probes != ["v(3)", "v(eric)", "v(2)", "v(1)", "v(x1:testnode)"]
            || !plan.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_issue451_resistor(
        element: &rspice_core::netlist::Element,
        nodes: [&str; 2],
    ) -> Result<(), String> {
        if element.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&element.nodes, &nodes)
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} resistor changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_issue451_current_source(
        element: &rspice_core::netlist::Element,
        nodes: [&str; 2],
    ) -> Result<(), String> {
        if element.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&element.nodes, &nodes)
            || !matches!(element.kind, ElementKind::CurrentSource(SourceSpec::Dc(value))
                if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{LABEL} current source changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_issue451_deferred_current_source(
        element: &rspice_core::netlist::Element,
        nodes: [&str; 2],
    ) -> Result<(), String> {
        if element.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&element.nodes, &nodes)
            || !matches!(&element.kind, ElementKind::CurrentSourceDeferred(expression)
                if expression.trim() == "1.0")
        {
            return Err(format!(
                "{LABEL} deferred current source changed: {element:?}"
            ));
        }
        Ok(())
    }

    fn validate_issue451_subcircuit(
        role: Issue451Role,
        subcircuit: &SubcircuitDef,
    ) -> Result<(), String> {
        let expected_count = match role {
            Issue451Role::HierarchicalOwner => 2,
            Issue451Role::ExplicitReference => 5,
        };
        if !subcircuit.name.eq_ignore_ascii_case("test")
            || !Self::nodes_match(&subcircuit.ports, &["A", "B"])
            || subcircuit.elements.len() != expected_count
            || !subcircuit.initial_conditions.is_empty()
            || !subcircuit.node_sets.is_empty()
            || !subcircuit.params.is_empty()
            || !subcircuit.expr_params.is_empty()
            || !subcircuit.string_params.is_empty()
            || !subcircuit.body_params.is_empty()
            || !subcircuit.body_expr_params.is_empty()
            || !subcircuit.body_string_params.is_empty()
            || !subcircuit.body_functions.is_empty()
            || !subcircuit.local_options.is_empty()
            || subcircuit.library_ref.is_some()
            || !subcircuit.nested_subcircuits.is_empty()
        {
            return Err(format!("{LABEL} subcircuit shape changed: {subcircuit:?}"));
        }
        let elements = subcircuit
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        let expected = match role {
            Issue451Role::HierarchicalOwner => BTreeSet::from(["rt1", "rt2"]),
            Issue451Role::ExplicitReference => {
                BTreeSet::from(["rt1", "rt2", "itest2", "rtest2", "rtest3"])
            }
        };
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected {
            return Err(format!("{LABEL} subcircuit element inventory changed"));
        }
        Self::validate_issue451_resistor(elements["rt1"], ["A", "testNode"])?;
        Self::validate_issue451_resistor(elements["rt2"], ["testNode", "B"])?;
        if role == Issue451Role::ExplicitReference {
            Self::validate_issue451_deferred_current_source(elements["itest2"], ["0", "fred"])?;
            Self::validate_issue451_resistor(elements["rtest2"], ["testNode", "fred"])?;
            Self::validate_issue451_resistor(elements["rtest3"], ["testNode", "0"])?;
        }
        Ok(())
    }

    fn validate_issue451_netlist(role: Issue451Role, netlist: &Netlist) -> Result<(), String> {
        let expected_elements = match role {
            Issue451Role::HierarchicalOwner => 9,
            Issue451Role::ExplicitReference => 6,
        };
        if netlist.elements.len() != expected_elements
            || netlist.subcircuits.len() != 1
            || netlist.output_requests.len() != 1
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Op])
            || !netlist.models.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }

        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        let expected = match role {
            Issue451Role::HierarchicalOwner => BTreeSet::from([
                "vin", "rin", "x1", "rout", "btest1", "rtest1", "itest2", "rtest2", "rtest3",
            ]),
            Issue451Role::ExplicitReference => {
                BTreeSet::from(["vin", "rin", "x1", "rout", "btest1", "rtest1"])
            }
        };
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>() != expected {
            return Err(format!("{LABEL} top-level element inventory changed"));
        }

        let vin = elements["vin"];
        if vin.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&vin.nodes, &["1", "0"])
            || !matches!(vin.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 1.0f64.to_bits())
        {
            return Err(format!("{LABEL} Vin changed: {vin:?}"));
        }
        Self::validate_issue451_resistor(elements["rin"], ["1", "2"])?;
        Self::validate_issue451_resistor(elements["rout"], ["3", "0"])?;
        Self::validate_issue451_resistor(elements["rtest1"], ["eric", "0"])?;

        let instance = elements["x1"];
        if instance.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&instance.nodes, &["2", "3"])
            || !matches!(&instance.kind, ElementKind::Subcircuit { subckt_name, params }
                if subckt_name.eq_ignore_ascii_case("test") && params.is_empty())
        {
            return Err(format!("{LABEL} X1 changed: {instance:?}"));
        }
        let behavioral = elements["btest1"];
        let expression = match &behavioral.kind {
            ElementKind::BehavioralVoltage { expression, .. } => expression,
            _ => return Err(format!("{LABEL} Btest1 changed: {behavioral:?}")),
        };
        let normalized_expression = expression
            .chars()
            .filter(|character| !character.is_whitespace())
            .collect::<String>()
            .to_ascii_lowercase();
        if behavioral.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&behavioral.nodes, &["eric", "0"])
            || normalized_expression != "v(x1:testnode)"
        {
            return Err(format!("{LABEL} Btest1 hierarchy reference changed"));
        }

        if role == Issue451Role::HierarchicalOwner {
            Self::validate_issue451_current_source(elements["itest2"], ["0", "fred"])?;
            Self::validate_issue451_resistor(elements["rtest2"], ["X1:testNode", "fred"])?;
            Self::validate_issue451_resistor(elements["rtest3"], ["X1:testNode", "0"])?;
        }
        Self::validate_issue451_subcircuit(role, &netlist.subcircuits[0])?;

        let request = &netlist.output_requests[0];
        let dependencies = ["3", "eric", "2", "1", "X1:testnode"];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != dependencies.len()
            || request
                .dependencies
                .iter()
                .zip(dependencies)
                .any(|(actual, symbol)| {
                    actual.expression
                        || actual.kind != OutputSymbolKind::Node
                        || !actual.operator.eq_ignore_ascii_case("V")
                        || !actual.symbol.eq_ignore_ascii_case(symbol)
                })
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }
        Ok(())
    }

    fn validate_issue451_table(role: Issue451Role, table: &XycePrnTable) -> Result<(), String> {
        const COLUMNS: [&str; 6] = ["Index", "V(3)", "V(eric)", "V(2)", "V(1)", "V(x1:testnode)"];
        if table.columns.len() != COLUMNS.len()
            || table
                .columns
                .iter()
                .zip(COLUMNS)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != 1
            || table.rows[0].len() != COLUMNS.len()
            || table.rows[0].iter().any(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        for (actual, expected, name) in [
            (table.rows[0][0], 0.0, "Index"),
            (table.rows[0][1], 0.375, "V(3)"),
            (table.rows[0][2], 0.75, "V(eric)"),
            (table.rows[0][3], 0.875, "V(2)"),
            (table.rows[0][4], 1.0, "V(1)"),
            (table.rows[0][5], 0.75, "V(X1:testNode)"),
        ] {
            let actual = Self::xyce_default_prn_roundtrip(actual)?;
            let expected = Self::xyce_default_prn_roundtrip(expected)?;
            if actual != expected {
                return Err(format!(
                    "{LABEL} {} {name} analytic value changed: {actual} != {expected}",
                    role.file_name()
                ));
            }
        }
        Ok(())
    }

    fn issue451_plan(&self, role: Issue451Role) -> Result<XyceStaticDcPlan, String> {
        let plan =
            self.static_dc_plan_for_path(&self.root.join(role.path()), ExpressionDialect::Xyce)?;
        Self::validate_issue451_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_issue451_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before execution"
            ));
        }
        let run = |member_role: Issue451Role| {
            let plan = self.issue451_plan(member_role)?;
            let (netlist, results) = self.run_static_dc_results(&plan, start).map_err(|error| {
                format!(
                    "{LABEL} {} execution failed: {error}",
                    member_role.file_name()
                )
            })?;
            Self::validate_issue451_netlist(member_role, &netlist)?;
            let table = self.dc_results_to_prn_table(&plan, &netlist, &results)?;
            Self::validate_issue451_table(member_role, &table)?;
            Ok::<_, String>((results, table))
        };

        let (_owner_results, owner_table) = run(Issue451Role::HierarchicalOwner)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline between independent runs"
            ));
        }
        let (_reference_results, reference_table) = run(Issue451Role::ExplicitReference)?;

        let exact = self.compare_serialized_default_prn_tables(&owner_table, &reference_table)?;
        if !exact.is_empty() {
            return Err(format!(
                "{LABEL} failed the strengthened exact owner/reference relation: {exact:?}"
            ));
        }

        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline"));
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

    fn parse_role(role: Issue451Role, source: &str) -> Result<Netlist, ParseError> {
        XyceTestRunner::parse_xyce_netlist(source, &corpus_root().join(role.path()))
    }

    #[test]
    fn issue451_both_roles_execute_the_exact_hierarchical_relation() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_issue451_oracle(Instant::now())
            .expect("canonical ISSUE451 relation passes");
    }

    #[test]
    fn issue451_typed_envelope_rejects_hierarchy_and_topology_mutations() {
        for role in Issue451Role::ALL {
            let source = fs::read_to_string(corpus_root().join(role.path()))
                .expect("read canonical ISSUE451 source");
            let netlist = parse_role(role, &source).expect("parse canonical ISSUE451 source");
            XyceTestRunner::validate_issue451_netlist(role, &netlist)
                .expect("canonical ISSUE451 typed envelope");
        }

        let owner =
            fs::read_to_string(corpus_root().join(XYCE_ISSUE451_OWNER_PATH)).expect("read owner");
        for mutated in [
            owner.replace("V(X1:testNode)", "V(testNode)"),
            owner.replace("Rtest3 X1:testNode 0 1.0", "Rtest3 X1:testNode 0 2.0"),
            owner.replace("Itest2 0 fred 1.0", "Itest2 fred 0 1.0"),
            owner.replace("X1 2 3 test", "X1 3 2 test"),
            owner.replace(".op", ".dc Vin 1 1 1"),
            owner.replace("v(x1:testnode)", "v(fred)"),
        ] {
            let parsed = parse_role(Issue451Role::HierarchicalOwner, &mutated)
                .expect("mutated ISSUE451 source remains parseable");
            assert!(
                XyceTestRunner::validate_issue451_netlist(
                    Issue451Role::HierarchicalOwner,
                    &parsed,
                )
                .is_err()
            );
        }
    }

    #[test]
    fn issue451_analytic_table_rejects_counterfactuals() {
        let canonical = XycePrnTable {
            columns: vec![
                "Index".into(),
                "V(3)".into(),
                "V(eric)".into(),
                "V(2)".into(),
                "V(1)".into(),
                "V(x1:testnode)".into(),
            ],
            rows: vec![vec![0.0, 0.375, 0.75, 0.875, 1.0, 0.75]],
        };
        XyceTestRunner::validate_issue451_table(Issue451Role::HierarchicalOwner, &canonical)
            .expect("canonical ISSUE451 analytic table");
        let mut wrong = canonical.clone();
        wrong.rows[0][5] = 0.5;
        assert!(
            XyceTestRunner::validate_issue451_table(Issue451Role::HierarchicalOwner, &wrong,)
                .is_err()
        );
        let mut malformed = canonical;
        malformed.columns.swap(1, 2);
        assert!(
            XyceTestRunner::validate_issue451_table(Issue451Role::HierarchicalOwner, &malformed,)
                .is_err()
        );
    }

    #[test]
    fn issue451_expired_deadline_fails_closed() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_issue451_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
