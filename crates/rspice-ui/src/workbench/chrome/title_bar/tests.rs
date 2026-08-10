//! Tests for the menu bar's contents.
//!
//! The suite asserts that every menu entry corresponds to a real, completed
//! workflow and carries its canonical icon - the title bar must not advertise
//! capability the workbench does not have.

use super::*;

#[cfg(not(target_arch = "wasm32"))]
fn title_test_app() -> RSpiceApp {
    RSpiceApp::test_instance()
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn desktop_and_compact_title_contexts_have_distinct_mockup_ownership() {
    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;

    assert_eq!(
        title_context_text(&app, false),
        app.state.workspace.project.display_name()
    );
    assert_eq!(title_context_text(&app, true), "top · schematic");
    assert_eq!(active_title_cell(&app), "top · schematic");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn closed_project_title_context_does_not_leak_the_previous_document_identity() {
    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;
    app.state.project_lifecycle.project_open = false;

    assert_eq!(title_context_text(&app, false), "RSpice Workbench");
    assert_eq!(title_context_text(&app, true), "No project open");
    assert_eq!(active_title_cell(&app), "No project open");
}

/// The rule above was asserted for years against a `#[cfg(test)]` helper the
/// painter did not call, so the shipped desktop title still read the project
/// name with nothing open. Assert what is painted, not what is computable.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_painted_desktop_title_names_no_project_when_none_is_open() {
    fn painted(project_open: bool) -> String {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = title_test_app();
        app.state.workbench.workspace = Workspace::Design;
        app.state.project_lifecycle.project_open = project_open;
        let bounds = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(640.0, 32.0));
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| {
                    paint_title_context(ui, &app, bounds, false, false)
                });
        });
        let mut text = String::new();
        for shape in &output.shapes {
            if let egui::epaint::Shape::Text(painted) = &shape.shape {
                text.push_str(&painted.galley.job.text);
                text.push('\n');
            }
        }
        text
    }

    let closed = painted(false);
    assert!(closed.contains("RSpice Workbench"), "{closed}");
    assert!(!closed.contains("Untitled Project"), "{closed}");
    assert!(painted(true).contains("Untitled Project"));
}

fn title_key_event(key: Key) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn run_title_menu_frame(ctx: &Context, app: &mut RSpiceApp, events: Vec<egui::Event>) {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1_440.0, 900.0),
        )),
        events,
        ..Default::default()
    };
    let _ = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            menus(ui, app, MenuProjection::All, false, 35.0);
        });
    });
}

