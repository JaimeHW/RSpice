use super::*;
use rspice_core::engine::{
    SimulationConfigOverrides, TransientCheckpointEncoding, XyceRestartJobPlan,
    resolve_simulation_config, xyce_restart_schedule_is_due,
};

const LABEL: &str = "BUG_456 transient-restart wrapper family";
const FAMILY_DIRECTORY: &str = "Netlists/Certification_Tests/BUG_456";
const FAMILY_PREFIX: &str = "netlists/certification_tests/bug_456/";

pub(super) const BUG456_OWNER_CONTRACT: &str = "bug456_transient_restart_relational_wrapper_owner";
pub(super) const BUG456_WORKER_CONTRACT: &str = "bug456_transient_restart_relational_worker";

const MAX_CHECKPOINT_BYTES: usize = 16 * 1024 * 1024;
// The exact simple.cir adaptive trajectory has just over 200k accepted
// points at Xyce's derived maximum step. Keep a bounded 25% safety margin;
// this is still below the runner's native 250k transient-oracle budget.
const MAX_RESULT_ROWS: usize = 250_000;
const MAX_RESTART_POINTS: usize = 64;
const HISTORICAL_GRID_ZERO_TOLERANCE: Value = 10.0e-15;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug456Case {
    ConverterPacked,
    ConverterUnpacked,
    ConverterOutput,
    EmitterTrapGear,
    EmitterGear,
    PushPullPacked,
    PushPullUnpacked,
    SimpleUnpacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Bug456Member {
    Owner,
    Baseline,
    Restart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct Bug456Role {
    case: Bug456Case,
    member: Bug456Member,
}

impl Bug456Role {
    pub(super) const ALL: [Self; 24] = [
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Restart),
    ];

    #[cfg(test)]
    pub(super) const OWNERS: [Self; 8] = [
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Owner),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Owner),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Owner),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Owner),
    ];

    #[cfg(test)]
    pub(super) const WORKERS: [Self; 16] = [
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterPacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Baseline),
        Self::new(Bug456Case::ConverterOutput, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterTrapGear, Bug456Member::Restart),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Baseline),
        Self::new(Bug456Case::EmitterGear, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullPacked, Bug456Member::Restart),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::PushPullUnpacked, Bug456Member::Restart),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Baseline),
        Self::new(Bug456Case::SimpleUnpacked, Bug456Member::Restart),
    ];

    const fn new(case: Bug456Case, member: Bug456Member) -> Self {
        Self { case, member }
    }

    pub(super) fn for_record(record: &str) -> Option<Self> {
        let normalized = XyceTestRunner::normalize_manifest_key(record);
        Self::ALL
            .into_iter()
            .find(|role| role.record() == normalized)
    }

    pub(super) const fn is_owner(self) -> bool {
        matches!(self.member, Bug456Member::Owner)
    }

    /// Owners enter the authoritative passing census only after their exact
    /// case-local save/resume oracle has completed successfully. This list is
    /// advanced methodically as the family is qualified.
    pub(super) const fn is_qualified_owner(self) -> bool {
        matches!(
            (self.case, self.member),
            (
                Bug456Case::ConverterUnpacked
                    | Bug456Case::ConverterPacked
                    | Bug456Case::ConverterOutput
                    | Bug456Case::EmitterGear
                    | Bug456Case::EmitterTrapGear
                    | Bug456Case::SimpleUnpacked,
                Bug456Member::Owner
            )
        )
    }

    pub(super) const fn contract(self) -> &'static str {
        if self.is_owner() {
            BUG456_OWNER_CONTRACT
        } else {
            BUG456_WORKER_CONTRACT
        }
    }

    const fn file_name(self) -> &'static str {
        match (self.case, self.member) {
            (Bug456Case::ConverterPacked, Bug456Member::Owner) => "bug456.cir",
            (Bug456Case::ConverterPacked, Bug456Member::Baseline) => "converter_baseline.cir",
            (Bug456Case::ConverterPacked, Bug456Member::Restart) => "converter_restart.cir",
            (Bug456Case::ConverterUnpacked, Bug456Member::Owner) => "bug456_unpacked.cir",
            (Bug456Case::ConverterUnpacked, Bug456Member::Baseline) => {
                "converter_baseline_unpacked.cir"
            }
            (Bug456Case::ConverterUnpacked, Bug456Member::Restart) => {
                "converter_restart_unpacked.cir"
            }
            (Bug456Case::ConverterOutput, Bug456Member::Owner) => "bug456output.cir",
            (Bug456Case::ConverterOutput, Bug456Member::Baseline) => {
                "converter_baseline_output.cir"
            }
            (Bug456Case::ConverterOutput, Bug456Member::Restart) => "converter_restart_output.cir",
            (Bug456Case::EmitterTrapGear, Bug456Member::Owner) => "bug456emit.cir",
            (Bug456Case::EmitterTrapGear, Bug456Member::Baseline) => "emitter_baseline.cir",
            (Bug456Case::EmitterTrapGear, Bug456Member::Restart) => "emitter_restart.cir",
            (Bug456Case::EmitterGear, Bug456Member::Owner) => "bug456emit_gear.cir",
            (Bug456Case::EmitterGear, Bug456Member::Baseline) => "emitter_baseline_gear.cir",
            (Bug456Case::EmitterGear, Bug456Member::Restart) => "emitter_restart_gear.cir",
            (Bug456Case::PushPullPacked, Bug456Member::Owner) => "bug456pp.cir",
            (Bug456Case::PushPullPacked, Bug456Member::Baseline) => "push_pull_xyce_baseline.cir",
            (Bug456Case::PushPullPacked, Bug456Member::Restart) => "push_pull_xyce_restart.cir",
            (Bug456Case::PushPullUnpacked, Bug456Member::Owner) => "bug456pp_unpacked.cir",
            (Bug456Case::PushPullUnpacked, Bug456Member::Baseline) => {
                "push_pull_xyce_baseline_unpacked.cir"
            }
            (Bug456Case::PushPullUnpacked, Bug456Member::Restart) => {
                "push_pull_xyce_restart_unpacked.cir"
            }
            (Bug456Case::SimpleUnpacked, Bug456Member::Owner) => "simple.cir",
            (Bug456Case::SimpleUnpacked, Bug456Member::Baseline) => "simple_baseline.cir",
            (Bug456Case::SimpleUnpacked, Bug456Member::Restart) => "simple_restart.cir",
        }
    }

    pub(super) fn path(self) -> String {
        format!("{FAMILY_DIRECTORY}/{}", self.file_name())
    }

    pub(super) fn record(self) -> String {
        format!("{FAMILY_PREFIX}{}", self.file_name().to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, Copy)]
