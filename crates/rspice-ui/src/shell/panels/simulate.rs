//! Simulate-context side panels: run history + engine info (left), analysis
//! detail (right).

use egui::Ui;

use crate::common::AppState;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{TreeRow, check_row, input_row, kv_row, section_header};

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

    if let Some(action) = section_header(ui, "Run history", Some("Clear")) {
        if action.clicked() {
            state.simulation.clear_runs();
        }
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
            let note: &str = match selected_tab {
                0 => {
                    kv_row(ui, "Run before", "every analysis");
                    kv_row(ui, "Write to", "run database");
                    "Solves the DC operating point and annotates the schematic."
                }
                1 => {
                    input_row(ui, "Stop time", &mut setup.tran.stop);
                    input_row(ui, "Max step", &mut setup.tran.max_step);
                    input_row(ui, "Step time", &mut setup.tran.step);
                    input_row(ui, "Start time", &mut setup.tran.start);
                    check_row(ui, "Use initial conditions", &mut setup.tran.uic);
                    "Local truncation error controls step size between limits."
                }
                2 => {
                    input_row(ui, "Start", &mut setup.ac.fstart);
                    input_row(ui, "Stop", &mut setup.ac.fstop);
                    input_row(ui, "Points", &mut setup.ac.points);
                    let sweep = ["dec", "oct", "lin"];
                    kv_row(ui, "Sweep", sweep[setup.ac.sweep.min(2)]);
                    "Small-signal sweep around the operating point."
                }
                3 => {
                    input_row(ui, "Source", &mut setup.dc.source);
                    input_row(ui, "Start", &mut setup.dc.start);
                    input_row(ui, "Stop", &mut setup.dc.stop);
                    input_row(ui, "Step", &mut setup.dc.step);
                    "Sweeps a source over the operating range."
                }
                4 => {
                    input_row(ui, "Output", &mut setup.noise.output);
                    input_row(ui, "Input ref", &mut setup.noise.input);
                    input_row(ui, "Start", &mut setup.noise.fstart);
                    input_row(ui, "Stop", &mut setup.noise.fstop);
                    "Integrated and spot noise at the chosen output."
                }
                5 => {
                    input_row(ui, "Input", &mut setup.pz.input_pos);
                    input_row(ui, "Output", &mut setup.pz.output_pos);
                    "Extracts poles and zeros of the small-signal transfer."
                }
                6 => {
                    input_row(ui, "Output", &mut setup.sens.output_expr);
                    "DC or AC sensitivity of the output to every parameter."
                }
                7 => {
                    input_row(ui, "Samples", &mut setup.mc.num_runs);
                    input_row(ui, "Seed", &mut setup.mc.seed);
                    let vary = ["gaussian", "uniform", "worst-case"];
                    kv_row(ui, "Vary", vary[setup.mc.distribution_idx.min(2)]);
                    "Statistical runs share the analysis list above."
                }
                8 => {
                    input_row(ui, "Fundamental", &mut setup.pss.fund_freq);
                    input_row(ui, "Harmonics", &mut setup.pss.num_harmonics);
                    "Shooting-method periodic steady state."
                }
                9 => {
                    input_row(ui, "Probe", &mut setup.stb.probe_source);
                    input_row(ui, "Start", &mut setup.stb.start_freq);
                    input_row(ui, "Stop", &mut setup.stb.stop_freq);
                    "Loop gain and phase margin via the probe source."
                }
                10 => {
                    input_row(ui, "Start", &mut setup.temp.temp_start);
                    input_row(ui, "Stop", &mut setup.temp.temp_stop);
                    input_row(ui, "Step", &mut setup.temp.temp_step);
                    "Repeats the selected analysis across temperature."
                }
                _ => "",
            };

            if !note.is_empty() {
                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new(note)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(c.text_faint),
                );
            }
        });
}