fn labels(projection: MenuProjection) -> Vec<&'static str> {
    projection
        .visible_menus()
        .iter()
        .map(|menu| menu.label())
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn rendered_results_menu_labels() -> Vec<String> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(MENU_OUTER_WIDTH, 720.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| results_menu(ui, &mut app));
        },
    );
    let mut items = output
        .platform_output
        .accesskit_update
        .expect("AccessKit results-menu tree")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            if node.role() == egui::accesskit::Role::MenuItem {
                node.label()
                    .zip(node.bounds())
                    .map(|(label, bounds)| (bounds.y0, bounds.x0, label.to_owned()))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        left.0
            .total_cmp(&right.0)
            .then_with(|| left.1.total_cmp(&right.1))
    });
    items.into_iter().map(|(_, _, label)| label).collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn rendered_overflow_menu_labels() -> Vec<String> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(MENU_OUTER_WIDTH, 720.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| {
                    overflow_menu(ui, &mut app, MenuProjection::ThroughModels)
                });
        },
    );
    output
        .platform_output
        .accesskit_update
        .expect("AccessKit overflow-menu tree")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            if node.role() == egui::accesskit::Role::MenuItem {
                node.label().map(str::to_owned)
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn menu_projection_matches_mockup_breakpoints() {
    assert_eq!(MenuProjection::for_width(820.0), MenuProjection::Hidden);
    assert_eq!(
        MenuProjection::for_width(821.0),
        MenuProjection::ThroughSimulate
    );
    assert_eq!(
        MenuProjection::for_width(1020.0),
        MenuProjection::ThroughSimulate
    );
    assert_eq!(
        MenuProjection::for_width(1021.0),
        MenuProjection::ThroughModels
    );
    assert_eq!(
        MenuProjection::for_width(1360.0),
        MenuProjection::ThroughModels
    );
    assert_eq!(MenuProjection::for_width(1361.0), MenuProjection::All);
    assert_eq!(
        MenuProjection::for_layout(844.0, true),
        MenuProjection::Hidden
    );
    assert!(title_context_is_left_aligned(
        MenuProjection::Hidden,
        844.0,
        true
    ));
    assert!(!title_context_is_left_aligned(
        MenuProjection::Hidden,
        820.0,
        true
    ));
}

#[test]
fn title_bar_rules_remain_inside_the_browser_clip() {
    let rect = egui::Rect::from_min_size(egui::pos2(0.0, 7.0), egui::vec2(1_440.0, 35.0));
    let (top, bottom) = title_bar_separator_positions(rect);
    assert_eq!(top, 7.5);
    assert_eq!(bottom, 41.5);
    assert!(top > rect.top());
    assert!(bottom < rect.bottom());
}

#[test]
fn each_projection_exposes_only_the_mockup_menu_prefix() {
    assert!(labels(MenuProjection::Hidden).is_empty());
    assert_eq!(
        labels(MenuProjection::ThroughSimulate),
        ["File", "Edit", "View", "Design", "Simulate"]
    );
    assert_eq!(
        labels(MenuProjection::ThroughModels),
        [
            "File", "Edit", "View", "Design", "Simulate", "Results", "Verify", "Models"
        ]
    );
    assert_eq!(
        labels(MenuProjection::All),
        [
            "File",
            "Edit",
            "View",
            "Design",
            "Simulate",
            "Results",
            "Verify",
            "Models",
            "Automation",
            "Window",
            "Help"
        ]
    );
}

#[test]
fn overflow_and_compact_title_labels_match_the_mockup() {
    assert_eq!(
        MenuProjection::ThroughModels.overflow_trigger_label(),
        "⋯  More"
    );
    assert_eq!(
        MenuProjection::ThroughSimulate.overflow_trigger_label(),
        "⋯"
    );
    assert!(!MenuProjection::Hidden.has_overflow());
    assert!(!MenuProjection::Hidden.shows_title_context());
    assert_eq!(search_button_width(1020.0, false), 31.0);
    assert!(search_button_width(1021.0, false) > 31.0);
    assert_eq!(search_button_width(1440.0, true), 44.0);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn overflow_keeps_mockup_specialist_and_jobs_routes_without_dropping_useful_extras() {
    let labels = rendered_overflow_menu_labels();
    for expected in [
        "Automation workspace",
        "Specialist workspaces\u{2026}",
        "Workspace layouts\u{2026}",
        "Windows, documents and session\u{2026}",
        "Jobs & targets",
        "Focus workspace",
        "Toggle console",
        "RSpice Help",
    ] {
        assert!(
            labels.iter().any(|label| label == expected),
            "overflow is missing {expected:?}: {labels:?}"
        );
    }
}

#[test]
fn menu_geometry_matches_the_mockup_contract() {
    assert_eq!(MENU_OUTER_WIDTH, 244.0);
    assert_eq!(MENU_CONTENT_WIDTH, 232.0);
    assert_eq!(MENU_ROW_HEIGHT, 27.0);
    assert_eq!(MENU_TOUCH_ROW_HEIGHT, 44.0);
    assert_eq!(MENU_MARGIN, 5.0);
    assert_eq!(MENU_BORDER_WIDTH, 1.0);
    assert_eq!(MENU_POPUP_GAP, 2.0);
    assert_eq!(SEARCH_KEYCAP_HEIGHT, 18.0);
    assert_eq!(SEARCH_KEYCAP_MIN_WIDTH, 19.0);
    assert_eq!(SEARCH_KEYCAP_INLINE_PADDING, 4.0);
    assert_eq!(SEARCH_KEYCAP_BORDER_WIDTH, 1.0);
    assert_eq!(
        overflow_trigger_width(MenuProjection::ThroughSimulate, false),
        28.0
    );
    assert_eq!(
        overflow_trigger_width(MenuProjection::ThroughModels, false),
        54.0
    );
    assert_eq!(
        overflow_trigger_width(MenuProjection::ThroughModels, true),
        44.0
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn desktop_menu_triggers_fill_the_title_track_without_vertical_clipping() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_440.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| {
                    menus(ui, &mut app, MenuProjection::All, false, 35.0);
                });
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;
    for label in ["File", "Edit", "View", "Design", "Simulate", "Help"] {
        let bounds = nodes
            .iter()
            .find(|(_, node)| {
                node.role() == egui::accesskit::Role::MenuItem && node.label() == Some(label)
            })
            .and_then(|(_, node)| node.bounds())
            .unwrap_or_else(|| panic!("missing application menu trigger {label}"));
        assert_eq!(bounds.y1 - bounds.y0, 35.0, "clipped trigger {label}");
    }
}

#[test]
fn command_keycap_uses_canonical_intrinsic_border_box_width() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut width = 0.0;
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            width = search_keycap_width(ui, "Ctrl K");
            let text_width = ui
                .painter()
                .layout_no_wrap(
                    "Ctrl K".to_owned(),
                    theme::mono(tokens::FS_1, FontWeight::Regular),
                    egui::Color32::WHITE,
                )
                .size()
                .x;
            assert_eq!(
                width,
                (text_width
                    + SEARCH_KEYCAP_INLINE_PADDING * 2.0
                    + SEARCH_KEYCAP_BORDER_WIDTH * 2.0)
                    .max(SEARCH_KEYCAP_MIN_WIDTH)
            );
        });
    });
    assert!(width >= SEARCH_KEYCAP_MIN_WIDTH);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn f10_then_enter_opens_the_focused_application_menu() {
    let ctx = Context::default();
    let mut app = title_test_app();
    run_title_menu_frame(&ctx, &mut app, vec![title_key_event(Key::F10)]);
    run_title_menu_frame(&ctx, &mut app, Vec::new());

    let trigger_id = Id::new((
        "workbench.title_menu.trigger",
        MenuTriggerKey::Application(ApplicationMenu::File),
    ));
    assert_eq!(ctx.memory(|memory| memory.focused()), Some(trigger_id));

    run_title_menu_frame(&ctx, &mut app, vec![title_key_event(Key::Enter)]);
    let popup_id = trigger_id.with("popup");
    assert!(Popup::is_id_open(&ctx, popup_id));
}

