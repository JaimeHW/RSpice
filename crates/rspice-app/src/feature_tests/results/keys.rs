//! The Results keys and commands: every Results command on a menu row with
//! its key, no key of Results shared with another command that acts there,
//! the keys acting through the shortcut resolver with a plot focused by Tab,
//! and a command raised where it cannot run refused with its reason.

use egui::{Context, Event, Key, Modifiers, RawInput, Rect};

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::{CommandRegistry, command_registry};
use rspice_app_results::workbench::results::commands::{OneAxisZoom, ResultsCommand};
use rspice_app_state::state::command_catalog::{CommandKey, CommandPlatform};
use rspice_app_state::state::shortcuts::{
    ShortcutBindingSlot, ShortcutPreferences, ShortcutProfileIssueCode, ShortcutSequence,
    ShortcutStroke,
};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::title_bar::application_menus::ALL_MENUS;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::{self, MenuRow, menu_keys};
use rspice_app_workbench::workbench::framework::command_registry::{
    CommandAvailability, CommandFlags, WorkbenchCommand,
};
use rspice_app_workbench::workbench::framework::host::Host;
use rspice_app_workbench::workbench::state::pane_focus::Pane;

/// Every command row the menus show, submenus included.
fn command_rows(rows: &[MenuRow], out: &mut Vec<menu_entries::CommandRow>) {
    for row in rows {
        match row {
            MenuRow::Command(row) => out.push(row.clone()),
            MenuRow::Submenu { rows, .. } => command_rows(rows, out),
            MenuRow::Separator => {}
        }
    }
}

/// Every Results command has a row in the menu bar, and a command with a key
/// shows it on its row, as ADE's and ViVA's menus print each bindkey.
#[test]
fn every_results_command_is_on_a_menu_row_with_its_key() {
    let app = RSpiceApp::test_instance();
    let mut keys = Vec::new();
    for menu in ALL_MENUS {
        menu_keys(app.menu_entries(menu), &mut keys);
    }
    let mut rows = Vec::new();
    for menu in ALL_MENUS {
        command_rows(
            &menu_entries::resolve_menu_rows(
                app.menu_entries(menu),
                &app,
                CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            ),
            &mut rows,
        );
    }
    let mut missing = Vec::new();
    for command in ResultsCommand::ALL {
        let key = command.key();
        if !keys.contains(&key) {
            missing.push(format!("{command:?} has no menu row"));
            continue;
        }
        let bound = command
            .declared_spec()
            .bindings
            .iter()
            .any(|binding| binding.supports(CommandPlatform::Desktop));
        if !bound {
            continue;
        }
        match rows.iter().find(|row| row.command == key) {
            Some(row) if row.shortcut.is_empty() => {
                missing.push(format!("{command:?}'s row shows no key"));
            }
            Some(_) => {}
            None => missing.push(format!("{command:?}'s row is not shown")),
        }
    }
    assert!(missing.is_empty(), "{}", missing.join("\n"));
}

/// No key a Results command is bound to is also a key, or the start of a
/// key, of another command that can act in Results, on any platform.
#[test]
fn no_results_key_collides_with_another_command() {
    let audit = ShortcutPreferences::default().audit(&CommandRegistry);
    let results: Vec<CommandKey> = ResultsCommand::ALL.map(ResultsCommand::key).to_vec();
    let collisions: Vec<&str> = audit
        .issues()
        .iter()
        .filter(|issue| {
            matches!(
                issue.code(),
                ShortcutProfileIssueCode::ExactCollision
                    | ShortcutProfileIssueCode::PrefixCollision
            )
        })
        .map(|issue| issue.message())
        .filter(|message| {
            results.iter().any(|key| {
                message.starts_with(&format!("{} conflicts with ", key.as_str()))
                    || message.contains(&format!(" conflicts with {} on ", key.as_str()))
            })
        })
        .collect();
    assert!(collisions.is_empty(), "{collisions:#?}");
}

const TIME: [f64; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

/// A finished transient run whose V(out) peaks once, at 0.5.
fn peaked_run() -> SimulationRun {
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                TIME.to_vec(),
                vec![0.0, 0.4, 1.0, 0.4, 0.0],
                "#ffffff",
            )
            .with_unit("V"),
        ]),
    );
    run
}

