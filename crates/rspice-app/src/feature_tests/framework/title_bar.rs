//! Tests for the menu bar's contents.
//!
//! The suite asserts that every menu entry corresponds to a real, completed
//! workflow and carries its canonical icon - the title bar must not advertise
//! capability the workbench does not have.

use egui::{Context, Frame, Id, Key, Modifiers, Popup, Rect, Ui};

use crate::state::ModelsPage;
use crate::state::VerificationPage;
use crate::state::command_catalog::CommandKey;
use crate::ui::panels::WorkbenchIcon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens;
use crate::workbench::RSpiceApp;
use crate::workbench::chrome::title_bar::application_menus::{
    ALL_MENUS, ApplicationMenu, file_menu_shows_exit, menu_popup_height_for_viewport,
    shortcut_for_occurrence,
};
use crate::workbench::chrome::title_bar::menu_entries::{MenuEntry, menu_rows};
use crate::workbench::chrome::title_bar::*;
use crate::workbench::cloud::commands::CloudCommand;
use crate::workbench::commands::vocabulary::{Command, command_registry};
use crate::workbench::framework::command_registry::{CommandAvailability, CommandFlags, ids};
use crate::workbench::framework::host::Host;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::models::commands::ModelsCommand;
use crate::workbench::project::commands::ProjectCommand;
use crate::workbench::results::commands::ResultsCommand;
use crate::workbench::sheets::commands::SheetsCommand;
use crate::workbench::simulate::commands::SimulateCommand;
use crate::workbench::state::Workspace;

mod access_keys;
#[cfg(not(target_arch = "wasm32"))]
mod capabilities;
mod macos;
mod pins;

#[cfg(not(target_arch = "wasm32"))]
use pins::{
    menu_pin, menu_pin_for, menu_pin_on, source_editor_app, symbol_view_app, veriloga_view_app,
};

#[cfg(not(target_arch = "wasm32"))]
fn title_test_app() -> RSpiceApp {
    RSpiceApp::test_instance()
}

/// A context that draws the Windows and Linux menus whatever the host, as
/// `shown_rows` and the pins read them. The macOS menus differ (the
/// RSpice menu holds About, Settings and Quit) and are pinned in `macos`.
#[cfg(not(target_arch = "wasm32"))]
fn title_test_ctx() -> Context {
    let ctx = Context::default();
    ctx.set_os(egui::os::OperatingSystem::Windows);
    ctx
}

