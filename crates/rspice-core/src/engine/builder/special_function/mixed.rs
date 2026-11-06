//! Hosts for the phase detector, sample-and-hold, modulator and varistor.

use super::*;
use crate::device::special_function::{Modulator, PhaseDetector, SampleHold, Varistor};
use std::f64::consts::{PI, TAU};

fn sensed_or_null(terminals: &Terminals<'_>, pin: usize) -> XspicePort {
    if terminals.used(pin) {
        terminals.sensed(pin)
    } else {
        XspicePort::Null
    }
}

/// Where a step's input crossed `level`, as a fraction of the step, when it
/// rose through it.
fn rising_fraction(before: Value, now: Value, level: Value) -> Option<Value> {
    (before < level && now >= level).then(|| ((level - before) / (now - before)).clamp(0.0, 1.0))
}

fn refuse_terminal(element: &Element, device: &str, pin: usize) -> SimulationError {
    SimulationError::Circuit(format!(
        "{}: {device} terminal {pin} must be tied to the common terminal",
        element.name
    ))
}

//=============================================================================
// PHASEDET
//=============================================================================

mod pfd_slot {
    pub(super) const STATE: usize = 0;
    /// The A and B input values; the operating point stores them low.
    pub(super) const A: usize = 1;
    pub(super) const B: usize = 2;
    pub(super) const COUNT: usize = 3;
}

struct PhaseDetectorModel {
    detector: PhaseDetector,
}

impl PhaseDetectorModel {
    fn state(&self, ctx: &CmContext) -> Value {
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            return 0.0;
        }
        let level = |value: Value| value >= self.detector.reference;
        let before = [
            level(ctx.state_prev(pfd_slot::A)),
            level(ctx.state_prev(pfd_slot::B)),
        ];
        let now = [level(ctx.input("a")), level(ctx.input("b"))];
        PhaseDetector::next(ctx.state_prev(pfd_slot::STATE), before, now)
    }
}

impl CodeModel for PhaseDetectorModel {
    fn name(&self) -> &str {
        "PHASEDET"
    }

    fn description(&self) -> &str {
        "LTspice phase-frequency detector"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("a", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("b", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("vout", PortType::DifferentialVoltage),
                PortSpec::output("out", PortType::DifferentialCurrent),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(pfd_slot::COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let state = self.state(ctx);
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            // An input already high counts as an edge at the first step.
            ctx.set_initial_state(pfd_slot::STATE, state);
            ctx.set_initial_state(pfd_slot::A, Value::MIN);
            ctx.set_initial_state(pfd_slot::B, Value::MIN);
        } else {
            let (a, b) = (ctx.input("a"), ctx.input("b"));
            ctx.set_state(pfd_slot::STATE, state);
            ctx.set_state(pfd_slot::A, a);
            ctx.set_state(pfd_slot::B, b);
        }
        let (current, _) = self.detector.current(state, ctx.input("vout"));
        ctx.set_output("out", current);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let (_, conductance) = self
            .detector
            .current(ctx.state(pfd_slot::STATE), ctx.input("vout"));
        vec![("vout".to_string(), conductance)]
    }
}

pub(super) fn phase_detector(
    element: &Element,
    terminals: &Terminals<'_>,
    params: &[(String, Value)],
) -> Result<Hosted, SimulationError> {
    let detector = PhaseDetector::from_params(params).map_err(|e| device_error(element, e))?;
    if !terminals.used(7) {
        return Err(refuse_terminal_output(element, "PHASEDET"));
    }
    Ok(Hosted {
        model: Arc::new(PhaseDetectorModel { detector }),
        ports: vec![
            sensed_or_null(terminals, 1),
            sensed_or_null(terminals, 2),
            terminals.sensed(7),
            terminals.driven(7),
        ],
        capacitances: Vec::new(),
    })
}

fn refuse_terminal_output(element: &Element, device: &str) -> SimulationError {
    SimulationError::Circuit(format!(
        "{}: {device} output terminal 7 is tied to the common terminal",
        element.name
    ))
}

//=============================================================================
// SAMPLEHOLD
//=============================================================================

mod sh_slot {
    /// The value the output holds.
    pub(super) const HELD: usize = 0;
    /// The CLK or S/H input value.
    pub(super) const CONTROL: usize = 1;
    /// The input value.
    pub(super) const INPUT: usize = 2;
    pub(super) const COUNT: usize = 3;
}

struct SampleHoldModel {
    device: SampleHold,
    /// Whether S/H (tracking) rather than CLK (sampling) is connected.
    tracking: bool,
}

impl SampleHoldModel {
    fn control_port(&self) -> &'static str {
        if self.tracking { "sh" } else { "clk" }
    }

    /// The value the output holds, and whether it follows the input now.
    fn held(&self, ctx: &CmContext) -> (Value, bool) {
        let input = ctx.input("in");
        let control = ctx.input(self.control_port());
        let high = control >= self.device.reference;
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            return (0.0, false);
        }
        let before = ctx.state_prev(sh_slot::CONTROL);
        let input_before = ctx.state_prev(sh_slot::INPUT);
        let at = |fraction: Value| input_before + fraction * (input - input_before);
        if self.tracking {
            if high {
                (input, true)
            } else if before >= self.device.reference {
                // S/H fell within the step: hold the input where it crossed.
                let fraction =
                    ((before - self.device.reference) / (before - control)).clamp(0.0, 1.0);
                (at(fraction), false)
            } else {
                (ctx.state_prev(sh_slot::HELD), false)
            }
        } else {
            match rising_fraction(before, control, self.device.reference) {
                Some(fraction) => (at(fraction), false),
                None => (ctx.state_prev(sh_slot::HELD), false),
            }
        }
    }
}

