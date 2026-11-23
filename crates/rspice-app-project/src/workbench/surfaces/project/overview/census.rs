//! What the design is made of: its devices by kind, flattened through every
//! instance, and how deep and how wide its hierarchy is.
//!
//! Read from the elaborated design the next run netlists — the projection
//! of the plan's configuration over the open drawings — so a count is the
//! one the engine will see, never a guess from the drawing in front. The
//! projection is shared and kept until an input changes, so the census is
//! kept against it and counted again only when the design changes.

use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Weak};

use rspice_design::hierarchy::HierarchyOccurrences;
use rspice_design::projection::{DesignProjection, ProjectedSchematic};

use crate::state::{CellViewRef, ComponentType};
use crate::workbench::app_state::Model;
use crate::workbench::project::configuration_state::count_scan;

use super::super::derived::derived;

/// The kinds a device is counted under, in the order the Overview lists
/// them: the active devices, the passives, then what drives the circuit.
const KINDS: [&str; 16] = [
    "MOSFET",
    "BJT",
    "JFET",
    "MESFET",
    "diode",
    "R",
    "C",
    "L",
    "K",
    "memristor",
    "line",
    "switch",
    "controlled",
    "XSPICE",
    "source",
    "subckt",
];

/// Where a leaf instance of a cell with no schematic is counted: a
/// subcircuit the engine reads from its own netlist or model.
const SUBCIRCUIT: usize = KINDS.len() - 1;

/// What the design is made of.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DesignCensus {
    /// How many levels its deepest instance sits under the top, the top
    /// among them.
    pub levels: usize,
    /// How many distinct cell views it places, the top among them.
    pub cells: usize,
    /// How many devices of each kind it holds, flattened, in [`KINDS`]
    /// order; kinds it holds none of are left out.
    pub devices: Vec<(&'static str, usize)>,
}

impl DesignCensus {
    /// "2 levels · 4 cells".
    pub fn hierarchy_text(&self) -> String {
        format!(
            "{} \u{b7} {}",
            counted(self.levels, "level", "levels"),
            counted(self.cells, "cell", "cells")
        )
    }

    /// "12 MOSFETs · 8 resistors · 4 capacitors · 3 sources", or "None" for
    /// a design that places nothing that simulates.
    pub fn devices_text(&self) -> String {
        if self.devices.is_empty() {
            return "None".to_owned();
        }
        self.devices
            .iter()
            .map(|(kind, count)| {
                let (one, many) = spelled(kind);
                counted(*count, one, many)
            })
            .collect::<Vec<_>>()
            .join(" \u{b7} ")
    }
}

/// A kind of [`KINDS`] in words, one and many.
fn spelled(kind: &str) -> (&'static str, &'static str) {
    match kind {
        "MOSFET" => ("MOSFET", "MOSFETs"),
        "BJT" => ("BJT", "BJTs"),
        "JFET" => ("JFET", "JFETs"),
        "MESFET" => ("MESFET", "MESFETs"),
        "diode" => ("diode", "diodes"),
        "R" => ("resistor", "resistors"),
        "C" => ("capacitor", "capacitors"),
        "L" => ("inductor", "inductors"),
        "K" => ("coupling", "couplings"),
        "memristor" => ("memristor", "memristors"),
        "line" => ("transmission line", "transmission lines"),
        "switch" => ("switch", "switches"),
        "controlled" => ("controlled source", "controlled sources"),
        "XSPICE" => ("XSPICE model", "XSPICE models"),
        "source" => ("source", "sources"),
        _ => ("subcircuit", "subcircuits"),
    }
}

fn counted(count: usize, one: &str, many: &str) -> String {
    format!("{count} {}", if count == 1 { one } else { many })
}

/// The census of the design the next run elaborates, kept until the
/// projection it was counted from is replaced; `None` where the plan's
/// configuration cannot be projected, which the Configuration fact says.
pub fn design_census(ctx: &egui::Context, model: &Model) -> Option<DesignCensus> {
    let projection = model.plan_inspect_design_projection().ok()?;
    Some(derived(
        ctx,
        "overview.design census",
        Counted(Arc::downgrade(&projection)),
        || {
            count_scan();
            census(&projection)
        },
    ))
}

/// The projection a census was counted from, by identity: held weakly, so
/// the census keeps nothing alive, and its allocation is not reused while
/// it is held, so a new projection is never taken for the old one.
#[derive(Clone)]
struct Counted(Weak<DesignProjection>);

impl PartialEq for Counted {
    fn eq(&self, other: &Self) -> bool {
        Weak::ptr_eq(&self.0, &other.0)
    }
}

