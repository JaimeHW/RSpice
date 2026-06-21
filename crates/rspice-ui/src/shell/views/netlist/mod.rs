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
use crate::ui::icons::Icon;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Button, IconButton, crumb_text, docbar};

mod baseline;
mod completion;
pub mod diagnostics;
mod editor;
mod highlight;
mod summary;
mod tuner;

pub use diagnostics::{Diagnostic, DiagnosticSeverity};
pub use tuner::right_panel;

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
    /// Exact editor buffer from the last successful manual deck run.
    pub last_run_buffer: Option<String>,
    /// Numeric `.param` values captured from `last_run_buffer`.
    pub last_run_params: HashMap<String, f64>,
    /// Editor buffer captured when the current manual deck run started.
    pub pending_run_buffer: Option<String>,
    /// Run id associated with `pending_run_buffer`.
    pub pending_manual_run_id: Option<u64>,
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

const NETLIST_DOCBAR_HORIZONTAL_INSET: f32 = 20.0;
const NETLIST_DOCBAR_SPACING: f32 = 8.0;
const NETLIST_DOCBAR_TEXT_CHAR_WIDTH: f32 = 7.0;
const NETLIST_DOCBAR_BADGE_WIDTH: f32 = 96.0;
const NETLIST_DOCBAR_COMPACT_BADGE_WIDTH: f32 = 64.0;
const NETLIST_DOCBAR_COPY_WIDTH: f32 = 58.0;
const NETLIST_DOCBAR_REGEN_WIDTH: f32 = 96.0;
const NETLIST_DOCBAR_REGEN_MANUAL_WIDTH: f32 = 176.0;
const NETLIST_DOCBAR_REGEN_COMPACT_WIDTH: f32 = 72.0;
const NETLIST_DOCBAR_RUN_WIDTH: f32 = 64.0;
const NETLIST_DOCBAR_RUNNING_WIDTH: f32 = 92.0;
const NETLIST_DOCBAR_PROGRESS_WIDTH: f32 = 190.0;
const NETLIST_DOCBAR_DELTA_WIDTH: f32 = 122.0;
const NETLIST_DOCBAR_ICON_BUTTON_WIDTH: f32 = 28.0;
const NETLIST_MAX_DELTA_CHIPS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NetlistDocbarPresentation {
    Full,
    Compact,
    Minimal,
    Iconic,
}

#[derive(Debug, Clone, Copy)]
struct NetlistDocbarInputs {
    manual: bool,
    running: bool,
    delta_count: usize,
    crumb_chars: usize,
    scope_chars: usize,
}

#[derive(Debug, Clone, Default)]
struct NetlistDocbarDeltas {
    measurements: Vec<summary::MeasurementDelta>,
    bode: Vec<summary::BodeDelta>,
}

fn netlist_docbar_content_width(available_width: f32) -> f32 {
    (available_width - NETLIST_DOCBAR_HORIZONTAL_INSET).max(0.0)
}

fn netlist_docbar_presentation(
    available_width: f32,
    inputs: NetlistDocbarInputs,
) -> NetlistDocbarPresentation {
    let content_width = netlist_docbar_content_width(available_width);
    for presentation in [
        NetlistDocbarPresentation::Full,
        NetlistDocbarPresentation::Compact,
        NetlistDocbarPresentation::Minimal,
    ] {
        if netlist_docbar_estimated_width(
            presentation,
            inputs.manual,
            inputs.running,
            inputs.delta_count,
            inputs.crumb_chars,
            inputs.scope_chars,
        ) <= content_width
        {
            return presentation;
        }
    }
    NetlistDocbarPresentation::Iconic
}