struct Bug456CaseSpec {
    case: Bug456Case,
    job: &'static str,
    packed: bool,
    stop: Value,
    seam: Value,
    print_step: Value,
    probes: &'static [&'static str],
    historical_grid_threshold: Option<Value>,
}

impl Bug456CaseSpec {
    const fn for_case(case: Bug456Case) -> Self {
        match case {
            Bug456Case::ConverterPacked => Self {
                case,
                job: "converter",
                packed: true,
                stop: 4.0e-4,
                seam: 2.0e-4,
                print_step: 0.0,
                probes: &["V(N8)"],
                historical_grid_threshold: Some(2.0e-4),
            },
            Bug456Case::ConverterUnpacked => Self {
                case,
                job: "converter_unpacked",
                packed: false,
                stop: 4.0e-4,
                seam: 2.0e-4,
                print_step: 0.0,
                probes: &["V(N8)"],
                historical_grid_threshold: None,
            },
            Bug456Case::ConverterOutput => Self {
                case,
                job: "converter_output",
                packed: true,
                stop: 4.0e-4,
                seam: 2.0e-4,
                print_step: 0.0,
                probes: &["V(N8)"],
                historical_grid_threshold: None,
            },
            Bug456Case::EmitterTrapGear => Self {
                case,
                job: "emitter",
                packed: true,
                stop: 5.0e-5,
                seam: 2.0e-5,
                print_step: 0.0,
                probes: &["V(1)", "V(Ve)"],
                historical_grid_threshold: Some(2.0e-5),
            },
            Bug456Case::EmitterGear => Self {
                case,
                job: "emittergear",
                packed: true,
                stop: 5.0e-5,
                seam: 2.0e-5,
                print_step: 0.0,
                probes: &["V(1)", "V(Ve)"],
                historical_grid_threshold: Some(2.0e-5),
            },
            Bug456Case::PushPullPacked => Self {
                case,
                job: "push_pull",
                packed: true,
                stop: 1.0e-3,
                seam: 5.0e-4,
                print_step: 1.0e-6,
                probes: &["V(HV)"],
                historical_grid_threshold: None,
            },
            Bug456Case::PushPullUnpacked => Self {
                case,
                job: "push_pull_unpacked",
                packed: false,
                stop: 1.0e-3,
                seam: 5.0e-4,
                print_step: 1.0e-6,
                probes: &["V(HV)"],
                historical_grid_threshold: None,
            },
            Bug456Case::SimpleUnpacked => Self {
                case,
                job: "restart",
                packed: false,
                stop: 1.0e-5,
                seam: 2.0e-6,
                print_step: 0.0,
                probes: &["V(1)", "V(2)", "V(3)"],
                // This is the historical wrapper's literal threshold. It is
                // above TSTOP and therefore intentionally leaves the extra
                // sequential grid loop vacuous after xyce_verify succeeds.
                historical_grid_threshold: Some(2.0e-4),
            },
        }
    }

