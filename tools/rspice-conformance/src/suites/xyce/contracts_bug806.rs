use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_806_SON DC DATA/baseline relation";
const OWNER_PATH: &str = "Netlists/Certification_Tests/BUG_806_SON/dc_data.cir";
const BASELINE_PATH: &str = "Netlists/Certification_Tests/BUG_806_SON/dc_baseline.cir";
const OWNER_RECORD: &str = "netlists/certification_tests/bug_806_son/dc_data.cir";
const BASELINE_RECORD: &str = "netlists/certification_tests/bug_806_son/dc_baseline.cir";
const OWNER_CONTRACT: &str = "bug806_dc_data_relational_wrapper_owner";
const BASELINE_CONTRACT: &str = "bug806_dc_data_relational_wrapper_baseline";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug806Role {
    DataOwner,
    Baseline,
}

impl Bug806Role {
    const ALL: [Self; 2] = [Self::DataOwner, Self::Baseline];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_CONTRACT,
            Self::Baseline => BASELINE_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_PATH,
            Self::Baseline => BASELINE_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::DataOwner => OWNER_RECORD,
            Self::Baseline => BASELINE_RECORD,
        }
    }

    fn file_name(self) -> &'static str {
        self.path().rsplit('/').next().expect("BUG806 file name")
    }
}

impl XyceTestRunner {
    fn validate_bug806_plan(role: Bug806Role, plan: &XyceStaticDcPlan) -> Result<(), String> {
        let probes = plan
            .print
            .probes
            .iter()
            .map(|probe| Self::normalize_probe(probe))
            .collect::<Vec<_>>();
        let print_precisions = Self::dc_print_precisions(&plan.source)?;
        if plan.execution_dir.is_some()
            || plan.expression_dialect != ExpressionDialect::Xyce
            || plan.parameter_redefinition_policy != ParameterRedefinitionPolicy::UseLast
            || plan.parameter_redefinition_diagnostic_policy
                != rspice_core::netlist::ParameterRedefinitionDiagnosticPolicy::Silent
            || plan.print_format.is_some()
            || !plan.steps.is_empty()
            || !plan.diagnostics.is_empty()
            || probes != ["{r1:r}", "{r2:r}", "v(4)", "v(5)"]
            || print_precisions.len() != 4
            || print_precisions.values().any(|precision| *precision != 4)
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        match role {
            Bug806Role::DataOwner => {
                let data = plan
                    .dc_data
                    .as_ref()
                    .ok_or_else(|| format!("{LABEL} owner lost its typed DATA sweep"))?;
                if data.rows.len() != 15
                    || !plan.dc.source.eq_ignore_ascii_case("DATA")
                    || plan.dc.start.to_bits() != 0.0f64.to_bits()
                    || plan.dc.stop.to_bits() != 14.0f64.to_bits()
                    || plan.dc.step.to_bits() != 1.0f64.to_bits()
                    || !matches!(plan.dc.mode, DcSweepMode::Linear)
                    || plan.dc.sweep2.is_some()
                {
                    return Err(format!("{LABEL} DATA sweep changed: {plan:?}"));
                }
                for (row_index, row) in data.rows.iter().enumerate() {
                    let expected_r1 = 8.0 + (row_index % 5) as Value;
                    let expected_r2 = 4.0 + (row_index / 5) as Value;
                    if row.overrides.len() != 2
                        || !matches!(&row.overrides[0], XyceDcDataOverride::Device { name, param_name: None, value }
                            if name.eq_ignore_ascii_case("R1") && value.to_bits() == expected_r1.to_bits())
                        || !matches!(&row.overrides[1], XyceDcDataOverride::Device { name, param_name: None, value }
                            if name.eq_ignore_ascii_case("R2") && value.to_bits() == expected_r2.to_bits())
                    {
                        return Err(format!("{LABEL} DATA row {} changed", row_index + 1));
                    }
                }
            }
            Bug806Role::Baseline => {
                let second = plan.dc.sweep2.as_ref();
                if plan.dc_data.is_some()
                    || !plan.dc.source.eq_ignore_ascii_case("R1:R")
                    || plan.dc.start.to_bits() != 8.0f64.to_bits()
                    || plan.dc.stop.to_bits() != 12.0f64.to_bits()
                    || plan.dc.step.to_bits() != 1.0f64.to_bits()
                    || !matches!(plan.dc.mode, DcSweepMode::Linear)
                    || !matches!(second, Some(second)
                        if second.source.eq_ignore_ascii_case("R2:R")
                            && second.start.to_bits() == 4.0f64.to_bits()
                            && second.stop.to_bits() == 6.0f64.to_bits()
                            && second.step.to_bits() == 1.0f64.to_bits()
                            && matches!(second.mode, DcSweepMode::Linear))
                {
                    return Err(format!("{LABEL} baseline sweep changed: {plan:?}"));
                }
            }
        }
        Ok(())
    }