/// What `menu` holds, as the shell hands it to the title bar.
fn application_menu_entries(menu: ApplicationMenu) -> &'static [MenuEntry] {
    RSpiceApp::test_instance().menu_entries(menu)
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn desktop_and_compact_title_contexts_have_distinct_mockup_ownership() {
    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;

    assert_eq!(
        title_context_text(&app, false),
        app.state.model.workspace.content.project.display_name()
    );
    assert_eq!(title_context_text(&app, true), "top · schematic");
    assert_eq!(active_title_cell(&app), "top · schematic");
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn closed_project_title_context_does_not_leak_the_previous_document_identity() {
    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;
    app.state.close_project_session();

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
        let ctx = title_test_ctx();
        crate::ui::Theme::default().apply(&ctx);
        let mut app = title_test_app();
        app.state.workbench.workspace = Workspace::Design;
        if !project_open {
            app.state.close_project_session();
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
        let ctx = title_test_ctx();
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

    app.state.model.schematic.session.is_dirty = true;
    assert_eq!(painted(&app, 640.0), "Untitled Project*");
    assert!(
        painted(&app, 90.0).ends_with("\u{2026}*"),
        "a narrow title shortens the name and keeps the marker: {}",
        painted(&app, 90.0)
    );

    app.state.close_project_session();
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
    let _ = menu_entries::show_menu_rows(ui, &rows);
}

/// Every command `entries` list, in order, whatever context their blocks
/// show in; a submenu's commands follow its row.
fn listed_commands(entries: &[MenuEntry]) -> Vec<CommandKey> {
    let mut commands = Vec::new();
    for entry in entries {
        match *entry {
            MenuEntry::Command(item) => {
                assert!(
                    command_registry().spec(item.command).is_some(),
                    "a menu names registered commands"
                );
                commands.push(item.command);
            }
            MenuEntry::Submenu { entries, .. } | MenuEntry::Only(_, entries) => {
                commands.extend(listed_commands(entries));
            }
            MenuEntry::Separator | MenuEntry::RecentFiles(_) => {}
        }
    }
    commands
}

/// The keys of `commands`, in order.
fn keys<const N: usize>(commands: [Command; N]) -> [CommandKey; N] {
    commands.map(Command::key)
}

/// `menu` lists `run` as consecutive rows.
fn assert_menu_lists(menu: ApplicationMenu, run: &[CommandKey]) {
    let listed = listed_commands(application_menu_entries(menu));
    assert!(
        listed.windows(run.len()).any(|window| window == run),
        "{} does not list {run:?} in order: {listed:?}",
        menu.label()
    );
}

#[cfg(not(target_arch = "wasm32"))]
fn rendered_results_menu_labels() -> Vec<String> {
    let ctx = title_test_ctx();
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
                    draw_menu_entries(ui, &app, application_menu_entries(ApplicationMenu::Results))
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
    let ctx = title_test_ctx();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.open_project_session();
    if !stable_analysis_plan {
        app.state.model.sim_setup.analysis_plan = None;
    }
    let availability = app.command_availability(SimulateCommand::ManagePlans.key());
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
                    draw_menu_entries(
                        ui,
                        &app,
                        application_menu_entries(ApplicationMenu::Simulate),
                    )
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
    command: SimulateCommand,
) -> Option<Option<String>> {
    rows.iter()
        .find(|(label, _)| label.as_str() == command.declared_spec().label)
        .map(|(_, description)| description.clone())
}

/// Above 820 pt the bar shows every menu; at and below it, and in the
/// compact shell at any width, one Menu button holds them. The in-between
/// widths used to drop menus into an overflow.
#[test]
fn the_bar_shows_every_menu_above_820_and_one_menu_button_below() {
    for width in [821.0, 1_021.0, 1_280.0, 1_440.0] {
        assert_eq!(
            MenuProjection::for_layout(width, false),
            MenuProjection::All
        );
    }
    assert_eq!(
        MenuProjection::for_layout(820.0, false),
        MenuProjection::Compact
    );
    assert_eq!(
        MenuProjection::for_layout(1_024.0, true),
        MenuProjection::Compact
    );
    assert!(title_context_is_left_aligned(844.0, true));
    assert!(!title_context_is_left_aligned(820.0, true));
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
fn the_full_bar_lists_every_menu_in_order() {
    assert!(labels(MenuProjection::Compact).is_empty());
    assert_eq!(
        labels(MenuProjection::All),
        [
            "File", "Edit", "View", "Design", "Simulate", "Results", "Verify", "Tools", "Window",
            "Help"
        ]
    );
}

#[test]
fn the_compact_bar_drops_the_title_and_the_search_field_narrows() {
    assert!(!MenuProjection::Compact.shows_title_context());
    assert!(MenuProjection::All.shows_title_context());
    assert_eq!(search_button_width(1020.0, false), 31.0);
    assert!(search_button_width(1021.0, false) > 31.0);
    assert_eq!(search_button_width(1440.0, true), 44.0);
}

/// The overflow menu the in-between widths showed is gone, and each of its
/// rows still has a home in a real menu. Its jobs row read "Jobs & targets"
/// and opens what is now the run history, so Simulate lists it under that
/// name. The overflow's only other own row, the specialist tool browser, is
/// retired.
#[test]
fn every_row_the_overflow_held_has_a_home_in_a_menu() {
    let listed = ALL_MENUS
        .into_iter()
        .flat_map(|menu| listed_commands(application_menu_entries(menu)))
        .collect::<Vec<_>>();
    assert!(
        listed.contains(&ResultsCommand::ResultViewer(crate::workbench::ResultViewer::Waves).key()),
        "no menu lists the waveform viewer"
    );
    assert!(
        listed.contains(&ModelsCommand::Page(ModelsPage::Models).key()),
        "no menu lists the model catalog"
    );
    for command in [
        Command::AutomationConsole,
        Command::WindowManager,
        Command::ToggleFocusMode,
        Command::ToggleConsole,
        Command::HelpCenter,
    ] {
        assert!(listed.contains(&command.key()), "no menu lists {command:?}");
    }
    for command in [
        SimulateCommand::Verify(VerificationPage::Yield),
        SimulateCommand::JobsManager,
    ] {
        assert!(listed.contains(&command.key()), "no menu lists {command:?}");
    }
    assert!(
        listed.contains(&ids::WORKSPACE_LAYOUTS),
        "no menu lists workspace layouts"
    );
    assert!(
        listed_commands(application_menu_entries(ApplicationMenu::Simulate))
            .contains(&SimulateCommand::JobsManager.key())
    );
    assert_eq!(
        SimulateCommand::JobsManager.declared_spec().label,
        "Run history\u{2026}"
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn desktop_menu_triggers_fill_the_title_track_without_vertical_clipping() {
    let ctx = title_test_ctx();
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
    let ctx = title_test_ctx();
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
    let app = RSpiceApp::test_instance();
    assert_eq!(
        shortcut_for_occurrence(
            &app,
            crate::state::command_catalog::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
            Command::KeyboardShortcuts.key(),
            Some("")
        ),
        ""
    );
    assert_eq!(
        shortcut_for_occurrence(
            &app,
            crate::state::command_catalog::CommandPlatform::Desktop,
            egui::os::OperatingSystem::Windows,
            Command::KeyboardShortcuts.key(),
            Some("Ctrl+Alt+R")
        ),
        "Ctrl+Alt+R"
    );
    assert_eq!(
        Command::DescendHierarchy.spec().label,
        "Descend with options…"
    );
}

#[test]
fn exit_is_a_desktop_lifecycle_action_not_a_browser_menu_item() {
    use crate::state::command_catalog::CommandPlatform;

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
fn application_menu_height_is_viewport_bounded() {
    assert_eq!(menu_popup_height_for_viewport(900.0), 836.0);
    assert_eq!(menu_popup_height_for_viewport(600.0), 536.0);
    assert_eq!(menu_popup_height_for_viewport(120.0), MENU_ROW_HEIGHT * 3.0);
}

#[test]
fn menu_shortcuts_are_projected_from_the_typed_registry() {
    let platform = crate::state::command_catalog::CommandPlatform::Desktop;
    let app = RSpiceApp::test_instance();
    for command in [
        ProjectCommand::OpenProject.declared_spec(),
        ProjectCommand::Save.declared_spec(),
        SimulateCommand::Run.declared_spec(),
    ] {
        let declared = command
            .bindings
            .iter()
            .find(|binding| binding.supports(platform))
            .map_or("", |binding| binding.chord.label);
        assert_eq!(
            shortcut_for_occurrence(
                &app,
                platform,
                egui::os::OperatingSystem::Windows,
                command.key,
                None
            ),
            declared
        );
    }
    for command in [
        Command::ToggleFullScreen,
        Command::GenerateNetlist,
        Command::ToggleConsole,
    ] {
        assert_eq!(
            shortcut_for_occurrence(
                &app,
                platform,
                egui::os::OperatingSystem::Windows,
                command.key(),
                None
            ),
            command.default_shortcut_label(platform)
        );
    }
    assert_eq!(
        shortcut_for_occurrence(
            &app,
            platform,
            egui::os::OperatingSystem::Windows,
            Command::KeyboardShortcuts.key(),
            None
        ),
        ""
    );
}

#[test]
fn core_menu_commands_are_all_real_dispatch_commands() {
    let commands = [
        Command::Undo,
        Command::PlaceWire,
        Command::PlaceBus,
        Command::PlaceBusTap,
        Command::PlaceJunction,
        Command::AutomationConsole,
    ];
    assert!(commands.iter().all(|command| !command.spec().id.is_empty()));
    assert!(
        command_registry()
            .spec(ResultsCommand::WaveformCalculator.key())
            .is_some()
    );
    assert!(
        command_registry()
            .spec(ModelsCommand::PdkSettings.key())
            .is_some()
    );
    for command in [ProjectCommand::OpenProject, ProjectCommand::Save] {
        assert!(
            command_registry().spec(command.key()).is_some(),
            "{command:?}"
        );
    }
    assert!(
        command_registry()
            .spec(SimulateCommand::Run.key())
            .is_some()
    );
}

/// No View-menu row is drawn twice.
///
/// `Visibility options…` was listed in the viewport group and then emitted a
/// second time immediately after the loop, so the menu showed the same row
/// twice in a row with the same shortcut and the same enablement.
#[test]
fn the_view_menu_lists_each_viewport_row_exactly_once() {
    let mut seen = std::collections::BTreeSet::new();
    let listed = listed_commands(application_menu_entries(ApplicationMenu::View));
    for command in &listed {
        assert!(
            seen.insert(command.as_str()),
            "{command:?} appears twice in the View menu"
        );
    }
    assert_eq!(seen.len(), listed.len());
    assert!(listed.contains(&Command::VisibilityOptions.key()));
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
const SELECTION_EDIT_COMMANDS: [Command; 5] = [
    Command::MoveSelection,
    Command::StretchSelection,
    Command::ArraySelection,
    Command::ReplaceInstance,
    Command::SelectionBulkEdit,
];
const DESIGN_MANAGEMENT_COMMANDS: [CommandKey; 6] = [
    Command::CreateHierarchy.key(),
    ProjectCommand::DesignManagement.key(),
    Command::ConnectivityManager.key(),
    Command::ConfigurationSets.key(),
    Command::RuleSetup.key(),
    Command::ReviewComments.key(),
];
const DESIGN_SHEET_COMMANDS: [SheetsCommand; 4] = [
    SheetsCommand::PreviousSheet,
    SheetsCommand::NextSheet,
    SheetsCommand::NewSheet,
    SheetsCommand::DeleteSheet,
];

/// The panels around the drawing are switched from View, beside what is
/// drawn over it; Window keeps the windows and the documents in them.
#[test]
fn the_panels_are_switched_from_view_and_not_from_window() {
    let panels = [
        Command::ToggleNavigator.key(),
        Command::ToggleInspector.key(),
        Command::ToggleConsole.key(),
        ResultsCommand::ToggleResultsSplit.key(),
        Command::OpenProblems.key(),
    ];
    let screen = [
        Command::ToggleFocusMode.key(),
        Command::ToggleFullScreen.key(),
    ];
    assert_menu_lists(ApplicationMenu::View, &panels);
    assert_menu_lists(ApplicationMenu::View, &screen);
    let window = listed_commands(application_menu_entries(ApplicationMenu::Window));
    for key in panels.into_iter().chain(screen) {
        assert!(!window.contains(&key), "Window still lists {key:?}");
    }
}

/// View > Grid draws one style at a time: its rows are radio items, the
/// drawn style's selected, and choosing one draws it; G still steps through
/// them.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_grid_rows_choose_the_drawn_style() {
    use crate::state::GridStyle;
    use egui::accesskit::{Role, Toggled};

    let grid = application_menu_entries(ApplicationMenu::View)
        .iter()
        .find_map(|entry| match *entry {
            MenuEntry::Submenu {
                label: "Grid",
                entries,
                ..
            } => Some(entries),
            _ => None,
        })
        .expect("View has a Grid submenu");
    let mut app = title_test_app();
    app.state.workbench.workspace = Workspace::Design;
    let ctx = title_test_ctx();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
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
                .show(ctx, |ui| draw_menu_entries(ui, &app, grid));
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit Grid-menu tree")
        .nodes;
    let read = |label: &str| {
        nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .map(|(_, node)| (node.role(), node.toggled()))
            .unwrap_or_else(|| panic!("the Grid menu has no {label:?} row"))
    };
    assert_eq!(read("Dots"), (Role::MenuItemRadio, Some(Toggled::True)));
    assert_eq!(read("Lines"), (Role::MenuItemRadio, Some(Toggled::False)));
    assert_eq!(read("Off"), (Role::MenuItemRadio, Some(Toggled::False)));

    app.execute(Command::SetGridStyle(GridStyle::Off).key());
    assert_eq!(app.state.design.grid, GridStyle::Off);
    assert_eq!(
        app.checked(Command::SetGridStyle(GridStyle::Off).key()),
        Some(true)
    );
    assert_eq!(
        app.checked(Command::SetGridStyle(GridStyle::Dots).key()),
        Some(false)
    );
    app.execute(Command::CycleGrid.key());
    assert_eq!(app.state.design.grid, GridStyle::Dots);
}

/// What is done to a selection is edited from Edit, where Cut, Copy and
/// Delete already are; Design keeps what builds the design.
#[test]
fn the_selection_edits_are_in_edit_and_not_in_design() {
    assert_menu_lists(ApplicationMenu::Edit, &keys(SELECTION_EDIT_COMMANDS));
    let design = listed_commands(application_menu_entries(ApplicationMenu::Design));
    for command in SELECTION_EDIT_COMMANDS {
        assert!(
            !design.contains(&command.key()),
            "Design still lists {command:?}"
        );
    }
}

/// Undo and Redo name the step they take and Repeat the command it runs
/// again, so a reader knows what the row does before choosing it; with
/// nothing to name, each reads its plain label.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn undo_redo_and_repeat_name_what_they_act_on() {
    let mut app = title_test_app();
    let edit_rows = |app: &RSpiceApp| {
        shown_rows(ApplicationMenu::Edit, app)
            .into_iter()
            .take(3)
            .map(|(label, _)| label)
            .collect::<Vec<_>>()
    };
    assert_eq!(edit_rows(&app), ["Undo", "Redo", "Repeat last command"]);

    assert!(
        app.state
            .model
            .schematic
            .with_undo("Place resistor", |schematic| {
                schematic.add_component(
                    crate::state::ComponentType::Resistor,
                    crate::state::Point::origin(),
                );
            })
    );
    assert_eq!(edit_rows(&app)[0], "Undo place resistor");

    app.execute(Command::Undo.key());
    assert_eq!(edit_rows(&app)[1], "Redo place resistor");

    app.state.workbench.last_repeatable_command = Some("place-wire".to_owned());
    assert_eq!(edit_rows(&app)[2], "Repeat draw wire");
}

/// Deselect all clears the selection Select all made, and is grey with
/// nothing selected.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn deselect_all_clears_the_selection() {
    let mut app = title_test_app();
    rspice_app_state::workbench::examples::load_example(
        "RC Lowpass Filter",
        &mut app.state.model.schematic,
    );
    assert_eq!(
        app.availability(Command::DeselectAll.key()),
        CommandAvailability::Disabled("nothing is selected")
    );
    app.execute(Command::SelectAll.key());
    assert!(
        !app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .is_empty()
    );
    assert!(app.is_enabled(Command::DeselectAll.key()));

    app.execute(Command::DeselectAll.key());
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .is_empty()
    );
    assert!(!app.is_enabled(Command::DeselectAll.key()));
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

    let ctx = title_test_ctx();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.workbench.activate(Workspace::Design);
    let document = app.state.model.schematic.document_mut_for_test();
    document.components = vec![
        Component::new(1, ComponentType::Resistor, Point::new(0, 0)),
        Component::new(2, ComponentType::Resistor, Point::new(200, 100)),
    ];
    let selection = &mut app.state.model.schematic.session.editor.selection;
    selection.select_component(1);
    selection.select_component(2);
    let arrange = menu_entries::submenu_row_id("Arrange");
    let design_menu = Id::new((
        "workbench.title_menu.trigger",
        MenuTriggerKey::Application(ApplicationMenu::Design),
    ))
    .with("popup");
    let arrange_labels: Vec<&str> = application_menu_entries(ApplicationMenu::Design)
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
        .map(|command| command_registry().label(*command))
        .collect();
    assert_eq!(arrange_labels.len(), 14);
    let before = app.state.model.schematic.document().components[1].pos;
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
        app.state.model.schematic.document().components[1].pos,
        before,
        "Rotate clockwise turned the selection"
    );
}

#[test]
fn design_sheet_menu_reads_navigation_before_authoring() {
    assert_menu_lists(
        ApplicationMenu::Design,
        &DESIGN_SHEET_COMMANDS.map(SheetsCommand::key),
    );
}

#[test]
fn upgraded_design_menu_commands_remain_palette_discoverable() {
    for command in DESIGN_PLACEMENT_COMMANDS
        .into_iter()
        .chain(SELECTION_EDIT_COMMANDS)
    {
        assert!(command.palette_visible());
    }
    for key in DESIGN_MANAGEMENT_COMMANDS {
        assert!(
            command_registry()
                .spec(key)
                .is_some_and(|spec| spec.flags.contains(CommandFlags::PALETTE_VISIBLE)),
            "{key:?}"
        );
    }
    for command in DESIGN_SHEET_COMMANDS {
        assert!(
            command
                .declared_spec()
                .flags
                .contains(CommandFlags::PALETTE_VISIBLE)
        );
    }
    assert!(Command::CreateHierarchy.palette_visible());
    assert!(
        ProjectCommand::CheckAndSave
            .declared_spec()
            .flags
            .contains(CommandFlags::PALETTE_VISIBLE)
    );
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn results_menu_exposes_only_truthful_completed_result_workflows() {
    let labels = rendered_results_menu_labels();
    assert_eq!(
        labels,
        [
            // No result is selected, so View as offers no viewer and is
            // left out rather than opening on nothing.
            "Results browser",
            "Waveforms",
            "New result document…",
            "Add pane…",
            "Compare datasets…",
            "Add measurement…",
            "Family slicing and pivot…",
            "Link A/B cursors",
            "Plot properties…",
            "Calculator…",
            "Expression diagnostics…",
            "Report editor…",
            "Clear result history",
        ]
    );
}

/// Every Verify row is a command the palette finds too: Parameter tuning
/// was unlisted, reachable only from the instance it tunes.
#[test]
fn every_verify_row_is_in_the_palette() {
    let registry = command_registry();
    let rows = listed_commands(application_menu_entries(ApplicationMenu::Verify));
    assert!(rows.contains(&SimulateCommand::Verify(VerificationPage::Tuning).key()));
    for key in rows {
        assert!(
            registry
                .spec(key)
                .is_some_and(|spec| spec.flags.contains(CommandFlags::PALETTE_VISIBLE)),
            "{key:?} is not in the palette"
        );
        assert!(registry.order().contains(&key), "{key:?} is not ordered");
    }
}

/// A row offered only while it can run is left out, not greyed, while it
/// cannot; a submenu whose rows are all left out is left out with them.
#[test]
fn offered_rows_and_empty_submenus_are_left_out() {
    let app = RSpiceApp::test_instance();
    let rows = menu_entries::resolve_menu_rows(
        application_menu_entries(ApplicationMenu::Results),
        &app,
        crate::state::command_catalog::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    let view_as = rows
        .iter()
        .find_map(|row| match row {
            menu_entries::MenuRow::Submenu {
                label: "View as",
                rows,
                ..
            } => Some(rows),
            _ => None,
        })
        .expect("Results holds View as");
    assert!(!view_as.is_empty());
    for row in view_as {
        if let menu_entries::MenuRow::Command(row) = row {
            assert_eq!(
                row.availability,
                CommandAvailability::Hidden,
                "{}",
                row.label
            );
        }
    }
    assert!(
        menu_entries::visible_rows(&rows)
            .iter()
            .all(|row| !matches!(
                row,
                menu_entries::MenuRow::Submenu {
                    label: "View as",
                    ..
                }
            )),
        "View as would open on nothing"
    );
}

/// The plan manager was reachable from the toolbar chip and the palette but
/// from no menu at all. It leads the Simulate menu now: it decides which plan
/// is active, and every row under it - run it, stop it, read its job history,
/// check it - acts on whatever it selected.
///
/// Adding to that plan reads second, directly under choosing it. Before
/// `SimulateCommand::AddAnalysis` existed, the catalogue had exactly one route - the
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
            "Add analysis\u{2026}",
            "Setup",
            "Check and netlist",
            "Run",
            "Stop",
            "Open generated netlist",
            "Run history\u{2026}",
            // Annotate DC operating point reads as a check box, not a
            // plain item, so this list of menu items leaves it out.
            "Probe",
            "Operating-point table",
        ]
    );
    // The rows are the commands', not look-alikes: their labels are the ones
    // the vocabulary publishes, so renaming a command renames its row.
    assert_eq!(
        labels.first().copied(),
        Some(SimulateCommand::ManagePlans.declared_spec().label)
    );
    assert_eq!(
        SimulateCommand::ManagePlans.declared_spec().group,
        "Simulate"
    );
    assert_eq!(
        labels.get(1).copied(),
        Some(SimulateCommand::AddAnalysis.declared_spec().label)
    );
    assert_eq!(
        SimulateCommand::AddAnalysis.declared_spec().group,
        "Simulate"
    );
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
        simulate_menu_row_description(&available, SimulateCommand::ManagePlans),
        Some(None),
        "an available plan manager row must not describe itself as unavailable: {available:?}"
    );

    let (availability, disabled) = rendered_simulate_menu_rows(false);
    let CommandAvailability::Disabled(reason) = availability else {
        panic!("the plan manager stayed available with no stable analysis plan");
    };
    assert_eq!(
        simulate_menu_row_description(&disabled, SimulateCommand::ManagePlans),
        Some(Some(reason.to_owned())),
        "the disabled row must name the command's own reason: {disabled:?}"
    );
}

