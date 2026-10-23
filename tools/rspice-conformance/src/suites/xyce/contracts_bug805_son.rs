use super::*;
use rspice_core::netlist::{OutputSymbolKind, SaveSignal, SourceSpec};

const LABEL: &str = "BUG_805_SON include-partition mutual-inductor relation";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug805SonRole {
    InlineOwner,
    SingleInclude,
    NestedInclude,
}

impl Bug805SonRole {
    const ALL: [Self; 3] = [Self::InlineOwner, Self::SingleInclude, Self::NestedInclude];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn path(self) -> &'static str {
        match self {
            Self::InlineOwner => "Netlists/Certification_Tests/BUG_805_SON/bug805_all.cir",
            Self::SingleInclude => "Netlists/Certification_Tests/BUG_805_SON/bug805_top.cir",
            Self::NestedInclude => "Netlists/Certification_Tests/BUG_805_SON/bug805_top2.cir",
        }
    }

    fn record(self) -> String {
        XyceTestRunner::normalize_manifest_key(self.path())
    }

    fn file_name(self) -> &'static str {
        self.path()
            .rsplit('/')
            .next()
            .expect("BUG805SON path has a name")
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::InlineOwner => "bug805son_inline_wrapper_owner",
            Self::SingleInclude => "bug805son_single_include_worker",
            Self::NestedInclude => "bug805son_nested_include_worker",
        }
    }
}

