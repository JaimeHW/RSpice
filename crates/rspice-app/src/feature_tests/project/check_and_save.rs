//! Check and save through the whole application: the command form drawn
//! without the wizard, and a landed save closing it with what it missed.

use egui::Context;

use crate::workbench::app::RSpiceApp;
use crate::workbench::app::dialogs::check_and_save::{NEWER_CHANGES, complete_validated_save};
use crate::workbench::app_state::AppState;

/// A save that landed closes the form, and edits that missed it are
/// said in the record rather than in a dialog left open to say it.
#[test]
fn a_landed_save_closes_the_form_and_records_what_it_missed() {
    let mut state = AppState::default();
    state.project.check_and_save.open = true;
    state.project.check_and_save.revision_note = "Compensation update".to_owned();
    complete_validated_save(&mut state, true);
    assert!(!state.project.check_and_save.open);
    assert!(state.project.check_and_save.revision_note.is_empty());
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the save is recorded");
    assert!(line.message.starts_with("Saved a validated revision"));
    assert!(line.message.ends_with(NEWER_CHANGES), "{}", line.message);
}

/// Check and save is a command form: its rows on one label grid, and
/// none of the wizard it was drawn as — four step tabs, a purpose line, a
/// "Validated snapshot" side table — nor the sample revision note.
#[test]
fn check_and_save_is_a_command_form_without_the_wizard() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    app.execute_command_key(
        crate::workbench::project::commands::ProjectCommand::CheckAndSave.key(),
    );
    assert!(app.state.project.check_and_save.open);

    let mut render = || {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui.ctx()),
        )
    };
    let _ = render();
    let nodes = render()
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    // Every case is collected rather than asserted on the spot, so one
    // run names every row that stopped publishing instead of only the
    // first.
    let mut unpublished = Vec::new();
    let mut published = |what: &str, matches: &dyn Fn(&egui::accesskit::Node) -> bool| {
        let found = nodes
            .iter()
            .map(|(_, node)| node)
            .find(|node| matches(node));
        match found {
            None => unpublished.push(format!("{what}: no node carries it")),
            Some(node) => match node.bounds() {
                None => unpublished.push(format!("{what}: published without bounds")),
                Some(bounds) if bounds.x1 <= bounds.x0 || bounds.y1 <= bounds.y0 => {
                    unpublished.push(format!("{what}: published with an empty rectangle"));
                }
                Some(_) => {}
            },
        }
    };

    // The two facts, each as the pair it paints, and the note field.
    for fact in ["Checks", "Documents"] {
        published(fact, &|node| {
            node.label() == Some(fact) && node.value().is_some_and(|value| !value.is_empty())
        });
    }
    published("Revision note", &|node| {
        node.label() == Some("Revision note")
    });

    assert!(
        unpublished.is_empty(),
        "Check and save paints rows that never reach the accessibility tree:\n  {}",
        unpublished.join("\n  ")
    );

    let wizard = nodes
        .iter()
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .filter(|text| {
            text.contains("Scope")
                || text.contains("Difference")
                || text.contains("snapshot")
                || text.contains("Update input-pair compensation")
                || text.contains("before explicit save")
        })
        .map(str::to_owned)
        .collect::<Vec<_>>();
    assert!(wizard.is_empty(), "the wizard is back: {wizard:?}");
}
