//! What a part a loaded model library holds becomes when it is dropped on a
//! sheet.
//!
//! A `.model` card *is* a native device — a zener is a diode with a diode's two
//! terminals and a diode's `D` card — so it is placed as one. A subcircuit
//! macromodel is not any native device: a five-terminal operational amplifier
//! has supplies that RSpice's three-pin ideal op-amp symbol has nowhere to put,
//! so it is drawn as a cell instance whose pins *are* the subcircuit's ports,
//! in the subcircuit's own order.

use std::path::Path;

use crate::state::model_library::{ModelLibrary, placement_component_for_model};
use crate::state::schematic::{ComponentType, LibraryCellInstance, PortSpec};

/// How one part is placed on a sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartPlacement {
    /// A native device carrying a model card. The card's name is the placed
    /// instance's value, which is where every native emitter reads it from.
    NativeDevice {
        component_type: ComponentType,
        /// A named symbol skin the device family ships, when the reference
        /// asked for one. `None` is the family's default artwork.
        variant: Option<String>,
        /// The model card's name.
        model: String,
    },
    /// A generic instance bound to the part's own ports.
    CellInstance(Box<LibraryCellInstance>),
}

impl PartPlacement {
    /// What a placement receipt calls the part once it is armed: the cell
    /// instance's library and cell, or the native device's model card.
    pub fn armed_label(&self) -> String {
        match self {
            Self::NativeDevice { model, .. } => model.clone(),
            Self::CellInstance(binding) => format!("{}/{}", binding.library, binding.cell),
        }
    }
}

/// Resolves a definition a loaded library holds into the placement the
/// schematic commits.
///
/// A subcircuit's ordered ports are a complete placement contract. A model
/// card's are not, so the card's declared device family decides which native
/// device wears it — the same family contract that decides whether a card may
/// be bound to an instance already on the sheet. A card whose family has no
/// schematic device is refused rather than drawn as something else.
pub fn plan_library_placement(library: &ModelLibrary, part: &str) -> Result<PartPlacement, String> {
    if let Some(interface) = library.subcircuits.get(part) {
        return Ok(PartPlacement::CellInstance(Box::new(cell_instance_over(
            part,
            &interface.ports,
            interface.description.as_deref(),
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
        // Nothing in a parsed `.model` line names a symbol skin, so the card
        // wears the family's default artwork rather than a guess.
        variant: None,
        model: part.to_owned(),
    })
}

/// One refusal, ended as a sentence.
///
/// The placement planner writes clause-shaped diagnostics — "part 'X' declares
/// 3 terminals but Diode has 2" — because its other readers are receipts. On a
/// disabled control it is read as prose, so it is capitalized and stopped.
/// Every shelf that renders a refused placement disabled goes through this,
/// so the same refusal reads the same way on each of them.
#[must_use]
pub fn refusal_sentence(reason: String) -> String {
    let mut characters = reason.chars();
    let mut sentence = match characters.next() {
        Some(first) => first.to_uppercase().chain(characters).collect::<String>(),
        None => return "This part cannot be placed.".to_owned(),
    };
    if !sentence.ends_with('.') {
        sentence.push('.');
    }
    sentence
}

/// The generic instance binding over one ordered terminal contract.
///
/// Terminal order is netlist order. Pin roles come from the terminal names and
/// the vendor's pin-order comment; a pin neither states stays bidirectional
/// rather than drawn with a role the source never claimed.
pub(crate) fn cell_instance_over(
    id: &str,
    terminals: &[String],
    description: Option<&str>,
    library: &str,
    source: Option<&Path>,
) -> Result<LibraryCellInstance, String> {
    if terminals.is_empty() {
        return Err(format!(
            "part '{id}' publishes no ordered terminal contract, so it has no placeable interface"
        ));
    }
    // `spice` is the implementation view a retained source-backed cell already
    // uses, so a placed part and a placed imported subcircuit are the same
    // kind of object to everything downstream.
    let mut binding = LibraryCellInstance::new(library, id, "spice");
    let roles = rspice_design::pin_roles::infer_pin_roles(terminals, description);
    binding.bind_interface_with_roles(
        &terminals
            .iter()
            .zip(&roles)
            .map(|(terminal, role)| PortSpec {
                name: terminal.clone(),
                direction: role.direction(),
            })
            .collect::<Vec<_>>(),
        &roles,
    );
    binding.source_path = source.map(Path::to_path_buf);
    binding.module_name = Some(id.to_owned());
    binding.reference_prefix = Some("X".to_owned());
    Ok(binding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design_model::port::PinRole;

    const OPAMP_ROLES: [PinRole; 5] = [
        PinRole::NonInvertingInput,
        PinRole::InvertingInput,
        PinRole::PositiveSupply,
        PinRole::NegativeSupply,
        PinRole::Output,
    ];

    #[test]
    fn a_numbered_vendor_subcircuit_takes_its_roles_from_the_pin_order_comment() {
        let mut library = ModelLibrary::new("vendor");
        library.subcircuits.insert(
            "AMP".to_owned(),
            rspice_model_library::ModelSubcircuitInterface {
                name: "AMP".to_owned(),
                ports: ["1", "2", "99", "50", "45"].map(str::to_owned).to_vec(),
                parameter_defaults: Default::default(),
                description: Some(
                    "Node Assignments noninverting input | inverting input | | positive                      supply | | | negative supply | | | | output | | | | |"
                        .to_owned(),
                ),
                file_path: None,
                source_line: Some(12),
                section: None,
            },
        );
        let PartPlacement::CellInstance(binding) =
            plan_library_placement(&library, "AMP").expect("the subcircuit places")
        else {
            panic!("a subcircuit places as a cell instance");
        };
        assert_eq!(binding.terminal_order, ["1", "2", "99", "50", "45"]);
        assert_eq!(binding.interface_roles(), OPAMP_ROLES);
        let component = crate::state::schematic::Component::new(
            1,
            ComponentType::CellInstance,
            rspice_design_model::Point::origin(),
        )
        .with_library_cell(*binding);
        let block = component.instance_block();
        let side = |name: &str| {
            block
                .pins
                .iter()
                .find(|pin| pin.name == name)
                .map(|pin| pin.side)
        };
        use rspice_design_model::symbol_pin::SymbolPinSide;
        assert_eq!(side("1"), Some(SymbolPinSide::Left));
        assert_eq!(side("2"), Some(SymbolPinSide::Left));
        assert_eq!(side("99"), Some(SymbolPinSide::Top));
        assert_eq!(side("50"), Some(SymbolPinSide::Bottom));
        assert_eq!(side("45"), Some(SymbolPinSide::Right));
        // The triangle: three corners, apex on the output.
        assert_eq!(block.outline[0].0.len(), 3);
    }
}