impl XyceTestRunner {
    fn bug805_son_nodes_match(nodes: &[String], expected: &[&str]) -> bool {
        nodes.len() == expected.len()
            && nodes
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug805_son_passive(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value: Value,
        inductor: bool,
    ) -> Result<(), String> {
        let kind_matches = if inductor {
            matches!(&element.kind, ElementKind::Inductor {
                value: actual, value_expr: None, initial_current: None, model: None,
                instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        } else {
            matches!(&element.kind, ElementKind::Resistor {
                value: actual, value_expr: None, model: None, instance_params, deferred_params,
            } if actual.to_bits() == value.to_bits()
                && instance_params.is_empty() && deferred_params.is_empty())
        };
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !Self::bug805_son_nodes_match(&element.nodes, &nodes)
            || !kind_matches
        {
            return Err(format!("{LABEL} passive {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug805_son_coupling(
        element: &rspice_core::netlist::Element,
        name: &str,
        right: &str,
        coefficient: Value,
    ) -> Result<(), String> {
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || !element.nodes.is_empty()
            || !matches!(&element.kind, ElementKind::Coupling { inductors, coefficient: actual, coefficient_expr: None, model: None }
                if actual.to_bits() == coefficient.to_bits()
                    && inductors.len() == 2
                    && inductors[0].eq_ignore_ascii_case("L1")
                    && inductors[1].eq_ignore_ascii_case(right))
        {
            return Err(format!("{LABEL} coupling {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn bug805_son_expected_probes() -> Vec<String> {
        ["i(vs)", "v(2)", "v(3)", "v(4)"]
            .into_iter()
            .map(str::to_string)
            .collect()
    }

    fn validate_bug805_son_netlist(netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 9
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.params.all_params().is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !Self::analytic_timeint_only_options_match(&netlist.options, None, None, None, None)
        {
            return Err(format!("{LABEL} typed envelope changed"));
        }
        let elements = netlist
            .elements
            .iter()
            .map(|element| (element.name.to_ascii_lowercase(), element))
            .collect::<BTreeMap<_, _>>();
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["vs", "r1", "l1", "r2", "l2", "k1", "r3", "l3", "k2"])
        {
            return Err(format!("{LABEL} element inventory changed"));
        }
        let source = elements["vs"];
        if source.provenance != ElementProvenance::Authored
            || !Self::bug805_son_nodes_match(&source.nodes, &["1", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset, amplitude, frequency, delay, damping, phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 169.7f64.to_bits()
                && frequency.to_bits() == 60.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} source changed: {source:?}"));
        }
        Self::validate_bug805_son_passive(elements["r1"], "R1", ["1", "2"], 1.0e3, false)?;
        Self::validate_bug805_son_passive(elements["l1"], "L1", ["2", "0"], 1.0e-3, true)?;
        Self::validate_bug805_son_passive(elements["r2"], "R2", ["3", "0"], 1.0e3, false)?;
        Self::validate_bug805_son_passive(elements["l2"], "L2", ["3", "0"], 1.0e-3, true)?;
        Self::validate_bug805_son_coupling(elements["k1"], "K1", "L2", 0.75)?;
        Self::validate_bug805_son_passive(elements["r3"], "R3", ["4", "0"], 1.0e3, false)?;
        Self::validate_bug805_son_passive(elements["l3"], "L3", ["4", "0"], 1.0e-3, true)?;
        Self::validate_bug805_son_coupling(elements["k2"], "K2", "L3", 0.8)?;

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == 100.0e-6f64.to_bits()
            && stop.to_bits() == (25.0e-3f64).to_bits())
        {
            return Err(format!("{LABEL} typed transient changed"));
        }
        let request = &netlist.output_requests[0];
        let expected_probes = Self::bug805_son_expected_probes();
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != 4
            || request
                .dependencies
                .iter()
                .enumerate()
                .any(|(index, dependency)| {
                    dependency.expression
                        || if index == 0 {
                            dependency.kind != OutputSymbolKind::Device
                                || !dependency.operator.eq_ignore_ascii_case("I")
                                || !dependency.symbol.eq_ignore_ascii_case("VS")
                        } else {
                            dependency.kind != OutputSymbolKind::Node
                                || !dependency.operator.eq_ignore_ascii_case("V")
                                || dependency.symbol != (index + 1).to_string()
                        }
                })
        {
            return Err(format!("{LABEL} typed PRINT request changed: {request:?}"));
        }
        let saves = netlist
            .saves
            .signals
            .iter()
            .map(|signal| match signal {
                SaveSignal::Current(name) => format!("i({})", name.to_ascii_lowercase()),
                SaveSignal::Voltage(node) => format!("v({})", node.to_ascii_lowercase()),
                other => format!("{other:?}"),
            })
            .collect::<Vec<_>>();
        if saves != expected_probes {
            return Err(format!("{LABEL} SaveSet changed: {saves:?}"));
        }
        Ok(())
    }

    fn bug805_son_plan(&self, role: Bug805SonRole) -> Result<XyceStaticTranPlan, String> {
        let purpose = if role == Bug805SonRole::InlineOwner {
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
        } else {
            XyceStaticTranPlanPurpose::RelationalFamily
        };
        let plan =
            self.static_tran_plan_for_path_with_purpose(&self.root.join(role.path()), purpose)?;
        let print = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} lost PRINT"))?;
        let expected_contract = if role == Bug805SonRole::InlineOwner {
            XyceStaticTranContract::WrapperStatic
        } else {
            XyceStaticTranContract::PlainStatic
        };
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != expected_contract
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.tran.step.to_bits() != 100.0e-6f64.to_bits()
            || plan.tran.stop.to_bits() != (25.0e-3f64).to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || probes != Self::bug805_son_expected_probes()
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(plan)
    }

    fn validate_bug805_son_table(table: &XycePrnTable) -> Result<(), String> {
        let expected = ["Index", "TIME", "I(VS)", "V(2)", "V(3)", "V(4)"];
        if table.columns.len() != expected.len()
            || table
                .columns
                .iter()
                .zip(expected)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() < 3
        {
            return Err(format!("{LABEL} output shape changed"));
        }
        let mut prior_time = None;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != expected.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || prior_time.is_some_and(|prior| row[1] <= prior)
            {
                return Err(format!("{LABEL} output row {index} is malformed"));
            }
            prior_time = Some(row[1]);
        }
        let first = &table.rows[0];
        let last = table.rows.last().expect("BUG805SON table is nonempty");
        let maxima = [2usize, 3, 4, 5].map(|column| {
            table
                .rows
                .iter()
                .map(|row| row[column].abs())
                .fold(0.0, Value::max)
        });
        let proportional_rows = table
            .rows
            .iter()
            .filter(|row| row[1] >= 1.0e-3)
            .filter(|row| row[4].abs().max(row[5].abs()) >= 1.0e-6)
            .collect::<Vec<_>>();
        let proportional = !proportional_rows.is_empty()
            && proportional_rows.iter().all(|row| {
                let residual = 0.8 * row[4] - 0.75 * row[5];
                residual.abs() <= 1.0e-8 + 1.0e-6 * row[4].abs().max(row[5].abs())
            });
        if first[1].to_bits() != 0.0f64.to_bits()
            || first[2..]
                .iter()
                .any(|value| value.to_bits() != 0.0f64.to_bits())
            || (last[1] - 25.0e-3).abs() > 1.0e-14
            || !(0.14..0.20).contains(&maxima[0])
            || !(0.04..0.09).contains(&maxima[1])
            || !(0.03..0.07).contains(&maxima[2])
            || !(0.03..0.08).contains(&maxima[3])
            || !proportional
        {
            return Err(format!(
                "{LABEL} output became vacuous or lost coupling: maxima={maxima:?}"
            ));
        }
        Ok(())
    }

    pub(super) fn validate_bug805_son_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let mut tables = BTreeMap::new();
        for member_role in Bug805SonRole::ALL {
            if abort.is_aborted() {
                return Err(format!("{LABEL} deadline expired between independent runs"));
            }
            let plan = self.bug805_son_plan(member_role)?;
            let parsed =
                Self::parse_xyce_netlist(&plan.source, &plan.deck_path).map_err(|error| {
                    format!("{LABEL} {} parse failed: {error}", member_role.file_name())
                })?;
            Self::validate_bug805_son_netlist(&parsed)?;
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!(
                        "{LABEL} {} execution failed: {other}",
                        member_role.file_name()
                    ),
                })?;
            Self::validate_bug805_son_netlist(&netlist)?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
            Self::validate_bug805_son_table(&table)?;
            tables.insert(member_role, table);
        }
        let owner = tables
            .get(&Bug805SonRole::InlineOwner)
            .expect("BUG805SON owner ran");
        for worker in [Bug805SonRole::SingleInclude, Bug805SonRole::NestedInclude] {
            let mismatches = self.compare_serialized_default_prn_tables(
                owner,
                tables.get(&worker).expect("BUG805SON worker ran"),
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {} default-PRN relation failed: {mismatches:?}",
                    worker.file_name()
                ));
            }
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
    fn bug805_son_all_roles_execute_the_include_relation() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug805_son_oracle(Instant::now())
            .expect("BUG805SON native include relation");
    }

    #[test]
    fn bug805_son_typed_and_numeric_counterfactuals_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let plan = runner
            .bug805_son_plan(Bug805SonRole::InlineOwner)
            .expect("canonical BUG805SON plan");
        let changed = plan.source.replacen("K2 L1 L3 0.8", "K2 L1 L3 0.7", 1);
        let netlist = XyceTestRunner::parse_xyce_netlist(&changed, &plan.deck_path)
            .expect("mutated BUG805SON still parses");
        assert!(XyceTestRunner::validate_bug805_son_netlist(&netlist).is_err());

        let table = XycePrnTable {
            columns: ["Index", "TIME", "I(VS)", "V(2)", "V(3)", "V(4)"]
                .into_iter()
                .map(str::to_string)
                .collect(),
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                vec![1.0, 1.0e-3, -0.16, 0.06, 0.045, 0.048],
                vec![2.0, 25.0e-3, 0.16, -0.06, -0.045, -0.048],
            ],
        };
        XyceTestRunner::validate_bug805_son_table(&table).expect("coupled table passes");
        let mut uncoupled = table.clone();
        for row in &mut uncoupled.rows {
            row[4] = 0.0;
            row[5] = 0.0;
        }
        assert!(XyceTestRunner::validate_bug805_son_table(&uncoupled).is_err());
        let mut unequal = table.clone();
        unequal.rows[1][4] += 1.0e-3;
        assert!(
            !runner
                .compare_serialized_default_prn_tables(&table, &unequal)
                .expect("counterfactual PRN compare")
                .is_empty()
        );
    }

    #[test]
    fn bug805_son_expired_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        assert!(
            runner
                .validate_bug805_son_oracle(Instant::now() - Duration::from_millis(10))
                .is_err()
        );
    }
}
