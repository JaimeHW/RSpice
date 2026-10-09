//! Portable execution state of a resource-free code-model context.
//!
//! The circuit must establish an accepted boundary and drain events/breakpoints
//! before calling this codec. Model and instance identity, input signatures,
//! shared-driver agreement and atomic circuit installation are outer contracts.

use super::*;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[cfg(test)]
#[path = "checkpoint_tests.rs"]
mod tests;

type Named<T> = Vec<(String, T)>;

#[derive(Clone, Copy, Debug)]
pub(crate) struct ContextCheckpointLimits {
    pub max_items: usize,
    pub max_name_bytes: usize,
}
impl Default for ContextCheckpointLimits {
    fn default() -> Self {
        Self {
            max_items: 16_777_216,
            max_name_bytes: 64 * 1024 * 1024,
        }
    }
}

/// Derived from the compiled port contract, never inferred from saved values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ContextPortShape {
    /// 0: analog, 1: digital, 2: real.
    pub domain: u8,
    pub vector: bool,
    pub width: usize,
}
#[derive(Default)]
pub(crate) struct ContextPortLayout {
    inputs: BTreeMap<String, ContextPortShape>,
    outputs: BTreeMap<String, ContextPortShape>,
}

impl ContextPortLayout {
    pub(crate) fn from_ports(
        ports: &[crate::xspice::PortSpec],
        connections: &[crate::xspice::PortConnection],
    ) -> Result<Self, String> {
        use crate::xspice::{PortConnection as C, PortDirection as D};
        if ports.len() != connections.len() {
            return Err("XSPICE context port/connection count differs".into());
        }
        let mut layout = Self::default();
        for (port, connection) in ports.iter().zip(connections) {
            let width = match connection {
                C::AnalogVector(v) | C::DigitalVector(v) | C::RealVector(v) => v.len(),
                C::TypedAnalogVector(v) => v.len(),
                C::DigitalVectorMapped(v) => v.len(),
                C::Null => 0,
                _ => 1,
            };
            let input_shape = match connection {
                C::Digital(_) | C::DigitalInverted(_) => Some((1, false)),
                C::DigitalVector(_) | C::DigitalVectorMapped(_) => Some((1, true)),
                C::Real(_) => Some((2, false)),
                C::RealVector(_) => Some((2, true)),
                C::AnalogVector(_) | C::TypedAnalogVector(_) => Some((0, true)),
                C::Null => None,
                _ => Some((0, false)),
            };
            if matches!(port.direction, D::In | D::InOut) {
                if let Some((domain, vector)) = input_shape {
                    if layout
                        .inputs
                        .insert(
                            port.name.clone(),
                            ContextPortShape {
                                domain,
                                vector,
                                width,
                            },
                        )
                        .is_some()
                    {
                        return Err("duplicate XSPICE context input port".into());
                    }
                }
            }
            if matches!(port.direction, D::Out | D::InOut) {
                let domain = match port.default_type {
                    PortType::Digital => 1,
                    PortType::Real => 2,
                    t if t.is_analog() && t != PortType::VoltageName => 0,
                    _ => return Err("unsupported XSPICE context output domain".into()),
                };
                let shape = ContextPortShape {
                    domain,
                    vector: port.is_vector,
                    width: if port.is_vector { width } else { 1 },
                };
                if layout.outputs.insert(port.name.clone(), shape).is_some() {
                    return Err("duplicate XSPICE context output port".into());
                }
            }
        }
        Ok(layout)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum PortImage {
    Analog([u64; 3]),
    AnalogVector(Vec<[u64; 3]>),
    Digital(DigitalValue),
    DigitalVector(Vec<DigitalValue>),
    Real(u64),
    RealVector(Vec<u64>),
}
impl PortImage {
    fn shape(&self) -> ContextPortShape {
        let (domain, vector, width) = match self {
            Self::Analog(_) => (0, false, 1),
            Self::AnalogVector(v) => (0, true, v.len()),
            Self::Digital(_) => (1, false, 1),
            Self::DigitalVector(v) => (1, true, v.len()),
            Self::Real(_) => (2, false, 1),
            Self::RealVector(v) => (2, true, v.len()),
        };
        ContextPortShape {
            domain,
            vector,
            width,
        }
    }
}
fn analog_image(v: &AnalogValue) -> [u64; 3] {
    [
        v.value.to_bits(),
        v.prev_value.to_bits(),
        v.partial.to_bits(),
    ]
}
fn analog_value(v: &[u64; 3]) -> AnalogValue {
    AnalogValue {
        value: f64::from_bits(v[0]),
        prev_value: f64::from_bits(v[1]),
        partial: f64::from_bits(v[2]),
    }
}
macro_rules! port_codec {
    ($ty:ident, $encode:ident, $decode:ident) => {
        fn $encode(v: &$ty) -> PortImage {
            match v {
                $ty::Analog(v) => PortImage::Analog(analog_image(v)),
                $ty::AnalogVector(v) => {
                    PortImage::AnalogVector(v.iter().map(analog_image).collect())
                }
                $ty::Digital(v) => PortImage::Digital(*v),
                $ty::DigitalVector(v) => PortImage::DigitalVector(v.clone()),
                $ty::Real(v) => PortImage::Real(v.to_bits()),
                $ty::RealVector(v) => PortImage::RealVector(bits(v)),
            }
        }
        fn $decode(v: &PortImage) -> $ty {
            match v {
                PortImage::Analog(v) => $ty::Analog(analog_value(v)),
                PortImage::AnalogVector(v) => {
                    $ty::AnalogVector(v.iter().map(analog_value).collect())
                }
                PortImage::Digital(v) => $ty::Digital(*v),
                PortImage::DigitalVector(v) => $ty::DigitalVector(v.clone()),
                PortImage::Real(v) => $ty::Real(f64::from_bits(*v)),
                PortImage::RealVector(v) => $ty::RealVector(values(v)),
            }
        }
    };
}
port_codec!(InputValue, input_image, input_value);
port_codec!(OutputValue, output_image, output_value);

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct TransitionImage {
    state: bool,
    event: u64,
    start: u64,
    end: u64,
}
impl From<AnalogTransition> for TransitionImage {
    fn from(v: AnalogTransition) -> Self {
        Self {
            state: v.state,
            event: v.event_time.to_bits(),
            start: v.transition_start.to_bits(),
            end: v.transition_end.to_bits(),
        }
    }
}
impl From<&TransitionImage> for AnalogTransition {
    fn from(v: &TransitionImage) -> Self {
        Self {
            state: v.state,
            event_time: f64::from_bits(v.event),
            transition_start: f64::from_bits(v.start),
            transition_end: f64::from_bits(v.end),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ContextRuntimeCheckpoint {
    version: u32,
    identity: [u8; 32],
    time: u64,
    time_prev: u64,
    timestep: u64,
    companion: [u64; 4],
    two_history: bool,
    xyce_order2: bool,
    call: u8,
    phase: u8,
    iteration: usize,
    inputs: Named<PortImage>,
    outputs: Named<PortImage>,
    committed: Named<Vec<DigitalValue>>,
    other: Named<Vec<Option<(DigitalValue, u64)>>>,
    input_times: Named<u64>,
    vector_times: Named<Vec<Option<u64>>>,
    string_revisions: Named<u64>,
    next_string_revision: u64,
    real_revisions: Named<u64>,
    next_real_revision: u64,
    state: Vec<u64>,
    state_prev: Vec<u64>,
    int_state: Vec<i64>,
    histories: Named<Vec<(u64, Vec<u64>)>>,
    input_transitions: Named<Vec<Option<TransitionImage>>>,
    output_transitions: Named<TransitionImage>,
    inertial: Named<(u64, DigitalValue)>,
    stamps: Vec<(usize, usize, u64)>,
    rhs: Vec<(usize, u64)>,
    static_stamps: Vec<(usize, usize, u64)>,
    static_rhs: Vec<(usize, u64)>,
}

fn bits(v: &[f64]) -> Vec<u64> {
    v.iter().map(|v| v.to_bits()).collect()
}
fn values(v: &[u64]) -> Vec<f64> {
    v.iter().map(|v| f64::from_bits(*v)).collect()
}
fn named<T, U>(map: &HashMap<String, T>, f: impl Fn(&T) -> U) -> Named<U> {
    let mut v: Vec<_> = map
        .iter()
        .map(|(name, value)| (name.clone(), f(value)))
        .collect();
    v.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    v
}
fn restored<T, U>(v: &Named<T>, f: impl Fn(&T) -> U) -> HashMap<String, U> {
    v.iter()
        .map(|(name, value)| (name.clone(), f(value)))
        .collect()
}
fn ordered<T>(v: &Named<T>) -> Result<(), String> {
    if v.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
        return Err("unordered or duplicate XSPICE context key".into());
    }
    Ok(())
}

/// Counts nested entries as well as values, so empty histories cannot bypass
/// the item limit. Names have their own aggregate byte ceiling.
struct Budget {
    items: usize,
    names: usize,
    limits: ContextCheckpointLimits,
}
impl Budget {
    fn new(limits: ContextCheckpointLimits) -> Self {
        Self {
            items: 0,
            names: 0,
            limits,
        }
    }
    fn add(&mut self, n: usize) -> Result<(), String> {
        self.items = self
            .items
            .checked_add(n)
            .filter(|n| *n <= self.limits.max_items)
            .ok_or("XSPICE context checkpoint item limit exceeded")?;
        Ok(())
    }
    fn name(&mut self, name: &str) -> Result<(), String> {
        self.add(1)?;
        self.names = self
            .names
            .checked_add(name.len())
            .filter(|n| *n <= self.limits.max_name_bytes)
            .ok_or("XSPICE context checkpoint name limit exceeded")?;
        Ok(())
    }
    fn map<'a, T: 'a>(
        &mut self,
        v: impl IntoIterator<Item = (&'a String, &'a T)>,
        f: impl Fn(&mut Self, &T) -> Result<(), String>,
    ) -> Result<(), String> {
        for (name, value) in v {
            self.name(name)?;
            f(self, value)?;
        }
        Ok(())
    }
}
macro_rules! port_size {
    ($v:expr, $ty:ident, $b:expr) => {
        match $v {
            $ty::Analog(_) => $b.add(3),
            $ty::AnalogVector(v) => {
                $b.add(v.len())?;
                $b.add(v.len())?;
                $b.add(v.len())
            }
            $ty::Digital(_) | $ty::Real(_) => $b.add(1),
            $ty::DigitalVector(v) => $b.add(v.len()),
            $ty::RealVector(v) => $b.add(v.len()),
        }
    };
}

impl CmContext {
    pub(crate) fn has_owned_runtime_checkpoint_state(&self) -> bool {
        self.has_serializable_checkpoint_state()
            || !self.transient_histories.is_empty()
            || !self.inertial_outputs.is_empty()
    }

    fn portable_boundary(&self) -> Result<(), String> {
        if self.resource_transaction.is_some()
            || !self.resources.values.is_empty()
            || !self.resources.transactional.is_empty()
        {
            return Err("XSPICE context has live host resources or an open transaction".into());
        }
        if self.has_pending_events() || self.has_requested_breakpoints() {
            return Err("XSPICE context events and breakpoint requests must be drained".into());
        }
        Ok(())
    }

    fn checkpoint_budget(&self, limits: ContextCheckpointLimits) -> Result<(), String> {
        let mut b = Budget::new(limits);
        b.add(self.state.len())?;
        b.add(self.state_prev.len())?;
        b.add(self.int_state.len())?;
        b.map(&self.inputs, |b, v| port_size!(v, InputValue, b))?;
        b.map(&self.outputs, |b, v| port_size!(v, OutputValue, b))?;
        b.map(&self.committed_digital_outputs, |b, v| b.add(v.len()))?;
        b.map(&self.other_digital_drivers, |b, v| {
            b.add(v.len())?;
            b.add(v.len())
        })?;
        b.map(&self.input_event_times, |b, _| b.add(1))?;
        b.map(&self.input_vector_event_times, |b, v| b.add(v.len()))?;
        b.map(&self.string_param_revisions, |b, _| b.add(1))?;
        b.map(&self.real_vector_param_revisions, |b, _| b.add(1))?;
        b.map(&self.transient_histories, |b, v| {
            b.add(v.len())?;
            for sample in v {
                b.add(sample.values.len())?;
            }
            Ok(())
        })?;
        b.map(&self.input_analog_vector_transitions, |b, v| {
            for _ in v {
                b.add(4)?;
            }
            Ok(())
        })?;
        b.map(&self.output_analog_transitions, |b, _| b.add(4))?;
        b.map(&self.inertial_outputs, |b, _| b.add(2))?;
        for v in [&self.stamps, &self.static_stamps] {
            for _ in v {
                b.add(3)?;
            }
        }
        for v in [&self.rhs, &self.static_rhs] {
            for _ in v {
                b.add(2)?;
            }
        }
        Ok(())
    }
}

// Canonical streaming fingerprint of receiving configuration. Real values use
// IEEE words and map keys are ordered; resource ceilings remain receiving policy.
struct Identity(blake3::Hasher);
impl Identity {
    fn word(&mut self, word: u64) {
        self.0.update(&word.to_le_bytes());
    }
    fn string(&mut self, value: &str) {
        self.word(value.len() as u64);
        self.0.update(value.as_bytes());
    }
    fn real(&mut self, value: f64) {
        self.word(value.to_bits());
    }
    fn optional(&mut self, value: Option<f64>) {
        self.word(u64::from(value.is_some()));
        if let Some(v) = value {
            self.real(v);
        }
    }
    fn vec<T>(&mut self, value: &[T], f: impl Fn(&mut Self, &T)) {
        self.word(value.len() as u64);
        for v in value {
            f(self, v);
        }
    }
    fn map<T>(&mut self, value: &HashMap<String, T>, f: impl Fn(&mut Self, &T)) {
        let mut ordered: Vec<_> = value.iter().collect();
        ordered.sort_unstable_by(|a, b| a.0.cmp(b.0));
        self.word(ordered.len() as u64);
        for (name, v) in ordered {
            self.string(name);
            f(self, v);
        }
    }
    fn ports(&mut self, ports: &BTreeMap<String, ContextPortShape>) {
        self.word(ports.len() as u64);
        for (name, p) in ports {
            self.string(name);
            self.word(u64::from(p.domain));
            self.word(u64::from(p.vector));
            self.word(p.width as u64);
        }
    }
}
fn identity(ctx: &CmContext, ports: &ContextPortLayout, matrix_size: usize) -> [u8; 32] {
    let mut h = Identity(blake3::Hasher::new());
    h.string("rspice-xspice-context-v1");
    h.word(matrix_size as u64);
    h.ports(&ports.inputs);
    h.ports(&ports.outputs);
    h.real(ctx.temperature);
    h.real(ctx.ramptime);
    h.optional(ctx.transient_step_hint);
    h.optional(ctx.transient_stop_time);
    h.word(u64::from(ctx.digital_delay_type.is_some()));
    if let Some(v) = ctx.digital_delay_type {
        h.word(v as u64);
    }
    h.map(&ctx.port_nodes, |h, v| h.word(*v as u64));
    h.map(&ctx.port_terminals, |h, v| {
        h.word(v.0 as u64);
        h.word(v.1 as u64);
    });
    h.map(&ctx.port_vector_terminals, |h, v| {
        h.vec(v, |h, v| {
            h.word(v.0 as u64);
            h.word(v.1 as u64);
        })
    });
    h.map(&ctx.port_control_columns, |h, v| h.word(*v as u64));
    h.map(&ctx.port_widths, |h, v| h.word(*v as u64));
    h.map(&ctx.port_total_loads, |h, v| h.real(*v));
    h.map(&ctx.port_vector_total_loads, |h, v| {
        h.vec(v, |h, v| h.real(*v))
    });
    h.map(&ctx.params, |h, v| h.real(*v));
    h.map(&ctx.complex_params, |h, v| {
        h.real(v.re);
        h.real(v.im);
    });
    h.map(&ctx.string_params, |h, v| h.string(v));
    h.map(&ctx.string_vector_params, |h, v| {
        h.vec(v, |h, v| h.string(v))
    });
    h.map(&ctx.complex_vector_params, |h, v| {
        h.vec(v, |h, v| {
            h.real(v.re);
            h.real(v.im);
        })
    });
    h.map(&ctx.real_vector_params, |h, v| h.vec(v, |h, v| h.real(*v)));
    h.map(&ctx.integer_vector_params, |h, v| {
        h.vec(v, |h, v| h.word(*v as u64))
    });
    let mut provided: Vec<_> = ctx.provided_params.iter().collect();
    provided.sort_unstable();
    h.word(provided.len() as u64);
    for name in provided {
        h.string(name);
    }
    for n in [ctx.state.len(), ctx.state_prev.len(), ctx.int_state.len()] {
        h.word(n as u64);
    }
    *h.0.finalize().as_bytes()
}

fn call_image(call: CallType) -> u8 {
    match call {
        CallType::Init => 0,
        CallType::DcAnalysis => 1,
        CallType::AcAnalysis => 2,
        CallType::TransientAnalysis => 3,
        CallType::EventDriven => 4,
        CallType::Probe => 5,
    }
}
fn phase_image(phase: EvaluationPhase) -> u8 {
    match phase {
        EvaluationPhase::DirectEvaluation => 0,
        EvaluationPhase::RollbackableProbe => 1,
        EvaluationPhase::CircuitTrial => 2,
        EvaluationPhase::AcceptedStep => 3,
    }
}

impl ContextRuntimeCheckpoint {
    pub(crate) fn capture(
        ctx: &CmContext,
        ports: &ContextPortLayout,
        accepted: f64,
        matrix_size: usize,
        limits: ContextCheckpointLimits,
    ) -> Result<Self, String> {
        ctx.portable_boundary()?;
        ctx.checkpoint_budget(limits)?;
        if ctx.analysis != AnalysisType::Transient {
            return Err("XSPICE runtime image requires transient analysis".into());
        }
        let c = ctx.transient_companion;
        let image = Self {
            version: 1,
            identity: identity(ctx, ports, matrix_size),
            time: ctx.time.to_bits(),
            time_prev: ctx.time_prev.to_bits(),
            timestep: ctx.timestep.to_bits(),
            companion: [
                c.coeff_g.to_bits(),
                c.coeff_v_n.to_bits(),
                c.coeff_v_n_minus_1.to_bits(),
                c.coeff_i_n.to_bits(),
            ],
            two_history: c.needs_two_history,
            xyce_order2: ctx.xyce_one_step_order2,
            call: call_image(ctx.call_type),
            phase: phase_image(ctx.evaluation_phase),
            iteration: ctx.iteration,
            inputs: named(&ctx.inputs, input_image),
            outputs: named(&ctx.outputs, output_image),
            committed: named(&ctx.committed_digital_outputs, Clone::clone),
            other: named(&ctx.other_digital_drivers, |v| {
                v.iter().map(|v| v.map(|(v, t)| (v, t.to_bits()))).collect()
            }),
            input_times: named(&ctx.input_event_times, |v| v.to_bits()),
            vector_times: named(&ctx.input_vector_event_times, |v| {
                v.iter().map(|v| v.map(f64::to_bits)).collect()
            }),
            string_revisions: named(&ctx.string_param_revisions, |v| *v),
            next_string_revision: ctx.next_string_param_revision,
            real_revisions: named(&ctx.real_vector_param_revisions, |v| *v),
            next_real_revision: ctx.next_real_vector_param_revision,
            state: bits(&ctx.state),
            state_prev: bits(&ctx.state_prev),
            int_state: ctx.int_state.clone(),
            histories: named(&ctx.transient_histories, |v| {
                v.iter()
                    .map(|v| (v.time.to_bits(), bits(&v.values)))
                    .collect()
            }),
            input_transitions: named(&ctx.input_analog_vector_transitions, |v| {
                v.iter().map(|v| v.map(Into::into)).collect()
            }),
            output_transitions: named(&ctx.output_analog_transitions, |v| (*v).into()),
            inertial: named(&ctx.inertial_outputs, |v| (v.when.to_bits(), v.prev)),
            stamps: ctx
                .stamps
                .iter()
                .map(|&(r, c, v)| (r, c, v.to_bits()))
                .collect(),
            rhs: ctx.rhs.iter().map(|&(r, v)| (r, v.to_bits())).collect(),
            static_stamps: ctx
                .static_stamps
                .iter()
                .map(|&(r, c, v)| (r, c, v.to_bits()))
                .collect(),
            static_rhs: ctx
                .static_rhs
                .iter()
                .map(|&(r, v)| (r, v.to_bits()))
                .collect(),
        };
        image.validate(ctx, ports, accepted, matrix_size, limits)?;
        Ok(image)
    }

    /// Returns a replacement, preserving receiving wiring, parameters and policy.
    /// The outer decoder must also cap file bytes before serde deserialization.
    pub(crate) fn restore(
        &self,
        ctx: &CmContext,
        ports: &ContextPortLayout,
        accepted: f64,
        matrix_size: usize,
        limits: ContextCheckpointLimits,
    ) -> Result<CmContext, String> {
        ctx.portable_boundary()?;
        self.validate(ctx, ports, accepted, matrix_size, limits)?;
        let mut c = ctx.clone();
        c.time = f64::from_bits(self.time);
        c.time_prev = f64::from_bits(self.time_prev);
        c.timestep = f64::from_bits(self.timestep);
        c.transient_companion = CompanionCoefficients {
            coeff_g: f64::from_bits(self.companion[0]),
            coeff_v_n: f64::from_bits(self.companion[1]),
            coeff_v_n_minus_1: f64::from_bits(self.companion[2]),
            coeff_i_n: f64::from_bits(self.companion[3]),
            needs_two_history: self.two_history,
        };
        c.xyce_one_step_order2 = self.xyce_order2;
        c.analysis = AnalysisType::Transient;
        c.call_type = match self.call {
            3 => CallType::TransientAnalysis,
            4 => CallType::EventDriven,
            _ => unreachable!("validated call"),
        };
        c.evaluation_phase = match self.phase {
            0 => EvaluationPhase::DirectEvaluation,
            2 => EvaluationPhase::CircuitTrial,
            3 => EvaluationPhase::AcceptedStep,
            _ => unreachable!("validated phase"),
        };
        c.iteration = self.iteration;
        c.inputs = restored(&self.inputs, input_value);
        c.outputs = restored(&self.outputs, output_value);
        c.committed_digital_outputs = restored(&self.committed, Clone::clone);
        c.other_digital_drivers = restored(&self.other, |v| {
            v.iter()
                .map(|v| v.map(|(v, t)| (v, f64::from_bits(t))))
                .collect()
        });
        c.input_event_times = restored(&self.input_times, |v| f64::from_bits(*v));
        c.input_vector_event_times = restored(&self.vector_times, |v| {
            v.iter().map(|v| v.map(f64::from_bits)).collect()
        });
        c.string_param_revisions = restored(&self.string_revisions, |v| *v);
        c.next_string_param_revision = self.next_string_revision;
        c.real_vector_param_revisions = restored(&self.real_revisions, |v| *v);
        c.next_real_vector_param_revision = self.next_real_revision;
        c.state = values(&self.state);
        c.state_prev = values(&self.state_prev);
        c.int_state = self.int_state.clone();
        c.transient_histories = restored(&self.histories, |v| {
            v.iter()
                .map(|(t, v)| TransientHistorySample {
                    time: f64::from_bits(*t),
                    values: values(v),
                })
                .collect()
        });
        c.input_analog_vector_transitions = restored(&self.input_transitions, |v| {
            v.iter().map(|v| v.as_ref().map(Into::into)).collect()
        });
        c.output_analog_transitions = restored(&self.output_transitions, |v| v.into());
        c.inertial_outputs = restored(&self.inertial, |(t, v)| InertialOutputState {
            when: f64::from_bits(*t),
            prev: *v,
        });
        c.stamps = self
            .stamps
            .iter()
            .map(|&(r, c, v)| (r, c, f64::from_bits(v)))
            .collect();
        c.rhs = self
            .rhs
            .iter()
            .map(|&(r, v)| (r, f64::from_bits(v)))
            .collect();
        c.static_stamps = self
            .static_stamps
            .iter()
            .map(|&(r, c, v)| (r, c, f64::from_bits(v)))
            .collect();
        c.static_rhs = self
            .static_rhs
            .iter()
            .map(|&(r, v)| (r, f64::from_bits(v)))
            .collect();
        Ok(c)
    }
}

fn event_time(bits: u64, accepted: f64) -> bool {
    let value = f64::from_bits(bits);
    value.is_finite() && value >= 0.0 && value <= accepted
}
fn transition_valid(t: &TransitionImage) -> bool {
    [t.event, t.start, t.end]
        .iter()
        .all(|v| f64::from_bits(*v).is_finite())
        && f64::from_bits(t.start) <= f64::from_bits(t.end)
}
fn port_matches(value: &PortImage, shape: Option<&ContextPortShape>) -> bool {
    shape.is_some_and(|shape| *shape == value.shape())
}
fn named_shape(
    ports: &BTreeMap<String, ContextPortShape>,
    name: &str,
    domain: u8,
    vector: bool,
    width: usize,
) -> bool {
    ports.get(name)
        == Some(&ContextPortShape {
            domain,
            vector,
            width,
        })
}
fn revision_valid<T>(v: &Named<u64>, params: &HashMap<String, T>, next: u64) -> bool {
    next > 0
        && v.len() == params.len()
        && v.iter()
            .all(|(name, revision)| params.contains_key(name) && *revision > 0)
}

impl ContextRuntimeCheckpoint {
    fn validate(
        &self,
        ctx: &CmContext,
        ports: &ContextPortLayout,
        accepted: f64,
        matrix_size: usize,
        limits: ContextCheckpointLimits,
    ) -> Result<(), String> {
        self.budget(limits)?;
        if self.version != 1 || self.identity != identity(ctx, ports, matrix_size) {
            return Err(
                "XSPICE context checkpoint schema or receiving configuration differs".into(),
            );
        }
        if !accepted.is_finite()
            || accepted < 0.0
            || !event_time(self.time, accepted)
            || !event_time(self.time_prev, f64::from_bits(self.time))
            || !f64::from_bits(self.timestep).is_finite()
            || f64::from_bits(self.timestep) < 0.0
            || !matches!(self.call, 3 | 4)
            || !matches!(self.phase, 0 | 2 | 3)
            || self
                .companion
                .iter()
                .any(|v| !f64::from_bits(*v).is_finite())
            || f64::from_bits(self.companion[0]) <= 0.0
        {
            return Err("invalid XSPICE context accepted clock, phase or integration rule".into());
        }
        if self.state.len() != ctx.state.len()
            || self.state_prev.len() != ctx.state_prev.len()
            || self.int_state.len() != ctx.int_state.len()
        {
            return Err("XSPICE context state shape differs from initialized model".into());
        }
        if self.inputs.len() != ports.inputs.len()
            || self.outputs.len() != ports.outputs.len()
            || self
                .inputs
                .iter()
                .any(|(name, v)| !port_matches(v, ports.inputs.get(name)))
            || self
                .outputs
                .iter()
                .any(|(name, v)| !port_matches(v, ports.outputs.get(name)))
        {
            return Err("XSPICE context input/output port shape differs".into());
        }
        for (_, v) in self.inputs.iter().chain(&self.outputs) {
            let finite = match v {
                PortImage::Analog(v) => v.iter().all(|v| f64::from_bits(*v).is_finite()),
                PortImage::AnalogVector(v) => {
                    v.iter().flatten().all(|v| f64::from_bits(*v).is_finite())
                }
                _ => true, // Real event payloads retain all IEEE encodings.
            };
            if !finite {
                return Err("nonfinite XSPICE analog port state".into());
            }
        }
        for (name, v) in &self.committed {
            let Some(shape) = ports.inputs.get(name) else {
                return Err("unknown XSPICE inout observation".into());
            };
            if shape.domain != 1 || ports.outputs.get(name) != Some(shape) || v.len() != shape.width
            {
                return Err("invalid XSPICE committed inout bank".into());
            }
        }
        for (name, v) in &self.other {
            let Some(shape) = ports.inputs.get(name) else {
                return Err("unknown XSPICE other-driver observation".into());
            };
            if shape.domain != 1
                || ports.outputs.get(name) != Some(shape)
                || v.len() != shape.width
                || v.iter()
                    .flatten()
                    .any(|(_, time)| !event_time(*time, accepted))
            {
                return Err("invalid XSPICE other-driver bank".into());
            }
        }
        for (name, t) in &self.input_times {
            if !event_time(*t, accepted)
                || !ports
                    .inputs
                    .get(name)
                    .is_some_and(|p| p.domain != 0 && !p.vector)
                || self
                    .inputs
                    .binary_search_by(|(input, _)| input.cmp(name))
                    .is_err()
            {
                return Err("invalid XSPICE scalar input event time".into());
            }
        }
        for (name, v) in &self.vector_times {
            if !named_shape(&ports.inputs, name, 1, true, v.len())
                || v.iter().flatten().any(|t| !event_time(*t, accepted))
                || self
                    .inputs
                    .binary_search_by(|(input, _)| input.cmp(name))
                    .is_err()
            {
                return Err("invalid XSPICE vector input event history".into());
            }
        }
        if !revision_valid(
            &self.string_revisions,
            &ctx.string_params,
            self.next_string_revision,
        ) || !revision_valid(
            &self.real_revisions,
            &ctx.real_vector_params,
            self.next_real_revision,
        ) {
            return Err("invalid XSPICE parameter cache revision".into());
        }
        for (_, history) in &self.histories {
            let mut previous = None;
            for (t, v) in history {
                let time = f64::from_bits(*t);
                if !time.is_finite()
                    || time > f64::from_bits(self.time)
                    || previous.is_some_and(|prior| prior >= time)
                    || v.iter().any(|v| !f64::from_bits(*v).is_finite())
                {
                    return Err("invalid XSPICE transient history".into());
                }
                previous = Some(time);
            }
        }
        for (name, v) in &self.input_transitions {
            if !named_shape(&ports.inputs, name, 0, true, v.len())
                || v.iter().flatten().any(|t| !transition_valid(t))
            {
                return Err("invalid XSPICE analog input transition metadata".into());
            }
        }
        for (name, v) in &self.output_transitions {
            if !ports.outputs.get(name).is_some_and(|p| p.domain == 0) || !transition_valid(v) {
                return Err("invalid XSPICE analog output transition metadata".into());
            }
        }
        for (name, (when, _)) in &self.inertial {
            let time = f64::from_bits(*when);
            if !named_shape(&ports.outputs, name, 1, false, 1)
                || !time.is_finite()
                || (time < 0.0 && time != -1.0)
            {
                return Err("invalid XSPICE inertial output state".into());
            }
        }
        for &(row, col, v) in self.stamps.iter().chain(&self.static_stamps) {
            if row >= matrix_size || col >= matrix_size || !f64::from_bits(v).is_finite() {
                return Err("invalid XSPICE retained matrix contribution".into());
            }
        }
        for &(row, v) in self.rhs.iter().chain(&self.static_rhs) {
            if row >= matrix_size || !f64::from_bits(v).is_finite() {
                return Err("invalid XSPICE retained RHS contribution".into());
            }
        }
        Ok(())
    }

    fn budget(&self, limits: ContextCheckpointLimits) -> Result<(), String> {
        let mut b = Budget::new(limits);
        b.add(self.state.len())?;
        b.add(self.state_prev.len())?;
        b.add(self.int_state.len())?;
        // Validate ordering before map reconstruction; serde maps otherwise
        // silently accept duplicate keys. The same accounting runs pre-capture.
        macro_rules! bank {
            ($field:ident, $f:expr) => {{
                ordered(&self.$field)?;
                b.map(self.$field.iter().map(|(k, v)| (k, v)), $f)?;
            }};
        }
        bank!(inputs, |b, v| port_size!(v, PortImage, b));
        bank!(outputs, |b, v| port_size!(v, PortImage, b));
        bank!(committed, |b, v| b.add(v.len()));
        bank!(other, |b, v| {
            b.add(v.len())?;
            b.add(v.len())
        });
        bank!(input_times, |b, _| b.add(1));
        bank!(vector_times, |b, v| b.add(v.len()));
        bank!(string_revisions, |b, _| b.add(1));
        bank!(real_revisions, |b, _| b.add(1));
        bank!(histories, |b, v| {
            b.add(v.len())?;
            for (_, values) in v {
                b.add(values.len())?;
            }
            Ok(())
        });
        bank!(input_transitions, |b, v| {
            for _ in v {
                b.add(4)?;
            }
            Ok(())
        });
        bank!(output_transitions, |b, _| b.add(4));
        bank!(inertial, |b, _| b.add(2));
        for v in [&self.stamps, &self.static_stamps] {
            for _ in v {
                b.add(3)?;
            }
        }
        for v in [&self.rhs, &self.static_rhs] {
            for _ in v {
                b.add(2)?;
            }
        }
        Ok(())
    }
}
