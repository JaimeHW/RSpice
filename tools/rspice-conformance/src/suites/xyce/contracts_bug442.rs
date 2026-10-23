use super::*;
use rspice_core::engine::TransientCheckpointEncoding;
use rspice_core::netlist::{SimulationOptions, SourceSpec};

const LABEL: &str = "BUG_442 packed/unpacked transmission-line restart wrapper";

pub(super) const BUG442_OWNER_CONTRACT: &str = "bug442_packed_unpacked_tline_restart_wrapper_owner";
pub(super) const BUG442_WORKER_CONTRACT: &str = "bug442_packed_unpacked_tline_restart_worker";

const PRINT_STEP: Value = 0.25e-9;
const FINAL_STOP: Value = 50.0e-9;
const RESTART_INTERVAL: Value = 5.0e-9;
const RESTART_TARGET: Value = 10.0e-9;
const SAVE_TIMES: [Value; 3] = [0.0, RESTART_INTERVAL, RESTART_TARGET];
const MAX_CHECKPOINT_BYTES: usize = 16 * 1024 * 1024;
const MAX_RESULT_ROWS: usize = 200_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug442Role {
    WrapperOwner,
    PackedBaseline,
    UnpackedBaseline,
    PackedRestarted,
    UnpackedRestarted,
}

impl Bug442Role {
    pub(super) const ALL: [Self; 5] = [
        Self::WrapperOwner,
        Self::PackedBaseline,
        Self::UnpackedBaseline,
        Self::PackedRestarted,
        Self::UnpackedRestarted,
    ];
    #[cfg(test)]
    const WORKERS: [Self; 4] = [
        Self::PackedBaseline,
        Self::UnpackedBaseline,
        Self::PackedRestarted,
        Self::UnpackedRestarted,
    ];

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) const fn contract(self) -> &'static str {
        match self {
            Self::WrapperOwner => BUG442_OWNER_CONTRACT,
            _ => BUG442_WORKER_CONTRACT,
        }
    }

    pub(super) fn path(self) -> &'static str {
        match self {
            Self::WrapperOwner => "Netlists/Certification_Tests/BUG_442/bug_442.cir",
            Self::PackedBaseline => "Netlists/Certification_Tests/BUG_442/bug_442_baseline.cir",
            Self::UnpackedBaseline => {
                "Netlists/Certification_Tests/BUG_442/bug_442_baseline_unpacked.cir"
            }
            Self::PackedRestarted => "Netlists/Certification_Tests/BUG_442/bug_442_restarted.cir",
            Self::UnpackedRestarted => {
                "Netlists/Certification_Tests/BUG_442/bug_442_restarted_unpacked.cir"
            }
        }
    }

    pub(super) fn record(self) -> &'static str {
        match self {
            Self::WrapperOwner => "netlists/certification_tests/bug_442/bug_442.cir",
            Self::PackedBaseline => "netlists/certification_tests/bug_442/bug_442_baseline.cir",
            Self::UnpackedBaseline => {
                "netlists/certification_tests/bug_442/bug_442_baseline_unpacked.cir"
            }
            Self::PackedRestarted => "netlists/certification_tests/bug_442/bug_442_restarted.cir",
            Self::UnpackedRestarted => {
                "netlists/certification_tests/bug_442/bug_442_restarted_unpacked.cir"
            }
        }
    }

    const fn print_file(self) -> Option<&'static str> {
        match self {
            Self::WrapperOwner => None,
            Self::PackedBaseline => Some("bug_442_baseline.cir.prn"),
            Self::UnpackedBaseline => Some("bug_442_baseline_unpacked.cir.prn"),
            Self::PackedRestarted => Some("bug_442_restarted.cir.prn"),
            Self::UnpackedRestarted => Some("bug_442_restarted_unpacked.cir.prn"),
        }
    }
}

