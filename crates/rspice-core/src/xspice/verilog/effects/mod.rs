//! What a digital design's system tasks do (IEEE 1364-2005 section 17), as
//! the host performs them (design P2 section 4.3).
//!
//! # Accepted-only output
//!
//! A system task never writes anywhere directly. It appends an
//! [`HdlOutputRecord`] to the host's journal, and the journal lives inside
//! [`super::host::DigitalHost`], which a mixed trial clones when it opens and
//! restores when the solver rejects it. So a record produced by a rejected
//! trial disappears with the trial, and the coordinator releases the journal
//! to the run's sink only when a trial commits. Randomness, the monitor and
//! the finish request roll back the same way, for the same reason.
//!
//! # The monitor region
//!
//! `$strobe` and `$monitor` print at the end of the time step (sections
//! 17.1.2 and 17.1.3), from argument functions the compiler built for them.
//! Both join one queue, in the order they were scheduled within the step:
//! a `$strobe` when it runs, the active `$monitor` when it is set up, turned
//! back on, or when the first change of one of its operands happens. That is
//! the order Icarus Verilog prints them in, and a deterministic one.

pub(crate) mod checkpoint;
pub(crate) mod format;
pub(crate) mod plusargs;
pub(crate) mod random;

use std::sync::Arc;

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, DigitalRadix, DigitalSystemArgument, DigitalSystemCall,
    DigitalSystemCallId, DigitalSystemCallKind, DigitalSystemLane, DigitalValueClass,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalClock, DigitalEvalError, DigitalFunctionOutcome, DigitalScalar, DigitalTaskControl,
    DigitalTaskOutcome,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalProcessId;
use rspice_veriloga::four_state::FourStateBit;

pub use crate::config::{HdlOrigin, HdlOutputRecord, HdlPoint, HdlSeverity, HdlStream};
use format::{FormatArgument, FormatContext, TimeFormat};

/// The run options the effects read, fixed for the host's life.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct HdlEffectsConfig {
    pub plusargs: Vec<String>,
    pub seed: u32,
}

/// What ended a run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum HdlFinishKind {
    Finish,
    Stop,
    Fatal,
}

/// A `$finish`, `$stop` or `$fatal` the design executed.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub(crate) struct HdlFinishRequest {
    pub kind: HdlFinishKind,
    pub level: u8,
    pub site: u32,
    pub tick: u64,
    pub seconds: f64,
}

/// One entry of the monitor region's queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) enum MonitorTask {
    /// A `$strobe`'s argument function.
    Strobe(u32),
    /// The active `$monitor`.
    Monitor,
}

/// The active `$monitor` (section 17.1.3: only one at a time).
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct ActiveMonitor {
    pub observer: u32,
    pub enabled: bool,
}

/// Severity task counts, for a front end's exit status.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct SeverityCounts {
    pub warnings: u32,
    pub errors: u32,
    pub fatals: u32,
}

/// Where a system call is being performed from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CallOrigin {
    /// A process reached the call.
    Process(DigitalProcessId),
    /// The monitor region is evaluating a `$strobe`/`$monitor`'s arguments.
    Observer,
}

/// The effects half of a running digital design.
#[derive(Debug, Clone)]
pub(crate) struct HdlEffects {
    config: Arc<HdlEffectsConfig>,
    /// Records not yet released to a sink.
    journal: Vec<HdlOutputRecord>,
    /// The design-global `$random` seed (section 17.9.1).
    random_seed: i32,
    /// Each process's `$urandom` stream, created at its first draw.
    urandom: Vec<Option<random::Pcg32>>,
    time_format: TimeFormat,
    monitor: Option<ActiveMonitor>,
    queue: Vec<MonitorTask>,
    monitor_queued: bool,
    finish: Option<HdlFinishRequest>,
    severities: SeverityCounts,
    design_precision: i8,
}

impl HdlEffects {
    pub(crate) fn new(plan: &CanonicalDigitalPlan) -> Self {
        Self::with_config(plan, Arc::new(HdlEffectsConfig::default()))
    }

