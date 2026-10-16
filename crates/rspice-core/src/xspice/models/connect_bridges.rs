//! The connect-rule converters: the A/D, D/A and bidirectional converters a
//! connect rule or an HDL port inserts at a digital/analog boundary.
//!
//! They share one parameter vocabulary with the connect-rule declarations
//! (`vthi`, `vtlo`, `txdel`, `vlo`, `vhi`, `rlo`, `rhi`, ...) and one law:
//!
//! - **A/D:** hysteresis between `vtlo` and `vthi`, and X once the input has
//!   stayed between them for `txdel` ([`AnalogBandLaw`]). `txdel = 0` is the
//!   XSPICE unknown band and `txdel = inf` pure hysteresis.
//! - **D/A:** each element is a Thevenin source whose level and resistance
//!   follow its state and strength, shaped like Verilog-AMS `transition()`:
//!   a change takes the full `trise`/`tfall` whatever its swing, restarts
//!   from the present output when interrupted, and starts `tdelay` after the
//!   digital event.
//! - **Supply:** the tracking form reads a `sup` port. `vthi`, `vtlo` and
//!   `vhi` not given explicitly are then fractions of V(sup) (one half, one
//!   half and one by default), and `vx`/`vz` follow them; a level given
//!   explicitly is absolute volts, and `vlo` always is. Below `vsupmin` the converter is unpowered: the A/D
//!   reads X and the D/A drives `vlo` through `rx`.
//!
//! A D/A converts per driver: each input element is one driver, so a net
//! with several drivers ties several output elements to one node and their
//! Thevenin branches sum there, rather than converting the digitally
//! resolved value.
//!
//! At their defaults the A/D is bit-identical to the HDL host's built-in
//! converter with `txdel = inf` and to `adc_bridge` with `txdel = 0`, and the
//! D/A to the HDL host's built-in D/A; the tests hold them to it.

#![cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "no boundary selects the connect-rule converters until the builder routes them"
    )
)]

use super::bridges::{
    ADC_UNINITIALIZED_STATE, OFFICIAL_BRIDGE_TIMING_MIN, adc_bridge_delay_for_transition,
    analog_vector_input_value, bridge_vector_width, digital_vector_input_value,
    finite_bridge_param, held_converter_descriptor, tracked_supply, with_supply_port,
};
use super::{AdcBridge, DacBridge};
use crate::Value;
use crate::xspice::{
    AnalogBandLaw, AnalogInputThreshold, AnalogThresholdBehavior, CmContext, CmError, CmResult,
    CodeModel, DigitalState, DigitalStrength, DigitalValue, EvaluationPhase, ParamSpec,
    PortDirection, PortSpec, PortType, XspiceCheckpointSupport, XspiceSmallSignalDescriptor,
};

#[cfg(test)]
mod tests;

/// The tracked fraction of the supply a threshold left at its default sits at.
const DEFAULT_THRESHOLD_FRACTION: Value = 0.5;
/// The tracked fraction of the supply `vhi` left at its default sits at.
const DEFAULT_HIGH_FRACTION: Value = 1.0;
/// The band-entry slot of an element whose input is outside the band.
const NO_BAND_ENTRY: Value = -1.0;

/// A level or threshold: `absolute + fraction·V(sup)`. A converter without a
/// live supply reads only `absolute`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Level {
    absolute: Value,
    fraction: Value,
}

impl Level {
    const fn volts(absolute: Value) -> Self {
        Self {
            absolute,
            fraction: 0.0,
        }
    }

    const fn of_supply(fraction: Value) -> Self {
        Self {
            absolute: 0.0,
            fraction,
        }
    }

    /// The level in volts. `supply` is the live supply voltage, `None` for a
    /// converter whose levels are all volts.
    fn at(self, supply: Option<Value>) -> Value {
        match supply {
            None => self.absolute,
            Some(supply) => self.absolute + self.fraction * supply,
        }
    }

    fn midpoint(self, other: Self) -> Self {
        Self {
            absolute: self.absolute + (other.absolute - self.absolute) / 2.0,
            fraction: self.fraction + (other.fraction - self.fraction) / 2.0,
        }
    }

    /// The level a fraction `progress` of the way to `target`; the target
    /// itself, exactly, once the progress is complete.
    fn toward(self, target: Self, progress: Value) -> Self {
        if progress >= 1.0 {
            return target;
        }
        Self {
            absolute: self.absolute + progress * (target.absolute - self.absolute),
            fraction: self.fraction + progress * (target.fraction - self.fraction),
        }
    }

    fn differs_from(self, other: Self) -> bool {
        (self.absolute - other.absolute).abs() > 1.0e-12
            || (self.fraction - other.fraction).abs() > 1.0e-12
    }
}

fn invalid(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_owned(),
        message: message.into(),
    }
}

/// A level parameter. In the tracking form a level that `tracks` and was
/// not given explicitly is its value as a fraction of the supply, so its
/// declared default is the default fraction; an explicit value is always
/// absolute volts.
fn level_param(
    ctx: &CmContext,
    model: &str,
    name: &str,
    tracking: bool,
    tracks: bool,
) -> CmResult<Level> {
    let value = finite_bridge_param(ctx, model, name)?;
    Ok(if tracking && tracks && !ctx.param_was_provided(name) {
        Level::of_supply(value)
    } else {
        Level::volts(value)
    })
}

