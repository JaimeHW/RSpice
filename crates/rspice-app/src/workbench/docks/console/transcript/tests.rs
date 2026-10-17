//! The Console page's contract: what it lays out, what a reader can select
//! and copy, what is announced, and where the notice centre lands.

use super::*;
use crate::diagnostics::{LogBuffer, LogSource};
use crate::state::SimulationRun;
use crate::ui::widgets::NotificationAction;
use crate::workbench::RSpiceApp;
use crate::workbench::state::ConsolePage;
use rspice_app_types::run_log::{RunLog, RunLogLine};

const SIZE: egui::Vec2 = egui::vec2(900.0, 600.0);

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx
}

fn input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, SIZE)),
        events,
        ..Default::default()
    }
}

fn frame(ctx: &egui::Context, state: &mut AppState, events: Vec<egui::Event>) -> egui::FullOutput {
    ctx.run_ui(input(events), |ctx| {
        egui::CentralPanel::default().show(ctx, |ui| console(ui, state));
    })
}

fn key(key: Key, modifiers: Modifiers) -> egui::Event {
    egui::Event::Key {
        key,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers,
    }
}

/// Ctrl as the desktop reports it: the control key, which is also the
/// platform's command key.
fn ctrl() -> Modifiers {
    Modifiers {
        ctrl: true,
        command: true,
        ..Modifiers::default()
    }
}

fn copied(output: &egui::FullOutput) -> Option<String> {
    output
        .platform_output
        .commands
        .iter()
        .find_map(|command| match command {
            egui::OutputCommand::CopyText(text) => Some(text.clone()),
            _ => None,
        })
}

/// A session whose log holds one line of each kind a reader meets.
fn session() -> AppState {
    let mut state = AppState::default();
    state.log_buffer.clear();
    state
        .log_buffer
        .log(LogSeverity::Info, LogSource::Studio, "Project opened");
    state.log_buffer.log(
        LogSeverity::Warning,
        LogSource::Netlist,
        "gmin stepping was needed",
    );
    state.log_buffer.log(
        LogSeverity::Error,
        LogSource::Studio,
        "Export failed: disk full",
    );
    state
}

/// Give the log the keyboard, as Tab would, and let the focus land.
fn focus_log(ctx: &egui::Context, state: &mut AppState) {
    let _ = frame(ctx, state, Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(body_id()));
    let _ = frame(ctx, state, Vec::new());
}

/// Ten thousand lines cost what fits on the screen: the body lays out the
/// rows in view, and the rest are heights.
#[test]
fn a_ten_thousand_line_session_lays_out_only_the_rows_in_view() {
    let mut state = AppState::default();
    state.log_buffer.clear();
    for index in 0..10_000 {
        state.log_buffer.log(
            LogSeverity::Info,
            LogSource::Studio,
            format!("Studio event {index}"),
        );
    }
    let ctx = context();
    for _ in 0..3 {
        let _ = frame(&ctx, &mut state, Vec::new());
    }
    // The galley cache keeps what the last two frames laid out. A body
    // 586 px tall shows under forty 16 px rows, each a message and a time.
    let laid_out = ctx.fonts(|fonts| fonts.num_galleys_in_cache());
    assert!(
        (10..200).contains(&laid_out),
        "{laid_out} galleys laid out for a body that shows under forty rows"
    );
}

/// The time column is as wide as the font sets `00:00:00`, which every time
/// of day fits, rather than a width somebody typed.
#[test]
fn the_time_column_is_measured_from_the_font_and_fits_any_time() {
    let ctx = context();
    let _ = ctx.run_ui(input(Vec::new()), |ui| {
        let columns = Columns::measure(ui, 900.0);
        for time in ["00:00:00", "23:59:59", "18:48:38"] {
            let width = ui
                .fonts_mut(|fonts| {
                    fonts.layout_no_wrap(time.to_owned(), row_font(), egui::Color32::PLACEHOLDER)
                })
                .size()
                .x;
            assert!(
                CONSOLE_MARK_GUTTER + width + CONSOLE_COLUMN_GAP <= columns.message + 0.01,
                "{time} is {width} px and runs into the message at {}",
                columns.message
            );
        }
    });
}

