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
        app.state.workspace.content.project.display_name()
    );
    assert_eq!(title_context_text(&app, true), "top · schematic");
    assert_eq!(active_title_cell(&app), "top · schematic");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn closed_project_title_context_does_not_leak_the_previous_document_identity() {
    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;
    app.state.project_lifecycle.authority.close_project();

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
        if !project_open {
            app.state.project_lifecycle.authority.close_project();
        }
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

/// Unsaved project changes are stated once, as a `*` after the visible title,
/// the way Virtuoso and ADS mark them. The status bar no longer says it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn unsaved_project_changes_put_a_star_after_the_painted_title() {
    fn painted(app: &RSpiceApp, width: f32) -> String {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let bounds = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(width, 32.0));
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| paint_title_context(ui, app, bounds, false, false));
        });
        output
            .shapes
            .iter()
            .filter_map(|shape| match &shape.shape {
                egui::epaint::Shape::Text(painted) => Some(painted.galley.job.text.clone()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;
    assert_eq!(painted(&app, 640.0), "Untitled Project");

    app.state.schematic.session.is_dirty = true;
    assert_eq!(painted(&app, 640.0), "Untitled Project*");
    assert!(
        painted(&app, 90.0).ends_with("\u{2026}*"),
        "a narrow title shortens the name and keeps the marker: {}",
        painted(&app, 90.0)
    );

    app.state.project_lifecycle.authority.close_project();
    assert_eq!(painted(&app, 640.0), "RSpice Workbench");
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

/// Draw `entries` the way an open menu draws them.
#[cfg(not(target_arch = "wasm32"))]
fn draw_menu_entries(ui: &mut Ui, app: &RSpiceApp, entries: &[MenuEntry]) {
    let rows = menu_rows(entries, app, ui.ctx());
    let _ = show_menu_rows(ui, &rows);
}

/// Every command `entries` list, in order, whatever context their blocks
/// show in; a submenu's commands follow its row.
fn listed_commands(entries: &[MenuEntry]) -> Vec<Command> {
    let mut commands = Vec::new();
    for entry in entries {
        match *entry {
            MenuEntry::Command(item) => commands.push(item.command),
            MenuEntry::Submenu { entries, .. } | MenuEntry::Only(_, entries) => {
                commands.extend(listed_commands(entries));
            }
            MenuEntry::Separator => {}
        }
    }
    commands
}

/// `menu` lists `run` as consecutive rows.
fn assert_menu_lists(menu: ApplicationMenu, run: &[Command]) {
    let listed = listed_commands(menu.entries());
    assert!(
        listed.windows(run.len()).any(|window| window == run),
        "{} does not list {run:?} in order: {listed:?}",
        menu.label()
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn rendered_results_menu_labels() -> Vec<String> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let app = title_test_app();
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
                    draw_menu_entries(ui, &app, ApplicationMenu::Results.entries())
                });
        },
    );
    let mut items = output
        .platform_output
        .accesskit_update
        .expect("AccessKit results-menu tree")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            if matches!(
                node.role(),
                egui::accesskit::Role::MenuItem | egui::accesskit::Role::MenuItemCheckBox
            ) {
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

/// Simulate-menu rows as the platform sees them: the painted label paired with
/// the description the row publishes only when its command is unavailable,
/// beside what the plan-manager command said about itself while they were
/// painted.
///
/// The application is built here rather than borrowed from the caller, the way
/// [`rendered_results_menu_labels`] does it. A helper that took the whole app
/// mutably would hand every caller reach over every subsystem to render a menu;
/// what the callers actually vary is whether the active plan has stable
/// analysis identity, so that is the parameter. Returning the availability read
/// from this same instance is what lets a caller compare a row's published
/// description against the command's own refusal without a second app to
/// disagree with.
#[cfg(not(target_arch = "wasm32"))]
fn rendered_simulate_menu_rows(
    stable_analysis_plan: bool,
) -> (CommandAvailability, Vec<(String, Option<String>)>) {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.project_lifecycle.authority.open_session();
    if !stable_analysis_plan {
        app.state.sim_setup.analysis_plan = None;
    }
    let availability = Command::ManageSimulationPlans.availability(&app);
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
                    draw_menu_entries(ui, &app, ApplicationMenu::Simulate.entries())
                });
        },
    );
    let mut items = output
        .platform_output
        .accesskit_update
        .expect("AccessKit simulate-menu tree")
        .nodes
        .into_iter()
        .filter_map(|(_, node)| {
            if node.role() == egui::accesskit::Role::MenuItem {
                node.label().zip(node.bounds()).map(|(label, bounds)| {
                    (
                        bounds.y0,
                        bounds.x0,
                        label.to_owned(),
                        node.description().map(str::to_owned),
                    )
                })
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
    (
        availability,
        items
            .into_iter()
            .map(|(_, _, label, description)| (label, description))
            .collect(),
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn simulate_menu_row_description(
    rows: &[(String, Option<String>)],
    command: Command,
) -> Option<Option<String>> {
    rows.iter()
        .find(|(label, _)| label.as_str() == command.spec().label)
        .map(|(_, description)| description.clone())
}

#[cfg(not(target_arch = "wasm32"))]
fn rendered_overflow_menu_labels() -> Vec<String> {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let app = title_test_app();
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
                    draw_menu_entries(ui, &app, overflow_entries(MenuProjection::ThroughModels))
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
            if matches!(
                node.role(),
                egui::accesskit::Role::MenuItem | egui::accesskit::Role::MenuItemCheckBox
            ) {
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
        "Focus mode",
        "Console",
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
    assert_eq!(
        Command::DescendHierarchy.spec().label,
        "Descend into selected instance…"
    );
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
    assert_menu_lists(
        ApplicationMenu::File,
        &[
            Command::SheetProperties,
            Command::SheetFormatManager,
            Command::CustomSheetSizes,
            Command::SheetDefaults,
        ],
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

/// No View-menu row is drawn twice.
///
/// `Visibility options…` was listed in the viewport group and then emitted a
/// second time immediately after the loop, so the menu showed the same row
/// twice in a row with the same shortcut and the same enablement.
#[test]
fn the_view_menu_lists_each_viewport_row_exactly_once() {
    let mut seen = std::collections::BTreeSet::new();
    let listed = listed_commands(ApplicationMenu::View.entries());
    for command in &listed {
        assert!(
            seen.insert(command.spec().id),
            "{command:?} appears twice in the View menu"
        );
    }
    assert_eq!(seen.len(), listed.len());
    assert!(listed.contains(&Command::VisibilityOptions));
}

const DESIGN_PLACEMENT_COMMANDS: [Command; 10] = [
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
];
const DESIGN_EDIT_COMMANDS: [Command; 4] = [
    Command::MoveSelection,
    Command::StretchSelection,
    Command::ArraySelection,
    Command::ReplaceInstance,
];
const DESIGN_MANAGEMENT_COMMANDS: [Command; 8] = [
    Command::CreateHierarchy,
    Command::DesignManagement,
    Command::SelectionBulkEdit,
    Command::ConnectivityManager,
    Command::ConfigurationSets,
    Command::RuleSetup,
    Command::ReviewComments,
    Command::RevisionHistory,
];
const DESIGN_SHEET_COMMANDS: [Command; 4] = [
    Command::PreviousSheet,
    Command::NextSheet,
    Command::NewSheet,
    Command::DeleteSheet,
];

#[test]
fn design_placement_menu_matches_the_mockup_order() {
    assert_menu_lists(ApplicationMenu::Design, &DESIGN_PLACEMENT_COMMANDS);
}

#[test]
fn window_layout_menu_places_split_with_results_before_focus_workspace() {
    assert_menu_lists(
        ApplicationMenu::Window,
        &[
            Command::ToggleNavigator,
            Command::ToggleInspector,
            Command::ToggleConsole,
            Command::ToggleResultsSplit,
            Command::ToggleFocusMode,
        ],
    );
}

#[test]
fn brand_lockup_does_not_reserve_space_after_the_wordmark() {
    assert_eq!(BRAND_WORDMARK_WIDTH, 88.0);
}

#[test]
fn design_edit_menu_matches_the_upgraded_authoring_group() {
    assert_menu_lists(ApplicationMenu::Design, &DESIGN_EDIT_COMMANDS);
}

/// One frame of what `draw` paints, returning the labels of the menu items
/// the platform sees and the label of the one with the keyboard focus.
#[cfg(not(target_arch = "wasm32"))]
fn menu_items_after_frame(
    ctx: &Context,
    events: Vec<egui::Event>,
    mut draw: impl FnMut(&mut Ui),
) -> (Vec<String>, Option<String>) {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1_440.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| draw(ui));
        },
    );
    let update = output
        .platform_output
        .accesskit_update
        .expect("AccessKit menu tree");
    let label_of = |id| {
        update
            .nodes
            .iter()
            .find(|(node_id, _)| *node_id == id)
            .and_then(|(_, node)| node.label().map(str::to_owned))
    };
    let items = update
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::MenuItem)
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect();
    (items, label_of(update.focus))
}

/// Rotate, Mirror, Align and Distribute were reachable only from the command
/// palette: no menu listed them (audit E-9, G-13). The Design menu's Arrange
/// row opens a submenu of all twelve. Right arrow walks into it with its
/// first row focused, Down steps one row, Enter runs the row, and Left arrow
/// walks back out onto Arrange.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_design_menu_arrange_submenu_reaches_every_arrange_command() {
    use crate::state::{Component, ComponentType, Point};

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.workbench.activate(Workspace::Design);
    let document = app.state.schematic.document_mut_for_test();
    document.components = vec![
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)),
        Component::new(2, ComponentType::Resistor, Point::new(200, 100)),
    ];
    let selection = &mut app.state.schematic.session.editor.selection;
    selection.select_component(1);
    selection.select_component(2);
    let arrange = menu_entries::submenu_row_id("Arrange");
    let design_menu = Id::new((
        "workbench.title_menu.trigger",
        MenuTriggerKey::Application(ApplicationMenu::Design),
    ))
    .with("popup");
    let arrange_labels: Vec<&str> = ApplicationMenu::Design
        .entries()
        .iter()
        .find_map(|entry| match entry {
            MenuEntry::Submenu {
                label: "Arrange",
                entries,
                ..
            } => Some(listed_commands(entries)),
            _ => None,
        })
        .expect("the Design menu lists Arrange")
        .iter()
        .map(|command| command.spec().label)
        .collect();
    assert_eq!(arrange_labels.len(), 13);
    let before = app.state.schematic.document().components[1].pos;
    let mut frame = |events| {
        menu_items_after_frame(&ctx, events, |ui| {
            menus(ui, &mut app, MenuProjection::All, false, 35.0);
        })
    };

    Popup::open_id(&ctx, design_menu);
    let (items, _) = frame(Vec::new());
    assert!(items.iter().any(|label| label == "Arrange"), "{items:?}");
    assert!(
        !items.iter().any(|label| label == "Align left"),
        "the submenu waits to be opened"
    );

    ctx.memory_mut(|memory| memory.request_focus(arrange));
    let _ = frame(Vec::new());
    let _ = frame(vec![title_key_event(Key::ArrowRight)]);
    let (items, focused) = frame(Vec::new());
    for label in &arrange_labels {
        assert!(
            items.iter().any(|item| item == label),
            "{label} in {items:?}"
        );
    }
    assert_eq!(focused.as_deref(), Some("Rotate clockwise"));

    let _ = frame(vec![title_key_event(Key::ArrowDown)]);
    let (_, focused) = frame(Vec::new());
    assert_eq!(focused.as_deref(), Some("Rotate counter-clockwise"));

    let _ = frame(vec![title_key_event(Key::ArrowLeft)]);
    let (items, focused) = frame(Vec::new());
    assert_eq!(focused.as_deref(), Some("Arrange"));
    assert!(
        !items.iter().any(|label| label == "Align left"),
        "Left arrow closes the submenu"
    );
    assert!(
        Popup::is_id_open(&ctx, design_menu),
        "the Design menu stays"
    );

    // Enter on a submenu row runs its command and closes the menus.
    let _ = frame(vec![title_key_event(Key::ArrowRight)]);
    let _ = frame(Vec::new());
    let _ = frame(vec![title_key_event(Key::Enter)]);
    let _ = frame(Vec::new());
    assert!(!Popup::is_id_open(&ctx, design_menu));
    assert_ne!(
        app.state.schematic.document().components[1].pos,
        before,
        "Rotate clockwise turned the selection"
    );
}