fn time_param(ctx: &CmContext, model: &str, name: &str) -> CmResult<Value> {
    let value = finite_bridge_param(ctx, model, name)?;
    if value < 0.0 {
        return Err(invalid(name, format!("{model} {name} must be nonnegative")));
    }
    Ok(value)
}

/// A resistance: positive, and finite unless `infinite_allowed`.
fn resistance_param(
    ctx: &CmContext,
    model: &str,
    name: &str,
    infinite_allowed: bool,
) -> CmResult<Value> {
    let value = ctx.param(name);
    if value.is_nan() || value <= 0.0 || (value.is_infinite() && !infinite_allowed) {
        return Err(invalid(
            name,
            format!("{model} {name} must be a positive resistance, got {value}"),
        ));
    }
    Ok(value)
}

/// The supply voltage below which a tracking converter is unpowered; zero
/// turns the check off.
fn supply_min_param(ctx: &CmContext, model: &str) -> CmResult<Value> {
    finite_bridge_param(ctx, model, "vsupmin")
}

fn unpowered(supply: Option<Value>, supply_min: Value) -> bool {
    supply.is_some_and(|supply| supply_min > 0.0 && supply < supply_min)
}

/// The threshold on the supply itself that localizes where a tracking
/// converter loses or regains power.
fn supply_threshold(supply_min: Value, behavior: AnalogThresholdBehavior) -> AnalogInputThreshold {
    AnalogInputThreshold {
        port: "sup".into(),
        element: 0,
        low: supply_min,
        high: supply_min,
        behavior,
    }
}

fn adc_parameters() -> Vec<ParamSpec> {
    vec![
        ParamSpec::real("vtlo", DEFAULT_THRESHOLD_FRACTION)
            .with_description("Falling threshold; a fraction of the supply when left to track it"),
        ParamSpec::real("vthi", DEFAULT_THRESHOLD_FRACTION)
            .with_description("Rising threshold; a fraction of the supply when left to track it"),
        ParamSpec::real("txdel", Value::INFINITY)
            .with_description("Time an input may stay between the thresholds before reading X"),
        ParamSpec::real("tdrise", 0.0).with_description("Delay to a published 1"),
        ParamSpec::real("tdfall", 0.0).with_description("Delay to a published 0"),
    ]
}

fn dac_parameters() -> Vec<ParamSpec> {
    vec![
        ParamSpec::real("vlo", 0.0).with_description("Level for 0"),
        ParamSpec::real("vhi", DEFAULT_HIGH_FRACTION)
            .with_description("Level for 1; the supply when left to track it"),
        ParamSpec::real("vx", 0.5).with_description("Level for X; midway between vlo and vhi"),
        ParamSpec::real("vz", 0.5).with_description("Level for Z; vx by default"),
        ParamSpec::real("rlo", 20.0).with_description("Strong 0 resistance"),
        ParamSpec::real("rhi", 20.0).with_description("Strong 1 resistance"),
        ParamSpec::real("rx", 20.0).with_description("X resistance; min(rlo, rhi) by default"),
        ParamSpec::real("rz", Value::INFINITY).with_description("Z resistance"),
        ParamSpec::real("rsup", 2.0)
            .with_description("Supply-strength resistance; min(rlo, rhi)/10 by default"),
        ParamSpec::real("rpull", 1.0e4).with_description("Pull-strength resistance"),
        ParamSpec::real("rweak", 1.0e5).with_description("Weak-strength resistance"),
        ParamSpec::real("trise", 0.0).with_description("Rise time of every rising change"),
        ParamSpec::real("tfall", 0.0).with_description("Fall time of every falling change"),
        ParamSpec::real("tdelay", 0.0).with_description("Delay before a change starts"),
    ]
}

fn supply_min_parameter() -> ParamSpec {
    ParamSpec::real("vsupmin", 0.0)
        .with_description("Supply below which the converter is unpowered; zero is off")
}

//=============================================================================
// A/D
//=============================================================================

/// The A/D law from an instance's parameters.
#[derive(Debug, Clone, Copy)]
struct AdcLaw {
    low: Level,
    high: Level,
    band: AnalogBandLaw,
    rise: Value,
    fall: Value,
    supply_min: Value,
}

impl AdcLaw {
    fn from_params(ctx: &CmContext, model: &str, tracking: bool) -> CmResult<Self> {
        let low = level_param(ctx, model, "vtlo", tracking, true)?;
        let high = level_param(ctx, model, "vthi", tracking, true)?;
        let inverted = (low.fraction == high.fraction && low.absolute > high.absolute)
            || (low.absolute == high.absolute && low.fraction > high.fraction);
        if inverted {
            return Err(invalid(
                "vtlo",
                format!("{model} vtlo must not exceed vthi"),
            ));
        }
        let timeout = ctx.param("txdel");
        if timeout.is_nan() || timeout < 0.0 {
            return Err(invalid(
                "txdel",
                format!("{model} txdel must be nonnegative, got {timeout}"),
            ));
        }
        Ok(Self {
            low,
            high,
            band: AnalogBandLaw { timeout },
            rise: time_param(ctx, model, "tdrise")?,
            fall: time_param(ctx, model, "tdfall")?,
            supply_min: supply_min_param(ctx, model)?,
        })
    }

