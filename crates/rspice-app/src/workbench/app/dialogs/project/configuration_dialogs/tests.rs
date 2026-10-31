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
    open_configuration_dialog(
        &mut app.state.project.configuration_dialog,
        &app.state.model,
        ConfigurationDialogRequest::New,
    );
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
fn starting_from_default_writes_every_view_default_stops_at() {
    let (views, stops) = StartFrom::Default.lists();
    assert_eq!(views[..3], ["schematic", "extracted", "spice"]);
    let default_stops = DEFAULT_STOP_VIEW_TYPES
        .iter()
        .map(ViewType::display_name)
        .collect::<Vec<_>>();
    assert_eq!(stops, default_stops);
    for stop in &stops {
        assert!(
            views.contains(stop),
            "stop {stop:?} must be in the view list {views:?}"
        );
    }
}

#[test]
fn a_first_configuration_is_used_for_runs_and_a_clone_follows_its_source() {
    let mut app = RSpiceApp::test_instance();
    open_configuration_dialog(
        &mut app.state.project.configuration_dialog,
        &app.state.model,
        ConfigurationDialogRequest::New,
    );
    assert!(matches!(
        app.state.project.configuration_dialog.open,
        Some(ConfigurationDialog::New {
            use_for_runs: true,
            ..
        })
    ));

    let (mut app, id) = reference_app();
    open_configuration_dialog(
        &mut app.state.project.configuration_dialog,
        &app.state.model,
        ConfigurationDialogRequest::Clone(id),
    );
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
    open_configuration_dialog(
        &mut app.state.project.configuration_dialog,
        &app.state.model,
        ConfigurationDialogRequest::Delete(id),
    );
    let words = announced(&mut app);
    assert!(
        words
            .iter()
            .any(|word| word.contains(&delete_refusal("Reference hierarchy"))),
        "{words:#?}"
    );
}

fn edit_views(app: &mut RSpiceApp, id: ConfigurationSetId, views: &[&str], stops: &[&str]) {
    let catalog = app.state.model.workspace.content.configuration_sets.clone();
    app.state
        .project
        .configuration
        .edit(&catalog, id, |definition| {
            definition.executable_view_policy =
                views.iter().map(|view| (*view).to_owned()).collect();
            definition.stop_views = stops.iter().map(|view| (*view).to_owned()).collect();
        });
}

fn leaving(app: &RSpiceApp) -> Option<&Leaving> {
    app.state.project.configuration_dialog.leaving.as_ref()
}

/// One frame of the configuration dialogs with `keys` pressed.
fn press(app: &mut RSpiceApp, ctx: &egui::Context, keys: &[egui::Key]) {
    let events = keys
        .iter()
        .map(|&key| egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        })
        .collect();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ui| app.render_configuration_dialog(ui.ctx()),
    );
}

#[test]
fn leaving_the_project_asks_about_configuration_edits_first() {
    let (mut app, id) = reference_app();
    assert!(
        !hold_for_configuration_edits(&mut app.state, ConfirmationAction::ProjectNew, None),
        "with nothing unapplied there is nothing to ask"
    );
    edit_views(&mut app, id, &["schematic", "spice"], &[]);
    let committed = app.state.model.workspace.content.configuration_sets.clone();

    // Cancel keeps the project and the edits.
    assert!(hold_for_configuration_edits(
        &mut app.state,
        ConfirmationAction::ProjectNew,
        None
    ));
    let words = announced(&mut app);
    assert!(
        words
            .iter()
            .any(|word| word.contains("Apply your edits to Reference hierarchy?")),
        "{words:#?}"
    );
    assert_eq!(
        answer_leaving(&mut app.state, DialogChoice::None),
        LeavingAnswer::Stay
    );
    assert!(
        leaving(&app).is_some(),
        "an unanswered question stays asked"
    );
    assert_eq!(
        answer_leaving(&mut app.state, DialogChoice::Ghost),
        LeavingAnswer::Stay
    );
    assert!(leaving(&app).is_none());
    assert!(app.state.project.configuration.is_dirty(&committed));

    // Discard drops them, then asks about the project's unsaved work as New
    // project always does.
    assert!(crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(&app.state));
    assert!(hold_for_configuration_edits(
        &mut app.state,
        ConfirmationAction::ProjectNew,
        None
    ));
    let LeavingAnswer::GoOn(going) = answer_leaving(&mut app.state, DialogChoice::Secondary) else {
        panic!("Discard goes on leaving");
    };
    assert!(app.state.project.configuration.draft().is_none());
    assert_eq!(
        app.state.model.workspace.content.configuration_sets,
        committed
    );
    assert_eq!(go_on_leaving(&mut app.state, going), None);
    assert!(app.state.workbench.confirmation_dialog.visible);
    assert_eq!(
        app.state.workbench.confirmation_dialog.pending_action,
        Some(ConfirmationAction::ProjectNew)
    );
}

#[test]
fn applying_before_leaving_commits_the_edits_and_keeps_the_recent_project_asked_for() {
    let (mut app, id) = reference_app();
    edit_views(&mut app, id, &["schematic", "spice"], &[]);
    let path = std::path::PathBuf::from("other.rspiceproj");
    assert!(hold_for_configuration_edits(
        &mut app.state,
        ConfirmationAction::OpenRecent,
        Some((path.clone(), RecentKind::Project)),
    ));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    press(&mut app, &ctx, &[]);
    press(&mut app, &ctx, &[egui::Key::Enter]);

    let catalog = &app.state.model.workspace.content.configuration_sets;
    assert_eq!(
        catalog.find(id).unwrap().executable_view_policy(),
        ["schematic", "spice"]
    );
    assert!(leaving(&app).is_none());
    // The applied edit changed the project, so the save question follows,
    // still about the project that was asked for.
    let confirmation = &app.state.workbench.confirmation_dialog;
    assert!(confirmation.visible);
    assert_eq!(
        confirmation.pending_action,
        Some(ConfirmationAction::OpenRecent)
    );
    assert_eq!(confirmation.pending_path.as_ref(), Some(&path));
    assert_eq!(confirmation.pending_recent_kind, Some(RecentKind::Project));
}

#[test]
fn a_refused_apply_before_leaving_stays_on_the_configuration_page() {
    let (mut app, id) = reference_app();
    edit_views(&mut app, id, &["schematic"], &["extracted"]);
    let committed = app.state.model.workspace.content.configuration_sets.clone();
    app.state.project.project_page = ProjectPage::Overview;
    assert!(hold_for_configuration_edits(
        &mut app.state,
        ConfirmationAction::Exit,
        None
    ));
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);

    press(&mut app, &ctx, &[]);
    press(&mut app, &ctx, &[egui::Key::Enter]);

    assert!(leaving(&app).is_none());
    assert!(!app.state.shell.exit_requested);
    assert!(!app.state.workbench.confirmation_dialog.visible);
    assert_eq!(
        app.state.model.workspace.content.configuration_sets,
        committed
    );
    assert!(app.state.project.configuration.is_dirty(&committed));
    assert!(
        app.state
            .project
            .configuration
            .refusal
            .as_deref()
            .is_some_and(|refusal| refusal.starts_with("Cannot apply Reference hierarchy")),
        "{:?}",
        app.state.project.configuration.refusal
    );
    assert_eq!(app.state.project.project_page, ProjectPage::Configuration);
}