    pub(crate) fn with_config(plan: &CanonicalDigitalPlan, config: Arc<HdlEffectsConfig>) -> Self {
        let design_precision = plan.timing.precision_exponent;
        Self {
            random_seed: config.seed as i32,
            config,
            journal: Vec::new(),
            urandom: vec![None; plan.processes.len()],
            time_format: TimeFormat::default_for(design_precision),
            monitor: None,
            queue: Vec::new(),
            monitor_queued: false,
            finish: None,
            severities: SeverityCounts::default(),
            design_precision,
        }
    }

    /// Install the run's options before anything has run.
    pub(crate) fn configure(&mut self, config: HdlEffectsConfig) {
        self.random_seed = config.seed as i32;
        self.config = Arc::new(config);
    }

    pub(crate) fn config(&self) -> &Arc<HdlEffectsConfig> {
        &self.config
    }

    /// The finish request the design made, if it made one.
    pub(crate) fn finish(&self) -> Option<&HdlFinishRequest> {
        self.finish.as_ref()
    }

    pub(crate) fn severities(&self) -> SeverityCounts {
        self.severities
    }

    /// Take every journaled record; called when a trial commits.
    pub(crate) fn drain_journal(&mut self) -> Vec<HdlOutputRecord> {
        std::mem::take(&mut self.journal)
    }

    pub(crate) fn has_journal(&self) -> bool {
        !self.journal.is_empty()
    }

    pub(crate) fn monitor_region_pending(&self) -> bool {
        !self.queue.is_empty()
    }

    /// Take the monitor region's queue in scheduling order.
    pub(crate) fn take_monitor_region(&mut self) -> Vec<MonitorTask> {
        self.monitor_queued = false;
        std::mem::take(&mut self.queue)
    }

    /// The active `$monitor`'s argument function, if it is enabled.
    pub(crate) fn enabled_monitor(&self) -> Option<u32> {
        self.monitor
            .as_ref()
            .filter(|monitor| monitor.enabled)
            .map(|monitor| monitor.observer)
    }

    /// One of the active `$monitor`'s operands changed: it prints at the end
    /// of this time step.
    pub(crate) fn monitor_operand_changed(&mut self) {
        self.queue_monitor();
    }

    fn queue_monitor(&mut self) {
        if !self.monitor_queued && self.enabled_monitor().is_some() {
            self.monitor_queued = true;
            self.queue.push(MonitorTask::Monitor);
        }
    }

    fn origin(plan: &CanonicalDigitalPlan, site: &DigitalSystemCall) -> HdlOrigin {
        let scope = plan
            .scopes
            .get(site.scope.index())
            .map_or_else(String::new, |scope| scope.path.to_string());
        match &site.origin {
            Some(line) => HdlOrigin {
                scope,
                file: line.file.to_string(),
                line: line.line,
            },
            None => HdlOrigin {
                scope,
                file: String::new(),
                line: 0,
            },
        }
    }

    fn record(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        clock: Option<DigitalClock>,
        stream: HdlStream,
        text: String,
    ) {
        let (tick, seconds) = clock.map_or((0, 0.0), |clock| (clock.tick, clock.absolute_seconds));
        self.journal.push(HdlOutputRecord {
            point: HdlPoint::Transient { time: seconds },
            tick,
            seconds,
            stream,
            text,
            origin: Self::origin(plan, site),
        });
    }

    /// Format a display-family call's arguments, starting at `first`.
    fn format(
        &self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        first: usize,
        radix: DigitalRadix,
    ) -> Result<String, DigitalEvalError> {
        let mut arguments = Vec::with_capacity(site.arguments.len());
        for argument in &site.arguments[first.min(site.arguments.len())..] {
            arguments.push(match argument {
                DigitalSystemArgument::Empty => FormatArgument::Empty,
                DigitalSystemArgument::String(text) => FormatArgument::Literal(text.as_str()),
                DigitalSystemArgument::Value { operand, class } => {
                    let value = args.get(*operand as usize).ok_or_else(|| {
                        DigitalEvalError::SystemCallFailed {
                            site: site.id,
                            detail: "an operand the site names is missing".into(),
                        }
                    })?;
                    let signed = matches!(class, DigitalValueClass::FourState { signed: true, .. });
                    FormatArgument::Value(value, signed)
                }
                // A display-family call has no other argument kinds.
                _ => FormatArgument::Empty,
            });
        }
        let scope = plan.scopes.get(site.scope.index());
        Ok(format::format(
            &arguments,
            &FormatContext {
                radix,
                scope: scope.map_or("", |scope| scope.path.as_str()),
                module: scope.map_or("", |scope| scope.module.as_str()),
                time_unit: site.time_scale.unit_exponent(),
                time_format: &self.time_format,
            },
        ))
    }

