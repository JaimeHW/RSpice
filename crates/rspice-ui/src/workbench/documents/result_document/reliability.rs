//! Reliability stress and lifetime checkpoint viewer.

use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
    ReliabilityDeviceEvidence, SimulationState,
};
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::{LegendChip, StripHeader};
use super::{AnalysisPresentationKey, ReliabilitySelection, panel_note, stat_table, well_hint};

fn active_reliability(
    simulation: &SimulationState,
) -> Option<(&AnalysisResult, &[ReliabilityDeviceEvidence])> {
    let analysis = simulation.active_analysis()?;
    let Some(AnalysisResultFamilyMetadata::Reliability { .. }) = analysis.family_metadata.as_ref()
    else {
        return None;
    };
    let Some(AnalysisResultPayload::Reliability { devices }) = analysis.result_payload.as_ref()
    else {
        return None;
    };
    if !analysis.success
        || analysis.analysis_type != AnalysisType::Reliability
        || analysis.validate_retained_evidence().is_err()
    {
        return None;
    }
    Some((analysis, devices))
}

fn nearest_checkpoint_index(
    device: &ReliabilityDeviceEvidence,
    requested_years: f64,
) -> Option<usize> {
    device
        .checkpoints
        .iter()
        .enumerate()
        .filter(|(_, checkpoint)| checkpoint.years.is_finite())
        .min_by(|(_, left), (_, right)| {
            (left.years - requested_years)
                .abs()
                .total_cmp(&(right.years - requested_years).abs())
        })
        .map(|(index, _)| index)
}

fn padded_range(values: &[f64], include_zero: bool) -> Option<(f64, f64)> {
    let (mut minimum, mut maximum) = super::finite_extremes(values)?;
    if include_zero {
        minimum = minimum.min(0.0);
        maximum = maximum.max(0.0);
    }
    if minimum < maximum {
        let pad = ((maximum - minimum) * 0.10).max(f64::EPSILON);
        Some((minimum - pad, maximum + pad))
    } else {
        let pad = minimum.abs().mul_add(0.10, 1.0);
        Some((minimum - pad, maximum + pad))
    }
}

fn lifetime_range(years: &[f64]) -> Option<(f64, f64)> {
    let (minimum, maximum) = super::finite_extremes(years)?;
    if minimum < 0.0 {
        return None;
    }
    let stop = if maximum > 0.0 { maximum * 1.05 } else { 1.0 };
    Some((0.0, stop))
}

fn device_cache_seed(analysis_id: u64, device_id: &str) -> u64 {
    device_id.as_bytes().iter().fold(
        0x5245_4C49_0000_0000_u64 ^ analysis_id.rotate_left(19),
        |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x100_0000_01B3),
    )
}

#[derive(Debug, Clone, Copy)]
enum DegradationPlot {
    ThresholdVoltage,
    RelativeShifts,
}