    /// How the mixed trial localizes this law's crossings. A finite timeout
    /// needs the band's entry localized too, which only the unknown band
    /// reports as a decision.
    fn root_behavior(self) -> AnalogThresholdBehavior {
        if self.band.timeout.is_infinite() {
            AnalogThresholdBehavior::Hysteresis
        } else {
            AnalogThresholdBehavior::UnknownBand
        }
    }

    /// The quantity compared and the two thresholds it is compared with.
    /// With a live supply the quantity is `input − low(V)` and the thresholds
    /// `0` and `high(V) − low(V)`.
    fn terms(self, input: Value, supply: Option<Value>) -> (Value, Value, Value) {
        match supply {
            None => (input, self.low.absolute, self.high.absolute),
            Some(_) => {
                let low = self.low.at(supply);
                (input - low, 0.0, self.high.at(supply) - low)
            }
        }
    }

    /// The declared thresholds on `port`. A tracking converter declares each
    /// band edge as a zero threshold of its own crossing function, and one
    /// only when the edges coincide; see [`Self::crossing`].
    fn thresholds(self, port: &str, width: usize, tracking: bool) -> Vec<AnalogInputThreshold> {
        let behavior = self.root_behavior();
        let mut thresholds = Vec::new();
        for element in 0..width {
            let threshold = |low, high| AnalogInputThreshold {
                port: port.into(),
                element,
                low,
                high,
                behavior,
            };
            if !tracking {
                thresholds.push(threshold(self.low.absolute, self.high.absolute));
                continue;
            }
            thresholds.push(threshold(0.0, 0.0));
            if self.low != self.high {
                thresholds.push(threshold(0.0, 0.0));
            }
        }
        if tracking && self.supply_min > 0.0 {
            thresholds.push(supply_threshold(self.supply_min, behavior));
        }
        thresholds
    }

    /// The crossing function of the `threshold`-th edge declared on an
    /// input element: `input − low(V)` first, then `input − high(V)`.
    fn crossing(self, supply: Value, threshold: usize, input: Value) -> Value {
        let edge = if threshold == 0 { self.low } else { self.high };
        input - edge.at(Some(supply))
    }
}

/// Where an A/D half reads, publishes and keeps its state.
#[derive(Debug, Clone, Copy)]
struct AdcSlots {
    input: &'static str,
    output: &'static str,
    /// First real state: the compared quantity per element, then the
    /// band-entry time per element.
    state: usize,
}

fn init_adc(ctx: &mut CmContext, slots: AdcSlots, width: usize) {
    ctx.allocate_int_states(width);
    ctx.allocate_states(slots.state + 2 * width);
    for index in 0..width {
        ctx.set_int_state(index, ADC_UNINITIALIZED_STATE);
        ctx.set_initial_state(slots.state + width + index, NO_BAND_ENTRY);
    }
}

fn adc_code(state: DigitalState) -> i64 {
    match state {
        DigitalState::Zero => 0,
        DigitalState::One => 1,
        _ => -1,
    }
}

fn evaluate_adc(
    ctx: &mut CmContext,
    model: &str,
    tracking: bool,
    slots: AdcSlots,
    width: usize,
) -> CmResult<()> {
    let law = AdcLaw::from_params(ctx, model, tracking)?;
    let commit = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;
    let supply = tracking.then(|| tracked_supply(ctx));
    let unpowered = unpowered(supply, law.supply_min);
    let timed = law.band.timeout > 0.0 && law.band.timeout.is_finite();
    for index in 0..width {
        let (term, low, high) =
            law.terms(analog_vector_input_value(ctx, slots.input, index), supply);
        let previous = ctx.int_state(index);
        let held = match previous {
            0 => Some(DigitalState::Zero),
            1 => Some(DigitalState::One),
            _ => None,
        };
        let entry_slot = slots.state + width + index;
        let accepted_entry = ctx.state_prev(entry_slot);
        let entry = if ctx.is_transient() && low < term && term < high {
            if accepted_entry >= 0.0 {
                accepted_entry
            } else {
                ctx.time
            }
        } else {
            NO_BAND_ENTRY
        };
        let expired = entry >= 0.0 && ctx.time >= law.band.deadline(entry);
        let decision = law.band.decision(
            term,
            (ctx.time > 0.0).then(|| ctx.state_prev(slots.state + index)),
            held,
            low,
            high,
            expired,
        );
        let state = if unpowered {
            -1
        } else {
            decision.map_or(
                if previous == ADC_UNINITIALIZED_STATE {
                    -1
                } else {
                    previous
                },
                |(state, _)| adc_code(state),
            )
        };
        if !commit {
            continue;
        }
        if state != previous {
            let value = match state {
                0 => DigitalValue::zero(),
                1 => DigitalValue::one(),
                _ => DigitalValue::unknown(),
            };
            // An X the timeout or the supply causes is published at its own
            // instant; the edge delays belong to the thresholds.
            let delay = if state == -1 && (unpowered || (timed && expired)) {
                0.0
            } else {
                adc_bridge_delay_for_transition(ctx.time, previous, state, law.rise, law.fall)
            };
            ctx.set_output_digital_vector_element(slots.output, index, value, delay);
        }
        ctx.set_int_state(index, state);
        ctx.set_state(slots.state + index, term);
        ctx.set_state(entry_slot, entry);
        if timed && entry >= 0.0 && state != -1 {
            let deadline = law.band.deadline(entry);
            if deadline > ctx.time {
                ctx.request_breakpoint(deadline);
            }
        }
    }
    Ok(())
}