    /// Refuse a kind whose host behaviour has not landed (file and waveform
    /// I/O is P2 lane L5's). The text names the feature, never the lane.
    fn refuse_unlanded(site: &DigitalSystemCall) -> Result<(), DigitalEvalError> {
        if site.kind.lane() == DigitalSystemLane::Files {
            return Err(DigitalEvalError::SystemCallUnavailable {
                site: site.id,
                detail: format!(
                    "HDL file and waveform I/O (`{}`) is not supported yet",
                    site.kind.name()
                ),
            });
        }
        Ok(())
    }

    /// Perform a system task enable.
    pub(crate) fn system_task(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: DigitalSystemCallId,
        args: &[DigitalScalar],
        clock: Option<DigitalClock>,
        origin: CallOrigin,
    ) -> Result<DigitalTaskOutcome, DigitalEvalError> {
        let site =
            plan.system_calls
                .get(site.index())
                .ok_or(DigitalEvalError::SystemCallUnavailable {
                    site,
                    detail: "the plan declares no such site".into(),
                })?;
        Self::refuse_unlanded(site)?;
        let mut outcome = DigitalTaskOutcome::CONTINUE;
        match site.kind {
            DigitalSystemCallKind::Display(radix) | DigitalSystemCallKind::Write(radix) => {
                let mut text = self.format(plan, site, args, 0, radix)?;
                if matches!(site.kind, DigitalSystemCallKind::Display(_)) {
                    text.push('\n');
                }
                self.record(plan, site, clock, HdlStream::Stdout, text);
            }
            DigitalSystemCallKind::Strobe(radix) | DigitalSystemCallKind::Monitor(radix) => {
                match origin {
                    CallOrigin::Observer => {
                        let mut text = self.format(plan, site, args, 0, radix)?;
                        text.push('\n');
                        self.record(plan, site, clock, HdlStream::Stdout, text);
                    }
                    CallOrigin::Process(_) => {
                        let observer = site.observer.ok_or(DigitalEvalError::SystemCallFailed {
                            site: site.id,
                            detail: "an observer call has no argument function".into(),
                        })?;
                        if matches!(site.kind, DigitalSystemCallKind::Strobe(_)) {
                            self.queue.push(MonitorTask::Strobe(observer));
                        } else {
                            // Section 17.1.3: a new `$monitor` replaces the
                            // active one and prints at the end of this step.
                            self.monitor = Some(ActiveMonitor {
                                observer,
                                enabled: true,
                            });
                            self.queue.retain(|task| *task != MonitorTask::Monitor);
                            self.monitor_queued = false;
                            self.queue_monitor();
                        }
                    }
                }
            }
            DigitalSystemCallKind::MonitorOn => {
                if let Some(monitor) = &mut self.monitor {
                    monitor.enabled = true;
                }
                // Section 17.1.3: turning monitoring on prints the arguments
                // at the end of this step whether or not they changed.
                self.queue_monitor();
            }
            DigitalSystemCallKind::MonitorOff => {
                if let Some(monitor) = &mut self.monitor {
                    monitor.enabled = false;
                }
                self.queue.retain(|task| *task != MonitorTask::Monitor);
                self.monitor_queued = false;
            }
            DigitalSystemCallKind::Finish | DigitalSystemCallKind::Stop => {
                let stop = site.kind == DigitalSystemCallKind::Stop;
                let level = site.level.unwrap_or(1);
                self.halt(
                    site,
                    clock,
                    if stop {
                        HdlFinishKind::Stop
                    } else {
                        HdlFinishKind::Finish
                    },
                    level,
                );
                self.record(
                    plan,
                    site,
                    clock,
                    HdlStream::Finish { stop, level },
                    String::new(),
                );
                outcome.control = DigitalTaskControl::Halt;
            }
            DigitalSystemCallKind::Info
            | DigitalSystemCallKind::Warning
            | DigitalSystemCallKind::Error
            | DigitalSystemCallKind::Fatal => {
                let severity = match site.kind {
                    DigitalSystemCallKind::Info => HdlSeverity::Info,
                    DigitalSystemCallKind::Warning => {
                        self.severities.warnings += 1;
                        HdlSeverity::Warning
                    }
                    DigitalSystemCallKind::Error => {
                        self.severities.errors += 1;
                        HdlSeverity::Error
                    }
                    _ => {
                        self.severities.fatals += 1;
                        HdlSeverity::Fatal
                    }
                };
                let text = self.format(plan, site, args, 0, DigitalRadix::Decimal)?;
                self.record(plan, site, clock, HdlStream::Diagnostic { severity }, text);
                if site.kind == DigitalSystemCallKind::Fatal {
                    self.halt(site, clock, HdlFinishKind::Fatal, site.level.unwrap_or(1));
                    outcome.control = DigitalTaskControl::Halt;
                }
            }
            DigitalSystemCallKind::TimeFormat => {
                self.time_format = self.time_format_from(site, args)?;
            }
            DigitalSystemCallKind::PrintTimeScale => {
                let scope = match site.arguments.first() {
                    Some(DigitalSystemArgument::Scope(scope)) => *scope,
                    _ => site.scope,
                };
                let scope =
                    plan.scopes
                        .get(scope.index())
                        .ok_or(DigitalEvalError::SystemCallFailed {
                            site: site.id,
                            detail: "the named scope is not declared".into(),
                        })?;
                let text = format!(
                    "Time scale of ({}) is {} / {}\n",
                    scope.path,
                    format::time_unit_name(scope.time_scale.unit_exponent()),
                    format::time_unit_name(scope.time_scale.precision_exponent())
                );
                self.record(plan, site, clock, HdlStream::Stdout, text);
            }
            kind if kind.is_function() => {
                // A function enabled as a task: its value is discarded, its
                // outputs still land.
                let process = match origin {
                    CallOrigin::Process(process) => Some(process),
                    CallOrigin::Observer => None,
                };
                let result = self.function(plan, site, args, process, clock)?;
                outcome.outputs = result.outputs;
            }
            other => {
                return Err(DigitalEvalError::SystemCallUnavailable {
                    site: site.id,
                    detail: format!("`{}` has no task behaviour", other.name()),
                });
            }
        }
        Ok(outcome)
    }

