//! Elaboration of the output of an E/G `FREQ` source.
//!
//! The output runs as a code model on the XSPICE host that follows one of two
//! private nodes the parser drives: operating points pass the source's input
//! through unscaled, as QSPICE and LTspice do, and the transient follows the
//! rational fit of the table. Small-signal analyses answer the owner with the
//! table itself, so the linearization here is the operating point's.

use super::*;
use crate::xspice::{
    CmContext, CmResult, CodeModel, ParamSpec, PortSpec, PortType, XspiceInstance,
};
use std::sync::{Arc, OnceLock};

/// Build the output `element` stands for into `circuit`; `nodes` are the
/// input and the fitted response.
pub(super) fn build(
    circuit: &mut CircuitData,
    element: &Element,
    nodes: (&str, &str),
    temperature: Value,
) -> Result<(), SimulationError> {
    let (input, fitted) = nodes;
    let model: Arc<dyn CodeModel> = Arc::new(FrequencyResponseOutput);
    let ports = vec![
        XspicePort::Analog(input.to_string()),
        XspicePort::Analog(fitted.to_string()),
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

struct FrequencyResponseOutput;

impl FrequencyResponseOutput {
    /// The port the output follows in the present analysis: the fit through
    /// the transient's steps, the input everywhere else, the operating point
    /// the transient starts from included.
    fn followed(ctx: &CmContext) -> &'static str {
        if ctx.is_transient() && !ctx.is_dc_operating_point() {
            "fitted"
        } else {
            "in"
        }
    }
}

impl CodeModel for FrequencyResponseOutput {
    fn name(&self) -> &str {
        "freq"
    }

    fn description(&self) -> &str {
        "Output of a source known by a measured frequency response"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::Voltage),
                PortSpec::input("fitted", PortType::Voltage),
                PortSpec::output("out", PortType::Voltage),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        &[]
    }

    fn probe_writes_only_outputs(&self) -> bool {
        true
    }

    fn has_memoryless_linear_transient_stamp(&self) -> bool {
        true
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let value = ctx.input(Self::followed(ctx));
        ctx.set_output("out", value);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        vec![(Self::followed(ctx).to_string(), 1.0)]
    }
}