/// The connect-rule analog-to-digital converter.
///
/// `tracking` is the live-supply form, with a `sup` port.
#[derive(Debug, Default)]
pub(crate) struct ConnectAdc {
    pub(crate) tracking: bool,
}

const ADC_SLOTS: AdcSlots = AdcSlots {
    input: "in",
    output: "out",
    state: 0,
};

impl CodeModel for ConnectAdc {
    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        held_converter_descriptor()
    }

    fn name(&self) -> &str {
        if self.tracking {
            "__rspice_connect_tracking_adc"
        } else {
            "__rspice_connect_adc"
        }
    }

    fn description(&self) -> &str {
        "Connect-rule analog to digital converter"
    }

    fn ports(&self) -> &[PortSpec] {
        static TRACKING: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        if self.tracking {
            TRACKING.get_or_init(|| with_supply_port(AdcBridge.ports()))
        } else {
            AdcBridge.ports()
        }
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: std::sync::OnceLock<Vec<ParamSpec>> = std::sync::OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = adc_parameters();
            params.push(supply_min_parameter());
            params
        })
    }

    fn analog_input_thresholds(&self, ctx: &CmContext) -> CmResult<Vec<AnalogInputThreshold>> {
        let width = bridge_vector_width(ctx, self.name())?;
        Ok(
            AdcLaw::from_params(ctx, self.name(), self.tracking)?.thresholds(
                ADC_SLOTS.input,
                width,
                self.tracking,
            ),
        )
    }

    fn analog_threshold_input(
        &self,
        ctx: &CmContext,
        port: &str,
        element: usize,
        input: Value,
    ) -> Value {
        self.analog_threshold_input_at(ctx, port, element, 0, input)
    }

    fn analog_threshold_input_at(
        &self,
        ctx: &CmContext,
        port: &str,
        _element: usize,
        threshold: usize,
        input: Value,
    ) -> Value {
        adc_crossing(ctx, self.name(), self.tracking, port, threshold, input)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, self.name())?;
        init_adc(ctx, ADC_SLOTS, width);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, self.name())?;
        evaluate_adc(ctx, self.name(), self.tracking, ADC_SLOTS, width)
    }
}

/// [`CodeModel::analog_threshold_input_at`] for an A/D half: the edge's
/// crossing function on its input, and the supply itself on `sup`.
fn adc_crossing(
    ctx: &CmContext,
    model: &str,
    tracking: bool,
    port: &str,
    threshold: usize,
    input: Value,
) -> Value {
    if !tracking || port.eq_ignore_ascii_case("sup") {
        return input;
    }
    match AdcLaw::from_params(ctx, model, tracking) {
        Ok(law) => law.crossing(tracked_supply(ctx), threshold, input),
        Err(_) => input,
    }
}

//=============================================================================
// D/A
//=============================================================================

/// A driver's strength, in IEEE 1364's order. XSPICE values reach only
/// strong, pull (`Resistive`), high impedance and undetermined; supply and
/// weak come from HDL drive strengths.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConverterStrength {
    Supply,
    Strong,
    Pull,
    Weak,
    HighZ,
    Undetermined,
}

/// A driver's logic state as a D/A converts it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DriveState {
    Zero,
    One,
    Unknown,
    HighZ,
}

/// A digital value as a state and a strength. A state that names its own
/// strength (`0r`, `1z`, ...) is authoritative over the value's strength.
fn classify(value: DigitalValue) -> (DriveState, ConverterStrength) {
    use ConverterStrength as S;
    let strength = match value.strength {
        DigitalStrength::Strong => S::Strong,
        DigitalStrength::Resistive => S::Pull,
        DigitalStrength::HighZ => S::HighZ,
        DigitalStrength::Undetermined => S::Undetermined,
    };
    match value.state {
        DigitalState::Zero => (DriveState::Zero, strength),
        DigitalState::One => (DriveState::One, strength),
        DigitalState::Unknown => (DriveState::Unknown, strength),
        DigitalState::ZeroR => (DriveState::Zero, S::Pull),
        DigitalState::OneR => (DriveState::One, S::Pull),
        DigitalState::UnknownR => (DriveState::Unknown, S::Pull),
        DigitalState::ZeroZ => (DriveState::Zero, S::HighZ),
        DigitalState::OneZ => (DriveState::One, S::HighZ),
        DigitalState::UnknownZ => (DriveState::Unknown, S::HighZ),
        DigitalState::HighZ => (DriveState::HighZ, S::HighZ),
    }
}

/// What an element drives toward, as persisted: a state, or unpowered.
const TARGET_ZERO: Value = 0.0;
const TARGET_ONE: Value = 1.0;
const TARGET_UNKNOWN: Value = 2.0;
const TARGET_HIGH_Z: Value = 3.0;
const TARGET_UNPOWERED: Value = 4.0;
/// An element no evaluation has committed yet.
const TARGET_UNCOMMITTED: Value = -1.0;

