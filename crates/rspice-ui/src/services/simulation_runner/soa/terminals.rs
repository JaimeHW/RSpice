//! External transistor terminal roles resolved by the actual model implementation.
use super::*;
use rspice_core::{Netlist, engine::Engine};

#[derive(Clone, Copy, Default)]
pub(super) struct TerminalLayout {
    pub body: Option<usize>,
    pub back_gate: Option<usize>,
    pub substrate: Option<usize>,
}
pub(super) type TerminalLayouts = HashMap<String, TerminalLayout>;

pub(super) fn resolve(
    netlist: &Netlist,
    elements: &[Element],
    config: &SoaRunConfig,
    engine: &Engine,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TerminalLayouts> {
    if !config
        .rules
        .iter()
        .any(|rule| rule.parameter.requires_terminal_layout())
    {
        return Ok(HashMap::new());
    }
    let circuit = engine
        .build_circuit_with_abort(netlist, abort)
        .map_err(|error| ServiceRunError::from_core("SOA terminal layout", error))?;
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
    }
    Ok(layouts)
}
