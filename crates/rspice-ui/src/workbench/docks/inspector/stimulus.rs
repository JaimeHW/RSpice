//! What the inspector says about the stimulus library and the definition the
//! browser is on.
//!
//! The stage states what a definition *is* — its shape, its quantity, the
//! fields a card carries. This dock states the library it belongs to and the
//! author's own note about it, which is the split every other workspace's
//! inspector already keeps between the document and the object inside it.

use egui::Ui;

use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::property_row;
use super::section_header;

pub(super) fn show(ui: &mut Ui, state: &AppState) {
    let messages = state.ui.messages();
    let library = &state.workspace.stimulus_library;
    section_header(ui, &messages.text(MessageId::StimulusLibrarySection), None);
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldDefinitions),
        &library.len().to_string(),
    );

    let Some(definition) = state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .and_then(|name| library.get(name))
    else {
        property_row(
            ui,
            &messages.text(MessageId::StimulusFieldSelection),
            &messages.text(MessageId::StimulusNone),
        );
        return;
    };

    section_header(
        ui,
        &messages.text(MessageId::StimulusDefinitionSection),
        Some(&messages.format(
            MessageId::StimulusSavedRevision,
            &[("revision", &definition.revision().to_string())],
        )),
    );
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldName),
        definition.name(),
    );
    let unstated = messages.text(MessageId::StimulusPurposeUnstated);
    let purpose = definition.purpose.trim();
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldPurpose),
        if purpose.is_empty() {
            unstated.as_str()
        } else {
            purpose
        },
    );
}
