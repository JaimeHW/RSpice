//! NOISE — amplitude-density instrument derived from retained power spectral
//! density and its contributor
//! evidence.
//!
//! The center surface owns the frequency spectrum. The right inspector keeps
//! the complete band-integrated contributor table associated with the exact
//! same noise analysis, so changing the active dataset or analysis cannot
//! leave either surface reading stale evidence.

use egui::{Align2, Sense, Ui};

use crate::state::{AnalysisResult, NoiseSummary};
use crate::ui::plot::fmt_si;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{measurement_table, section_header};
use crate::workbench::AppState;

mod figure;

const RANK_W: f32 = 30.0;
const SOURCE_W: f32 = 132.0;
const POWER_W: f32 = 94.0;
const SHARE_W: f32 = 70.0;
const ROW_H: f32 = 31.0;
const CELL_INSET: f32 = 6.0;

fn contributor_table_width() -> f32 {
    RANK_W + SOURCE_W + POWER_W + SHARE_W
}

fn selected_noise_analysis(state: &AppState) -> Option<&AnalysisResult> {
    let run = state.simulation.active_run()?;
    let index = super::bode::selected_noise_analysis_index(state)?;
    run.analyses.get(index)
}

fn selected_summary(state: &AppState) -> Option<(&NoiseSummary, &str)> {
    let analysis = selected_noise_analysis(state)?;
    Some((analysis.noise_summary.as_ref()?, analysis.label.as_str()))
}

/// Why no contributor evidence is on screen. The table binds strictly to the
/// selected noise analysis, so an empty table has two distinct causes and the
/// reader is owed the one that applies.
fn contributor_absence_reason(state: &AppState) -> &'static str {
    if selected_noise_analysis(state).is_some() {
        "This noise result contains a spectrum, but no band-integrated contributor evidence was retained."
    } else {
        "The selected analysis carries no usable ordinary-noise spectrum, so no contributor evidence is bound to it."
    }
}

mod csv;
pub(crate) use csv::export_csv;