impl XyceTestRunner {
    fn validate_bug442_role_source(role: Bug442Role, source: &str) -> Result<(), String> {
        if role == Bug442Role::WrapperOwner {
            return source
                .is_empty()
                .then_some(())
                .ok_or_else(|| format!("{LABEL} wrapper owner must remain exactly zero bytes"));
        }
        let expected_restart = match role {
            Bug442Role::PackedBaseline => ".options restart job=trans_test initial_interval=5n",
            Bug442Role::UnpackedBaseline => {
                ".options restart pack=0 job=trans_test_unpacked initial_interval=5n"
            }
            Bug442Role::PackedRestarted => ".options restart file=trans_test1e-08",
            Bug442Role::UnpackedRestarted => ".options restart file=trans_test_unpacked1e-08",
            Bug442Role::WrapperOwner => unreachable!(),
        };
        let expected = [
            "transmission line circuit".to_string(),
            "vin 1 0 pulse(0 5 0 0.1n 0.1n 5n 25n)".to_string(),
            "rin 1 2 50".to_string(),
            "tline 2 0 3 0 z0=50 td=10n".to_string(),
            "rl 3 0 50".to_string(),
            ".tran 0.25n 50n".to_string(),
            format!(
                ".print tran file={} v(2) v(3)",
                role.print_file().expect("worker has print file")
            ),
            expected_restart.to_string(),
            ".end".to_string(),
        ];
        let significant = source
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty() && !line.starts_with('*'))
            .map(str::to_ascii_lowercase)
            .collect::<Vec<_>>();
        if significant != expected {
            return Err(format!(
                "{LABEL} {role:?} authored source envelope changed: {significant:?}"
            ));
        }
        Ok(())
    }

    fn bug442_element<'a>(
        netlist: &'a Netlist,
        name: &str,
    ) -> Result<&'a rspice_core::netlist::Element, String> {
        netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .ok_or_else(|| format!("{LABEL} is missing element {name}"))
    }

    fn bug442_resistor_matches(
        element: &rspice_core::netlist::Element,
        name: &str,
        nodes: [&str; 2],
    ) -> bool {
        element.provenance == ElementProvenance::Authored
            && element.name.eq_ignore_ascii_case(name)
            && element.nodes.iter().map(String::as_str).eq(nodes)
            && matches!(
                &element.kind,
                ElementKind::Resistor {
                    value,
                    value_expr: None,
                    model: None,
                    instance_params,
                    deferred_params,
                } if value.to_bits() == 50.0f64.to_bits()
                    && instance_params.is_empty()
                    && deferred_params.is_empty()
            )
    }

    fn validate_bug442_topology(netlist: &Netlist) -> Result<(), String> {
        let vin = Self::bug442_element(netlist, "VIN")?;
        let rin = Self::bug442_element(netlist, "RIN")?;
        let tline = Self::bug442_element(netlist, "TLINE")?;
        let rl = Self::bug442_element(netlist, "RL")?;
        let pulse = matches!(
            &vin.kind,
            ElementKind::VoltageSource(SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                pulse_count,
                width_defaults_to_zero: false,
            }) if v1.to_bits() == 0.0f64.to_bits()
                && v2.to_bits() == 5.0f64.to_bits()
                && delay.to_bits() == 0.0f64.to_bits()
                && rise.to_bits() == 0.1e-9f64.to_bits()
                && fall.to_bits() == 0.1e-9f64.to_bits()
                && width.to_bits() == 5.0e-9f64.to_bits()
                && period.to_bits() == 25.0e-9f64.to_bits()
                && pulse_count.to_bits() == 0.0f64.to_bits()
        );
        let line = tline.provenance == ElementProvenance::Authored
            && tline.name.eq_ignore_ascii_case("TLINE")
            && tline
                .nodes
                .iter()
                .map(String::as_str)
                .eq(["2", "0", "3", "0"])
            && matches!(
                &tline.kind,
                ElementKind::TransmissionLine {
                    z0: Some(z0),
                    td: Some(td),
                    freq: None,
                    nl: None,
                    model: None,
                    ..
                } if z0.to_bits() == 50.0f64.to_bits()
                    && td.to_bits() == 10.0e-9f64.to_bits()
            );
        if vin.provenance != ElementProvenance::Authored
            || !vin.nodes.iter().map(String::as_str).eq(["1", "0"])
            || !pulse
            || !Self::bug442_resistor_matches(rin, "RIN", ["1", "2"])
            || !line
            || !Self::bug442_resistor_matches(rl, "RL", ["3", "0"])
        {
            return Err(format!(
                "{LABEL} exact pulse/resistor/lossless-line topology changed: {:?}",
                netlist.elements
            ));
        }
        Ok(())
    }

    fn bug442_restart_options_match(role: Bug442Role, netlist: &Netlist) -> bool {
        let mut remaining = netlist.options.clone();
        let restart = remaining.restart.take();
        if format!("{remaining:?}") != format!("{:?}", SimulationOptions::default()) {
            return false;
        }
        let Some(restart) = restart else {
            return false;
        };
        if restart.print_timeint_options.is_some()
            || restart.start_time.is_some()
            || !restart.intervals.is_empty()
        {
            return false;
        }
        match role {
            Bug442Role::PackedBaseline => {
                restart.pack.is_none()
                    && restart.job.as_deref() == Some("trans_test")
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits)
                        == Some(RESTART_INTERVAL.to_bits())
            }
            Bug442Role::UnpackedBaseline => {
                restart.pack == Some(false)
                    && restart.job.as_deref() == Some("trans_test_unpacked")
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits)
                        == Some(RESTART_INTERVAL.to_bits())
            }
            Bug442Role::PackedRestarted => {
                restart.pack.is_none()
                    && restart.job.is_none()
                    && restart.file.as_deref() == Some("trans_test1e-08")
                    && restart.initial_interval.is_none()
            }
            Bug442Role::UnpackedRestarted => {
                restart.pack.is_none()
                    && restart.job.is_none()
                    && restart.file.as_deref() == Some("trans_test_unpacked1e-08")
                    && restart.initial_interval.is_none()
            }
            Bug442Role::WrapperOwner => false,
        }
    }

    fn bug442_worker_plan(
        &self,
        role: Bug442Role,
        source: &str,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if role == Bug442Role::WrapperOwner {
            return Err(format!("{LABEL} wrapper owner is not a simulation deck"));
        }
        Self::validate_bug442_role_source(role, source)?;
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} does not admit simulator scripting blocks"));
        }
        Self::reject_unsupported_source_directives(source)?;
        let path = self.root.join(role.path());
        let netlist = Self::parse_xyce_netlist(source, &path)
            .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        if netlist.title != "Transmission Line Circuit"
            || netlist.elements.len() != 4
            || netlist.analyses.len() != 1
            || netlist.lin_analysis.is_some()
            || !netlist.fft_analyses.is_empty()
            || netlist.output_requests.len() != 1
            || !netlist.measurements.is_empty()
            || !netlist.models.is_empty()
            || !netlist.subcircuits.is_empty()
            || !netlist.diagnostics.is_empty()
            || !netlist.data_tables.is_empty()
            || !netlist.initial_conditions.is_empty()
            || netlist.device_initial_conditions.is_some()
            || !netlist.node_sets.is_empty()
            || !netlist.global_nodes.is_empty()
            || !netlist.veriloga_includes.is_empty()
            || !netlist.spef_includes.is_empty()
            || !netlist.control_dispositions.is_empty()
            || !netlist.params.all_string_params().is_empty()
            || !netlist.params.all_functions().is_empty()
            || !netlist.params.all_parameter_expressions().is_empty()
            || !netlist.params.all_global_expressions().is_empty()
            || netlist.source_text.as_deref() != Some(source)
            || netlist.source_path.as_deref() != Some(path.as_path())
            || !Self::bug442_restart_options_match(role, &netlist)
        {
            return Err(format!(
                "{LABEL} {role:?} typed envelope changed: analyses={:?}, outputs={:?}, options={:?}",
                netlist.analyses, netlist.output_requests, netlist.options
            ));
        }
        if !matches!(
            netlist.analyses.as_slice(),
            [AnalysisCommand::Tran {
                step,
                stop,
                start: None,
                max_step: None,
                uic: false,
            }] if step.to_bits() == PRINT_STEP.to_bits()
                && stop.to_bits() == FINAL_STOP.to_bits()
        ) {
            return Err(format!(
                "{LABEL} {role:?} .TRAN changed: {:?}",
                netlist.analyses
            ));
        }
        Self::validate_bug442_topology(&netlist)?;
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} output symbols changed: {error}"))?;
        let outputs = Self::print_output_requests(source, "TRAN")?;
        if !matches!(
            outputs.as_slice(),
            [output] if output.format.is_none()
                && output.file.as_deref() == role.print_file()
                && output.probes.iter().map(String::as_str).eq(["V(2)", "V(3)"])
        ) {
            return Err(format!(
                "{LABEL} {role:?} FILE/PRINT contract changed: {outputs:?}"
            ));
        }
        let tran = Self::single_tran_analysis(&netlist)?;
        let plan = XyceStaticTranPlan {
            deck_path: path.clone(),
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: vec!["V(2)".into(), "V(3)".into()],
            }),
            output_override: false,
            timeint_conststep: false,
            tran,
            steps: Vec::new(),
            contract: XyceStaticTranContract::WrapperStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        if plan.deck_path != path
            || plan.tran.step.to_bits() != PRINT_STEP.to_bits()
            || plan.tran.stop.to_bits() != FINAL_STOP.to_bits()
            || plan.tran.start.is_some()
            || plan.tran.max_step.is_some()
            || plan.tran.uic
        {
            return Err(format!(
                "{LABEL} {role:?} manual transient plan changed: {plan:?}"
            ));
        }
        Ok((plan, netlist))
    }

    fn bug442_require_result_horizon(
        label: &str,
        result: &TransientResult,
        start: Value,
        stop: Value,
    ) -> Result<(), String> {
        Self::validate_transient_result_time_grid(result)?;
        if result.time.is_empty()
            || result.time.len() > MAX_RESULT_ROWS
            || result.time.first().map(|value| value.to_bits()) != Some(start.to_bits())
            || result.time.last().map(|value| value.to_bits()) != Some(stop.to_bits())
            || result
                .voltages
                .iter()
                .chain(&result.branch_currents)
                .flat_map(|waveform| waveform.iter())
                .any(|value| !value.is_finite())
        {
            return Err(format!(
                "{LABEL} {label} result horizon/framing changed: rows={}, endpoints={:?}/{:?}",
                result.time.len(),
                result.time.first(),
                result.time.last()
            ));
        }
        Ok(())
    }

    fn bug442_require_tline_history(checkpoint: &TransientCheckpoint) -> Result<(), String> {
        let text = checkpoint.to_text();
        let header = text
            .lines()
            .find(|line| line.starts_with("tline_state "))
            .ok_or_else(|| format!("{LABEL} checkpoint omitted TLINE state"))?;
        let fields = header.split_whitespace().collect::<Vec<_>>();
        let counts = fields
            .get(8..11)
            .ok_or_else(|| format!("{LABEL} malformed TLINE state header: {header:?}"))?;
        let has_history = counts
            .iter()
            .any(|field| field.parse::<usize>().is_ok_and(|count| count > 0));
        if !text.lines().any(|line| line == "tline_state_available 1")
            || !text.lines().any(|line| line == "tline_blockers 0")
            || !text.lines().any(|line| line == "tline_states 1")
            || fields
                .get(1)
                .is_none_or(|name| !name.eq_ignore_ascii_case("TLINE"))
            || !has_history
        {
            return Err(format!(
                "{LABEL} checkpoint did not retain real TLINE history: {header:?}"
            ));
        }
        Ok(())
    }

    fn bug442_round_trip_checkpoint(
        checkpoint: &TransientCheckpoint,
        encoding: TransientCheckpointEncoding,
    ) -> Result<(TransientCheckpoint, Vec<u8>), String> {
        Self::bug442_require_tline_history(checkpoint)?;
        let encoded = checkpoint
            .to_bytes(encoding)
            .map_err(|error| format!("{LABEL} {encoding:?} encoding failed: {error}"))?;
        if encoded.is_empty() || encoded.len() > MAX_CHECKPOINT_BYTES {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint violated byte envelope: {}",
                encoded.len()
            ));
        }
        let restored =
            TransientCheckpoint::from_bytes_with_encoding(&encoded, encoding, MAX_CHECKPOINT_BYTES)
                .map_err(|error| format!("{LABEL} {encoding:?} decoding failed: {error}"))?;
        let wrong_encoding = match encoding {
            TransientCheckpointEncoding::Unpacked => TransientCheckpointEncoding::Packed,
            TransientCheckpointEncoding::Packed => TransientCheckpointEncoding::Unpacked,
        };
        if TransientCheckpoint::from_bytes_with_encoding(
            &encoded,
            wrong_encoding,
            MAX_CHECKPOINT_BYTES,
        )
        .is_ok()
        {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint was accepted by the {wrong_encoding:?} decoder"
            ));
        }
        if restored != *checkpoint {
            return Err(format!("{LABEL} {encoding:?} checkpoint was not bit-exact"));
        }
        Self::bug442_require_tline_history(&restored)?;
        Ok((restored, encoded))
    }

    fn compare_bug442_restart_tables(
        &self,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<Vec<XyceValueMismatch>, String> {
        let expected = ["Index", "TIME", "V(2)", "V(3)"];
        for (label, table) in [("GOOD", good), ("TEST", test)] {
            if table.columns.iter().map(String::as_str).ne(expected)
                || table.rows.is_empty()
                || table.rows.len() > MAX_RESULT_ROWS
                || table.rows.iter().any(|row| {
                    row.len() != expected.len() || row.iter().any(|value| !value.is_finite())
                })
            {
                return Err(format!("{LABEL} {label} PRN framing changed"));
            }
        }
        self.compare_xyce_verify_transient_tables_with_uniform_tolerance(
            good,
            test,
            XyceVerifyTransientTolerance::release_7_10_default(),
            XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
        )
    }

    fn bug442_require_comparison(
        &self,
        label: &str,
        good: &XycePrnTable,
        test: &XycePrnTable,
    ) -> Result<(), String> {
        let mismatches = self.compare_bug442_restart_tables(good, test)?;
        if mismatches.is_empty() {
            Ok(())
        } else {
            Err(format!(
                "{LABEL} {label} produced {} Release-7.10 xyce_verify mismatch(es): {mismatches:?}",
                mismatches.len()
            ))
        }
    }

    fn bug442_run_baseline(
        &self,
        role: Bug442Role,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<(TransientResult, TransientCheckpoint), String> {
        let max_step = Self::transient_family_max_step(netlist, &plan.tran)?;
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(&plan.tran));
        let (result, checkpoints) = engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                netlist,
                FINAL_STOP,
                max_step,
                TransientStartupMode::from_uic(false),
                &SAVE_TIMES,
                abort,
            )
            .map_err(|error| format!("{LABEL} {role:?} baseline failed: {error}"))?;
        Self::bug442_require_result_horizon(&format!("{role:?}"), &result, 0.0, FINAL_STOP)?;
        if checkpoints.len() != SAVE_TIMES.len()
            || checkpoints
                .iter()
                .map(|scheduled| scheduled.nominal_time.to_bits())
                .ne(SAVE_TIMES.map(Value::to_bits))
        {
            return Err(format!("{LABEL} {role:?} checkpoint schedule changed"));
        }
        let checkpoint = checkpoints
            .into_iter()
            .find(|scheduled| scheduled.nominal_time.to_bits() == RESTART_TARGET.to_bits())
            .ok_or_else(|| format!("{LABEL} {role:?} omitted the historical 10 ns checkpoint"))?
            .checkpoint;
        if checkpoint.time.to_bits() != RESTART_TARGET.to_bits() {
            return Err(format!(
                "{LABEL} {role:?} checkpoint captured at {:.17e}, expected exact 10 ns",
                checkpoint.time
            ));
        }
        Self::bug442_require_tline_history(&checkpoint)?;
        Ok((result, checkpoint))
    }

    fn bug442_resume(
        &self,
        role: Bug442Role,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
        checkpoint: &TransientCheckpoint,
        abort: &dyn AbortSignal,
    ) -> Result<TransientResult, String> {
        let max_step = Self::transient_family_max_step(netlist, &plan.tran)?;
        let engine = self
            .create_xyce_static_tran_engine(None, Self::xyce_initial_timestep_for_tran(&plan.tran));
        let (result, final_checkpoint) = engine
            .run_tran_restart_resume_with_abort(netlist, checkpoint, FINAL_STOP, max_step, abort)
            .map_err(|error| format!("{LABEL} {role:?} resume failed: {error}"))?;
        Self::bug442_require_result_horizon(
            &format!("{role:?}"),
            &result,
            RESTART_TARGET,
            FINAL_STOP,
        )?;
        if final_checkpoint.time.to_bits() != FINAL_STOP.to_bits() {
            return Err(format!("{LABEL} {role:?} final checkpoint horizon changed"));
        }
        Ok(result)
    }

    fn bug442_read_worker_plan(
        &self,
        role: Bug442Role,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        let bytes = Self::read_corpus_canonical(LABEL, &self.root.join(role.path()))?;
        let source = String::from_utf8(bytes)
            .map_err(|error| format!("{LABEL} {role:?} source is not UTF-8: {error}"))?;
        self.bug442_worker_plan(role, &source)
    }

    pub(super) fn validate_bug442_oracle(&self, start: Instant) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before planning"));
        }
        let (packed_baseline_plan, packed_baseline_netlist) =
            self.bug442_read_worker_plan(Bug442Role::PackedBaseline)?;
        let (unpacked_baseline_plan, unpacked_baseline_netlist) =
            self.bug442_read_worker_plan(Bug442Role::UnpackedBaseline)?;
        let (packed_restarted_plan, packed_restarted_netlist) =
            self.bug442_read_worker_plan(Bug442Role::PackedRestarted)?;
        let (unpacked_restarted_plan, unpacked_restarted_netlist) =
            self.bug442_read_worker_plan(Bug442Role::UnpackedRestarted)?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} deadline expired before execution"));
        }

        let (packed_baseline, packed_checkpoint) = self.bug442_run_baseline(
            Bug442Role::PackedBaseline,
            &packed_baseline_plan,
            &packed_baseline_netlist,
            &abort,
        )?;
        let (unpacked_baseline, unpacked_checkpoint) = self.bug442_run_baseline(
            Bug442Role::UnpackedBaseline,
            &unpacked_baseline_plan,
            &unpacked_baseline_netlist,
            &abort,
        )?;
        let (packed_checkpoint, packed_bytes) = Self::bug442_round_trip_checkpoint(
            &packed_checkpoint,
            TransientCheckpointEncoding::Packed,
        )?;
        let (unpacked_checkpoint, unpacked_bytes) = Self::bug442_round_trip_checkpoint(
            &unpacked_checkpoint,
            TransientCheckpointEncoding::Unpacked,
        )?;
        if packed_bytes == unpacked_bytes
            || unpacked_bytes.as_slice() != unpacked_checkpoint.to_text().as_bytes()
        {
            return Err(format!(
                "{LABEL} packed and unpacked branches did not use genuinely distinct encodings"
            ));
        }
        let packed_restarted = self.bug442_resume(
            Bug442Role::PackedRestarted,
            &packed_restarted_plan,
            &packed_restarted_netlist,
            &packed_checkpoint,
            &abort,
        )?;
        let unpacked_restarted = self.bug442_resume(
            Bug442Role::UnpackedRestarted,
            &unpacked_restarted_plan,
            &unpacked_restarted_netlist,
            &unpacked_checkpoint,
            &abort,
        )?;

        let packed_baseline_table = Self::transient_family_result_to_prn_table(
            &packed_baseline_plan,
            &packed_baseline_netlist,
            &packed_baseline,
        )?;
        let unpacked_baseline_table = Self::transient_family_result_to_prn_table(
            &unpacked_baseline_plan,
            &unpacked_baseline_netlist,
            &unpacked_baseline,
        )?;
        let packed_restarted_table = Self::transient_family_result_to_prn_table(
            &packed_restarted_plan,
            &packed_restarted_netlist,
            &packed_restarted,
        )?;
        let unpacked_restarted_table = Self::transient_family_result_to_prn_table(
            &unpacked_restarted_plan,
            &unpacked_restarted_netlist,
            &unpacked_restarted,
        )?;
        self.bug442_require_comparison(
            "packed baseline GOOD vs packed restart TEST",
            &packed_baseline_table,
            &packed_restarted_table,
        )?;
        self.bug442_require_comparison(
            "unpacked baseline GOOD vs unpacked restart TEST",
            &unpacked_baseline_table,
            &unpacked_restarted_table,
        )?;
        self.bug442_require_comparison(
            "packed restart GOOD vs unpacked restart TEST",
            &packed_restarted_table,
            &unpacked_restarted_table,
        )?;
        if abort.is_aborted() {
            return Err(format!("{LABEL} execution exceeded its shared deadline"));
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

    fn test_abort() -> DeadlineAbort {
        DeadlineAbort::new(Instant::now(), 30_000)
    }

    #[test]
    fn bug442_roles_paths_and_contracts_are_exact() {
        let records = Bug442Role::ALL
            .map(Bug442Role::record)
            .into_iter()
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), Bug442Role::ALL.len());
        for role in Bug442Role::ALL {
            assert_eq!(Bug442Role::for_record(role.path()), Some(role));
            assert_eq!(Bug442Role::for_record(role.record()), Some(role));
            assert_eq!(
                role.contract(),
                if role == Bug442Role::WrapperOwner {
                    BUG442_OWNER_CONTRACT
                } else {
                    BUG442_WORKER_CONTRACT
                }
            );
        }
        assert_eq!(
            Bug442Role::for_record("Netlists/Certification_Tests/BUG_442/bug_442.CIR"),
            Some(Bug442Role::WrapperOwner),
            "routing may recognize normalized case so provenance can reject its noncanonical path"
        );
        for near_miss in [
            "Netlists/Certification_Tests/BUG_442/bug_442.cir.sh",
            "Netlists/Certification_Tests/BUG_442/invented.cir",
            "Netlists/Certification_Tests/BUG_442x/bug_442.cir",
        ] {
            assert_eq!(
                Bug442Role::for_record(near_miss),
                None,
                "claimed {near_miss}"
            );
        }
    }

    #[test]
    fn bug442_all_worker_plans_are_exact() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        for role in Bug442Role::WORKERS {
            runner
                .bug442_read_worker_plan(role)
                .unwrap_or_else(|error| panic!("{role:?}: {error}"));
        }
    }

    #[test]
    fn bug442_source_semantic_mutations_are_rejected_before_execution() {
        let root = corpus_root();
        let runner = XyceTestRunner::new(&root, XyceRunnerConfig::default());
        for (role, old, new) in [
            (Bug442Role::PackedBaseline, "Z0=50", "Z0=49"),
            (Bug442Role::PackedBaseline, "TD=10N", "TD=11N"),
            (Bug442Role::PackedBaseline, "0.25N 50N", "0.5N 50N"),
            (Bug442Role::PackedBaseline, "V(2) V(3)", "V(3) V(2)"),
            (Bug442Role::UnpackedBaseline, "pack=0", "pack=1"),
            (
                Bug442Role::PackedRestarted,
                "trans_test1e-08",
                "trans_test2e-08",
            ),
        ] {
            let source = String::from_utf8(
                XyceTestRunner::read_corpus_canonical(LABEL, &root.join(role.path())).unwrap(),
            )
            .unwrap();
            let changed = source.replacen(old, new, 1);
            assert_ne!(changed, source, "{role:?} source contains {old:?}");
            assert!(
                runner.bug442_worker_plan(role, &changed).is_err(),
                "{role:?} admitted mutation {old:?}->{new:?}"
            );
        }
    }

    #[test]
    fn bug442_comparator_is_directional_and_rejects_history_loss() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let columns = ["Index", "TIME", "V(2)", "V(3)"]
            .map(str::to_string)
            .to_vec();
        let good = XycePrnTable {
            columns: columns.clone(),
            rows: vec![
                vec![0.0, 10.0e-9, 100.0, 2.5],
                vec![1.0, 20.0e-9, 100.0, 2.5],
            ],
        };
        let near = XycePrnTable {
            columns: columns.clone(),
            rows: vec![vec![0.0, 10.0e-9, 99.0, 2.5], vec![1.0, 20.0e-9, 99.0, 2.5]],
        };
        assert!(
            runner
                .compare_bug442_restart_tables(&good, &near)
                .unwrap()
                .is_empty()
        );
        assert!(
            !runner
                .compare_bug442_restart_tables(&near, &good)
                .unwrap()
                .is_empty()
        );
        let mut lost = near;
        lost.rows[0][3] = 0.0;
        lost.rows[1][3] = 0.0;
        assert!(
            !runner
                .compare_bug442_restart_tables(&good, &lost)
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn bug442_packed_codec_corruption_and_history_loss_fail_closed() {
        let runner = XyceTestRunner::new(corpus_root(), XyceRunnerConfig::default());
        let (plan, netlist) = runner
            .bug442_read_worker_plan(Bug442Role::PackedBaseline)
            .unwrap();
        let (_, checkpoint) = runner
            .bug442_run_baseline(Bug442Role::PackedBaseline, &plan, &netlist, &test_abort())
            .unwrap();
        let (_, mut packed) = XyceTestRunner::bug442_round_trip_checkpoint(
            &checkpoint,
            TransientCheckpointEncoding::Packed,
        )
        .unwrap();
        let last = packed.len() - 1;
        packed[last] ^= 0x01;
        assert!(
            TransientCheckpoint::from_bytes_with_encoding(
                &packed,
                TransientCheckpointEncoding::Packed,
                MAX_CHECKPOINT_BYTES,
            )
            .is_err()
        );
        let history_lost = checkpoint
            .to_text()
            .replace("tline_state_available 1", "tline_state_available 0");
        assert!(TransientCheckpoint::from_text(&history_lost).is_err());
    }

    #[test]
    fn bug442_expired_shared_deadline_fails_before_work() {
        let root = corpus_root();
        let config = XyceRunnerConfig {
            max_time_per_test_ms: 1,
            ..XyceRunnerConfig::default()
        };
        let runner = XyceTestRunner::new(&root, config);
        assert!(
            runner
                .validate_bug442_oracle(Instant::now() - Duration::from_secs(1))
                .is_err()
        );
    }
}
