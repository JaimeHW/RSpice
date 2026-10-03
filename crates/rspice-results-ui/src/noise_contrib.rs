//! Band-integrated noise contributors, conversion provenance and noise-figure presentation.

use egui::{Align2, Sense, Ui};
use rspice_results::noise::NoiseSummary;
use rspice_ui_kit::plot::{DecimationCache, fmt_si};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{measurement_table, section_header};

mod figure;

/// Contributor evidence bound by the host to the displayed noise spectrum.
pub enum ContributorEvidence<'a> {
    /// The selected, qualified analysis retains a contributor summary.
    Retained {
        summary: &'a NoiseSummary,
        label: &'a str,
    },
    /// A spectrum is displayed, but its analysis retained no contributor summary.
    NotRetained,
    /// No usable noise analysis is bound to the spectrum.
    Unavailable,
}

const RANK_W: f32 = 30.0;
const SOURCE_W: f32 = 132.0;
const POWER_W: f32 = 94.0;
const SHARE_W: f32 = 70.0;
const ROW_H: f32 = 31.0;
const CELL_INSET: f32 = 6.0;

fn contributor_table_width() -> f32 {
    RANK_W + SOURCE_W + POWER_W + SHARE_W
}

/// Show contributors for the host-selected analysis and its validated noise-figure evidence.
pub fn right_panel(ui: &mut Ui, evidence: ContributorEvidence<'_>, cache: &mut DecimationCache) {
    let (summary, label) = match evidence {
        ContributorEvidence::Retained { summary, label } => (summary, label),
        absent => {
            ui.add_space(8.0);
            section_header(ui, "Contributors", Some("not retained"));
            let reason = match absent {
                ContributorEvidence::NotRetained => {
                    "This noise result contains a spectrum, but no band-integrated contributor evidence was retained."
                }
                _ => {
                    "The selected analysis carries no usable ordinary-noise spectrum, so no contributor evidence is bound to it."
                }
            };
            crate::presentation::panel_note(ui, reason);
            return;
        }
    };

    ui.add_space(8.0);
    section_header(ui, "Contributor evidence", None);
    let band = format!(
        "{} – {}",
        fmt_si(summary.band.0, "Hz", 3),
        fmt_si(summary.band.1, "Hz", 3)
    );
    let total = summary
        .total_rms
        .map(|value| fmt_si(value, "V rms", 3))
        .unwrap_or_else(|| "Not retained".to_owned());
    let input = summary
        .input_rms
        .map(|value| fmt_si(value, summary.input_rms_unit(), 3))
        .unwrap_or_else(|| "Not retained".to_owned());
    let count = summary.rows.len().to_string();
    measurement_table(
        ui,
        &[
            ("Analysis", label),
            ("Band", band.as_str()),
            ("Output integrated", total.as_str()),
            ("Input integrated", input.as_str()),
            ("Contributors", count.as_str()),
        ],
    );

    if let Some(conversion) = &summary.conversion {
        if let Some(sampling) = &conversion.sampling {
            section_header(ui, "Sampling", None);
            let mut rows = vec![
                (
                    "Output phase",
                    format!("{:.9}°", sampling.output.phase_degrees),
                ),
                ("Output voltage", fmt_si(sampling.output.voltage, "V", 6)),
                (
                    "Output slew",
                    fmt_si(sampling.output.slew_volts_per_second, "V/s", 6),
                ),
            ];
            if let Some(reference) = &sampling.reference {
                rows.push((
                    "Reference signal",
                    format!(
                        "V({},{})",
                        reference.node,
                        reference.reference.as_deref().unwrap_or("0")
                    ),
                ));
                rows.push((
                    "Reference phase",
                    format!("{:.9}°", reference.phase_degrees),
                ));
                rows.push((
                    "Reference slew",
                    fmt_si(reference.slew_volts_per_second, "V/s", 6),
                ));
            }
            if let Some(delay) = sampling.nominal_delay_seconds {
                rows.push(("Nominal delay", fmt_si(delay, "s", 6)));
            }
            measurement_table(
                ui,
                &rows
                    .iter()
                    .map(|(label, value)| (*label, value.as_str()))
                    .collect::<Vec<_>>(),
            );
        }
        section_header(ui, "Conversion channels", Some("offset axis"));
        let rows = [
            (
                "Input source",
                if conversion.input_source.is_empty() {
                    "Not requested".into()
                } else {
                    conversion.input_source.clone()
                },
            ),
            (
                "Carrier fundamental",
                fmt_si(conversion.carrier_hz, "Hz", 6),
            ),
            (
                "Input sideband",
                if conversion.input_source.is_empty() {
                    "Not requested".into()
                } else {
                    conversion.input_sideband.to_string()
                },
            ),
            ("Output sideband", conversion.output_sideband.to_string()),
            (
                "Folding window",
                format!(
                    "−{} … +{}",
                    conversion.max_sideband, conversion.max_sideband
                ),
            ),
            (
                "Input frequency band",
                if conversion.input_source.is_empty() {
                    "Not requested".into()
                } else {
                    format!(
                        "{:.6e} … {:.6e} Hz",
                        conversion.input_frequency(summary.band.0),
                        conversion.input_frequency(summary.band.1)
                    )
                },
            ),
            (
                "Output frequency band",
                format!(
                    "{:.6e} … {:.6e} Hz",
                    conversion.output_frequency(summary.band.0),
                    conversion.output_frequency(summary.band.1)
                ),
            ),
        ];
        measurement_table(
            ui,
            &rows
                .iter()
                .map(|(label, value)| (*label, value.as_str()))
                .collect::<Vec<_>>(),
        );
        crate::presentation::panel_note(
            ui,
            "The plot axis is offset Hz. Physical channel frequency = offset + sideband × fundamental. Negative frequencies denote conjugate channels.",
        );
    }
    if let Some(figure) = &summary.noise_figure {
        figure::show(ui, figure, cache);
    }

    ui.add_space(8.0);
    section_header(
        ui,
        "Ranked contributors",
        Some(&format!("integrated {}", summary.power_unit())),
    );
    if summary.rows.is_empty() {
        crate::presentation::panel_note(ui, "No per-device contributor rows were retained.");
        return;
    }
    contributor_table(ui, summary);
}

