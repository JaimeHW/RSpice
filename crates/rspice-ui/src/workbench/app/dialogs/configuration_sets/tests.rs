//! Behavioral and transactional coverage for configuration-set dialogs.

use super::*;

#[test]
fn configuration_page_handoff_keeps_immediate_name_input() {
    for label in ["New configuration", "Clone"] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let (mut app, _) = valid_configuration_app();
        open_configuration_sets_dialog(&mut app.state);
        let catalog = app.state.workspace.configuration_sets.clone();
        let mut render = |events| {
            ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(1280.0, 900.0),
                    )),
                    events,
                    ..Default::default()
                },
                |ui| app.render_frame_dialogs(ui),
            )
        };
        let _ = render(Vec::new());
        let output = render(Vec::new());
        let tree = output.platform_output.accesskit_update.unwrap();
        let bounds = tree
            .nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::Button && node.label() == Some(label)
            })
            .unwrap()
            .1
            .bounds()
            .unwrap();
        let pos = egui::pos2(
            ((bounds.x0 + bounds.x1) / 2.0) as f32,
            ((bounds.y0 + bounds.y1) / 2.0) as f32,
        );
        let pointer = |pressed| egui::Event::PointerButton {
            pos,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        let _ = render(vec![egui::Event::PointerMoved(pos), pointer(true)]);
        let _ = render(vec![
            pointer(false),
            egui::Event::Key {
                key: egui::Key::A,
                physical_key: Some(egui::Key::A),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
            },
            egui::Event::Paste("Production bias sweep".to_owned()),
        ]);
        assert_eq!(
            app.state.dialogs.configuration_sets.new_name, "Production bias sweep",
            "{label} opening input"
        );
        assert_eq!(app.state.workspace.configuration_sets, catalog);
        assert!(
            app.state.dialogs.configuration_sets.query.is_empty(),
            "new page typing must not change the manager filter"
        );
    }
}

fn opening_configuration_text(page: ConfigurationDialogPage) {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let (mut app, _) = valid_configuration_app();
    open_configuration_sets_dialog(&mut app.state);
    match page {
        ConfigurationDialogPage::New => app.handle_configuration_body_action(BodyAction::New),
        ConfigurationDialogPage::Clone => app.handle_configuration_body_action(BodyAction::Clone),
        ConfigurationDialogPage::Manager => {}
        ConfigurationDialogPage::Binding => {
            app.handle_configuration_body_action(BodyAction::Binding)
        }
    }
    let catalog = app.state.workspace.configuration_sets.clone();
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1280.0, 800.0),
            )),
            events: vec![
                egui::Event::Key {
                    key: egui::Key::A,
                    physical_key: Some(egui::Key::A),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::CTRL | egui::Modifiers::COMMAND,
                },
                egui::Event::Paste("Production bias sweep".to_owned()),
            ],
            ..Default::default()
        },
        |ui| app.render_frame_dialogs(ui),
    );
    let dialog = &app.state.dialogs.configuration_sets;
    match page {
        ConfigurationDialogPage::Binding => assert_eq!(
            dialog.draft.as_ref().unwrap().executable_view_policy,
            vec!["Production bias sweep"]
        ),
        ConfigurationDialogPage::Manager => assert_eq!(dialog.query, "Production bias sweep"),
        ConfigurationDialogPage::New | ConfigurationDialogPage::Clone => {
            assert_eq!(dialog.new_name, "Production bias sweep")
        }
    }
    assert_eq!(app.state.workspace.configuration_sets, catalog);
}

#[test]
fn initial_focus_accepts_binding_policy_typing() {
    opening_configuration_text(ConfigurationDialogPage::Binding);
}

#[test]
fn initial_focus_accepts_configuration_filter_typing() {
    opening_configuration_text(ConfigurationDialogPage::Manager);
}

#[test]
fn initial_focus_accepts_new_configuration_typing() {
    opening_configuration_text(ConfigurationDialogPage::New);
}

#[test]
fn initial_focus_accepts_cloned_configuration_typing() {
    opening_configuration_text(ConfigurationDialogPage::Clone);
}

