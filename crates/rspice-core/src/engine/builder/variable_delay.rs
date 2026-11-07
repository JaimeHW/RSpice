//! Elaboration of a behavioral `delay(x,td[,tdmax])` whose delay varies during
//! the simulation.
//!
//! The delay runs as a code model on the XSPICE host. Its inputs are the
//! private nodes the parser drives with `x`, `td` and `tdmax`; its output is
//! an ideal voltage. The transient keeps the accepted samples of `x`, as far
//! back as `tdmax` when that is given, and reads them linearly at `t-td(t)`,
//! with `x` at time zero before the history starts. The operating point and
//! the AC linearization take `td` at the operating point: the output is `x`
//! there, and `x·e^(-jω·td)` in AC.

use super::*;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortSpec, PortType,
    XspiceInstance,
};
use num_complex::Complex64;
use std::sync::{Arc, OnceLock};

/// The model's state slots. The three accepted samples before the present
/// one locate corners of `x` for the breakpoints; the partials are those of
/// the latest evaluation.
mod slot {
    pub(super) const TIME: usize = 0;
    pub(super) const INPUT: usize = 1;
    pub(super) const EARLIER_TIME: usize = 2;
    pub(super) const EARLIER_INPUT: usize = 3;
    pub(super) const EARLIEST_TIME: usize = 4;
    pub(super) const EARLIEST_INPUT: usize = 5;
    pub(super) const PARTIAL_INPUT: usize = 6;
    pub(super) const PARTIAL_DELAY: usize = 7;
    /// How many transient samples have been taken, the present one included.
    pub(super) const COUNT: usize = 8;
}

/// Key of the input history in the instance context. Each sample is
/// `[time, x, previous time, previous x]`, so the sample at or after a
/// time brackets it.
const HISTORY: &str = "delay";

/// A slope change, relative to the larger slope, that makes a corner of `x`
/// whose delayed arrival is a breakpoint.
const CORNER_RELATIVE: Value = 0.5;
/// The smallest change a corner makes over its two segments, relative to
/// the input's magnitude and never under a microvolt.
const CORNER_ABSOLUTE: Value = 1e-6;

/// Build the variable delay `element` stands for into `circuit`.
pub(super) fn build(
    circuit: &mut CircuitData,
    element: &Element,
    owner: &str,
    nodes: (&str, &str, Option<&str>),
    temperature: Value,
) -> Result<(), SimulationError> {
    let (input, delay, maximum) = nodes;
    let model: Arc<dyn CodeModel> = Arc::new(VariableDelay {
        owner: owner.to_string(),
        bounded: maximum.is_some(),
    });
    let ports = vec![
        XspicePort::Analog(input.to_string()),
        XspicePort::Analog(delay.to_string()),
        maximum.map_or(XspicePort::Null, |node| {
            XspicePort::Analog(node.to_string())
        }),
        XspicePort::Analog(element.nodes[0].clone()),
    ];
    let connections =
        coerce_xspice_connections(circuit, model.ports(), &ports, &element.name, model.name())?;
    let mut instance =
        XspiceInstance::new(element.name.clone(), model, connections, &[], &[], &[], &[]).map_err(
            |error| xspice_creation_error(format!("Failed to create {}", element.name), error),
        )?;
    instance.set_temperature(temperature);
    assign_xspice_output_branches(circuit, &mut instance, &element.name)?;
    instance.init().map_err(|error| {
        SimulationError::Circuit(format!("Failed to initialize {}: {error}", element.name))
    })?;
    circuit.add_xspice_instance(instance);
    Ok(())
}

struct VariableDelay {
    /// The behavioral source whose expression wrote the `delay()` call.
    owner: String,
    /// Whether the `maximum` port carries `tdmax`.
    bounded: bool,
}

impl VariableDelay {
    fn refuse(&self, message: String) -> CmError {
        CmError::EvaluationError(format!(
            "behavioral source '{}': delay() {message}",
            self.owner
        ))
    }

    /// The delay at this evaluation, held inside `[0, tdmax]`, and `tdmax`.
    ///
    /// Newton iterates may stray outside the bounds; an accepted transient
    /// point that does is refused, as QSPICE refuses it.
    fn delay(&self, ctx: &CmContext) -> CmResult<(Value, Option<Value>)> {
        let delay = ctx.input("delay");
        let maximum = self.bounded.then(|| ctx.input("maximum"));
        if ctx.is_transient() && ctx.evaluation_phase() == EvaluationPhase::AcceptedStep {
            if !(delay.is_finite() && delay >= 0.0) {
                return Err(self.refuse(format!("delay {delay} is not a non-negative time")));
            }
            if let Some(maximum) = maximum {
                if !(maximum.is_finite() && maximum > 0.0) {
                    return Err(
                        self.refuse(format!("maximum delay {maximum} is not a positive time"))
                    );
                }
                if delay > maximum {
                    return Err(self.refuse(format!("delay {delay} exceeds its maximum {maximum}")));
                }
            }
        }
        let maximum = maximum.filter(|maximum| maximum.is_finite() && *maximum > 0.0);
        let delay = if delay.is_finite() {
            delay.max(0.0)
        } else {
            0.0
        };
        Ok((maximum.map_or(delay, |maximum| delay.min(maximum)), maximum))
    }