#[test]
fn design_management_menu_matches_the_upgraded_governed_group() {
    assert_menu_lists(ApplicationMenu::Design, &DESIGN_MANAGEMENT_COMMANDS);
}

#[test]
fn design_sheet_menu_reads_navigation_before_authoring() {
    assert_menu_lists(ApplicationMenu::Design, &DESIGN_SHEET_COMMANDS);
}

#[test]
fn upgraded_design_menu_commands_remain_palette_discoverable() {
    for command in DESIGN_PLACEMENT_COMMANDS
        .into_iter()
        .chain(DESIGN_EDIT_COMMANDS)
        .chain(DESIGN_SHEET_COMMANDS)
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
            "Open waveform viewer",
            "Dataset and manifest browser…",
            "Create result document…",
            "Add result comparison…",
            "Report page and datasheet editor…",
            "Link A/B cursors",
            "Calculator…",
            "Expression and unit diagnostics…",
            "Measurement library…",
            "Family slicing and pivot…",
            "Plot document properties…",
            "Import result dataset…",
            "Export dataset…",
        ]
    );
}

/// The plan manager was reachable from the toolbar chip and the palette but
/// from no menu at all. It leads the Simulate menu now: it decides which plan
/// is active, and every row under it - run it, stop it, read its job history,
/// check it - acts on whatever it selected.
///
/// Adding to that plan reads second, directly under choosing it. Before
/// `Command::AddAnalysis` existed, the catalogue had exactly one route - the
/// navigator's creating action - and a reader with the panel collapsed, or in
/// focus mode, had no way to add an analysis at all.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn simulate_menu_puts_the_plan_manager_above_the_actions_on_the_active_plan() {
    let (_, rows) = rendered_simulate_menu_rows(true);
    let labels = rows
        .iter()
        .map(|(label, _)| label.as_str())
        .collect::<Vec<_>>();

    assert_eq!(
        labels,
        [
            "Manage simulation plans\u{2026}",
            "Add analysis or workflow\u{2026}",
            "Run active plan",
            "Stop active run",
            "Jobs, targets and run history\u{2026}",
            "Preflight checks",
            "Simulation Studio",
            "Stimulus Library",
            "Global solver & convergence",
            "Open generated netlist",
            "Edit specification matrix",
        ]
    );
    // The rows are the commands', not look-alikes: their labels are the ones
    // the vocabulary publishes, so renaming a command renames its row.
    assert_eq!(
        labels.first().copied(),
        Some(Command::ManageSimulationPlans.spec().label)
    );
    assert_eq!(Command::ManageSimulationPlans.spec().group, "Simulate");
    assert_eq!(
        labels.get(1).copied(),
        Some(Command::AddAnalysis.spec().label)
    );
    assert_eq!(Command::AddAnalysis.spec().group, "Simulate");
}