/// A menu row wears an icon only where the command has a toolbar button,
/// and then the button's own icon, so the two read as one command. Every
/// row used to carry one, most of them stand-ins (a check for Legal, a
/// folder for Close project) that matched nothing on screen. The check
/// buttons' glyph is the exception: in a menu a check says a toggle is on,
/// and Check cellview and Check design wore one beside View's Navigator.
#[test]
fn menu_icons_mark_only_the_toolbars_twins_with_the_buttons_icon() {
    let app = RSpiceApp::test_instance();
    for (command, icon) in [
        (Command::PlaceInstance, WorkbenchIcon::Instance),
        (Command::PlaceJunction, WorkbenchIcon::Junction),
        (Command::PlacePin, WorkbenchIcon::Pin),
        (Command::PlaceText, WorkbenchIcon::Text),
        (Command::Delete, WorkbenchIcon::Trash),
        (Command::AscendHierarchy, WorkbenchIcon::ArrowUp),
        (Command::DescendHierarchyDirect, WorkbenchIcon::Layers),
        (Command::VisibilityOptions, WorkbenchIcon::Visibility),
        (Command::CompileVerilogA, WorkbenchIcon::Netlist),
        (Command::AutomationConsole, WorkbenchIcon::Terminal),
    ] {
        assert_eq!(
            app.command_icon(command.key()),
            Some(icon),
            "{}",
            command.spec().id
        );
    }
    assert_eq!(
        app.command_icon(ModelsCommand::ModelCompareRelease.key()),
        Some(WorkbenchIcon::Compare)
    );
    assert_eq!(
        app.command_icon(SimulateCommand::Run.key()),
        Some(WorkbenchIcon::Run)
    );
    for (command, icon) in [
        (ProjectCommand::NewProject, WorkbenchIcon::Add),
        (ProjectCommand::Save, WorkbenchIcon::Save),
    ] {
        assert_eq!(app.command_icon(command.key()), Some(icon), "{command:?}");
    }
    assert_eq!(app.command_icon(ProjectCommand::CloseProject.key()), None);
    assert_eq!(
        app.command_icon(ids::PRINT_HARDCOPY),
        Some(WorkbenchIcon::File)
    );
    for command in [
        Command::LegalPrivacy,
        Command::About,
        Command::HelpCenter,
        Command::Exit,
        Command::SelectAll,
        Command::RenameSelection,
        Command::DescendHierarchy,
        Command::KeyboardShortcuts,
        Command::CheckCellview,
        Command::CheckDesign,
        Command::ValidateCodeDocument,
    ] {
        assert_eq!(
            app.command_icon(command.key()),
            None,
            "{}",
            command.spec().id
        );
    }
    for key in [
        SimulateCommand::Preflight.key(),
        ModelsCommand::ModelValidate.key(),
    ] {
        assert_eq!(app.command_icon(key), None, "{key:?}");
    }
}

