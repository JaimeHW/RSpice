//! The real-valued connect-rule converters: the `r2e`, `e2r` and `rbidir` a
//! connect rule naming the shipped real modules inserts where a `wreal` port
//! meets an electrical node. `rbidir` is the two halves on one port; see
//! [`ConnectRbidir`].
//!
//! - **`r2e`:** the port's resolved value behind `rout`, a Thevenin source
//!   whose conductance is `1/rout`; `rout = 0` is an ideal source. A change
//!   ramps from the present level to the new value over the full `trise`
//!   (upward) or `tfall` (downward), starting at the real event, and the
//!   stepper lands on the ramp's end.
//! - **`e2r`:** the node's voltage, sampled at analog evaluations and
//!   published as a real event once it has moved `vdelta` from a reference
//!   that advances in whole steps of `vdelta`, so a ramp publishes once per
//!   `vdelta` whatever the step sizes; never sooner than `ttol` after the last
//!   publication. It names no future time, so it never moves the step
//!   sequence (the sampling ruling of `v_to_real`).
//!
//! At their defaults (`rout = 0` with `trise = tfall`, and `vdelta = ttol =
//! 0`) the engine converts with the stock `real_to_v` and `v_to_real`
//! instead, so a deck that names the modules without changing them converts
//! exactly as one that names none.
//!
//! Small-signal analyses hold the event state: `r2e` stamps its conductance,
//! or an ideal source at its held level, and `e2r` reads its node and stamps
//! nothing. Noise analyses add `4kT/rout` across a Thevenin `r2e`.

use super::bridges::held_converter_descriptor;
use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortSpec, PortType,
    XspiceCheckpointSupport, XspiceSmallSignalDescriptor,
};

#[cfg(test)]
mod tests;

/// The shortest ramp, the floor `real_to_v` puts under its transition time.
const MIN_RAMP: Value = 1.0e-15;

fn invalid(name: &str, message: String) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message,
    }
}

fn nonnegative(ctx: &CmContext, model: &str, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() || value < 0.0 {
        return Err(invalid(
            name,
            format!("{model} parameter must be finite and nonnegative, got {value}"),
        ));
    }
    Ok(value)
}

fn committed(ctx: &CmContext) -> bool {
    ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
}

/// The rise and fall times of a real drive.
#[derive(Debug, Clone, Copy)]
struct RampLaw {
    rise: Value,
    fall: Value,
}

impl RampLaw {
    fn from_params(ctx: &CmContext, model: &str) -> CmResult<Self> {
        Ok(Self {
            rise: nonnegative(ctx, model, "trise")?.max(MIN_RAMP),
            fall: nonnegative(ctx, model, "tfall")?.max(MIN_RAMP),
        })
    }
}

/// One ramp of a real drive: from `from` at `start` to `target`. Three
/// states from `base`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Ramp {
    start: Value,
    from: Value,
    target: Value,
}

const RAMP_STATES: usize = 3;

impl Ramp {
    fn settled(time: Value, level: Value) -> Self {
        Self {
            start: time,
            from: level,
            target: level,
        }
    }

    fn accepted(ctx: &CmContext, base: usize) -> Self {
        Self {
            start: ctx.state_prev(base),
            from: ctx.state_prev(base + 1),
            target: ctx.state_prev(base + 2),
        }
    }

    fn store(self, ctx: &mut CmContext, base: usize) {
        if ctx.is_transient() {
            ctx.set_state(base, self.start);
            ctx.set_state(base + 1, self.from);
            ctx.set_state(base + 2, self.target);
        } else {
            ctx.set_initial_state(base, self.start);
            ctx.set_initial_state(base + 1, self.from);
            ctx.set_initial_state(base + 2, self.target);
        }
    }

    fn duration(self, law: RampLaw) -> Value {
        if self.target >= self.from {
            law.rise
        } else {
            law.fall
        }
    }

    /// The level at `time`. At the ramp's own instant it is still the left
    /// limit, as every digital-to-analog path here reads an edge.
    fn at(self, time: Value, law: RampLaw) -> Value {
        if time <= self.start {
            return self.from;
        }
        let duration = self.duration(law);
        // Compared against the end the stepper is asked to land on, so the
        // ramp is at its target exactly there.
        if self.from == self.target || time >= self.start + duration {
            self.target
        } else {
            self.from + (self.target - self.from) * ((time - self.start) / duration)
        }
    }

