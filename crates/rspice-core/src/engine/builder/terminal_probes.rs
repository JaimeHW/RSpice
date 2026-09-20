//! Ideal terminal-current meters, materialized after topology simplification.

use super::*;

pub(super) fn materialize(
    netlist: &Netlist,
    elements: &mut Vec<Element>,
    known_device_names: &mut HashSet<String>,
    max_elements: usize,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let probes = &netlist.ast_overlay.terminal_current_probes;
    if probes.is_empty() {
        return Ok(());
    }
    ResourceLimitError::ensure(
        ResourceKind::FlattenedElements,
        elements.len().saturating_add(probes.len()),
        max_elements,
    )?;
    let devices = elements
        .iter()
        .enumerate()
        .map(|(index, element)| (element.name.to_ascii_uppercase(), index))
        .collect::<HashMap<_, _>>();
    let mut nodes = elements
        .iter()
        .flat_map(|element| element.nodes.iter())
        .map(|node| node.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let mut names = known_device_names.clone();
    let mut terminals = HashSet::new();
    let mut targets = Vec::with_capacity(probes.len());
    // Validate the complete overlay before changing any electrical connection.
    for probe in probes {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let fail = |detail: &str| {
            SimulationError::Circuit(format!(
                "Terminal current probe '{}': {detail}",
                probe.source_name
            ))
        };
        let index = *devices
            .get(&probe.device.to_ascii_uppercase())
            .ok_or_else(|| fail("target device is absent after hierarchy/topology expansion"))?;
        if probe.terminal >= elements[index].nodes.len() {
            return Err(fail(
                "target terminal is outside the device's authored node list",
            ));
        }
        if !terminals.insert((index, probe.terminal)) {
            return Err(fail("target terminal already has a current probe"));
        }
        if !probe.source_name.to_ascii_uppercase().starts_with('V')
            || probe.source_name.chars().any(char::is_whitespace)
            || !names.insert(probe.source_name.to_ascii_uppercase())
        {
            return Err(fail("source name must be a unique voltage-source name"));
        }
        if probe.node_name.is_empty()
            || probe.node_name.chars().any(char::is_whitespace)
            || netlist.ground_policy().canonical_node(&probe.node_name) == "0"
            || !nodes.insert(probe.node_name.to_ascii_uppercase())
        {
            return Err(fail("private node must be unique and cannot be ground"));
        }
        targets.push(index);
    }
    for (probe, index) in probes.iter().zip(targets) {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let external = std::mem::replace(
            &mut elements[index].nodes[probe.terminal],
            probe.node_name.clone(),
        );
        elements.push(Element {
            name: probe.source_name.clone(),
            kind: ElementKind::VoltageSource(SourceSpec::Dc(0.0)),
            nodes: vec![external, probe.node_name.clone()],
            provenance: Default::default(),
        });
    }
    *known_device_names = names;
    Ok(())
}
