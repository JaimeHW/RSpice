//! The configuration dialogs as a reader meets them.

use super::*;

use crate::workbench::examples::hierarchy_reference;

fn reference_app() -> (RSpiceApp, ConfigurationSetId) {
    let mut app = RSpiceApp::test_instance();
    let fixture = hierarchy_reference::build();
    app.state = fixture.state;
    (app, fixture.configuration)
}

/// Every accessible name and value one frame of the open dialog announces.
fn announced(app: &mut RSpiceApp) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            ..Default::default()
        },
        |ui| app.render_configuration_dialog(ui.ctx()),
    );
    output
        .platform_output
        .accesskit_update
        .expect("an accessibility update")
        .nodes
        .iter()
        .flat_map(|(_, node)| {
            [node.label(), node.value(), node.description()]
                .into_iter()
                .flatten()
                .map(str::to_owned)
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn new_configuration_asks_for_a_name_a_start_and_whether_runs_use_it() {
    let (mut app, _) = reference_app();
    open_configuration_dialog(&mut app.state, ConfigurationDialogRequest::New);
    let Some(ConfigurationDialog::New { use_for_runs, .. }) =
        &app.state.project.configuration_dialog.open
    else {
        panic!("New configuration is open");
    };
    assert!(
        !use_for_runs,
        "with a configuration already there, a new one waits to be chosen"
    );
    let words = announced(&mut app);
    for expected in [
        "New configuration",
        "Name",
        "Start from",
        "Top cell",
        "Use for runs",
    ] {
        assert!(
            words.iter().any(|word| word.contains(expected)),
            "{expected:?} not in {words:#?}"
        );
    }
    for banned in [
        "contract",
        "lineage",
        "semantic",
        "catalog",
        "Testbench configuration sets",
    ] {
        assert!(
            !words.iter().any(|word| word
                .to_ascii_lowercase()
                .contains(&banned.to_ascii_lowercase())),
            "{banned:?} in {words:#?}"
        );
    }
}

#[test]
fn a_first_configuration_is_used_for_runs_and_a_clone_follows_its_source() {
    let mut app = RSpiceApp::test_instance();
    open_configuration_dialog(&mut app.state, ConfigurationDialogRequest::New);
    assert!(matches!(
        app.state.project.configuration_dialog.open,
        Some(ConfigurationDialog::New {
            use_for_runs: true,
            ..
        })
    ));

    let (mut app, id) = reference_app();
    open_configuration_dialog(&mut app.state, ConfigurationDialogRequest::Clone(id));
    assert!(matches!(
        &app.state.project.configuration_dialog.open,
        Some(ConfigurationDialog::Clone {
            use_for_runs: true,
            name,
            ..
        }) if name == "Reference hierarchy copy"
    ));
}

#[test]
fn default_offers_only_a_new_configuration() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut offered = Vec::new();
    for selection in [
        ConfigurationSelection::Default,
        ConfigurationSelection::Configuration(ConfigurationSetId::new()),
    ] {
        let output = ctx.run_ui(Default::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                configuration_actions_menu(ui, selection, true);
            });
        });
        // The accessibility tree is unordered; the set is what is pinned.
        let mut labels = output
            .platform_output
            .accesskit_update
            .unwrap()
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect::<Vec<_>>();
        labels.sort();
        offered.push(labels);
    }
    assert_eq!(offered[0], ["New configuration\u{2026}"]);
    assert_eq!(
        offered[1],
        [
            "Clone\u{2026}",
            "Delete",
            "New configuration\u{2026}",
            "Rename\u{2026}"
        ]
    );
}

#[test]
fn deleting_what_runs_use_says_why_it_cannot() {
    let (mut app, id) = reference_app();
    open_configuration_dialog(&mut app.state, ConfigurationDialogRequest::Delete(id));
    let words = announced(&mut app);
    assert!(
        words
            .iter()
            .any(|word| word.contains(&delete_refusal("Reference hierarchy"))),
        "{words:#?}"
    );
}