    fn halt(
        &mut self,
        site: &DigitalSystemCall,
        clock: Option<DigitalClock>,
        kind: HdlFinishKind,
        level: u8,
    ) {
        if self.finish.is_none() {
            let (tick, seconds) =
                clock.map_or((0, 0.0), |clock| (clock.tick, clock.absolute_seconds));
            self.finish = Some(HdlFinishRequest {
                kind,
                level,
                site: site.id.0,
                tick,
                seconds,
            });
        }
    }

    /// `$timeformat(units, precision, suffix, minimum_width)`, or the
    /// defaults with no arguments (section 17.3.2).
    fn time_format_from(
        &self,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
    ) -> Result<TimeFormat, DigitalEvalError> {
        let mut format = TimeFormat::default_for(self.design_precision);
        for (position, argument) in site.arguments.iter().enumerate() {
            match (position, argument) {
                (2, DigitalSystemArgument::String(text)) => format.suffix = text.to_string(),
                (_, DigitalSystemArgument::Value { operand, class }) => {
                    let signed = matches!(class, DigitalValueClass::FourState { signed: true, .. });
                    let value = integer_of(args.get(*operand as usize), signed).ok_or(
                        DigitalEvalError::SystemCallFailed {
                            site: site.id,
                            detail: "a `$timeformat` argument is not a known integer".into(),
                        },
                    )?;
                    match position {
                        0 => {
                            if !(-15..=0).contains(&value) {
                                return Err(DigitalEvalError::SystemCallFailed {
                                    site: site.id,
                                    detail: format!(
                                        "`$timeformat` units {value} are outside 0 to -15 \
                                         (IEEE 1364-2005 table 17-9)"
                                    ),
                                });
                            }
                            format.units = value as i8;
                        }
                        1 => format.precision = value.clamp(0, 30) as u8,
                        3 => format.min_width = value.clamp(0, 1 << 20) as u32,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        Ok(format)
    }

    /// Call a system function.
    pub(crate) fn system_function(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: DigitalSystemCallId,
        args: &[DigitalScalar],
        process: Option<DigitalProcessId>,
        clock: Option<DigitalClock>,
    ) -> Result<DigitalFunctionOutcome, DigitalEvalError> {
        let site =
            plan.system_calls
                .get(site.index())
                .ok_or(DigitalEvalError::SystemCallUnavailable {
                    site,
                    detail: "the plan declares no such site".into(),
                })?;
        Self::refuse_unlanded(site)?;
        self.function(plan, site, args, process, clock)
    }

    fn function(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        args: &[DigitalScalar],
        process: Option<DigitalProcessId>,
        clock: Option<DigitalClock>,
    ) -> Result<DigitalFunctionOutcome, DigitalEvalError> {
        use DigitalSystemCallKind as Kind;
        let operand = |index: usize| -> Option<(&DigitalScalar, bool)> {
            let (operand, class) = match site.arguments.get(index)? {
                DigitalSystemArgument::Value { operand, class } => (*operand, *class),
                DigitalSystemArgument::Output {
                    operand: Some(operand),
                    class,
                    ..
                } => (*operand, *class),
                _ => return None,
            };
            let signed = matches!(class, DigitalValueClass::FourState { signed: true, .. });
            args.get(operand as usize).map(|value| (value, signed))
        };
        let integer_argument = |index: usize| -> Result<i32, DigitalEvalError> {
            let (value, signed) = operand(index).ok_or(DigitalEvalError::SystemCallFailed {
                site: site.id,
                detail: format!("argument {} is missing", index + 1),
            })?;
            Ok(integer_of(Some(value), signed).map_or(0, |value| value as i32))
        };
        let signed_word = |value: i32| {
            DigitalScalar::FourState(FourStateValue::from_u64(32, value as u32 as u64))
        };
        let output_word = |value: i32, index: usize| -> DigitalScalar {
            match site.arguments.get(index) {
                Some(DigitalSystemArgument::Output {
                    class: DigitalValueClass::Real,
                    ..
                }) => DigitalScalar::Real(f64::from(value)),
                _ => signed_word(value),
            }
        };
        let seeded = |this: &mut Self,
                      draw: &dyn Fn(&mut i32) -> random::Distribution|
         -> Result<DigitalFunctionOutcome, DigitalEvalError> {
            let mut seed = integer_argument(0)?;
            match draw(&mut seed) {
                random::Distribution::Value(value) => Ok(DigitalFunctionOutcome {
                    value: signed_word(value),
                    outputs: vec![output_word(seed, 0)],
                }),
                random::Distribution::Invalid(message) => {
                    this.warn(plan, site, clock, message);
                    Ok(DigitalFunctionOutcome {
                        value: signed_word(0),
                        outputs: vec![output_word(seed, 0)],
                    })
                }
            }
        };
        match site.kind {
            Kind::Random => {
                if site.arguments.is_empty() {
                    let value = random::random(&mut self.random_seed);
                    Ok(DigitalFunctionOutcome {
                        value: signed_word(value),
                        outputs: Vec::new(),
                    })
                } else {
                    let mut seed = integer_argument(0)?;
                    let value = random::random(&mut seed);
                    Ok(DigitalFunctionOutcome {
                        value: signed_word(value),
                        outputs: vec![output_word(seed, 0)],
                    })
                }
            }
            Kind::DistUniform => {
                let (start, end) = (integer_argument(1)?, integer_argument(2)?);
                seeded(self, &|seed| {
                    random::Distribution::Value(random::dist_uniform(seed, start, end))
                })
            }
            Kind::DistNormal => {
                let (mean, deviation) = (integer_argument(1)?, integer_argument(2)?);
                seeded(self, &|seed| random::dist_normal(seed, mean, deviation))
            }
            Kind::DistExponential => {
                let mean = integer_argument(1)?;
                seeded(self, &|seed| random::dist_exponential(seed, mean))
            }
            Kind::DistPoisson => {
                let mean = integer_argument(1)?;
                seeded(self, &|seed| random::dist_poisson(seed, mean))
            }
            Kind::DistChiSquare => {
                let degrees = integer_argument(1)?;
                seeded(self, &|seed| random::dist_chi_square(seed, degrees))
            }
            Kind::DistT => {
                let degrees = integer_argument(1)?;
                seeded(self, &|seed| random::dist_t(seed, degrees))
            }
            Kind::DistErlang => {
                let (k, mean) = (integer_argument(1)?, integer_argument(2)?);
                seeded(self, &|seed| random::dist_erlang(seed, k, mean))
            }
            Kind::Urandom | Kind::UrandomRange => {
                // 1800 section 18.13.1: an explicit seed restarts the calling
                // thread's stream; section 18.13.2 swaps reversed bounds.
                let reseed = if site.kind == Kind::Urandom && !site.arguments.is_empty() {
                    Some(integer_argument(0)? as u32)
                } else {
                    None
                };
                let range = if site.kind == Kind::UrandomRange {
                    let maximum = integer_argument(0)? as u32;
                    let minimum = if site.arguments.len() > 1 {
                        integer_argument(1)? as u32
                    } else {
                        0
                    };
                    Some((maximum, minimum))
                } else {
                    None
                };
                let generator = self.urandom_stream(plan, process, site)?;
                if let Some(seed) = reseed {
                    *generator = random::Pcg32::from_seed(seed);
                }
                let value = match range {
                    Some((maximum, minimum)) => generator.range(maximum, minimum),
                    None => generator.next_u32(),
                };
                Ok(DigitalFunctionOutcome {
                    value: DigitalScalar::FourState(FourStateValue::from_u64(32, u64::from(value))),
                    outputs: Vec::new(),
                })
            }
            Kind::TestPlusargs => {
                let name =
                    string_argument(site, args, 0).ok_or(DigitalEvalError::SystemCallFailed {
                        site: site.id,
                        detail: "`$test$plusargs` needs a string".into(),
                    })?;
                let found = plusargs::test(&self.config.plusargs, &name);
                Ok(DigitalFunctionOutcome {
                    value: signed_word(i32::from(found)),
                    outputs: Vec::new(),
                })
            }
            Kind::ValuePlusargs => {
                let format =
                    string_argument(site, args, 0).ok_or(DigitalEvalError::SystemCallFailed {
                        site: site.id,
                        detail: "`$value$plusargs` needs a format string".into(),
                    })?;
                let Some(DigitalSystemArgument::Output { class, .. }) = site.arguments.get(1)
                else {
                    return Err(DigitalEvalError::SystemCallFailed {
                        site: site.id,
                        detail: "`$value$plusargs` needs a variable to assign".into(),
                    });
                };
                let converted =
                    plusargs::value(&self.config.plusargs, &format, *class).map_err(|detail| {
                        DigitalEvalError::SystemCallFailed {
                            site: site.id,
                            detail,
                        }
                    })?;
                // An unmatched plusarg leaves the variable alone (section
                // 17.10.2), which is a write-back of what it already holds.
                let (found, output) = match converted {
                    Some(value) => (1, value),
                    None => (
                        0,
                        current_output(site, args).unwrap_or_else(|| unknown_of(*class)),
                    ),
                };
                Ok(DigitalFunctionOutcome {
                    value: signed_word(found),
                    outputs: vec![output],
                })
            }
            other => Err(DigitalEvalError::SystemCallUnavailable {
                site: site.id,
                detail: format!("`{}` has no function behaviour", other.name()),
            }),
        }
    }

    fn urandom_stream(
        &mut self,
        plan: &CanonicalDigitalPlan,
        process: Option<DigitalProcessId>,
        site: &DigitalSystemCall,
    ) -> Result<&mut random::Pcg32, DigitalEvalError> {
        let process = process.ok_or(DigitalEvalError::SystemCallFailed {
            site: site.id,
            detail: "`$urandom` runs only in a process".into(),
        })?;
        let index = usize::from(process);
        if index >= self.urandom.len() {
            self.urandom.resize(index + 1, None);
        }
        if self.urandom[index].is_none() {
            // The process's identity is its scope path and where it was
            // written, so adding a process elsewhere moves nothing here.
            let scope = plan
                .scopes
                .get(site.scope.index())
                .map_or("", |scope| scope.path.as_str());
            let span = plan
                .process(process)
                .map_or(0, |process| process.span.start);
            let identity = format!("{scope}@{span}");
            self.urandom[index] = Some(random::Pcg32::for_process(self.config.seed, &identity));
        }
        Ok(self.urandom[index].as_mut().expect("seeded stream"))
    }

    fn warn(
        &mut self,
        plan: &CanonicalDigitalPlan,
        site: &DigitalSystemCall,
        clock: Option<DigitalClock>,
        message: &str,
    ) {
        self.severities.warnings += 1;
        let text = format!("{}: {message}", site.kind.name());
        self.record(
            plan,
            site,
            clock,
            HdlStream::Diagnostic {
                severity: HdlSeverity::Warning,
            },
            text,
        );
    }

    /// Restore the state a checkpoint saved.
    pub(crate) fn restore_state(&mut self, image: &checkpoint::EffectsImage) -> Result<(), String> {
        image.restore_into(self)
    }
}

fn unknown_of(class: DigitalValueClass) -> DigitalScalar {
    match class {
        DigitalValueClass::FourState { width, .. } => {
            DigitalScalar::FourState(FourStateValue::splat(width, FourStateBit::Unknown))
        }
        DigitalValueClass::Real => DigitalScalar::Real(0.0),
    }
}

/// The value an output argument held before the call, when it was read.
fn current_output(site: &DigitalSystemCall, args: &[DigitalScalar]) -> Option<DigitalScalar> {
    site.arguments.iter().find_map(|argument| match argument {
        DigitalSystemArgument::Output {
            operand: Some(operand),
            ..
        } => args.get(*operand as usize).cloned(),
        _ => None,
    })
}

/// A known integer's value, two's complement under `signed`.
fn integer_of(value: Option<&DigitalScalar>, signed: bool) -> Option<i64> {
    match value? {
        DigitalScalar::FourState(bits) => {
            let low = bits.resized(bits.width().min(64));
            let value = low.to_integer(signed && bits.width() <= 64)?;
            // A value wider than 64 bits keeps its low 64, as a C cast does.
            Some(i64::try_from(value).unwrap_or(value as i64))
        }
        DigitalScalar::Integer(value) => Some(i64::from(*value)),
        DigitalScalar::Real(value) => value.is_finite().then(|| value.round() as i64),
        DigitalScalar::Effect => None,
    }
}

/// A string argument: a literal, or a packed value read eight bits per
/// character with leading NULs dropped.
fn string_argument(
    site: &DigitalSystemCall,
    args: &[DigitalScalar],
    index: usize,
) -> Option<String> {
    match site.arguments.get(index)? {
        DigitalSystemArgument::String(text) => Some(text.to_string()),
        DigitalSystemArgument::Value { operand, .. } => {
            let DigitalScalar::FourState(bits) = args.get(*operand as usize)? else {
                return None;
            };
            let bytes = bits.width().div_ceil(8);
            let mut text = String::new();
            for byte in (0..bytes).rev() {
                let value = (0..8).fold(0u8, |value, bit| {
                    let index = byte * 8 + bit;
                    if index < bits.width() && bits.bit(index) == FourStateBit::One {
                        value | 1 << bit
                    } else {
                        value
                    }
                });
                if value != 0 {
                    text.push(char::from(value));
                }
            }
            Some(text)
        }
        _ => None,
    }
}