    /// Ask for a breakpoint where the corner of `x` at the middle of the
    /// three accepted samples before this one arrives.
    fn request_corner_breakpoint(ctx: &mut CmContext, delay: Value) {
        if ctx.state_prev(slot::COUNT) < 3.0 {
            return;
        }
        let points = [
            (
                ctx.state_prev(slot::EARLIEST_TIME),
                ctx.state_prev(slot::EARLIEST_INPUT),
            ),
            (
                ctx.state_prev(slot::EARLIER_TIME),
                ctx.state_prev(slot::EARLIER_INPUT),
            ),
            (ctx.state_prev(slot::TIME), ctx.state_prev(slot::INPUT)),
        ];
        let [(t0, x0), (t1, x1), (t2, x2)] = points;
        if !(t0 < t1 && t1 < t2) {
            return;
        }
        let (before, after) = ((x1 - x0) / (t1 - t0), (x2 - x1) / (t2 - t1));
        let turn = (after - before).abs();
        let scale = x0.abs().max(x1.abs()).max(x2.abs()).max(1.0);
        if turn > CORNER_RELATIVE * before.abs().max(after.abs())
            && turn * (t2 - t0) > CORNER_ABSOLUTE * scale
            && t1 + delay > ctx.time
        {
            ctx.request_breakpoint(t1 + delay);
        }
    }
}

impl CodeModel for VariableDelay {
    fn name(&self) -> &str {
        "delay"
    }

    fn description(&self) -> &str {
        "Behavioral delay whose delay varies during the simulation"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage),
                PortSpec::input("delay", PortType::Voltage),
                PortSpec::input("maximum", PortType::Voltage).nullable(),
                PortSpec::output("out", PortType::Voltage),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(slot::COUNT + 1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input("in");
        let (delay, maximum) = self.delay(ctx)?;
        if !ctx.is_transient() {
            ctx.set_state(slot::PARTIAL_INPUT, 1.0);
            ctx.set_state(slot::PARTIAL_DELAY, 0.0);
            ctx.set_output("out", input);
            return Ok(());
        }

        let time = ctx.time;
        let accepted = ctx.state_prev(slot::COUNT);
        let (previous_time, previous_input) = if accepted == 0.0 {
            (time, input)
        } else {
            (ctx.state_prev(slot::TIME), ctx.state_prev(slot::INPUT))
        };
        let shifted = [
            (slot::TIME, time),
            (slot::INPUT, input),
            (slot::EARLIER_TIME, previous_time),
            (slot::EARLIER_INPUT, previous_input),
            (slot::EARLIEST_TIME, ctx.state_prev(slot::EARLIER_TIME)),
            (slot::EARLIEST_INPUT, ctx.state_prev(slot::EARLIER_INPUT)),
            (slot::COUNT, accepted + 1.0),
        ];
        for (index, value) in shifted {
            ctx.set_state(index, value);
        }
        // A sample older than the longest delay from the last accepted time
        // is never read again, even by a retried step.
        let retention = maximum.map_or(Value::INFINITY, |maximum| maximum + (time - previous_time));
        ctx.record_transient_history(
            HISTORY,
            time,
            vec![time, input, previous_time, previous_input],
            retention,
        );
        Self::request_corner_breakpoint(ctx, delay);

        let target = time - delay;
        let Some(&[later_time, later, earlier_time, earlier]) = ctx
            .transient_history_values_at_or_after(HISTORY, target)
            .and_then(|sample| <&[Value; 4]>::try_from(sample).ok())
        else {
            return Err(self.refuse(format!("has no history at time {target}")));
        };
        // The present sample enters the output, and its partial, only when
        // the delay is shorter than the step.
        let present = (later_time - time).abs() <= Value::EPSILON * time.abs().max(1.0);
        let (value, partial_input, partial_delay) = if later_time > earlier_time {
            let weight = ((target - earlier_time) / (later_time - earlier_time)).clamp(0.0, 1.0);
            let slope = (later - earlier) / (later_time - earlier_time);
            (
                earlier + weight * (later - earlier),
                if present { weight } else { 0.0 },
                -slope,
            )
        } else {
            (later, if present { 1.0 } else { 0.0 }, 0.0)
        };
        ctx.set_state(slot::PARTIAL_INPUT, partial_input);
        ctx.set_state(slot::PARTIAL_DELAY, partial_delay);
        ctx.set_output("out", value);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        vec![
            ("in".to_string(), ctx.state(slot::PARTIAL_INPUT)),
            ("delay".to_string(), ctx.state(slot::PARTIAL_DELAY)),
        ]
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let phase = -std::f64::consts::TAU * frequency * ctx.input("delay");
        vec![("in".to_string(), Complex64::from_polar(1.0, phase))]
    }
}
