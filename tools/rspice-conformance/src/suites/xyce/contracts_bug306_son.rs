use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_306_SON numeric/string TIMEINT METHOD equivalence";

const GEAR_OWNER_CONTRACT: &str = "bug306_gear_numeric_method_wrapper_owner";
const GEAR_CONTROL_CONTRACT: &str = "bug306_gear_string_method_release_control";
const TRAP_OWNER_CONTRACT: &str = "bug306_trap_numeric_method_wrapper_owner";
const TRAP_CONTROL_CONTRACT: &str = "bug306_trap_string_method_release_control";
const TRAPEZOIDAL_CONTROL_CONTRACT: &str = "bug306_trapezoidal_string_method_release_control";

const RESULT_COLUMNS: usize = 5;
const PRINT_PRECISION: usize = 10;
const PRINT_WIDTH: usize = 19;
const MAX_ADAPTIVE_RESULT_POINTS: usize = 4_096;
const MAX_KCL_RESIDUAL: Value = 1.0e-6;
// `20us` reads as the double nearest the decimal it writes, 2e-5 (not the
// double-rounded product `20.0 * 1.0e-6`, which is one ulp below it).
const TRANSIENT_STOP: Value = 20.0e-6;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug306SonRole {
    GearNumericOwner,
    GearStringControl,
    TrapNumericOwner,
    TrapStringControl,
    TrapezoidalStringControl,
}

impl Bug306SonRole {
    const ALL: [Self; 5] = [
        Self::GearNumericOwner,
        Self::GearStringControl,
        Self::TrapNumericOwner,
        Self::TrapStringControl,
        Self::TrapezoidalStringControl,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let record = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL.into_iter().find(|role| role.record() == record)
    }

    fn file_name(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "lead_bjt_gear.cir",
            Self::GearStringControl => "lead_bjt_gear_strings.cir",
            Self::TrapNumericOwner => "lead_bjt_trap.cir",
            Self::TrapStringControl => "lead_bjt_trap_strings.cir",
            Self::TrapezoidalStringControl => "lead_bjt_trap_strings2.cir",
        }
    }

    fn path(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear.cir",
            Self::GearStringControl => {
                "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_gear_strings.cir"
            }
            Self::TrapNumericOwner => "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap.cir",
            Self::TrapStringControl => {
                "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap_strings.cir"
            }
            Self::TrapezoidalStringControl => {
                "Netlists/Certification_Tests/BUG_306_SON/lead_bjt_trap_strings2.cir"
            }
        }
    }

    fn record(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "netlists/certification_tests/bug_306_son/lead_bjt_gear.cir",
            Self::GearStringControl => {
                "netlists/certification_tests/bug_306_son/lead_bjt_gear_strings.cir"
            }
            Self::TrapNumericOwner => "netlists/certification_tests/bug_306_son/lead_bjt_trap.cir",
            Self::TrapStringControl => {
                "netlists/certification_tests/bug_306_son/lead_bjt_trap_strings.cir"
            }
            Self::TrapezoidalStringControl => {
                "netlists/certification_tests/bug_306_son/lead_bjt_trap_strings2.cir"
            }
        }
    }

    pub(super) fn contract(self) -> &'static str {
        match self {
            Self::GearNumericOwner => GEAR_OWNER_CONTRACT,
            Self::GearStringControl => GEAR_CONTROL_CONTRACT,
            Self::TrapNumericOwner => TRAP_OWNER_CONTRACT,
            Self::TrapStringControl => TRAP_CONTROL_CONTRACT,
            Self::TrapezoidalStringControl => TRAPEZOIDAL_CONTROL_CONTRACT,
        }
    }

    fn authored_method(self) -> &'static str {
        match self {
            Self::GearNumericOwner => "8",
            Self::GearStringControl => "gear",
            Self::TrapNumericOwner => "7",
            Self::TrapStringControl => "trap",
            Self::TrapezoidalStringControl => "trapezoidal",
        }
    }

    fn is_owner(self) -> bool {
        matches!(self, Self::GearNumericOwner | Self::TrapNumericOwner)
    }

    fn purpose(self) -> XyceStaticTranPlanPurpose {
        if self.is_owner() {
            XyceStaticTranPlanPurpose::GeneratedReferenceRelationalFamily
        } else {
            XyceStaticTranPlanPurpose::RelationalFamily
        }
    }
}