fn netlist_docbar_estimated_width(
    presentation: NetlistDocbarPresentation,
    manual: bool,
    running: bool,
    delta_count: usize,
    crumb_chars: usize,
    scope_chars: usize,
) -> f32 {
    let mut widths = Vec::new();
    match presentation {
        NetlistDocbarPresentation::Full => {
            let crumb_width =
                (crumb_chars + scope_chars).max(1) as f32 * NETLIST_DOCBAR_TEXT_CHAR_WIDTH + 24.0;
            widths.push(crumb_width);
            widths.push(NETLIST_DOCBAR_BADGE_WIDTH);
            widths.push(NETLIST_DOCBAR_COPY_WIDTH);
            widths.push(if manual {
                NETLIST_DOCBAR_REGEN_MANUAL_WIDTH
            } else {
                NETLIST_DOCBAR_REGEN_WIDTH
            });
            if running {
                widths.push(NETLIST_DOCBAR_PROGRESS_WIDTH);
            }
            widths.push(netlist_docbar_run_width(running));
            let delta_width_count = delta_count.min(NETLIST_MAX_DELTA_CHIPS);
            widths.resize(widths.len() + delta_width_count, NETLIST_DOCBAR_DELTA_WIDTH);
        }
        NetlistDocbarPresentation::Compact => {
            widths.push(NETLIST_DOCBAR_COMPACT_BADGE_WIDTH);
            widths.push(NETLIST_DOCBAR_COPY_WIDTH);
            widths.push(NETLIST_DOCBAR_REGEN_COMPACT_WIDTH);
            widths.push(netlist_docbar_run_width(running));
        }
        NetlistDocbarPresentation::Minimal => {
            widths.push(NETLIST_DOCBAR_REGEN_COMPACT_WIDTH);
            widths.push(netlist_docbar_run_width(running));
        }
        NetlistDocbarPresentation::Iconic => {
            widths.push(NETLIST_DOCBAR_ICON_BUTTON_WIDTH);
            widths.push(NETLIST_DOCBAR_ICON_BUTTON_WIDTH);
        }
    }

    let spacing = widths.len().saturating_sub(1) as f32 * NETLIST_DOCBAR_SPACING;
    widths.iter().sum::<f32>() + spacing
}

fn netlist_docbar_run_width(running: bool) -> f32 {
    if running {
        NETLIST_DOCBAR_RUNNING_WIDTH
    } else {
        NETLIST_DOCBAR_RUN_WIDTH
    }
}

fn netlist_docbar_regen_label(
    presentation: NetlistDocbarPresentation,
    manual: bool,
) -> &'static str {
    match presentation {
        NetlistDocbarPresentation::Full if manual => "Regenerate (discard edits)",
        NetlistDocbarPresentation::Full => "Regenerate",
        NetlistDocbarPresentation::Compact
        | NetlistDocbarPresentation::Minimal
        | NetlistDocbarPresentation::Iconic => "Regen",
    }
}

fn netlist_docbar_show_copy(presentation: NetlistDocbarPresentation) -> bool {
    matches!(
        presentation,
        NetlistDocbarPresentation::Full | NetlistDocbarPresentation::Compact
    )
}

fn netlist_docbar_show_deltas(presentation: NetlistDocbarPresentation) -> bool {
    matches!(presentation, NetlistDocbarPresentation::Full)
}

fn netlist_docbar_show_progress(presentation: NetlistDocbarPresentation) -> bool {
    matches!(presentation, NetlistDocbarPresentation::Full)
}

fn netlist_docbar_compact_badge(manual: bool) -> &'static str {
    if manual { "manual" } else { "generated" }
}

fn netlist_docbar_deltas(state: &AppState) -> NetlistDocbarDeltas {
    let bode = summary::bode_deltas(state, NETLIST_MAX_DELTA_CHIPS.min(2));
    let measurement_limit = NETLIST_MAX_DELTA_CHIPS.saturating_sub(bode.len());
    let measurements = summary::measurement_deltas(state, measurement_limit);
    NetlistDocbarDeltas { measurements, bode }
}