fn show_degradation_plot(
    ui: &mut Ui,
    results: &mut super::ResultsState,
    analysis_id: u64,
    device: &ReliabilityDeviceEvidence,
    selected_checkpoint: Option<usize>,
    plot_kind: DegradationPlot,
) -> Option<usize> {
    let colors = Tokens::get(ui.ctx()).color;
    let years: Vec<f64> = device.checkpoints.iter().map(|point| point.years).collect();
    let (primary, secondary, y_label, accessible_name, plot_index) = match plot_kind {
        DegradationPlot::ThresholdVoltage => (
            device
                .checkpoints
                .iter()
                .map(|point| point.shift.threshold_voltage_shift_v)
                .collect::<Vec<_>>(),
            None,
            "Î”VTH Â· V",
            "Threshold-voltage degradation",
            0,
        ),
        DegradationPlot::RelativeShifts => (
            device
                .checkpoints
                .iter()
                .map(|point| point.shift.mobility_shift)
                .collect::<Vec<_>>(),
            Some(
                device
                    .checkpoints
                    .iter()
                    .map(|point| point.shift.drain_source_resistance_shift)
                    .collect::<Vec<_>>(),
            ),
            "relative shift",
            "Mobility and drain-source-resistance degradation",
            1,
        ),
    };
    let primary_color = match plot_kind {
        DegradationPlot::ThresholdVoltage => colors.traces[2],
        DegradationPlot::RelativeShifts => colors.traces[0],
    };
    let (auto_x0, auto_x1) = lifetime_range(&years)?;
    let mut all_y = primary.clone();
    if let Some(secondary) = &secondary {
        all_y.extend_from_slice(secondary);
    }
    let (auto_y0, auto_y1) = padded_range(&all_y, true)?;
    let view = results.plot_view(super::ResultViewer::Reliability, plot_index);
    let (x0, x1) = view.x.unwrap_or((auto_x0, auto_x1));
    let (y0, y1) = view.y.unwrap_or((auto_y0, auto_y1));
    let mut spec = PlotSpec::new(
        Axis::linear_with(x0, x1, "years", 6),
        XScale::Linear,
        Axis::linear_with(y0, y1, "", 6).with_label(y_label),
    )
    .accessible_name(accessible_name)
    .accessible_detail(
        "Lines connect exact retained reliability checkpoints; hover and selection snap to a retained checkpoint and no lifetime extrapolation is shown.",
    );
    let cache_seed = device_cache_seed(analysis_id, &device.device_id);
    spec.traces.push(
        Trace::new(&years, &primary, primary_color)
            .marker_style(0)
            .cache_key(cache_seed ^ plot_index as u64),
    );
    if let Some(secondary) = &secondary {
        spec.traces.push(
            Trace::new(&years, secondary, colors.traces[1])
                .marker_style(1)
                .cache_key(cache_seed ^ 0x100 ^ plot_index as u64),
        );
    }
    if let Some(index) = selected_checkpoint
        && let Some((&year, &value)) = years.get(index).zip(primary.get(index))
    {
        spec.markers.push(plot::Marker {
            x: year,
            y: value,
            color: colors.accent,
            label: format!("selected {year:.6e} y"),
            drop_line: true,
            label_dy: 0.0,
            shape: plot::MarkerShape::Point,
        });
    }
    let readout = |requested: f64| {
        nearest_checkpoint_index(device, requested).map_or_else(Vec::new, |index| {
            let checkpoint = &device.checkpoints[index];
            let mut rows = vec![
                ("years".to_owned(), format!("{:.17e}", checkpoint.years)),
                (
                    "Î”VTH".to_owned(),
                    format!("{:+.17e} V", checkpoint.shift.threshold_voltage_shift_v),
                ),
            ];
            if matches!(plot_kind, DegradationPlot::RelativeShifts) {
                rows.extend([
                    (
                        "Î” mobility".to_owned(),
                        format!("{:+.17e}", checkpoint.shift.mobility_shift),
                    ),
                    (
                        "Î” RDS".to_owned(),
                        format!("{:+.17e}", checkpoint.shift.drain_source_resistance_shift),
                    ),
                ]);
            }
            rows
        })
    };
    let response = plot::show(ui, &spec, &mut results.cache, None, Some(&readout));
    if response.view.any() {
        results
            .plot_view_mut(super::ResultViewer::Reliability, plot_index)
            .apply(&response.view);
    }
    response
        .clicked_x
        .and_then(|requested| nearest_checkpoint_index(device, requested))
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    active_reliability(&state.simulation).is_some()
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(run) = state.simulation.active_run() else {
        well_hint(ui, "Select a dataset with retained reliability evidence");
        return;
    };
    let Some((analysis, devices)) = active_reliability(&state.simulation) else {
        well_hint(ui, "Select a validated reliability analysis");
        return;
    };
    let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let selected = state.ui.results.selected_reliability.clone();
    let mut requested = None;
    let checkpoint_count: usize = devices.iter().map(|device| device.checkpoints.len()).sum();
    let plotted_device = selected
        .as_ref()
        .filter(|selection| selection.analysis == analysis_key)
        .and_then(|selection| {
            devices
                .iter()
                .find(|device| device.device_id == selection.device_id)
        })
        .or_else(|| devices.first());
    let selected_checkpoint = plotted_device.and_then(|device| {
        selected.as_ref().and_then(|selection| {
            device
                .checkpoints
                .iter()
                .position(|checkpoint| checkpoint.years.to_bits() == selection.checkpoint_year_bits)
        })
    });
    let tokens = Tokens::get(ui.ctx());
    let legend = [
        LegendChip {
            name: "Î”VTH",
            color: tokens.color.traces[2],
            on: true,
        },
        LegendChip {
            name: "Î” mobility",
            color: tokens.color.traces[0],
            on: true,
        },
        LegendChip {
            name: "Î”RDS",
            color: tokens.color.traces[1],
            on: true,
        },
    ];
    let plot_zoomed = state
        .ui
        .results
        .plot_view(super::ResultViewer::Reliability, 0)
        .is_zoomed()
        || state
            .ui
            .results
            .plot_view(super::ResultViewer::Reliability, 1)
            .is_zoomed();

    let header = StripHeader::new(
        "RELIABILITY",
        &format!(
            "{} Â· {} devices Â· {} retained lifetime checkpoints{}",
            analysis.label,
            devices.len(),
            checkpoint_count,
            plotted_device.map_or_else(String::new, |device| format!(" Â· {}", device.device_id))
        ),
        &legend,
    )
    .zoomed(plot_zoomed)
    .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Reliability, 0);
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Reliability, 1);
    }

    if let Some(device) = plotted_device.filter(|device| !device.checkpoints.is_empty()) {
        let available_height = ui.available_height();
        let wide = ui.available_width() >= 760.0;
        let total_plot_height = (available_height * 0.40)
            .clamp(
                if wide { 150.0 } else { 240.0 },
                if wide { 220.0 } else { 340.0 },
            )
            .min((available_height - 140.0).max(if wide { 120.0 } else { 190.0 }));
        let mut clicked = None;
        if wide {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), total_plot_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.columns(2, |columns| {
                        for (column, kind) in columns.iter_mut().zip([
                            DegradationPlot::ThresholdVoltage,
                            DegradationPlot::RelativeShifts,
                        ]) {
                            column.set_min_height(total_plot_height);
                            clicked = show_degradation_plot(
                                column,
                                &mut state.ui.results,
                                analysis.id,
                                device,
                                selected_checkpoint,
                                kind,
                            )
                            .or(clicked);
                        }
                    });
                },
            );
        } else {
            let each_height = total_plot_height * 0.5;
            for kind in [
                DegradationPlot::ThresholdVoltage,
                DegradationPlot::RelativeShifts,
            ] {
                let result = ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), each_height),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ui.set_min_height(each_height);
                        show_degradation_plot(
                            ui,
                            &mut state.ui.results,
                            analysis.id,
                            device,
                            selected_checkpoint,
                            kind,
                        )
                    },
                );
                clicked = result.inner.or(clicked);
            }
        }
        if let Some(index) = clicked {
            let checkpoint = &device.checkpoints[index];
            requested = Some(ReliabilitySelection {
                analysis: analysis_key,
                device_id: device.device_id.clone(),
                checkpoint_year_bits: checkpoint.years.to_bits(),
            });
        }
    }

    let width = ui.available_width().max(1_090.0);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.reliability-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(width);
            TableBuilder::new(ui)
                .id_salt("rspice.results.reliability")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::remainder().at_least(180.0))
                .column(Column::initial(112.0))
                .column(Column::initial(142.0))
                .column(Column::initial(142.0))
                .column(Column::initial(142.0))
                .column(Column::initial(142.0))
                .column(Column::initial(90.0))
                .header(31.0, |mut header| {
                    for label in [
                        "DEVICE",
                        "YEARS",
                        "Î”VTH",
                        "Î”MOBILITY",
                        "Î”RDS",
                        "AVG. TEMP",
                        "STATE",
                    ] {
                        header.col(|ui| table_header(ui, label));
                    }
                })
                .body(|mut body| {
                    for device in devices {
                        for checkpoint in &device.checkpoints {
                            let is_selected = selected.as_ref().is_some_and(|selection| {
                                selection.analysis == analysis_key
                                    && selection.device_id == device.device_id
                                    && selection.checkpoint_year_bits == checkpoint.years.to_bits()
                            });
                            body.row(29.0, |mut row| {
                                row.set_selected(is_selected);
                                row.col(|ui| {
                                    if ui
                                        .selectable_label(
                                            is_selected,
                                            RichText::new(&device.device_id).monospace(),
                                        )
                                        .clicked()
                                    {
                                        requested = Some(ReliabilitySelection {
                                            analysis: analysis_key,
                                            device_id: device.device_id.clone(),
                                            checkpoint_year_bits: checkpoint.years.to_bits(),
                                        });
                                    }
                                });
                                row.col(|ui| mono(ui, &format!("{:.9e}", checkpoint.years)));
                                row.col(|ui| {
                                    mono(
                                        ui,
                                        &format!(
                                            "{:+.9e} V",
                                            checkpoint.shift.threshold_voltage_shift_v
                                        ),
                                    )
                                });
                                row.col(|ui| {
                                    mono(ui, &format!("{:+.9e}", checkpoint.shift.mobility_shift))
                                });
                                row.col(|ui| {
                                    mono(
                                        ui,
                                        &format!(
                                            "{:+.9e}",
                                            checkpoint.shift.drain_source_resistance_shift
                                        ),
                                    )
                                });
                                row.col(|ui| {
                                    mono(
                                        ui,
                                        &format!("{:.6} K", device.stress.average_temperature_k),
                                    )
                                });
                                row.col(|ui| retained_badge(ui));
                            });
                        }
                    }
                });
        });
    if let Some(selection) = requested {
        let device_changed = selected
            .as_ref()
            .is_none_or(|current| current.device_id != selection.device_id);
        if device_changed {
            state
                .ui
                .results
                .reset_plot_view(super::ResultViewer::Reliability, 0);
            state
                .ui
                .results
                .reset_plot_view(super::ResultViewer::Reliability, 1);
        }
        state.ui.results.select_reliability(selection);
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_reliability.clone() else {
        section_header(ui, "Reliability selection", None);
        panel_note(
            ui,
            "Select a checkpoint row to inspect its exact retained shift and stress conditions.",
        );
        return;
    };
    let Some(run) = state.simulation.active_run() else {
        state.ui.results.selected_reliability = None;
        section_header(ui, "Reliability selection", None);
        panel_note(ui, "Select a retained reliability analysis and checkpoint.");
        return;
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        state.ui.results.selected_reliability = None;
        section_header(ui, "Reliability selection", None);
        panel_note(
            ui,
            "The selected checkpoint no longer belongs to the active retained dataset.",
        );
        return;
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        state.ui.results.selected_reliability = None;
        section_header(ui, "Reliability selection", None);
        panel_note(
            ui,
            "Select a reliability checkpoint in the active analysis.",
        );
        return;
    }
    let Some(AnalysisResultPayload::Reliability { devices }) = analysis.result_payload.as_ref()
    else {
        state.ui.results.selected_reliability = None;
        section_header(ui, "Reliability selection", None);
        panel_note(
            ui,
            "The active analysis no longer contains retained reliability evidence.",
        );
        return;
    };
    let Some(device) = devices
        .iter()
        .find(|device| device.device_id == selection.device_id)
    else {
        state.ui.results.selected_reliability = None;
        section_header(ui, "Reliability selection", None);
        panel_note(ui, "The selected reliability device is no longer retained.");
        return;
    };
    let Some(checkpoint) = device
        .checkpoints
        .iter()
        .find(|checkpoint| checkpoint.years.to_bits() == selection.checkpoint_year_bits)
    else {
        state.ui.results.selected_reliability = None;
        section_header(ui, "Reliability selection", None);
        panel_note(
            ui,
            "The selected reliability checkpoint is no longer retained.",
        );
        return;
    };

    section_header(ui, "Selected reliability checkpoint", Some("RETAINED"));
    let rows = vec![
        ("Device", device.device_id.clone(), true),
        ("Lifetime", format!("{:.17e} years", checkpoint.years), true),
        (
            "Threshold shift",
            format!("{:+.17e} V", checkpoint.shift.threshold_voltage_shift_v),
            true,
        ),
        (
            "Mobility shift",
            format!("{:+.17e}", checkpoint.shift.mobility_shift),
            false,
        ),
        (
            "RDS shift",
            format!("{:+.17e}", checkpoint.shift.drain_source_resistance_shift),
            false,
        ),
        (
            "Gate stress",
            format!("{:.17e} V", device.stress.average_gate_stress_v),
            false,
        ),
        (
            "Drain stress",
            format!("{:.17e} V", device.stress.average_drain_stress_v),
            false,
        ),
        (
            "Temperature",
            format!("{:.17e} K", device.stress.average_temperature_k),
            false,
        ),
        (
            "Stress duration",
            format!("{:.17e} s", device.stress.duration_s),
            false,
        ),
        ("Checkpoints", device.checkpoints.len().to_string(), false),
    ];
    stat_table(ui, &rows);
    panel_note(
        ui,
        "No pass/fail or predicted-life claim is inferred: this panel reports only the retained stress and shift evidence produced by the analysis.",
    );
}