/// The resolver and the Results centre run together a frame at a time, as
/// the workbench runs them.
struct Desk {
    ctx: Context,
    app: RSpiceApp,
    time: f64,
    /// Whether a navigator stands beside the document, a pane F6 goes to.
    navigator: bool,
    /// The texts the last frame painted.
    texts: Vec<String>,
}

/// The one control of the navigator a [`Desk`] may draw.
const NAVIGATOR_CONTROL: &str = "Navigator control";

impl Desk {
    fn new() -> Self {
        Self::with(peaked_run(), false)
    }

    fn with(run: SimulationRun, navigator: bool) -> Self {
        let ctx = Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.model.simulation.retained.runs = vec![run].into();
        assert!(app.state.model.simulation.select_run(0));
        app.state.workbench.activate(Workspace::Results);
        let mut desk = Self {
            ctx,
            app,
            time: 0.0,
            navigator,
            texts: Vec::new(),
        };
        desk.frame(Vec::new());
        desk.frame(Vec::new());
        desk
    }

    /// Run one frame and return the label of the node the keyboard is on.
    fn frame(&mut self, events: Vec<Event>) -> Option<String> {
        self.time += 0.02;
        let app = &mut self.app;
        let navigator = self.navigator;
        let output = self.ctx.run_ui(
            RawInput {
                events,
                focused: true,
                time: Some(self.time),
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1600.0, 900.0),
                )),
                ..Default::default()
            },
            |ui| {
                app.state.workbench.pane_focus.begin_frame();
                app.handle_shortcuts(ui.ctx());
                if navigator {
                    let panel = egui::Panel::left("navigator").show(ui, |ui| {
                        let _ = ui.button(NAVIGATOR_CONTROL);
                    });
                    app.state.workbench.pane_focus.record(
                        ui.ctx(),
                        Pane::Navigator,
                        &panel.response,
                    );
                }
                let document = egui::CentralPanel::default().show(ui, |ui| {
                    super::with_surface_parts(app, |state, controller| {
                        rspice_app_results::workbench::documents::result_document::show(
                            ui, state, controller,
                        );
                    });
                });
                app.state
                    .workbench
                    .pane_focus
                    .record(ui.ctx(), Pane::Document, &document.response);
                app.state.take_pane_focus_step(ui.ctx());
            },
        );
        self.texts = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect();
        let update = output.platform_output.accesskit_update?;
        update
            .nodes
            .iter()
            .find(|(id, _)| *id == update.focus)
            .and_then(|(_, node)| node.label().map(str::to_owned))
    }

    /// Press and release `key` with `modifiers`.
    fn press(&mut self, key: Key, modifiers: Modifiers) -> Option<String> {
        let event = |pressed| Event::Key {
            key,
            physical_key: Some(key),
            pressed,
            repeat: false,
            modifiers,
        };
        self.frame(vec![event(true)]);
        self.frame(vec![event(false)])
    }

    fn cursor_a(&self) -> Option<f64> {
        self.app.state.results.results.session.cursors.a
    }

    /// Tab until the plot has the keyboard.
    fn focus_plot(&mut self) {
        for _ in 0..80 {
            if self
                .press(Key::Tab, Modifiers::NONE)
                .is_some_and(|label| label.contains("Drag to pan"))
            {
                return;
            }
        }
        panic!("Tab reaches no plot");
    }

    /// Whether the plot states a zoom armed on X alone.
    fn zoom_x_armed(&self) -> bool {
        self.texts.iter().any(|text| text == "zoom X")
    }
}