#[test]
fn occurrence_specific_shortcuts_can_be_suppressed_or_overridden() {
    let shortcuts = crate::workbench::ShortcutPreferences::default();
    assert_eq!(
        shortcut_for_occurrence(
            &shortcuts,
            crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
            Command::KeyboardShortcuts,
            Some("")
        ),
        ""
    );
    assert_eq!(
        shortcut_for_occurrence(
            &shortcuts,
            crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
            Command::KeyboardShortcuts,
            Some("Ctrl+Alt+R")
        ),
        "Ctrl+Alt+R"
    );
    assert_eq!(DESCEND_MENU_LABEL, "Descend into selected instance…");
    assert_eq!(COMMAND_REFERENCE_MENU_LABEL, "Command reference");
}

#[test]
fn exit_is_a_desktop_lifecycle_action_not_a_browser_menu_item() {
    use crate::workbench::commands::vocabulary::CommandPlatform;

    assert!(file_menu_shows_exit(CommandPlatform::Desktop));
    for platform in [
        CommandPlatform::Browser,
        CommandPlatform::Tablet,
        CommandPlatform::Phone,
    ] {
        assert!(!file_menu_shows_exit(platform), "{}", platform.label());
    }
}

#[test]
fn file_menu_exposes_the_canonical_drawing_sheet_workflows_in_order() {
    assert_eq!(
        DRAWING_SHEET_FILE_COMMANDS,
        [
            Command::PageSetup,
            Command::SheetFormatManager,
            Command::CustomSheetSizes,
        ]
    );
}

