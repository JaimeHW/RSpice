//! Semantic filtering for Xyce `.PREPROCESS REMOVEUNUSED`.
//!
//! Lexical preprocessing remains in the parser, while this module owns the
//! AST-level and post-flatten rules shared by parsing and hierarchy flattening.

use std::collections::{HashMap, HashSet};

use crate::abort_signal::AbortSignal;

use super::{
    Element, ElementKind, ElementProvenance, GeneratedPassiveHelperRole, ParseWithAbortError,
    RemoveUnusedDeviceType, RemoveUnusedPolicy, ensure_parse_not_aborted, poll_parse_abort,
};

pub(crate) fn filter_elements_with_abort(
    elements: &[Element],
    policy: &RemoveUnusedPolicy,
    abort: &dyn AbortSignal,
) -> Result<Vec<Element>, ParseWithAbortError> {
    let mut series_helpers_by_owner = HashMap::new();
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        if let ElementProvenance::GeneratedPassiveHelper {
            owner,
            role: GeneratedPassiveHelperRole::SeriesResistance,
        } = &element.provenance
        {
            series_helpers_by_owner.insert(owner.to_ascii_uppercase(), index);
        }
    }

    let mut rejected_owners = HashSet::new();
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, elements.len().saturating_add(index))?;
        if rejects_element(element, policy)
            || rejects_series_rewritten_passive(element, elements, &series_helpers_by_owner, policy)
        {
            rejected_owners.insert(element.name.to_ascii_uppercase());
        }
    }

    let mut filtered = Vec::with_capacity(elements.len().saturating_sub(rejected_owners.len()));
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(
            abort,
            elements.len().saturating_mul(2).saturating_add(index),
        )?;
        let rejected = match &element.provenance {
            ElementProvenance::Authored => {
                rejected_owners.contains(&element.name.to_ascii_uppercase())
            }
            ElementProvenance::GeneratedPassiveHelper { owner, .. } => {
                rejected_owners.contains(&owner.to_ascii_uppercase())
            }
        };
        if !rejected {
            filtered.push(element.clone());
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(filtered)
}

pub(crate) fn designator_type(designator: char) -> Option<(RemoveUnusedDeviceType, usize)> {
    match designator.to_ascii_uppercase() {
        'C' => Some((RemoveUnusedDeviceType::Capacitor, 2)),
        'D' => Some((RemoveUnusedDeviceType::Diode, 2)),
        'I' => Some((RemoveUnusedDeviceType::CurrentSource, 2)),
        'L' => Some((RemoveUnusedDeviceType::Inductor, 2)),
        'M' => Some((RemoveUnusedDeviceType::Mosfet, 3)),
        'Q' => Some((RemoveUnusedDeviceType::Bjt, 3)),
        'R' => Some((RemoveUnusedDeviceType::Resistor, 2)),
        'V' => Some((RemoveUnusedDeviceType::VoltageSource, 2)),
        _ => None,
    }
}

fn rejects_element(element: &Element, policy: &RemoveUnusedPolicy) -> bool {
    if !matches!(element.provenance, ElementProvenance::Authored) {
        return false;
    }
    let Some((device_type, compared_nodes)) = element_device_type(element) else {
        return false;
    };
    if !policy.contains(device_type) || element.nodes.len() < compared_nodes {
        return false;
    }
    element.nodes[..compared_nodes]
        .windows(2)
        .all(|pair| pair[0].eq_ignore_ascii_case(&pair[1]))
}

fn qualified_element_leaf(name: &str) -> &str {
    name.rsplit(['.', ':']).next().unwrap_or(name)
}

fn rejects_series_rewritten_passive(
    element: &Element,
    elements: &[Element],
    series_helpers_by_owner: &HashMap<String, usize>,
    policy: &RemoveUnusedPolicy,
) -> bool {
    if !matches!(element.provenance, ElementProvenance::Authored) {
        return false;
    }
    let Some((device_type, compared_nodes)) = element_device_type(element) else {
        return false;
    };
    if compared_nodes != 2
        || !policy.contains(device_type)
        || !matches!(
            &element.kind,
            ElementKind::Resistor { .. }
                | ElementKind::Capacitor { .. }
                | ElementKind::Inductor { .. }
                | ElementKind::JilesAthertonInductor { .. }
        )
        || element.nodes.len() < 2
    {
        return false;
    }

    let Some(series) = series_helpers_by_owner
        .get(&element.name.to_ascii_uppercase())
        .and_then(|index| elements.get(*index))
    else {
        return false;
    };
    matches!(&series.kind, ElementKind::Resistor { .. })
        && series.nodes.len() >= 2
        && series.nodes[1].eq_ignore_ascii_case(&element.nodes[0])
        && series.nodes[0].eq_ignore_ascii_case(&element.nodes[1])
}

fn element_device_type(element: &Element) -> Option<(RemoveUnusedDeviceType, usize)> {
    let leaf = qualified_element_leaf(&element.name);
    let (selection, compared_nodes, designator) = match &element.kind {
        ElementKind::Capacitor { .. } => (RemoveUnusedDeviceType::Capacitor, 2, 'C'),
        ElementKind::Diode { .. } => (RemoveUnusedDeviceType::Diode, 2, 'D'),
        ElementKind::CurrentSource(_) | ElementKind::CurrentSourceDeferred(_) => {
            (RemoveUnusedDeviceType::CurrentSource, 2, 'I')
        }
        ElementKind::Inductor { .. } | ElementKind::JilesAthertonInductor { .. } => {
            (RemoveUnusedDeviceType::Inductor, 2, 'L')
        }
        ElementKind::Mosfet { .. } => (RemoveUnusedDeviceType::Mosfet, 3, 'M'),
        ElementKind::Bjt { .. } => (RemoveUnusedDeviceType::Bjt, 3, 'Q'),
        ElementKind::Resistor { .. } => (RemoveUnusedDeviceType::Resistor, 2, 'R'),
        ElementKind::VoltageSource(_) | ElementKind::VoltageSourceDeferred(_) => {
            (RemoveUnusedDeviceType::VoltageSource, 2, 'V')
        }
        _ => return None,
    };
    leaf.chars()
        .next()
        .is_some_and(|first| first.eq_ignore_ascii_case(&designator))
        .then_some((selection, compared_nodes))
}
