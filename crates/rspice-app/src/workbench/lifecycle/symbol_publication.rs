//! Symbol publication rules, paired metadata encoding, and pin reports.

#[cfg(test)]
mod tests;

use crate::services::drc::{DrcLocation, DrcResult, DrcViolation, DrcViolationType};
use crate::state::{
    CellViewRef, MAX_SYMBOL_DOCUMENT_BYTES, PinFindingKind, PortDirection, PortSpec,
    SYMBOL_DOCUMENT_METADATA_KEY, SYMBOL_EDITOR_METADATA_KEY, SymbolDocument, SymbolEditorMetadata,
    View,
};

/// One row of the symbol's publication contract.
///
/// Every row is computed from the document and its interface alone, so the
/// same rows decide a headless `Command::SymbolSave` and fill the dialog's
/// table. A row that renders one verdict and enforces another is the defect
/// this split exists to make impossible.
#[derive(Debug, Clone)]
pub struct SymbolSaveCheck {
    pub label: &'static str,
    pub expected: String,
    pub observed: String,
    pub passed: bool,
}

impl SymbolSaveCheck {
    /// The console line a refusal reports, naming the row that blocked it.
    pub fn refusal(&self) -> String {
        format!(
            "Symbol not saved - {} check expected {}, found {}",
            self.label.to_ascii_lowercase(),
            self.expected,
            self.observed
        )
    }
}

/// The publication contract of `document`, judged against the interface it
/// answers to and the model definition it is bound to, if any.
pub fn symbol_save_checks(
    definition: Option<&crate::state::ModelBoundSymbolDefinition>,
    cell: &str,
    document: &SymbolDocument,
    ports: &[PortSpec],
) -> Vec<SymbolSaveCheck> {
    let mut expected_names = ports
        .iter()
        .map(|port| port.name.clone())
        .collect::<Vec<_>>();
    let expected_directions = if let Some(definition) = definition {
        let mut pins = definition.pins.iter().collect::<Vec<_>>();
        pins.sort_by_key(|pin| pin.order);
        expected_names = pins.iter().map(|pin| pin.name.clone()).collect();
        pins.into_iter()
            .map(|pin| {
                (
                    pin.electrical_type,
                    pin.direction,
                    pin.side,
                    pin.name.clone(),
                )
            })
            .collect::<Vec<_>>()
    } else {
        ports
            .iter()
            .map(|port| {
                (
                    match port.direction {
                        PortDirection::Supply => crate::state::SymbolElectricalType::Power,
                        _ => crate::state::SymbolElectricalType::Analog,
                    },
                    port.direction,
                    match port.direction {
                        PortDirection::In => crate::state::SymbolPinSide::Left,
                        PortDirection::Out | PortDirection::InOut => {
                            crate::state::SymbolPinSide::Right
                        }
                        PortDirection::Supply => crate::state::SymbolPinSide::Top,
                    },
                    port.name.clone(),
                )
            })
            .collect()
    };
    let observed_names = document
        .pins
        .iter()
        .map(|pin| pin.name.clone())
        .collect::<Vec<_>>();
    let electrical_match = expected_directions.len() == document.pins.len()
        && expected_directions.iter().zip(&document.pins).all(
            |((electrical, direction, _, name), pin)| {
                pin.name.eq_ignore_ascii_case(name)
                    && pin.electrical_type() == *electrical
                    && pin.direction == *direction
            },
        );
    let placement_valid = document
        .pins
        .iter()
        .all(|pin| pin.position.is_some() && pin.terminal_on_grid());
    // The same finding the symbol check publishes as `SymbolPinOffGrid`, so
    // the dialog and the check console can never disagree about the count.
    let off_grid = document
        .pin_findings(ports)
        .into_iter()
        .filter(|finding| finding.kind == PinFindingKind::PinOffGrid)
        .count();
    let family = definition
        .map(|definition| definition.identity.cell.as_str())
        .unwrap_or("standalone symbol");
    vec![
        SymbolSaveCheck {
            label: "Pin count",
            expected: expected_names.len().to_string(),
            observed: document.pins.len().to_string(),
            passed: expected_names.len() == document.pins.len(),
        },
        SymbolSaveCheck {
            label: "Netlist order",
            expected: expected_names.join(" "),
            observed: observed_names.join(" "),
            passed: expected_names.len() == observed_names.len()
                && expected_names
                    .iter()
                    .zip(&observed_names)
                    .all(|(expected, observed)| expected.eq_ignore_ascii_case(observed)),
        },
        SymbolSaveCheck {
            label: "Electrical types",
            expected: "bound contract".to_owned(),
            observed: if electrical_match {
                "matched".to_owned()
            } else {
                "mismatch".to_owned()
            },
            passed: electrical_match,
        },
        SymbolSaveCheck {
            label: "Pin placement",
            expected: "placed on terminal grid".to_owned(),
            observed: if placement_valid {
                "all placed".to_owned()
            } else {
                "unplaced or off grid".to_owned()
            },
            passed: placement_valid,
        },
        SymbolSaveCheck {
            label: "Off-grid terminals",
            expected: "0".to_owned(),
            observed: off_grid.to_string(),
            passed: off_grid == 0,
        },
        SymbolSaveCheck {
            label: "Model family",
            expected: family.to_owned(),
            observed: cell.to_owned(),
            passed: definition.is_none_or(|definition| definition.identity.cell == cell),
        },
    ]
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
        document.validate()?;
        let encoded_document = serde_json::to_string(document)
            .map_err(|error| format!("Could not serialize symbol metadata: {error}"))?;
        if encoded_document.len() > MAX_SYMBOL_DOCUMENT_BYTES {
            return Err(format!(
                "Could not serialize symbol metadata: document is {} bytes; the limit is {MAX_SYMBOL_DOCUMENT_BYTES}",
                encoded_document.len()
            ));
        }
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

/// Check symbol pins and retain every finding with its cellview location.
pub fn check_symbol_pins(
    document: &SymbolDocument,
    ports: &[PortSpec],
    reference: &CellViewRef,
) -> DrcResult {
    let findings = document.pin_findings(ports);
    let mut result = DrcResult::new();
    result.completed = true;

    for (index, finding) in findings.iter().enumerate() {
        let violation_type = match finding.kind {
            PinFindingKind::UnplacedPin => DrcViolationType::SymbolUnplacedPin,
            PinFindingKind::OrphanedPin => DrcViolationType::SymbolOrphanedPin,
            PinFindingKind::PinOffGrid => DrcViolationType::SymbolPinOffGrid,
        };
        let point = document.pin(&finding.pin_name).and_then(|pin| pin.position);
        let message = format!("{}: {}", violation_type.description(), finding.pin_name);
        result.add_violation(DrcViolation::new(
            index + 1,
            violation_type,
            message,
            DrcLocation::SymbolPin {
                reference: reference.clone(),
                pin_name: finding.pin_name.clone(),
                point,
            },
        ));
    }

    result
}