fn target_code(state: DriveState) -> Value {
    match state {
        DriveState::Zero => TARGET_ZERO,
        DriveState::One => TARGET_ONE,
        DriveState::Unknown => TARGET_UNKNOWN,
        DriveState::HighZ => TARGET_HIGH_Z,
    }
}

/// The D/A law from an instance's parameters.
#[derive(Debug, Clone, Copy)]
struct DacLaw {
    low: Level,
    high: Level,
    unknown: Level,
    released: Level,
    r_low: Value,
    r_high: Value,
    r_unknown: Value,
    r_released: Value,
    r_supply: Value,
    r_pull: Value,
    r_weak: Value,
    rise: Value,
    fall: Value,
    delay: Value,
    supply_min: Value,
}

impl DacLaw {
    fn from_params(ctx: &CmContext, model: &str, tracking: bool) -> CmResult<Self> {
        let low = level_param(ctx, model, "vlo", tracking, false)?;
        let high = level_param(ctx, model, "vhi", tracking, true)?;
        let unknown = if ctx.param_was_provided("vx") {
            Level::volts(finite_bridge_param(ctx, model, "vx")?)
        } else {
            low.midpoint(high)
        };
        let released = if ctx.param_was_provided("vz") {
            Level::volts(finite_bridge_param(ctx, model, "vz")?)
        } else {
            unknown
        };
        let r_low = resistance_param(ctx, model, "rlo", false)?;
        let r_high = resistance_param(ctx, model, "rhi", false)?;
        let strong = r_low.min(r_high);
        let derived = |name: &str, default: Value| {
            if ctx.param_was_provided(name) {
                resistance_param(ctx, model, name, false)
            } else {
                Ok(default)
            }
        };
        Ok(Self {
            low,
            high,
            unknown,
            released,
            r_low,
            r_high,
            r_unknown: derived("rx", strong)?,
            r_released: resistance_param(ctx, model, "rz", true)?,
            r_supply: derived("rsup", strong / 10.0)?,
            r_pull: resistance_param(ctx, model, "rpull", false)?,
            r_weak: resistance_param(ctx, model, "rweak", false)?,
            rise: time_param(ctx, model, "trise")?,
            fall: time_param(ctx, model, "tfall")?,
            delay: time_param(ctx, model, "tdelay")?,
            supply_min: supply_min_param(ctx, model)?,
        })
    }

    /// The output resistance of a driver in `state` at `strength`.
    pub(crate) fn resistance(&self, state: DriveState, strength: ConverterStrength) -> Value {
        use ConverterStrength as S;
        match (state, strength) {
            (DriveState::HighZ, _) | (_, S::HighZ) => self.r_released,
            (DriveState::Zero | DriveState::One, S::Supply) => self.r_supply,
            (DriveState::Zero, S::Strong) => self.r_low,
            (DriveState::One, S::Strong) => self.r_high,
            (_, S::Pull) => self.r_pull,
            (_, S::Weak) => self.r_weak,
            (DriveState::Unknown, S::Supply | S::Strong) | (_, S::Undetermined) => self.r_unknown,
        }
    }

    fn level(&self, target: Value) -> Level {
        if target == TARGET_ONE {
            self.high
        } else if target == TARGET_UNKNOWN {
            self.unknown
        } else if target == TARGET_HIGH_Z {
            self.released
        } else {
            self.low
        }
    }

    /// The full duration of a change toward `target` from `from`: `trise`
    /// for a rise and `tfall` for a fall, whatever the swing.
    fn duration(&self, from: Level, target: Value, supply: Option<Value>) -> Value {
        if target == TARGET_ONE {
            self.rise
        } else if target == TARGET_ZERO || target == TARGET_UNPOWERED {
            self.fall
        } else if self.level(target).at(supply) >= from.at(supply) {
            self.rise
        } else {
            self.fall
        }
    }
}

fn conductance(resistance: Value) -> Value {
    if resistance.is_infinite() {
        0.0
    } else {
        1.0 / resistance
    }
}

/// Per-element D/A state stride: start time, start level (absolute,
/// fraction), target, and the conductances from and to.
const DAC_STATE_STRIDE: usize = 6;

/// One element's change: from `from` with conductance `g_from` at `start`,
/// to `target` with `g_to`.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Change {
    start: Value,
    from: Level,
    target: Value,
    g_from: Value,
    g_to: Value,
}

impl Change {
    fn settled(time: Value, level: Level, target: Value, conductance: Value) -> Self {
        Self {
            start: time,
            from: level,
            target,
            g_from: conductance,
            g_to: conductance,
        }
    }

    fn accepted(ctx: &CmContext, base: usize) -> Self {
        Self {
            start: ctx.state_prev(base),
            from: Level {
                absolute: ctx.state_prev(base + 1),
                fraction: ctx.state_prev(base + 2),
            },
            target: ctx.state_prev(base + 3),
            g_from: ctx.state_prev(base + 4),
            g_to: ctx.state_prev(base + 5),
        }
    }

    fn store(self, ctx: &mut CmContext, base: usize) {
        ctx.set_state(base, self.start);
        ctx.set_state(base + 1, self.from.absolute);
        ctx.set_state(base + 2, self.from.fraction);
        ctx.set_state(base + 3, self.target);
        ctx.set_state(base + 4, self.g_from);
        ctx.set_state(base + 5, self.g_to);
    }