fn contributor_table(ui: &mut Ui, summary: &NoiseSummary) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let viewport_width = ui.available_width().max(1.0);

    let table = ui
        .scope(|ui| {
            egui::ScrollArea::horizontal()
                .id_salt("rspice.results.noise.contributors")
                .auto_shrink([false, true])
                .scroll_bar_visibility(egui::scroll_area::ScrollBarVisibility::VisibleWhenNeeded)
                .show(ui, |ui| {
                    let width = viewport_width.max(contributor_table_width());
                    ui.set_min_width(width);
                    contributor_header(ui, width);
                    for (rank, row) in summary.rows.iter().enumerate() {
                        let (rect, response) =
                            ui.allocate_exact_size(egui::vec2(width, ROW_H), Sense::hover());
                        response.widget_info(|| {
                            egui::WidgetInfo::labeled(
                                egui::WidgetType::Label,
                                ui.is_enabled(),
                                format!(
                                    "Contributor rank {}, {} {}, integrated noise power {:.6e} {}, share {:.3} percent",
                                    rank + 1,
                                    row.device,
                                    row.mechanism,
                                    row.power,
                                    summary.power_unit(),
                                    row.share_pct
                                ),
                            )
                        });
                        ui.ctx().accesskit_node_builder(response.id, |node| {
                            node.set_role(egui::accesskit::Role::Row);
                        });
                        if !ui.is_rect_visible(rect) {
                            continue;
                        }
                        if response.hovered() {
                            ui.painter().rect_filled(rect, 0.0, c.bg_hover);
                        }
                        ui.painter().hline(
                            rect.x_range(),
                            rect.bottom() - 0.5,
                            egui::Stroke::new(1.0, c.border.gamma_multiply(0.6)),
                        );
                        paint_cell(
                            ui,
                            rect,
                            0.0,
                            RANK_W,
                            rank + 1,
                            Align2::LEFT_CENTER,
                            c.text_faint,
                        );
                        let source = format!("{}\n{}", row.device, row.mechanism);
                        paint_cell(ui, rect, RANK_W, SOURCE_W, source, Align2::LEFT_CENTER, c.text);
                        paint_cell(
                            ui,
                            rect,
                            RANK_W + SOURCE_W,
                            POWER_W,
                            format!("{:.3e}", row.power),
                            Align2::RIGHT_CENTER,
                            c.text_dim,
                        );
                        let share_offset = RANK_W + SOURCE_W + POWER_W;
                        let share_cell = column_rect(rect, share_offset, width - share_offset);
                        let fill_rect = share_cell.shrink2(egui::vec2(CELL_INSET, 8.0));
                        let fill = fill_rect.width()
                            * (row.share_pct as f32 / 100.0).clamp(0.0, 1.0);
                        ui.painter().rect_filled(
                            egui::Rect::from_min_size(
                                fill_rect.min,
                                egui::vec2(fill, fill_rect.height()),
                            ),
                            2.0,
                            c.accent_dim,
                        );
                        paint_cell(
                            ui,
                            rect,
                            share_offset,
                            width - share_offset,
                            format!("{:.1}%", row.share_pct),
                            Align2::RIGHT_CENTER,
                            c.text,
                        );
                    }
                });
        })
        .response;
    table.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Label,
            ui.is_enabled(),
            format!("Ranked noise contributors, {} rows", summary.rows.len()),
        )
    });
    ui.ctx().accesskit_node_builder(table.id, |node| {
        node.set_role(egui::accesskit::Role::Table);
        node.set_row_count(summary.rows.len().saturating_add(1));
        node.set_column_count(4);
    });
}