    fn role(self, member: Bug456Member) -> Bug456Role {
        Bug456Role::new(self.case, member)
    }

    fn expected_schedule(self) -> Vec<Value> {
        let mut times = vec![0.0];
        let mut next = self.seam;
        while next <= self.stop {
            times.push(next);
            next += self.seam;
        }
        times
    }
}

fn bug456_checkpoint_is_in_accepted_step_window(
    checkpoint_time: Value,
    seam_time: Value,
    stop_time: Value,
) -> bool {
    xyce_restart_schedule_is_due(checkpoint_time, seam_time) && checkpoint_time < stop_time
}

impl XyceTestRunner {
    fn read_bug456_family(&self) -> Result<BTreeMap<String, Vec<u8>>, String> {
        let mut retained = BTreeMap::new();
        for role in Bug456Role::ALL {
            let name = role.file_name();
            let source = Self::read_corpus_canonical(
                &format!("{LABEL} {name}"),
                &self.root.join(role.path()),
            )?;
            retained.insert(name.to_ascii_lowercase(), source);
        }
        Ok(retained)
    }

    fn bug456_worker_plan(
        &self,
        role: Bug456Role,
        retained: &BTreeMap<String, Vec<u8>>,
    ) -> Result<(XyceStaticTranPlan, Netlist), String> {
        if role.is_owner() {
            return Err(format!("{LABEL} wrapper owner is not a simulation deck"));
        }
        let spec = Bug456CaseSpec::for_case(role.case);
        let source_bytes = retained
            .get(&role.file_name().to_ascii_lowercase())
            .ok_or_else(|| format!("{LABEL} lost {role:?} source"))?;
        let source = std::str::from_utf8(source_bytes)
            .map_err(|error| format!("{LABEL} {role:?} source is not UTF-8: {error}"))?;
        if Self::contains_control_block(source) {
            return Err(format!("{LABEL} does not admit simulator scripting blocks"));
        }
        Self::reject_unsupported_source_directives(source)?;
        let path = self.root.join(role.path());
        let netlist = Self::parse_xyce_netlist(source, &path)
            .map_err(|error| format!("{LABEL} {role:?} parse failed: {error}"))?;
        validate_output_symbols(&netlist)
            .map_err(|error| format!("{LABEL} {role:?} output symbols changed: {error}"))?;
        let tran = Self::single_tran_analysis(&netlist)?;
        if tran.step.to_bits() != spec.print_step.to_bits()
            || tran.stop.to_bits() != spec.stop.to_bits()
            || tran.start.is_some()
            || tran.max_step.is_some()
            || tran.uic
        {
            return Err(format!("{LABEL} {role:?} .TRAN changed: {tran:?}"));
        }
        let outputs = Self::print_output_requests(source, "TRAN")?;
        if !matches!(
            outputs.as_slice(),
            [output] if output.format.is_none()
                && output.file.is_none()
                && output.probes.len() == spec.probes.len()
                && output.probes.iter().zip(spec.probes).all(|(actual, expected)|
                    actual.eq_ignore_ascii_case(expected))
        ) {
            return Err(format!("{LABEL} {role:?} .PRINT changed: {outputs:?}"));
        }
        let restart = netlist
            .options
            .restart
            .as_ref()
            .ok_or_else(|| format!("{LABEL} {role:?} omitted typed restart options"))?;
        if restart.start_time.is_some()
            || restart.print_timeint_options.is_some()
            || !restart.intervals.is_empty()
        {
            return Err(format!(
                "{LABEL} {role:?} restart envelope changed: {restart:?}"
            ));
        }
        let restart_matches = match role.member {
            Bug456Member::Baseline => {
                restart.job.as_deref() == Some(spec.job)
                    && restart.file.is_none()
                    && restart.initial_interval.map(Value::to_bits) == Some(spec.seam.to_bits())
                    && restart.pack.unwrap_or(true) == spec.packed
            }
            Bug456Member::Restart => {
                restart.job.is_none()
                    && restart.initial_interval.is_none()
                    && restart.pack.is_none()
                    && restart.file.as_deref()
                        == XyceRestartJobPlan::new(
                            spec.job,
                            spec.seam,
                            &[],
                            spec.stop,
                            Some(spec.packed),
                            MAX_RESTART_POINTS,
                        )
                        .map_err(|error| format!("{LABEL} restart plan failed: {error}"))?
                        .logical_name(spec.seam)
                        .as_deref()
            }
            Bug456Member::Owner => false,
        };
        if !restart_matches {
            return Err(format!(
                "{LABEL} {role:?} restart options changed: {restart:?}"
            ));
        }
        let plan = XyceStaticTranPlan {
            deck_path: path,
            oracle: XyceStaticTranOracle::None,
            source: source.to_string(),
            print: Some(XycePrintRequest {
                probes: spec
                    .probes
                    .iter()
                    .map(|probe| (*probe).to_string())
                    .collect(),
            }),
            output_override: false,
            timeint_conststep: false,
            tran,
            steps: Vec::new(),
            contract: XyceStaticTranContract::PlainStatic,
            wrapper_tolerance: None,
            comparison_mode: XyceStaticTranComparisonMode::Pointwise,
        };
        Ok((plan, netlist))
    }