#[test]
fn application_menu_height_is_viewport_bounded() {
    assert_eq!(menu_popup_height_for_viewport(900.0), 560.0);
    assert_eq!(menu_popup_height_for_viewport(600.0), 536.0);
    assert_eq!(menu_popup_height_for_viewport(120.0), MENU_ROW_HEIGHT * 3.0);
}

#[test]
fn menu_shortcuts_are_projected_from_the_typed_registry() {
    let platform = crate::workbench::commands::vocabulary::CommandPlatform::Desktop;
    let shortcuts = crate::workbench::ShortcutPreferences::default();
    for command in [
        Command::OpenProject,
        Command::Save,
        Command::RunSimulation,
        Command::ToggleFullScreen,
        Command::GenerateNetlist,
        Command::ToggleConsole,
    ] {
        assert_eq!(
            shortcut_for_occurrence(
                &shortcuts,
                platform,
                egui::os::OperatingSystem::Windows,
                command,
                None
            ),
            command.default_shortcut_label(platform)
        );
    }
    assert_eq!(
        shortcut_for_occurrence(
            &shortcuts,
            platform,
            egui::os::OperatingSystem::Windows,
            Command::KeyboardShortcuts,
            None
        ),
        ""
    );
}

#[test]
fn core_menu_commands_are_all_real_dispatch_commands() {
    let commands = [
        Command::OpenProject,
        Command::Save,
        Command::Undo,
        Command::PlaceWire,
        Command::PlaceBus,
        Command::PlaceBusTap,
        Command::PlaceJunction,
        Command::RunSimulation,
        Command::WaveformCalculator,
        Command::PdkSettings,
        Command::AutomationConsole,
    ];
    assert!(commands.iter().all(|command| !command.spec().id.is_empty()));
}

#[test]
fn design_placement_menu_matches_the_mockup_order() {
    assert_eq!(
        DESIGN_PLACEMENT_COMMANDS,
        [
            Command::PlaceInstance,
            Command::PlaceWire,
            Command::PlaceBus,
            Command::PlaceBusTap,
            Command::PlaceJunction,
            Command::PlaceLabel,
            Command::PlaceProbe,
            Command::PlacePin,
            Command::PlaceText,
            Command::PlaceShape,
        ]
    );
}

#[test]
fn window_layout_menu_places_split_with_results_before_focus_workspace() {
    assert_eq!(
        WINDOW_LAYOUT_COMMANDS,
        [
            Command::ToggleNavigator,
            Command::ToggleInspector,
            Command::ToggleConsole,
            Command::ToggleResultsSplit,
            Command::ToggleFocusMode,
        ]
    );
}

#[test]
fn brand_lockup_does_not_reserve_space_after_the_wordmark() {
    assert_eq!(BRAND_WORDMARK_WIDTH, 88.0);
}

#[test]
fn design_edit_menu_matches_the_upgraded_authoring_group() {
    assert_eq!(
        DESIGN_EDIT_COMMANDS,
        [
            Command::MoveSelection,
            Command::StretchSelection,
            Command::ArraySelection,
            Command::ReplaceInstance,
        ]
    );
}

#[test]
fn design_management_menu_matches_the_upgraded_governed_group() {
    assert_eq!(
        DESIGN_MANAGEMENT_COMMANDS,
        [
            Command::CreateHierarchy,
            Command::DesignManagement,
            Command::SelectionBulkEdit,
            Command::ConnectivityManager,
            Command::ConfigurationSets,
            Command::ReviewComments,
            Command::RevisionHistory,
        ]
    );
}

