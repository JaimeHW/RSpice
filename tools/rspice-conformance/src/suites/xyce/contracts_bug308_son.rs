use super::*;
use rspice_core::netlist::SourceSpec;

const LABEL: &str = "BUG_308_SON stepped TEMP output-framing relation";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_308_SON";
const OUTPUT_DIRECTORY: &str = "OutputData/Certification_Tests/BUG_308_SON";
const OWNER_CONTRACT: &str = "bug308_stepped_temp_output_framing_wrapper_owner";

const OUTPUT_FILE: &str = "comparator3.cir.res";
const EXPECTED_RES: &str = "STEP                 TEMP\n0          0.00000000e+00      \n1          1.00000000e+01      \nEnd of Xyce(TM) Parameter Sweep\n";

const PRINT_PRECISION: usize = 12;
const PRINT_WIDTH: usize = 21;
const MAX_ROWS_PER_RUN: usize = 4_096;
const MAX_STREAM_ROWS: usize = 2 * MAX_ROWS_PER_RUN;
const MAX_STREAM_BYTES: usize = 2_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Bug308SonRole {
    WrapperOwner,
}

impl Bug308SonRole {
    pub(super) fn for_record(record: &str) -> Option<Self> {
        (XyceTestRunner::normalize_manifest_key(record)
            == "netlists/certification_tests/bug_308_son/comparator3.cir")
            .then_some(Self::WrapperOwner)
    }

    pub(super) const fn contract(self) -> &'static str {
        OWNER_CONTRACT
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Bug308WorkerRole {
    SteppedOwner,
    Temp0Control,
    Temp10Control,
}

impl Bug308WorkerRole {
    const ALL: [Self; 3] = [Self::SteppedOwner, Self::Temp0Control, Self::Temp10Control];

    const fn file_name(self) -> &'static str {
        match self {
            Self::SteppedOwner => "comparator3.cir",
            Self::Temp0Control => "comparator0.cir",
            Self::Temp10Control => "comparator.cir",
        }
    }

    const fn expected_temp(self) -> Value {
        match self {
            Self::SteppedOwner | Self::Temp10Control => 10.0,
            Self::Temp0Control => 0.0,
        }
    }

    const fn expected_header(self) -> Option<bool> {
        match self {
            Self::Temp10Control => Some(false),
            Self::SteppedOwner | Self::Temp0Control => None,
        }
    }

    const fn expected_footer(self) -> Option<bool> {
        match self {
            Self::Temp0Control => Some(false),
            Self::SteppedOwner | Self::Temp10Control => None,
        }
    }

    const fn purpose(self) -> XyceStaticTranPlanPurpose {
        XyceStaticTranPlanPurpose::Bug308SonSteppedTempOutputFramingRelationalFamily
    }
}

#[derive(Debug)]
struct Bug308Worker {
    plan: XyceStaticTranPlan,
    netlist: Netlist,
}

impl XyceTestRunner {
    pub(super) fn validate_bug308_son_static_step_contract(
        netlist: &Netlist,
        steps: &[StepCommand],
    ) -> Result<(), String> {
        Self::validate_bug308_topology(netlist)?;
        if steps.len() != 1
            || !matches!(
                &steps[0],
                StepCommand {
                    target: StepTarget::Temp,
                    param_name: None,
                    sweep: StepSweep::Linear { start, stop, step },
                    ..
                } if start.to_bits() == 0.0f64.to_bits()
                    && stop.to_bits() == 10.0f64.to_bits()
                    && step.to_bits() == 10.0f64.to_bits()
            )
        {
            return Err(format!(
                "{LABEL} requires the exact finite two-run TEMP=0,10 step"
            ));
        }
        Ok(())
    }

    fn bug308_source(&self, role: Bug308WorkerRole) -> Result<String, String> {
        fs::read_to_string(self.root.join(FAMILY_DIRECTORY).join(role.file_name()))
            .map_err(|error| format!("{LABEL} cannot read {:?}: {error}", role.file_name()))
    }

    fn bug308_nodes(actual: &[String], expected: &[&str]) -> bool {
        actual.len() == expected.len()
            && actual
                .iter()
                .zip(expected)
                .all(|(actual, expected)| actual.eq_ignore_ascii_case(expected))
    }

    fn bug308_param(params: &[(String, Value)], name: &str) -> Option<Value> {
        let values = params
            .iter()
            .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .map(|(_, value)| *value)
            .collect::<Vec<_>>();
        matches!(values.as_slice(), [_]).then_some(values[0])
    }