#[test]
fn initial_focus_keeps_inspection_only_configuration_fields_read_only() {
    for page in [
        ConfigurationDialogPage::Manager,
        ConfigurationDialogPage::New,
        ConfigurationDialogPage::Clone,
        ConfigurationDialogPage::Binding,
    ] {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
        let (mut app, _) = valid_configuration_app();
        open_configuration_sets_dialog(&mut app.state);
        match page {
            ConfigurationDialogPage::Manager => {}
            ConfigurationDialogPage::New => app.handle_configuration_body_action(BodyAction::New),
            ConfigurationDialogPage::Clone => {
                app.handle_configuration_body_action(BodyAction::Clone)
            }
            ConfigurationDialogPage::Binding => {
                app.handle_configuration_body_action(BodyAction::Binding)
            }
        }
        app.state.workbench.safe_mode.activate(
            crate::workbench::state::LocalSafeModeOptions {
                open_project_read_only: true,
                ..Default::default()
            },
            "UI focus qualification".to_owned(),
        );
        let before = app.state.dialogs.configuration_sets.clone();
        let catalog = app.state.workspace.configuration_sets.clone();
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 900.0),
                )),
                events: vec![egui::Event::Paste("Inspection filter".to_owned())],
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui),
        );
        let dialog = &app.state.dialogs.configuration_sets;
        assert_eq!(dialog.new_name, before.new_name);
        assert_eq!(dialog.draft, before.draft);
        assert_eq!(app.state.workspace.configuration_sets, catalog);
        if page == ConfigurationDialogPage::Manager {
            assert_eq!(dialog.query, "Inspection filter");
        } else {
            assert!(!ctx.memory(|memory| memory.has_focus(name_id(page))
                || memory.has_focus(field_id("ordered-views"))));
        }
    }
}

#[test]
fn closing_in_the_editing_frame_keeps_the_configuration_draft() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx.options_mut(|options| options.max_passes = std::num::NonZeroUsize::new(1).unwrap());
    let (mut app, _) = valid_configuration_app();
    open_configuration_sets_dialog(&mut app.state);
    let catalog = app.state.workspace.configuration_sets.clone();
    let mut render = |events| {
        ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 900.0),
                )),
                events,
                ..Default::default()
            },
            |ui| app.render_frame_dialogs(ui),
        )
    };
    let _ = render(Vec::new());
    let output = render(Vec::new());
    let tree = output.platform_output.accesskit_update.unwrap();
    let close_id = tree
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::Button && node.label() == Some("Close")
        })
        .expect("Close action")
        .0;
    let bounds = tree
        .nodes
        .iter()
        .find(|(_, node)| {
            node.role() == egui::accesskit::Role::TextInput && node.label() == Some("Name")
        })
        .expect("configuration name field")
        .1
        .bounds()
        .unwrap();
    let pos = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let pointer = |pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    let _ = render(vec![egui::Event::PointerMoved(pos), pointer(true)]);
    let _ = render(vec![pointer(false)]);
    let _ = render(vec![
        egui::Event::Paste(" updated".to_owned()),
        egui::Event::AccessKitActionRequest(egui::accesskit::ActionRequest {
            action: egui::accesskit::Action::Click,
            target_tree: egui::accesskit::TreeId::ROOT,
            target_node: close_id,
            data: None,
        }),
    ]);
    let dialog = &app.state.dialogs.configuration_sets;
    assert!(dialog.open);
    assert!(dialog.discard_confirmation);
    assert!(dialog.draft.as_ref().unwrap().name.contains(" updated"));
    assert_eq!(app.state.workspace.configuration_sets, catalog);
}

fn definition(name: &str) -> ConfigurationSetDefinition {
    ConfigurationSetDefinition {
        name: name.to_owned(),
        root: CellViewRef::default_top(),
        dut_path: "/top/XDUT".to_owned(),
        executable_view_policy: vec!["schematic".to_owned()],
        stop_views: Vec::new(),
        unresolved_policy: UnresolvedBindingPolicy::BlockNetlist,
        black_box_policy: ConfigurationBlackBoxPolicy::MaterializedSourceBoundariesOnly,
        overrides: Vec::new(),
        model_profile: ConfigurationModelProfile::ProjectRunSetSections,
        owner: "Local project".to_owned(),
    }
}

fn valid_configuration_app() -> (RSpiceApp, ConfigurationSetId) {
    let mut app = RSpiceApp::test_instance();
    let cell_name = "configuration_test_dut";
    let mut cell = crate::state::Cell::new(cell_name);
    cell.add_view(crate::state::View::new(
        "schematic",
        crate::state::ViewType::Schematic,
    ));
    if let Some(library) = app.state.library_manager.get_library_mut("work") {
        library.add_cell(cell);
    } else {
        let mut library = crate::state::Library::new("work");
        library.add_cell(cell);
        app.state.library_manager.add_library(library);
    }
    let reference = CellViewRef::new("work", cell_name, "schematic");
    let mut master = crate::state::SchematicState::default();
    for (name, position) in [
        ("a", crate::state::Point::new(0, 0)),
        ("b", crate::state::Point::new(40, 0)),
    ] {
        let id = master.add_component(crate::state::ComponentType::Port, position);
        master
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("master port")
            .value = name.to_owned();
    }
    app.state
        .workspace
        .schematic_buffers
        .insert(reference.key(), master);
    let mut binding = crate::state::LibraryCellInstance::new("work", cell_name, "schematic");
    binding.terminal_order = vec!["a".to_owned(), "b".to_owned()];
    app.state
        .schematic
        .add_library_cell_component(crate::state::Point::new(20, 20), binding);
    let mut definition = definition("Release");
    definition.dut_path = "/top/X1".to_owned();
    let id = app
        .state
        .workspace
        .configuration_sets
        .create(definition)
        .expect("valid configuration");
    (app, id)
}