fn table_header(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
}

fn mono(ui: &mut Ui, text: &str) {
    ui.label(RichText::new(text).font(theme::mono(tokens::FS_0, FontWeight::Regular)));
}

fn retained_badge(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    ui.label(RichText::new("RETAINED").strong().color(t.color.ok));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        ReliabilityCheckpointEvidence, ReliabilityShiftEvidence, ReliabilityStressEvidence,
    };

    fn checkpoint(years: f64) -> ReliabilityCheckpointEvidence {
        ReliabilityCheckpointEvidence {
            years,
            shift: ReliabilityShiftEvidence {
                threshold_voltage_shift_v: years * 0.01,
                mobility_shift: -years * 0.02,
                drain_source_resistance_shift: years * 0.03,
            },
        }
    }

    #[test]
    fn degradation_plot_clicks_snap_to_retained_checkpoint() {
        let device = ReliabilityDeviceEvidence {
            device_id: "M1".to_owned(),
            stress: ReliabilityStressEvidence {
                average_gate_stress_v: 1.0,
                average_drain_stress_v: 1.0,
                average_temperature_k: 350.0,
                duration_s: 1.0,
            },
            checkpoints: vec![checkpoint(1.0), checkpoint(5.0), checkpoint(10.0)],
        };
        assert_eq!(nearest_checkpoint_index(&device, 4.8), Some(1));
        assert_eq!(nearest_checkpoint_index(&device, 9.9), Some(2));
    }

    #[test]
    fn degradation_ranges_include_zero_without_collapsing() {
        let (minimum, maximum) = padded_range(&[2.0, 2.0], true).expect("finite range");
        assert!(minimum < 0.0);
        assert!(maximum > 2.0);
    }

    #[test]
    fn lifetime_axis_never_invents_negative_age() {
        assert_eq!(lifetime_range(&[1.0, 5.0, 10.0]), Some((0.0, 10.5)));
        assert_eq!(lifetime_range(&[-1.0, 1.0]), None);
    }
}