    fn progress(self, time: Value, law: &DacLaw, supply: Option<Value>) -> Value {
        let duration = law.duration(self.from, self.target, supply);
        if time <= self.start {
            0.0
        } else if duration <= 0.0 {
            1.0
        } else {
            ((time - self.start) / duration).min(1.0)
        }
    }

    /// The level and conductance at `time`. At the change's own instant both
    /// are still the left limit: the solution at an edge is computed with
    /// the drive from before it.
    fn drive_at(self, time: Value, law: &DacLaw, supply: Option<Value>) -> (Level, Value) {
        if time <= self.start {
            return (self.from, self.g_from);
        }
        let progress = self.progress(time, law, supply);
        let level = self.from.toward(law.level(self.target), progress);
        // Releasing and seizing the net are switches at the edge instant;
        // between two driven states the conductance moves with the level.
        let conductance = if self.g_from == 0.0 || self.g_to == 0.0 || progress >= 1.0 {
            self.g_to
        } else {
            self.g_from + progress * (self.g_to - self.g_from)
        };
        (level, conductance)
    }
}

/// One D/A element's drive at the present evaluation, and the change it is
/// on. A pure read of the context, so the Jacobian and the noise analysis
/// can ask for it without committing an evaluation.
#[derive(Debug, Clone, Copy)]
struct DacDrive {
    change: Change,
    level: Level,
    conductance: Value,
    /// The start of a change still waiting out `tdelay`.
    pending: Option<Value>,
}

/// Where a D/A half reads its drivers and the node it drives.
#[derive(Debug, Clone, Copy)]
struct DacSlots {
    input: &'static str,
    /// The analog input that reads the driven node, if any: a converter that
    /// seizes the net starts its change from the node's voltage.
    sense: Option<&'static str>,
}

fn dac_drive(
    ctx: &CmContext,
    law: &DacLaw,
    slots: DacSlots,
    index: usize,
    supply: Option<Value>,
) -> DacDrive {
    let base = index * DAC_STATE_STRIDE;
    let unpowered = unpowered(supply, law.supply_min);
    let (target, g_target) = if unpowered {
        (TARGET_UNPOWERED, conductance(law.r_unknown))
    } else {
        let (state, strength) = classify(digital_vector_input_value(ctx, slots.input, index));
        (
            target_code(state),
            conductance(law.resistance(state, strength)),
        )
    };
    let settled = |time| Change::settled(time, law.level(target), target, g_target);
    let held = |change: Change| {
        let (level, conductance) = change.drive_at(ctx.time, law, supply);
        DacDrive {
            change,
            level,
            conductance,
            pending: None,
        }
    };
    if !ctx.is_transient() || ctx.time == 0.0 {
        return held(settled(ctx.time));
    }
    let accepted = Change::accepted(ctx, base);
    let event = ctx.input_digital_vector_event_time(slots.input, index);
    if accepted.target == TARGET_UNCOMMITTED && event.is_none_or(|time| time <= 0.0) {
        return held(settled(ctx.time));
    }
    if target == accepted.target && g_target == accepted.g_to {
        return held(accepted);
    }
    // A change of supply is not a digital event: it starts where it is seen.
    let power_changed = target == TARGET_UNPOWERED || accepted.target == TARGET_UNPOWERED;
    let event = if power_changed {
        ctx.time
    } else {
        event.unwrap_or(ctx.time)
    };
    let start = event + law.delay;
    if ctx.time < start {
        return DacDrive {
            pending: Some(start),
            ..held(accepted)
        };
    }
    let (left_level, g_from) = accepted.drive_at(start, law, supply);
    let from = match slots.sense {
        Some(sense) if g_from == 0.0 => ctx
            .input_analog_vector_values(sense)
            .and_then(|values| values.get(index))
            .map_or(left_level, |value| Level::volts(value.value)),
        _ => left_level,
    };
    held(Change {
        start,
        from,
        target,
        g_from,
        g_to: g_target,
    })
}

fn init_dac(ctx: &mut CmContext, width: usize, model: &str) -> CmResult<usize> {
    let count = width
        .checked_mul(DAC_STATE_STRIDE)
        .ok_or_else(|| CmError::InvalidPortConnection(format!("{model} width is too large")))?;
    ctx.allocate_states(count);
    for index in 0..width {
        ctx.set_initial_state(index * DAC_STATE_STRIDE + 3, TARGET_UNCOMMITTED);
    }
    Ok(count)
}

fn evaluate_dac(
    ctx: &mut CmContext,
    model: &str,
    tracking: bool,
    slots: DacSlots,
    width: usize,
) -> CmResult<()> {
    let law = DacLaw::from_params(ctx, model, tracking)?;
    let supply = tracking.then(|| tracked_supply(ctx));
    let commit = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;
    for index in 0..width {
        let drive = dac_drive(ctx, &law, slots, index, supply);
        if commit {
            drive.change.store(ctx, index * DAC_STATE_STRIDE);
            if ctx.is_transient() {
                request_dac_breakpoints(ctx, &law, drive, supply);
            }
        }
        let voltage = drive.level.at(supply);
        ctx.set_output_vector_element_with_partial(
            "out",
            index,
            -drive.conductance * voltage,
            drive.conductance,
        )?;
    }
    Ok(())
}