/// The menu row has to be exactly as honest as the command behind it: a project
/// whose plan never migrated to stable analysis identity gets an inert row that
/// names the reason, not a row that opens an empty dialog.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn simulate_menu_plan_manager_row_reports_the_commands_truthful_unavailable_reason() {
    let (availability, available) = rendered_simulate_menu_rows(true);
    assert_eq!(availability, CommandAvailability::Available);
    assert_eq!(
        simulate_menu_row_description(&available, Command::ManageSimulationPlans),
        Some(None),
        "an available plan manager row must not describe itself as unavailable: {available:?}"
    );

    let (availability, disabled) = rendered_simulate_menu_rows(false);
    let CommandAvailability::Disabled(reason) = availability else {
        panic!("the plan manager stayed available with no stable analysis plan");
    };
    assert_eq!(
        simulate_menu_row_description(&disabled, Command::ManageSimulationPlans),
        Some(Some(reason.to_owned())),
        "the disabled row must name the command's own reason: {disabled:?}"
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
        (Command::ModelCompareRelease, WorkbenchIcon::Compare),
    ] {
        assert_eq!(command_icon(command), icon, "{}", command.spec().id);
    }
}

/// Every row `entries` show for `app`, hidden ones included, as
/// `label | command id | key | state`, then `| checked` or `| unchecked` for
/// a row that switches a state; separators read `---` and a submenu row
/// `label >`. Keys are the Windows desktop ones.
#[cfg(not(target_arch = "wasm32"))]
fn menu_pin(entries: &[MenuEntry], app: &RSpiceApp) -> Vec<String> {
    menu_entries::resolve_menu_rows(
        entries,
        app,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    )
    .iter()
    .map(|row| match row {
        menu_entries::MenuRow::Command(row) => {
            let state = match row.availability {
                CommandAvailability::Available => "on".to_owned(),
                CommandAvailability::Disabled(reason) => format!("off: {reason}"),
                CommandAvailability::Hidden => "hidden".to_owned(),
            };
            let mark = match row.checked {
                Some(true) => " | checked",
                Some(false) => " | unchecked",
                None => "",
            };
            format!(
                "{} | {} | {} | {state}{mark}",
                row.label,
                row.command.stable_id(),
                row.shortcut
            )
        }
        menu_entries::MenuRow::Separator => "---".to_owned(),
        menu_entries::MenuRow::Submenu { label, .. } => format!("{label} >"),
    })
    .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn source_editor_app() -> RSpiceApp {
    let mut app = title_test_app();
    app.state.ui.active_source_editor = Some(
        crate::workbench::documents::text_editor_commands::ActiveSourceEditor {
            id: Id::new("menu-pin-source"),
            editable: true,
            navigable: true,
        },
    );
    app
}

/// Each menu's rows exactly, for an open project with nothing selected and
/// for a source editor in front, so a change to what a menu shows is a
/// change to this list.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_menu_shows_exactly_its_pinned_rows() {
    let mut mismatches = Vec::new();
    let mut check = |name: &str, actual: Vec<String>, expected: &[&str]| {
        if actual != expected {
            mismatches.push(format!(
                "{name}:\n{}",
                actual
                    .iter()
                    .map(|line| format!("        {line:?},"))
                    .collect::<Vec<_>>()
                    .join("\n")
            ));
        }
    };
    for (menu, expected) in PINNED_MENUS {
        check(
            menu.label(),
            menu_pin(menu.entries(), &title_test_app()),
            expected,
        );
    }
    for (menu, expected) in PINNED_SOURCE_EDITOR_MENUS {
        check(
            &format!("{} with a source editor", menu.label()),
            menu_pin(menu.entries(), &source_editor_app()),
            expected,
        );
    }
    for (projection, expected) in PINNED_OVERFLOW_MENUS {
        check(
            &format!("{projection:?} overflow"),
            menu_pin(overflow_entries(projection), &title_test_app()),
            expected,
        );
    }
    assert!(mismatches.is_empty(), "{}", mismatches.join("\n\n"));
}