/// Render spectrum provenance and the full contributor table for the exact
/// analysis shown in the center instrument (the waves pane-stack).
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    super::bode::noise_spectrum_right_panel(ui, state);

    let Some((summary, label)) = selected_summary(state) else {
        ui.add_space(8.0);
        section_header(ui, "Contributors", Some("not retained"));
        super::panel_note(ui, contributor_absence_reason(state));
        return;
    };
    let summary = summary.clone();

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
        .map(|value| fmt_si(value, "V rms", 3))
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
        super::panel_note(
            ui,
            "The plot axis is offset Hz. Physical channel frequency = offset + sideband × fundamental. Negative frequencies denote conjugate channels.",
        );
    }
    if let Some(figure) = &summary.noise_figure {
        figure::show(ui, figure, &mut state.ui.results.cache);
    }

    ui.add_space(8.0);
    section_header(ui, "Ranked contributors", Some("integrated V²"));
    if summary.rows.is_empty() {
        super::panel_note(ui, "No per-device contributor rows were retained.");
        return;
    }
    contributor_table(ui, &summary);
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
                                    "Contributor rank {}, {} {}, integrated noise power {:.6e} volts squared, share {:.3} percent",
                                    rank + 1,
                                    row.device,
                                    row.mechanism,
                                    row.power,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{AnalysisResult, AnalysisType, SimulationRun, WaveformData};

    #[test]
    fn hbnoise_csv_separates_decibels_from_density_and_retains_source_reference() {
        let figure = std::sync::Arc::new(crate::state::NoiseFigureEvidence {
            input_source: "V1".into(),
            source_resistor: "Rs".into(),
            source_resistance_ohm: 50.0,
            source_temperature_kelvin: 300.15,
            reference_temperature_kelvin: 290.0,
            frequencies: vec![1e3, 1e4],
            decibels: vec![2.0, 3.0],
        });
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::Hbnoise, "HBNOISE")
                .with_waveforms(vec![
                    WaveformData::new(
                        "onoise",
                        figure.frequencies.clone(),
                        vec![1e-18, 2e-18],
                        "#fff",
                    ),
                    WaveformData::new(
                        "Noise figure (SSB)",
                        figure.frequencies.clone(),
                        figure.decibels.clone(),
                        "#fff",
                    )
                    .with_unit("dB"),
                ])
                .with_noise_summary(NoiseSummary {
                    conversion: Some(crate::state::PeriodicNoiseConversionEvidence {
                        input_source: "V1".into(),
                        carrier_hz: 1e6,
                        input_sideband: 1,
                        output_sideband: -1,
                        max_sideband: 4,
                    }),
                    noise_figure: Some(figure),
                    band: (1e3, 1e4),
                    rows: vec![crate::state::NoiseContributorRow {
                        device: "Rs".into(),
                        mechanism: "thermal".into(),
                        power: 9e-15,
                        share_pct: 50.0,
                    }],
                    ..Default::default()
                }),
        );
        let export = export_csv(&run, &[0]).unwrap();
        let lines: Vec<Vec<&str>> = export
            .contents
            .lines()
            .map(|line| line.split(',').collect())
            .collect();
        assert_eq!(lines.len(), 7);
        assert!(lines.iter().all(|row| row.len() == 28), "{:?}", lines);
        let figure_rows: Vec<_> = lines
            .iter()
            .filter(|row| row[0] == "noise_figure")
            .collect();
        assert_eq!(figure_rows.len(), 2);
        assert_eq!(figure_rows[0][15].parse::<f64>().unwrap(), 2.0);
        assert_eq!(figure_rows[1][15].parse::<f64>().unwrap(), 3.0);
        for row in figure_rows {
            assert!(row[8].is_empty());
            assert_eq!(row[16], "V1");
            assert_eq!(row[17], "Rs");
            assert_eq!(row[18].parse::<f64>().unwrap(), 50.0);
            assert_eq!(row[19].parse::<f64>().unwrap(), 300.15);
            assert_eq!(row[20].parse::<f64>().unwrap(), 290.0);
        }
        for row in lines.iter().skip(1) {
            assert_eq!(row[21], "offset_hz");
            assert_eq!(row[22].parse::<f64>().unwrap(), 1e6);
            assert_eq!(row[23], "1");
            assert_eq!(row[24], "-1");
            assert_eq!(row[25], "4");
            if !row[7].is_empty() {
                let offset = row[7].parse::<f64>().unwrap();
                assert_eq!(row[26].parse::<f64>().unwrap(), offset + 1e6);
                assert_eq!(row[27].parse::<f64>().unwrap(), offset - 1e6);
            }
        }
        let densities: Vec<_> = lines.iter().filter(|row| row[0] == "spectrum").collect();
        assert_eq!(densities.len(), 2);
        assert!(
            densities
                .iter()
                .all(|row| row[3] == "onoise" && row[15].is_empty())
        );
    }

    #[test]
    fn contributor_table_preserves_all_columns_at_narrow_inspector_width() {
        assert_eq!(contributor_table_width(), 326.0);
        assert!(SOURCE_W >= 120.0);
        assert!(POWER_W >= 90.0);
    }

    #[test]
    fn selected_summary_is_bound_to_the_same_renderable_noise_analysis() {
        let mut first = AnalysisResult::new(1, AnalysisType::Noise, "first").with_waveforms(vec![
            WaveformData::new("inoise", vec![1.0, 10.0], vec![1.0e-9, 2.0e-9], "#fff"),
        ]);
        first.noise_summary = Some(NoiseSummary {
            conversion: None,
            noise_figure: None,
            band: (1.0, 10.0),
            ..NoiseSummary::default()
        });
        let mut second =
            AnalysisResult::new(2, AnalysisType::Noise, "second").with_waveforms(vec![
                WaveformData::new("inoise", vec![1.0, 10.0], vec![3.0e-9, 4.0e-9], "#fff"),
            ]);
        second.noise_summary = Some(NoiseSummary {
            conversion: None,
            noise_figure: None,
            band: (2.0, 20.0),
            ..NoiseSummary::default()
        });
        let mut state = AppState::default();
        let mut run = SimulationRun::new(1);
        run.add_analysis(first);
        run.add_analysis(second);
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(1));

        let (summary, label) = selected_summary(&state).expect("selected noise summary");
        assert_eq!(label, "second");
        assert_eq!(summary.band, (2.0, 20.0));
    }
}
