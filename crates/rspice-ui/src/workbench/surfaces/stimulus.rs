//! The Stimulus Library stage.
//!
//! An empty library is a real state, not a missing feature: a project that has
//! never authored a definition should say so and say where definitions come
//! from, rather than offer controls that refuse. What the stage shows when the
//! library is not empty is the definition the browser is on, read exactly as
//! the project stores it — name, shape, quantity, and the two fields adoption
//! copies onto an instance.
//!
//! Editing those fields is the instrument's job and the instrument is not
//! built, so nothing here is a control. That is deliberate: a disabled field
//! would claim an editor exists.

use egui::{ScrollArea, Ui};

use crate::state::stimulus_library::definition::StimulusDefinition;
use crate::workbench::{AppState, MessageCatalog, MessageId};

use super::super::design_system::{WorkbenchIcon, empty_state, property_row, section_header};

pub(super) fn show(ui: &mut Ui, state: &AppState) {
    let messages = state.ui.messages();
    let library = &state.workspace.stimulus_library;
    if library.is_empty() {
        empty_state(
            ui,
            WorkbenchIcon::Source,
            &messages.text(MessageId::StimulusLibraryEmpty),
            &messages.text(MessageId::StimulusLibraryEmptyDetail),
        );
        return;
    }
    let Some(definition) = state
        .workbench
        .selected_stimulus_definition
        .as_deref()
        .and_then(|name| library.get(name))
    else {
        empty_state(
            ui,
            WorkbenchIcon::Source,
            &messages.text(MessageId::StimulusNoSelection),
            &messages.text(MessageId::StimulusNoSelectionDetail),
        );
        return;
    };
    ScrollArea::vertical()
        .id_salt("workbench.stimulus.stage")
        .auto_shrink([false, false])
        .show(ui, |ui| definition_report(ui, messages, definition));
}

fn definition_report(ui: &mut Ui, messages: MessageCatalog, definition: &StimulusDefinition) {
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
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldFamily),
        definition.family().label(),
    );
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldQuantity),
        definition.kind().word(),
    );
    // `value` and `params` are the two fields adoption copies onto an
    // instance, in the instance's own spelling. Stating them is stating what
    // this definition would place — nothing here derives a card, because the
    // nets a card names belong to a sheet and a library definition has none.
    let unset = messages.text(MessageId::StimulusValueUnset);
    let value = definition.value.trim();
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldValue),
        if value.is_empty() {
            unset.as_str()
        } else {
            value
        },
    );
    let none = messages.text(MessageId::StimulusNone);
    let params = definition.params.trim();
    property_row(
        ui,
        &messages.text(MessageId::StimulusFieldParameters),
        if params.is_empty() {
            none.as_str()
        } else {
            params
        },
    );
    if let Some(file) = definition.pwl_file.as_ref() {
        property_row(
            ui,
            &messages.text(MessageId::StimulusFieldRetainedFile),
            &messages.format(
                MessageId::StimulusRetainedFile,
                &[
                    ("name", file.file_name.as_str()),
                    ("bytes", &file.contents.len().to_string()),
                ],
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;
    use crate::workbench::RSpiceApp;

    /// Every string the stage publishes, as a reader's tree carries it: a
    /// property row's label and value, and the plain copy of an empty state.
    fn published(state: &AppState) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    // The fit the workspace is held to.
                    egui::vec2(1024.0, 640.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, state));
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit stage tree")
            .nodes
            .into_iter()
            .flat_map(|(_, node)| {
                [
                    node.label().map(str::to_owned),
                    node.value().map(str::to_owned),
                ]
            })
            .flatten()
            .collect()
    }

    fn sine(name: &str) -> StimulusDefinition {
        let mut definition = StimulusDefinition::new(name, ComponentType::VoltageSourceSin)
            .expect("a sine source is one of the twenty-two");
        definition.value = "SIN(0 1 1k)".to_owned();
        definition
    }

    /// An empty library is a state, not a missing feature, so the stage has to
    /// say what the library holds and where a definition comes from. It must
    /// not offer a control: the authoring instrument is not built, and a
    /// disabled button would claim it is.
    #[test]
    fn an_empty_library_says_where_definitions_come_from_and_offers_no_control() {
        let app = RSpiceApp::test_instance();
        assert!(app.state.workspace.stimulus_library.is_empty());
        let published = published(&app.state);
        assert!(published.contains(&"No stimulus definitions".to_owned()));
        assert!(published.iter().any(|text| text
            == "This project has no stimulus definitions yet. Definitions are authored here or \
                saved from a placed source's properties."));
    }

    /// A library with definitions in it and nothing chosen is a different
    /// state from an empty one, and saying "no stimulus definitions" over five
    /// of them would be a lie the browser beside it immediately contradicts.
    #[test]
    fn a_library_with_nothing_chosen_does_not_report_itself_empty() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .stimulus_library
            .insert(sine("bridge_drive"))
            .expect("a fresh name");
        let published = published(&app.state);
        assert!(published.contains(&"No definition selected".to_owned()));
        assert!(!published.contains(&"No stimulus definitions".to_owned()));
    }

    /// What the stage reports is what the project stores — the shape, the
    /// quantity, and the two fields adoption copies — rather than anything
    /// derived. A card is not among them: a card names nets, and a library
    /// definition has none until it is placed.
    #[test]
    fn a_chosen_definition_is_reported_exactly_as_the_project_stores_it() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .stimulus_library
            .insert(sine("bridge_drive"))
            .expect("a fresh name");
        app.state.workbench.selected_stimulus_definition = Some("bridge_drive".to_owned());
        let published = published(&app.state);
        for expected in [
            // The section head announces its title and its metadata as one
            // string, which is how a reader hears the revision.
            "DEFINITION, saved \u{b7} r1",
            "Name",
            "bridge_drive",
            "Family",
            "SIN",
            "Quantity",
            "voltage",
            "Value",
            "SIN(0 1 1k)",
            "Parameters",
            "none",
        ] {
            assert!(
                published.contains(&expected.to_owned()),
                "the stage never published {expected:?}: {published:?}"
            );
        }
    }

    /// A selection the library no longer holds — renamed, deleted, or reverted
    /// out from under the workspace — reads as no selection rather than as an
    /// empty definition.
    #[test]
    fn a_selection_the_library_no_longer_holds_reads_as_no_selection() {
        let mut app = RSpiceApp::test_instance();
        app.state
            .workspace
            .stimulus_library
            .insert(sine("bridge_drive"))
            .expect("a fresh name");
        app.state.workbench.selected_stimulus_definition = Some("was_deleted".to_owned());
        assert!(published(&app.state).contains(&"No definition selected".to_owned()));
    }
}