#[test]
fn templates_only_advertise_bindable_view_names() {
    for template in ConfigurationTemplate::ALL {
        let (views, stops) = template.policy();
        assert!(views.iter().all(|view| view_name_rejection(view).is_none()));
        assert!(stops.iter().all(|stop| views.contains(stop)));
    }
}

#[test]
fn a_view_policy_field_refuses_only_what_is_not_a_view_name() {
    assert_eq!(view_policy_rejection(&comma_edit_values("spice_tt,")), None);
    assert_eq!(
        view_policy_rejection(&comma_edit_values("schematic_fast, spice_tt")),
        None
    );
    assert!(
        view_policy_rejection(&comma_edit_values("fast schematic"))
            .is_some_and(|rejection| rejection.contains("fast schematic")),
        "a name the library grammar refuses is quoted back"
    );
    assert!(view_name_rejection("a/b").is_some());
    assert!(view_name_rejection("").is_some());
}

#[test]
fn the_binding_page_states_which_configured_views_the_libraries_hold() {
    let (app, _) = valid_configuration_app();
    let library_views = project_view_names(&app.state.library_manager);
    assert!(library_views.iter().any(|view| view == "schematic"));
    assert!(
        library_views
            .windows(2)
            .all(|pair| pair[0].to_lowercase() <= pair[1].to_lowercase()),
        "the offered names are ordered: {library_views:?}"
    );
    assert!(!library_views.iter().any(|view| view == "spice_tt"));
}

#[test]
fn generated_names_are_case_insensitively_unique() {
    let mut catalog = ConfigurationSetCatalog::default();
    catalog
        .create(definition("Release"))
        .expect("configuration");
    assert_eq!(unique_configuration_name(&catalog, "release"), "release 2");
}

#[test]
fn hierarchy_binding_entry_opens_the_binding_page_and_requires_a_configuration() {
    let (mut app, _) = valid_configuration_app();
    open_configuration_binding_dialog(&mut app.state);
    assert!(app.state.dialogs.configuration_sets.open);
    assert_eq!(
        app.state.dialogs.configuration_sets.page,
        ConfigurationDialogPage::Binding
    );
    assert!(app.state.dialogs.configuration_sets.error.is_none());

    let mut empty = RSpiceApp::test_instance();
    open_configuration_binding_dialog(&mut empty.state);
    assert!(empty.state.dialogs.configuration_sets.open);
    assert_eq!(
        empty.state.dialogs.configuration_sets.page,
        ConfigurationDialogPage::Manager
    );
    assert_eq!(
        empty.state.dialogs.configuration_sets.error.as_deref(),
        Some("Create a configuration set before editing hierarchy bindings.")
    );
}

#[test]
fn same_row_and_subordinate_actions_never_discard_a_dirty_draft() {
    let mut app = RSpiceApp::test_instance();
    let selected = app
        .state
        .workspace
        .configuration_sets
        .create(definition("Release"))
        .expect("configuration");
    let other = app
        .state
        .workspace
        .configuration_sets
        .create(definition("Characterization"))
        .expect("second configuration");
    open_configuration_sets_dialog(&mut app.state);
    app.state
        .dialogs
        .configuration_sets
        .draft
        .as_mut()
        .expect("draft")
        .name = "Unsaved release".to_owned();

    app.handle_configuration_body_action(BodyAction::Select(selected));
    assert_eq!(
        app.state
            .dialogs
            .configuration_sets
            .draft
            .as_ref()
            .map(|draft| draft.name.as_str()),
        Some("Unsaved release")
    );

    for action in [
        BodyAction::New,
        BodyAction::Clone,
        BodyAction::Select(other),
    ] {
        app.handle_configuration_body_action(action);
        assert_eq!(
            app.state.dialogs.configuration_sets.page,
            ConfigurationDialogPage::Manager
        );
        assert_eq!(
            app.state.dialogs.configuration_sets.selected_id,
            Some(selected)
        );
        assert_eq!(
            app.state
                .dialogs
                .configuration_sets
                .draft
                .as_ref()
                .map(|draft| draft.name.as_str()),
            Some("Unsaved release")
        );
        assert!(app.state.dialogs.configuration_sets.error.is_some());
    }
}

