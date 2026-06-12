//! Netlist workspace — the text-first editing loop.
//!
//! The netlist-editor design (design/app/volta-netlist-editor.html): an
//! editable SPICE buffer with gutter diagnostics and diff pips, a
//! `.param` tuner in the right panel that re-simulates on drag, and a
//! run bar whose delta chips compare the latest run's measurements
//! against the previous baseline. Editing switches the deck to *manual
//! source* mode — runs then execute the buffer instead of regenerating
//! from the schematic; Regenerate returns to the generated artifact.

use std::collections::{HashMap, HashSet};

use egui::Ui;

use crate::common::AppState;
use crate::properties::engineering::format_engineering_value;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, crumb_text, docbar};

mod completion;
mod editor;
mod highlight;
mod tuner;

pub use tuner::right_panel;

/// One parse diagnostic (the parser reports the first error it hits).
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 0-based buffer line, when the parser localizes the error.
    pub line: Option<usize>,
    /// Human-readable message.
    pub message: String,
}

/// Transient editor state: diagnostics, diff pips, tuner mode.
#[derive(Debug, Clone, Default)]
pub struct NetlistEditorState {
    /// Buffer revision, bumped on every edit.
    pub revision: u64,
    /// Revision the diagnostics were parsed for.
    diag_revision: Option<u64>,
    /// `ui.input(..).time` of the last edit, for the parse debounce.
    last_edit_time: f64,
    /// Current parse diagnostics (empty = deck parses clean).
    pub diagnostics: Vec<Diagnostic>,
    /// 0-based lines edited since the last completed run (diff pips).
    pub edited_lines: HashSet<usize>,
    /// `simulation.data_version` that last baselined the diff pips.
    seen_data_version: u64,
    /// Cursor line from the previous frame (drives the scope crumb).
    pub cursor_line: usize,
    /// Tuner re-simulates on every slider movement instead of on release.
    pub tuner_live: bool,
    /// A re-run was requested while the engine was busy.
    pub rerun_queued: bool,
    /// Slider ranges captured when a parameter first appears, so the
    /// range doesn't chase the value while dragging.
    pub param_ranges: HashMap<String, (f64, f64)>,
    /// The completion popover was open last frame (keys pre-consumed).
    pub completion_open: bool,
    /// Selected row in the completion popover.
    pub completion_index: usize,
    /// Buffer revision at which Esc dismissed the popover.
    pub completion_dismissed_at: Option<u64>,
    /// `.model` / `.subckt` symbols harvested from the last clean parse.
    pub symbols: Vec<completion::SymbolEntry>,
}

/// Whether the buffer (not the schematic) is the simulation source.
pub fn is_manual(state: &AppState) -> bool {
    state.workspace.netlist_source.is_some()
}

/// Render the netlist workspace center view.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    sync_run_baseline(state);
    flush_queued_run(state);

    // Ensure the buffer exists: manual source wins, otherwise generate.
    if let Some(source) = &state.workspace.netlist_source {
        if state.simulation.netlist_content.is_empty() {
            state.simulation.netlist_content = source.clone();
        }
    } else if state.simulation.netlist_content.is_empty() {
        regenerate(state);
    }

    show_docbar(ui, state);
    editor::show(ui, state);
}

fn show_docbar(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let manual = is_manual(state);
    let scope = subckt_scope(
        &state.simulation.netlist_content,
        state.shell.netlist.cursor_line,
    );

    docbar(ui, |ui| {
        let reference = &state.workspace.active_view;
        let mut crumbs: Vec<(&str, bool)> = vec![
            (reference.library.as_str(), false),
            (reference.cell.as_str(), true),
            ("netlist", false),
        ];
        if let Some(scope) = &scope {
            crumbs.push((scope.as_str(), false));
        }
        crumb_text(ui, &crumbs);
        let (badge, color) = if manual {
            ("· manual source", t.color.accent)
        } else {
            ("· generated from schematic", t.color.text_faint)
        };
        ui.label(
            egui::RichText::new(badge)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(color),
        );

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            let running = state.simulation.is_running;
            let run = Button::new(if running { "Running…" } else { "Run" })
                .accent()
                .hint("F5")
                .enabled(!running)
                .show(ui);
            if run.clicked() {
                request_run(state);
            }

            let regen_label = if manual {
                "Regenerate (discard edits)"
            } else {
                "Regenerate"
            };
            if Button::new(regen_label).show(ui).clicked() {
                state.workspace.netlist_source = None;
                regenerate(state);
                let netlist = &mut state.shell.netlist;
                netlist.edited_lines.clear();
                netlist.diagnostics.clear();
                netlist.diag_revision = None;
                netlist.revision += 1;
            }
            if Button::new("Copy").show(ui).clicked() {
                ui.ctx()
                    .copy_text(state.simulation.netlist_content.clone());
                state
                    .shell
                    .toasts
                    .info(ui.ctx(), "Netlist copied to clipboard");
            }

            delta_chips(ui, state);
        });
    });
}

