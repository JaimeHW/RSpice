//! PVT, sweeps & variation.
//!
//! The run space is composed from independent axes — process, supply,
//! temperature — and resolves to an exact point count. The page is that
//! composition: each axis states what it contributes, and the forecast states
//! what they multiply out to, derived from the same corner configuration the
//! run consumes.

use egui::Ui;

use crate::simulation::dialog::corner::ProcessCorner;
use crate::simulation::plan::AnalysisKind;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{mono_input, select};
use crate::workbench::RSpiceApp;

use super::page_kit::{
    CARD_PAD_X, Tone, card, card_body, card_note, card_row, field_pair, ledger_head, ledger_row,
    rule_row,
};

/// Rows shown before the point table truncates. A resolved run space can be
/// large; a table that rendered all of it would be a scrolling wall rather
/// than a check on the composition.
const POINT_TABLE_LIMIT: usize = 40;

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    let sweeping = app
        .state
        .sim_setup
        .has_enabled_analysis_kind(AnalysisKind::Corner);
    forecast(ui, app, sweeping);
    if sweeping {
        card_row(ui, app, process_axis, supply_axis);
        card_row(ui, app, temperature_axis, composition_policy);
        point_table(ui, app);
    } else {
        card_row(ui, app, reference_point, composition_policy);
    }
}

// ------------------------------------------------------------------- forecast

fn forecast(ui: &mut Ui, app: &RSpiceApp, sweeping: bool) {
    let t = Tokens::get(ui.ctx());
    let points = app.state.sim_setup.run_set_point_count();
    let error = app.state.sim_setup.run_set_error();
    let analyses = app.state.sim_setup.enabled_analysis_instance_count();
    let strip_width = ui.available_width();
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(egui::Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .inner_margin(egui::Margin::symmetric(CARD_PAD_X as i8, 9))
        .show(ui, |ui| {
            ui.set_width(strip_width - CARD_PAD_X * 2.0 - 2.0);
            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing = egui::vec2(14.0, 6.0);
                match (points, &error) {
                    (Some(points), _) => {
                        ui.label(
                            egui::RichText::new(points.to_string())
                                .font(theme::mono(tokens::FS_4, FontWeight::SemiBold))
                                .color(t.color.text),
                        );
                        ui.label(
                            egui::RichText::new(if points == 1 { "point" } else { "points" })
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.text_dim),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "× {analyses} enabled {} = {} task{}",
                                if analyses == 1 {
                                    "analysis"
                                } else {
                                    "analyses"
                                },
                                points * analyses,
                                if points * analyses == 1 { "" } else { "s" }
                            ))
                            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                        );
                    }
                    (None, Some(error)) => {
                        ui.label(
                            egui::RichText::new(format!("run space unresolved · {error}"))
                                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                                .color(t.color.err),
                        );
                    }
                    (None, None) => {}
                }
                ui.label(
                    egui::RichText::new(if sweeping {
                        "corner sweep enabled · the axes below compose the space"
                    } else {
                        "single reference point · enable a corner analysis to sweep"
                    })
                    .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text_dim),
                );
            });
        });
}

// ----------------------------------------------------------------------- axes

fn process_axis(ui: &mut Ui, app: &mut RSpiceApp) {
    let corners = [
        (ProcessCorner::TT, "Typical"),
        (ProcessCorner::SS, "Slow / slow"),
        (ProcessCorner::FF, "Fast / fast"),
        (ProcessCorner::SF, "Slow / fast"),
        (ProcessCorner::FS, "Fast / slow"),
    ];
    let selected = selected_process_count(app);
    let status = format!("{selected} of 5");
    card(
        ui,
        "Process axis",
        Some((
            status.as_str(),
            if selected == 0 { Tone::Error } else { Tone::Ok },
        )),
        |ui| {
            card_body(ui, |ui| {
                for (corner, meaning) in corners {
                    let value = process_flag(app, corner);
                    let mut enabled = *value;
                    if ui
                        .checkbox(
                            &mut enabled,
                            egui::RichText::new(format!("{} · {meaning}", corner.short_name()))
                                .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
                        )
                        .changed()
                    {
                        *process_flag(app, corner) = enabled;
                    }
                }
            });
            card_note(
                ui,
                "A corner selects the model section each library resolves to. Deselecting every \
                 corner leaves the sweep with no process axis, which preflight refuses rather \
                 than silently running at typical.",
            );
        },
    );
}