fn netlist_docbar_delta_count(deltas: &NetlistDocbarDeltas) -> usize {
    deltas.measurements.len() + deltas.bode.len()
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
    let crumb_chars = state.workspace.active_view.library.len()
        + state.workspace.active_view.cell.len()
        + "netlist".len();
    let scope_chars = scope.as_ref().map_or(0, |scope| scope.len());
    let deltas = netlist_docbar_deltas(state);
    let presentation = netlist_docbar_presentation(
        ui.available_width(),
        NetlistDocbarInputs {
            manual,
            running: state.simulation.is_running,
            delta_count: netlist_docbar_delta_count(&deltas),
            crumb_chars,
            scope_chars,
        },
    );

    docbar(ui, |ui| {
        if matches!(presentation, NetlistDocbarPresentation::Full) {
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
        } else if matches!(presentation, NetlistDocbarPresentation::Compact) {
            let color = if manual {
                t.color.accent
            } else {
                t.color.text_faint
            };
            ui.label(
                egui::RichText::new(netlist_docbar_compact_badge(manual))
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(color),
            );
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            let running = state.simulation.is_running;
            let run_block_reason = state.manual_deck_run_block_reason();
            let can_run = run_block_reason.is_none();
            if matches!(presentation, NetlistDocbarPresentation::Iconic) {
                let run_tip = run_block_reason.as_deref().unwrap_or(if running {
                    "Queue rerun (F5)"
                } else {
                    "Run (F5)"
                });
                let run = IconButton::new(Icon::Run)
                    .enabled(can_run)
                    .tooltip(run_tip)
                    .show(ui);
                if run.clicked() {
                    request_run(state);
                }

                let regen_tip = if manual {
                    "Regenerate from schematic (discard edits)"
                } else {
                    "Regenerate from schematic"
                };
                if IconButton::new(Icon::Redo)
                    .tooltip(regen_tip)
                    .show(ui)
                    .clicked()
                {
                    regenerate_from_docbar(state);
                }

                return;
            }

            let run = Button::new(if running { "Running…" } else { "Run" })
                .accent()
                .hint("F5")
                .enabled(can_run)
                .show(ui);
            let run_clicked = run.clicked();
            if let Some(reason) = run_block_reason.as_deref() {
                run.on_hover_text(reason);
            }
            if run_clicked {
                request_run(state);
            }
            if netlist_docbar_show_progress(presentation) {
                run_progress(ui, state);
            }

            let regen_label = netlist_docbar_regen_label(presentation, manual);
            if Button::new(regen_label).show(ui).clicked() {
                regenerate_from_docbar(state);
            }
            if netlist_docbar_show_copy(presentation) && Button::new("Copy").show(ui).clicked() {
                ui.ctx().copy_text(state.simulation.netlist_content.clone());
                state
                    .shell
                    .toasts
                    .info(ui.ctx(), "Netlist copied to clipboard");
            }

            if netlist_docbar_show_deltas(presentation) {
                delta_chips(ui, &deltas);
            }
        });
    });
}

fn regenerate_from_docbar(state: &mut AppState) {
    state.workspace.netlist_source = None;
    state.workspace.netlist_source_path = None;
    state.workspace.set_netlist_source_dirty(false);
    regenerate(state);
    let netlist = &mut state.shell.netlist;
    netlist.edited_lines.clear();
    netlist.diagnostics.clear();
    netlist.diag_revision = None;
    netlist.last_run_buffer = None;
    netlist.last_run_params.clear();
    netlist.pending_run_buffer = None;
    netlist.pending_manual_run_id = None;
    netlist.revision += 1;
}

fn run_progress(ui: &mut Ui, state: &AppState) {
    if !state.simulation.is_running {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        let (rect, response) = ui.allocate_exact_size(egui::vec2(120.0, 4.0), egui::Sense::hover());
        ui.painter().rect_filled(rect, 2.0, c.bg_inset);
        let progress = state.simulation.progress.clamp(0.0, 1.0) as f32;
        let fill = egui::Rect::from_min_max(
            rect.min,
            egui::pos2(
                rect.left() + rect.width() * progress.max(0.08),
                rect.bottom(),
            ),
        );
        ui.painter().rect_filled(fill, 2.0, c.accent);
        response.on_hover_text(progress_label(state));
        ui.label(
            egui::RichText::new(progress_label(state))
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(c.text_dim),
        );
    });
}

fn progress_label(state: &AppState) -> String {
    if state.simulation.status.is_empty() {
        "running".to_string()
    } else {
        state.simulation.status.clone()
    }
}