/// Run is grey only while a run is in progress: an unrunnable plan stays
/// enabled on purpose, so the run itself can say why it cannot start. The
/// grey row said the plan was not runnable, which was never the reason.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_grey_run_row_says_a_run_is_already_in_progress() {
    let mut app = title_test_app();
    app.state.simulation.execution.trigger_simulation = true;
    assert_eq!(
        Command::RunSimulation.availability(&app),
        CommandAvailability::Disabled("a run is already in progress")
    );
    let rows = menu_pin(ApplicationMenu::Simulate.entries(), &app);
    assert!(
        rows.iter().any(
            |row| row == "Run active plan | start-run | F5 | off: a run is already in progress"
        ),
        "{rows:?}"
    );
}

/// A build with no cloud endpoints can never host a live session or publish,
/// so the File menu leaves both rows out instead of showing them grey for
/// good, and the rule that framed them goes with them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_build_without_cloud_endpoints_leaves_out_live_session_and_publish() {
    let app = title_test_app();
    assert_eq!(
        app.cloud_account.availability(),
        crate::services::cloud_account::CloudAccountAvailability::UnconfiguredBuild,
        "the test build pins no cloud endpoints"
    );
    for command in [Command::LiveSession, Command::PublishToWeb] {
        assert_eq!(command.availability(&app), CommandAvailability::Hidden);
    }
    let rows = menu_entries::resolve_menu_rows(
        ApplicationMenu::File.entries(),
        &app,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    let shown = visible_row_names(&rows);
    let export = shown
        .iter()
        .position(|name| name == "Export SPICE netlist…")
        .expect("File lists the SPICE export");
    assert_eq!(
        shown[export + 1..export + 3],
        ["---", "Close active document"],
        "{shown:?}"
    );
}

/// The rows that switch a state carry its mark, and the mark reads the
/// state the row flips: choosing a row always turns its mark over. The
/// console reads off in focus mode, which hides it, and choosing it there
/// leaves focus mode and shows it.
#[test]
fn switching_rows_are_checked_while_their_state_is_on() {
    let mut app = title_test_app();
    let switches = [
        Command::ToggleNavigator,
        Command::ToggleInspector,
        Command::ToggleConsole,
        Command::ToggleResultsSplit,
        Command::ToggleFocusMode,
        Command::ToggleFullScreen,
        Command::ToggleLinkedCursors,
    ];
    for command in switches {
        assert!(command.checked(&app).is_some(), "{command:?} reads no state");
    }
    assert_eq!(Command::Save.checked(&app), None);
    assert_eq!(Command::ResetLayout.checked(&app), None);

    for command in [
        Command::ToggleNavigator,
        Command::ToggleInspector,
        Command::ToggleConsole,
        Command::ToggleFocusMode,
        Command::ToggleConsole,
    ] {
        let before = command.checked(&app).expect("a switch");
        command.execute(&mut app);
        assert_eq!(
            command.checked(&app),
            Some(!before),
            "choosing {command:?} did not turn its mark over"
        );
    }

    app.state.workbench.focus_mode = false;
    app.state.workbench.console_visible = true;
    Command::ToggleFocusMode.execute(&mut app);
    assert_eq!(Command::ToggleConsole.checked(&app), Some(false));
    Command::ToggleConsole.execute(&mut app);
    assert_eq!(Command::ToggleConsole.checked(&app), Some(true));
    assert_eq!(Command::ToggleFocusMode.checked(&app), Some(false));

    for on in [true, false] {
        app.state.workbench.split_with_results = on;
        app.state.workbench.full_screen_presentation = on;
        app.state.ui.results.session.linked_cursors = on;
        for command in [
            Command::ToggleResultsSplit,
            Command::ToggleFullScreen,
            Command::ToggleLinkedCursors,
        ] {
            assert_eq!(command.checked(&app), Some(on), "{command:?}");
        }
    }
}

/// A switching row reads to a screen reader as a checkable menu item with
/// its state, and the check is painted where the icon would be; any other
/// row stays a plain menu item.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn switching_rows_read_as_checkable_menu_items_with_their_state() {
    use egui::accesskit::{Role, Toggled};

    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.workbench.navigator_visible = true;
    app.state.workbench.inspector_visible = false;
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
                    draw_menu_entries(ui, &app, ApplicationMenu::Window.entries())
                });
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit Window-menu tree")
        .nodes;
    let read = |label: &str| {
        nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .map(|(_, node)| (node.role(), node.toggled()))
            .unwrap_or_else(|| panic!("the Window menu has no {label:?} row"))
    };
    assert_eq!(
        read("Navigator"),
        (Role::MenuItemCheckBox, Some(Toggled::True))
    );
    assert_eq!(
        read("Inspector"),
        (Role::MenuItemCheckBox, Some(Toggled::False))
    );
    assert_eq!(
        read("Split with results"),
        (Role::MenuItemCheckBox, Some(Toggled::False))
    );
    assert_eq!(read("Reset workspace layout…"), (Role::MenuItem, None));
}