fn process_flag(app: &mut RSpiceApp, corner: ProcessCorner) -> &mut bool {
    let state = &mut app.state.sim_setup.corner;
    match corner {
        ProcessCorner::TT => &mut state.process_tt,
        ProcessCorner::SS => &mut state.process_ss,
        ProcessCorner::FF => &mut state.process_ff,
        ProcessCorner::SF => &mut state.process_sf,
        ProcessCorner::FS => &mut state.process_fs,
    }
}

fn selected_process_count(app: &RSpiceApp) -> usize {
    let state = &app.state.sim_setup.corner;
    usize::from(state.process_tt)
        + usize::from(state.process_ss)
        + usize::from(state.process_ff)
        + usize::from(state.process_sf)
        + usize::from(state.process_fs)
}

fn supply_axis(ui: &mut Ui, app: &mut RSpiceApp) {
    let on = app.state.sim_setup.corner.enable_voltage_sweep;
    let status = if on { "3 values" } else { "nominal only" };
    card(
        ui,
        "Supply axis",
        Some((status, if on { Tone::Ok } else { Tone::Neutral })),
        |ui| {
            card_body(ui, |ui| {
                let mut enable = on;
                if ui
                    .checkbox(
                        &mut enable,
                        egui::RichText::new("Sweep supply voltage")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
                    )
                    .changed()
                {
                    app.state.sim_setup.corner.enable_voltage_sweep = enable;
                }
                field_pair(
                    ui,
                    ("Minimum", &mut |ui: &mut Ui, width: f32| {
                        mono_input(ui, &mut app.state.sim_setup.corner.voltage_min, width);
                    }),
                    Some(("Nominal", &mut |ui: &mut Ui, width: f32| {
                        mono_input(ui, &mut app.state.sim_setup.corner.voltage_nom, width);
                    })),
                );
                field_pair(
                    ui,
                    ("Maximum", &mut |ui: &mut Ui, width: f32| {
                        mono_input(ui, &mut app.state.sim_setup.corner.voltage_max, width);
                    }),
                    None,
                );
            });
            card_note(
                ui,
                "With the sweep off only the nominal value is used and the axis contributes one \
                 point; the minimum and maximum are retained so turning the axis back on restores \
                 the same space.",
            );
        },
    );
}

fn temperature_axis(ui: &mut Ui, app: &mut RSpiceApp) {
    let on = app.state.sim_setup.corner.enable_temp_sweep;
    let status = if on { "3 values" } else { "reference only" };
    card(
        ui,
        "Temperature axis",
        Some((status, if on { Tone::Ok } else { Tone::Neutral })),
        |ui| {
            card_body(ui, |ui| {
                let mut enable = on;
                if ui
                    .checkbox(
                        &mut enable,
                        egui::RichText::new("Sweep temperature")
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular)),
                    )
                    .changed()
                {
                    app.state.sim_setup.corner.enable_temp_sweep = enable;
                }
                field_pair(
                    ui,
                    ("Cold · °C", &mut |ui: &mut Ui, width: f32| {
                        mono_input(ui, &mut app.state.sim_setup.corner.temp_cold, width);
                    }),
                    Some(("Room · °C", &mut |ui: &mut Ui, width: f32| {
                        mono_input(ui, &mut app.state.sim_setup.corner.temp_room, width);
                    })),
                );
                field_pair(
                    ui,
                    ("Hot · °C", &mut |ui: &mut Ui, width: f32| {
                        mono_input(ui, &mut app.state.sim_setup.corner.temp_hot, width);
                    }),
                    None,
                );
            });
            card_note(
                ui,
                "Temperature enters device models through TNOM as well as the ambient value, so a \
                 temperature axis is a model-evaluation axis and not only an operating condition.",
            );
        },
    );
}

