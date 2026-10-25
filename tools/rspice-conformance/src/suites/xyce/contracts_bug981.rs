use super::*;

const LABEL: &str = "BUG_981_SON OUTPUTTIMEPOINTS/BREAKPOINTS relation";
const REQUESTED_TIMES: [Value; 4] = [1.0e-3, 2.0e-3, 3.0e-3, 4.0e-3];
const EXPECTED_VALUES: [&str; 5] = [
    "5.16060279e0",
    "6.32332358e0",
    "6.75106466e0",
    "6.90842181e0",
    "6.96631027e0",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug981Role {
    WrapperOwner,
    OutputTimePoints,
    Breakpoints,
}

impl Bug981Role {
    const ALL: [Self; 3] = [
        Self::WrapperOwner,
        Self::OutputTimePoints,
        Self::Breakpoints,
    ];
    const WORKERS: [Self; 2] = [Self::OutputTimePoints, Self::Breakpoints];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG981_OWNER_CONTRACT,
            Self::OutputTimePoints => XYCE_BUG981_OUTPUT_CONTRACT,
            Self::Breakpoints => XYCE_BUG981_BREAKPOINT_CONTRACT,
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG981_OWNER_PATH,
            Self::OutputTimePoints => XYCE_BUG981_OUTPUT_PATH,
            Self::Breakpoints => XYCE_BUG981_BREAKPOINT_PATH,
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => XYCE_BUG981_OWNER_RECORD,
            Self::OutputTimePoints => XYCE_BUG981_OUTPUT_RECORD,
            Self::Breakpoints => XYCE_BUG981_BREAKPOINT_RECORD,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::WrapperOwner => "wrapper owner",
            Self::OutputTimePoints => "OUTPUTTIMEPOINTS control",
            Self::Breakpoints => "BREAKPOINTS control",
        }
    }
}