/// With a symbol in front, Design places the symbol's own shapes, offers the
/// way back out of a symbol edited in place and saves the symbol; with a
/// Verilog-A view in front, it compiles. Rotate counter-clockwise turns a
/// symbol's selection as Rotate clockwise does: it was grey in every symbol.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn design_follows_the_view_in_front() {
    let veriloga = shown_rows(ApplicationMenu::Design, &veriloga_view_app())
        .into_iter()
        .map(|(label, _)| label)
        .collect::<Vec<_>>();
    assert!(
        veriloga.ends_with(&["Compile Verilog-A".to_owned(), "Build settings…".to_owned()]),
        "{veriloga:?}"
    );
    let schematic = shown_rows(ApplicationMenu::Design, &title_test_app())
        .into_iter()
        .map(|(label, _)| label)
        .collect::<Vec<_>>();
    assert!(
        !schematic.iter().any(|label| label == "Compile Verilog-A"),
        "{schematic:?}"
    );

    let mut app = symbol_view_app();
    let rotate_ccw = Command::RotateSelectionCounterClockwise.key();
    assert!(!app.is_enabled(rotate_ccw), "nothing is selected yet");
    app.execute(Command::SelectAll.key());
    assert!(
        app.is_enabled(rotate_ccw),
        "Rotate counter-clockwise turns the symbol's selection"
    );
    app.execute(rotate_ccw);
    let turned = app
        .state
        .model
        .load_active_symbol_document()
        .expect("the symbol loads");
    assert_eq!(
        turned.body,
        [crate::state::SymbolShape::Dot {
            center: crate::state::Point::new(0, -20),
            radius: 2,
        }],
        "a quarter counter-clockwise about the origin"
    );
}