fn reference_point(ui: &mut Ui, app: &mut RSpiceApp) {
    let corners: Vec<String> = [
        ProcessCorner::TT,
        ProcessCorner::SS,
        ProcessCorner::FF,
        ProcessCorner::SF,
        ProcessCorner::FS,
    ]
    .iter()
    .map(|corner| corner.short_name().to_owned())
    .collect();
    let current = app
        .state
        .sim_setup
        .reference_pvt
        .process
        .short_name()
        .to_owned();
    let mut temperature = format!(
        "{:.1}",
        app.state.sim_setup.reference_pvt.temperature_celsius
    );
    card(
        ui,
        "Reference point",
        Some(("what one run resolves to", Tone::Neutral)),
        |ui| {
            card_body(ui, |ui| {
                let mut picked = None;
                let mut temperature_response = None;
                field_pair(
                    ui,
                    ("Process corner", &mut |ui: &mut Ui, width: f32| {
                        picked = select(
                            ui,
                            "simulation.runset.process",
                            "Reference process corner",
                            &current,
                            &corners,
                            width,
                        );
                    }),
                    Some(("Temperature · °C", &mut |ui: &mut Ui, width: f32| {
                        temperature_response = Some(mono_input(ui, &mut temperature, width));
                    })),
                );
                let requested_process = picked.map(|index| {
                    [
                        ProcessCorner::TT,
                        ProcessCorner::SS,
                        ProcessCorner::FF,
                        ProcessCorner::SF,
                        ProcessCorner::FS,
                    ][index.min(4)]
                });
                let committed = temperature_response.is_some_and(|response| response.lost_focus())
                    || requested_process.is_some();
                if committed {
                    let process =
                        requested_process.unwrap_or(app.state.sim_setup.reference_pvt.process);
                    let celsius = temperature
                        .trim()
                        .parse::<f64>()
                        .unwrap_or(app.state.sim_setup.reference_pvt.temperature_celsius);
                    if let Err(error) = app.state.sim_setup.set_reference_pvt(process, celsius) {
                        app.state.workbench.analysis_lifecycle_status = error;
                    } else {
                        app.invalidate_simulation_preflight();
                    }
                }
            });
            card_note(
                ui,
                "The reference temperature is the solver's TEMP option: changing it here changes \
                 what every analysis in the plan is evaluated at, which is why the solver page \
                 shows it as owned by the run set rather than editable there.",
            );
        },
    );
}

fn composition_policy(ui: &mut Ui, app: &mut RSpiceApp) {
    let full_matrix = app.state.sim_setup.corner.full_matrix;
    let sweeping = app
        .state
        .sim_setup
        .has_enabled_analysis_kind(AnalysisKind::Corner);
    let variation = app
        .state
        .sim_setup
        .has_enabled_analysis_kind(AnalysisKind::MonteCarlo);
    card(
        ui,
        "Composition",
        Some((
            if full_matrix { "cartesian" } else { "diagonal" },
            Tone::Neutral,
        )),
        |ui| {
            card_body(ui, |ui| {
                if sweeping {
                    let options = vec![
                        "Cartesian · every combination".to_owned(),
                        "Diagonal · zipped".to_owned(),
                    ];
                    let selected = options[usize::from(!full_matrix)].clone();
                    let mut picked = None;
                    field_pair(
                        ui,
                        ("Axis composition", &mut |ui: &mut Ui, width: f32| {
                            picked = select(
                                ui,
                                "simulation.runset.composition",
                                "Axis composition",
                                &selected,
                                &options,
                                width,
                            );
                        }),
                        None,
                    );
                    if let Some(index) = picked {
                        app.state.sim_setup.corner.full_matrix = index == 0;
                    }
                }
                rule_row(
                    ui,
                    "Statistical variation",
                    if variation {
                        "enabled · a Monte Carlo analysis is in the plan"
                    } else {
                        "disabled · no Monte Carlo analysis is enabled"
                    },
                );
                rule_row(
                    ui,
                    "Point identity",
                    "each point carries its corner, supply and temperature into the manifest",
                );
            });
            card_note(
                ui,
                "Cartesian composition multiplies the axes; diagonal zips them and requires every \
                 enabled axis to have the same length. Variation is a separate dimension owned by \
                 its analysis, not an axis of this space.",
            );
        },
    );
}

// -------------------------------------------------------------- resolved points

const POINT_COLUMNS: [f32; 2] = [0.18, 0.82];

fn point_table(ui: &mut Ui, app: &RSpiceApp) {
    let Some(names) = app.state.sim_setup.run_set_point_names() else {
        return;
    };
    let total = names.len();
    let shown = total.min(POINT_TABLE_LIMIT);
    let status = if total > shown {
        format!("first {shown} of {total}")
    } else {
        format!("{total} resolved")
    };
    card(
        ui,
        "Resolved points",
        Some((status.as_str(), Tone::Neutral)),
        |ui| {
            ledger_head(ui, &POINT_COLUMNS, &["Index", "Point"]);
            for (index, name) in names.iter().take(shown).enumerate() {
                ledger_row(
                    ui,
                    &POINT_COLUMNS,
                    &[
                        (format!("{:03}", index + 1).as_str(), Tone::Neutral),
                        (name.as_str(), Tone::Neutral),
                    ],
                    false,
                );
            }
            if total > shown {
                card_note(
                    ui,
                    "The remaining points are not listed here. Every point is still executed and \
                     recorded in the run manifest; this table exists to check the composition, \
                     not to enumerate the run.",
                );
            }
        },
    );
}
