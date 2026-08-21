//! What a published part becomes when it is dropped on a sheet.
//!
//! A pack manifest says how each of its parts is drawn with one string —
//! `symbol.ref` — and a pin map from the part's own terminals to that symbol's
//! pins. This module is the only place that reads either. It turns them into a
//! placement the schematic already knows how to commit, so nothing downstream
//! learns that packs exist.
//!
//! # The reference vocabulary
//!
//! `symbol.ref` names an identity this client already owns: the stable ids in
//! [`crate::state::schematic::DeviceDescriptor`]. Two forms are meaningful to a
//! model pack, and they are meaningful for different reasons.
//!
//! | `symbol.ref`                    | Resolves to                                    |
//! |---------------------------------|------------------------------------------------|
//! | `rspice.native.<device>`        | the native device symbol for that stable id     |
//! | `rspice.native.<device>:<skin>` | the same device drawn with a named symbol skin  |
//! | `rspice.library.cell_instance`  | a generic cell instance over the part's ports   |
//!
//! The split is not cosmetic. A `.model` card *is* a native device — a zener
//! is a diode with a diode's two terminals and a diode's `D` card — so it is
//! placed as one, and the suffix picks the skin the family already ships
//! (`zener`, `schottky`, `led`, …) rather than inventing artwork. A subcircuit
//! macromodel is not any native device: a five-terminal operational amplifier
//! has supplies that RSpice's three-pin ideal op-amp symbol has nowhere to put,
//! so it is drawn as a cell instance whose pins *are* the subcircuit's ports,
//! in the subcircuit's own order.
//!
//! # The pin map is a check, not a rename
//!
//! Terminal order is netlist order. Reordering or renaming terminals here
//! would silently change which node a port connects to, so the map is never
//! applied as a transformation: it is required to name exactly the part's
//! terminals, exactly once each, and anything else refuses the part rather
//! than guessing. What the map buys is that a manifest which disagrees with
//! its own source is caught before a symbol is ever drawn.

use std::path::Path;

use rspice_pack::{Part, PartKind, Symbol};

use crate::state::model_library::{ModelLibrary, placement_component_for_model};
use crate::state::schematic::{ComponentType, LibraryCellInstance, PortDirection, PortSpec};

/// The `symbol.ref` value that asks for a generic cell instance.
const CELL_INSTANCE_REFERENCE: &str = "rspice.library.cell_instance";

/// The prefix every native device stable id carries.
const NATIVE_PREFIX: &str = "rspice.native.";

/// How one published part is placed on a sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartPlacement {
    /// A native device carrying a model card. The card's name is the placed
    /// instance's value, which is where every native emitter reads it from.
    NativeDevice {
        component_type: ComponentType,
        /// A named symbol skin the device family ships, when the reference
        /// asked for one. `None` is the family's default artwork.
        variant: Option<String>,
        /// The model card name, as the signed manifest publishes it.
        model: String,
    },
    /// A generic instance bound to the part's own ports.
    CellInstance(Box<LibraryCellInstance>),
}

impl PartPlacement {
    /// The symbol identity a receipt or a test can name.
    #[must_use]
    pub fn symbol_reference(&self) -> String {
        match self {
            Self::NativeDevice {
                component_type,
                variant,
                ..
            } => match variant {
                Some(variant) => format!("{}:{variant}", component_type.descriptor().stable_id),
                None => component_type.descriptor().stable_id.to_owned(),
            },
            Self::CellInstance(_) => CELL_INSTANCE_REFERENCE.to_owned(),
        }
    }
}

