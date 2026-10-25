use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_1455 model-parameter punctuation relation";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug1455Role {
    EqualsOwner,
    WhitespaceReference,
}

impl Bug1455Role {
    const ALL: [Self; 2] = [Self::EqualsOwner, Self::WhitespaceReference];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::EqualsOwner => XYCE_BUG1455_OWNER_CONTRACT,
            Self::WhitespaceReference => XYCE_BUG1455_REFERENCE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::EqualsOwner => XYCE_BUG1455_OWNER_PATH,
            Self::WhitespaceReference => XYCE_BUG1455_REFERENCE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::EqualsOwner => XYCE_BUG1455_OWNER_RECORD,
            Self::WhitespaceReference => XYCE_BUG1455_REFERENCE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG1455 path has a file name")
    }
}

impl XyceTestRunner {
    fn validate_bug1455_plan(role: Bug1455Role, plan: &XyceStaticDcPlan) -> Result<(), String> {
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
            || !plan.dc.source.eq_ignore_ascii_case("VDD")
            || plan.dc.start.to_bits() != 0.0f64.to_bits()
            || plan.dc.stop.to_bits() != 5.0f64.to_bits()
            || plan.dc.step.to_bits() != 1.0f64.to_bits()
            || probes != ["v(2)", "i(vmon)", "v(2,3)", "v(2,1)"]
            || !plan.diagnostics.is_empty()
        {
            return Err(format!(
                "{LABEL} {} DC plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn exact_params(actual: &[(String, Value)], expected: &[(&str, Value)]) -> bool {
        if actual.len() != expected.len() {
            return false;
        }
        expected.iter().all(|(name, value)| {
            actual.iter().any(|(actual_name, actual_value)| {
                actual_name.eq_ignore_ascii_case(name) && actual_value.to_bits() == value.to_bits()
            })
        })
    }

    fn validate_resistor(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        resistance: Value,
    ) -> Result<(), String> {
        if element.provenance != ElementProvenance::Authored
            || !element.name.eq_ignore_ascii_case(name)
            || !Self::nodes_match(&element.nodes, &nodes)
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == resistance.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} resistor {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug1455_netlist(role: Bug1455Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 6
            || netlist.models.len() != 1
            || netlist.output_requests.len() != 1
            || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                mode: DcSweepMode::Linear,
                sweep2: None,
                ..
            }] if source.eq_ignore_ascii_case("VDD")
                && start.to_bits() == 0.0f64.to_bits()
                && stop.to_bits() == 5.0f64.to_bits()
                && step.to_bits() == 1.0f64.to_bits())
            || !netlist.subcircuits.is_empty()
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
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed",
                role.file_name()
            ));
        }
        let model = &netlist.models[0];
        if !model.name.eq_ignore_ascii_case("PFET")
            || !model.model_type.eq_ignore_ascii_case("PMOS")
            || !Self::exact_params(
                &model.params,
                &[("LEVEL", 1.0), ("KP", 25.0e-6), ("VTO", -0.8)],
            )
            || !model.expr_params.is_empty()
            || !model.string_params.is_empty()
            || !model.string_vector_params.is_empty()
            || !model.real_vector_params.is_empty()
            || !model.real_vector_expr_params.is_empty()
            || !model.integer_vector_params.is_empty()
        {
            return Err(format!("{LABEL} PFET model changed: {model:?}"));
        }

        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vdd", "r1", "r2", "rd", "vmon", "m1"])
        {
            return Err(format!("{LABEL} element inventory changed"));
        }
        Self::validate_resistor(elements["r1"], "R1", ["2", "1"], 50e3)?;
        Self::validate_resistor(elements["r2"], "R2", ["1", "0"], 50e3)?;
        Self::validate_resistor(elements["rd"], "RD", ["4", "0"], 7.5e3)?;
        for (name, nodes, expected) in [("vdd", ["2", "0"], 5.0f64), ("vmon", ["3", "4"], 0.0f64)] {
            let source = elements[name];
            if source.provenance != ElementProvenance::Authored
                || !Self::nodes_match(&source.nodes, &nodes)
                || !matches!(source.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                    if value.to_bits() == expected.to_bits())
            {
                return Err(format!("{LABEL} voltage source {name} changed"));
            }
        }
        let mosfet = elements["m1"];
        let ElementKind::Mosfet {
            model,
            mos_type: _,
            compact_syntax: false,
            instance_params,
            deferred_params,
        } = &mosfet.kind
        else {
            return Err(format!("{LABEL} M1 kind changed: {mosfet:?}"));
        };
        if mosfet.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&mosfet.nodes, &["3", "1", "2", "2"])
            || !model.eq_ignore_ascii_case("PFET")
            || !Self::exact_params(instance_params, &[("L", 10.0e-6), ("W", 160.0e-6)])
            || !deferred_params.is_empty()
        {
            return Err(format!("{LABEL} M1 topology or geometry changed"));
        }

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request
                .dependencies
                .iter()
                .any(|dependency| dependency.expression)
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }
        let devices = request
            .dependencies
            .iter()
            .filter(|dependency| dependency.kind == OutputSymbolKind::Device)
            .collect::<Vec<_>>();
        let nodes = request
            .dependencies
            .iter()
            .filter(|dependency| dependency.kind == OutputSymbolKind::Node)
            .map(|dependency| dependency.symbol.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if devices.len() != 1
            || !devices[0].operator.eq_ignore_ascii_case("I")
            || !devices[0].symbol.eq_ignore_ascii_case("VMON")
            || nodes != BTreeSet::from(["1".to_string(), "2".to_string(), "3".to_string()])
        {
            return Err(format!(
                "{LABEL} typed .PRINT dependencies changed: {request:?}"
            ));
        }
        Ok(())
    }

    fn bug1455_analytic_row(vdd: Value) -> [Value; 5] {
        let vsg = vdd * 0.5;
        let overdrive = (vsg - 0.8).max(0.0);
        let beta = 25.0e-6 * (160.0e-6 / 10.0e-6);
        let rd = 7.5e3;
        let (current, vsd) = if overdrive == 0.0 {
            (0.0, vdd)
        } else {
            let saturation_current = 0.5 * beta * overdrive * overdrive;
            let saturation_vsd = vdd - rd * saturation_current;
            if saturation_vsd >= overdrive {
                (saturation_current, saturation_vsd)
            } else {
                let scaled = rd * beta;
                let linear = scaled * overdrive + 1.0;
                let discriminant = linear * linear - 2.0 * scaled * vdd;
                let vsd = (linear - discriminant.sqrt()) / scaled;
                ((vdd - vsd) / rd, vsd)
            }
        };
        [vdd, current, vsd, vdd * 0.5, vdd - vsd]
    }

    fn validate_bug1455_table(role: Bug1455Role, table: &XycePrnTable) -> Result<(), String> {
        const COLUMNS: [&str; 5] = ["Index", "V(2)", "I(VMON)", "V(2,3)", "V(2,1)"];
        if table.columns.len() != COLUMNS.len()
            || table
                .columns
                .iter()
                .zip(COLUMNS)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() != 6
        {
            return Err(format!(
                "{LABEL} {} table shape changed: {table:?}",
                role.file_name()
            ));
        }
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != COLUMNS.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed: {row:?}",
                    role.file_name()
                ));
            }
            let expected = Self::bug1455_analytic_row(index as Value);
            for (column, (actual, expected)) in
                row[1..].iter().zip(expected[..4].iter()).enumerate()
            {
                let absolute = if column == 1 { 5e-10 } else { 5e-6 };
                let relative = expected.abs() * 2e-5;
                if (*actual - *expected).abs() > absolute + relative {
                    return Err(format!(
                        "{LABEL} {} row {index} column {} violated MOS1/load-line oracle: actual={actual}, expected={expected}",
                        role.file_name(),
                        column + 1,
                    ));
                }
            }
        }
        Ok(())
    }

    fn bug1455_plan(&self, role: Bug1455Role) -> Result<XyceStaticDcPlan, String> {
        let plan =
            self.static_dc_plan_for_path(&self.root.join(role.path()), ExpressionDialect::Xyce)?;
        Self::validate_bug1455_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug1455_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before execution"
            ));
        }
        let run = |member_role: Bug1455Role| {
            let plan = self.bug1455_plan(member_role)?;
            let (netlist, results) = self.run_static_dc_results(&plan, start).map_err(|error| {
                format!(
                    "{LABEL} {} execution failed: {error}",
                    member_role.file_name()
                )
            })?;
            Self::validate_bug1455_netlist(member_role, &netlist)?;
            let table = self.dc_results_to_prn_table(&plan, &netlist, &results)?;
            Self::validate_bug1455_table(member_role, &table)?;
            Ok::<_, String>(table)
        };

        let owner = run(Bug1455Role::EqualsOwner)?;
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline between independent runs"
            ));
        }
        let reference = run(Bug1455Role::WhitespaceReference)?;
        let mismatches =
            self.compare_serialized_default_prn_tables_case_insensitive(&owner, &reference)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} failed its case-insensitive byte relation: {mismatches:?}"
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

    #[test]
    fn bug1455_both_roles_execute_the_model_punctuation_relation() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_bug1455_oracle(Instant::now())
            .expect("canonical BUG1455 relation passes");
    }

    #[test]
    fn bug1455_analytic_oracle_rejects_counterfactuals() {
        let mut table = XycePrnTable {
            columns: vec![
                "Index".into(),
                "V(2)".into(),
                "I(VMON)".into(),
                "V(2,3)".into(),
                "V(2,1)".into(),
            ],
            rows: (0..=5)
                .map(|index| {
                    let expected = XyceTestRunner::bug1455_analytic_row(index as Value);
                    vec![
                        index as Value,
                        expected[0],
                        expected[1],
                        expected[2],
                        expected[3],
                    ]
                })
                .collect(),
        };
        XyceTestRunner::validate_bug1455_table(Bug1455Role::EqualsOwner, &table)
            .expect("canonical analytic table");
        table.rows[5][2] *= 0.5;
        assert!(XyceTestRunner::validate_bug1455_table(Bug1455Role::EqualsOwner, &table).is_err());
    }

    #[test]
    fn bug1455_source_mutations_fail_typed_admission() {
        let source = fs::read_to_string(corpus_root().join(XYCE_BUG1455_OWNER_PATH))
            .expect("read BUG1455 owner");
        for mutated in [
            source.replace("KP=25U", "KP=20U"),
            source.replace("VTO=-0.8V", "VTO=-0.7V"),
            source.replace("RD 4 0 7.5K", "RD 4 0 8K"),
            source.replace("M1 3 1 2 2", "M1 4 1 2 2"),
            source.replace("W=160U", "W=80U"),
            source.replace(".dc VDD 0 5 1", ".dc VDD 0 4 1"),
            source.replace("I(VMON)", "I(VDD)"),
        ] {
            let netlist = XyceTestRunner::parse_xyce_netlist(
                &mutated,
                &corpus_root().join(XYCE_BUG1455_OWNER_PATH),
            )
            .expect("mutated BUG1455 source remains parseable");
            assert!(
                XyceTestRunner::validate_bug1455_netlist(Bug1455Role::EqualsOwner, &netlist)
                    .is_err(),
                "mutation escaped typed admission: {mutated}"
            );
        }
    }

    #[test]
    fn bug1455_expired_deadline_fails_closed() {
        let runner = XyceTestRunner::new(
            corpus_root(),
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_bug1455_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