    fn validate_bug308_topology(netlist: &Netlist) -> Result<(), String> {
        const MOS: [(&str, [&str; 4], &str, Value, Value); 17] = [
            ("M1", ["Anot", "A", "E1", "E1"], "PMOS", 3.6e-6, 1.2e-6),
            ("M2", ["Anot", "A", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M3", ["Bnot", "B", "E1", "E1"], "PMOS", 3.6e-6, 1.2e-6),
            ("M4", ["Bnot", "B", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M5", ["AorBnot", "0", "E1", "E1"], "PMOS", 1.8e-6, 3.6e-6),
            ("M6", ["AorBnot", "B", "1", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M7", ["1", "Anot", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M8", ["Lnot", "0", "E1", "E1"], "PMOS", 1.8e-6, 3.6e-6),
            ("M9", ["Lnot", "Bnot", "2", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M10", ["2", "A", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M11", ["Qnot", "0", "E1", "E1"], "PMOS", 3.6e-6, 3.6e-6),
            ("M12", ["Qnot", "AorBnot", "3", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("M13", ["3", "Lnot", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("MQLO", ["8", "Qnot", "E1", "E1"], "PMOS", 3.6e-6, 1.2e-6),
            ("MQL1", ["8", "Qnot", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
            ("MLTO", ["9", "Lnot", "E1", "E1"], "PMOS", 3.6e-6, 1.2e-6),
            ("MLT1", ["9", "Lnot", "0", "0"], "NMOS", 1.8e-6, 1.2e-6),
        ];
        let mos = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Mosfet { .. }))
            .collect::<Vec<_>>();
        if mos.len() != MOS.len() {
            return Err(format!("{LABEL} lost the exact 17-MOS comparator topology"));
        }
        for (element, (name, nodes, model_name, width, length)) in mos.into_iter().zip(MOS) {
            let ElementKind::Mosfet {
                model,
                compact_syntax,
                instance_params,
                deferred_params,
                ..
            } = &element.kind
            else {
                unreachable!()
            };
            if element.provenance != ElementProvenance::Authored
                || !element.name.eq_ignore_ascii_case(name)
                || !Self::bug308_nodes(&element.nodes, &nodes)
                || !model.eq_ignore_ascii_case(model_name)
                || *compact_syntax
                || instance_params.len() != 2
                || Self::bug308_param(instance_params, "W")
                    .is_none_or(|value| value.to_bits() != width.to_bits())
                || Self::bug308_param(instance_params, "L")
                    .is_none_or(|value| value.to_bits() != length.to_bits())
                || !deferred_params.is_empty()
            {
                return Err(format!("{LABEL} MOSFET {name} changed: {element:?}"));
            }
        }
        let capacitors = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::Capacitor { .. }))
            .collect::<Vec<_>>();
        if capacitors.len() != 2
            || !matches!(&capacitors[0].kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
                if capacitors[0].name.eq_ignore_ascii_case("CQ") && Self::bug308_nodes(&capacitors[0].nodes, &["Qnot", "0"])
                    && value.to_bits() == 30.0e-15f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
            || !matches!(&capacitors[1].kind, ElementKind::Capacitor { value, value_expr: None, initial_voltage: None, model: None, instance_params, deferred_params }
                if capacitors[1].name.eq_ignore_ascii_case("CL") && Self::bug308_nodes(&capacitors[1].nodes, &["Lnot", "0"])
                    && value.to_bits() == 10.0e-15f64.to_bits() && instance_params.is_empty() && deferred_params.is_empty())
        {
            return Err(format!(
                "{LABEL} capacitive load topology changed: {capacitors:?}"
            ));
        }
        let sources = netlist
            .elements
            .iter()
            .filter(|element| matches!(element.kind, ElementKind::VoltageSource(_)))
            .collect::<Vec<_>>();
        if sources.len() != 3
            || !matches!(&sources[0].kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if sources[0].name.eq_ignore_ascii_case("Vdd") && Self::bug308_nodes(&sources[0].nodes, &["E1", "0"])
                    && value.to_bits() == 5.0f64.to_bits())
            || !matches!(&sources[1].kind, ElementKind::VoltageSource(SourceSpec::Pulse { v1, v2, delay, rise, fall, width, period, pulse_count, width_defaults_to_zero })
                if sources[1].name.eq_ignore_ascii_case("Va") && Self::bug308_nodes(&sources[1].nodes, &["A", "0"])
                    && v1.to_bits() == 0.0f64.to_bits() && v2.to_bits() == 5.0f64.to_bits()
                    && delay.to_bits() == 0.0f64.to_bits() && rise.to_bits() == 0.1e-9f64.to_bits()
                    && fall.to_bits() == 0.1e-9f64.to_bits() && width.to_bits() == 15.0e-9f64.to_bits()
                    && period.to_bits() == 30.0e-9f64.to_bits() && pulse_count.to_bits() == 0.0f64.to_bits()
                    && !width_defaults_to_zero)
            || !matches!(&sources[2].kind, ElementKind::VoltageSource(SourceSpec::Dc(value))
                if sources[2].name.eq_ignore_ascii_case("Vb") && Self::bug308_nodes(&sources[2].nodes, &["B", "0"])
                    && value.to_bits() == 0.0f64.to_bits())
        {
            return Err(format!("{LABEL} source topology changed: {sources:?}"));
        }
        for model_name in ["NMOS", "PMOS"] {
            let model = netlist
                .models
                .iter()
                .find(|model| model.name.eq_ignore_ascii_case(model_name))
                .ok_or_else(|| format!("{LABEL} lost {model_name} model"))?;
            if !model.model_type.eq_ignore_ascii_case(model_name)
                || !matches!(model.params.as_slice(), [(name, value)]
                    if name.eq_ignore_ascii_case("LEVEL") && value.to_bits() == 9.0f64.to_bits())
            {
                return Err(format!("{LABEL} {model_name} LEVEL=9 model changed"));
            }
        }
        Ok(())
    }

    fn validate_bug308_worker(
        &self,
        role: Bug308WorkerRole,
        source: &str,
        path: &Path,
    ) -> Result<Bug308Worker, String> {
        let plan = self.static_tran_plan_for_path_with_purpose(path, role.purpose())?;
        let expected_steps = usize::from(role == Bug308WorkerRole::SteppedOwner);
        if plan.deck_path != path
            || plan.source.as_bytes() != source.as_bytes()
            || !matches!(plan.oracle, XyceStaticTranOracle::None)
            || plan.output_override
            || plan.timeint_conststep
            || plan.wrapper_tolerance.is_some()
            || plan.comparison_mode != XyceStaticTranComparisonMode::Pointwise
            || plan.steps.len() != expected_steps
            || plan.tran.step.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.stop.to_bits() != 1.0e-9f64.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
            || plan.print.as_ref().is_none_or(|print| {
                print
                    .probes
                    .iter()
                    .map(String::as_str)
                    .ne(["v(a)", "v(b)", "{v(9)+0.2}", "v(8)"])
            })
        {
            return Err(format!("{LABEL} {role:?} transient plan changed: {plan:?}"));
        }
        if role == Bug308WorkerRole::SteppedOwner
            && !matches!(&plan.steps[0], StepCommand { target: StepTarget::Temp, name, param_name: None, sweep: StepSweep::Linear { start, stop, step } }
                if name.eq_ignore_ascii_case("TEMP") && start.to_bits() == 0.0f64.to_bits()
                    && stop.to_bits() == 10.0f64.to_bits() && step.to_bits() == 10.0f64.to_bits())
        {
            return Err(format!("{LABEL} owner TEMP step changed: {:?}", plan.steps));
        }
        let netlist = Self::parse_xyce_netlist(source, path)
            .map_err(|error| format!("{LABEL} {role:?} Xyce parse failed: {error}"))?;
        if !netlist.diagnostics.is_empty()
            || netlist.title != "COMPARATOR - BSIM3 Transient Analysis"
            || netlist.elements.len() != 22
            || netlist.models.len() != 2
            || netlist.analyses.len() != expected_steps + 1
            || netlist.output_requests.len() != 1
            || !netlist.subcircuits.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.measurements.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || netlist.options.temp.map(Value::to_bits) != Some(role.expected_temp().to_bits())
            || netlist.options.timeint_reltol.map(Value::to_bits) != Some(1.0e-4f64.to_bits())
            || netlist.options.method.as_deref() != Some("GEAR")
            || netlist.options.device_debug_level != Some(-100)
            || netlist.options.timeint_debug_level != Some(-100)
            || netlist.options.output_print_header != role.expected_header()
            || netlist.options.output_print_footer != role.expected_footer()
        {
            return Err(format!(
                "{LABEL} {role:?} typed envelope changed: analyses={:?}, options={:?}, diagnostics={:?}",
                netlist.analyses, netlist.options, netlist.diagnostics
            ));
        }
        Self::validate_bug308_topology(&netlist)?;
        let request = &netlist.output_requests[0];
        if request.directive != OutputDirectiveKind::Print
            || request.analysis != Some(rspice_core::netlist::OutputAnalysisKind::Tran)
            || request.print_delimiter.as_ref() != Some(&PrintDelimiter::Whitespace)
            || request.print_precision != Some(PRINT_PRECISION as i32)
            || request.print_width != Some(PRINT_WIDTH as i32)
            || request.operands.iter().map(String::as_str).ne([
                "v(a)",
                "v(b)",
                "{v(9)+0.2}",
                "v(8)",
            ])
        {
            return Err(format!("{LABEL} {role:?} typed PRINT changed: {request:?}"));
        }
        let resolved = self
            .create_xyce_engine()
            .resolved_for_netlist(&netlist)
            .config()
            .integration_method;
        if resolved != rspice_core::numerics::integration::IntegrationMethod::Gear2 {
            return Err(format!(
                "{LABEL} {role:?} METHOD=GEAR resolved to {resolved:?}"
            ));
        }
        let circuit = self
            .create_xyce_engine()
            .build_circuit(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} assembly failed: {error}"))?;
        if circuit.has_generated_veriloga_devices()
            || circuit.device_op_report().entries.len() != 17
        {
            return Err(format!("{LABEL} {role:?} lost its native 17-BSIM3 route"));
        }
        Ok(Bug308Worker { plan, netlist })
    }

    fn validate_bug308_table(
        role: Bug308WorkerRole,
        temperature: Value,
        table: &XycePrnTable,
        result: &TransientResult,
    ) -> Result<(), String> {
        let columns = ["Index", "TIME", "V(A)", "V(B)", "{V(9)+0.2}", "V(8)"];
        if table.columns.len() != columns.len()
            || table
                .columns
                .iter()
                .zip(columns)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
            || !(2..=MAX_ROWS_PER_RUN).contains(&table.rows.len())
            || !matches!(temperature, 0.0 | 10.0)
        {
            return Err(format!(
                "{LABEL} {role:?} TEMP={temperature} table shape changed: {:?}/{}",
                table.columns,
                table.rows.len()
            ));
        }
        let mut previous = None;
        let mut ranges = [(Value::INFINITY, Value::NEG_INFINITY); 4];
        for (index, row) in table.rows.iter().enumerate() {
            let expected_va = if row.get(1).copied().unwrap_or(Value::NAN) <= 0.1e-9 {
                5.0 * row[1] / 0.1e-9
            } else {
                5.0
            };
            if row.len() != columns.len()
                || row.iter().any(|value| !value.is_finite())
                || row[0].to_bits() != (index as Value).to_bits()
                || previous.is_some_and(|time| row[1] <= time)
                || (row[2] - expected_va).abs() > 1.0e-10
                || row[3].abs() > 1.0e-12
                || !(-0.5..=5.5).contains(&row[2])
                || !(-0.3..=5.7).contains(&row[4])
                || !(-0.5..=5.5).contains(&row[5])
            {
                return Err(format!("{LABEL} {role:?} malformed row {index}: {row:?}"));
            }
            previous = Some(row[1]);
            for (range, value) in ranges.iter_mut().zip(&row[2..]) {
                range.0 = range.0.min(*value);
                range.1 = range.1.max(*value);
            }
        }
        let first = table.rows.first().expect("validated nonempty");
        let last = table.rows.last().expect("validated nonempty");
        if first[1].abs() > 1.0e-18
            || (last[1] - 1.0e-9).abs() > 1.0e-15
            || ranges[0].1 - ranges[0].0 < 4.0
            || ranges[2].1 - ranges[2].0 < 0.1
            || !(0.1..=0.3).contains(&first[4])
            || !(4.5..=5.5).contains(&first[5])
            || !(4.5..=5.5).contains(&last[5])
        {
            return Err(format!(
                "{LABEL} {role:?} TEMP={temperature} waveform became trivial or incomplete: first={first:?}, last={last:?}, ranges={ranges:?}"
            ));
        }
        Self::validate_bug308_projection(role, table, result)?;
        Ok(())
    }

    fn validate_bug308_projection(
        role: Bug308WorkerRole,
        table: &XycePrnTable,
        result: &TransientResult,
    ) -> Result<(), String> {
        let node = |name: &str| -> Result<&[Value], String> {
            let index = result
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .ok_or_else(|| format!("{LABEL} {role:?} result lost node {name}"))?;
            result
                .voltages
                .get(index)
                .map(Vec::as_slice)
                .filter(|values| values.len() == result.time.len())
                .ok_or_else(|| format!("{LABEL} {role:?} node {name} trace is incomplete"))
        };
        let va = node("A")?;
        let vb = node("B")?;
        let v8 = node("8")?;
        let v9 = node("9")?;
        if table.rows.len() != result.time.len() {
            return Err(format!(
                "{LABEL} {role:?} PRN projection dropped accepted samples"
            ));
        }
        for (index, (row, time)) in table.rows.iter().zip(&result.time).enumerate() {
            if row[1].to_bits() != time.to_bits()
                || row[2].to_bits() != va[index].to_bits()
                || row[3].to_bits() != vb[index].to_bits()
                || row[4].to_bits() != (v9[index] + 0.2).to_bits()
                || row[5].to_bits() != v8[index].to_bits()
            {
                return Err(format!(
                    "{LABEL} {role:?} typed PRN projection changed at row {index}"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug308_raw_stream(
        text: &str,
        tables: &[XycePrnTable],
        expected_footer: XycePrnFooter,
    ) -> Result<(), String> {
        if text.contains('\r') || !text.ends_with('\n') {
            return Err(format!("{LABEL} raw PRN is not canonical LF text"));
        }
        let lines = text.lines().collect::<Vec<_>>();
        let expected_data_rows = tables.iter().map(|table| table.rows.len()).sum::<usize>();
        if lines.len() != expected_data_rows + 2 {
            return Err(format!(
                "{LABEL} raw PRN line count changed: {}",
                lines.len()
            ));
        }
        let header = lines[0].split_ascii_whitespace().collect::<Vec<_>>();
        let expected_header = ["Index", "TIME", "V(A)", "V(B)", "{V(9)+0.2}", "V(8)"];
        if header.len() != expected_header.len()
            || header
                .iter()
                .zip(expected_header)
                .any(|(actual, expected)| !actual.eq_ignore_ascii_case(expected))
        {
            return Err(format!("{LABEL} raw PRN header changed: {header:?}"));
        }
        let footer = match expected_footer {
            XycePrnFooter::None => {
                return Err(format!("{LABEL} combined stream requires a footer"));
            }
            XycePrnFooter::Simulation => "End of Xyce(TM) Simulation",
            XycePrnFooter::ParameterSweep => "End of Xyce(TM) Parameter Sweep",
        };
        if lines.last().copied() != Some(footer)
            || lines.iter().filter(|line| line.trim() == footer).count() != 1
            || lines
                .iter()
                .filter(|line| line.trim_start().starts_with("End of Xyce(TM)"))
                .count()
                != 1
            || lines
                .iter()
                .filter(|line| line.split_ascii_whitespace().next() == Some("Index"))
                .count()
                != 1
        {
            return Err(format!("{LABEL} raw PRN footer/header framing changed"));
        }
        let mut line_index = 1usize;
        for table in tables {
            for expected_index in 0..table.rows.len() {
                let fields = lines[line_index]
                    .split_ascii_whitespace()
                    .collect::<Vec<_>>();
                if fields.len() != table.columns.len()
                    || fields[0].parse::<usize>().ok() != Some(expected_index)
                {
                    return Err(format!(
                        "{LABEL} raw PRN local Index framing changed at line {line_index}: {:?}",
                        lines[line_index]
                    ));
                }
                line_index += 1;
            }
        }
        Ok(())
    }

    fn validate_bug308_relation(
        &self,
        controls: &[XycePrnTable],
        owner: &[XycePrnTable],
    ) -> Result<(), String> {
        if controls.len() != 2 || owner.len() != 2 {
            return Err(format!("{LABEL} requires two ordered TEMP runs"));
        }
        for (run, (good, test)) in controls.iter().zip(owner).enumerate() {
            if good.rows.len() != test.rows.len() {
                return Err(format!("{LABEL} run {run} row counts differ"));
            }
            for (row, (good_row, test_row)) in good.rows.iter().zip(&test.rows).enumerate() {
                if (good_row[1] - test_row[1]).abs() > 1.0e-14 {
                    return Err(format!(
                        "{LABEL} historical line-aligned time check failed at run {run}, row {row}"
                    ));
                }
            }
            let mismatches = self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
                good,
                test,
                XyceVerifyTransientTolerance::release_7_10_default(),
                PRINT_PRECISION,
            )?;
            if !mismatches.is_empty() {
                return Err(format!(
                    "{LABEL} directional Release-7.10 controls-GOOD/owner-TEST run {run} failed: {mismatches:?}"
                ));
            }
        }
        Ok(())
    }

    fn validate_bug308_res(output: &[u8], temperatures: &[Value]) -> Result<(), String> {
        let canonical = Self::canonical_lf_text_identity(LABEL, output)?;
        if canonical.as_slice() != EXPECTED_RES.as_bytes()
            || temperatures.len() != 2
            || temperatures[0].to_bits() != 0.0f64.to_bits()
            || temperatures[1].to_bits() != 10.0f64.to_bits()
        {
            return Err(format!("{LABEL} retained STEP TEMP .res semantics changed"));
        }
        Ok(())
    }

    pub(super) fn validate_bug308_son_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }
        let output = fs::read(self.root.join(OUTPUT_DIRECTORY).join(OUTPUT_FILE))
            .map_err(|error| format!("{LABEL} cannot read {OUTPUT_FILE}: {error}"))?;
        let mut workers = BTreeMap::new();
        for worker_role in Bug308WorkerRole::ALL {
            let path = self
                .root
                .join(FAMILY_DIRECTORY)
                .join(worker_role.file_name());
            let source = self.bug308_source(worker_role)?;
            workers.insert(
                worker_role,
                self.validate_bug308_worker(worker_role, &source, &path)?,
            );
        }
        let owner = workers
            .get(&Bug308WorkerRole::SteppedOwner)
            .ok_or_else(|| format!("{LABEL} lost owner"))?;
        let step_engine = self.create_xyce_engine();
        let step_plan = step_engine
            .plan_step_commands_with_abort(
                &owner.netlist,
                &owner.plan.steps,
                xyce_step_plan_limits(),
                &abort,
            )
            .map_err(|error| format!("{LABEL} TEMP STEP planning failed: {error}"))?;
        if step_plan.total_runs() != 2 {
            return Err(format!("{LABEL} TEMP STEP materialized run count changed"));
        }
        let mut temperatures = Vec::with_capacity(2);
        let mut owner_tables = Vec::with_capacity(2);
        for run_index in 0..2 {
            let (bindings, run_netlist) = step_engine
                .materialize_step_run_with_abort(&step_plan, run_index, &abort)
                .map_err(|error| format!("{LABEL} owner run {run_index} failed: {error}"))?
                .into_parts();
            let [temperature] = bindings.as_slice() else {
                return Err(format!("{LABEL} owner TEMP binding shape changed"));
            };
            if run_netlist.options.temp.map(Value::to_bits) != Some(temperature.to_bits()) {
                return Err(format!("{LABEL} owner effective TEMP binding changed"));
            }
            let result = self
                .run_transient_family_netlist(&owner.plan, &run_netlist, start, None, None)
                .map_err(|error| format!("{LABEL} owner TEMP={temperature} failed: {error}"))?;
            let table =
                Self::transient_family_result_to_prn_table(&owner.plan, &run_netlist, &result)?;
            Self::validate_bug308_table(
                Bug308WorkerRole::SteppedOwner,
                *temperature,
                &table,
                &result,
            )?;
            temperatures.push(*temperature);
            owner_tables.push(table);
        }
        Self::validate_bug308_res(&output, &temperatures)?;

        let mut control_tables = Vec::with_capacity(2);
        for control_role in [
            Bug308WorkerRole::Temp0Control,
            Bug308WorkerRole::Temp10Control,
        ] {
            let worker = workers
                .get(&control_role)
                .ok_or_else(|| format!("{LABEL} lost {control_role:?}"))?;
            let result = self
                .run_transient_family_netlist(&worker.plan, &worker.netlist, start, None, None)
                .map_err(|error| format!("{LABEL} {control_role:?} failed: {error}"))?;
            let table =
                Self::transient_family_result_to_prn_table(&worker.plan, &worker.netlist, &result)?;
            Self::validate_bug308_table(
                control_role,
                control_role.expected_temp(),
                &table,
                &result,
            )?;
            control_tables.push(table);
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        Self::validate_bug308_relation(self, &control_tables, &owner_tables)?;

        let owner_request = &owner.netlist.output_requests[0];
        let owner_raw = serialize_xyce_prn_sequence(
            &owner_tables,
            owner_request,
            &owner.netlist.options,
            XycePrnFooter::ParameterSweep,
            XycePrnLimits::new(MAX_STREAM_ROWS, MAX_STREAM_BYTES),
        )
        .map_err(|error| format!("{LABEL} owner PRN rendering failed: {error}"))?;
        Self::validate_bug308_raw_stream(&owner_raw, &owner_tables, XycePrnFooter::ParameterSweep)?;
        let temp0 = workers
            .get(&Bug308WorkerRole::Temp0Control)
            .expect("worker census");
        let temp10 = workers
            .get(&Bug308WorkerRole::Temp10Control)
            .expect("worker census");
        let mut controls_raw = serialize_xyce_prn_sequence(
            &control_tables[..1],
            &temp0.netlist.output_requests[0],
            &temp0.netlist.options,
            XycePrnFooter::None,
            XycePrnLimits::new(MAX_STREAM_ROWS, MAX_STREAM_BYTES),
        )
        .map_err(|error| format!("{LABEL} TEMP=0 PRN rendering failed: {error}"))?;
        controls_raw.push_str(
            &serialize_xyce_prn_sequence(
                &control_tables[1..],
                &temp10.netlist.output_requests[0],
                &temp10.netlist.options,
                XycePrnFooter::Simulation,
                XycePrnLimits::new(
                    MAX_STREAM_ROWS,
                    MAX_STREAM_BYTES
                        .checked_sub(controls_raw.len())
                        .ok_or_else(|| format!("{LABEL} raw output envelope overflowed"))?,
                ),
            )
            .map_err(|error| format!("{LABEL} TEMP=10 PRN rendering failed: {error}"))?,
        );
        if controls_raw.len() > MAX_STREAM_BYTES {
            return Err(format!(
                "{LABEL} concatenated control PRN exceeded byte envelope"
            ));
        }
        Self::validate_bug308_raw_stream(
            &controls_raw,
            &control_tables,
            XycePrnFooter::Simulation,
        )?;
        if owner_raw == controls_raw
            || !owner_raw.ends_with("End of Xyce(TM) Parameter Sweep\n")
            || !controls_raw.ends_with("End of Xyce(TM) Simulation\n")
        {
            return Err(format!(
                "{LABEL} distinct Release footer contexts collapsed"
            ));
        }
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded deadline"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn runner() -> XyceTestRunner {
        XyceTestRunner::new(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../..")
                .join("tests/xyce"),
            XyceRunnerConfig::default(),
        )
    }

    #[test]
    fn bug308_workers_preserve_typed_layout_topology_and_step_semantics() {
        let runner = runner();
        for role in Bug308WorkerRole::ALL {
            let path = runner.root.join(FAMILY_DIRECTORY).join(role.file_name());
            let source = runner.bug308_source(role).unwrap();
            runner.validate_bug308_worker(role, &source, &path).unwrap();
        }
    }

    #[test]
    fn bug308_output_options_print_layout_and_topology_mutations_fail_closed() {
        let canonical = runner();
        let role = Bug308WorkerRole::Temp0Control;
        let source = canonical.bug308_source(role).unwrap();
        for mutation in [
            source.replace("PRINTFOOTER=false", "PRINTFOOTER=true"),
            source.replace("precision=12", "precision=11"),
            source.replace("width=21", "width=20"),
            source.replacen("M1 Anot A E1 E1", "M1 Anot B E1 E1", 1),
            source.replace("method=gear", "method=trap"),
        ] {
            let temporary = tempfile::tempdir().unwrap();
            let path = temporary.path().join(role.file_name());
            fs::write(&path, &mutation).unwrap();
            let isolated = XyceTestRunner::new(temporary.path(), XyceRunnerConfig::default());
            assert!(
                isolated
                    .validate_bug308_worker(role, &mutation, &path)
                    .is_err()
            );
        }
    }

    #[test]
    fn bug308_renderer_preserves_header_footer_and_local_index_boundaries() {
        let source =
            "framing\nV1 a 0 1\n.PRINT TRAN PRECISION=12 WIDTH=21 V(a)\n.TRAN 1n 1n\n.END\n";
        let netlist = Netlist::parse(source).unwrap();
        let table = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(A)".into()],
            rows: vec![vec![0.0, 0.0, 1.0], vec![1.0, 1e-9, 1.0]],
        };
        for (footer, marker) in [
            (XycePrnFooter::None, None),
            (
                XycePrnFooter::Simulation,
                Some("End of Xyce(TM) Simulation\n"),
            ),
            (
                XycePrnFooter::ParameterSweep,
                Some("End of Xyce(TM) Parameter Sweep\n"),
            ),
        ] {
            let text = serialize_xyce_prn_sequence(
                &[table.clone(), table.clone()],
                &netlist.output_requests[0],
                &netlist.options,
                footer,
                XycePrnLimits::new(4, 10_000),
            )
            .unwrap();
            let lines = text.lines().collect::<Vec<_>>();
            assert_eq!(
                lines[0],
                format!(
                    "Index{}TIME{}V(A){}",
                    " ".repeat(9),
                    " ".repeat(18),
                    " ".repeat(9)
                )
            );
            assert_eq!(
                lines[1],
                "0        0.000000000000e+00    1.000000000000e+00"
            );
            assert_eq!(
                text.lines().filter(|line| line.contains("Index")).count(),
                1
            );
            assert_eq!(
                text.lines()
                    .filter(|line| line.split_ascii_whitespace().next() == Some("0"))
                    .count(),
                2
            );
            assert_eq!(
                marker.is_some_and(|marker| text.ends_with(marker)),
                marker.is_some()
            );
        }
        assert_eq!(
            XyceTestRunner::xyce_prn_scientific_text(0.0, 12).unwrap(),
            "0.000000000000e0"
        );
        assert_eq!(
            XyceTestRunner::xyce_prn_scientific_text(1.0e-9, 12).unwrap(),
            "1.000000000000e-9"
        );

        let mut extended = netlist.output_requests[0].clone();
        extended.print_precision = Some(17);
        extended.print_width = Some(-1);
        let extended_text = serialize_xyce_prn_sequence(
            std::slice::from_ref(&table),
            &extended,
            &netlist.options,
            XycePrnFooter::None,
            XycePrnLimits::new(2, 10_000),
        )
        .unwrap();
        assert!(extended_text.contains("0.00000000000000000e+00"));

        extended.print_precision = Some(0);
        let zero_precision_text = serialize_xyce_prn_sequence(
            std::slice::from_ref(&table),
            &extended,
            &netlist.options,
            XycePrnFooter::None,
            XycePrnLimits::new(2, 10_000),
        )
        .unwrap();
        assert!(zero_precision_text.contains("0e+00"));
        assert!(
            serialize_xyce_prn_sequence(
                &[table],
                &netlist.output_requests[0],
                &netlist.options,
                XycePrnFooter::Simulation,
                XycePrnLimits::new(1, 10_000),
            )
            .is_err()
        );
    }

    #[test]
    fn bug308_directional_xyce_verify_and_time_alignment_fail_closed() {
        let runner = runner();
        let table = |times: &[Value]| XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(A)".into()],
            rows: times
                .iter()
                .enumerate()
                .map(|(index, time)| vec![index as Value, *time, 1.0])
                .collect(),
        };
        let good = table(&[0.0, 0.5, 1.0]);
        let test = table(&[0.25, 0.75]);
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    &good,
                    &test,
                    XyceVerifyTransientTolerance::release_7_10_default(),
                    PRINT_PRECISION,
                )
                .unwrap()
                .is_empty()
        );
        assert!(
            runner
                .compare_xyce_verify_transient_tables_with_uniform_tolerance(
                    &test,
                    &good,
                    XyceVerifyTransientTolerance::release_7_10_default(),
                    PRINT_PRECISION,
                )
                .is_err()
        );
        let shifted = table(&[0.0, 0.5 + 2.0e-14, 1.0]);
        assert!(
            runner
                .validate_bug308_relation(
                    &[good.clone(), good.clone()],
                    &[shifted.clone(), shifted]
                )
                .is_err()
        );

        let shared_wrong = XycePrnTable {
            columns: vec![
                "Index".into(),
                "TIME".into(),
                "V(A)".into(),
                "V(B)".into(),
                "{V(9)+0.2}".into(),
                "V(8)".into(),
            ],
            rows: vec![
                vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                vec![1.0, 1.0e-9, 5.0, 0.0, 0.2, 0.2],
            ],
        };
        assert!(
            runner
                .validate_bug308_relation(
                    &[shared_wrong.clone(), shared_wrong.clone()],
                    &[shared_wrong.clone(), shared_wrong.clone()],
                )
                .is_ok(),
            "counterfactual must prove the historical relation alone admits identical wrong tables"
        );
        let shared_wrong_result = TransientResult {
            current_impulses: None,
            voltage_impulses: None,
            time: vec![0.0, 1.0e-9],
            step_sizes: vec![0.0, 1.0e-9],
            voltages: vec![
                vec![0.0, 5.0],
                vec![0.0, 0.0],
                vec![0.0, 0.2],
                vec![-0.2, 0.0],
            ],
            branch_currents: Vec::new(),
            num_nodes: 4,
            node_names: vec!["A".into(), "B".into(), "8".into(), "9".into()],
            branch_names: Vec::new(),
            digital_traces: Vec::new(),
            digital_buses: Vec::new(),
            real_traces: Vec::new(),
            hdl_sources: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        };
        assert!(
            XyceTestRunner::validate_bug308_table(
                Bug308WorkerRole::SteppedOwner,
                0.0,
                &shared_wrong,
                &shared_wrong_result,
            )
            .is_err(),
            "independent source/logic gate must reject identical shared-wrong owner and controls"
        );
    }

    #[test]
    fn bug308_owner_executes_complete_release_relation() {
        let runner = runner();
        runner.validate_bug308_son_oracle(Instant::now()).unwrap();
    }

    #[test]
    fn bug308_expired_deadline_fails_closed() {
        let canonical = runner();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let expired = XyceTestRunner::new(canonical.root.clone(), config);
        assert!(
            expired
                .validate_bug308_son_oracle(Instant::now() - Duration::from_millis(10))
                .is_err()
        );
    }
}