/// The instants the stepper must land on: a delayed start, the end of a
/// ramp, and the completion of a zero-duration edge.
fn request_dac_breakpoints(
    ctx: &mut CmContext,
    law: &DacLaw,
    drive: DacDrive,
    supply: Option<Value>,
) {
    // A delayed change waits on the one still in force, which may itself be
    // ramping toward an end of its own.
    if let Some(start) = drive.pending {
        ctx.request_breakpoint(start);
    }
    let change = drive.change;
    if !change.from.differs_from(law.level(change.target)) {
        return;
    }
    let duration = law.duration(change.from, change.target, supply);
    if duration > 0.0 {
        let end = change.start + duration;
        if end > ctx.time + 1.0e-18 {
            ctx.request_breakpoint(end);
        }
    } else {
        let done = change.start + OFFICIAL_BRIDGE_TIMING_MIN;
        if done > ctx.time + 1.0e-18 {
            ctx.request_edge_completion(done);
        }
    }
}

/// The supply partial of a tracking D/A element's output current
/// `G·(V(out) − level)`: `−G·fraction`.
fn dac_supply_partials(
    ctx: &CmContext,
    model: &str,
    tracking: bool,
    slots: DacSlots,
    output_port: &str,
    index: usize,
) -> Vec<(String, Value)> {
    if !tracking || !output_port.eq_ignore_ascii_case("out") {
        return Vec::new();
    }
    let Ok(law) = DacLaw::from_params(ctx, model, tracking) else {
        return Vec::new();
    };
    let drive = dac_drive(ctx, &law, slots, index, Some(tracked_supply(ctx)));
    if drive.conductance == 0.0 {
        return Vec::new();
    }
    vec![("sup".to_string(), -drive.conductance * drive.level.fraction)]
}

/// Every element not released is its level behind a physical resistance,
/// and so noisy at the circuit temperature.
fn dac_noise_conductances(
    ctx: &CmContext,
    model: &str,
    tracking: bool,
    slots: DacSlots,
    width: usize,
) -> Vec<(usize, Value)> {
    let Ok(law) = DacLaw::from_params(ctx, model, tracking) else {
        return Vec::new();
    };
    let supply = tracking.then(|| tracked_supply(ctx));
    (0..width)
        .filter_map(|index| {
            let conductance = dac_drive(ctx, &law, slots, index, supply).conductance;
            (conductance > 0.0 && conductance.is_finite()).then_some((index, conductance))
        })
        .collect()
}

/// The thresholds a D/A declares: only where a tracking one loses power.
fn dac_thresholds(
    ctx: &CmContext,
    model: &str,
    tracking: bool,
) -> CmResult<Vec<AnalogInputThreshold>> {
    let supply_min = supply_min_param(ctx, model)?;
    Ok(if tracking && supply_min > 0.0 {
        vec![supply_threshold(
            supply_min,
            AnalogThresholdBehavior::Hysteresis,
        )]
    } else {
        Vec::new()
    })
}

fn conductance_output(ports: &mut [PortSpec], name: &str) {
    for port in ports.iter_mut().filter(|port| port.name == name) {
        port.default_type = PortType::Conductance;
        port.allowed_types = vec![PortType::Conductance, PortType::DifferentialConductance];
    }
}

/// The connect-rule digital-to-analog converter. Each element is one driver
/// behind its own Thevenin source; see the module documentation.
///
/// `tracking` is the live-supply form, with a `sup` port.
#[derive(Debug, Default)]
pub(crate) struct ConnectDac {
    pub(crate) tracking: bool,
}

const DAC_SLOTS: DacSlots = DacSlots {
    input: "in",
    sense: None,
};

impl CodeModel for ConnectDac {
    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        held_converter_descriptor()
    }

    fn name(&self) -> &str {
        if self.tracking {
            "__rspice_connect_tracking_dac"
        } else {
            "__rspice_connect_dac"
        }
    }

    fn description(&self) -> &str {
        "Connect-rule digital to analog converter"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        static TRACKING: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        let ports = || {
            let mut ports = DacBridge.ports().to_vec();
            conductance_output(&mut ports, "out");
            ports
        };
        if self.tracking {
            TRACKING.get_or_init(|| with_supply_port(&ports()))
        } else {
            PORTS.get_or_init(ports)
        }
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: std::sync::OnceLock<Vec<ParamSpec>> = std::sync::OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = dac_parameters();
            params.push(supply_min_parameter());
            params
        })
    }

    fn analog_input_thresholds(&self, ctx: &CmContext) -> CmResult<Vec<AnalogInputThreshold>> {
        dac_thresholds(ctx, self.name(), self.tracking)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, self.name())?;
        init_dac(ctx, width, self.name()).map(drop)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bridge_vector_width(ctx, self.name())?;
        evaluate_dac(ctx, self.name(), self.tracking, DAC_SLOTS, width)
    }

    fn output_vector_input_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        output_index: usize,
    ) -> Vec<(String, Value)> {
        dac_supply_partials(
            ctx,
            self.name(),
            self.tracking,
            DAC_SLOTS,
            output_port,
            output_index,
        )
    }

    fn excludes_output_from_transient_voltage_lte(&self, port: &str) -> bool {
        port.eq_ignore_ascii_case("out")
    }

    fn converter_noise_conductances(&self, ctx: &CmContext) -> Vec<(usize, Value)> {
        let Ok(width) = bridge_vector_width(ctx, self.name()) else {
            return Vec::new();
        };
        dac_noise_conductances(ctx, self.name(), self.tracking, DAC_SLOTS, width)
    }
}

