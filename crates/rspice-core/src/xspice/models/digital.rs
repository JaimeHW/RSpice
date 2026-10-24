//! Digital Code Models - Gates, Flip-Flops, Memory
//!
//! Provides digital logic primitives for mixed-signal simulation.

use crate::xspice::{
    CmContext, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue, ParamSpec,
    PortSpec, PortType,
};

mod cosim;
mod gates;
mod lookup;
mod memory;
mod oscillators;
mod process;
mod pspice_stim;
mod sequential;
mod sources;
mod vcd_source;

pub use cosim::DigitalCosim;
pub(crate) use cosim::unhosted_cosim_refusal;
pub use gates::{
    DigitalAnd, DigitalBuffer, DigitalInverter, DigitalNand, DigitalNor, DigitalOpenCollector,
    DigitalOpenEmitter, DigitalOr, DigitalPulldown, DigitalPullup, DigitalTristate, DigitalXnor,
    DigitalXor,
};
pub use lookup::{DigitalGenericLookupTable, DigitalLookupTable};
pub use memory::DigitalRam;
pub use oscillators::{DigitalOscillator, DigitalPwmOscillator, NumericallyControlledOscillator};
pub use process::DigitalProcess;
pub(crate) use pspice_stim::PspiceDigitalStimulus;
pub use sequential::{
    DFlipFlop, DLatch, DigitalFrequencyDivider, JkFlipFlop, SrFlipFlop, SrLatch, TFlipFlop,
};
pub use sources::{DigitalSource, DigitalStateMachine};
pub(crate) use vcd_source::{DigitalVcdSource, vcd_source_digest, vcd_source_file};

/// The latest input event on any element of `ports` that the clock has
/// reached ([`CmContext::is_due`]): the event an output responding to those
/// inputs is owed its delay after.
fn latest_input_event_time(ctx: &CmContext, ports: &[&str]) -> Option<crate::Value> {
    ports
        .iter()
        .flat_map(|&port| {
            let width = ctx.input_digital_vector_values(port).map_or(0, <[_]>::len);
            ctx.input_digital_event_time(port).into_iter().chain(
                (0..width)
                    .filter_map(move |index| ctx.input_digital_vector_event_time(port, index)),
            )
        })
        .filter(|time| time.is_finite() && ctx.is_due(*time))
        .max_by(|a, b| a.total_cmp(b))
}

/// `delay` measured from the input event that caused an output, `cause`,
/// rather than from the timepoint evaluating it.
///
/// The queue delivers an event at the timepoint that reaches it, and the
/// breakpoint schedule can land that timepoint a hair before the event's
/// instant or a restart step after it. The output is owed its delay after the
/// event either way, as the gates date theirs. An event at the timepoint
/// itself leaves the delay as it is.
fn delay_from_input_event(
    ctx: &CmContext,
    cause: Option<crate::Value>,
    delay: crate::Value,
) -> crate::Value {
    match cause {
        Some(at) if at != ctx.time => at + delay - ctx.time,
        _ => delay,
    }
}
