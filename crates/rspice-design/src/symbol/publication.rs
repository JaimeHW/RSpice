//! Symbol pin checks, paired metadata encoding, and pin reports.

#[cfg(test)]
mod tests;

use super::{
    PinFindingKind, SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SymbolDocument,
    SymbolEditorMetadata, SymbolInterface,
};
use crate::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use crate::library::View;
use rspice_design_model::{cell_view::CellViewRef, port::PortSpec};
use rspice_model_library::symbol::ModelBoundSymbolDefinition;

/// The interface a symbol bound to `definition` answers to: the model's pins,
/// in its netlist order.
pub fn bound_interface(definition: &ModelBoundSymbolDefinition) -> SymbolInterface {
    let mut pins = definition.pins.iter().collect::<Vec<_>>();
    pins.sort_by_key(|pin| pin.order);
    SymbolInterface::Declared(
        pins.into_iter()
            .map(|pin| PortSpec {
                name: pin.name.clone(),
                direction: pin.direction,
            })
            .collect(),
    )
}

/// Validated encodings of a symbol document and its editor metadata.
///
/// Both encodings are prepared before either metadata entry is published.
pub struct EncodedSymbolEditorBundle {
    encoded_document: String,
    encoded_editor: String,
}

impl EncodedSymbolEditorBundle {
    pub fn encode(
        document: &SymbolDocument,
        metadata: &SymbolEditorMetadata,
    ) -> Result<Self, String> {
        let encoded_document = document.encode()?;
        let encoded_editor = metadata.encode()?;
        Ok(Self {
            encoded_document,
            encoded_editor,
        })
    }

    pub fn store_in_view(self, view: &mut View) {
        view.metadata.insert(
            SYMBOL_DOCUMENT_METADATA_KEY.to_owned(),
            self.encoded_document,
        );
        view.metadata
            .insert(SYMBOL_EDITOR_METADATA_KEY.to_owned(), self.encoded_editor);
        view.metadata.remove("generated");
        view.metadata.remove("ports");
        view.modified = true;
    }
}

/// Check a symbol against the interface it answers to, retaining every
/// finding with its cellview location.
///
/// `binding` is the model the symbol is bound to, if any; its pins are then
/// the interface, and their electrical types and the model's cell are checked
/// too. `source` names what the interface is read from (the schematic, the
/// compiled module, the model), in the words a finding about a pin it does
/// not declare uses. Every finding is a statement on the Problems page: none
/// of them stops the symbol being saved.
pub fn check_symbol_pins(
    document: &SymbolDocument,
    interface: &SymbolInterface,
    binding: Option<&ModelBoundSymbolDefinition>,
    reference: &CellViewRef,
    source: &str,
) -> DrcResult {
    let mut found: Vec<(DrcViolationType, String, DrcLocation)> = Vec::new();
    let pin_location = |pin_name: &str| DrcLocation::SymbolPin {
        reference: reference.clone(),
        pin_name: pin_name.to_owned(),
        point: document.pin(pin_name).and_then(|pin| pin.position),
    };
    let cell_location = || DrcLocation::CellView {
        reference: reference.clone(),
    };

    for finding in document.pin_findings(interface) {
        let violation_type = match finding.kind {
            PinFindingKind::UnplacedPin => DrcViolationType::SymbolUnplacedPin,
            PinFindingKind::OrphanedPin => DrcViolationType::SymbolOrphanedPin,
            PinFindingKind::PinOffGrid => DrcViolationType::SymbolPinOffGrid,
        };
        let message = match finding.kind {
            PinFindingKind::OrphanedPin => {
                format!("Symbol pin is not in the {source}: {}", finding.pin_name)
            }
            PinFindingKind::UnplacedPin | PinFindingKind::PinOffGrid => {
                format!("{}: {}", violation_type.description(), finding.pin_name)
            }
        };
        found.push((violation_type, message, pin_location(&finding.pin_name)));
    }

    let ports = interface.ports();
    if let Some((expected, observed)) = pin_order_difference(document, ports) {
        found.push((
            DrcViolationType::SymbolPinOrder,
            format!(
                "{}: expected {expected}, found {observed}",
                DrcViolationType::SymbolPinOrder.description()
            ),
            cell_location(),
        ));
    }

    for pin in &document.pins {
        let disagrees = match binding {
            Some(definition) => definition
                .pins
                .iter()
                .find(|declared| declared.name.eq_ignore_ascii_case(&pin.name))
                .is_some_and(|declared| {
                    pin.electrical_type() != declared.electrical_type
                        || pin.direction != declared.direction
                }),
            None => ports
                .iter()
                .find(|port| port.name.eq_ignore_ascii_case(&pin.name))
                .is_some_and(|port| !pin.states_direction(port.direction)),
        };
        if disagrees {
            let violation_type = DrcViolationType::SymbolPinDirection;
            let message = format!("{}: {}", violation_type.description(), pin.name);
            found.push((violation_type, message, pin_location(&pin.name)));
        }
    }

    if let Some(definition) = binding
        && definition.identity.cell != reference.cell
    {
        found.push((
            DrcViolationType::SymbolModelFamily,
            format!(
                "{}: {}",
                DrcViolationType::SymbolModelFamily.description(),
                definition.identity.cell
            ),
            cell_location(),
        ));
    }

    let mut result = DrcResult::new();
    result.completed = true;
    for (index, (violation_type, message, location)) in found.into_iter().enumerate() {
        result.add_violation(DrcViolation::new(
            index + 1,
            violation_type,
            message,
            location,
        ));
    }
    result
}

/// The interface's pin names and the symbol's, each in order, where the
/// symbol carries exactly the interface's pins in a different order.
fn pin_order_difference(document: &SymbolDocument, ports: &[PortSpec]) -> Option<(String, String)> {
    let expected = ports
        .iter()
        .map(|port| port.name.to_ascii_lowercase())
        .collect::<Vec<_>>();
    let observed = document
        .pins
        .iter()
        .map(|pin| pin.name.to_ascii_lowercase())
        .collect::<Vec<_>>();
    let mut expected_set = expected.clone();
    let mut observed_set = observed.clone();
    expected_set.sort();
    observed_set.sort();
    (expected != observed && expected_set == observed_set).then(|| {
        (
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>()
                .join(" "),
            document
                .pins
                .iter()
                .map(|pin| pin.name.as_str())
                .collect::<Vec<_>>()
                .join(" "),
        )
    })
}