/// A long path wraps at the message column and every row is set below the
/// one before it, however many lines it took.
#[test]
fn wrapped_rows_never_overlap() {
    let mut state = session();
    // A 300-character path.
    let path = format!("C:/projects/{}/netlist.cir", "deep/".repeat(55));
    state.log_buffer.log(
        LogSeverity::Error,
        LogSource::Netlist,
        format!("Could not read {path}"),
    );
    state
        .log_buffer
        .log(LogSeverity::Info, LogSource::Studio, "Project saved");
    let ctx = context();
    ctx.enable_accesskit();
    let mut output = frame(&ctx, &mut state, Vec::new());
    for _ in 0..2 {
        output = frame(&ctx, &mut state, Vec::new());
    }
    let mut rows: Vec<Rect> = output
        .platform_output
        .accesskit_update
        .expect("accesskit is on")
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::ListBoxOption)
        .filter_map(|(_, node)| node.bounds())
        .map(|bounds| {
            Rect::from_min_max(
                pos2(bounds.x0 as f32, bounds.y0 as f32),
                pos2(bounds.x1 as f32, bounds.y1 as f32),
            )
        })
        .collect();
    rows.sort_by(|a, b| a.top().total_cmp(&b.top()));
    assert_eq!(rows.len(), 5, "{rows:?}");
    assert!(
        rows.iter()
            .any(|row| row.height() > 2.0 * CONSOLE_ROW_MIN_HEIGHT),
        "the path wraps rather than running off the row: {rows:?}"
    );
    for pair in rows.windows(2) {
        assert!(
            pair[0].bottom() <= pair[1].top() + 0.01,
            "{:?} overlaps {:?}",
            pair[0],
            pair[1]
        );
    }
}

/// Each row is announced as a reader would say it: severity, time, message.
#[test]
fn a_row_is_announced_as_severity_time_and_message() {
    let mut state = session();
    let ctx = context();
    ctx.enable_accesskit();
    let _ = frame(&ctx, &mut state, Vec::new());
    let output = frame(&ctx, &mut state, Vec::new());
    let labels: Vec<String> = output
        .platform_output
        .accesskit_update
        .expect("accesskit is on")
        .nodes
        .iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::ListBoxOption)
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect();
    let warning = labels
        .iter()
        .find(|label| label.ends_with("gmin stepping was needed"))
        .unwrap_or_else(|| panic!("no row announced the warning: {labels:?}"));
    let time = warning
        .strip_prefix("Warning, ")
        .and_then(|rest| rest.strip_suffix(", gmin stepping was needed"))
        .unwrap_or_else(|| panic!("{warning}"));
    assert_eq!(time.len(), 8, "HH:MM:SS: {time}");
    assert_eq!(time.matches(':').count(), 2, "{time}");
}

/// Shift with an arrow extends the selection, and Ctrl+C copies what is
/// selected as `HH:MM:SS  Severity  message` lines.
#[test]
fn shift_arrows_extend_the_selection_and_ctrl_c_copies_it() {
    let mut state = session();
    let ctx = context();
    let _ = frame(&ctx, &mut state, Vec::new());
    let first = state.log_buffer.entries().next().map(|entry| entry.id);
    let last = state.log_buffer.entries().last().map(|entry| entry.id);
    state.workbench.console_view.selection = first.map(ConsoleSelection::single);
    focus_log(&ctx, &mut state);
    let output = frame(
        &ctx,
        &mut state,
        vec![key(Key::ArrowDown, Modifiers::SHIFT), egui::Event::Copy],
    );
    assert_eq!(
        state.workbench.console_view.selection,
        first
            .zip(first.map(|key| key + 1))
            .map(|(anchor, cursor)| { ConsoleSelection { anchor, cursor } }),
        "Shift+Down extends the selection by one line"
    );
    let text = copied(&output).expect("Ctrl+C copied the selection");
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2, "{text}");
    assert!(lines[0].ends_with("  Info     Project opened"), "{text}");
    assert!(
        lines[1].ends_with("  Warning  gmin stepping was needed"),
        "{text}"
    );

    // Ctrl+A takes every shown line.
    let output = frame(
        &ctx,
        &mut state,
        vec![key(Key::A, ctrl()), egui::Event::Copy],
    );
    assert_eq!(
        state.workbench.console_view.selection,
        first
            .zip(last)
            .map(|(anchor, cursor)| ConsoleSelection { anchor, cursor })
    );
    assert_eq!(copied(&output).map(|text| text.lines().count()), Some(3));
}

