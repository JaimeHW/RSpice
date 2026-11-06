//! Elaboration of the LTspice special-function devices.
//!
//! Each instance runs as an analog behavioral model on the code-model host.
//! Inputs are differential voltage ports against the device common. Each
//! output is a current port on (output, common) whose Norton linearization
//! reads its own terminal voltage through a sensing port, so clamps and
//! output impedances stamp as conductances in every analysis. Output
//! capacitance is an ordinary capacitor to the common.

use super::*;
use crate::device::special_function::{Ota, SpecialFunctionError};
use crate::netlist::SpecialFunction;
use crate::xspice::{
    CmContext, CmResult, CodeModel, ParamSpec, PortSpec, PortType,
    XspiceCheckpointSupport, XspiceInstance, XspiceSmallSignalDescriptor,
};
use std::sync::Arc;

/// The terminals of one instance, with every terminal tied to the common
/// reported as unused.
struct Terminals<'a> {
    nodes: &'a [String],
}

impl<'a> Terminals<'a> {
    fn new(element: &'a Element) -> Result<Self, SimulationError> {
        if element.nodes.len() != 8 {
            return Err(SimulationError::Circuit(format!(
                "special function {} needs eight terminals, found {}",
                element.name,
                element.nodes.len()
            )));
        }
        Ok(Self {
            nodes: &element.nodes,
        })
    }

    /// Terminal `pin` (1-based).
    fn node(&self, pin: usize) -> &'a str {
        &self.nodes[pin - 1]
    }

    fn common(&self) -> &'a str {
        self.node(8)
    }

    fn used(&self, pin: usize) -> bool {
        !self.node(pin).eq_ignore_ascii_case(self.common())
    }

    /// The voltage port of terminal `pin` against the common.
    fn sensed(&self, pin: usize) -> XspicePort {
        XspicePort::DifferentialVoltage {
            pos: self.node(pin).to_string(),
            neg: self.common().to_string(),
        }
    }

    fn driven(&self, pin: usize) -> XspicePort {
        XspicePort::DifferentialCurrent {
            pos: self.node(pin).to_string(),
            neg: self.common().to_string(),
        }
    }
}

fn device_error(element: &Element, error: SpecialFunctionError) -> SimulationError {
    SimulationError::Circuit(format!("{}: {error}", element.name))
}

/// Build one special-function instance into `circuit`.
pub(super) fn build(
    circuit: &mut CircuitData,
    element: &Element,
    function: SpecialFunction,
    params: &[(String, Value)],
    temperature: Value,
) -> Result<(), SimulationError> {
    let terminals = Terminals::new(element)?;
    let (model, ports, capacitances): (Arc<dyn CodeModel>, Vec<XspicePort>, Vec<(usize, Value)>) =
        match function {
            SpecialFunction::Ota => {
                let ota = Ota::from_params(params).map_err(|error| device_error(element, error))?;
                for pin in [5, 6] {
                    if terminals.used(pin) {
                        return Err(SimulationError::Circuit(format!(
                            "{}: OTA terminal {pin} must be tied to the common terminal",
                            element.name
                        )));
                    }
                }
                let multiplier = terminals.used(3) || terminals.used(4);
                let ports = vec![
                    XspicePort::DifferentialVoltage {
                        pos: terminals.node(2).to_string(),
                        neg: terminals.node(1).to_string(),
                    },
                    if multiplier {
                        XspicePort::DifferentialVoltage {
                            pos: terminals.node(3).to_string(),
                            neg: terminals.node(4).to_string(),
                        }
                    } else {
                        XspicePort::Null
                    },
                    terminals.sensed(7),
                    terminals.driven(7),
                ];
                let capacitances = vec![(7, ota.cout)];
                (Arc::new(OtaModel { ota, multiplier }), ports, capacitances)
            }
            other => {
                return Err(SimulationError::Circuit(format!(
                    "{}: LTspice special function {} is not supported",
                    element.name,
                    other.ltspice_name()
                )));
            }
        };

    let connections = coerce_xspice_connections(
        circuit,
        model.ports(),
        &ports,
        &element.name,
        model.name(),
    )?;
    let mut instance =
        XspiceInstance::new(element.name.clone(), model, connections, &[], &[], &[], &[])
            .map_err(|error| {
                xspice_creation_error(format!("Failed to create {}", element.name), error)
            })?;
    instance.set_temperature(temperature);
    assign_xspice_output_branches(circuit, &mut instance, &element.name)?;
    instance.init().map_err(|error| {
        SimulationError::Circuit(format!("Failed to initialize {}: {error}", element.name))
    })?;
    circuit.add_xspice_instance(instance);

    let common = node_id(circuit, terminals.common());
    for (pin, capacitance) in capacitances {
        if capacitance > 0.0 && terminals.used(pin) {
            let node = node_id(circuit, terminals.node(pin));
            circuit.capacitors.add_internal(
                format!("{}#cout{pin}", element.name),
                node,
                common,
                capacitance,
            );
        }
    }
    Ok(())
}

fn node_id(circuit: &mut CircuitData, name: &str) -> crate::NodeId {
    if name == "0" {
        0
    } else {
        circuit.get_or_create_node(name)
    }
}

//=============================================================================
// OTA
//=============================================================================

struct OtaModel {
    ota: Ota,
    multiplier: bool,
}

impl OtaModel {
    fn multiplier(&self, ctx: &CmContext) -> Value {
        if self.multiplier {
            ctx.input("mult")
        } else {
            1.0
        }
    }

    /// The current into the output terminal and its partials with respect
    /// to the input, multiplier and output-voltage ports.
    fn output(&self, ctx: &CmContext) -> (Value, [Value; 3]) {
        let drive = self.ota.drive(ctx.input("in"), self.multiplier(ctx));
        let v = ctx.input("vout");
        let (clamp, clamp_conductance) = self.ota.clamp(v);
        let load = self.ota.rout.map_or(0.0, |rout| 1.0 / rout);
        (
            clamp + load * v - drive.current,
            [-drive.d_input, -drive.d_multiplier, clamp_conductance + load],
        )
    }
}

impl CodeModel for OtaModel {
    fn name(&self) -> &str {
        "OTA"
    }

    fn description(&self) -> &str {
        "LTspice transconductance amplifier"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("in", PortType::DifferentialVoltage),
                PortSpec::input("mult", PortType::DifferentialVoltage).nullable(),
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
        let (current, _) = self.output(ctx);
        ctx.set_output("out", current);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        let (_, [d_input, d_multiplier, d_output]) = self.output(ctx);
        let mut partials = vec![("in".to_string(), d_input), ("vout".to_string(), d_output)];
        if self.multiplier {
            partials.push(("mult".to_string(), d_multiplier));
        }
        partials
    }
}