/// The labels of the command and submenu rows `menu` shows for `app`, in
/// order, and whether each one can be chosen.
#[cfg(not(target_arch = "wasm32"))]
fn shown_rows(menu: ApplicationMenu, app: &RSpiceApp) -> Vec<(String, bool)> {
    let rows = menu_entries::resolve_menu_rows(
        menu.entries(),
        app,
        crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    menu_entries::visible_rows(&rows)
        .into_iter()
        .filter_map(|row| match row {
            menu_entries::MenuRow::Command(row) => {
                Some((row.label.to_owned(), row.availability.is_available()))
            }
            menu_entries::MenuRow::Submenu { label, .. } => Some(((*label).to_owned(), true)),
            menu_entries::MenuRow::Separator => None,
        })
        .collect()
}

/// A greyed row takes the keyboard focus like any other, so Down walks
/// every row a reader can see and a screen reader hears why a row is grey.
/// Enter on a greyed row does nothing: the menu stays open on it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_keyboard_walks_greyed_rows_and_enter_on_one_does_nothing() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    let results_menu = Id::new((
        "workbench.title_menu.trigger",
        MenuTriggerKey::Application(ApplicationMenu::Results),
    ))
    .with("popup");
    let shown = shown_rows(ApplicationMenu::Results, &app);
    let grey_index = shown
        .iter()
        .position(|(_, enabled)| !enabled)
        .expect("the Results menu has a grey row without results");
    let grey = shown[grey_index].0.clone();
    let frame = |app: &mut RSpiceApp, events| {
        menu_items_after_frame(&ctx, events, |ui| {
            menus(ui, app, MenuProjection::All, false, 35.0);
        })
    };

    Popup::open_id(&ctx, results_menu);
    let _ = frame(&mut app, Vec::new());
    let mut walked = Vec::new();
    for _ in &shown {
        let _ = frame(&mut app, vec![title_key_event(Key::ArrowDown)]);
        let (_, focused) = frame(&mut app, Vec::new());
        walked.push(focused.expect("a row has the focus"));
    }
    assert_eq!(
        walked,
        shown.iter().map(|(label, _)| label.clone()).collect::<Vec<_>>()
    );

    let _ = frame(&mut app, vec![title_key_event(Key::Home)]);
    for _ in 0..grey_index {
        let _ = frame(&mut app, Vec::new());
        let _ = frame(&mut app, vec![title_key_event(Key::ArrowDown)]);
    }
    let (_, focused) = frame(&mut app, Vec::new());
    assert_eq!(focused.as_deref(), Some(grey.as_str()));
    let _ = frame(&mut app, vec![title_key_event(Key::Enter)]);
    let (_, focused) = frame(&mut app, Vec::new());
    assert!(
        Popup::is_id_open(&ctx, results_menu),
        "Enter on {grey:?} closed the menu"
    );
    assert_eq!(focused.as_deref(), Some(grey.as_str()));
}

/// A menu whose rows are all grey still answers Down, End, Home,
/// type-to-find and Escape. It answered none of them: the keys stepped only
/// among enabled rows and gave up when there were none.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_menu_of_greyed_rows_still_answers_its_keys() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.project_lifecycle.authority.close_project();
    let (menu, shown) = ALL_MENUS
        .into_iter()
        .map(|menu| (menu, shown_rows(menu, &app)))
        .find(|(_, rows)| rows.len() > 2 && rows.iter().all(|(_, enabled)| !enabled))
        .expect("some menu is all grey with no project open");
    let labels = shown
        .iter()
        .map(|(label, _)| label.as_str())
        .collect::<Vec<_>>();
    let trigger = Id::new((
        "workbench.title_menu.trigger",
        MenuTriggerKey::Application(menu),
    ));
    let popup = trigger.with("popup");
    let frame = |app: &mut RSpiceApp, events| {
        menu_items_after_frame(&ctx, events, |ui| {
            menus(ui, app, MenuProjection::All, false, 35.0);
        })
    };
    // One frame carries the event, the next shows where the focus went.
    let press = |app: &mut RSpiceApp, events: Vec<egui::Event>| {
        let _ = frame(app, events);
        frame(app, Vec::new()).1
    };
    let key = |key| vec![title_key_event(key)];

    Popup::open_id(&ctx, popup);
    let _ = press(&mut app, Vec::new());
    assert_eq!(
        press(&mut app, key(Key::ArrowDown)).as_deref(),
        Some(labels[0])
    );
    assert_eq!(
        press(&mut app, key(Key::ArrowDown)).as_deref(),
        Some(labels[1])
    );
    assert_eq!(
        press(&mut app, key(Key::End)).as_deref(),
        labels.last().copied()
    );
    assert_eq!(
        press(&mut app, key(Key::Home)).as_deref(),
        Some(labels[0])
    );
    let letter = labels[1]
        .chars()
        .next()
        .expect("a label")
        .to_lowercase()
        .to_string();
    let found = labels
        .iter()
        .cycle()
        .skip(1)
        .take(labels.len())
        .find(|label| label.to_lowercase().starts_with(&letter))
        .copied();
    assert_eq!(
        press(&mut app, vec![egui::Event::Text(letter.clone())]).as_deref(),
        found,
        "typing {letter:?} in {labels:?}"
    );
    assert_eq!(
        press(&mut app, key(Key::Escape)).as_deref(),
        Some(menu.label())
    );
    assert!(!Popup::is_id_open(&ctx, popup), "Escape closes {menu:?}");
}

