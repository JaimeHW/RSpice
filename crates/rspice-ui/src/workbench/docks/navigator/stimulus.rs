//! The Stimulus Library's browser: one row per definition the project owns.
//!
//! The rows are the whole navigator. A definition is not a tree — it has no
//! children, and the scopes the design draws over it (adopted, unadopted, and
//! the behind count beside each row) are readings of the placed sources that
//! adopted it, which nothing in the shell computes yet. Inventing those
//! facets here would give the reader three filters that all return the same
//! list.

use egui::{ScrollArea, Ui};

use crate::workbench::{AppState, MessageId};

use super::super::super::design_system::{WorkbenchIcon, section_header};
use super::{empty_navigator_row, nav_matches, nav_row};

pub(super) fn show(ui: &mut Ui, state: &mut AppState) {
    let messages = state.ui.messages();
    let query = state.workbench.navigator_filter().trim().to_lowercase();
    let selected = state.workbench.selected_stimulus_definition.clone();
    // The rows are read off the library before anything is drawn, so the
    // click that changes the selection does not have to fight the borrow that
    // produced the row it was on.
    let rows = state
        .workspace
        .stimulus_library
        .definitions()
        .iter()
        .filter(|definition| nav_matches(&query, definition.name()))
        .map(|definition| {
            (
                definition.name().to_owned(),
                messages.format(
                    MessageId::StimulusBrowserRowMeta,
                    &[
                        ("family", definition.family().label()),
                        ("revision", &definition.revision().to_string()),
                    ],
                ),
            )
        })
        .collect::<Vec<_>>();

    if state.workspace.stimulus_library.is_empty() {
        // An empty library and a query that matched nothing must not look
        // alike: the first is the project's state, the second is the
        // reader's.
        empty_navigator_row(ui, &messages.text(MessageId::StimulusBrowserEmpty));
        return;
    }
    section_header(
        ui,
        &messages.text(MessageId::StimulusFieldDefinitions),
        Some(&rows.len().to_string()),
    );
    if rows.is_empty() {
        empty_navigator_row(ui, &messages.text(MessageId::StimulusBrowserNoMatch));
        return;
    }

    let mut chosen = None;
    ScrollArea::vertical()
        .id_salt("workbench.stimulus.navigator")
        .show(ui, |ui| {
            for (name, meta) in &rows {
                let active = selected
                    .as_deref()
                    .is_some_and(|current| current.eq_ignore_ascii_case(name));
                if nav_row(ui, WorkbenchIcon::Source, name, active, Some(meta)) {
                    chosen = Some(name.clone());
                }
            }
        });
    if let Some(name) = chosen {
        state.workbench.selected_stimulus_definition = Some(name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ComponentType;
    use crate::state::stimulus_library::definition::StimulusDefinition;
    use crate::workbench::RSpiceApp;

    fn published(state: &mut AppState) -> Vec<String> {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    // The navigator dock's own minimum, at the fit the
                    // workspace is held to.
                    egui::vec2(228.0, 640.0),
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
            .expect("AccessKit browser tree")
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

    fn library(state: &mut AppState, names: &[&str]) {
        for name in names {
            state
                .workspace
                .stimulus_library
                .insert(
                    StimulusDefinition::new(*name, ComponentType::VoltageSourceSin)
                        .expect("a sine source is one of the twenty-two"),
                )
                .expect("a fresh name");
        }
    }

    #[test]
    fn every_definition_the_project_owns_gets_a_row() {
        let mut app = RSpiceApp::test_instance();
        library(&mut app.state, &["bridge_drive", "vdd_operate"]);
        let published = published(&mut app.state);
        assert!(published.contains(&"bridge_drive".to_owned()));
        assert!(published.contains(&"vdd_operate".to_owned()));
    }

    /// A library nobody has authored into and a filter nobody's definitions
    /// match are different facts, and a browser that drew the same blank panel
    /// for both would send the reader looking for definitions that are there.
    #[test]
    fn an_unauthored_library_and_a_filter_that_matched_nothing_read_differently() {
        let mut empty = RSpiceApp::test_instance();
        assert!(
            published(&mut empty.state)
                .contains(&"This project has no stimulus definitions".to_owned())
        );

        let mut filtered = RSpiceApp::test_instance();
        library(&mut filtered.state, &["bridge_drive"]);
        filtered
            .state
            .workbench
            .set_navigator_filter("nothing_matches_this");
        let published = published(&mut filtered.state);
        assert!(published.contains(&"No definition matches this filter".to_owned()));
        assert!(!published.contains(&"This project has no stimulus definitions".to_owned()));
    }
}