impl XyceTestRunner {
    fn validate_bug981_worker(
        &self,
        role: Bug981Role,
        source: &str,
        path: &Path,
    ) -> Result<XyceStaticTranPlan, String> {
        if !Bug981Role::WORKERS.contains(&role) {
            return Err(format!("{LABEL} owner is not executable"));
        }
        let plan = self.static_tran_plan_for_path_with_purpose(
            path,
            XyceStaticTranPlanPurpose::RelationalFamily,
        )?;
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.contract != XyceStaticTranContract::PlainStatic
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.print.as_ref().is_none_or(|print| {
                print.probes.len() != 1 || !print.probes[0].eq_ignore_ascii_case("V(1)")
            })
            || plan.tran.step.to_bits() != 1.0e-3f64.to_bits()
            || plan.tran.stop.to_bits() != 5.0e-3f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!("{LABEL} {} plan changed: {plan:?}", role.label()));
        }
        let print_outputs = Self::print_output_requests(source, "TRAN")?;
        if !matches!(print_outputs.as_slice(), [request]
            if request.format.as_deref().is_some_and(|format| format.eq_ignore_ascii_case("NOINDEX"))
                && request.file.is_none()
                && matches!(request.probes.as_slice(), [probe] if probe.eq_ignore_ascii_case("V(1)")))
        {
            return Err(format!(
                "{LABEL} {} authored NOINDEX output changed",
                role.label()
            ));
        }

        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {} no longer parses: {error}", role.label()))?;
        if netlist.title != "testing breakpoints "
            || netlist.elements.len() != 4
            || netlist.analyses.len() != 1
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
            || !netlist.params.all_global_expressions().is_empty()
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed: title={:?}, elements={:?}, analyses={:?}, outputs={:?}, diagnostics={:?}, options={:?}",
                role.label(),
                netlist.title,
                netlist.elements,
                netlist.analyses,
                netlist.output_requests,
                netlist.diagnostics,
                netlist.options
            ));
        }

        let mut remaining_options = netlist.options.clone();
        let output_time_points = std::mem::take(&mut remaining_options.output_time_points);
        let timeint_breakpoints = std::mem::take(&mut remaining_options.timeint_breakpoints);
        let schedule_matches = match role {
            Bug981Role::OutputTimePoints => {
                output_time_points == REQUESTED_TIMES && timeint_breakpoints.is_empty()
            }
            Bug981Role::Breakpoints => {
                timeint_breakpoints == REQUESTED_TIMES && output_time_points.is_empty()
            }
            Bug981Role::WrapperOwner => false,
        };
        if !schedule_matches
            || !Self::analytic_timeint_only_options_match(
                &remaining_options,
                None,
                None,
                None,
                None,
            )
        {
            return Err(format!("{LABEL} {} option state changed", role.label()));
        }

        let [offset, exponential, resistor, capacitor] = netlist.elements.as_slice() else {
            unreachable!("BUG981 element count was checked")
        };
        if offset.provenance != ElementProvenance::Authored
            || !offset.name.eq_ignore_ascii_case("VOFF")
            || !Self::nodes_match(&offset.nodes, &["A", "0"])
            || !matches!(&offset.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Dc(value)
            ) if value.to_bits() == 2.0f64.to_bits())
            || exponential.provenance != ElementProvenance::Authored
            || !exponential.name.eq_ignore_ascii_case("VEXP")
            || !Self::nodes_match(&exponential.nodes, &["1", "A"])
            || !matches!(&exponential.kind, ElementKind::VoltageSource(
                rspice_core::netlist::SourceSpec::Exp { v1, v2, td1, tau1, td2, tau2 }
            ) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 5.0f64.to_bits()
                && td1.to_bits() == 0.0f64.to_bits()
                && tau1.to_bits() == 1.0e-3f64.to_bits()
                && td2.to_bits() == 1.0f64.to_bits()
                && tau2.is_nan())
            || resistor.provenance != ElementProvenance::Authored
            || !resistor.name.eq_ignore_ascii_case("R1")
            || !Self::nodes_match(&resistor.nodes, &["1", "2"])
            || !matches!(&resistor.kind, ElementKind::Resistor {
                value,
                value_expr: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
            || capacitor.provenance != ElementProvenance::Authored
            || !capacitor.name.eq_ignore_ascii_case("C1")
            || !Self::nodes_match(&capacitor.nodes, &["2", "0"])
            || !matches!(&capacitor.kind, ElementKind::Capacitor {
                value,
                value_expr: None,
                initial_voltage: None,
                model: None,
                instance_params,
                deferred_params,
            } if value.to_bits() == 1.0f64.to_bits()
                && instance_params.is_empty()
                && deferred_params.is_empty())
        {
            return Err(format!("{LABEL} {} topology changed", role.label()));
        }

        if !matches!(&netlist.analyses[0], AnalysisCommand::Tran {
            step,
            stop,
            start: None,
            max_step: None,
            uic: false,
        } if step.to_bits() == 1.0e-3f64.to_bits()
            && stop.to_bits() == 5.0e-3f64.to_bits())
        {
            return Err(format!("{LABEL} {} typed TRAN changed", role.label()));
        }
        let request = &netlist.output_requests[0];
        let expected_line = if role == Bug981Role::OutputTimePoints {
            9
        } else {
            10
        };
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.name.is_some()
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || !request.expressions.is_empty()
            || request.origin.line != expected_line
            || request
                .origin
                .path
                .as_deref()
                .is_none_or(|origin_path| !Self::same_path(origin_path, path))
            || request.dependencies.len() != 1
            || request.dependencies[0].kind != OutputSymbolKind::Node
            || request.dependencies[0].expression
            || !request.dependencies[0].operator.eq_ignore_ascii_case("V")
            || !request.dependencies[0].symbol.eq_ignore_ascii_case("1")
        {
            return Err(format!(
                "{LABEL} {} PRINT request changed: {request:?}",
                role.label()
            ));
        }
        Ok(plan)
    }

    fn validate_bug981_table(role: Bug981Role, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != 3
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || !table.columns[2].eq_ignore_ascii_case("V(1)")
            || table.rows.is_empty()
            || table.rows.iter().enumerate().any(|(index, row)| {
                row.len() != 3
                    || row[0].to_bits() != (index as Value).to_bits()
                    || row.iter().any(|value| !value.is_finite())
            })
            || table.rows.windows(2).any(|rows| rows[0][1] >= rows[1][1])
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != 5.0e-3f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} produced malformed output",
                role.label()
            ));
        }
        match role {
            Bug981Role::OutputTimePoints
                if table.rows.len() != 5 || table.rows[0][1].to_bits() != 1.0e-3f64.to_bits() =>
            {
                return Err(format!("{LABEL} OUTPUTTIMEPOINTS projection changed"));
            }
            Bug981Role::Breakpoints
                if table.rows.len() <= 6 || table.rows[0][1].to_bits() != 0.0f64.to_bits() =>
            {
                return Err(format!("{LABEL} BREAKPOINTS must preserve adaptive output"));
            }
            _ => {}
        }
        for (expected_time, expected_value) in REQUESTED_TIMES
            .into_iter()
            .chain([5.0e-3])
            .zip(EXPECTED_VALUES)
        {
            let row = table
                .rows
                .iter()
                .find(|row| row[1].to_bits() == expected_time.to_bits())
                .ok_or_else(|| format!("{LABEL} {} missed {expected_time}", role.label()))?;
            let actual = Self::xyce_default_prn_text(row[2])?;
            if actual != expected_value {
                return Err(format!(
                    "{LABEL} {} has V(1)={actual} at {expected_time}, expected {expected_value}",
                    role.label()
                ));
            }
        }
        Ok(())
    }

    fn bug981_noindex_table(table: &XycePrnTable) -> Result<XycePrnTable, String> {
        if table.columns.first().is_none_or(|column| column != "Index") {
            return Err(format!("{LABEL} indexed observation lost Index"));
        }
        Ok(XycePrnTable {
            columns: table.columns[1..].to_vec(),
            rows: table
                .rows
                .iter()
                .map(|row| {
                    row.get(1..)
                        .map(<[Value]>::to_vec)
                        .ok_or_else(|| format!("{LABEL} row lost its no-index projection"))
                })
                .collect::<Result<_, _>>()?,
        })
    }

    fn validate_bug981_relation(
        output: &XycePrnTable,
        breakpoints: &XycePrnTable,
    ) -> Result<(), String> {
        let output = Self::bug981_noindex_table(output)?;
        let targets = REQUESTED_TIMES
            .into_iter()
            .chain([5.0e-3])
            .map(Self::xyce_default_prn_text)
            .collect::<Result<BTreeSet<_>, _>>()?;
        let filtered = XycePrnTable {
            columns: breakpoints.columns[1..].to_vec(),
            rows: breakpoints
                .rows
                .iter()
                .filter(|row| {
                    row.get(1)
                        .and_then(|time| Self::xyce_default_prn_text(*time).ok())
                        .is_some_and(|time| targets.contains(&time))
                })
                .map(|row| row[1..].to_vec())
                .collect(),
        };
        if filtered.rows.len() != 5 {
            return Err(format!(
                "{LABEL} patfile selection found {} data rows",
                filtered.rows.len()
            ));
        }
        let output_text =
            Self::xyce_legacy_compact_prn_for_comparison(&output, &PrintDelimiter::Whitespace)?;
        let filtered_text =
            Self::xyce_legacy_compact_prn_for_comparison(&filtered, &PrintDelimiter::Whitespace)?;
        if output_text != filtered_text {
            return Err(format!("{LABEL} Release grep/diff relation changed"));
        }
        Ok(())
    }

    fn run_bug981_worker(
        &self,
        role: Bug981Role,
        plan: &XyceStaticTranPlan,
        start: Instant,
    ) -> Result<XycePrnTable, String> {
        let (netlist, result) = self
            .run_transient_family_plan(plan, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} {} exceeded deadline", role.label()),
                other => format!("{LABEL} {} failed: {other}", role.label()),
            })?;
        let table = Self::transient_family_result_to_prn_table(plan, &netlist, &result)
            .map_err(|error| format!("{LABEL} {} PRN failed: {error}", role.label()))?;
        Self::validate_bug981_table(role, &table)?;
        Ok(table)
    }

    pub(super) fn validate_bug981_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before validation"));
        }
        let mut plans = BTreeMap::new();
        for worker in Bug981Role::WORKERS {
            let path = self.root.join(worker.path());
            let source = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            plans.insert(worker, self.validate_bug981_worker(worker, &source, &path)?);
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} validation exceeded deadline"));
        }
        let output = self.run_bug981_worker(
            Bug981Role::OutputTimePoints,
            plans
                .get(&Bug981Role::OutputTimePoints)
                .expect("output plan"),
            start,
        )?;
        let breakpoints = self.run_bug981_worker(
            Bug981Role::Breakpoints,
            plans
                .get(&Bug981Role::Breakpoints)
                .expect("breakpoint plan"),
            start,
        )?;
        Self::validate_bug981_relation(&output, &breakpoints)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
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
    fn bug981_workers_have_exact_typed_contracts() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in Bug981Role::WORKERS {
            let path = runner.root.join(role.path());
            let source = fs::read_to_string(&path).expect("read BUG981 worker");
            runner
                .validate_bug981_worker(role, &source, &path)
                .expect("validate BUG981 worker");
        }
    }

    #[test]
    fn bug981_oracle_executes_adaptive_schedule_relation() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        runner
            .validate_bug981_oracle(Instant::now())
            .expect("execute BUG981 relation");
    }

    #[test]
    fn bug981_typed_contract_rejects_schedule_and_stale_source_mutations() {
        let temporary = tempfile::tempdir().expect("create BUG981 worker fixture");
        let path = temporary.path().join(XYCE_BUG981_OUTPUT_PATH);
        fs::create_dir_all(path.parent().expect("worker parent")).expect("create worker family");
        let canonical = fs::read_to_string(corpus_root().join(XYCE_BUG981_OUTPUT_PATH))
            .expect("read BUG981 control");
        let mutated = canonical.replace("4ms", "4.5ms");
        fs::write(&path, &mutated).expect("write schedule mutation");
        let runner = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
        assert!(
            runner
                .validate_bug981_worker(Bug981Role::OutputTimePoints, &mutated, &path)
                .is_err()
        );
        fs::write(&path, &canonical).expect("restore worker");
        runner
            .validate_bug981_worker(Bug981Role::OutputTimePoints, &canonical, &path)
            .expect("canonical BUG981 worker passes outside the corpus");
        assert!(
            runner
                .validate_bug981_worker(Bug981Role::OutputTimePoints, &mutated, &path)
                .is_err(),
            "planner source must match the caller's provenance-bound bytes"
        );
    }

    #[test]
    fn bug981_relation_rejects_selected_row_value_and_census_mutations() {
        let output = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(1)".into()],
            rows: REQUESTED_TIMES
                .into_iter()
                .chain([5.0e-3])
                .zip([1.0, 2.0, 3.0, 4.0, 5.0])
                .enumerate()
                .map(|(index, (time, value))| vec![index as Value, time, value])
                .collect(),
        };
        let mut breakpoints = XycePrnTable {
            columns: output.columns.clone(),
            rows: std::iter::once(vec![0.0, 0.0, 0.0])
                .chain(
                    output
                        .rows
                        .iter()
                        .enumerate()
                        .map(|(index, row)| vec![(index + 1) as Value, row[1], row[2]]),
                )
                .collect(),
        };
        XyceTestRunner::validate_bug981_relation(&output, &breakpoints)
            .expect("canonical structural grep/diff relation");

        breakpoints.rows[2][2] += 1.0e-4;
        assert!(XyceTestRunner::validate_bug981_relation(&output, &breakpoints).is_err());
        breakpoints.rows[2][2] -= 1.0e-4;
        breakpoints.rows.remove(3);
        assert!(XyceTestRunner::validate_bug981_relation(&output, &breakpoints).is_err());
    }

    #[test]
    fn bug981_oracle_rejects_expired_shared_deadline() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let expired = Instant::now()
            - Duration::from_millis(
                u64::try_from(runner.config.max_time_per_test_ms.max(1) + 1)
                    .expect("timeout fits u64"),
            );
        assert!(runner.validate_bug981_oracle(expired).is_err());
    }
}