#[test]
fn comma_editor_preserves_an_in_progress_trailing_entry() {
    assert_eq!(
        comma_edit_values("schematic,"),
        vec!["schematic".to_owned(), String::new()]
    );
    assert_eq!(
        comma_edit_values("schematic, spice"),
        vec!["schematic".to_owned(), "spice".to_owned()]
    );
    assert!(comma_edit_values("  ").is_empty());
}

#[test]
fn new_configuration_requires_a_real_dut_in_the_selected_root() {
    let mut app = RSpiceApp::test_instance();
    let root = app.state.workspace.active_view.clone();
    assert!(default_dut_path_for_root(&app.state.workspace, &app.state.schematic, &root).is_none());

    for (id, name) in [(1u64, "XB"), (2, "XA")] {
        let mut instance = crate::state::Component::new(
            id,
            crate::state::ComponentType::CellInstance,
            crate::state::Point::new(20 * i32::try_from(id).expect("small id"), 20),
        );
        instance.name = name.to_owned();
        app.state.schematic.components.push(instance);
    }
    assert_eq!(
        default_dut_path_for_root(&app.state.workspace, &app.state.schematic, &root).as_deref(),
        Some("/XA"),
        "the lowest instance name, below the implicit design root"
    );
}

#[test]
fn the_path_fields_take_the_canonical_spelling_and_report_the_text_they_refuse() {
    assert_eq!(instance_path_rejection("/XAFE"), None);
    assert_eq!(instance_path_rejection("/XAFE/XBIAS"), None);
    assert_eq!(instance_path_rejection("/"), None);
    assert!(
        instance_path_rejection("/XAFE/").is_some_and(|rejection| rejection.contains("/XAFE/")),
        "an empty trailing segment is refused and quoted back"
    );
    assert!(
        instance_path_rejection("XAFE").is_some_and(|rejection| rejection.contains("XAFE")),
        "the engine spelling is refused and quoted back"
    );

    assert_eq!(instance_pattern_rejection("/XAFE"), None);
    assert_eq!(instance_pattern_rejection("/XAFE/*"), None);
    assert!(instance_pattern_rejection("XAFE").is_some_and(|rejection| rejection.contains("XAFE")));
    assert!(
        instance_pattern_rejection("/XAFE/").is_some_and(|rejection| rejection.contains("/XAFE/"))
    );
}

#[test]
fn a_new_override_starts_one_instance_below_the_design_root() {
    assert_eq!(new_override_scope(1), "/XINSTANCE1");
    assert_eq!(instance_pattern_rejection(&new_override_scope(2)), None);
}

#[test]
fn normalization_only_save_revalidates_without_mutating_project_authority() {
    let (mut app, id) = valid_configuration_app();
    open_configuration_sets_dialog(&mut app.state);
    let revision = app.state.workspace.project.revision();
    app.state
        .dialogs
        .configuration_sets
        .draft
        .as_mut()
        .expect("configuration draft")
        .name = "  Release  ".to_owned();

    app.commit_configuration_update()
        .expect("normalization-only save validates and reloads");

    assert_eq!(app.state.workspace.project.revision(), revision);
    assert_eq!(
        app.state
            .workspace
            .configuration_sets
            .find(id)
            .expect("configuration remains")
            .name(),
        "Release"
    );
    assert_eq!(
        app.state
            .dialogs
            .configuration_sets
            .draft
            .as_ref()
            .map(|draft| draft.name.as_str()),
        Some("Release")
    );
}

#[test]
fn invalid_save_preserves_catalog_and_project_revision() {
    let (mut app, _) = valid_configuration_app();
    open_configuration_sets_dialog(&mut app.state);
    let catalog = app.state.workspace.configuration_sets.clone();
    let revision = app.state.workspace.project.revision();
    app.state
        .dialogs
        .configuration_sets
        .draft
        .as_mut()
        .expect("configuration draft")
        .dut_path = "/top/XMISSING".to_owned();

    let error = app
        .commit_configuration_update()
        .expect_err("unresolved configured DUT must block publication");

    assert!(error.contains("does not exist"), "{error}");
    assert_eq!(app.state.workspace.configuration_sets, catalog);
    assert_eq!(app.state.workspace.project.revision(), revision);
}