/// Resolves a manifest part into the placement the schematic commits.
///
/// `library` is the project model library the part's bytes were retained into,
/// and `source` is the retained file that defines it. Both are recorded on a
/// cell instance so a placed macromodel still names where its definition came
/// from after the pack is uninstalled.
pub fn plan_part_placement(
    part: &Part,
    library: &str,
    source: Option<&Path>,
) -> Result<PartPlacement, String> {
    let reference = part
        .symbol
        .as_ref()
        .map(|symbol| symbol.reference.as_str())
        .unwrap_or(match part.kind {
            // A subcircuit's ports are a complete placement contract on their
            // own, so an older manifest that predates `symbol` still places.
            // A model card's is not: nothing in a `.model` line says which
            // device symbol to draw, and guessing one would draw a resistor
            // for a transistor.
            PartKind::Subckt => CELL_INSTANCE_REFERENCE,
            PartKind::Model => "",
        });
    if reference.is_empty() {
        return Err(format!(
            "part '{}' publishes a model card without a symbol reference, so RSpice cannot know \
             which device to draw",
            part.id
        ));
    }
    check_pin_map(part)?;

    if reference == CELL_INSTANCE_REFERENCE {
        return Ok(PartPlacement::CellInstance(Box::new(cell_instance(
            part, library, source,
        )?)));
    }
    let Some(device) = reference.strip_prefix(NATIVE_PREFIX) else {
        return Err(format!(
            "part '{}' names the symbol '{reference}', which is not a symbol identity this build \
             publishes",
            part.id
        ));
    };
    let (device, variant) = match device.split_once(':') {
        Some((device, variant)) => (device, Some(variant)),
        None => (device, None),
    };
    let stable_id = format!("{NATIVE_PREFIX}{device}");
    let component_type = ComponentType::ALL
        .into_iter()
        .find(|candidate| candidate.descriptor().stable_id == stable_id)
        .ok_or_else(|| {
            format!(
                "part '{}' names the device '{stable_id}', which this build does not offer",
                part.id
            )
        })?;
    if part.terminals.len() != component_type.terminal_count() {
        return Err(format!(
            "part '{}' declares {} terminal(s) but {} has {}",
            part.id,
            part.terminals.len(),
            component_type.display_name(),
            component_type.terminal_count()
        ));
    }
    let variant = match variant {
        Some(variant) => Some(checked_variant(component_type, variant, &part.id)?),
        None => None,
    };
    Ok(PartPlacement::NativeDevice {
        component_type,
        variant,
        model: part.id.clone(),
    })
}

/// Resolves a definition the project already holds into the same placement.
///
/// [`plan_part_placement`] reads a signed manifest's `symbol.ref`. A definition
/// the project retained — or one compiled into this build — has no manifest to
/// read: it is parsed source. So its placement is read from what parsing
/// produced, and lands on the same two shapes through the same builders, which
/// is what makes a retained part and a pack part the same object to the
/// schematic and to every emitter below it.
///
/// A subcircuit's ordered ports are a complete placement contract, exactly as
/// they are for a pack part that publishes no `symbol` block. A model card's
/// are not, so the card's declared device family decides which native device
/// wears it — the same family contract that decides whether a card may be bound
/// to an instance already on the sheet. A card whose family has no schematic
/// device is refused rather than drawn as something else.
pub fn plan_library_placement(library: &ModelLibrary, part: &str) -> Result<PartPlacement, String> {
    if let Some(interface) = library.subcircuits.get(part) {
        return Ok(PartPlacement::CellInstance(Box::new(cell_instance_over(
            part,
            &interface.ports,
            &library.name,
            library.root_path.as_deref(),
        )?)));
    }
    let card = library
        .models
        .get(part)
        .or_else(|| library.top_level_models.get(part))
        .ok_or_else(|| {
            format!(
                "library '{}' holds no definition named '{part}'",
                library.name
            )
        })?;
    let component_type = placement_component_for_model(card).ok_or_else(|| {
        format!(
            "'{part}' is a {} card, which no device in this build is drawn for, so RSpice cannot \
             know which symbol to place",
            card.spice_type
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| card.model_type.display_name())
        )
    })?;
    Ok(PartPlacement::NativeDevice {
        component_type,
        // Nothing in a parsed `.model` line names a symbol skin. A pack part
        // says so in its manifest; a retained card that never claimed one wears
        // the family's default artwork rather than a guess.
        variant: None,
        model: part.to_owned(),
    })
}

/// Requires the manifest's pin map to describe the part's own terminals.
///
/// An absent map is accepted: the terminals then stand for themselves, which
/// is the same contract the map would have spelled out. A *present* map that
/// disagrees is refused, because the disagreement is the interesting case.
fn check_pin_map(part: &Part) -> Result<(), String> {
    let Some(Symbol { pins, .. }) = part.symbol.as_ref() else {
        return Ok(());
    };
    if pins.is_empty() {
        return Ok(());
    }
    for terminal in &part.terminals {
        if !pins.contains_key(terminal) {
            return Err(format!(
                "part '{}' maps no symbol pin for its terminal '{terminal}'",
                part.id
            ));
        }
    }
    for terminal in pins.keys() {
        if !part.terminals.contains(terminal) {
            return Err(format!(
                "part '{}' maps a symbol pin for '{terminal}', which is not one of its terminals",
                part.id
            ));
        }
    }
    let mut targets = pins.values().collect::<Vec<_>>();
    targets.sort();
    let assigned = targets.len();
    targets.dedup();
    if targets.len() != assigned {
        return Err(format!(
            "part '{}' maps two terminals onto one symbol pin",
            part.id
        ));
    }
    Ok(())
}