/// Every menu and submenu is as wide as its longest row needs, so no label
/// is cut short, within the 244 to 360 pt a menu may be. At the old fixed
/// 244 pt some rows were cut. Measured on the rows each open menu draws, at
/// desktop and touch row heights, with the popup as tall as its rows up to
/// the window's bound: it used to stop at 400 pt and scroll, whose bar cut
/// the rows narrower still.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_menu_is_wide_enough_for_its_longest_row() {
    fn fits(ui: &Ui, name: &str, rows: &[menu_entries::MenuRow], out: &mut Vec<(String, f32)>) {
        out.push((name.to_owned(), menu_entries::menu_rows_fit(ui, rows)));
        for row in rows {
            if let menu_entries::MenuRow::Submenu { label, rows, .. } = row {
                fits(ui, label, rows, out);
            }
        }
    }
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut needed = Vec::new();
    for app in [title_test_app(), source_editor_app()] {
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for menu in ALL_MENUS {
                    let rows = menu_rows(menu.entries(), &app, ui.ctx());
                    fits(ui, menu.label(), &rows, &mut needed);
                }
            });
        });
    }
    for (name, fit) in &needed {
        assert!(
            *fit <= MENU_MAX_CONTENT_WIDTH,
            "{name} needs {fit} pt, more than a menu may be"
        );
    }
    assert!(
        needed.iter().any(|(_, fit)| *fit > MENU_CONTENT_WIDTH),
        "no menu needs more than the narrowest width: {needed:?}"
    );

    for large_targets in [false, true] {
        let row_height = if large_targets {
            MENU_TOUCH_ROW_HEIGHT
        } else {
            MENU_ROW_HEIGHT
        };
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = title_test_app();
        for menu in ALL_MENUS {
            let popup = Id::new((
                "workbench.title_menu.trigger",
                MenuTriggerKey::Application(menu),
            ))
            .with("popup");
            Popup::open_id(&ctx, popup);
            let mut fit = 0.0;
            let mut rows_height = 0.0;
            let mut drawn = Vec::new();
            for _ in 0..3 {
                let output = ctx.run_ui(
                    egui::RawInput {
                        screen_rect: Some(egui::Rect::from_min_size(
                            egui::Pos2::ZERO,
                            egui::vec2(1_440.0, 900.0),
                        )),
                        ..Default::default()
                    },
                    |ctx| {
                        egui::CentralPanel::default().show(ctx, |ui| {
                            let rows = menu_rows(menu.entries(), &app, ui.ctx());
                            fit = menu_entries::menu_rows_fit(ui, &rows);
                            rows_height = menu_entries::visible_rows(&rows)
                                .into_iter()
                                .map(|row| match row {
                                    menu_entries::MenuRow::Separator => 9.0,
                                    _ => row_height,
                                })
                                .sum::<f32>();
                            menus(ui, &mut app, MenuProjection::All, large_targets, 35.0);
                        });
                    },
                );
                drawn = output
                    .platform_output
                    .accesskit_update
                    .expect("AccessKit menu tree")
                    .nodes
                    .into_iter()
                    .filter(|(_, node)| {
                        matches!(
                            node.role(),
                            egui::accesskit::Role::MenuItem
                                | egui::accesskit::Role::MenuItemCheckBox
                        )
                    })
                    .filter_map(|(_, node)| {
                        let bounds = node.bounds()?;
                        (bounds.y0 > 35.0).then(|| {
                            (
                                node.label().unwrap_or_default().to_owned(),
                                bounds.width() as f32,
                                bounds.height() as f32,
                            )
                        })
                    })
                    .collect::<Vec<_>>();
            }
            assert!(!drawn.is_empty(), "{} drew no rows", menu.label());
            // The popup is as tall as its rows up to the window's bound,
            // so a menu that fits never scrolls.
            let outer_height = ctx
                .memory(|memory| memory.area_rect(popup))
                .expect("the menu is open")
                .height();
            let frame = 2.0 * (MENU_MARGIN + MENU_BORDER_WIDTH);
            let expected = rows_height.min(menu_popup_height_for_viewport(900.0)) + frame;
            assert!(
                (outer_height - expected).abs() < 1.0,
                "{} is {outer_height} pt tall for {rows_height} pt of rows \
                 (touch rows: {large_targets})",
                menu.label()
            );
            for (label, width, height) in &drawn {
                assert!(
                    *width >= fit.min(MENU_MAX_CONTENT_WIDTH) - 0.5
                        && *width <= MENU_MAX_CONTENT_WIDTH + 0.5,
                    "{} row {label:?} is {width} pt wide; its longest row needs {fit} \
                     (touch rows: {large_targets})",
                    menu.label()
                );
                assert!(
                    (*height - row_height).abs() < 0.01,
                    "{} row {label:?} is {height} pt tall",
                    menu.label()
                );
            }
        }
    }
}

/// What a reader sees of `rows`: each row's label, `---` for a rule.
#[cfg(not(target_arch = "wasm32"))]
fn visible_row_names(rows: &[menu_entries::MenuRow]) -> Vec<String> {
    menu_entries::visible_rows(rows)
        .into_iter()
        .map(|row| match row {
            menu_entries::MenuRow::Command(row) => row.label.to_owned(),
            menu_entries::MenuRow::Separator => "---".to_owned(),
            menu_entries::MenuRow::Submenu { label, .. } => (*label).to_owned(),
        })
        .collect()
}