#[derive(Debug)]
struct Bug306Worker {
    plan: XyceStaticTranPlan,
    netlist: Netlist,
}

impl XyceTestRunner {
    fn validate_bug306_print_syntax(source: &str) -> Result<(), String> {
        let directives = source
            .lines()
            .map(Self::strip_netlist_comment)
            .map(str::trim)
            .filter(|line| line.to_ascii_lowercase().starts_with(".print"))
            .collect::<Vec<_>>();
        let [directive] = directives.as_slice() else {
            return Err(format!(
                "{LABEL} requires exactly one authored .PRINT directive"
            ));
        };
        let tokens = directive.split_ascii_whitespace().collect::<Vec<_>>();
        if tokens.len() != 7
            || !tokens[0].eq_ignore_ascii_case(".print")
            || !tokens[1].eq_ignore_ascii_case("tran")
            || !tokens[2].eq_ignore_ascii_case(&format!("precision={PRINT_PRECISION}"))
            || !tokens[3].eq_ignore_ascii_case(&format!("width={PRINT_WIDTH}"))
            || !tokens[4].eq_ignore_ascii_case("{i(vib)-ib(q1)}")
            || !tokens[5].eq_ignore_ascii_case("{i(vic)-ic(q1)}")
            || !tokens[6].eq_ignore_ascii_case("{i(vie)-ie(q1)}")
        {
            return Err(format!(
                "{LABEL} requires exact TRAN PRECISION={PRINT_PRECISION} WIDTH={PRINT_WIDTH} and ordered expression probes, got {directive:?}"
            ));
        }
        Ok(())
    }

    fn validate_bug306_comp_tolerances(
        source: &str,
    ) -> Result<Vec<XyceVerifyTransientTolerance>, String> {
        let columns = [
            "{I(VIB)-IB(Q1)}".to_string(),
            "{I(VIC)-IC(Q1)}".to_string(),
            "{I(VIE)-IE(Q1)}".to_string(),
        ];
        let tolerances = Self::xyce_verify_comp_tolerances(source, &columns)?;
        if tolerances.len() != columns.len()
            || tolerances.iter().any(|tolerance| {
                tolerance.relative.to_bits() != 0.01f64.to_bits()
                    || tolerance.absolute.to_bits() != 1.0e-6f64.to_bits()
                    || tolerance.zero.to_bits() != 1.0e-7f64.to_bits()
                    || tolerance.absolute_difference.to_bits() != 1.0e-12f64.to_bits()
                    || tolerance.offset.to_bits() != 0.0f64.to_bits()
            })
        {
            return Err(format!(
                "{LABEL} Release *COMP tolerances changed: {tolerances:?}"
            ));
        }
        Ok(tolerances)
    }