/// Every context the hierarchy resolved, each once: a cell placed many
/// times counts its devices once per placement through its multiplicity.
fn census(projection: &DesignProjection) -> DesignCensus {
    let resolution = projection.hierarchy_resolution();
    let occurrences = &resolution.occurrences;
    let Some(root) = occurrences.root() else {
        return DesignCensus::default();
    };
    let mut counts = [0_usize; KINDS.len()];
    let mut cells = HashSet::new();
    let mut seen = HashSet::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if !seen.insert(node) {
            continue;
        }
        let multiplicity = occurrences.occurrences_of(node);
        let reference = occurrences
            .own_row(node)
            .and_then(|row| resolution.bindings.get(row))
            .map(|binding| &binding.reference);
        if let Some(reference) = reference {
            cells.insert(reference.key().to_ascii_lowercase());
        }
        match reference.and_then(|reference| schematic(projection, reference)) {
            Some(schematic) => {
                for component in &schematic.document().components {
                    if let Some(kind) = kind_of(component.kind) {
                        counts[kind] = counts[kind].saturating_add(multiplicity);
                    }
                }
            }
            None if node != root => {
                counts[SUBCIRCUIT] = counts[SUBCIRCUIT].saturating_add(multiplicity);
            }
            None => {}
        }
        stack.extend(occurrences.placed(node).map(|(_, child)| child));
    }
    let mut depths = HashMap::new();
    DesignCensus {
        levels: depth(occurrences, root, &mut depths),
        cells: cells.len(),
        devices: KINDS
            .iter()
            .zip(counts)
            .filter(|(_, count)| *count > 0)
            .map(|(kind, count)| (*kind, count))
            .collect(),
    }
}

/// How many levels stand under `node`, itself among them. The hierarchy
/// refuses recursion, so the walk ends; each context is measured once.
fn depth(
    occurrences: &HierarchyOccurrences,
    node: usize,
    depths: &mut HashMap<usize, usize>,
) -> usize {
    if let Some(&known) = depths.get(&node) {
        return known;
    }
    // Marked first, so a cycle the resolver let through ends here.
    depths.insert(node, 1);
    let children = occurrences
        .placed(node)
        .map(|(_, child)| child)
        .collect::<Vec<_>>();
    let below = children
        .into_iter()
        .map(|child| depth(occurrences, child, depths))
        .max()
        .unwrap_or(0);
    depths.insert(node, below + 1);
    below + 1
}

/// The drawing `reference` is projected from, when it is a schematic.
fn schematic<'a>(
    projection: &'a DesignProjection,
    reference: &CellViewRef,
) -> Option<&'a ProjectedSchematic> {
    let key = reference.key();
    let buffers = projection.schematic_buffers();
    buffers.get(&key).or_else(|| {
        buffers
            .iter()
            .find(|(buffer, _)| buffer.eq_ignore_ascii_case(&key))
            .map(|(_, schematic)| schematic)
    })
}

/// The kind a placed component is counted under, by the element the deck
/// writes for it; `None` for what the deck writes no element for, and for
/// a cell instance, whose devices are counted in its own cell.
fn kind_of(kind: ComponentType) -> Option<usize> {
    let named = match kind {
        ComponentType::LoopProbe
        | ComponentType::RfPort
        | ComponentType::Ground
        | ComponentType::Port
        | ComponentType::CellInstance => return None,
        _ => match kind.spice_prefix() {
            "M" => "MOSFET",
            "Q" => "BJT",
            "J" => "JFET",
            "Z" => "MESFET",
            "D" => "diode",
            "R" => "R",
            "C" => "C",
            "L" => "L",
            "K" => "K",
            "MR" => "memristor",
            "T" | "O" | "P" => "line",
            "S" | "W" => "switch",
            "E" | "F" | "G" | "H" | "B" => "controlled",
            "A" => "XSPICE",
            "V" | "I" => "source",
            _ => return None,
        },
    };
    KINDS.iter().position(|listed| *listed == named)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_device_is_counted_under_the_element_the_deck_writes_for_it() {
        let named = |kind| kind_of(kind).map(|at| KINDS[at]);
        assert_eq!(named(ComponentType::Nmos), Some("MOSFET"));
        assert_eq!(named(ComponentType::PnpBjt4), Some("BJT"));
        assert_eq!(named(ComponentType::SaturableInductor), Some("L"));
        assert_eq!(named(ComponentType::VoltageSourcePulse), Some("source"));
        assert_eq!(named(ComponentType::OpAmp), Some("controlled"));
        assert_eq!(named(ComponentType::LossyTransmissionLine), Some("line"));
        // A probe and a port are the circuit's furniture, not its devices;
        // an instance is counted through its cell.
        assert_eq!(named(ComponentType::LoopProbe), None);
        assert_eq!(named(ComponentType::Ground), None);
        assert_eq!(named(ComponentType::CellInstance), None);
    }

    #[test]
    fn the_census_reads_its_counts_in_one_line() {
        let census = DesignCensus {
            levels: 2,
            cells: 1,
            devices: vec![("R", 4), ("source", 1)],
        };
        assert_eq!(census.hierarchy_text(), "2 levels \u{b7} 1 cell");
        assert_eq!(census.devices_text(), "4 resistors \u{b7} 1 source");
        assert_eq!(DesignCensus::default().devices_text(), "None");
    }
}