impl CodeModel for SampleHoldModel {
    fn name(&self) -> &str {
        "SAMPLEHOLD"
    }

    fn description(&self) -> &str {
        "LTspice sample-and-hold"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::DifferentialVoltage),
                PortSpec::input("clk", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("sh", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("vout", PortType::DifferentialVoltage),
                PortSpec::output("out", PortType::DifferentialCurrent),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(sh_slot::COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (held, _) = self.held(ctx);
        let (input, control) = (ctx.input("in"), ctx.input(self.control_port()));
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            ctx.set_initial_state(sh_slot::HELD, held);
            ctx.set_initial_state(sh_slot::CONTROL, control);
            ctx.set_initial_state(sh_slot::INPUT, input);
        } else {
            ctx.set_state(sh_slot::HELD, held);
            ctx.set_state(sh_slot::CONTROL, control);
            ctx.set_state(sh_slot::INPUT, input);
        }
        let (current, _) = self.device.current(held, ctx.input("vout"));
        ctx.set_output("out", current);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let (held, follows) = self.held(ctx);
        let (_, conductance) = self.device.current(held, ctx.input("vout"));
        let mut partials = vec![("vout".to_string(), conductance)];
        if follows {
            partials.push(("in".to_string(), -1.0 / self.device.rout));
        }
        partials
    }
}

pub(super) fn sample_hold(
    element: &Element,
    terminals: &Terminals<'_>,
    params: &[(String, Value)],
) -> Result<Hosted, SimulationError> {
    let device = SampleHold::from_params(params).map_err(|e| device_error(element, e))?;
    let tracking = match (terminals.used(3), terminals.used(4)) {
        (true, false) => false,
        (false, true) => true,
        (true, true) => {
            return Err(SimulationError::Circuit(format!(
                "{}: SAMPLEHOLD takes CLK (terminal 3) or S/H (terminal 4), not both",
                element.name
            )));
        }
        (false, false) => {
            return Err(SimulationError::Circuit(format!(
                "{}: SAMPLEHOLD needs CLK (terminal 3) or S/H (terminal 4) connected",
                element.name
            )));
        }
    };
    if terminals.used(6) {
        return Err(refuse_terminal(element, "SAMPLEHOLD", 6));
    }
    if !terminals.used(7) {
        return Err(refuse_terminal_output(element, "SAMPLEHOLD"));
    }
    Ok(Hosted {
        model: Arc::new(SampleHoldModel { device, tracking }),
        ports: vec![
            XspicePort::DifferentialVoltage {
                pos: terminals.node(1).to_string(),
                neg: terminals.node(2).to_string(),
            },
            sensed_or_null(terminals, 3),
            sensed_or_null(terminals, 4),
            terminals.sensed(7),
            terminals.driven(7),
        ],
        capacitances: Vec::new(),
    })
}

//=============================================================================
// MODULATOR
//=============================================================================

mod mod_slot {
    /// The oscillator phase in radians, within one turn.
    pub(super) const PHASE: usize = 0;
    /// The frequency at the accepted point.
    pub(super) const FREQUENCY: usize = 1;
    pub(super) const COUNT: usize = 2;
}

/// Time steps per oscillator period the modulator asks for.
const STEPS_PER_PERIOD: Value = 50.0;

struct ModulatorModel {
    modulator: Modulator,
    /// Whether FM (terminal 1) and AM (terminal 2) are used.
    fm: bool,
    am: bool,
    /// Whether `cos` (terminal 6) and `sin` (terminal 7) are used.
    outputs: [bool; 2],
}

impl ModulatorModel {
    fn frequency(&self, ctx: &CmContext) -> Value {
        let fm = if self.fm { ctx.input("fm") } else { 1.0 };
        let Modulator { mark, space, .. } = self.modulator;
        space + (mark - space) * fm
    }