    fn bug306_nodes_match(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn validate_bug306_model(model: &rspice_core::netlist::ModelDef) -> bool {
        let expected: [(&str, Value); 38] = [
            ("BF", 130.0),
            ("BR", 1.0),
            ("IS", 3.0e-14),
            ("TF", 1.0e-8),
            ("VJS", 0.68),
            ("NE", 1.6),
            ("NC", 2.0),
            ("RB", 450.0),
            ("CJE", 1.0e-6),
            ("CJC", 1.0e-6),
            ("CJS", 1.0e-6),
            ("IKF", 0.002),
            ("IKR", 0.002),
            ("VAF", 50.0),
            ("VAR", 50.0),
            ("NF", 1.0),
            ("ISE", 0.0),
            ("NR", 1.0),
            ("ISC", 0.0),
            ("IRB", 0.0),
            ("RBM", 450.0),
            ("RE", 0.0),
            ("RC", 0.0),
            ("VJE", 0.75),
            ("MJE", 0.33),
            ("XTF", 0.0),
            ("VTF", 100.0),
            ("ITF", 0.0),
            ("PTF", 0.0),
            ("VJC", 0.75),
            ("MJS", 0.0),
            ("XTB", 0.0),
            ("EG", 1.11),
            ("XTI", 3.0),
            ("KF", 0.0),
            ("AF", 1.0),
            ("FC", 0.5),
            ("TNOM", 27.0),
        ];
        model.name.eq_ignore_ascii_case("qjunk")
            && model.model_type.eq_ignore_ascii_case("npn")
            && model.params.len() == expected.len()
            && expected.iter().all(|(name, value)| {
                model.params.iter().any(|(actual_name, actual_value)| {
                    actual_name.eq_ignore_ascii_case(name)
                        && actual_value.to_bits() == value.to_bits()
                })
            })
            && model.expr_params.is_empty()
            && model.string_params.is_empty()
            && model.string_vector_params.is_empty()
            && model.real_vector_params.is_empty()
            && model.real_vector_expr_params.is_empty()
            && model.integer_vector_params.is_empty()
    }

    fn bug306_plan_from_netlist(
        role: Bug306SonRole,
        path: &Path,
        source: &str,
        netlist: &Netlist,
    ) -> Result<XyceStaticTranPlan, String> {
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} must not acquire a control block"));
        }
        Self::reject_unsupported_source_directives(source)?;
        Self::validate_bug306_print_syntax(source)?;
        let output = Self::single_tran_print_output_request(source)?;
        if output.format.is_some() || output.file.is_some() || output.probes.len() != 3 {
            return Err(format!("{LABEL} {} PRINT schema changed", role.file_name()));
        }
        let plan = XyceStaticTranPlan {
            deck_path: path.to_path_buf(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: output.probes,
            }),
            output_override: false,
            timeint_conststep: Self::source_enables_constant_time_step_output(source),
            tran: Self::single_tran_analysis(netlist)?,
            steps: Self::step_commands(netlist)?,
            contract: if role.is_owner() {
                XyceStaticTranContract::WrapperStatic
            } else {
                XyceStaticTranContract::PlainStatic
            },
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        plan.validate_oracle_contract(role.purpose(), role.is_owner())?;
        Ok(plan)
    }

    fn validate_bug306_netlist(
        role: Bug306SonRole,
        netlist: &Netlist,
        plan: &XyceStaticTranPlan,
    ) -> Result<(), String> {
        let probes = plan
            .print
            .as_ref()
            .map(|request| {
                request
                    .probes
                    .iter()
                    .map(|probe| Self::normalize_probe(probe))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if probes != ["{i(vib)-ib(q1)}", "{i(vic)-ic(q1)}", "{i(vie)-ie(q1)}"]
            || !plan.steps.is_empty()
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.tran.step.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != TRANSIENT_STOP.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {} plan changed: {plan:?}",
                role.file_name()
            ));
        }
        Self::validate_bug306_comp_tolerances(&plan.source)?;
        if netlist.title.trim_end() != "Lead current test for level 1 BJT"
            || netlist.elements.len() != 4
            || netlist.models.len() != 1
            || !Self::validate_bug306_model(&netlist.models[0])
            || netlist.analyses.len() != 1
            || netlist.output_requests.len() != 1
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
            || !netlist
                .options
                .method
                .as_deref()
                .is_some_and(|method| method.eq_ignore_ascii_case(role.authored_method()))
            || netlist.options.nonlin_transient_rhstol.map(Value::to_bits)
                != Some(1.0e-7f64.to_bits())
        {
            return Err(format!(
                "{LABEL} {} typed envelope changed: elements={:?}, models={:?}, outputs={:?}, options={:?}, diagnostics={:?}",
                role.file_name(),
                netlist.elements,
                netlist.models,
                netlist.output_requests,
                netlist.options,
                netlist.diagnostics
            ));
        }
        let [vie, vic, vib, transistor] = netlist.elements.as_slice() else {
            unreachable!("BUG306 element count was checked")
        };
        let plain_dc_source = |element: &rspice_core::netlist::Element,
                               name: &str,
                               nodes: &[&str],
                               expected: Value| {
            element.provenance == ElementProvenance::Authored
                && element.name.eq_ignore_ascii_case(name)
                && Self::bug306_nodes_match(&element.nodes, nodes)
                && matches!(&element.kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                    if value.to_bits() == expected.to_bits())
        };
        if !plain_dc_source(vie, "vie", &["0", "1"], 0.0)
            || !plain_dc_source(vic, "vic", &["0", "3"], 5.0)
            || vib.provenance != ElementProvenance::Authored
            || !vib.name.eq_ignore_ascii_case("vib")
            || !Self::bug306_nodes_match(&vib.nodes, &["0", "2"])
            || !matches!(&vib.kind, ElementKind::VoltageSource(SourceSpec::Pulse {
                v1, v2, delay, rise, fall, width, ..
            }) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 1.0f64.to_bits()
                && delay.to_bits() == 1.0e-9f64.to_bits()
                && rise.to_bits() == 1.0e-9f64.to_bits()
                && fall.to_bits() == 1.0e-9f64.to_bits()
                && width.to_bits() == 1.0e-6f64.to_bits())
            || transistor.provenance != ElementProvenance::Authored
            || !transistor.name.eq_ignore_ascii_case("q1")
            || !Self::bug306_nodes_match(&transistor.nodes, &["3", "2", "1"])
            || !matches!(&transistor.kind, ElementKind::Bjt {
                model,
                bjt_type: rspice_core::netlist::BjtType::Npn,
                instance_params,
                deferred_params,
            } if model.eq_ignore_ascii_case("qjunk")
                && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!(
                "{LABEL} {} native topology changed",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn prepare_bug306_worker(
        &self,
        bytes: &[u8],
        role: Bug306SonRole,
        abort: &dyn AbortSignal,
    ) -> Result<Bug306Worker, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before planning"));
        }
        let source = std::str::from_utf8(bytes)
            .map_err(|error| format!("{LABEL} {} is not UTF-8: {error}", role.file_name()))?
            .to_string();
        if source.lines().any(|line| {
            let line = Self::strip_netlist_comment(line)
                .trim()
                .to_ascii_lowercase();
            line.starts_with(".inc") || line.starts_with(".include") || line.starts_with(".lib")
        }) {
            return Err(format!(
                "{LABEL} unexpectedly acquired a filesystem dependency"
            ));
        }
        let path = self.root.join(role.path());
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Nominal,
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist =
            Netlist::parse_with_path_and_options_and_abort(&source, &path, options, abort)
                .map_err(|error| format!("{LABEL} {} parse failed: {error}", role.file_name()))?;
        let plan = Self::bug306_plan_from_netlist(role, &path, &source, &netlist)?;
        Self::validate_bug306_netlist(role, &netlist, &plan)?;
        Ok(Bug306Worker { plan, netlist })
    }

    fn validate_bug306_table(role: Bug306SonRole, table: &XycePrnTable) -> Result<(), String> {
        if table.columns.len() != RESULT_COLUMNS
            || !(2..=MAX_ADAPTIVE_RESULT_POINTS).contains(&table.rows.len())
            || table.columns[0] != "Index"
            || !table.columns[1].eq_ignore_ascii_case("TIME")
            || table.columns[2..]
                .iter()
                .map(|column| Self::normalize_probe(column))
                .ne(["{i(vib)-ib(q1)}", "{i(vic)-ic(q1)}", "{i(vie)-ie(q1)}"])
        {
            return Err(format!(
                "{LABEL} {} table schema changed: columns={:?}, rows={}",
                role.file_name(),
                table.columns,
                table.rows.len()
            ));
        }
        let scalar_count = table
            .rows
            .len()
            .checked_mul(table.columns.len())
            .ok_or_else(|| format!("{LABEL} result scalar count overflowed"))?;
        if scalar_count > MAX_ADAPTIVE_RESULT_POINTS * RESULT_COLUMNS {
            return Err(format!("{LABEL} result resource envelope changed"));
        }
        let mut max_residual = 0.0f64;
        for (index, row) in table.rows.iter().enumerate() {
            if row.len() != RESULT_COLUMNS
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || row[1] < 0.0
                || row[1] > TRANSIENT_STOP
                || (index > 0 && row[1] <= table.rows[index - 1][1])
            {
                return Err(format!(
                    "{LABEL} {} row {index} is malformed: {row:?}",
                    role.file_name()
                ));
            }
            max_residual = row[2..]
                .iter()
                .map(|value| value.abs())
                .fold(max_residual, Value::max);
        }
        if table
            .rows
            .first()
            .is_none_or(|row| row[1].to_bits() != 0.0f64.to_bits())
            || table
                .rows
                .last()
                .is_none_or(|row| row[1].to_bits() != TRANSIENT_STOP.to_bits())
            || max_residual > MAX_KCL_RESIDUAL
        {
            return Err(format!(
                "{LABEL} {} KCL/endpoints changed: max residual={max_residual:e}",
                role.file_name()
            ));
        }
        Ok(())
    }

    fn execute_bug306_worker(
        &self,
        role: Bug306SonRole,
        worker: &Bug306Worker,
        start: Instant,
        abort: &dyn AbortSignal,
    ) -> Result<XycePrnTable, String> {
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let result = self
            .run_transient_family_netlist(&worker.plan, &worker.netlist, start, None, None)
            .map_err(|error| match error {
                SimulationError::Aborted => format!("{LABEL} exceeded its shared deadline"),
                other => format!("{LABEL} {} execution failed: {other}", role.file_name()),
            })?;
        let nonempty_branches = result
            .branch_currents
            .iter()
            .filter(|values| !values.is_empty())
            .collect::<Vec<_>>();
        let branch_peak = nonempty_branches
            .iter()
            .flat_map(|values| values.iter())
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        let point_count = result.time.len();
        let device_op_schema = result
            .device_op_traces
            .iter()
            .map(|trace| (trace.device_name.as_str(), trace.parameter.as_str()))
            .collect::<Vec<_>>();
        if !(2..=MAX_ADAPTIVE_RESULT_POINTS).contains(&point_count)
            || result.step_sizes.len() != point_count
            || result.time.iter().any(|value| !value.is_finite())
            || result.node_names != ["1", "3", "2", "Q1.__bint"]
            || result.branch_names != ["VIE", "VIC", "VIB", "Q1.__rb"]
            || result.voltages.len() != result.node_names.len()
            || result.voltages.iter().any(|values| {
                values.len() != point_count || values.iter().any(|value| !value.is_finite())
            })
            || nonempty_branches.len() != result.branch_names.len()
            || nonempty_branches.iter().any(|values| {
                values.len() != point_count || values.iter().any(|value| !value.is_finite())
            })
            // The BJT operating report gained `temp` (d8d148918), `power`
            // (310ed2d84) and the six intrinsic junction voltages
            // (4685ce40a); lead-current output retains the whole report.
            || result.device_op_traces.len() != 16
            || device_op_schema
                != [
                    ("Q1", "ic"),
                    ("Q1", "ib"),
                    ("Q1", "ie"),
                    ("Q1", "is"),
                    ("Q1", "vbe"),
                    ("Q1", "vce"),
                    ("Q1", "beta"),
                    ("Q1", "gm"),
                    ("Q1", "temp"),
                    ("Q1", "power"),
                    ("Q1", "vbe_intrinsic"),
                    ("Q1", "vce_intrinsic"),
                    ("Q1", "vbc_intrinsic"),
                    ("Q1", "vcsub_intrinsic"),
                    ("Q1", "vbsub_intrinsic"),
                    ("Q1", "vesub_intrinsic"),
                ]
            || result.device_op_traces.iter().any(|trace| {
                trace.values.len() != point_count
                    || trace.values.iter().any(|value| !value.is_finite())
            })
            || !result.digital_traces.is_empty()
            || !result.real_traces.is_empty()
            || !result.store_traces.is_empty()
            || !branch_peak.is_finite()
            || branch_peak < 1.0e-5
        {
            return Err(format!(
                "{LABEL} {} retained result envelope changed: time={}, steps={}, nodes={:?}, branches={:?}, active_branches={}, device_ops={device_op_schema:?}, peak={branch_peak:e}",
                role.file_name(),
                result.time.len(),
                result.step_sizes.len(),
                result.node_names,
                result.branch_names,
                nonempty_branches.len(),
            ));
        }
        let table =
            Self::transient_family_result_to_prn_table(&worker.plan, &worker.netlist, &result)
                .map_err(|error| {
                    format!(
                        "{LABEL} {} PRN projection failed: {error}",
                        role.file_name()
                    )
                })?;
        Self::validate_bug306_table(role, &table)?;
        Ok(table)
    }

    fn compare_bug306_relation(
        &self,
        owner_role: Bug306SonRole,
        owner_source: &str,
        good: &XycePrnTable,
        test_role: Bug306SonRole,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        Self::validate_bug306_table(owner_role, good)?;
        Self::validate_bug306_table(test_role, test)?;
        let tolerances = Self::validate_bug306_comp_tolerances(owner_source)?;
        let mismatches = self.compare_xyce_verify_transient_tables_with_probe_tolerances(
            good,
            test,
            &tolerances,
            PRINT_PRECISION,
        )?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} Release relation {} GOOD -> {} TEST found {} mismatch(es): {mismatches:?}",
                owner_role.file_name(),
                test_role.file_name(),
                mismatches.len()
            ))
        }
    }

    pub(super) fn validate_bug306_son_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} shared deadline expired before execution"));
        }
        let mut workers = BTreeMap::new();
        let mut outputs = BTreeMap::new();
        for member_role in Bug306SonRole::ALL {
            let bytes = fs::read(self.root.join(member_role.path())).map_err(|error| {
                format!("{LABEL} cannot read {}: {error}", member_role.file_name())
            })?;
            let worker = self.prepare_bug306_worker(&bytes, member_role, &abort)?;
            let table = self.execute_bug306_worker(member_role, &worker, start, &abort)?;
            workers.insert(member_role, worker);
            outputs.insert(member_role, table);
        }
        for (owner, control) in [
            (
                Bug306SonRole::GearNumericOwner,
                Bug306SonRole::GearStringControl,
            ),
            (
                Bug306SonRole::TrapNumericOwner,
                Bug306SonRole::TrapStringControl,
            ),
            (
                Bug306SonRole::TrapNumericOwner,
                Bug306SonRole::TrapezoidalStringControl,
            ),
        ] {
            let owner_worker = workers.get(&owner).expect("all BUG306 workers ran");
            self.compare_bug306_relation(
                owner,
                &owner_worker.plan.source,
                outputs.get(&owner).expect("all BUG306 outputs exist"),
                control,
                outputs.get(&control).expect("all BUG306 outputs exist"),
            )?;
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
    fn bug306_all_roles_parse_to_exact_native_level1_bjt_methods() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        for role in Bug306SonRole::ALL {
            let bytes = fs::read(root.join(role.path())).unwrap();
            let worker = runner.prepare_bug306_worker(&bytes, role, &abort).unwrap();
            assert!(
                worker
                    .netlist
                    .options
                    .method
                    .as_deref()
                    .is_some_and(|method| method.eq_ignore_ascii_case(role.authored_method()))
            );
            let base = runner.create_xyce_static_tran_engine_with_step_sizes(
                None,
                None,
                XyceTestRunner::xyce_initial_timestep_for_tran(&worker.plan.tran),
            );
            assert_eq!(
                base.config().integration_method,
                rspice_core::numerics::integration::IntegrationMethod::Trapezoidal
            );
            let resolved = base.resolved_for_netlist(&worker.netlist);
            let expected = match role {
                Bug306SonRole::GearNumericOwner | Bug306SonRole::GearStringControl => {
                    rspice_core::numerics::integration::IntegrationMethod::Gear2
                }
                Bug306SonRole::TrapNumericOwner
                | Bug306SonRole::TrapStringControl
                | Bug306SonRole::TrapezoidalStringControl => {
                    rspice_core::numerics::integration::IntegrationMethod::Trapezoidal
                }
            };
            assert_eq!(resolved.config().integration_method, expected);
        }
    }

    #[test]
    fn bug306_canonical_five_role_oracle_executes() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        runner.validate_bug306_son_oracle(Instant::now()).unwrap();
    }

    #[test]
    fn bug306_method_model_and_probe_mutations_fail_closed() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        let abort = DeadlineAbort::new(Instant::now(), 30_000);
        let role = Bug306SonRole::GearNumericOwner;
        let source = fs::read_to_string(root.join(role.path())).unwrap();
        for (label, changed) in [
            ("METHOD", source.replacen("method=8", "method=7", 1)),
            ("model", source.replacen("bf=130", "bf=131", 1)),
            (
                "COMP probe",
                source.replacen("{i(vib)-ib(q1)}", "{i(vib)-ic(q1)}", 1),
            ),
            (
                "PRINT probe",
                source.replacen(
                    ".print tran  PRECISION=10 WIDTH=19 {i(vib)-ib(q1)}",
                    ".print tran  PRECISION=10 WIDTH=19 {i(vib)-ic(q1)}",
                    1,
                ),
            ),
            (
                "PRINT precision",
                source.replacen("PRECISION=10", "PRECISION=9", 1),
            ),
            ("PRINT width", source.replacen("WIDTH=19", "WIDTH=18", 1)),
        ] {
            assert!(
                runner
                    .prepare_bug306_worker(changed.as_bytes(), role, &abort)
                    .is_err(),
                "{label} mutation must fail closed"
            );
        }
    }

    #[test]
    fn bug306_relation_rejects_shared_wrong_and_accepts_release_frame() {
        const SYNTHETIC_ROWS: usize = 64;
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, Default::default());
        let rows = (0..SYNTHETIC_ROWS)
            .map(|index| {
                vec![
                    index as Value,
                    TRANSIENT_STOP * (index as Value / (SYNTHETIC_ROWS - 1) as Value),
                    0.0,
                    0.0,
                    0.0,
                ]
            })
            .collect::<Vec<_>>();
        let baseline = XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "{I(VIB)-IB(Q1)}".into(),
                "{I(VIC)-IC(Q1)}".into(),
                "{I(VIE)-IE(Q1)}".into(),
            ],
            rows,
        };
        let source = fs::read_to_string(root.join(Bug306SonRole::GearNumericOwner.path())).unwrap();
        runner
            .compare_bug306_relation(
                Bug306SonRole::GearNumericOwner,
                &source,
                &baseline,
                Bug306SonRole::GearStringControl,
                &baseline,
            )
            .unwrap();
        let mut shared_wrong = baseline.clone();
        for row in &mut shared_wrong.rows {
            row[2] = 1.000_001e-6;
        }
        assert!(
            runner
                .compare_bug306_relation(
                    Bug306SonRole::GearNumericOwner,
                    &source,
                    &shared_wrong,
                    Bug306SonRole::GearStringControl,
                    &shared_wrong,
                )
                .is_err(),
            "identical METHOD aliases must not pass by sharing a physically invalid KCL waveform"
        );
        let mut boundary = baseline.clone();
        for row in &mut boundary.rows {
            row[2] = 1.0e-6;
        }
        runner
            .compare_bug306_relation(
                Bug306SonRole::GearNumericOwner,
                &source,
                &baseline,
                Bug306SonRole::GearStringControl,
                &boundary,
            )
            .expect("Release xyce_verify's normalized-RMS boundary is inclusive");
        let mut wrong = boundary;
        for row in &mut wrong.rows {
            row[2] = 1.000_001e-6;
        }
        assert!(
            runner
                .compare_bug306_relation(
                    Bug306SonRole::GearNumericOwner,
                    &source,
                    &baseline,
                    Bug306SonRole::GearStringControl,
                    &wrong,
                )
                .is_err()
        );

        let columns = vec!["Index".into(), "TIME".into(), "I(VIB)-IB(Q1)".into()];
        let full_domain = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 0.0, 0.0],
                vec![1.0, TRANSIENT_STOP / 2.0, 0.0],
                vec![2.0, TRANSIENT_STOP, 0.0],
            ],
        };
        let interior_domain = XycePrnTable {
            columns,
            rows: vec![
                vec![0.0, TRANSIENT_STOP / 4.0, 0.0],
                vec![1.0, 3.0 * TRANSIENT_STOP / 4.0, 0.0],
            ],
        };
        let tolerance = XyceTestRunner::validate_bug306_comp_tolerances(&source).unwrap()[0];
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    &full_domain,
                    &interior_domain,
                    tolerance,
                    PRINT_PRECISION,
                )
                .unwrap()
                .is_empty(),
            "the Release wrapper's numeric owner must be the interpolated GOOD waveform"
        );
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    &interior_domain,
                    &full_domain,
                    tolerance,
                    PRINT_PRECISION,
                )
                .is_err(),
            "reversing the Release GOOD/TEST direction must not silently pass"
        );
    }

    #[test]
    fn bug306_expired_shared_deadline_fails_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(
            &root,
            XyceRunnerConfig {
                max_time_per_test_ms: 1,
                ..Default::default()
            },
        );
        assert!(
            runner
                .validate_bug306_son_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