    fn validate_bug806_resistor(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
        value_bits: u64,
    ) -> Result<(), String> {
        if !element.name.eq_ignore_ascii_case(name)
            || element.provenance != ElementProvenance::Authored
            || element.nodes.len() != 2
            || element
                .nodes
                .iter()
                .zip(nodes)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || !matches!(&element.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == value_bits
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} resistor {name} changed: {element:?}"));
        }
        Ok(())
    }

    fn validate_bug806_netlist(role: Bug806Role, netlist: &Netlist) -> Result<(), String> {
        if netlist.elements.len() != 3
            || netlist.output_requests.len() != 1
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
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
        if elements.keys().map(String::as_str).collect::<BTreeSet<_>>()
            != BTreeSet::from(["r1", "r2", "vt1"])
        {
            return Err(format!("{LABEL} element inventory changed"));
        }
        let source = elements["vt1"];
        if source.provenance != ElementProvenance::Authored
            || source.nodes.len() != 2
            || !source.nodes[0].eq_ignore_ascii_case("4")
            || !source.nodes[1].eq_ignore_ascii_case("0")
            || !matches!(source.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if value.to_bits() == 10.0f64.to_bits())
        {
            return Err(format!("{LABEL} voltage source changed: {source:?}"));
        }
        Self::validate_bug806_resistor(elements["r1"], "R1", ["4", "5"], 10.0f64.to_bits())?;
        Self::validate_bug806_resistor(elements["r2"], "R2", ["5", "0"], 5.0f64.to_bits())?;

        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Dc)
            || request.name.is_some()
            || request.print_delimiter != Some(PrintDelimiter::Whitespace)
        {
            return Err(format!("{LABEL} typed .PRINT request changed: {request:?}"));
        }

        match role {
            Bug806Role::DataOwner => {
                if netlist.data_tables.len() != 1
                    || netlist.data_tables[0].name != "test"
                    || netlist.data_tables[0].params != ["r1", "r2"]
                    || netlist.data_tables[0].rows.len() != 15
                    || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                        source,
                        start,
                        stop,
                        step,
                        mode: DcSweepMode::Linear,
                        sweep2: None,
                        ..
                    }] if source.eq_ignore_ascii_case("VT1")
                        && start.to_bits() == 10.0f64.to_bits()
                        && stop.to_bits() == 15.0f64.to_bits()
                        && step.to_bits() == 1.0f64.to_bits())
                {
                    return Err(format!("{LABEL} owner DATA/precedence structure changed"));
                }
                for (row_index, row) in netlist.data_tables[0].rows.iter().enumerate() {
                    let expected = [
                        8.0 + (row_index % 5) as Value,
                        4.0 + (row_index / 5) as Value,
                    ];
                    if row.len() != 2
                        || row
                            .iter()
                            .zip(expected)
                            .any(|(actual, expected)| actual.to_bits() != expected.to_bits())
                    {
                        return Err(format!("{LABEL} typed DATA row {} changed", row_index + 1));
                    }
                }
            }
            Bug806Role::Baseline => {
                if !netlist.data_tables.is_empty()
                    || !matches!(netlist.analyses.as_slice(), [AnalysisCommand::Dc {
                        source,
                        start,
                        stop,
                        step,
                        mode: DcSweepMode::Linear,
                        sweep2: Some(second),
                        ..
                    }] if source.eq_ignore_ascii_case("R1:R")
                        && start.to_bits() == 8.0f64.to_bits()
                        && stop.to_bits() == 12.0f64.to_bits()
                        && step.to_bits() == 1.0f64.to_bits()
                        && second.source.eq_ignore_ascii_case("R2:R")
                        && second.start.to_bits() == 4.0f64.to_bits()
                        && second.stop.to_bits() == 6.0f64.to_bits()
                        && second.step.to_bits() == 1.0f64.to_bits()
                        && matches!(second.mode, DcSweepMode::Linear))
                {
                    return Err(format!("{LABEL} baseline typed sweep changed"));
                }
            }
        }
        Ok(())
    }

    fn bug806_data_results_to_prn_table(
        plan: &XyceStaticDcPlan,
        points: &[XyceDcDataPointResult],
    ) -> Result<XycePrnTable, String> {
        let mut columns = vec!["Index".to_string()];
        columns.extend(plan.print.probes.iter().cloned());
        let mut rows = Vec::with_capacity(points.len());
        for (row_index, point) in points.iter().enumerate() {
            let expected_sweep_value = (row_index + 1) as Value;
            if point.point.sweep_value.to_bits() != expected_sweep_value.to_bits() {
                return Err(format!(
                    "{LABEL} DATA row {} synthetic sweep coordinate changed: {} != {expected_sweep_value}",
                    row_index + 1,
                    point.point.sweep_value
                ));
            }
            let sweep_point = XyceDcSweepPoint {
                primary: point.point.sweep_value,
                secondary: None,
            };
            let mut row = vec![row_index as Value];
            for probe in &plan.print.probes {
                row.push(Self::evaluate_dc_probe(
                    probe,
                    &point.netlist,
                    &plan.dc,
                    sweep_point,
                    &point.point.result,
                    &point.point.device_op_report,
                )?);
            }
            rows.push(row);
        }
        Ok(XycePrnTable { columns, rows })
    }

    fn validate_bug806_table(table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 5
            || table.rows.len() != 15
            || !table.columns[0].eq_ignore_ascii_case("Index")
            || table.columns[1..]
                .iter()
                .map(|column| Self::normalize_probe(column))
                .ne(["{r1:r}", "{r2:r}", "v(4)", "v(5)"])
        {
            return Err(format!("{LABEL} output shape changed: {table:?}"));
        }
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != 5 || row.iter().any(|value| !value.is_finite()) {
                return Err(format!("{LABEL} row {row_index} is malformed"));
            }
            let r1 = 8.0 + (row_index % 5) as Value;
            let r2 = 4.0 + (row_index / 5) as Value;
            let expected = [row_index as Value, r1, r2, 10.0, 10.0 * r2 / (r1 + r2)];
            for (column_index, (actual, expected)) in row.iter().zip(expected).enumerate() {
                let actual = Self::xyce_prn_scientific_roundtrip(*actual, 4)?;
                let expected = Self::xyce_prn_scientific_roundtrip(expected, 4)?;
                if actual != expected {
                    return Err(format!(
                        "{LABEL} analytic row {row_index} column {column_index} changed: {actual} != {expected}"
                    ));
                }
            }
        }
        Ok(())
    }

    fn compare_bug806_precision4_tables(
        expected: &XycePrnTable,
        actual: &XycePrnTable,
    ) -> Result<bool, String> {
        if expected.columns != actual.columns || expected.rows.len() != actual.rows.len() {
            return Ok(false);
        }
        for (row_index, (expected_row, actual_row)) in
            expected.rows.iter().zip(&actual.rows).enumerate()
        {
            if expected_row.len() != expected.columns.len()
                || actual_row.len() != actual.columns.len()
                || expected_row[0].to_bits() != (row_index as Value).to_bits()
                || actual_row[0].to_bits() != (row_index as Value).to_bits()
            {
                return Ok(false);
            }
            for column_index in 1..expected.columns.len() {
                if Self::xyce_prn_scientific_text(expected_row[column_index], 4)?
                    != Self::xyce_prn_scientific_text(actual_row[column_index], 4)?
                {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    fn bug806_plan(&self, role: Bug806Role) -> Result<XyceStaticDcPlan, String> {
        let plan =
            self.static_dc_plan_for_path(&self.root.join(role.path()), ExpressionDialect::Xyce)?;
        Self::validate_bug806_plan(role, &plan)?;
        Ok(plan)
    }

    pub(super) fn validate_bug806_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!(
                "{LABEL} exceeded its shared deadline before execution"
            ));
        }
        let baseline_plan = self.bug806_plan(Bug806Role::Baseline)?;
        let (baseline_netlist, baseline_results) = self
            .run_static_dc_results(&baseline_plan, start)
            .map_err(|error| format!("{LABEL} baseline execution failed: {error}"))?;
        Self::validate_bug806_netlist(Bug806Role::Baseline, &baseline_netlist)?;
        let baseline_table =
            self.dc_results_to_prn_table(&baseline_plan, &baseline_netlist, &baseline_results)?;
        Self::validate_bug806_table(&baseline_table)?;

        if abort.is_aborted() {
            return Err(format!("{LABEL} exceeded its shared deadline between runs"));
        }
        let data_plan = self.bug806_plan(Bug806Role::DataOwner)?;
        let data_netlist = Self::parse_xyce_netlist(&data_plan.source, &data_plan.deck_path)
            .map_err(|error| format!("{LABEL} DATA parse failed: {error}"))?;
        Self::validate_bug806_netlist(Bug806Role::DataOwner, &data_netlist)?;
        let data_sweep = data_plan
            .dc_data
            .as_ref()
            .ok_or_else(|| format!("{LABEL} DATA plan lost its sweep"))?;
        let data_results = self
            .run_static_dc_data_results(&data_netlist, data_sweep, start)
            .map_err(|error| format!("{LABEL} DATA execution failed: {error}"))?;
        let data_table = Self::bug806_data_results_to_prn_table(&data_plan, &data_results)?;
        Self::validate_bug806_table(&data_table)?;

        if !Self::compare_bug806_precision4_tables(&baseline_table, &data_table)? {
            let mut serialized_baseline = baseline_table.clone();
            for row in &mut serialized_baseline.rows {
                for value in &mut row[1..] {
                    *value = Self::xyce_prn_scientific_roundtrip(*value, 4)?;
                }
            }
            let fallback = self.compare_dc_data_prn_reference(
                &serialized_baseline,
                &data_plan.print,
                &data_plan.source,
                &data_plan.dc,
                &data_results,
            )?;
            if !fallback.is_empty() {
                return Err(format!(
                    "{LABEL} failed both exact Release diff and xyce_verify fallback: {fallback:?}"
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
    fn bug806_typed_plans_and_native_oracle_pass_for_both_roles() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        runner
            .validate_bug806_oracle(Instant::now())
            .expect("BUG806 roles execute their exact native relation");
    }

    #[test]
    fn bug806_analytic_oracle_rejects_a_shared_wrong_divider() {
        let mut table = XycePrnTable {
            columns: vec![
                "Index".to_string(),
                "{R1:R}".to_string(),
                "{R2:R}".to_string(),
                "V(4)".to_string(),
                "V(5)".to_string(),
            ],
            rows: (0..15)
                .map(|row_index| {
                    let r1 = 8.0 + (row_index % 5) as Value;
                    let r2 = 4.0 + (row_index / 5) as Value;
                    vec![row_index as Value, r1, r2, 10.0, 10.0 * r2 / (r1 + r2)]
                })
                .collect(),
        };
        XyceTestRunner::validate_bug806_table(&table).expect("analytic fixture passes");
        table.rows[7][4] += 0.1;
        assert!(XyceTestRunner::validate_bug806_table(&table).is_err());
    }

    #[test]
    fn bug806_typed_mutations_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        let owner_path = root.join(OWNER_PATH);
        let source = fs::read_to_string(&owner_path).expect("read BUG806 owner");

        let changed_row = source.replacen("1.0000e+01  5.0000e+00", "1.0000e+01  5.5000e+00", 1);
        let plan = runner
            .static_dc_plan_for_source_with_execution_dir(
                &owner_path,
                changed_row,
                ExpressionDialect::Xyce,
                None,
            )
            .expect("mutated DATA deck still plans");
        assert!(XyceTestRunner::validate_bug806_plan(Bug806Role::DataOwner, &plan).is_err());

        let changed_precedence = source.replacen(".DC VT1 10 15 1", ".DC VT1 10 14 1", 1);
        let netlist = XyceTestRunner::parse_xyce_netlist(&changed_precedence, &owner_path)
            .expect("mutated precedence deck still parses");
        assert!(XyceTestRunner::validate_bug806_netlist(Bug806Role::DataOwner, &netlist).is_err());
    }

    #[test]
    fn bug806_expired_shared_deadline_rejects_before_execution() {
        let root = corpus_root();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(&root, config);
        let error = runner
            .validate_bug806_oracle(Instant::now() - Duration::from_secs(1))
            .expect_err("expired BUG806 deadline must reject");
        assert!(error.contains("deadline"));
    }
}