#[test]
fn upgraded_design_menu_commands_remain_palette_discoverable() {
    for command in DESIGN_PLACEMENT_COMMANDS
        .into_iter()
        .chain(DESIGN_EDIT_COMMANDS)
        .chain(DESIGN_MANAGEMENT_COMMANDS)
    {
        assert!(command.palette_visible());
    }
    for command in [
        Command::CreateHierarchy,
        Command::DesignSpecialistWorkspaces,
        Command::CheckAndSave,
    ] {
        assert!(command.palette_visible());
    }
}

#[test]
fn design_specialist_row_uses_the_mockup_scoped_command_identity() {
    assert_eq!(
        Command::DesignSpecialistWorkspaces.stable_id(),
        "specialist-tools-design"
    );
    assert_eq!(
        Command::DesignSpecialistWorkspaces.spec().label,
        "Specialist workspaces…"
    );
    assert_eq!(
        command_icon(Command::DesignSpecialistWorkspaces),
        WorkbenchIcon::Grid
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn results_menu_exposes_only_truthful_completed_result_workflows() {
    let labels = rendered_results_menu_labels();
    assert_eq!(
        labels,
        [
            "Open results workspace",
            "Dataset and manifest browser…",
            "Create result document…",
            "Add result comparison…",
            "Report page and datasheet editor…",
            "Trace and family manager…",
            "Cursor groups and links…",
            "Review notes…",
            "Calculator…",
            "Measurement library…",
            "Family slicing and pivot…",
            "Plot document properties…",
            "Import result dataset…",
            "Export dataset…",
        ]
    );
}

#[test]
fn canonical_menu_icons_do_not_fall_back_to_generic_file_glyphs() {
    for (command, icon) in [
        (Command::NewProject, WorkbenchIcon::File),
        (Command::RevertActiveDocument, WorkbenchIcon::Refresh),
        (Command::CloseActiveDocument, WorkbenchIcon::File),
        (Command::CloseProject, WorkbenchIcon::Folder),
        (Command::Exit, WorkbenchIcon::Stop),
        (Command::Copy, WorkbenchIcon::Copy),
        (Command::Delete, WorkbenchIcon::Trash),
        (Command::SelectAll, WorkbenchIcon::Grid),
        (Command::RenameSelection, WorkbenchIcon::Label),
        (Command::ObjectProperties, WorkbenchIcon::Sliders),
        (Command::ResetActiveView, WorkbenchIcon::Refresh),
        (Command::AscendHierarchy, WorkbenchIcon::ArrowLeft),
        (Command::DescendHierarchy, WorkbenchIcon::Folder),
        (Command::PlaceInstance, WorkbenchIcon::Component),
        (Command::CompileVerilogA, WorkbenchIcon::Code),
        (Command::AutomationConsole, WorkbenchIcon::Terminal),
        (Command::KeyboardShortcuts, WorkbenchIcon::Search),
        (Command::InteroperabilityMatrix, WorkbenchIcon::Compare),
    ] {
        assert_eq!(command_icon(command), icon, "{}", command.spec().id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn title_context_ellipsis_is_measured_instead_of_character_estimated() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut fitted = String::new();
    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let font = theme::sans(tokens::FS_0, FontWeight::Regular);
            fitted = ellipsize_to_width(
                ui.painter(),
                "WWW · precision-sensor-front-end",
                &font,
                52.0,
            );
            let width = ui
                .painter()
                .layout_no_wrap(fitted.clone(), font, egui::Color32::WHITE)
                .size()
                .x;
            assert!(width <= 52.0);
        });
    });
    assert!(fitted.ends_with('…'));
    assert_ne!(fitted, "WWW · precision-sensor-front-end");
}