/// Whether a device family ships the named symbol skin.
///
/// The allowed set is the one the property registry already publishes for the
/// `symbol` appearance property, so a pack can never select a skin the
/// properties dialog would refuse — and adding a skin in one place adds it
/// here too.
fn checked_variant(
    component_type: ComponentType,
    variant: &str,
    part_id: &str,
) -> Result<String, String> {
    let registry = crate::state::PropertyRegistry::new();
    let options = registry
        .get(component_type)
        .and_then(|sheet| sheet.iter().find(|property| property.name == "symbol"))
        .and_then(|property| match &property.default_value {
            crate::state::PropertyValue::Enum { options, .. } => Some(options.clone()),
            _ => None,
        })
        .unwrap_or_default();
    if !options.iter().any(|option| option == variant) {
        return Err(format!(
            "part '{part_id}' asks for the '{variant}' symbol skin, which {} does not ship",
            component_type.display_name()
        ));
    }
    Ok(variant.to_owned())
}

/// Builds the generic instance binding for a subcircuit part.
fn cell_instance(
    part: &Part,
    library: &str,
    source: Option<&Path>,
) -> Result<LibraryCellInstance, String> {
    cell_instance_over(&part.id, &part.terminals, library, source)
}

/// The generic instance binding over one ordered terminal contract.
///
/// Taken as an id and a terminal list rather than as a manifest part so the
/// retained route builds exactly the same object from a parsed subcircuit's
/// ports. Terminal order is netlist order on both paths.
fn cell_instance_over(
    id: &str,
    terminals: &[String],
    library: &str,
    source: Option<&Path>,
) -> Result<LibraryCellInstance, String> {
    if terminals.is_empty() {
        return Err(format!(
            "part '{id}' publishes no ordered terminal contract, so it has no placeable interface"
        ));
    }
    // `spice` is the implementation view a retained source-backed cell already
    // uses, so a placed pack part and a placed imported subcircuit are the
    // same kind of object to everything downstream.
    let mut binding = LibraryCellInstance::new(library, id, "spice");
    binding.bind_interface(
        &terminals
            .iter()
            .map(|terminal| PortSpec {
                name: terminal.clone(),
                // A subcircuit port carries no declared direction, and
                // inventing one would draw an arrow the source never claimed.
                direction: PortDirection::InOut,
            })
            .collect::<Vec<_>>(),
    );
    binding.source_path = source.map(Path::to_path_buf);
    binding.module_name = Some(id.to_owned());
    binding.reference_prefix = Some("X".to_owned());
    Ok(binding)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn part(id: &str, kind: PartKind, terminals: &[&str], reference: Option<&str>) -> Part {
        Part {
            id: id.to_owned(),
            kind,
            device: "proving".to_owned(),
            aliases: Vec::new(),
            source: rspice_pack::SourceRef {
                path: "models/proving.lib".to_owned(),
                line: 1,
            },
            terminals: terminals.iter().map(|name| (*name).to_owned()).collect(),
            symbol: reference.map(|reference| Symbol {
                reference: reference.to_owned(),
                pins: terminals
                    .iter()
                    .map(|name| ((*name).to_owned(), (*name).to_owned()))
                    .collect(),
            }),
        }
    }

    #[test]
    fn a_model_card_resolves_to_its_native_device_and_named_skin() {
        let zener = part(
            "1N4728A",
            PartKind::Model,
            &["A", "K"],
            Some("rspice.native.diode:zener"),
        );
        assert_eq!(
            plan_part_placement(&zener, "diodes", None).expect("a zener places"),
            PartPlacement::NativeDevice {
                component_type: ComponentType::Diode,
                variant: Some("zener".to_owned()),
                model: "1N4728A".to_owned(),
            }
        );

        let plain = part(
            "1N4001",
            PartKind::Model,
            &["A", "K"],
            Some("rspice.native.diode"),
        );
        assert_eq!(
            plan_part_placement(&plain, "diodes", None).expect("a diode places"),
            PartPlacement::NativeDevice {
                component_type: ComponentType::Diode,
                variant: None,
                model: "1N4001".to_owned(),
            }
        );
    }

    #[test]
    fn a_subcircuit_places_as_a_cell_instance_in_its_own_port_order() {
        let opamp = part(
            "LM324",
            PartKind::Subckt,
            &["INP", "INN", "VCC", "VEE", "OUT"],
            Some(CELL_INSTANCE_REFERENCE),
        );
        let placement = plan_part_placement(
            &opamp,
            "rspice_opamps",
            Some(Path::new(
                "/packs/rspice-opamps/1.0.0/files/models/lm324.lib",
            )),
        )
        .expect("an opamp places");
        let PartPlacement::CellInstance(binding) = placement else {
            panic!("a five-terminal macromodel is not a native device");
        };
        assert_eq!(binding.cell, "LM324");
        assert_eq!(binding.module_name.as_deref(), Some("LM324"));
        assert_eq!(binding.reference_prefix.as_deref(), Some("X"));
        assert_eq!(
            binding.terminal_order,
            ["INP", "INN", "VCC", "VEE", "OUT"].map(str::to_owned)
        );
        assert!(binding.interface_bound);
        assert!(binding.source_path.is_some());
    }

    #[test]
    fn a_manifest_that_disagrees_with_itself_is_refused_rather_than_repaired() {
        // A skin the diode family does not ship.
        let bogus_skin = part(
            "X1",
            PartKind::Model,
            &["A", "K"],
            Some("rspice.native.diode:holographic"),
        );
        assert!(
            plan_part_placement(&bogus_skin, "lib", None)
                .expect_err("an unknown skin is refused")
                .contains("holographic")
        );

        // A device this build does not offer.
        let bogus_device = part("X2", PartKind::Model, &["A"], Some("rspice.native.tachyon"));
        assert!(
            plan_part_placement(&bogus_device, "lib", None)
                .expect_err("an unknown device is refused")
                .contains("rspice.native.tachyon")
        );

        // A terminal count the device cannot have.
        let wrong_arity = part(
            "X3",
            PartKind::Model,
            &["A", "K", "G"],
            Some("rspice.native.diode"),
        );
        assert!(
            plan_part_placement(&wrong_arity, "lib", None)
                .expect_err("a three-terminal diode is refused")
                .contains("terminal")
        );

        // A model card with no symbol reference at all.
        let unadorned = part("X4", PartKind::Model, &["A", "K"], None);
        assert!(
            plan_part_placement(&unadorned, "lib", None)
                .expect_err("an unadorned model card is refused")
                .contains("symbol reference")
        );

        // A pin map naming a terminal the part does not have.
        let mut stray = part(
            "X5",
            PartKind::Subckt,
            &["IN", "OUT"],
            Some(CELL_INSTANCE_REFERENCE),
        );
        if let Some(symbol) = stray.symbol.as_mut() {
            symbol.pins.insert("VDD".to_owned(), "VDD".to_owned());
        }
        assert!(
            plan_part_placement(&stray, "lib", None)
                .expect_err("a stray pin mapping is refused")
                .contains("VDD")
        );

        // A pin map that collapses two terminals onto one pin.
        let mut collapsed = part(
            "X6",
            PartKind::Subckt,
            &["IN", "OUT"],
            Some(CELL_INSTANCE_REFERENCE),
        );
        if let Some(symbol) = collapsed.symbol.as_mut() {
            symbol.pins.insert("OUT".to_owned(), "IN".to_owned());
        }
        assert!(
            plan_part_placement(&collapsed, "lib", None)
                .expect_err("a collapsed pin map is refused")
                .contains("one symbol pin")
        );
    }

    #[test]
    fn a_subcircuit_without_a_symbol_block_still_places_from_its_ports() {
        let legacy = part("DIV", PartKind::Subckt, &["IN", "OUT"], None);
        let placement = plan_part_placement(&legacy, "lib", None).expect("a legacy subckt places");
        assert_eq!(placement.symbol_reference(), CELL_INSTANCE_REFERENCE);
    }
}