/// A rule only ever stands between two groups a reader can see: none opens
/// or closes a menu, and none sits beside another where a whole group is
/// hidden. The Window menu's last group (previous and next workspace) is
/// hidden, and it used to leave a rule hanging at the menu's foot.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn no_menu_draws_a_rule_at_its_edge_or_two_rules_together() {
    for app in [title_test_app(), source_editor_app()] {
        for menu in ALL_MENUS {
            let rows = menu_entries::resolve_menu_rows(
                menu.entries(),
                &app,
                crate::workbench::commands::vocabulary::CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            );
            let shown = visible_row_names(&rows);
            assert_ne!(shown.first().map(String::as_str), Some("---"), "{shown:?}");
            assert_ne!(shown.last().map(String::as_str), Some("---"), "{shown:?}");
            assert!(
                !shown
                    .windows(2)
                    .any(|pair| pair[0] == "---" && pair[1] == "---"),
                "{} draws two rules together: {shown:?}",
                menu.label()
            );
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
const PINNED_MENUS: [(ApplicationMenu, &[&str]); 11] = [
    (
        ApplicationMenu::File,
        &[
            "Project launcher… | project-launcher | Ctrl+Shift+O | on",
            "---",
            "Open project… | open-project | Ctrl+O | on",
            "Open netlist… | open-netlist | Ctrl+Shift+L | on",
            "New project… | new-project | Ctrl+Shift+N | on",
            "Recent projects… | recent-projects |  | on",
            "---",
            "New cell… | new-cell |  | on",
            "Open schematic… | open-schematic |  | on",
            "---",
            "Save | save-project | Ctrl+S | on",
            "Save as… | save-project-as | Ctrl+Alt+Shift+S | on",
            "Save all | save-all | Ctrl+Shift+S | on",
            "Revert active document… | revert-document |  | off: active document has no changes to revert",
            "---",
            "Import SPICE deck… | import-netlist |  | on",
            "Import Verilog-A… | import-veriloga |  | on",
            "---",
            "Export schematic SVG… | export-schematic-svg |  | on",
            "Export dataset… | export-results |  | off: no result dataset is available",
            "Export publication snapshot… | export-publication-snapshot |  | on",
            "Export SPICE netlist… | export-netlist-spice |  | on",
            "---",
            "Live session… | live-session |  | hidden",
            "Publish to web… | publish-web |  | hidden",
            "---",
            "Close active document | close-document | Ctrl+W | off: no closable document is active",
            "Close project… | close-project | Ctrl+Shift+W | on",
            "---",
            "Sheet properties | page-setup | Ctrl+Alt+Shift+P | on",
            "Sheet formats across this document… | sheet-format-manager |  | off: give the open sheet a property of its own in Sheet properties first",
            "Custom sheet sizes… | sheet-preset-library |  | on",
            "Sheet defaults… | sheet-defaults |  | on",
            "Print / hardcopy… | print-hardcopy | Ctrl+P | off: command is unavailable in this context",
            "Export active view… | export-active-view | Ctrl+Shift+P | off: command is unavailable in this context",
            "---",
            "Exit RSpice… | exit-rspice | Alt+F4 | on",
        ],
    ),
    (
        ApplicationMenu::Edit,
        &[
            "Undo | undo | Ctrl+Z | off: nothing to undo",
            "Redo | redo | Ctrl+Shift+Z | off: nothing to redo",
            "---",
            "Cut selection | cut-selection | Ctrl+X | off: select an editable object",
            "Copy selection | copy-selection | Ctrl+C | off: select an editable object",
            "Paste | paste-selection | Ctrl+V | off: clipboard has no compatible content",
            "Duplicate selection | duplicate-selection | Ctrl+D | off: select an editable object",
            "Delete selection | delete-selection | Delete | off: select an editable object",
            "Select all | select-all | Ctrl+A | on",
            "---",
            "Rename selected object… | rename-selection | F2 | off: select one editable component, net label, or declared bus",
            "Object properties… | object-properties | Q | off: select one inspectable object",
            "Find in design… | find-design | Ctrl+F | on",
            "Find and replace… | find-replace | Ctrl+H | on",
            "---",
            "Preferences… | preferences | Ctrl+, | on",
        ],
    ),
    (
        ApplicationMenu::View,
        &[
            "Zoom in | zoom-in | + | on",
            "Zoom out | zoom-out | − | on",
            "Fit drawing sheet | canvas-fit-sheet | F | on",
            "Fit schematic content | canvas-fit-content | Shift+F | on",
            "Zoom 100% | zoom-one-to-one |  | on",
            "Zoom to selection | zoom-to-selection |  | off: command is unavailable in this context",
            "Zoom box | zoom-box | Z | on",
            "Cycle grid display | cycle-grid | G | on",
            "Grid, snap and wire routing… | grid-snap-routing |  | on",
            "Drawing-sheet layers… | sheet-layer-visibility |  | on",
            "Schematic visibility… | visibility-options |  | on",
            "---",
            "Full screen | full-screen | F11 | on | unchecked",
            "Reset active view | reset-active-view |  | on",
            "---",
            "Engineering table view… | engineering-table-manager | Ctrl+Shift+T | on",
        ],
    ),
    (
        ApplicationMenu::Design,
        &[
            "Open active schematic | design | Alt+2 | on",
            "Ascend hierarchy | ascend-hierarchy | Shift+H | off: already at top hierarchy",
            "Descend into selected instance… | descend-hierarchy | H | off: select one hierarchical instance",
            "---",
            "Place instance… | place-instance | Shift+I | on",
            "Draw wire | place-wire | W | on",
            "Draw bus | place-bus | B | on",
            "Place bus tap | place-bus-tap | T | on",
            "Place junction | place-junction | J | on",
            "Place net label | place-label | N | on",
            "Place probe | place-probe | P | on",
            "Create pins… | place-pin | Shift+P | on",
            "Place text or note… | place-text | Shift+T | on",
            "Draw documentation shape… | place-shape |  | on",
            "---",
            "Move selection | move-selection | M | off: select an editable object",
            "Stretch selection | stretch-selection | S | off: select an editable object",
            "Create array… | array-selection |  | off: select an editable object",
            "Replace instance… | replace-instance |  | off: select an editable object",
            "Arrange >",
            "Join collinear wires | join-collinear-wires |  | on",
            "---",
            "Previous sheet | previous-sheet | PageUp | off: the active cell view has a single sheet",
            "Next sheet | next-sheet | PageDown | off: the active cell view has a single sheet",
            "New sheet | new-sheet |  | on",
            "Delete active sheet | delete-sheet |  | off: the active cell view has a single sheet",
            "---",
            "Create hierarchy from selection… | create-hierarchy |  | off: select an editable object",
            "Sheets, variants and annotation… | design-management |  | on",
            "Selection and bulk editing… | design-bulk-tools |  | on",
            "Connectivity and bus manager… | design-connectivity-tools |  | on",
            "Configuration sets… | configuration-sets |  | on",
            "Design rule setup… | design-rule-setup |  | on",
            "Review comments… | review-comments |  | on",
            "Revision history… | revision-history |  | on",
            "---",
            "Specialist workspaces… | specialist-tools-design |  | on",
            "---",
            "Check cellview | check-cellview | Ctrl+E | on",
            "Check design | check-design |  | on",
            "Check and save | check-and-save | Ctrl+Shift+E | on",
            "---",
            "Browse all design commands… | command-palette | Ctrl+K | on",
        ],
    ),
    (
        ApplicationMenu::Simulate,
        &[
            "Manage simulation plans… | manage-simulation-plans |  | on",
            "Add analysis or workflow… | add-analysis |  | on",
            "Run active plan | start-run | F5 | on",
            "Stop active run | stop-run | Shift+F5 | off: no simulation is running",
            "Jobs, targets and run history… | jobs-manager |  | on",
            "Preflight checks | check | Ctrl+E | on",
            "---",
            "Simulation Studio | simulate | Alt+3 | on",
            "Stimulus Library | stimulus | Alt+8 | on",
            "Global solver & convergence | solver |  | on",
            "Open generated netlist | generated-netlist | Ctrl+L | on",
            "Edit specification matrix | specifications |  | on",
        ],
    ),
    (
        ApplicationMenu::Results,
        &[
            "Open waveform viewer | waveforms |  | on",
            "Dataset and manifest browser… | dataset-browser |  | off: no retained result dataset is available",
            "Create result document… | create-result-document |  | off: no retained result dataset is available",
            "Add result comparison… | compare-datasets |  | off: open an active result document",
            "Report page and datasheet editor… | report-page-editor |  | on",
            "---",
            "Link A/B cursors | toggle-linked-cursors | Shift+C | off: open a result in Results | unchecked",
            "Calculator… | calculator |  | on",
            "Expression and unit diagnostics… | expression-diagnostics |  | off: no result dataset is available",
            "Measurement library… | measurement-library |  | off: open a result in Results",
            "Family slicing and pivot… | family-slicing |  | off: open a result in Results",
            "Plot document properties… | plot-properties |  | off: open a result in Results",
            "---",
            "Import result dataset… | import-dataset |  | on",
            "Export dataset… | export-results |  | off: no result dataset is available",
        ],
    ),
    (
        ApplicationMenu::Verify,
        &[
            "Verification cockpit | yield |  | on",
            "Corner matrix | corners |  | on",
            "Optimization | optimization |  | on",
            "Safe operating area | device-safety |  | on",
            "Regression plan | regression |  | on",
        ],
    ),
    (
        ApplicationMenu::Models,
        &[
            "Model & library catalog | models-catalog |  | on",
            "Device model and parameter editor… | model-editor |  | off: select one model in Model & library catalog",
            "Measurement correlation… | model-correlation |  | off: select one model in Model & library catalog",
            "---",
            "Verilog-A/AMS compiler | veriloga |  | off: open a Verilog-A cell view in Design",
        ],
    ),
    (
        ApplicationMenu::Automation,
        &["Automation workspace | automation | Ctrl+` | on"],
    ),
    (
        ApplicationMenu::Window,
        &[
            "Navigator | toggle-navigator | Ctrl+B | on | checked",
            "Inspector | toggle-inspector | Ctrl+I | on | checked",
            "Console | console | Ctrl+J | on | unchecked",
            "Split with results | toggle-split-view | Ctrl+\\ | off: no retained result dataset is available | unchecked",
            "Focus mode | toggle-focus-mode | Ctrl+Shift+F | on | unchecked",
            "---",
            "New application window… | new-application-window | Ctrl+Alt+Shift+N | on",
            "Detach active document… | detach-document |  | on",
            "Move active document to window… | move-document-window |  | off: create another application window before moving this document",
            "Reattach active document | reattach-document |  | off: the active document is already in the main window",
            "Consolidate all windows… | consolidate-windows |  | off: there are no detached application windows",
            "Recover off-screen windows… | monitor-recovery |  | off: there are no secondary application windows to recover",
            "---",
            "Previous document | previous-document | Ctrl+Shift+Tab | off: command is unavailable in this context",
            "Next document | next-document | Ctrl+Tab | off: command is unavailable in this context",
            "Close other documents | close-other-documents |  | off: command is unavailable in this context",
            "Close all documents | close-all-documents |  | off: command is unavailable in this context",
            "---",
            "Workspace layouts… | workspace-layouts |  | on",
            "Windows, documents and session… | window-manager |  | on",
            "Reset workspace layout… | reset-workspace-layout |  | on",
            "---",
            "Previous workspace | previous-workspace |  | hidden",
            "Next workspace | next-workspace |  | hidden",
        ],
    ),
    (
        ApplicationMenu::Help,
        &[
            "RSpice Help | help-center | F1 | on",
            "Command reference | command-reference |  | on",
            "Release notes and compatibility changes… | release-notes |  | on",
            "Project migration guide… | migration-guide |  | on",
            "System diagnostics and log locations… | system-diagnostics |  | on",
            "Create support bundle… | support-bundle |  | on",
            "Legal, privacy and notices… | legal-privacy-center |  | on",
            "---",
            "About RSpice | about |  | on",
        ],
    ),
];

#[cfg(not(target_arch = "wasm32"))]
const PINNED_SOURCE_EDITOR_MENUS: [(ApplicationMenu, &[&str]); 2] = [
    (
        ApplicationMenu::Edit,
        &[
            "Undo | undo | Ctrl+Z | on",
            "Redo | redo | Ctrl+Shift+Z | on",
            "---",
            "Cut selection | cut-selection | Ctrl+X | on",
            "Copy selection | copy-selection | Ctrl+C | on",
            "Paste | paste-selection | Ctrl+V | on",
            "Duplicate line | duplicate-line | Ctrl+D | on",
            "Delete selection | delete-selection | Delete | on",
            "Select all | select-all | Ctrl+A | on",
            "---",
            "Toggle line comment | toggle-line-comment | Ctrl+/ | on",
            "Format document | format-document |  | on",
            "Go to line… | go-to-line | Ctrl+G | on",
            "Find in source… | find-code | Ctrl+F | off: command is unavailable in this context",
            "---",
            "Preferences… | preferences | Ctrl+, | on",
        ],
    ),
    (
        ApplicationMenu::View,
        &[
            "Zoom in | zoom-in | + | on",
            "Zoom out | zoom-out | − | on",
            "Fit drawing sheet | canvas-fit-sheet | F | on",
            "Fit schematic content | canvas-fit-content | Shift+F | on",
            "Zoom 100% | zoom-one-to-one |  | on",
            "Zoom to selection | zoom-to-selection |  | off: command is unavailable in this context",
            "Zoom box | zoom-box | Z | on",
            "Cycle grid display | cycle-grid | G | on",
            "Grid, snap and wire routing… | grid-snap-routing |  | on",
            "Drawing-sheet layers… | sheet-layer-visibility |  | on",
            "Schematic visibility… | visibility-options |  | on",
            "---",
            "Fold selection | fold-selection | Ctrl+Shift+[ | on",
            "Unfold at caret | unfold-at-caret | Ctrl+Shift+] | on",
            "Fold all | fold-all |  | on",
            "Unfold all | unfold-all |  | on",
            "---",
            "Full screen | full-screen | F11 | on | unchecked",
            "Reset active view | reset-active-view |  | on",
            "---",
            "Engineering table view… | engineering-table-manager | Ctrl+Shift+T | on",
        ],
    ),
];

#[cfg(not(target_arch = "wasm32"))]
const PINNED_OVERFLOW_MENUS: [(MenuProjection, &[&str]); 2] = [
    (
        MenuProjection::ThroughSimulate,
        &[
            "Results workspace | waveforms |  | on",
            "Verification workspace | yield |  | on",
            "Models workspace | models-catalog |  | on",
            "Automation workspace | automation |  | on",
            "Specialist workspaces… | specialist-tool-browser |  | on",
            "---",
            "Workspace layouts… | workspace-layouts |  | on",
            "Windows, documents and session… | window-manager |  | on",
            "Jobs & targets | jobs-manager |  | on",
            "Focus mode | toggle-focus-mode | Ctrl+Shift+F | on | unchecked",
            "Console | console | Ctrl+J | on | unchecked",
            "---",
            "RSpice Help | help-center | F1 | on",
        ],
    ),
    (
        MenuProjection::ThroughModels,
        &[
            "Automation workspace | automation |  | on",
            "Specialist workspaces… | specialist-tool-browser |  | on",
            "---",
            "Workspace layouts… | workspace-layouts |  | on",
            "Windows, documents and session… | window-manager |  | on",
            "Jobs & targets | jobs-manager |  | on",
            "Focus mode | toggle-focus-mode | Ctrl+Shift+F | on | unchecked",
            "Console | console | Ctrl+J | on | unchecked",
            "---",
            "RSpice Help | help-center | F1 | on",
        ],
    ),
];

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