    fn end(self, law: RampLaw) -> Option<Value> {
        (self.from != self.target).then(|| self.start + self.duration(law))
    }

    /// The ramp toward `target` that starts at `time` from where this one is.
    fn toward(self, time: Value, target: Value, law: RampLaw) -> Self {
        Self {
            start: time,
            from: self.at(time, law),
            target,
        }
    }
}

/// The ramp a real drive is on at the present evaluation: a pure read of the
/// context, so the noise analysis can ask for it without committing.
fn drive_ramp(ctx: &CmContext, law: RampLaw, input: &str, base: usize) -> Ramp {
    let target = ctx.input_real(input).unwrap_or(0.0);
    if !ctx.is_transient() || ctx.time == 0.0 {
        return Ramp::settled(ctx.time, target);
    }
    let accepted = Ramp::accepted(ctx, base);
    if target == accepted.target {
        return accepted;
    }
    let event = ctx
        .input_real_event_time(input)
        .unwrap_or(ctx.time)
        .min(ctx.time);
    accepted.toward(event, target, law)
}

/// Commit a drive's ramp and ask the stepper to land on its end.
fn commit_ramp(ctx: &mut CmContext, ramp: Ramp, law: RampLaw, base: usize) {
    if !ctx.is_transient() || committed(ctx) {
        ramp.store(ctx, base);
    }
    if ctx.is_transient()
        && committed(ctx)
        && let Some(end) = ramp.end(law)
        && ctx.is_ahead(end)
    {
        ctx.request_breakpoint(end);
    }
}

fn rout_conductance(ctx: &CmContext, model: &str) -> CmResult<Value> {
    let rout = nonnegative(ctx, model, "rout")?;
    if rout <= 0.0 {
        return Err(invalid(
            "rout",
            format!("{model} drives through a positive rout, got {rout}"),
        ));
    }
    Ok(1.0 / rout)
}

fn ramp_parameters(rout: Value) -> Vec<ParamSpec> {
    vec![
        ParamSpec::real("rout", rout).with_description("Output resistance; 0 is an ideal source"),
        ParamSpec::real("trise", 1.0e-9).with_description("Time an upward change takes"),
        ParamSpec::real("tfall", 1.0e-9).with_description("Time a downward change takes"),
    ]
}

fn conductance_output(name: &str) -> PortSpec {
    let mut port = PortSpec::vector_output(name, PortType::Conductance);
    port.allowed_types = vec![PortType::Conductance, PortType::DifferentialConductance];
    port
}

//=============================================================================
// r2e
//=============================================================================

/// The real-to-electrical converter. `ideal` is the `rout = 0` form with
/// asymmetric edges: a voltage output with its own branch, where the
/// Thevenin form drives a one-element conductance output.
#[derive(Debug, Default)]
pub(crate) struct ConnectR2e {
    pub(crate) ideal: bool,
}

impl CodeModel for ConnectR2e {
    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        held_converter_descriptor()
    }

    fn name(&self) -> &str {
        if self.ideal {
            "__rspice_connect_ideal_r2e"
        } else {
            "__rspice_connect_r2e"
        }
    }

    fn description(&self) -> &str {
        "Connect-rule real to electrical converter"
    }

    fn ports(&self) -> &[PortSpec] {
        static IDEAL: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        static THEVENIN: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        if self.ideal {
            IDEAL.get_or_init(|| {
                vec![
                    PortSpec::input("in", PortType::Real),
                    PortSpec::output("out", PortType::Voltage),
                ]
            })
        } else {
            THEVENIN.get_or_init(|| {
                vec![
                    PortSpec::input("in", PortType::Real),
                    conductance_output("out"),
                ]
            })
        }
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: std::sync::OnceLock<Vec<ParamSpec>> = std::sync::OnceLock::new();
        PARAMS.get_or_init(|| ramp_parameters(0.0))
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(RAMP_STATES);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let law = RampLaw::from_params(ctx, self.name())?;
        let conductance = if self.ideal {
            0.0
        } else {
            rout_conductance(ctx, self.name())?
        };
        let ramp = drive_ramp(ctx, law, "in", 0);
        commit_ramp(ctx, ramp, law, 0);
        let level = ramp.at(ctx.time, law);
        if self.ideal {
            ctx.set_output("out", level);
            Ok(())
        } else {
            ctx.set_output_vector_element_with_partial("out", 0, -conductance * level, conductance)
        }
    }

    fn excludes_output_from_transient_voltage_lte(&self, port: &str) -> bool {
        !self.ideal && port.eq_ignore_ascii_case("out")
    }

    fn converter_noise_conductances(&self, ctx: &CmContext) -> Vec<(usize, Value)> {
        if self.ideal {
            return Vec::new();
        }
        rout_conductance(ctx, self.name())
            .map(|conductance| vec![(0, conductance)])
            .unwrap_or_default()
    }
}

