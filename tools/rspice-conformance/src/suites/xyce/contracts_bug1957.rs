use super::*;
use rspice_core::netlist::{OutputSymbolKind, SaveSignal, SourceSpec};

const LABEL: &str = "BUG_1957 multi-winding mutual-inductor relation";
const FAMILY_PATH: &str = "Netlists/Certification_Tests/BUG_1957";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug1957Family {
    Even,
    Odd,
}

impl Bug1957Family {
    fn stem(self) -> &'static str {
        match self {
            Self::Even => "mutindlin_even",
            Self::Odd => "mutindlin_odd",
        }
    }

    fn winding_count(self) -> usize {
        match self {
            Self::Even => 4,
            Self::Odd => 5,
        }
    }

    fn gold_file(self) -> String {
        format!("{}_s3f5.cir.prn.gs", self.stem())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bug1957Representation {
    Anchor,
    Multiwinding,
    Pairwise,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct Bug1957Role {
    family: Bug1957Family,
    representation: Bug1957Representation,
}

impl Bug1957Role {
    const ALL: [Self; 6] = [
        Self::new(Bug1957Family::Even, Bug1957Representation::Anchor),
        Self::new(Bug1957Family::Even, Bug1957Representation::Multiwinding),
        Self::new(Bug1957Family::Even, Bug1957Representation::Pairwise),
        Self::new(Bug1957Family::Odd, Bug1957Representation::Anchor),
        Self::new(Bug1957Family::Odd, Bug1957Representation::Multiwinding),
        Self::new(Bug1957Family::Odd, Bug1957Representation::Pairwise),
    ];

    const fn new(family: Bug1957Family, representation: Bug1957Representation) -> Self {
        Self {
            family,
            representation,
        }
    }

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    fn file_name(self) -> String {
        let suffix = match self.representation {
            Bug1957Representation::Anchor => ".cir",
            Bug1957Representation::Multiwinding => "_ps.cir",
            Bug1957Representation::Pairwise => "_s3f5.cir",
        };
        format!("{}{suffix}", self.family.stem())
    }

    fn path(self) -> String {
        format!("{FAMILY_PATH}/{}", self.file_name())
    }

    fn record(self) -> String {
        Self::normalize_path(&self.path())
    }

    fn normalize_path(path: &str) -> String {
        path.replace('\\', "/").to_ascii_lowercase()
    }

    pub(super) fn contract(self) -> &'static str {
        match (self.family, self.representation) {
            (Bug1957Family::Even, Bug1957Representation::Anchor) => {
                "bug1957_even_mutual_inductor_wrapper_owner"
            }
            (Bug1957Family::Even, Bug1957Representation::Multiwinding) => {
                "bug1957_even_multiwinding_worker"
            }
            (Bug1957Family::Even, Bug1957Representation::Pairwise) => {
                "bug1957_even_pairwise_worker"
            }
            (Bug1957Family::Odd, Bug1957Representation::Anchor) => {
                "bug1957_odd_mutual_inductor_wrapper_owner"
            }
            (Bug1957Family::Odd, Bug1957Representation::Multiwinding) => {
                "bug1957_odd_multiwinding_worker"
            }
            (Bug1957Family::Odd, Bug1957Representation::Pairwise) => "bug1957_odd_pairwise_worker",
        }
    }

    fn worker(family: Bug1957Family, representation: Bug1957Representation) -> Self {
        debug_assert!(representation != Bug1957Representation::Anchor);
        Self::new(family, representation)
    }
}

impl XyceTestRunner {
    fn bug1957_expected_probes(family: Bug1957Family) -> Vec<String> {
        let mut probes = vec!["i(vs)".to_string()];
        probes.extend((2..=family.winding_count() + 1).map(|node| format!("v({node})")));
        probes
    }

    fn validate_bug1957_plan(role: Bug1957Role, plan: &XyceStaticTranPlan) -> Result<(), String> {
        let print = plan
            .print
            .as_ref()
            .ok_or_else(|| format!("{LABEL} {} lost its PRINT", role.file_name()))?;
        let probes = print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        if role.representation == Bug1957Representation::Anchor
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || !plan.steps.is_empty()
            || plan.wrapper_tolerance.is_some()
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || probes != Self::bug1957_expected_probes(role.family)
            || plan.tran.step.to_bits() != 100.0e-6f64.to_bits()
            || plan.tran.stop.to_bits() != 25.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn validate_bug1957_passive(
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
            || !Self::nodes_match(&element.nodes, &nodes)
            || !kind_matches
        {
            return Err(format!("{LABEL} passive {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn bug1957_pair_set(netlist: &Netlist) -> Result<BTreeSet<(String, String)>, String> {
        let mut pairs = BTreeSet::new();
        for element in &netlist.elements {
            let ElementKind::Coupling {
                inductors,
                coefficient,
                model,
                ..
            } = &element.kind
            else {
                continue;
            };
            if element.provenance != ElementProvenance::Authored
                || !element.nodes.is_empty()
                || coefficient.to_bits() != 0.75f64.to_bits()
                || model.is_some()
                || inductors.len() < 2
            {
                return Err(format!("{LABEL} coupling {} changed", element.name));
            }
            for left in 0..inductors.len() {
                for right in left + 1..inductors.len() {
                    let mut pair = [
                        inductors[left].to_ascii_lowercase(),
                        inductors[right].to_ascii_lowercase(),
                    ];
                    pair.sort();
                    if !pairs.insert((pair[0].clone(), pair[1].clone())) {
                        return Err(format!("{LABEL} duplicate effective coupling {pair:?}"));
                    }
                }
            }
        }
        Ok(pairs)
    }

    fn validate_bug1957_netlist(role: Bug1957Role, netlist: &Netlist) -> Result<(), String> {
        let winding_count = role.family.winding_count();
        let pair_count = winding_count * (winding_count - 1) / 2;
        let coupling_count = if role.representation == Bug1957Representation::Multiwinding {
            1
        } else {
            pair_count
        };
        if role.representation == Bug1957Representation::Anchor
            || netlist.elements.len() != 1 + 2 * winding_count + coupling_count
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
        let source = elements
            .get("vs")
            .ok_or_else(|| format!("{LABEL} lost VS"))?;
        if source.provenance != ElementProvenance::Authored
            || !Self::nodes_match(&source.nodes, &["1", "0"])
            || !matches!(&source.kind, ElementKind::VoltageSource(SourceSpec::Sin {
                offset, amplitude, frequency, delay, damping, phase,
            }) if offset.to_bits() == 0.0f64.to_bits()
                && amplitude.to_bits() == 169.7f64.to_bits()
                && frequency.to_bits() == 60.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && damping.to_bits() == 0.0f64.to_bits()
                && phase.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} sinusoidal source changed: {source:?}"));
        }
        for index in 0..winding_count {
            let resistor = format!("r{}", index + 1);
            let inductor = if index == 0 {
                "lp".to_string()
            } else {
                format!("l{index}")
            };
            let node = (index + 2).to_string();
            let resistor_nodes = if index == 0 {
                ["1", node.as_str()]
            } else {
                [node.as_str(), "0"]
            };
            Self::validate_bug1957_passive(
                elements
                    .get(&resistor)
                    .ok_or_else(|| format!("{LABEL} lost {resistor}"))?,
                &resistor,
                resistor_nodes,
                1.0e3,
                false,
            )?;
            Self::validate_bug1957_passive(
                elements
                    .get(&inductor)
                    .ok_or_else(|| format!("{LABEL} lost {inductor}"))?,
                &inductor,
                [node.as_str(), "0"],
                1.0e-3,
                true,
            )?;
        }

        let couplings = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Coupling { .. }))
            .collect::<Vec<_>>();
        if couplings.len() != coupling_count {
            return Err(format!("{LABEL} coupling count changed"));
        }
        if role.representation == Bug1957Representation::Multiwinding {
            let expected = (0..winding_count)
                .map(|index| {
                    if index == 0 {
                        "lp".to_string()
                    } else {
                        format!("l{index}")
                    }
                })
                .collect::<Vec<_>>();
            if !couplings[0].name.eq_ignore_ascii_case("K1")
                || !matches!(&couplings[0].kind, ElementKind::Coupling { inductors, .. }
                    if inductors.iter().map(|name| name.to_ascii_lowercase()).collect::<Vec<_>>() == expected)
            {
                return Err(format!("{LABEL} multi-winding syntax changed"));
            }
        }
        let pairs = Self::bug1957_pair_set(netlist)?;
        if pairs.len() != pair_count {
            return Err(format!(
                "{LABEL} effective coupling topology changed: {pairs:?}"
            ));
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step, stop, start: None, max_step: None, uic: false,
        } if step.to_bits() == 100.0e-6f64.to_bits()
            && stop.to_bits() == 25.0e-3f64.to_bits())
        {
            return Err(format!("{LABEL} typed transient changed"));
        }
        let request = &netlist.output_requests[0];
        let expected_probes = Self::bug1957_expected_probes(role.family);
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.dependencies.len() != expected_probes.len()
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

    fn validate_bug1957_table(family: Bug1957Family, table: &XycePrnTable) -> Result<(), String> {
        let mut expected = vec!["Index".to_string(), "TIME".to_string()];
        expected.extend(Self::bug1957_expected_probes(family));
        if table.columns.len() != expected.len()
            || table
                .columns
                .iter()
                .zip(&expected)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || table.rows.len() < 3
        {
            return Err(format!("{LABEL} output shape changed: {table:?}"));
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
        let last = table.rows.last().expect("BUG1957 table is nonempty");
        if first[1].to_bits() != 0.0f64.to_bits()
            || first[2..]
                .iter()
                .any(|value| value.to_bits() != 0.0f64.to_bits())
            || (last[1] - 25.0e-3).abs() > 1.0e-14
            || table
                .rows
                .iter()
                .all(|row| row[2..].iter().all(|value| value.abs() < 1.0e-8))
        {
            return Err(format!("{LABEL} output domain became vacuous"));
        }
        Ok(())
    }

    fn bug1957_plan(&self, role: Bug1957Role) -> Result<XyceStaticTranPlan, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(
            &self.root.join(role.path()),
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        Self::validate_bug1957_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug1957_oracle(
        &self,
        role: Bug1957Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let run = |representation: Bug1957Representation| {
            let member_role = Bug1957Role::worker(role.family, representation);
            let plan = self.bug1957_plan(member_role)?;
            let (netlist, result) = self
                .run_transient_family_plan(&plan, start, None, None)
                .map_err(|error| match error {
                    SimulationError::Aborted => format!("{LABEL} execution exceeded deadline"),
                    other => format!("{LABEL} execution failed: {other}"),
                })?;
            Self::validate_bug1957_netlist(member_role, &netlist)?;
            let table = Self::transient_family_result_to_prn_table(&plan, &netlist, &result)?;
            Self::validate_bug1957_table(role.family, &table)?;
            Ok::<_, String>((plan, netlist, table))
        };

        // Preserve the Release wrapper's independent-run order.
        let (pairwise_plan, pairwise_netlist, pairwise_table) =
            run(Bug1957Representation::Pairwise)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired between independent runs"));
        }
        let (multi_plan, multi_netlist, multi_table) = run(Bug1957Representation::Multiwinding)?;
        if Self::bug1957_pair_set(&pairwise_netlist)? != Self::bug1957_pair_set(&multi_netlist)? {
            return Err(format!(
                "{LABEL} representations no longer expand to the same topology"
            ));
        }
        let relational =
            self.compare_serialized_default_prn_tables(&pairwise_table, &multi_table)?;
        if !relational.is_empty() {
            return Err(format!("{LABEL} syntax relation failed: {relational:?}"));
        }

        let gold_name = role.family.gold_file();
        let gold_source = fs::read_to_string(self.root.join(FAMILY_PATH).join(&gold_name))
            .map_err(|error| format!("{LABEL} cannot read {gold_name}: {error}"))?;
        let gold = Self::parse_prn_table(&gold_source)?;
        Self::validate_bug1957_table(role.family, &gold)?;
        for (plan, table, representation) in [
            (&pairwise_plan, &pairwise_table, "pairwise"),
            (&multi_plan, &multi_table, "multi-winding"),
        ] {
            let print = plan.print.as_ref().expect("BUG1957 plan has PRINT");
            let tolerances = Self::xyce_verify_comp_tolerances(&plan.source, &print.probes)?;
            let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
                &gold,
                table,
                &tolerances,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} {representation} absolute Release oracle failed: {mismatches:?}"
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
    fn bug1957_all_roles_execute_the_absolute_and_relational_oracles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for role in Bug1957Role::ALL {
            runner
                .validate_bug1957_oracle(role, Instant::now())
                .unwrap_or_else(|error| panic!("{}: {error}", role.file_name()));
        }
    }

    #[test]
    fn bug1957_typed_and_numeric_counterfactuals_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let role = Bug1957Role::worker(Bug1957Family::Even, Bug1957Representation::Pairwise);
        let mut plan = runner.bug1957_plan(role).expect("canonical BUG1957 plan");
        let netlist = XyceTestRunner::parse_xyce_netlist(&plan.source, &plan.deck_path)
            .expect("canonical BUG1957 netlist");
        XyceTestRunner::validate_bug1957_netlist(role, &netlist)
            .expect("canonical BUG1957 topology");
        let changed_coupling = plan.source.replacen(".75", ".70", 1);
        let changed_netlist =
            XyceTestRunner::parse_xyce_netlist(&changed_coupling, &plan.deck_path)
                .expect("mutated BUG1957 coupling still parses");
        assert!(XyceTestRunner::validate_bug1957_netlist(role, &changed_netlist).is_err());
        plan.tran.stop = 24.0e-3;
        assert!(XyceTestRunner::validate_bug1957_plan(role, &plan).is_err());

        let gold =
            XyceTestRunner::parse_prn_file(&root.join(FAMILY_PATH).join(role.family.gold_file()))
                .expect("parse BUG1957 gold");
        let mut wrong = gold.clone();
        wrong.rows[5][3] += 1.0;
        let tolerances = XyceTestRunner::xyce_verify_comp_tolerances(
            &runner.bug1957_plan(role).expect("BUG1957 plan").source,
            &runner
                .bug1957_plan(role)
                .expect("BUG1957 plan")
                .print
                .expect("BUG1957 print")
                .probes,
        )
        .expect("BUG1957 tolerances");
        assert!(
            !runner
                .compare_xyce_verify_transient_tables_with_probe_tolerances(
                    &gold,
                    &wrong,
                    &tolerances,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )
                .expect("counterfactual comparison")
                .is_empty()
        );
    }

    #[test]
    fn bug1957_expired_deadline_rejects_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..XyceRunnerConfig::default()
            },
        );
        let role = Bug1957Role::new(Bug1957Family::Even, Bug1957Representation::Anchor);
        assert!(
            runner
                .validate_bug1957_oracle(role, Instant::now() - Duration::from_millis(10))
                .is_err()
        );
    }
}