/// The arrows move the selection, Home and End jump to either end, and
/// Escape lets go of the log.
#[test]
fn the_arrows_move_the_selection_and_escape_lets_go() {
    let mut state = session();
    let ctx = context();
    focus_log(&ctx, &mut state);
    let keys: Vec<u64> = state.log_buffer.entries().map(|entry| entry.id).collect();

    let _ = frame(&ctx, &mut state, vec![key(Key::ArrowUp, Modifiers::NONE)]);
    assert_eq!(
        state.workbench.console_view.selection,
        Some(ConsoleSelection::single(keys[2])),
        "with nothing selected, the arrows start at the newest line"
    );
    let _ = frame(&ctx, &mut state, vec![key(Key::ArrowUp, Modifiers::NONE)]);
    assert_eq!(
        state.workbench.console_view.selection,
        Some(ConsoleSelection::single(keys[1]))
    );
    let _ = frame(&ctx, &mut state, vec![key(Key::Home, Modifiers::NONE)]);
    assert_eq!(
        state.workbench.console_view.selection,
        Some(ConsoleSelection::single(keys[0]))
    );
    let _ = frame(&ctx, &mut state, vec![key(Key::End, Modifiers::NONE)]);
    assert_eq!(
        state.workbench.console_view.selection,
        Some(ConsoleSelection::single(keys[2]))
    );
    assert!(ctx.memory(|memory| memory.has_focus(body_id())));

    let _ = frame(&ctx, &mut state, vec![key(Key::Escape, Modifiers::NONE)]);
    assert!(!ctx.memory(|memory| memory.has_focus(body_id())));
    assert_eq!(state.workbench.console_view.selection, None);
}

/// Ctrl+F in the log goes to the find field, and Escape there clears the
/// find and gives the keyboard back to the log.
#[test]
fn ctrl_f_finds_and_escape_returns_to_the_log() {
    // The header band and the body, in the order the dock draws them.
    fn page(ctx: &egui::Context, state: &mut AppState, events: Vec<egui::Event>) {
        let _ = ctx.run_ui(input(events), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.horizontal(|ui| toolbar(ui, state));
                console(ui, state);
            });
        });
    }

    let mut state = session();
    let ctx = context();
    page(&ctx, &mut state, Vec::new());
    ctx.memory_mut(|memory| memory.request_focus(body_id()));
    page(&ctx, &mut state, Vec::new());
    page(&ctx, &mut state, vec![key(Key::F, ctrl())]);
    assert!(ctx.memory(|memory| memory.has_focus(find_id())));

    page(&ctx, &mut state, vec![egui::Event::Text("gmin".to_owned())]);
    assert_eq!(state.workbench.console_view.find, "gmin");
    page(&ctx, &mut state, vec![key(Key::Escape, Modifiers::NONE)]);
    assert!(state.workbench.console_view.find.is_empty());
    page(&ctx, &mut state, Vec::new());
    assert!(ctx.memory(|memory| memory.has_focus(body_id())));
}

/// Enter on a run's line opens that run's log; the session's channel is not
/// changed by anything but the reader.
#[test]
fn enter_on_a_run_line_opens_its_log() {
    let mut state = session();
    state.simulation.retained.runs.push(SimulationRun::new(5));
    let line = state.log_buffer.log_anchored(
        LogSeverity::Info,
        LogSource::Run,
        "Run 5 started: Transient",
        Some(LogAnchor::ResultRun { run_sequence: 5 }),
    );
    state.workbench.console_view.selection = Some(ConsoleSelection::single(line));
    let ctx = context();
    focus_log(&ctx, &mut state);
    let _ = frame(&ctx, &mut state, vec![key(Key::Enter, Modifiers::NONE)]);
    assert_eq!(state.workbench.console_view.channel, ConsoleChannel::Run(5));
    assert_eq!(state.workbench.console_view.selection, None);
}