//=============================================================================
// e2r
//=============================================================================

/// The publication threshold of a sampled real: `vdelta` on a reference grid,
/// and `ttol` between publications.
#[derive(Debug, Clone, Copy)]
struct SampleLaw {
    vdelta: Value,
    ttol: Value,
}

impl SampleLaw {
    fn from_params(ctx: &CmContext, model: &str) -> CmResult<Self> {
        Ok(Self {
            vdelta: nonnegative(ctx, model, "vdelta")?,
            ttol: nonnegative(ctx, model, "ttol")?,
        })
    }

    /// The reference after `sample` is published, or `None` while `sample`
    /// stays within `vdelta` of `reference`.
    fn crossed(self, reference: Value, sample: Value) -> Option<Value> {
        let moved = sample - reference;
        if self.vdelta == 0.0 {
            return (moved != 0.0).then_some(sample);
        }
        let steps = (moved.abs() / self.vdelta).floor();
        (steps >= 1.0).then(|| reference + moved.signum() * steps * self.vdelta)
    }
}

/// The reference and publication-time states of a sampling half, from `base`.
const SAMPLE_STATES: usize = 2;

/// Publish `sample` on `output` when the law says the node has moved, and
/// keep the reference it was measured against.
fn sample_node(ctx: &mut CmContext, law: SampleLaw, sample: Value, output: &str, base: usize) {
    if !committed(ctx) {
        return;
    }
    if !ctx.is_transient() || ctx.time == 0.0 {
        if ctx.is_transient() {
            ctx.set_state(base, sample);
            ctx.set_state(base + 1, ctx.time);
        } else {
            ctx.set_initial_state(base, sample);
            ctx.set_initial_state(base + 1, ctx.time);
        }
        if ctx
            .output_real(output)
            .is_none_or(|previous| previous.to_bits() != sample.to_bits())
        {
            ctx.set_output_real(output, sample, 0.0);
        }
        return;
    }
    let (reference, published) = (ctx.state_prev(base), ctx.state_prev(base + 1));
    let next = law
        .crossed(reference, sample)
        .filter(|_| ctx.time - published >= law.ttol);
    match next {
        Some(reference) => {
            ctx.set_state(base, reference);
            ctx.set_state(base + 1, ctx.time);
            ctx.set_output_real(output, sample, 0.0);
        }
        None => {
            ctx.set_state(base, reference);
            ctx.set_state(base + 1, published);
        }
    }
}

fn sample_parameters() -> Vec<ParamSpec> {
    vec![
        ParamSpec::real("vdelta", 0.0)
            .with_description("Change that publishes a sample; 0 publishes every change"),
        ParamSpec::real("ttol", 0.0).with_description("Shortest time between two publications"),
    ]
}

/// The electrical-to-real converter.
#[derive(Debug, Default)]
pub(crate) struct ConnectE2r;

impl CodeModel for ConnectE2r {
    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        held_converter_descriptor()
    }

    fn name(&self) -> &str {
        "__rspice_connect_e2r"
    }

    fn description(&self) -> &str {
        "Connect-rule electrical to real converter"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage),
                PortSpec::output("out", PortType::Real),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: std::sync::OnceLock<Vec<ParamSpec>> = std::sync::OnceLock::new();
        PARAMS.get_or_init(sample_parameters)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(SAMPLE_STATES);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let law = SampleLaw::from_params(ctx, self.name())?;
        let sample = ctx.input("in");
        sample_node(ctx, law, sample, "out", 0);
        Ok(())
    }
}

//=============================================================================
// rbidir
//=============================================================================

/// The drive half's activity state, after its ramp.
const RBIDIR_ACTIVE: usize = RAMP_STATES;
/// The receive half's sampling states, after the activity state.
const RBIDIR_SAMPLE: usize = RAMP_STATES + 1;