/// Delta chips: latest run's measurements against the previous run.
/// Good/bad coloring follows the spec direction, not the sign.
fn delta_chips(ui: &mut Ui, deltas: &NetlistDocbarDeltas) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    for delta in &deltas.measurements {
        let color = match delta.improved {
            Some(true) => c.ok,
            Some(false) => c.err,
            None => c.text_dim,
        };
        let text = format!(
            "{} {} → {}",
            delta.name,
            format_engineering_value(delta.old),
            format_engineering_value(delta.new)
        );
        ui.label(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        )
        .on_hover_text("Latest run vs previous baseline");
    }
    for delta in &deltas.bode {
        let color = match delta.improved {
            Some(true) => c.ok,
            Some(false) => c.err,
            None => c.text_dim,
        };
        let text = format!(
            "{} {} → {}",
            delta.name,
            format_bode_delta_value(delta.old, delta.unit),
            format_bode_delta_value(delta.new, delta.unit)
        );
        ui.label(
            egui::RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(color),
        )
        .on_hover_text("Latest AC stability summary vs previous baseline");
    }
}

fn format_bode_delta_value(value: f64, unit: &str) -> String {
    match unit {
        "dB" => format!("{value:.1} dB"),
        "deg" => format!("{value:.1} deg"),
        "Hz" => format!("{} Hz", format_engineering_value(value)),
        _ => format_engineering_value(value),
    }
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

/// Request a manual-deck simulation run; queues one rerun when the engine is busy.
pub(super) fn request_run(state: &mut AppState) {
    state.request_netlist_manual_deck_run();
}

/// Fire a queued re-run once the engine is idle (live-tuning loop).
fn flush_queued_run(state: &mut AppState) {
    if state.shell.netlist.rerun_queued && !state.simulation.is_running {
        state.shell.netlist.rerun_queued = false;
        state.simulation.request_manual_deck_run();
    }
}

/// Recompute diff pips against the last successful manual deck snapshot.
pub(super) fn refresh_diff_pips_from_baseline(state: &mut AppState) {
    let Some(baseline) = state.shell.netlist.last_run_buffer.as_deref() else {
        return;
    };
    state.shell.netlist.edited_lines =
        baseline::changed_lines_against_baseline(&state.simulation.netlist_content, baseline);
}

/// Compatibility fallback for older/generated result flows. Manual deck runs
/// promote their exact source snapshot in the simulation controller.
fn sync_run_baseline(state: &mut AppState) {
    let version = state.simulation.data_version;
    let netlist = &mut state.shell.netlist;
    if netlist.last_run_buffer.is_none() && netlist.seen_data_version != version {
        netlist.seen_data_version = version;
        netlist.edited_lines.clear();
    }
}

#[cfg(test)]
mod run_intent_tests {
    use super::*;
    use crate::state::SimulationRunIntent;

    #[test]
    fn netlist_request_run_sets_manual_deck_intent_without_enabled_run_set() {
        let mut state = AppState::default();
        state.sim_setup.enabled.clear();
        state.simulation.netlist_content = "deck\nR1 out 0 1k\nV1 out 0 1\n.op\n.end\n".to_string();
        state.workspace.netlist_source = Some(state.simulation.netlist_content.clone());

        request_run(&mut state);

        assert!(state.simulation.trigger_simulation);
        assert_eq!(state.simulation.run_intent, SimulationRunIntent::ManualDeck);
        assert!(!state.shell.netlist.rerun_queued);
    }

    #[test]
    fn netlist_request_run_queues_one_manual_deck_rerun_while_running() {
        let mut state = AppState::default();
        state.simulation.is_running = true;
        state.workspace.netlist_source = Some(".op\n.end\n".to_string());

        request_run(&mut state);
        request_run(&mut state);

        assert!(!state.simulation.trigger_simulation);
        assert_eq!(state.simulation.run_intent, SimulationRunIntent::ManualDeck);
        assert!(state.shell.netlist.rerun_queued);
    }

    #[test]
    fn regenerate_from_docbar_discards_manual_source_dirty_state() {
        let mut state = AppState::default();
        state.workspace.netlist_source = Some("manual deck\n.op\n.end\n".to_string());
        state.workspace.netlist_source_path = Some(std::path::PathBuf::from("manual.cir"));
        state.workspace.set_netlist_source_dirty(true);
        state.simulation.netlist_content = state.workspace.netlist_source.clone().unwrap();

        regenerate_from_docbar(&mut state);

        assert!(state.workspace.netlist_source.is_none());
        assert!(state.workspace.netlist_source_path.is_none());
        assert!(!state.workspace.netlist_source_dirty);
        assert!(!state.workspace.any_dirty());
    }

    #[test]
    fn progress_label_falls_back_to_running_when_status_is_empty() {
        let mut state = AppState::default();
        state.simulation.is_running = true;
        state.simulation.status.clear();

        assert_eq!(progress_label(&state), "running");

        state.simulation.status = "Transient Analysis".to_string();
        assert_eq!(progress_label(&state), "Transient Analysis");
    }

    #[test]
    fn phone_width_netlist_docbar_uses_compact_presentation_that_fits() {
        let presentation = netlist_docbar_presentation(
            390.0,
            NetlistDocbarInputs {
                manual: true,
                running: false,
                delta_count: 4,
                crumb_chars: 15,
                scope_chars: 0,
            },
        );

        assert_eq!(presentation, NetlistDocbarPresentation::Compact);
        assert!(
            netlist_docbar_estimated_width(presentation, true, false, 4, 15, 0)
                <= netlist_docbar_content_width(390.0)
        );
        assert_eq!(netlist_docbar_regen_label(presentation, true), "Regen");
        assert!(!netlist_docbar_show_deltas(presentation));
    }

    #[test]
    fn desktop_width_netlist_docbar_keeps_full_context_and_deltas() {
        let presentation = netlist_docbar_presentation(
            1280.0,
            NetlistDocbarInputs {
                manual: true,
                running: false,
                delta_count: 4,
                crumb_chars: 15,
                scope_chars: 0,
            },
        );

        assert_eq!(presentation, NetlistDocbarPresentation::Full);
        assert_eq!(
            netlist_docbar_regen_label(presentation, true),
            "Regenerate (discard edits)"
        );
        assert!(netlist_docbar_show_deltas(presentation));
    }

    #[test]
    fn very_narrow_netlist_docbar_keeps_primary_actions_inside_width() {
        let presentation = netlist_docbar_presentation(
            300.0,
            NetlistDocbarInputs {
                manual: true,
                running: true,
                delta_count: 4,
                crumb_chars: 24,
                scope_chars: 12,
            },
        );

        assert_eq!(presentation, NetlistDocbarPresentation::Minimal);
        assert!(
            netlist_docbar_estimated_width(presentation, true, true, 4, 24, 12)
                <= netlist_docbar_content_width(300.0)
        );
        assert!(!netlist_docbar_show_copy(presentation));
        assert!(!netlist_docbar_show_deltas(presentation));
    }

    #[test]
    fn constrained_mobile_netlist_docbar_uses_icon_actions_that_fit() {
        let presentation = netlist_docbar_presentation(
            180.0,
            NetlistDocbarInputs {
                manual: true,
                running: true,
                delta_count: 4,
                crumb_chars: 24,
                scope_chars: 12,
            },
        );

        assert_eq!(presentation, NetlistDocbarPresentation::Iconic);
        assert!(
            netlist_docbar_estimated_width(presentation, true, true, 4, 24, 12)
                <= netlist_docbar_content_width(180.0)
        );
        assert!(!netlist_docbar_show_copy(presentation));
        assert!(!netlist_docbar_show_deltas(presentation));
    }

    #[test]
    fn netlist_docbar_delta_count_uses_precomputed_delta_bundle() {
        let deltas = NetlistDocbarDeltas {
            measurements: vec![summary::MeasurementDelta {
                name: "vmax".to_string(),
                old: 1.0,
                new: 2.0,
                improved: None,
            }],
            bode: vec![
                summary::BodeDelta {
                    name: "PM",
                    unit: "deg",
                    old: 45.0,
                    new: 60.0,
                    improved: Some(true),
                },
                summary::BodeDelta {
                    name: "A_dc",
                    unit: "dB",
                    old: 0.0,
                    new: -1.0,
                    improved: None,
                },
            ],
        };

        assert_eq!(netlist_docbar_delta_count(&deltas), 3);
    }
}

/// Regenerate the buffer from the schematic (generated mode).
fn regenerate(state: &mut AppState) {
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
        &state.library_manager,
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
