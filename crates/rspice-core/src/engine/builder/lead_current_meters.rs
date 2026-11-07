//! LTspice reads the current of a diode, a current source, a voltage-
//! controlled current source or a switch, `I(D1)`, in behavioral,
//! switch-control and state-machine expressions. None of these has a branch
//! current of its own, so a zero-volt meter goes in series with its first
//! terminal and the device's name reads the meter's branch. LTspice reports a
//! voltage-controlled switch's current from its second terminal to its first,
//! the other way round from every other device, and that meter is turned to
//! match. Xyce has no such lead current, and its dialect keeps refusing it.

use super::*;

/// Puts a meter in series with each branchless device an expression reads
/// the current of, and returns each `(device, meter)` name pair.
pub(super) fn materialize(
    elements: &mut Vec<Element>,
    known_device_names: &mut HashSet<String>,
    max_elements: usize,
) -> Result<Vec<(String, String)>, SimulationError> {
    let operands = resistor_currents::current_reads(elements);
    if operands.is_empty() {
        return Ok(Vec::new());
    }
    let mut nodes = elements
        .iter()
        .flat_map(|element| element.nodes.iter())
        .map(|node| node.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    let mut meters = Vec::new();
    let mut pairs = Vec::new();
    for element in elements.iter_mut() {
        if element.nodes.len() != 2 || !operands.contains(&element.name.to_ascii_uppercase()) {
            continue;
        }
        let reversed = match &element.kind {
            ElementKind::Diode { .. }
            | ElementKind::BehavioralCurrent { .. }
            | ElementKind::CurrentSource(_)
            | ElementKind::Vccs { .. }
            | ElementKind::ISwitch { .. } => false,
            ElementKind::VSwitch { .. } => true,
            _ => continue,
        };
        let private = unused_name(&format!("{}.__LEAD", element.name), &mut nodes);
        let meter = unused_name(&format!("{}.__LEAD", element.name), known_device_names);
        let external = std::mem::replace(&mut element.nodes[0], private.clone());
        meters.push(Element {
            name: meter.clone(),
            kind: ElementKind::VoltageSource(SourceSpec::Dc(0.0)),
            nodes: if reversed {
                vec![private, external]
            } else {
                vec![external, private]
            },
            provenance: Default::default(),
        });
        pairs.push((element.name.clone(), meter));
    }
    ResourceLimitError::ensure(
        ResourceKind::FlattenedElements,
        elements.len().saturating_add(meters.len()),
        max_elements,
    )?;
    elements.extend(meters);
    Ok(pairs)
}

/// `base`, or `base` with the first numeric suffix `taken` lacks, recorded
/// in `taken` by its upper-case spelling.
fn unused_name(base: &str, taken: &mut HashSet<String>) -> String {
    let mut name = base.to_owned();
    let mut suffix = 1usize;
    while !taken.insert(name.to_ascii_uppercase()) {
        suffix += 1;
        name = format!("{base}{suffix}");
    }
    name
}