/// Delta chips: latest run's measurements against the previous run.
/// Good/bad coloring follows the spec direction, not the sign.
fn delta_chips(ui: &mut Ui, state: &AppState) {
    let runs = &state.simulation.runs;
    if runs.len() < 2 {
        return;
    }
    let latest = run_measurements(&runs[runs.len() - 1]);
    let previous = run_measurements(&runs[runs.len() - 2]);
    if latest.is_empty() || previous.is_empty() {
        return;
    }

    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let mut names: Vec<&String> = latest.keys().collect();
    names.sort();

    const MAX_CHIPS: usize = 4;
    let mut shown = 0usize;
    for key in names {
        if shown == MAX_CHIPS {
            break;
        }
        let (name, new) = &latest[key];
        let Some((_, old)) = previous.get(key) else {
            continue;
        };
        if old == new {
            continue;
        }

        let color = match delta_verdict(state, name, *old, *new) {
            Some(true) => c.ok,
            Some(false) => c.err,
            None => c.text_dim,
        };
        let text = format!(
            "{} {} → {}",
            name,
            format_engineering_value(*old),
            format_engineering_value(*new)
        );
        ui.label(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        )
        .on_hover_text("Latest run vs previous baseline");
        shown += 1;
    }
}

/// Improvement verdict for a measurement delta against its spec:
/// `Some(true)` better, `Some(false)` worse, `None` no spec / no change.
fn delta_verdict(state: &AppState, name: &str, old: f64, new: f64) -> Option<bool> {
    let spec = state
        .workspace
        .specs
        .iter()
        .find(|spec| spec.measurement.eq_ignore_ascii_case(name))?;
    let (old_v, new_v) = (spec.violation(old), spec.violation(new));
    if old_v == new_v {
        // Both inside the bounds: distance to the nearest bound decides.
        return None;
    }
    Some(new_v < old_v)
}

/// Latest value per measurement name in one run (lowercase key).
fn run_measurements(run: &crate::state::SimulationRun) -> HashMap<String, (String, f64)> {
    let mut out = HashMap::new();
    for analysis in &run.analyses {
        for m in &analysis.measurements {
            if let Some(value) = m.value {
                out.insert(m.name.to_ascii_lowercase(), (m.name.clone(), value));
            }
        }
    }
    out
}

/// Innermost `.subckt` scope containing the cursor line, if any.
fn subckt_scope(buffer: &str, cursor_line: usize) -> Option<String> {
    let mut stack: Vec<&str> = Vec::new();
    for (idx, line) in buffer.lines().enumerate() {
        if idx > cursor_line {
            break;
        }
        let trimmed = line.trim_start();
        if let Some(rest) = strip_dot_command(trimmed, ".subckt") {
            if let Some(name) = rest.split_whitespace().next() {
                stack.push(name);
            }
        } else if strip_dot_command(trimmed, ".ends").is_some() {
            stack.pop();
        }
    }
    stack.last().map(|name| (*name).to_owned())
}

fn strip_dot_command<'a>(line: &'a str, command: &str) -> Option<&'a str> {
    if line.len() >= command.len() && line[..command.len()].eq_ignore_ascii_case(command) {
        let rest = &line[command.len()..];
        if rest.is_empty() || rest.starts_with(char::is_whitespace) {
            return Some(rest);
        }
    }
    None
}

/// Request a simulation run; queues it when the engine is busy. An empty
/// run set drops the request — the tuner's live loop must not spam the
/// controller's empty-set warning on every slider move.
pub(super) fn request_run(state: &mut AppState) {
    if state.sim_setup.enabled.is_empty() {
        return;
    }
    if state.simulation.is_running {
        state.shell.netlist.rerun_queued = true;
    } else {
        state.simulation.trigger_simulation = true;
    }
}

/// Fire a queued re-run once the engine is idle (live-tuning loop).
fn flush_queued_run(state: &mut AppState) {
    if state.shell.netlist.rerun_queued
        && !state.simulation.is_running
        && !state.sim_setup.enabled.is_empty()
    {
        state.shell.netlist.rerun_queued = false;
        state.simulation.trigger_simulation = true;
    }
}

/// Clear the diff pips when a run lands — the run baselines the edits.
fn sync_run_baseline(state: &mut AppState) {
    let version = state.simulation.data_version;
    let netlist = &mut state.shell.netlist;
    if netlist.seen_data_version != version {
        netlist.seen_data_version = version;
        netlist.edited_lines.clear();
    }
}

/// Regenerate the buffer from the schematic (generated mode).
fn regenerate(state: &mut AppState) {
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_buffers(
        &state.workspace.schematic_buffers,
    );
    let result = crate::simulation::netlist_gen::generate_netlist_hierarchical(
        &state.schematic,
        &[],
        &hierarchy,
    );
    state.simulation.netlist_content = result.netlist;
    for warning in &result.warnings {
        state.push_console_message_with_source(
            crate::panels::LogSource::Netlist,
            crate::common::app::ConsoleMessage::warning(warning.clone()),
        );
    }
}