    fn bug456_resolved_config(
        &self,
        plan: &XyceStaticTranPlan,
        netlist: &Netlist,
    ) -> SimulationConfig {
        let mut base = self.xyce_engine_config(None);
        base.transient_initial_timestep = Self::xyce_initial_timestep_for_tran(&plan.tran);
        resolve_simulation_config(
            &base,
            Some(&netlist.options),
            &SimulationConfigOverrides::default(),
        )
    }

    fn bug456_require_result_horizon(
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

    fn bug456_round_trip_checkpoint(
        checkpoint: &TransientCheckpoint,
        encoding: TransientCheckpointEncoding,
    ) -> Result<TransientCheckpoint, String> {
        let encoded = checkpoint
            .to_bytes(encoding)
            .map_err(|error| format!("{LABEL} {encoding:?} encoding failed: {error}"))?;
        if encoded.is_empty() || encoded.len() > MAX_CHECKPOINT_BYTES {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint violated byte envelope: {}",
                encoded.len()
            ));
        }
        if encoding == TransientCheckpointEncoding::Unpacked
            && encoded.as_slice() != checkpoint.to_text().as_bytes()
        {
            return Err(format!(
                "{LABEL} unpacked checkpoint was not the canonical text encoding"
            ));
        }
        let restored =
            TransientCheckpoint::from_bytes_with_encoding(&encoded, encoding, MAX_CHECKPOINT_BYTES)
                .map_err(|error| format!("{LABEL} {encoding:?} decoding failed: {error}"))?;
        let wrong = match encoding {
            TransientCheckpointEncoding::Packed => TransientCheckpointEncoding::Unpacked,
            TransientCheckpointEncoding::Unpacked => TransientCheckpointEncoding::Packed,
        };
        if TransientCheckpoint::from_bytes_with_encoding(&encoded, wrong, MAX_CHECKPOINT_BYTES)
            .is_ok()
        {
            return Err(format!(
                "{LABEL} {encoding:?} checkpoint was accepted by the {wrong:?} decoder"
            ));
        }
        if restored != *checkpoint {
            return Err(format!("{LABEL} {encoding:?} checkpoint was not bit-exact"));
        }
        Ok(restored)
    }

    fn bug456_require_historical_grid(
        good: &XycePrnTable,
        test: &XycePrnTable,
        threshold: Value,
    ) -> Result<(), String> {
        let mut test_index = 0usize;
        for good_row in &good.rows {
            let good_time = Self::xyce_prn_scientific_roundtrip(
                *good_row
                    .get(1)
                    .ok_or_else(|| format!("{LABEL} GOOD PRN row omitted TIME"))?,
                XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
            )?;
            if good_time >= threshold - HISTORICAL_GRID_ZERO_TOLERANCE {
                let test_row = test.rows.get(test_index).ok_or_else(|| {
                    format!("{LABEL} historical sequential grid exhausted TEST at row {test_index}")
                })?;
                let test_time = Self::xyce_prn_scientific_roundtrip(
                    *test_row
                        .get(1)
                        .ok_or_else(|| format!("{LABEL} TEST PRN row omitted TIME"))?,
                    XYCE_DEFAULT_PRN_SCIENTIFIC_PRECISION,
                )?;
                if !(good_time >= test_time - HISTORICAL_GRID_ZERO_TOLERANCE
                    && good_time < test_time + HISTORICAL_GRID_ZERO_TOLERANCE)
                {
                    return Err(format!(
                        "{LABEL} historical sequential grid differs at TEST row {test_index}: GOOD={good_time:.17e}, TEST={test_time:.17e}"
                    ));
                }
                test_index += 1;
            }
        }
        Ok(())
    }

    fn validate_bug456_case(
        &self,
        spec: Bug456CaseSpec,
        retained: &BTreeMap<String, Vec<u8>>,
        abort: &dyn AbortSignal,
    ) -> Result<(), String> {
        let baseline_role = spec.role(Bug456Member::Baseline);
        let restart_role = spec.role(Bug456Member::Restart);
        let (baseline_plan, baseline_netlist) = self.bug456_worker_plan(baseline_role, retained)?;
        let (restart_plan, restart_netlist) = self.bug456_worker_plan(restart_role, retained)?;
        let baseline_config = self.bug456_resolved_config(&baseline_plan, &baseline_netlist);
        let restart_config = self.bug456_resolved_config(&restart_plan, &restart_netlist);
        if format!("{baseline_config:#?}") != format!("{restart_config:#?}") {
            return Err(format!(
                "{LABEL} {:?} save/resume resolved configurations differ: baseline={baseline_config:#?}, restart={restart_config:#?}",
                spec.case
            ));
        }

        let restart_options = baseline_netlist
            .options
            .restart
            .as_ref()
            .expect("typed baseline restart options were validated");
        let job_plan = XyceRestartJobPlan::new(
            restart_options
                .job
                .as_deref()
                .expect("typed baseline restart job was validated"),
            restart_options
                .initial_interval
                .expect("typed baseline restart interval was validated"),
            &restart_options.intervals,
            spec.stop,
            restart_options.pack,
            MAX_RESTART_POINTS,
        )
        .map_err(|error| format!("{LABEL} {:?} restart plan failed: {error}", spec.case))?;
        let expected_schedule = spec.expected_schedule();
        if job_plan
            .nominal_times()
            .iter()
            .copied()
            .map(Value::to_bits)
            .ne(expected_schedule.iter().copied().map(Value::to_bits))
            || job_plan.encoding()
                != if spec.packed {
                    TransientCheckpointEncoding::Packed
                } else {
                    TransientCheckpointEncoding::Unpacked
                }
        {
            return Err(format!(
                "{LABEL} {:?} restart schedule/encoding changed: {:?}/{:?}",
                spec.case,
                job_plan.nominal_times(),
                job_plan.encoding()
            ));
        }
        let restart_file = restart_netlist
            .options
            .restart
            .as_ref()
            .and_then(|restart| restart.file.as_deref())
            .ok_or_else(|| format!("{LABEL} {:?} restart deck lost FILE", spec.case))?;

        let baseline_max_step =
            Self::transient_family_max_step(&baseline_netlist, &baseline_plan.tran)?;
        let baseline_engine = Engine::new(baseline_config);
        let (baseline_result, checkpoints) = baseline_engine
            .run_tran_checkpoint_schedule_with_startup_mode_and_abort(
                &baseline_netlist,
                spec.stop,
                baseline_max_step,
                TransientStartupMode::from_uic(false),
                job_plan.nominal_times(),
                abort,
            )
            .map_err(|error| format!("{LABEL} {:?} baseline failed: {error}", spec.case))?;
        Self::bug456_require_result_horizon(
            &format!("{:?} baseline", spec.case),
            &baseline_result,
            0.0,
            spec.stop,
        )?;
        if checkpoints.len() != job_plan.nominal_times().len()
            || checkpoints
                .iter()
                .map(|scheduled| scheduled.nominal_time.to_bits())
                .ne(job_plan.nominal_times().iter().copied().map(Value::to_bits))
        {
            return Err(format!(
                "{LABEL} {:?} checkpoint schedule changed",
                spec.case
            ));
        }

        let mut seam_checkpoint = None;
        for scheduled in checkpoints {
            if abort.is_aborted() {
                return Err(format!(
                    "{LABEL} {:?} checkpoint validation aborted",
                    spec.case
                ));
            }
            let restored =
                Self::bug456_round_trip_checkpoint(&scheduled.checkpoint, job_plan.encoding())?;
            if job_plan.logical_name(scheduled.nominal_time).as_deref() == Some(restart_file)
                && seam_checkpoint.replace(restored).is_some()
            {
                return Err(format!(
                    "{LABEL} {:?} restart FILE selected multiple checkpoints",
                    spec.case
                ));
            }
        }
        let seam_checkpoint = seam_checkpoint.ok_or_else(|| {
            format!(
                "{LABEL} {:?} restart FILE {restart_file:?} did not select a scheduled checkpoint",
                spec.case
            )
        })?;
        if !bug456_checkpoint_is_in_accepted_step_window(seam_checkpoint.time, spec.seam, spec.stop)
        {
            return Err(format!(
                "{LABEL} {:?} seam checkpoint captured outside its accepted-step window at {:.17e}",
                spec.case, seam_checkpoint.time
            ));
        }

        let restart_max_step =
            Self::transient_family_max_step(&restart_netlist, &restart_plan.tran)?;
        if baseline_max_step.to_bits() != restart_max_step.to_bits() {
            return Err(format!(
                "{LABEL} {:?} save/resume max steps differ",
                spec.case
            ));
        }
        let restart_engine = Engine::new(restart_config);
        let (restart_result, final_checkpoint) = restart_engine
            .run_tran_restart_resume_with_abort(
                &restart_netlist,
                &seam_checkpoint,
                spec.stop,
                restart_max_step,
                abort,
            )
            .map_err(|error| format!("{LABEL} {:?} restart failed: {error}", spec.case))?;
        let final_checkpoint = final_checkpoint
            .into_checkpoint()
            .map_err(|error| format!("{LABEL} {:?} restart ended: {error}", spec.case))?;
        Self::bug456_require_result_horizon(
            &format!("{:?} restart", spec.case),
            &restart_result,
            seam_checkpoint.time,
            spec.stop,
        )?;
        if final_checkpoint.time.to_bits() != spec.stop.to_bits() {
            return Err(format!(
                "{LABEL} {:?} final checkpoint stopped at {:.17e}",
                spec.case, final_checkpoint.time
            ));
        }

        let good = Self::transient_family_result_to_prn_table(
            &baseline_plan,
            &baseline_netlist,
            &baseline_result,
        )?;
        let test = Self::transient_family_result_to_prn_table(
            &restart_plan,
            &restart_netlist,
            &restart_result,
        )?;
        if good.rows.is_empty()
            || test.rows.is_empty()
            || good.rows.len() > MAX_RESULT_ROWS
            || test.rows.len() > MAX_RESULT_ROWS
        {
            return Err(format!("{LABEL} {:?} PRN framing changed", spec.case));
        }
        let mismatches =
            self.compare_xyce_verify_transient_tables_with_abort(&good, &test, abort)?;
        if !mismatches.is_empty() {
            return Err(format!(
                "{LABEL} {:?} produced {} Release-7.10 xyce_verify mismatch(es): {mismatches:?}",
                spec.case,
                mismatches.len()
            ));
        }
        if let Some(threshold) = spec.historical_grid_threshold {
            Self::bug456_require_historical_grid(&good, &test, threshold)?;
        }
        Ok(())
    }

    pub(super) fn validate_bug456_oracle(
        &self,
        role: Bug456Role,
        start: Instant,
    ) -> Result<(), String> {
        let abort = DeadlineAbort::new(start, self.config.max_time_per_test_ms.max(1));
        let retained = self.read_bug456_family()?;
        self.validate_bug456_case(Bug456CaseSpec::for_case(role.case), &retained, &abort)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug456_role_map_is_exact_and_partitioned() {
        let all = Bug456Role::ALL.into_iter().collect::<BTreeSet<_>>();
        let owners = Bug456Role::OWNERS.into_iter().collect::<BTreeSet<_>>();
        let workers = Bug456Role::WORKERS.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(all.len(), 24);
        assert_eq!(owners.len(), 8);
        assert_eq!(workers.len(), 16);
        assert!(owners.is_disjoint(&workers));
        assert_eq!(
            owners.union(&workers).copied().collect::<BTreeSet<_>>(),
            all
        );
        let records = Bug456Role::ALL
            .map(Bug456Role::record)
            .into_iter()
            .collect::<BTreeSet<_>>();
        assert_eq!(records.len(), 24);
        for role in Bug456Role::ALL {
            assert_eq!(Bug456Role::for_record(&role.path()), Some(role));
            assert_eq!(Bug456Role::for_record(&role.record()), Some(role));
        }
        assert_eq!(
            Bug456Role::for_record("Netlists/Certification_Tests/BUG_456/invented.cir"),
            None
        );
    }

    #[test]
    fn bug456_restart_plans_derive_exact_historical_schedules() {
        for owner in Bug456Role::OWNERS {
            let spec = Bug456CaseSpec::for_case(owner.case);
            let plan = XyceRestartJobPlan::new(
                spec.job,
                spec.seam,
                &[],
                spec.stop,
                Some(spec.packed),
                MAX_RESTART_POINTS,
            )
            .expect("construct bounded historical restart plan");
            assert!(
                plan.nominal_times()
                    .iter()
                    .copied()
                    .map(Value::to_bits)
                    .eq(spec.expected_schedule().into_iter().map(Value::to_bits))
            );
            assert_eq!(
                plan.logical_name(spec.seam).as_deref(),
                Some(match spec.case {
                    Bug456Case::ConverterPacked => "converter0.0002",
                    Bug456Case::ConverterUnpacked => "converter_unpacked0.0002",
                    Bug456Case::ConverterOutput => "converter_output0.0002",
                    Bug456Case::EmitterTrapGear => "emitter2e-05",
                    Bug456Case::EmitterGear => "emittergear2e-05",
                    Bug456Case::PushPullPacked => "push_pull0.0005",
                    Bug456Case::PushPullUnpacked => "push_pull_unpacked0.0005",
                    Bug456Case::SimpleUnpacked => "restart2e-06",
                })
            );
        }
    }

    #[test]
    fn bug456_historical_simple_grid_clause_remains_vacuous() {
        let columns = vec!["Index".into(), "TIME".into(), "V(1)".into()];
        let good = XycePrnTable {
            columns: columns.clone(),
            rows: vec![vec![0.0, 0.0, 1.0], vec![1.0, 1.0e-5, 1.0]],
        };
        let test = XycePrnTable {
            columns,
            rows: Vec::new(),
        };
        XyceTestRunner::bug456_require_historical_grid(&good, &test, 2.0e-4)
            .expect("the historical simple.cir threshold is intentionally above TSTOP");
    }

    #[test]
    fn bug456_checkpoint_window_accepts_xyce_early_schedule_match() {
        let seam = 2.0e-4;
        let stop = 4.0e-4;

        assert!(bug456_checkpoint_is_in_accepted_step_window(
            seam - 3.0e-15,
            seam,
            stop
        ));
        assert!(!bug456_checkpoint_is_in_accepted_step_window(
            seam - 5.0e-15,
            seam,
            stop
        ));
        assert!(!bug456_checkpoint_is_in_accepted_step_window(
            stop, seam, stop
        ));
    }
}
