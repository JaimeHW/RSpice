//! Simulate-context side panels: run history + engine info (left), analysis
//! detail (right).

use egui::Ui;

use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{TreeRow, kv_row, section_header};

// ---------------------------------------------------------------------------
// Left panel
// ---------------------------------------------------------------------------

/// Render the simulate context's left panel.
pub fn left(ui: &mut Ui, state: &mut AppState) {
    run_history_section(ui, state);
    engine_section(ui, state);
}

fn run_history_section(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    if let Some(action) = section_header(ui, "Run history", Some("Clear"))
        && action.clicked()
    {
        state.simulation.clear_runs();
    }

    ui.scope(|ui| {
        ui.spacing_mut().item_spacing.y = 0.0;
        if state.simulation.runs.is_empty() {
            ui.add_space(4.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No runs yet — press Run (F5)")
                        .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                        .color(c.text_faint),
                );
            });
            ui.add_space(8.0);
            return;
        }

        let rows: Vec<(usize, String, String, bool)> = state
            .simulation
            .runs
            .iter()
            .enumerate()
            .map(|(idx, run)| {
                let label = format!("#{} · {}", run.id, state.shell.corner);
                let meta = if run.success {
                    format!("{:.1} s", run.elapsed_time)
                } else {
                    "failed".to_owned()
                };
                (idx, label, meta, run.success)
            })
            .collect();
        for (idx, label, meta, success) in rows {
            let selected = state.simulation.active_run_idx == Some(idx);
            let dot = if success { c.ok } else { c.err };
            let row = TreeRow::new(&label)
                .chip_dot(dot)
                .meta(&meta)
                .selected(selected)
                .show(ui);
            if row.response.clicked() {
                state.simulation.select_run(idx);
            }
        }
    });
}

fn engine_section(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Engine", None);
    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 0.0,
            bottom: 12.0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            kv_row(
                ui,
                "Solver",
                concat!("rspice-core ", env!("CARGO_PKG_VERSION")),
            );
            let threads = std::thread::available_parallelism()
                .map(std::num::NonZeroUsize::get)
                .unwrap_or(1);
            kv_row(ui, "Threads", &threads.to_string());
            kv_row(ui, "Matrix pkg", "faer sparse");
            let devices = state.schematic.components.len();
            let nets: std::collections::HashSet<&String> =
                state.schematic.net_mapping.values().collect();
            kv_row(ui, "Devices", &format!("{devices} · {} nets", nets.len()));
        });
}

// ---------------------------------------------------------------------------
// Right panel — analysis detail
// ---------------------------------------------------------------------------

/// Every analysis the runner knows, in run order. `(analysis index, mono
/// id, description)` — the index matches `SimSetupState::enabled`
/// semantics and the controller's analysis dispatch.
pub const ANALYSES: &[(usize, &str, &str)] = &[
    (0, "op", "DC operating point"),
    (1, "tran", "time-domain transient"),
    (2, "ac", "small-signal frequency sweep"),
    (3, "dc", "DC transfer sweep"),
    (4, "noise", "output-referred noise"),
    (5, "pz", "pole-zero extraction"),
    (6, "sens", "sensitivity"),
    (7, "mc", "Monte Carlo statistical"),
    (8, "pss", "periodic steady state"),
    (9, "stb", "loop stability"),
    (10, "temp", "temperature sweep"),
    (11, "hb", "harmonic balance"),
    (12, "sp", "S-parameters"),
    (13, "pac", "periodic AC"),
    (14, "pnoise", "periodic noise"),
    (15, "pxf", "periodic transfer"),
    (16, "pstb", "periodic stability"),
    (17, "xf", "transfer function"),
    (18, "corner", "process corners"),
    (19, "env", "envelope transient"),
    (20, "four", "Fourier components"),
    (21, "rel", "reliability / aging"),
    (22, "opt", "optimization"),
    (23, "soa", "safe operating area"),
    (24, "disto", "harmonic distortion"),
];

/// Mono id + description for an analysis index.
pub fn analysis_meta(index: usize) -> (&'static str, &'static str) {
    ANALYSES
        .iter()
        .find(|(idx, _, _)| *idx == index)
        .map_or(("?", ""), |(_, name, description)| (*name, *description))
}

/// Render the simulate context's right panel (selected analysis detail).
pub fn right(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let selected_tab = state.shell.selected_analysis.unwrap_or(1);
    let name = ANALYSES
        .iter()
        .find(|(tab, _, _)| *tab == selected_tab)
        .map_or("tran", |(_, name, _)| *name);

    section_header(ui, &format!("Analysis · {name}"), None);

    egui::Frame::none()
        .inner_margin(egui::Margin {
            left: 12.0,
            right: 12.0,
            top: 0.0,
            bottom: 12.0,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 2.0;
            let setup = &mut state.sim_setup;
            setup.ensure_initialized();

            let note = super::simulate_forms::form(ui, setup, selected_tab);

            // Inline validation — the same parse the controller runs.
            if let Some(error) = setup.validation_error(selected_tab) {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(error)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.err),
                );
            }

            if !note.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(note)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            }

            // The exact card this configuration writes into the deck —
            // live validation feedback, formerly the add-dialog's strip.
            ui.add_space(10.0);
            let y = ui.cursor().top();
            ui.painter().hline(
                ui.max_rect().x_range(),
                y - 0.5,
                egui::Stroke::new(1.0, c.border),
            );
            ui.add_space(7.0);
            let mut caption = egui::text::LayoutJob::default();
            caption.append(
                "WRITES",
                0.0,
                egui::TextFormat {
                    font_id: theme::mono(9.5, FontWeight::Medium),
                    color: c.text_faint,
                    extra_letter_spacing: 0.1 * 9.5,
                    ..Default::default()
                },
            );
            ui.label(caption);
            ui.add_space(2.0);
            match setup.spice_preview(selected_tab) {
                Ok(card) => {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(card)
                                .font(theme::mono(tokens::FS_1, FontWeight::Regular))
                                .color(c.text_dim),
                        )
                        .truncate(),
                    );
                }
                Err(_) => {
                    // The error itself already renders above the note.
                    ui.label(
                        egui::RichText::new("fix the error above — no card emitted")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(c.err),
                    );
                }
            }
        });
}