/// Annotate DC operating point shows the operating point on the schematic
/// while checked and leaves only the violations drawn while not.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn annotate_dc_operating_point_switches_the_schematic_annotation() {
    use crate::state::SchematicAnnotationVisibility;
    let mut app = title_test_app();
    app.state.workbench.activate(Workspace::Design);
    let key = Command::AnnotateOperatingPoint.key();
    assert!(app.is_enabled(key));
    assert_eq!(app.checked(key), Some(true));
    app.execute(key);
    assert_eq!(
        app.state.design.schematic_visibility.annotations,
        SchematicAnnotationVisibility::ViolationsOnly
    );
    assert_eq!(app.checked(key), Some(false));
    app.execute(key);
    assert_eq!(
        app.state.design.schematic_visibility.annotations,
        SchematicAnnotationVisibility::OperatingPoint
    );
}

/// Merge all windows and Recover off-screen windows are a desktop
/// application's: the browser and a tablet show one window, so Window
/// leaves them out there. Workspace reaches all eight workspaces, each by
/// its Alt digit, in the digits' order.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn window_gathers_windows_only_on_the_desktop_and_reaches_every_workspace() {
    use crate::state::command_catalog::CommandPlatform;
    let app = title_test_app();
    let window = application_menu_entries(ApplicationMenu::Window);
    let gathers = |rows: &[String]| {
        rows.iter().any(|row| {
            row.starts_with("Merge all windows\u{2026} |")
                || row.starts_with("Recover off-screen windows\u{2026} |")
        })
    };
    let os = egui::os::OperatingSystem::Windows;
    assert!(gathers(&menu_pin_for(
        window,
        &app,
        CommandPlatform::Desktop,
        os
    )));
    for platform in [CommandPlatform::Browser, CommandPlatform::Tablet] {
        let rows = menu_pin_for(window, &app, platform, os);
        assert!(!gathers(&rows), "{}: {rows:?}", platform.label());
    }

    let rows = menu_pin(window, &app);
    let workspaces = rows
        .iter()
        .skip_while(|row| *row != "Workspace >")
        .skip(1)
        .take_while(|row| row.starts_with("  "))
        .collect::<Vec<_>>();
    assert_eq!(workspaces.len(), 8, "{rows:?}");
    for (digit, row) in (1..).zip(&workspaces) {
        assert!(row.contains(&format!("| Alt+{digit} |")), "{row}");
    }
}