//=============================================================================
// Bidirectional
//=============================================================================

/// The connect-rule bidirectional converter: the D/A, driven by the port's
/// own drivers on `in` and driving `out`, and the A/D reading the same node
/// through `sense` and publishing to the port's receivers on `rcv`. Both
/// halves work at once, so a port driving 0 against a pull-up reads its own
/// V_OL.
///
/// `tracking` is the live-supply form, with a `sup` port both halves read.
#[derive(Debug, Default)]
pub(crate) struct ConnectBidi {
    pub(crate) tracking: bool,
}

const BIDI_DAC_SLOTS: DacSlots = DacSlots {
    input: "in",
    sense: Some("sense"),
};

fn bidi_adc_slots(width: usize) -> AdcSlots {
    AdcSlots {
        input: "sense",
        output: "rcv",
        state: width * DAC_STATE_STRIDE,
    }
}

fn bidi_width(ctx: &CmContext, model: &str) -> CmResult<usize> {
    let width = ctx.port_width("in");
    if width == 0 {
        return Err(CmError::InvalidPortConnection(format!(
            "{model} requires non-empty port vectors"
        )));
    }
    for port in ["out", "sense", "rcv"] {
        let other = ctx.port_width(port);
        if other != width {
            return Err(CmError::InvalidPortConnection(format!(
                "{model} {port} vector width {other} does not match in vector width {width}"
            )));
        }
    }
    Ok(width)
}

impl CodeModel for ConnectBidi {
    fn checkpoint_support(&self, _ctx: &CmContext) -> XspiceCheckpointSupport {
        XspiceCheckpointSupport::Serializable
    }

    fn small_signal_descriptor(&self, _ctx: &CmContext) -> CmResult<XspiceSmallSignalDescriptor> {
        held_converter_descriptor()
    }

    fn name(&self) -> &str {
        if self.tracking {
            "__rspice_connect_tracking_bidir"
        } else {
            "__rspice_connect_bidir"
        }
    }

    fn description(&self) -> &str {
        "Connect-rule bidirectional converter"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        static TRACKING: std::sync::OnceLock<Vec<PortSpec>> = std::sync::OnceLock::new();
        let ports = || {
            let mut ports = ConnectDac { tracking: false }.ports().to_vec();
            ports.push(PortSpec {
                name: "sense".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
                allowed_types: vec![PortType::Voltage, PortType::DifferentialVoltage],
                is_vector: true,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "The driven node, read by the receiving half".to_string(),
            });
            ports.push(
                PortSpec::vector_output("rcv", PortType::Digital)
                    .with_description("The port's receivers"),
            );
            ports
        };
        if self.tracking {
            TRACKING.get_or_init(|| with_supply_port(&ports()))
        } else {
            PORTS.get_or_init(ports)
        }
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: std::sync::OnceLock<Vec<ParamSpec>> = std::sync::OnceLock::new();
        PARAMS.get_or_init(|| {
            let mut params = adc_parameters();
            params.extend(dac_parameters());
            params.push(supply_min_parameter());
            params
        })
    }

    fn analog_input_thresholds(&self, ctx: &CmContext) -> CmResult<Vec<AnalogInputThreshold>> {
        let width = bidi_width(ctx, self.name())?;
        Ok(
            AdcLaw::from_params(ctx, self.name(), self.tracking)?.thresholds(
                "sense",
                width,
                self.tracking,
            ),
        )
    }

    fn analog_threshold_input(
        &self,
        ctx: &CmContext,
        port: &str,
        element: usize,
        input: Value,
    ) -> Value {
        self.analog_threshold_input_at(ctx, port, element, 0, input)
    }

    fn analog_threshold_input_at(
        &self,
        ctx: &CmContext,
        port: &str,
        _element: usize,
        threshold: usize,
        input: Value,
    ) -> Value {
        adc_crossing(ctx, self.name(), self.tracking, port, threshold, input)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bidi_width(ctx, self.name())?;
        let dac_states = init_dac(ctx, width, self.name())?;
        init_adc(ctx, bidi_adc_slots(width), width);
        debug_assert_eq!(bidi_adc_slots(width).state, dac_states);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = bidi_width(ctx, self.name())?;
        evaluate_dac(ctx, self.name(), self.tracking, BIDI_DAC_SLOTS, width)?;
        evaluate_adc(
            ctx,
            self.name(),
            self.tracking,
            bidi_adc_slots(width),
            width,
        )
    }

    fn output_vector_input_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        output_index: usize,
    ) -> Vec<(String, Value)> {
        dac_supply_partials(
            ctx,
            self.name(),
            self.tracking,
            BIDI_DAC_SLOTS,
            output_port,
            output_index,
        )
    }

    fn excludes_output_from_transient_voltage_lte(&self, port: &str) -> bool {
        port.eq_ignore_ascii_case("out")
    }

    fn converter_noise_conductances(&self, ctx: &CmContext) -> Vec<(usize, Value)> {
        let Ok(width) = bidi_width(ctx, self.name()) else {
            return Vec::new();
        };
        dac_noise_conductances(ctx, self.name(), self.tracking, BIDI_DAC_SLOTS, width)
    }
}
