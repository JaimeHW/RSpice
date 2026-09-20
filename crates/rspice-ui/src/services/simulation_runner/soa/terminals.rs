//! External body/back-gate roles resolved by the actual model implementation.
use super::*;
use rspice_core::{Netlist, circuit::MosTerminalLayout, engine::Engine};

pub(super) type MosLayouts = HashMap<String, MosTerminalLayout>;

pub(super) fn resolve(
    netlist: &Netlist,
    elements: &[Element],
    config: &SoaRunConfig,
    engine: &Engine,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<MosLayouts> {
    if !config
        .rules
        .iter()
        .any(|rule| rule.parameter.requires_mos_layout())
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
            layouts.insert(element.name.clone(), layout);
        }
    }
    Ok(layouts)
}