/// Exit ends the File menu on Windows and Linux; on macOS the application
/// menu's Quit ends the application, so File does not repeat it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn exit_ends_the_file_menu_on_windows_and_linux_but_not_macos() {
    let app = title_test_app();
    let file = application_menu_entries(ApplicationMenu::File);
    for os in [
        egui::os::OperatingSystem::Windows,
        egui::os::OperatingSystem::Nix,
    ] {
        let rows = menu_pin_on(file, &app, os);
        assert_eq!(
            rows.last().map(String::as_str),
            Some("Exit | exit-rspice | Alt+F4 | on"),
            "{os:?}"
        );
    }
    let rows = menu_pin_on(file, &app, egui::os::OperatingSystem::Mac);
    assert!(
        rows.iter().all(|row| !row.contains("exit-rspice")),
        "{rows:#?}"
    );
}

/// File ▸ Open recent lists the recent-files list as it stands, newest
/// first, projects by name and schematics by file name, and ends in the
/// launcher holding every project.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn open_recent_lists_the_recent_files_newest_first_then_all_projects() {
    use crate::services::recent_files::{RecentFile, RecentKind};
    let recent = |kind, path: &str| RecentFile {
        kind,
        path: std::path::PathBuf::from(path),
        opened_at_unix_ms: 0,
        pinned: false,
        owner: None,
        tags: Vec::new(),
    };
    let mut app = title_test_app();
    app.state.platform.recent_files = vec![
        recent(RecentKind::Project, "C:/work/amplifier.rsproj"),
        recent(RecentKind::Schematic, "C:/work/bias.rsch"),
        recent(RecentKind::Project, "C:/work/filter.rsproj"),
    ];
    let rows = menu_pin(application_menu_entries(ApplicationMenu::File), &app);
    let start = rows
        .iter()
        .position(|row| row == "Open recent >")
        .expect("File lists Open recent");
    assert_eq!(
        rows[start + 1..start + 6],
        [
            "  amplifier | open-project |  | on",
            "  bias.rsch | open-project |  | on",
            "  filter | open-project |  | on",
            "  ---",
            "  All projects… | project-launcher | Ctrl+Shift+O | on",
        ]
    );
}

/// Run is grey only while a run is in progress: an unrunnable plan stays
/// enabled on purpose, so the run itself can say why it cannot start. The
/// grey row said the plan was not runnable, which was never the reason.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_grey_run_row_says_a_run_is_already_in_progress() {
    let mut app = title_test_app();
    app.state.model.simulation.execution.trigger_simulation = true;
    assert_eq!(
        app.command_availability(SimulateCommand::Run.key()),
        CommandAvailability::Disabled("a run is already in progress")
    );
    let rows = menu_pin(application_menu_entries(ApplicationMenu::Simulate), &app);
    assert!(
        rows.iter()
            .any(|row| row == "Run | start-run | F5 | off: a run is already in progress"),
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
        app.cloud.account.availability(),
        crate::services::cloud_account::CloudAccountAvailability::UnconfiguredBuild,
        "the test build pins no cloud endpoints"
    );
    for command in [CloudCommand::LiveSession, CloudCommand::PublishToWeb] {
        assert_eq!(
            app.command_availability(command.key()),
            CommandAvailability::Hidden
        );
    }
    let rows = menu_entries::resolve_menu_rows(
        application_menu_entries(ApplicationMenu::File),
        &app,
        crate::state::command_catalog::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    let shown = visible_row_names(&rows);
    let export = shown
        .iter()
        .position(|name| name == "Export")
        .expect("File lists the Export submenu");
    assert_eq!(
        shown[export + 1..export + 3],
        ["---", "Print…"],
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
        Command::ToggleNavigator.key(),
        Command::ToggleInspector.key(),
        Command::ToggleConsole.key(),
        ResultsCommand::ToggleResultsSplit.key(),
        Command::ToggleFocusMode.key(),
        Command::ToggleFullScreen.key(),
        ResultsCommand::ToggleLinkedCursors.key(),
    ];
    for key in switches {
        assert!(Host::checked(&app, key).is_some(), "{key:?} reads no state");
    }
    assert_eq!(Host::checked(&app, ProjectCommand::Save.key()), None);
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
        app.state.results.results.session.linked_cursors = on;
        for key in [
            ResultsCommand::ToggleResultsSplit.key(),
            Command::ToggleFullScreen.key(),
            ResultsCommand::ToggleLinkedCursors.key(),
        ] {
            assert_eq!(Host::checked(&app, key), Some(on), "{key:?}");
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

    let ctx = title_test_ctx();
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
                    draw_menu_entries(ui, &app, application_menu_entries(ApplicationMenu::View))
                });
        },
    );
    let nodes = output
        .platform_output
        .accesskit_update
        .expect("AccessKit View-menu tree")
        .nodes;
    let read = |label: &str| {
        nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .map(|(_, node)| (node.role(), node.toggled()))
            .unwrap_or_else(|| panic!("the View menu has no {label:?} row"))
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
    assert_eq!(read("Display options…"), (Role::MenuItem, None));
}