/// The bidirectional real converter: an `r2e` driving `out` from the port's
/// resolved net on `in`, and an `e2r` sampling `sense` onto `d`, the port's
/// receiving driver.
///
/// The drive half is active once the port's own drivers have produced a
/// value (LRM 2.4 section 3.7: a driver that has not run contributes
/// nothing). The circuit says so through the other-drivers observation on
/// `d` (see [`CodeModel::receives_real_output`]); the converter never infers
/// it from the values it sees. Until then the port reads the node: `d` is
/// the net's only value and `out` drives nothing. From then on the net
/// resolves its own drivers, `d` no longer counts, and `out` seizes the node
/// from its present voltage and drives the resolved value behind `rout`,
/// which is finite because the node has other sources. A driver cannot
/// release a real net, so the drive half stays active.
#[derive(Debug, Default)]
pub(crate) struct ConnectRbidir;

impl ConnectRbidir {
    /// Whether the port's own drivers drive its net, and since when.
    fn driven(ctx: &CmContext) -> Option<Value> {
        ctx.other_digital_drivers("d", 0)
            .filter(|(state, _)| *state != crate::xspice::DigitalValue::high_z())
            .map(|(_, since)| since)
    }

    /// The drive half's ramp and whether it drives: a pure read of the
    /// context, so the noise analysis can ask for it without committing.
    fn drive(ctx: &CmContext, law: RampLaw) -> Option<Ramp> {
        let since = Self::driven(ctx)?;
        let target = ctx.input_real("in").unwrap_or(0.0);
        if !ctx.is_transient() || ctx.time == 0.0 {
            return Some(Ramp::settled(ctx.time, target));
        }
        if ctx.state_prev(RBIDIR_ACTIVE) > 0.5 {
            return Some(drive_ramp(ctx, law, "in", 0));
        }
        // Seizing the node: from its voltage, where the port took it over.
        Some(Ramp {
            start: since.min(ctx.time),
            from: ctx.input("sense"),
            target,
        })
    }
}

impl CodeModel for ConnectRbidir {
    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        held_converter_descriptor()
    }

    fn name(&self) -> &str {
        "__rspice_connect_rbidir"
    }

    fn description(&self) -> &str {
        "Connect-rule bidirectional real/electrical converter"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Real),
                PortSpec::output("d", PortType::Real),
                conductance_output("out"),
                PortSpec::input("sense", PortType::Voltage),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: std::sync::OnceLock<Vec<ParamSpec>> = std::sync::OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut parameters = ramp_parameters(20.0);
            parameters.extend(sample_parameters());
            parameters
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(RBIDIR_SAMPLE + SAMPLE_STATES);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let law = RampLaw::from_params(ctx, self.name())?;
        let sampling = SampleLaw::from_params(ctx, self.name())?;
        let conductance = rout_conductance(ctx, self.name())?;
        let drive = Self::drive(ctx, law);
        let ramp = drive.unwrap_or_else(|| Ramp::settled(ctx.time, 0.0));
        commit_ramp(ctx, ramp, law, 0);
        let active = if drive.is_some() { 1.0 } else { 0.0 };
        if !ctx.is_transient() {
            ctx.set_initial_state(RBIDIR_ACTIVE, active);
        } else if committed(ctx) {
            ctx.set_state(RBIDIR_ACTIVE, active);
        }
        match drive {
            // The receive half publishes while the port reads the node.
            None => {
                let sample = ctx.input("sense");
                sample_node(ctx, sampling, sample, "d", RBIDIR_SAMPLE);
            }
            Some(_) if ctx.is_transient() && committed(ctx) => {
                for index in RBIDIR_SAMPLE..RBIDIR_SAMPLE + SAMPLE_STATES {
                    let kept = ctx.state_prev(index);
                    ctx.set_state(index, kept);
                }
            }
            Some(_) => {}
        }
        let (level, g) = match drive {
            Some(ramp) => (ramp.at(ctx.time, law), conductance),
            None => (0.0, 0.0),
        };
        ctx.set_output_vector_element_with_partial("out", 0, -g * level, g)
    }

    fn excludes_output_from_transient_voltage_lte(&self, port: &str) -> bool {
        port.eq_ignore_ascii_case("out")
    }

    fn converter_noise_conductances(&self, ctx: &CmContext) -> Vec<(usize, Value)> {
        let Ok(law) = RampLaw::from_params(ctx, self.name()) else {
            return Vec::new();
        };
        match (Self::drive(ctx, law), rout_conductance(ctx, self.name())) {
            (Some(_), Ok(conductance)) => vec![(0, conductance)],
            _ => Vec::new(),
        }
    }

    fn receives_real_output(&self, port: &str) -> bool {
        port.eq_ignore_ascii_case("d")
    }
}