    fn amplitude(&self, ctx: &CmContext) -> Value {
        if self.am { ctx.input("am") } else { 1.0 }
    }

    /// The phase, and its partial with respect to the FM input.
    fn phase(&self, ctx: &CmContext) -> (Value, Value) {
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            return (0.0, 0.0);
        }
        let dt = ctx.time - ctx.time_prev;
        let advance = PI * (ctx.state_prev(mod_slot::FREQUENCY) + self.frequency(ctx)) * dt;
        let Modulator { mark, space, .. } = self.modulator;
        (
            (ctx.state_prev(mod_slot::PHASE) + advance).rem_euclid(TAU),
            PI * (mark - space) * dt,
        )
    }

    /// The used outputs as (sense port, drive port, is `sin`).
    fn output_ports(&self) -> impl Iterator<Item = (&'static str, &'static str, bool)> + '_ {
        [("vcos", "cos", false), ("vsin", "sin", true)]
            .into_iter()
            .zip(self.outputs)
            .filter(|(_, used)| *used)
            .map(|(ports, _)| ports)
    }
}

impl CodeModel for ModulatorModel {
    fn name(&self) -> &str {
        "MODULATOR"
    }

    fn description(&self) -> &str {
        "LTspice quadrature modulator"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("fm", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("am", PortType::DifferentialVoltage).nullable(),
                PortSpec::input("vcos", PortType::DifferentialVoltage).nullable(),
                PortSpec::output("cos", PortType::DifferentialCurrent).nullable(),
                PortSpec::input("vsin", PortType::DifferentialVoltage).nullable(),
                PortSpec::output("sin", PortType::DifferentialCurrent).nullable(),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(mod_slot::COUNT);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (phase, _) = self.phase(ctx);
        let frequency = self.frequency(ctx);
        if ctx.is_dc_operating_point() || !ctx.is_transient() {
            ctx.set_initial_state(mod_slot::PHASE, phase);
            ctx.set_initial_state(mod_slot::FREQUENCY, frequency);
        } else {
            ctx.set_state(mod_slot::PHASE, phase);
            ctx.set_state(mod_slot::FREQUENCY, frequency);
            if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe && frequency != 0.0 {
                ctx.request_breakpoint(ctx.time + 1.0 / (STEPS_PER_PERIOD * frequency.abs()));
            }
        }
        let amplitude = self.amplitude(ctx);
        let rout = self.modulator.rout;
        for (sense, drive, sine) in self.output_ports() {
            let source = amplitude * if sine { phase.sin() } else { phase.cos() };
            ctx.set_output(drive, (ctx.input(sense) - source) / rout);
        }
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        let Some((sense, _, sine)) = self
            .output_ports()
            .find(|(_, drive, _)| output_port.eq_ignore_ascii_case(drive))
        else {
            return Vec::new();
        };
        let (phase, d_phase) = self.phase(ctx);
        let amplitude = self.amplitude(ctx);
        let rout = self.modulator.rout;
        // d/dφ of A·sin φ is A·cos φ, and of A·cos φ is −A·sin φ.
        let (wave, slope) = if sine {
            (phase.sin(), phase.cos())
        } else {
            (phase.cos(), -phase.sin())
        };
        let mut partials = vec![(sense.to_string(), 1.0 / rout)];
        if self.am {
            partials.push(("am".to_string(), -wave / rout));
        }
        if self.fm {
            partials.push(("fm".to_string(), -amplitude * slope * d_phase / rout));
        }
        partials
    }
}

pub(super) fn modulator(
    element: &Element,
    terminals: &Terminals<'_>,
    params: &[(String, Value)],
) -> Result<Hosted, SimulationError> {
    let modulator = Modulator::from_params(params).map_err(|e| device_error(element, e))?;
    let outputs = [terminals.used(6), terminals.used(7)];
    let mut ports = vec![sensed_or_null(terminals, 1), sensed_or_null(terminals, 2)];
    for (pin, used) in [(6, outputs[0]), (7, outputs[1])] {
        if used {
            ports.push(terminals.sensed(pin));
            ports.push(terminals.driven(pin));
        } else {
            ports.push(XspicePort::Null);
            ports.push(XspicePort::Null);
        }
    }
    Ok(Hosted {
        model: Arc::new(ModulatorModel {
            modulator,
            fm: terminals.used(1),
            am: terminals.used(2),
            outputs,
        }),
        ports,
        capacitances: Vec::new(),
    })
}

//=============================================================================
// VARISTOR
//=============================================================================

struct VaristorModel {
    varistor: Varistor,
}

impl CodeModel for VaristorModel {
    fn name(&self) -> &str {
        "VARISTOR"
    }