/// With no one signed in and no license, the account button shows a
/// generic person instead of "RS", initials that named no one; a license
/// holder's initials still show.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_account_button_shows_a_person_until_an_identity_names_someone() {
    fn painted(initials: Option<&str>) -> (Vec<String>, usize) {
        let ctx = title_test_ctx();
        crate::ui::Theme::default().apply(&ctx);
        let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default()
                .frame(Frame::NONE)
                .show(ctx, |ui| {
                    let _ = account_action(ui, initials, false);
                });
        });
        let mut texts = Vec::new();
        let mut marks = 0;
        for shape in &output.shapes {
            match &shape.shape {
                egui::epaint::Shape::Text(text) => texts.push(text.galley.job.text.clone()),
                _ => marks += 1,
            }
        }
        (texts, marks)
    }

    let mut app = title_test_app();
    assert_eq!(account_initials(&app), None);
    let (texts, person_marks) = painted(None);
    assert!(
        texts.is_empty(),
        "no initials without an identity: {texts:?}"
    );

    app.state.cloud.license = Some(crate::services::license::LicenseInfo {
        licensed_to: "Ada Lovelace".to_owned(),
        tier: String::new(),
        updates_until: String::new(),
        license_id: String::new(),
        features: Vec::new(),
        updates_expired: false,
    });
    assert_eq!(account_initials(&app).as_deref(), Some("AL"));
    let (texts, lettered_marks) = painted(Some("AL"));
    assert_eq!(texts, ["AL"]);
    assert!(
        person_marks > lettered_marks,
        "the person glyph is painted in place of the letters"
    );
}