/// Tab onto the plot, and the Results keys go on acting: Shift+Right takes
/// cursor A to the peak, C disarms and arms the cursors, and Delete hides the
/// selected trace. Focusing a pane turned every Results key off before. The
/// bare arrows are the focused plot's own: they pan it and leave the cursor.
#[test]
fn with_a_plot_focused_by_tab_the_cursor_keys_and_delete_still_act() {
    let mut desk = Desk::new();
    let mut focus = None;
    for _ in 0..80 {
        focus = desk.press(Key::Tab, Modifiers::NONE);
        if focus
            .as_deref()
            .is_some_and(|label| label.contains("Drag to pan"))
        {
            break;
        }
    }
    let plot = focus.expect("Tab reaches the plot");
    assert!(plot.contains("Drag to pan"), "Tab reaches the plot: {plot}");

    let focus = desk.press(Key::ArrowRight, Modifiers::SHIFT);
    assert_eq!(desk.cursor_a(), Some(0.5), "Shift+Right goes to the peak");
    assert!(
        focus.is_some_and(|label| label.contains("Drag to pan")),
        "and the plot keeps the keyboard"
    );

    desk.press(Key::ArrowRight, Modifiers::NONE);
    assert_eq!(
        desk.cursor_a(),
        Some(0.5),
        "Right is the focused plot's: it pans, the cursor stays"
    );

    desk.press(Key::C, Modifiers::NONE);
    assert_eq!(desk.cursor_a(), None, "C disarms the cursors");
    desk.press(Key::C, Modifiers::NONE);
    assert!(
        desk.app
            .state
            .results
            .results
            .session
            .cursor_tool
            .is_armed(),
        "and arms them again"
    );

    let run = &desk.app.state.model.simulation.retained.runs[0];
    desk.app.state.results.results.session.selected_trace = Some(
        rspice_results_ui::selection::SelectedResultTrace::from_run_indices(run, 0, 0)
            .expect("V(out) is selectable"),
    );
    assert!(
        desk.app
            .command_key_available(WorkbenchCommand::Delete.key())
    );
    desk.press(Key::Delete, Modifiers::NONE);
    assert_eq!(
        desk.app.state.results.results.session.selected_trace, None,
        "Delete takes the selected trace off the sheet"
    );
    let hidden = desk
        .app
        .state
        .model
        .workspace
        .content
        .visualization_documents
        .iter()
        .filter(|document| document.working_view().is_some())
        .flat_map(|document| document.traces())
        .any(|trace| trace.label == "V(out)" && !trace.visible);
    assert!(hidden, "the view hides V(out)");
}

/// A command raised where it cannot run is not run, and its menu row is off
/// and says why. With no result retained, arming the cursors from a
/// feature's menu or another command armed them regardless.
#[test]
fn an_unavailable_command_is_not_run_and_its_row_says_why() {
    let mut app = RSpiceApp::test_instance();
    app.state.model.simulation.retained.runs = Vec::new().into();
    app.state.workbench.activate(Workspace::Results);
    let key = ResultsCommand::ToggleCursorTool.key();
    let CommandAvailability::Disabled(reason) = app.command_availability(key) else {
        panic!("the cursors cannot be armed with no result retained");
    };
    let armed = app.state.results.results.session.cursor_tool.is_armed();

    app.execute_command_key(key);

    assert_eq!(
        app.state.results.results.session.cursor_tool.is_armed(),
        armed,
        "the refused command ran"
    );
    let mut rows = Vec::new();
    for menu in ALL_MENUS {
        command_rows(
            &menu_entries::resolve_menu_rows(
                app.menu_entries(menu),
                &app,
                CommandPlatform::Desktop,
                egui::os::OperatingSystem::Windows,
            ),
            &mut rows,
        );
    }
    let row = rows
        .iter()
        .find(|row| row.command == key)
        .expect("the cursors have a menu row");
    assert_eq!(row.availability, CommandAvailability::Disabled(reason));
}

/// Tab onto a trace's legend entry and Shift+F10 opens the trace's menu, as
/// a right click does: the trace's menu took the pointer alone before.
#[test]
fn shift_f10_opens_the_trace_menu() {
    let mut desk = Desk::new();
    let mut focus = None;
    for _ in 0..80 {
        focus = desk.press(Key::Tab, Modifiers::NONE);
        if focus.as_deref() == Some("V(out)") {
            break;
        }
    }
    assert_eq!(focus.as_deref(), Some("V(out)"), "Tab reaches the trace");
    assert!(!egui::Popup::is_any_open(&desk.ctx));
    desk.press(Key::F10, Modifiers::SHIFT);
    assert!(
        egui::Popup::is_any_open(&desk.ctx),
        "Shift+F10 opens the trace's menu"
    );
}