    fn description(&self) -> &str {
        "LTspice voltage-controlled varistor"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("ctl", PortType::DifferentialVoltage),
                PortSpec::input("vout", PortType::DifferentialVoltage),
                PortSpec::output("out", PortType::DifferentialCurrent),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn probe_writes_only_outputs(&self) -> bool {
        true
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        Ok(XspiceSmallSignalDescriptor::AffineAc)
    }

    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Stateless
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let (current, _, _) = self.varistor.current(ctx.input("vout"), ctx.input("ctl"));
        ctx.set_output("out", current);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let (_, d_out, d_control) = self.varistor.current(ctx.input("vout"), ctx.input("ctl"));
        vec![("vout".to_string(), d_out), ("ctl".to_string(), d_control)]
    }
}

pub(super) fn varistor(
    element: &Element,
    terminals: &Terminals<'_>,
    params: &[(String, Value)],
) -> Result<Hosted, SimulationError> {
    let varistor = Varistor::from_params(params).map_err(|e| device_error(element, e))?;
    if !terminals.used(7) {
        return Err(refuse_terminal_output(element, "VARISTOR"));
    }
    Ok(Hosted {
        model: Arc::new(VaristorModel { varistor }),
        ports: vec![
            XspicePort::DifferentialVoltage {
                pos: terminals.node(1).to_string(),
                neg: terminals.node(2).to_string(),
            },
            terminals.sensed(7),
            terminals.driven(7),
        ],
        capacitances: Vec::new(),
    })
}