/// The labels of the command and submenu rows `menu` shows for `app`, in
/// order, and whether each one can be chosen.
#[cfg(not(target_arch = "wasm32"))]
fn shown_rows(menu: ApplicationMenu, app: &RSpiceApp) -> Vec<(String, bool)> {
    let rows = menu_entries::resolve_menu_rows(
        application_menu_entries(menu),
        app,
        crate::state::command_catalog::CommandPlatform::Desktop,
        egui::os::OperatingSystem::Windows,
    );
    menu_entries::visible_rows(&rows)
        .into_iter()
        .filter_map(|row| match row {
            menu_entries::MenuRow::Command(row) => {
                Some((row.label.to_string(), row.availability.is_available()))
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
    let ctx = title_test_ctx();
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
    let mut frame = |events| {
        menu_items_after_frame(&ctx, events, |ui| {
            menus(ui, &mut app, MenuProjection::All, false, 35.0);
        })
    };

    Popup::open_id(&ctx, results_menu);
    let _ = frame(Vec::new());
    let mut walked = Vec::new();
    for _ in &shown {
        let _ = frame(vec![title_key_event(Key::ArrowDown)]);
        let (_, focused) = frame(Vec::new());
        walked.push(focused.expect("a row has the focus"));
    }
    assert_eq!(
        walked,
        shown
            .iter()
            .map(|(label, _)| label.clone())
            .collect::<Vec<_>>()
    );

    let _ = frame(vec![title_key_event(Key::Home)]);
    for _ in 0..grey_index {
        let _ = frame(Vec::new());
        let _ = frame(vec![title_key_event(Key::ArrowDown)]);
    }
    let (_, focused) = frame(Vec::new());
    assert_eq!(focused.as_deref(), Some(grey.as_str()));
    let _ = frame(vec![title_key_event(Key::Enter)]);
    let (_, focused) = frame(Vec::new());
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
    let ctx = title_test_ctx();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = title_test_app();
    app.state.close_project_session();
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
    let mut frame = |events| {
        menu_items_after_frame(&ctx, events, |ui| {
            menus(ui, &mut app, MenuProjection::All, false, 35.0);
        })
    };
    // One frame carries the event, the next shows where the focus went.
    let mut press = |events: Vec<egui::Event>| {
        let _ = frame(events);
        frame(Vec::new()).1
    };
    let key = |key| vec![title_key_event(key)];

    Popup::open_id(&ctx, popup);
    let _ = press(Vec::new());
    assert_eq!(press(key(Key::ArrowDown)).as_deref(), Some(labels[0]));
    assert_eq!(press(key(Key::ArrowDown)).as_deref(), Some(labels[1]));
    assert_eq!(press(key(Key::End)).as_deref(), labels.last().copied());
    assert_eq!(press(key(Key::Home)).as_deref(), Some(labels[0]));
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
        press(vec![egui::Event::Text(letter.clone())]).as_deref(),
        found,
        "typing {letter:?} in {labels:?}"
    );
    assert_eq!(press(key(Key::Escape)).as_deref(), Some(menu.label()));
    assert!(!Popup::is_id_open(&ctx, popup), "Escape closes {menu:?}");
}

/// The title bar in a window of `size`, laid out as the shell lays it out.
#[cfg(not(target_arch = "wasm32"))]
struct TitleBarWindow {
    ctx: Context,
    app: RSpiceApp,
    size: egui::Vec2,
}

#[cfg(not(target_arch = "wasm32"))]
impl TitleBarWindow {
    fn new(size: egui::Vec2) -> Self {
        let ctx = title_test_ctx();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        Self {
            ctx,
            app: title_test_app(),
            size,
        }
    }

    /// One frame carrying `events`, and the labelled AccessKit nodes it
    /// published with their bounds.
    fn frame(&mut self, events: Vec<egui::Event>) -> Vec<(String, egui::accesskit::Role, Rect)> {
        let size = self.size;
        let app = &mut self.app;
        let output = self.ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                events,
                ..Default::default()
            },
            |ui| {
                let layout = LayoutSpec::resolve(size.x, size.y, &app.state.workbench);
                show(ui, app, layout);
            },
        );
        output
            .platform_output
            .accesskit_update
            .expect("AccessKit title-bar tree")
            .nodes
            .into_iter()
            .filter_map(|(_, node)| {
                let bounds = node.bounds()?;
                Some((
                    node.label()?.to_owned(),
                    node.role(),
                    Rect::from_min_max(
                        egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                        egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                    ),
                ))
            })
            .collect()
    }

    /// Frames until a just-opened popup has settled its size.
    fn settle(&mut self) -> Vec<(String, egui::accesskit::Role, Rect)> {
        let mut nodes = Vec::new();
        for _ in 0..3 {
            nodes = self.frame(Vec::new());
        }
        nodes
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn is_menu_row(role: egui::accesskit::Role) -> bool {
    matches!(
        role,
        egui::accesskit::Role::MenuItem | egui::accesskit::Role::MenuItemCheckBox
    )
}

/// At every desktop width above 820 pt the bar shows all eleven menus in
/// order, clear of the buttons at its right, and each opens onto every row
/// it holds, on screen. At 821 to 1360 pt the bar used to hide the last
/// three to six menus behind an overflow.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_menu_is_reachable_on_the_bar_at_every_desktop_width() {
    for width in [821.0, 1_021.0, 1_280.0, 1_440.0] {
        let mut window = TitleBarWindow::new(egui::vec2(width, 900.0));
        let nodes = window.settle();
        let triggers = ALL_MENUS
            .into_iter()
            .map(|menu| {
                nodes
                    .iter()
                    .find(|(label, role, rect)| {
                        label == menu.label() && is_menu_row(*role) && rect.top() < 40.0
                    })
                    .map(|(_, _, rect)| *rect)
                    .unwrap_or_else(|| panic!("{width} pt: the bar has no {menu:?} menu"))
            })
            .collect::<Vec<_>>();
        let cluster_left = nodes
            .iter()
            .filter(|(label, _, _)| {
                [
                    "Search and run a command",
                    "Open preferences",
                    "Open account, organization, and licensing",
                ]
                .contains(&label.as_str())
            })
            .map(|(_, _, rect)| rect.left())
            .fold(f32::INFINITY, f32::min);
        assert!(
            cluster_left.is_finite(),
            "{width} pt: no right-hand buttons"
        );
        for pair in triggers.windows(2) {
            assert!(
                pair[0].right() <= pair[1].left() + 0.5,
                "{width} pt: {pair:?}"
            );
        }
        let last = triggers.last().expect("eleven menus");
        assert!(
            last.right() <= cluster_left,
            "{width} pt: Help ends at {} under the buttons from {cluster_left}",
            last.right()
        );

        for menu in ALL_MENUS {
            let popup = Id::new((
                "workbench.title_menu.trigger",
                MenuTriggerKey::Application(menu),
            ))
            .with("popup");
            Popup::open_id(&window.ctx, popup);
            let nodes = window.settle();
            for (label, _) in shown_rows(menu, &window.app) {
                assert!(
                    nodes.iter().any(|(drawn, role, rect)| {
                        *drawn == label
                            && is_menu_row(*role)
                            && rect.top() >= 35.0
                            && rect.left() >= 0.0
                            && rect.right() <= width + 0.5
                    }),
                    "{width} pt: {menu:?} does not show {label:?} on screen"
                );
            }
        }
    }
}

/// In the compact layout, at phone and tablet widths, one Menu button
/// holds every menu as a submenu on 44 pt rows, and each submenu opens onto
/// every row its menu holds, on 44 pt rows too. The compact layout used to
/// offer no application menu at all.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_compact_menu_button_reaches_every_menu_on_touch_rows() {
    for size in [egui::vec2(390.0, 844.0), egui::vec2(768.0, 1_024.0)] {
        let mut window = TitleBarWindow::new(size);
        let trigger = Id::new(("workbench.title_menu.trigger", MenuTriggerKey::Compact));
        let nodes = window.settle();
        assert!(
            nodes
                .iter()
                .any(|(label, role, _)| label == "Menu" && is_menu_row(*role)),
            "{size:?}: no Menu button"
        );
        assert!(
            !nodes.iter().any(|(label, _, _)| label == "File"),
            "{size:?}: the compact bar shows no menu of its own"
        );

        Popup::open_id(&window.ctx, trigger.with("popup"));
        let nodes = window.settle();
        for menu in ALL_MENUS {
            let row = nodes
                .iter()
                .find(|(label, role, rect)| {
                    label == menu.label() && is_menu_row(*role) && rect.top() > 30.0
                })
                .unwrap_or_else(|| panic!("{size:?}: the Menu button has no {menu:?} row"));
            assert!(
                (row.2.height() - MENU_TOUCH_ROW_HEIGHT).abs() < 0.01,
                "{row:?}"
            );
        }

        for menu in ALL_MENUS {
            window.ctx.memory_mut(|memory| {
                memory.request_focus(menu_entries::submenu_row_id(menu.label()));
            });
            let _ = window.frame(Vec::new());
            let _ = window.frame(vec![title_key_event(Key::ArrowRight)]);
            let nodes = window.settle();
            // On screen whole, scrolling where its rows are taller.
            let area = window
                .ctx
                .memory(|memory| {
                    memory.area_rect(egui::containers::menu::SubMenu::id_from_widget_id(
                        menu_entries::submenu_row_id(menu.label()),
                    ))
                })
                .unwrap_or_else(|| panic!("{size:?}: {menu:?} did not open"));
            assert!(
                area.top() >= 0.0
                    && area.bottom() <= size.y + 0.5
                    && area.left() >= -0.5
                    && area.right() <= size.x + 0.5,
                "{size:?}: {menu:?} hangs off screen at {area:?}"
            );
            // A submenu too tall to open below its row opens above it, over
            // the bar, so its rows are found from its top down rather than
            // below the bar.
            for (label, _) in shown_rows(menu, &window.app) {
                let row = nodes
                    .iter()
                    .find(|(drawn, role, rect)| {
                        *drawn == label && is_menu_row(*role) && rect.top() >= area.top()
                    })
                    .unwrap_or_else(|| panic!("{size:?}: {menu:?} does not show {label:?}"));
                assert!(
                    (row.2.height() - MENU_TOUCH_ROW_HEIGHT).abs() < 0.01,
                    "{size:?}: {menu:?} row {row:?}"
                );
            }
            let _ = window.frame(vec![title_key_event(Key::ArrowLeft)]);
        }
    }
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
    let ctx = title_test_ctx();
    crate::ui::Theme::default().apply(&ctx);
    let mut needed = Vec::new();
    for app in [title_test_app(), source_editor_app()] {
        let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                for menu in ALL_MENUS {
                    let rows = menu_rows(application_menu_entries(menu), &app, ui.ctx());
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
        let ctx = title_test_ctx();
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
                            let rows = menu_rows(application_menu_entries(menu), &app, ui.ctx());
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
            // On desktop rows, no menu scrolls in a window 900 pt tall.
            assert!(
                large_targets || rows_height <= menu_popup_height_for_viewport(900.0),
                "{} scrolls its {rows_height} pt of rows in a 900 pt window",
                menu.label()
            );
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
            menu_entries::MenuRow::Command(row) => row.label.to_string(),
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
                application_menu_entries(menu),
                &app,
                crate::state::command_catalog::CommandPlatform::Desktop,
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
#[test]
fn title_context_ellipsis_is_measured_instead_of_character_estimated() {
    let ctx = title_test_ctx();
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