/// F6 steps from one pane's plot to the next before it goes on to the next
/// part of the workbench, and Shift+F6 comes back the same way: a strip's
/// panes took the pointer alone to reach before.
#[test]
fn f6_steps_through_the_panes_then_on_to_the_navigator() {
    let mut run = peaked_run();
    run.analyses[0].waveforms.push(
        RetainedTrace::new(
            "I(V1)",
            TIME.to_vec(),
            vec![0.0, 1.0, 2.0, 1.0, 0.0],
            "#ff8800",
        )
        .with_unit("A"),
    );
    let mut desk = Desk::with(run, true);
    let plot = |label: &Option<String>| {
        label
            .as_deref()
            .is_some_and(|label| label.contains("Drag to pan"))
    };
    let mut focus = None;
    for _ in 0..80 {
        focus = desk.press(Key::Tab, Modifiers::NONE);
        if plot(&focus) {
            break;
        }
    }
    let first = focus.expect("Tab reaches a plot");
    assert!(plot(&Some(first.clone())), "Tab reaches a plot: {first}");

    let second = desk.press(Key::F6, Modifiers::NONE);
    assert!(plot(&second), "F6 goes to the next pane's plot: {second:?}");
    assert_ne!(second.as_deref(), Some(first.as_str()));
    assert_eq!(
        desk.press(Key::F6, Modifiers::NONE).as_deref(),
        Some(NAVIGATOR_CONTROL),
        "after the last pane, F6 goes on to the next part"
    );
    assert_eq!(
        desk.press(Key::F6, Modifiers::NONE).as_deref(),
        Some(first.as_str()),
        "and comes back into the document at its first pane"
    );
    assert_eq!(
        desk.press(Key::F6, Modifiers::SHIFT).as_deref(),
        Some(NAVIGATOR_CONTROL)
    );
    assert_eq!(
        desk.press(Key::F6, Modifiers::SHIFT),
        second,
        "Shift+F6 comes back into the document at its last pane"
    );
}

/// Tab onto the plot and the Menu key opens its menu, as Shift+F10 and a
/// right click do: the Menu key did nothing on a plot before.
#[test]
fn the_menu_key_opens_the_focused_plots_menu() {
    let mut desk = Desk::new();
    desk.focus_plot();
    assert!(!egui::Popup::is_any_open(&desk.ctx));
    rspice_ui_kit::input::press_menu_key(&desk.ctx);
    desk.frame(Vec::new());
    desk.frame(Vec::new());
    assert!(
        egui::Popup::is_any_open(&desk.ctx),
        "the Menu key opened no menu on the focused plot"
    );
}

/// X and Y, which arm the next box zoom to change one axis alone, are
/// Results commands, listed with their keys, so a rebinding moves them: with
/// the plot focused, X arms it, as the plot states, and X again disarms it;
/// bound to Shift+X instead, X arms nothing and Shift+X arms it. The plot
/// answered X and Y itself before, where no list or rebinding reached them.
#[test]
fn the_one_axis_zoom_keys_are_listed_commands_a_rebinding_moves() {
    let registry = command_registry();
    let defaults = ShortcutPreferences::default();
    for (axis, key) in [(OneAxisZoom::X, Key::X), (OneAxisZoom::Y, Key::Y)] {
        let command = ResultsCommand::OneAxisZoom(axis).key();
        assert!(
            registry
                .ordered()
                .any(|spec| spec.key == command
                    && spec.flags.contains(CommandFlags::PALETTE_VISIBLE)),
            "{axis:?} is not listed"
        );
        assert!(
            defaults
                .effective_bindings(&CommandRegistry, command)
                .iter()
                .any(|binding| binding.supports(CommandPlatform::Desktop)
                    && binding.sequence().strokes()
                        == [ShortcutStroke::new(key, false, false, false)]),
            "{axis:?} is not on {key:?}"
        );
    }

    let mut desk = Desk::new();
    desk.focus_plot();
    assert!(!desk.zoom_x_armed());
    desk.press(Key::X, Modifiers::NONE);
    assert!(desk.zoom_x_armed(), "X armed no zoom on the focused plot");
    desk.press(Key::X, Modifiers::NONE);
    assert!(!desk.zoom_x_armed(), "X again left the zoom armed");

    desk.app
        .state
        .model
        .preferences
        .shortcuts_mut()
        .expect("the shortcut profile is editable")
        .set_binding(
            &CommandRegistry,
            ResultsCommand::OneAxisZoom(OneAxisZoom::X).key(),
            ShortcutBindingSlot::Primary,
            vec![CommandPlatform::Desktop],
            Some(
                ShortcutSequence::new(vec![ShortcutStroke::new(Key::X, false, false, true)])
                    .expect("one stroke"),
            ),
        )
        .expect("Shift+X is free in Results");
    desk.press(Key::X, Modifiers::NONE);
    assert!(
        !desk.zoom_x_armed(),
        "X still arms the zoom it was moved off"
    );
    desk.press(Key::X, Modifiers::SHIFT);
    assert!(
        desk.zoom_x_armed(),
        "Shift+X, the zoom's new key, armed nothing"
    );
}
