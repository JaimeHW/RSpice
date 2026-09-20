//! External transistor terminal roles resolved by the actual model implementation.
use super::*;
use rspice_core::{Netlist, engine::Engine};

#[derive(Clone, Copy, Default)]
pub(super) struct TerminalLayout {
    pub body: Option<usize>,
    pub back_gate: Option<usize>,
    pub substrate: Option<usize>,
    pub intrinsic_voltages: u128,
}
pub(super) type TerminalLayouts = HashMap<String, TerminalLayout>;

pub(super) fn resolve(
    netlist: &Netlist,
    elements: &[Element],
    config: &SoaRunConfig,
    engine: &Engine,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TerminalLayouts> {
    if !config.rules.iter().any(|rule| {
        rule.parameter.requires_terminal_layout()
            || rule.voltage_basis == SoaVoltageBasis::IntrinsicNodes
    }) {
        return Ok(HashMap::new());
    }
    let circuit = engine
        .build_circuit_with_abort(netlist, abort)
        .map_err(|error| ServiceRunError::from_core("SOA terminal layout", error))?;
    let intrinsic = if config
        .rules
        .iter()
        .any(|r| r.voltage_basis == SoaVoltageBasis::IntrinsicNodes)
    {
        circuit.intrinsic_voltage_catalog()
    } else {
        HashMap::new()
    };
    let mut layouts = HashMap::new();
    for (index, element) in elements.iter().enumerate() {
        poll_periodically(abort, index)?;
        if let Some(layout) = circuit.mos_terminal_layout(&element.name) {
            layouts.insert(
                element.name.clone(),
                TerminalLayout {
                    body: layout.body,
                    back_gate: layout.back_gate,
                    substrate: None,
                    intrinsic_voltages: 0,
                },
            );
        } else if let Some(layout) = circuit.bjt_terminal_layout(&element.name) {
            layouts.insert(
                element.name.clone(),
                TerminalLayout {
                    substrate: layout.substrate,
                    ..Default::default()
                },
            );
        }
        if let Some(labels) = intrinsic.get(&element.name) {
            let layout = layouts.entry(element.name.clone()).or_default();
            for parameter in [
                SoAParameter::Vgs,
                SoAParameter::Vds,
                SoAParameter::Vgd,
                SoAParameter::Vbs,
                SoAParameter::Vbd,
                SoAParameter::Vgb,
                SoAParameter::Ves,
                SoAParameter::Ved,
                SoAParameter::Vge,
                SoAParameter::VbodyBackgate,
                SoAParameter::Vbe,
                SoAParameter::Vce,
                SoAParameter::Vbc,
                SoAParameter::Vcsub,
                SoAParameter::Vbsub,
                SoAParameter::Vesub,
                SoAParameter::Vak,
            ] {
                if labels
                    .iter()
                    .any(|label| Some(label.as_str()) == parameter.intrinsic_voltage_parameter())
                {
                    layout.intrinsic_voltages |= 1u128 << parameter as u32;
                }
            }
        }
    }
    Ok(layouts)
}
