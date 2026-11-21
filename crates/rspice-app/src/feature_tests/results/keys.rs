//! The Results keys: every Results command on a menu row with its key, no
//! key of Results shared with another command that acts there, and the keys
//! acting through the shortcut resolver with a plot focused by Tab.

use egui::{Context, Event, Key, Modifiers, RawInput, Rect};

use crate::workbench::RSpiceApp;
use crate::workbench::commands::vocabulary::CommandRegistry;
use rspice_app_results::workbench::results::commands::ResultsCommand;
use rspice_app_state::state::command_catalog::{CommandKey, CommandPlatform};
use rspice_app_state::state::shortcuts::{ShortcutPreferences, ShortcutProfileIssueCode};
use rspice_app_state::state::{
    AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::chrome::title_bar::application_menus::ALL_MENUS;
use rspice_app_workbench::workbench::chrome::title_bar::menu_entries::{self, MenuRow, menu_keys};
use rspice_app_workbench::workbench::framework::command_registry::WorkbenchCommand;
use rspice_app_workbench::workbench::framework::host::Host;

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
}

impl Desk {
    fn new() -> Self {
        let ctx = Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut app = RSpiceApp::test_instance();
        app.state.model.simulation.retained.runs = vec![peaked_run()].into();
        assert!(app.state.model.simulation.select_run(0));
        app.state.workbench.activate(Workspace::Results);
        let mut desk = Self {
            ctx,
            app,
            time: 0.0,
        };
        desk.frame(Vec::new());
        desk.frame(Vec::new());
        desk
    }

    /// Run one frame and return the label of the node the keyboard is on.
    fn frame(&mut self, events: Vec<Event>) -> Option<String> {
        self.time += 0.02;
        let app = &mut self.app;
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
                app.handle_shortcuts(ui.ctx());
                egui::CentralPanel::default().show(ui, |ui| {
                    super::with_surface_parts(app, |state, controller| {
                        rspice_app_results::workbench::documents::result_document::show(
                            ui, state, controller,
                        );
                    });
                });
            },
        );
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
