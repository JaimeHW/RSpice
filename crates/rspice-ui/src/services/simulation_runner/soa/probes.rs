//! Total current at authored transistor pins, including accepted charge current.

use super::*;
use rspice_core::netlist::{Netlist, SaveSignal, TerminalCurrentProbe};
use std::collections::HashSet;

pub(super) fn register(
    netlist: &mut Netlist,
    elements: &[Element],
    resolved: &[(usize, SoADefinition)],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<HashMap<(usize, SoAParameter), String>> {
    let names = elements
        .iter()
        .map(|e| e.name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let nodes = elements
        .iter()
        .flat_map(|e| &e.nodes)
        .map(|n| n.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let mut probes = HashMap::new();
    let mut next = 0usize;
    for (index, definition) in resolved {
        for limit in &definition.limits {
            ensure_not_aborted(abort)?;
            let parameter = limit.parameter.base_parameter();
            let Some(terminal) = rules::current_terminal(parameter) else {
                continue;
            };
            if probes.contains_key(&(*index, parameter)) {
                continue;
            }
            let (source_name, node_name) = loop {
                let source = format!("V__SOA_CURRENT_{next}");
                let node = format!("__SOA_CURRENT_{next}");
                next += 1;
                if !names.contains(&source) && !nodes.contains(&node) {
                    break (source, node);
                }
                ensure_not_aborted(abort)?;
            };
            netlist.add_terminal_current_probe(TerminalCurrentProbe {
                device: elements[*index].name.clone(),
                terminal,
                source_name: source_name.clone(),
                node_name,
            });
            netlist
                .saves
                .signals
                .push(SaveSignal::Current(source_name.clone()));
            probes.insert((*index, parameter), source_name);
        }
    }
    Ok(probes)
}