fn contributor_header(ui: &mut Ui, width: f32) {
    let c = Tokens::get(ui.ctx()).color;
    let (rect, row) = ui.allocate_exact_size(egui::vec2(width, 23.0), Sense::hover());
    ui.ctx().accesskit_node_builder(row.id, |node| {
        node.set_role(egui::accesskit::Role::Row);
    });
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        egui::Stroke::new(1.0, c.border),
    );
    for (index, (label, offset, column_width, align)) in [
        ("#", 0.0, RANK_W, Align2::LEFT_CENTER),
        ("SOURCE", RANK_W, SOURCE_W, Align2::LEFT_CENTER),
        ("POWER", RANK_W + SOURCE_W, POWER_W, Align2::RIGHT_CENTER),
        (
            "SHARE",
            RANK_W + SOURCE_W + POWER_W,
            width - RANK_W - SOURCE_W - POWER_W,
            Align2::RIGHT_CENTER,
        ),
    ]
    .into_iter()
    .enumerate()
    {
        let cell = column_rect(rect, offset, column_width);
        let response = ui.interact(cell, row.id.with(index), Sense::hover());
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label)
        });
        ui.ctx().accesskit_node_builder(response.id, |node| {
            node.set_role(egui::accesskit::Role::ColumnHeader);
            node.set_label(label);
        });
        paint_cell(ui, rect, offset, column_width, label, align, c.text_faint);
    }
}

fn column_rect(row: egui::Rect, offset: f32, width: f32) -> egui::Rect {
    egui::Rect::from_min_size(
        egui::pos2(row.left() + offset, row.top()),
        egui::vec2(width.max(0.0), row.height()),
    )
}

fn paint_cell(
    ui: &Ui,
    row: egui::Rect,
    offset: f32,
    width: f32,
    text: impl ToString,
    align: Align2,
    color: egui::Color32,
) {
    let cell = column_rect(row, offset, width);
    let x = if align == Align2::RIGHT_CENTER {
        cell.right() - CELL_INSET
    } else {
        cell.left() + CELL_INSET
    };
    ui.painter()
        .with_clip_rect(cell.shrink2(egui::vec2(2.0, 0.0)))
        .text(
            egui::pos2(x, cell.center().y),
            align,
            text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        );
}