/// A run's channel reads that run's own log, and a find narrows it the way
/// it narrows the session's.
#[test]
fn a_run_channel_shows_the_runs_own_lines() {
    let mut state = session();
    let mut run = SimulationRun::new(2);
    let mut log = RunLog::default();
    for (severity, message) in [
        (LogSeverity::Info, "Run 2 \u{b7} RSpice"),
        (LogSeverity::Warning, "timestep too small at 1.2 us"),
        (LogSeverity::Info, "Transient: 0.42 s"),
    ] {
        log.push(RunLogLine {
            unix_ms: 0,
            severity,
            source: LogSource::Run,
            message: message.to_owned(),
        });
    }
    run.log = log;
    state.simulation.retained.runs.push(run);
    state.workbench.console_view.open(ConsoleChannel::Run(2));
    let ctx = context();
    let mut text = String::new();
    for _ in 0..2 {
        text = painted(&frame(&ctx, &mut state, Vec::new()));
    }
    assert!(text.contains("timestep too small at 1.2 us"), "{text}");
    assert!(!text.contains("Project opened"), "{text}");

    state.workbench.console_view.find = "TRANSIENT".to_owned();
    let text = painted(&frame(&ctx, &mut state, Vec::new()));
    assert!(text.contains("Transient: 0.42 s"), "{text}");
    assert!(!text.contains("timestep"), "{text}");

    // A run the session drops takes its channel with it.
    state.simulation.retained.runs.clear();
    let _ = frame(&ctx, &mut state, Vec::new());
    assert_eq!(
        state.workbench.console_view.channel,
        ConsoleChannel::Session
    );
}

/// The notice centre's offer opens the session's log on the line the notice
/// was lifted from, selected and shown whatever hid it; a line the ring has
/// since dropped opens the log at its end.
#[test]
fn show_in_console_selects_the_line_the_notice_came_from() {
    let mut app = RSpiceApp::test_instance();
    app.state.log_buffer.clear();
    app.state
        .log_buffer
        .log(LogSeverity::Info, LogSource::Studio, "Project opened");
    let failure = app
        .state
        .log_buffer
        .log(LogSeverity::Error, LogSource::Studio, "Export failed");
    app.state.workbench.console_page = ConsolePage::Problems;
    app.state.workbench.console_view.find = "opened".to_owned();
    app.state
        .workbench
        .console_view
        .toggle(crate::workbench::state::ConsoleSeverity::Errors);

    crate::workbench::commands::result_navigation::perform_notification_action(
        &mut app,
        NotificationAction::ShowInConsole { log_id: failure },
    );
    let view = &app.state.workbench.console_view;
    assert!(app.state.workbench.console_visible);
    assert_eq!(app.state.workbench.console_page, ConsolePage::Console);
    assert_eq!(view.channel, ConsoleChannel::Session);
    assert_eq!(view.selection, Some(ConsoleSelection::single(failure)));
    assert_eq!(view.reveal, Some(ConsoleReveal::Line(failure)));
    assert!(
        view.find.is_empty(),
        "nothing the reader set hides the line"
    );
    assert!(view.shows(crate::workbench::state::ConsoleSeverity::Errors));

    crate::workbench::commands::result_navigation::perform_notification_action(
        &mut app,
        NotificationAction::ShowInConsole { log_id: 9_999 },
    );
    let view = &app.state.workbench.console_view;
    assert_eq!(view.selection, None);
    assert_eq!(view.reveal, Some(ConsoleReveal::End));
}

/// Saved and copied lines carry the time, the severity word and the
/// message, and nothing the page draws around them.
#[test]
fn a_saved_log_is_the_shown_lines_as_text() {
    let mut log = LogBuffer::new(10);
    log.log(LogSeverity::Error, LogSource::Studio, "Export failed");
    log.log(LogSeverity::Debug, LogSource::Studio, "cache warm");
    let mut rows = Rows::default();
    rows.sync(Source::Session(&log), &ConsoleView::default());
    let text = copy_of(Source::Session(&log), &rows, |_| true);
    let lines: Vec<&str> = text.lines().collect();
    assert_eq!(lines.len(), 2);
    assert!(lines[0].ends_with("  Error    Export failed"), "{text}");
    assert!(lines[1].ends_with("  Debug    cache warm"), "{text}");
    assert_eq!(file_name(ConsoleChannel::Run(12)), "run-12.log");
    assert_eq!(file_name(ConsoleChannel::Session), "session.log");
}

fn painted(output: &egui::FullOutput) -> String {
    fn collect(shape: &egui::epaint::Shape, text: &mut String) {
        match shape {
            egui::epaint::Shape::Text(shape) => {
                text.push_str(&shape.galley.job.text);
                text.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, text);
                }
            }
            _ => {}
        }
    }
    let mut text = String::new();
    for clipped in &output.shapes {
        collect(&clipped.shape, &mut text);
    }
    text
}
