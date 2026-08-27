//! Safe-operating-area rule evidence viewer.

use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisResultPayload, AnalysisType,
    SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, WaveformData,
};
use crate::ui::plot::{Axis, LimitLine, Marker, PlotSpec, Trace, XScale};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::frame_work::{self, DatasetWalk};
use super::strip::StripHeader;
use super::{
    AnalysisPresentationKey, SoaRuleFilter, SoaRuleSelection, panel_note, stat_table, well_hint,
};

const ROW_HEIGHT: f32 = 29.0;

fn active_soa(
    simulation: &crate::state::SimulationState,
) -> Option<(&AnalysisResult, &[SoaEvaluationEvidence], usize)> {
    let analysis = simulation.active_analysis()?;
    let payload = analysis.result_payload.as_ref()?;
    let AnalysisResultPayload::Soa {
        evaluations,
        violations,
    } = payload
    else {
        return None;
    };
    if !analysis.success || analysis.analysis_type != AnalysisType::Soa || {
        frame_work::note(DatasetWalk::EvidenceValidation);
        analysis.validate_retained_evidence().is_err()
    } {
        return None;
    }
    Some((analysis, evaluations, violations.len()))
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    active_soa(&state.simulation).is_some()
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(dataset_id) = state.simulation.active_run().map(|run| run.dataset_id) else {
        well_hint(ui, "Select a dataset with retained SOA evidence");
        return;
    };
    let Some((analysis, evaluations, violation_count)) = active_soa(&state.simulation) else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
    let analysis_key = AnalysisPresentationKey::new(dataset_id, analysis);
    let selected = state.ui.results.selected_soa_rule.clone();
    let initial_filter = state.ui.results.soa_rule_filter;
    let mut filter = initial_filter;
    let mut trace_open = state.ui.results.soa_stress_trace_open;
    let mut requested = selected.clone();
    let mut cross_probe = None;

    let stress_view = state.ui.results.plot_view(super::ResultViewer::Soa, 0);
    let header = StripHeader::new(
        "SOA",
        &format!(
            "{} · {} evaluated rules · {} retained violation events",
            analysis.label,
            evaluations.len(),
            violation_count
        ),
        &[],
    )
    .zoomed(trace_open && stress_view.is_zoomed())
    .show(ui);
    if header.fit_clicked {
        state
            .ui
            .results
            .reset_plot_view(super::ResultViewer::Soa, 0);
    }

    let attention_count = evaluations
        .iter()
        .filter(|evaluation| SoaRuleFilter::Violations.matches(evaluation.verdict))
        .count();
    let passing_count = evaluations.len().saturating_sub(attention_count);
    egui::Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .stroke(egui::Stroke::new(1.0, Tokens::get(ui.ctx()).color.border))
        .inner_margin(egui::Margin::symmetric(10, 6))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("SHOW").strong());
                egui::ComboBox::from_id_salt("rspice.results.soa-filter")
                    .selected_text(filter.label())
                    .show_ui(ui, |ui| {
                        for candidate in SoaRuleFilter::ALL {
                            ui.selectable_value(&mut filter, candidate, candidate.label());
                        }
                    });
                ui.separator();
                ui.label(format!(
                    "{attention_count} attention · {passing_count} passing · {} total",
                    evaluations.len()
                ));
                if trace_open {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("Close stress trace").clicked() {
                            trace_open = false;
                        }
                    });
                }
            });
        });

    if filter != initial_filter
        && let Some(selection) = requested.as_ref()
        && let Some(evaluation) = evaluations.iter().find(|evaluation| {
            selection.analysis == analysis_key
                && selection.device_id == evaluation.device_id
                && selection.parameter == evaluation.parameter
        })
        && !filter.matches(evaluation.verdict)
    {
        requested = None;
        trace_open = false;
    }

    if trace_open
        && let Some(selection) = requested.as_ref()
        && selection.analysis == analysis_key
        && let Some(evaluation) = evaluations.iter().find(|evaluation| {
            selection.device_id == evaluation.device_id
                && selection.parameter == evaluation.parameter
        })
    {
        if let Some(waveform) = stress_waveform(analysis, evaluation) {
            stress_trace_card(ui, &mut state.ui.results, waveform, evaluation);
        } else {
            legacy_stress_history_note(ui);
        }
    }

    let visible_evaluations = evaluations
        .iter()
        .filter(|evaluation| filter.matches(evaluation.verdict))
        .collect::<Vec<_>>();

    let width = ui.available_width().max(1_270.0);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.soa-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(width);
            TableBuilder::new(ui)
                .id_salt("rspice.results.soa")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::remainder().at_least(170.0))
                .column(Column::remainder().at_least(190.0))
                .column(Column::initial(118.0))
                .column(Column::initial(118.0))
                .column(Column::initial(118.0))
                .column(Column::initial(190.0))
                .column(Column::initial(92.0))
                .column(Column::initial(210.0))
                .header(31.0, |mut header| {
                    for label in [
                        "DEVICE",
                        "RULE",
                        "OBSERVED",
                        "LIMIT",
                        "MARGIN",
                        "WORST INTERVAL",
                        "STATUS",
                        "OPEN",
                    ] {
                        header.col(|ui| table_header(ui, label));
                    }
                })
                .body(|mut body| {
                    for &evaluation in &visible_evaluations {
                        let is_selected = selected.as_ref().is_some_and(|selection| {
                            selection.analysis == analysis_key
                                && selection.device_id == evaluation.device_id
                                && selection.parameter == evaluation.parameter
                        });
                        body.row(ROW_HEIGHT, |mut row| {
                            row.set_selected(is_selected);
                            row.col(|ui| {
                                if ui
                                    .selectable_label(
                                        is_selected,
                                        RichText::new(&evaluation.device_id).monospace(),
                                    )
                                    .clicked()
                                {
                                    requested = Some(SoaRuleSelection {
                                        analysis: analysis_key,
                                        device_id: evaluation.device_id.clone(),
                                        parameter: evaluation.parameter,
                                    });
                                }
                            });
                            row.col(|ui| {
                                ui.label(parameter_label(evaluation.parameter));
                            });
                            row.col(|ui| {
                                exact_quantity(ui, evaluation.worst_actual_value, &evaluation.unit)
                            });
                            row.col(|ui| {
                                exact_quantity(ui, evaluation.limit_value, &evaluation.unit)
                            });
                            row.col(|ui| {
                                exact_quantity(
                                    ui,
                                    evaluation.limit_value - evaluation.worst_actual_value,
                                    &evaluation.unit,
                                )
                            });
                            row.col(|ui| {
                                mono(ui, &worst_interval_text(analysis, evaluation, true))
                            });
                            row.col(|ui| verdict(ui, evaluation.verdict));
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    let trace_available =
                                        stress_waveform(analysis, evaluation).is_some();
                                    let response = ui
                                        .add_enabled(
                                            trace_available,
                                            egui::Button::new("Stress trace"),
                                        )
                                        .on_disabled_hover_text(
                                            "This dataset predates exact SOA stress-history retention; run the analysis again.",
                                        );
                                    if response.clicked() {
                                        requested = Some(SoaRuleSelection {
                                            analysis: analysis_key,
                                            device_id: evaluation.device_id.clone(),
                                            parameter: evaluation.parameter,
                                        });
                                        trace_open = true;
                                    }

                                    let schematic_available = soa_device_target(
                                        state,
                                        analysis_key,
                                        &evaluation.device_id,
                                    )
                                    .is_some();
                                    let response = ui
                                        .add_enabled(
                                            schematic_available,
                                            egui::Button::new("Schematic"),
                                        )
                                        .on_disabled_hover_text(
                                            "The result is stale or the retained device has no exact identity in the active schematic.",
                                        );
                                    if response.clicked() {
                                        let selection = SoaRuleSelection {
                                            analysis: analysis_key,
                                            device_id: evaluation.device_id.clone(),
                                            parameter: evaluation.parameter,
                                        };
                                        requested = Some(selection.clone());
                                        cross_probe = Some(selection);
                                    }
                                });
                            });
                        });
                    }
                });
        });
    if visible_evaluations.is_empty() {
        well_hint(ui, "No retained SOA rules match the selected filter");
    }

    state.ui.results.soa_rule_filter = filter;
    state.ui.results.soa_stress_trace_open = trace_open;
    if requested != selected {
        if let Some(selection) = requested {
            if trace_open {
                state
                    .ui
                    .results
                    .reset_plot_view(super::ResultViewer::Soa, 0);
            }
            state.ui.results.selected_soa_rule = Some(selection);
        } else {
            state.ui.results.selected_soa_rule = None;
        }
    }
    if let Some(selection) = cross_probe {
        apply_schematic_cross_probe(ui, state, &selection);
    }
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_soa_rule.clone() else {
        section_header(ui, "SOA rule selection", None);
        panel_note(
            ui,
            "Select a rule row to inspect its exact worst point, limit, and coverage.",
        );
        return;
    };
    let Some(run) = state.simulation.active_run() else {
        state.ui.results.selected_soa_rule = None;
        section_header(ui, "SOA rule selection", None);
        panel_note(ui, "Select a retained SOA analysis and rule.");
        return;
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        state.ui.results.selected_soa_rule = None;
        section_header(ui, "SOA rule selection", None);
        panel_note(
            ui,
            "The selected rule no longer belongs to the active retained dataset.",
        );
        return;
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        state.ui.results.selected_soa_rule = None;
        section_header(ui, "SOA rule selection", None);
        panel_note(ui, "Select an SOA rule in the active analysis.");
        return;
    }
    let Some(AnalysisResultPayload::Soa {
        evaluations,
        violations,
    }) = analysis.result_payload.as_ref()
    else {
        state.ui.results.selected_soa_rule = None;
        section_header(ui, "SOA rule selection", None);
        panel_note(
            ui,
            "The active analysis no longer contains retained SOA evidence.",
        );
        return;
    };
    let Some(evaluation) = evaluations.iter().find(|evaluation| {
        evaluation.device_id == selection.device_id && evaluation.parameter == selection.parameter
    }) else {
        state.ui.results.selected_soa_rule = None;
        section_header(ui, "SOA rule selection", None);
        panel_note(ui, "The selected SOA rule is no longer retained.");
        return;
    };
    let event_count = violations
        .iter()
        .filter(|event| {
            event.device_id == evaluation.device_id && event.parameter == evaluation.parameter
        })
        .count();
    let margin = evaluation.limit_value - evaluation.worst_actual_value;

    section_header(ui, "Selected SOA rule", Some(evaluation.verdict.label()));
    stat_table(
        ui,
        &[
            ("Device", evaluation.device_id.clone(), true),
            (
                "Quantity",
                parameter_label(evaluation.parameter).to_owned(),
                false,
            ),
            (
                "Observed",
                format!("{:.17e} {}", evaluation.worst_actual_value, evaluation.unit),
                true,
            ),
            (
                "Limit",
                format!("{:.17e} {}", evaluation.limit_value, evaluation.unit),
                false,
            ),
            (
                "Signed margin",
                format!("{margin:.17e} {}", evaluation.unit),
                true,
            ),
            (
                "Worst time",
                format!("{:.17e} s", evaluation.worst_time_s),
                false,
            ),
            (
                "Worst interval",
                worst_interval_text(analysis, evaluation, false),
                false,
            ),
            (
                "Samples evaluated",
                evaluation.sample_count.to_string(),
                false,
            ),
            ("Violation events", event_count.to_string(), false),
        ],
    );
    panel_note(ui, &evaluation.description);

    let trace_available = stress_waveform(analysis, evaluation).is_some();
    let schematic_available =
        soa_device_target(state, selection.analysis, &evaluation.device_id).is_some();
    let mut open_trace = false;
    let mut locate_device = false;
    ui.horizontal_wrapped(|ui| {
        let response = ui
            .add_enabled(trace_available, egui::Button::new("Open stress trace"))
            .on_disabled_hover_text(
                "This legacy dataset does not retain the complete SOA stress history; run it again.",
            );
        open_trace = response.clicked();
        let response = ui
            .add_enabled(
                schematic_available,
                egui::Button::new("Locate in schematic"),
            )
            .on_disabled_hover_text(
                "The result is stale or the retained device has no exact identity in the active schematic.",
            );
        locate_device = response.clicked();
    });
    if open_trace {
        state.ui.results.soa_stress_trace_open = true;
    }
    if locate_device {
        apply_schematic_cross_probe(ui, state, &selection);
    }
}

impl SoaRuleFilter {
    const ALL: [Self; 3] = [Self::Violations, Self::Passing, Self::All];

    const fn label(self) -> &'static str {
        match self {
            Self::Violations => "Violations",
            Self::Passing => "Passing",
            Self::All => "All rules",
        }
    }

    const fn matches(self, verdict: SoaRuleVerdictEvidence) -> bool {
        match self {
            Self::Violations => !matches!(verdict, SoaRuleVerdictEvidence::Pass),
            Self::Passing => matches!(verdict, SoaRuleVerdictEvidence::Pass),
            Self::All => true,
        }
    }
}

fn stress_waveform<'a>(
    analysis: &'a AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
) -> Option<&'a WaveformData> {
    frame_work::note(DatasetWalk::SoaStressScan);
    let expected_samples = usize::try_from(evaluation.sample_count).ok()?;
    if expected_samples == 0 {
        return None;
    }
    let AnalysisResultFamilyMetadata::Soa { time } = analysis.family_metadata.as_ref()? else {
        return None;
    };
    let name = crate::services::safety::soa_stress_waveform_name(
        &evaluation.device_id,
        runtime_parameter(evaluation.parameter),
    );
    analysis.waveforms.iter().find(|waveform| {
        if waveform.name != name
            || waveform.x.as_slice() != time.as_slice()
            || waveform.x.len() != expected_samples
            || waveform.y.len() != expected_samples
        {
            return false;
        }
        let Some(worst_index) = waveform
            .x
            .iter()
            .position(|value| value.to_bits() == evaluation.worst_time_s.to_bits())
        else {
            return false;
        };
        waveform.y[worst_index].to_bits() == evaluation.worst_actual_value.to_bits()
            && waveform
                .y
                .iter()
                .all(|value| *value <= evaluation.worst_actual_value)
    })
}

fn worst_interval_text(
    analysis: &AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
    compact: bool,
) -> String {
    if evaluation.verdict == SoaRuleVerdictEvidence::Pass {
        return if compact {
            format!("— · worst {:.6e} s", evaluation.worst_time_s)
        } else {
            "Not applicable (rule passed)".to_owned()
        };
    }
    let Some(waveform) = stress_waveform(analysis, evaluation) else {
        return if compact {
            format!("worst {:.6e} s", evaluation.worst_time_s)
        } else {
            "Complete interval was not retained; rerun the analysis".to_owned()
        };
    };
    let worst_index = nearest_sample_index(waveform.x.as_slice(), evaluation.worst_time_s);
    let threshold = if evaluation.verdict == SoaRuleVerdictEvidence::Warning {
        evaluation.limit_value * 0.9
    } else {
        evaluation.limit_value
    };
    let Some((start, end)) = active_interval_indices(waveform.y.as_slice(), worst_index, threshold)
    else {
        return if compact {
            format!("worst {:.6e} s", evaluation.worst_time_s)
        } else {
            "Retained history does not reproduce the rule verdict".to_owned()
        };
    };
    let (start, end) = (waveform.x[start], waveform.x[end]);
    if compact {
        format!("{start:.5e}…{end:.5e} s")
    } else {
        let kind = if evaluation.verdict == SoaRuleVerdictEvidence::Warning {
            "Warning band"
        } else {
            "Limit exceedance"
        };
        format!("{kind}: {start:.17e} s to {end:.17e} s")
    }
}

fn active_interval_indices(
    values: &[f64],
    worst_index: usize,
    threshold: f64,
) -> Option<(usize, usize)> {
    if worst_index >= values.len() || values[worst_index] <= threshold {
        return None;
    }
    let mut start = worst_index;
    while start > 0 && values[start - 1] > threshold {
        start -= 1;
    }
    let mut end = worst_index;
    while end + 1 < values.len() && values[end + 1] > threshold {
        end += 1;
    }
    Some((start, end))
}

const fn runtime_parameter(
    parameter: SoaParameterEvidence,
) -> crate::services::safety::SoAParameter {
    use crate::services::safety::SoAParameter;
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => SoAParameter::Vgs,
        SoaParameterEvidence::DrainSourceVoltage => SoAParameter::Vds,
        SoaParameterEvidence::GateDrainVoltage => SoAParameter::Vgd,
        SoaParameterEvidence::BaseEmitterVoltage => SoAParameter::Vbe,
        SoaParameterEvidence::CollectorEmitterVoltage => SoAParameter::Vce,
        SoaParameterEvidence::BaseCollectorVoltage => SoAParameter::Vbc,
        SoaParameterEvidence::DrainCurrent => SoAParameter::Id,
        SoaParameterEvidence::CollectorCurrent => SoAParameter::Ic,
        SoaParameterEvidence::PowerDissipation => SoAParameter::Pdiss,
        SoaParameterEvidence::Temperature => SoAParameter::Temp,
    }
}

fn result_mapping_is_current(state: &AppState, analysis_key: AnalysisPresentationKey) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    analysis_key.resolve(run).is_some()
        && run
            .prepared_receipt()
            .is_some_and(|receipt| receipt.project_revision() == state.workspace.project.revision())
        && state.simulation.cross_probe.is_current_for(
            &state.workspace.active_view,
            state.schematic.topology_version(),
        )
}

fn soa_device_target(
    state: &AppState,
    analysis_key: AnalysisPresentationKey,
    device_id: &str,
) -> Option<(u64, crate::state::Point)> {
    if !result_mapping_is_current(state, analysis_key) {
        return None;
    }
    state
        .schematic
        .components
        .iter()
        .find(|component| {
            component
                .spice_instance_name()
                .eq_ignore_ascii_case(device_id)
        })
        .map(|component| (component.id, component.pos))
}

fn apply_schematic_cross_probe(ui: &Ui, state: &mut AppState, selection: &SoaRuleSelection) {
    let Some((component_id, position)) =
        soa_device_target(state, selection.analysis, &selection.device_id)
    else {
        state.ui.toasts.warn_with_title(
            ui.ctx(),
            "Cannot locate SOA device",
            "The selected result is stale or its retained device no longer resolves to the active schematic revision.",
        );
        return;
    };
    state
        .schematic
        .selection
        .select_only_component(component_id);
    state.schematic.center_request = Some(position);
    state.ui.schematic_visibility.annotations =
        crate::state::SchematicAnnotationVisibility::ViolationsOnly;
    state
        .workbench
        .activate(crate::workbench::state::Workspace::Design);
}

fn legacy_stress_history_note(ui: &mut Ui) {
    egui::Frame::new()
        .fill(Tokens::get(ui.ctx()).color.bg_panel)
        .stroke(egui::Stroke::new(
            1.0,
            Tokens::get(ui.ctx()).color.border,
        ))
        .inner_margin(egui::Margin::symmetric(11, 8))
        .show(ui, |ui| {
            ui.label(RichText::new("Stress history unavailable").strong());
            ui.label(
                "This legacy dataset retains the exact worst point but not the complete rule history. Run the SOA analysis again to enable the linked trace.",
            );
        });
}

fn stress_trace_card(
    ui: &mut Ui,
    results: &mut super::ResultsState,
    waveform: &WaveformData,
    evaluation: &SoaEvaluationEvidence,
) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .fill(t.color.canvas_bg)
        .stroke(egui::Stroke::new(1.0, t.color.border_strong))
        .inner_margin(egui::Margin::symmetric(8, 7))
        .show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.label(
                    RichText::new(format!(
                        "{} · {}",
                        evaluation.device_id,
                        parameter_label(evaluation.parameter)
                    ))
                    .strong(),
                );
                ui.separator();
                mono(
                    ui,
                    &format!(
                        "worst {:.9e} {} at {:.9e} s · limit {:.9e} {} · {} exact samples",
                        evaluation.worst_actual_value,
                        evaluation.unit,
                        evaluation.worst_time_s,
                        evaluation.limit_value,
                        evaluation.unit,
                        waveform.x.len()
                    ),
                );
            });

            let (x_min, x_max) = padded_range(waveform.x.iter().copied(), None);
            let (y_min, y_max) = padded_range(
                waveform.y.iter().copied(),
                Some([0.0, evaluation.limit_value, evaluation.worst_actual_value]),
            );
            let x_min = x_min.max(0.0);
            let y_min = y_min.max(0.0);
            let view = results.plot_view(super::ResultViewer::Soa, 0);
            let (x_min, x_max) = view.x.unwrap_or((x_min, x_max));
            let (y_min, y_max) = view.y.unwrap_or((y_min, y_max));
            let title = format!(
                "SOA stress history for {} {}",
                evaluation.device_id,
                parameter_label(evaluation.parameter)
            );
            let detail = format!(
                "Exact retained stress samples with a limit of {:.17e} {} and worst point {:.17e} {} at {:.17e} seconds.",
                evaluation.limit_value,
                evaluation.unit,
                evaluation.worst_actual_value,
                evaluation.unit,
                evaluation.worst_time_s
            );
            let mut spec = PlotSpec::new(
                Axis::linear(x_min, x_max, "s").with_label("Time"),
                XScale::Linear,
                Axis::linear(y_min, y_max, &evaluation.unit)
                    .with_label(parameter_label(evaluation.parameter)),
            )
            .accessible_name(&title)
            .accessible_detail(&detail);
            spec.traces.push(Trace::new(
                waveform.x.as_slice(),
                waveform.y.as_slice(),
                t.color.traces[0],
            ));
            spec.limit_lines.push(LimitLine {
                y: evaluation.limit_value,
                color: t.color.err,
                label: format!("LIMIT {:.6e} {}", evaluation.limit_value, evaluation.unit),
            });
            spec.markers.push(Marker::point(
                evaluation.worst_time_s,
                evaluation.worst_actual_value,
                t.color.accent,
                "WORST",
            ));
            let readout = |hover_x: f64| {
                let index = nearest_sample_index(waveform.x.as_slice(), hover_x);
                vec![
                    ("t".to_owned(), format!("{:.17e} s", waveform.x[index])),
                    (
                        parameter_label(evaluation.parameter).to_owned(),
                        format!("{:.17e} {}", waveform.y[index], evaluation.unit),
                    ),
                    (
                        "Margin".to_owned(),
                        format!(
                            "{:.17e} {}",
                            evaluation.limit_value - waveform.y[index],
                            evaluation.unit
                        ),
                    ),
                ]
            };
            let response = ui
                .allocate_ui(egui::vec2(ui.available_width(), 210.0), |ui| {
                    crate::ui::plot::show(
                        ui,
                        &spec,
                        &mut results.cache,
                        None,
                        Some(&readout),
                    )
                })
                .inner;
            if response.view.any() {
                results
                    .plot_view_mut(super::ResultViewer::Soa, 0)
                    .apply(&response.view);
            }
        });
}

fn padded_range(values: impl Iterator<Item = f64>, include: Option<[f64; 3]>) -> (f64, f64) {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for value in values.chain(include.into_iter().flatten()) {
        if value.is_finite() {
            min = min.min(value);
            max = max.max(value);
        }
    }
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0);
    }
    if min == max {
        let pad = min.abs().max(1.0) * 0.05;
        return (min - pad, max + pad);
    }
    let pad = (max - min) * 0.06;
    (min - pad, max + pad)
}

fn nearest_sample_index(x: &[f64], query: f64) -> usize {
    debug_assert!(!x.is_empty());
    let upper = x.partition_point(|value| *value < query).min(x.len() - 1);
    if upper == 0 || (x[upper] - query).abs() < (query - x[upper - 1]).abs() {
        upper
    } else {
        upper - 1
    }
}

fn parameter_label(parameter: SoaParameterEvidence) -> &'static str {
    match parameter {
        SoaParameterEvidence::GateSourceVoltage => "Gate-source voltage",
        SoaParameterEvidence::DrainSourceVoltage => "Drain-source voltage",
        SoaParameterEvidence::GateDrainVoltage => "Gate-drain voltage",
        SoaParameterEvidence::BaseEmitterVoltage => "Base-emitter voltage",
        SoaParameterEvidence::CollectorEmitterVoltage => "Collector-emitter voltage",
        SoaParameterEvidence::BaseCollectorVoltage => "Base-collector voltage",
        SoaParameterEvidence::DrainCurrent => "Drain current",
        SoaParameterEvidence::CollectorCurrent => "Collector current",
        SoaParameterEvidence::PowerDissipation => "Power dissipation",
        SoaParameterEvidence::Temperature => "Temperature",
    }
}

fn verdict(ui: &mut Ui, value: SoaRuleVerdictEvidence) {
    let t = Tokens::get(ui.ctx());
    let color = match value {
        SoaRuleVerdictEvidence::Pass => t.color.ok,
        SoaRuleVerdictEvidence::Warning => t.color.warn,
        SoaRuleVerdictEvidence::Violation | SoaRuleVerdictEvidence::Critical => t.color.err,
    };
    ui.label(RichText::new(value.label()).strong().color(color));
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

fn exact_quantity(ui: &mut Ui, value: f64, unit: &str) {
    mono(ui, &format!("{value:.9e} {unit}"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soa_filter_keeps_attention_and_passing_rules_disjoint() {
        for verdict in [
            SoaRuleVerdictEvidence::Warning,
            SoaRuleVerdictEvidence::Violation,
            SoaRuleVerdictEvidence::Critical,
        ] {
            assert!(SoaRuleFilter::Violations.matches(verdict));
            assert!(!SoaRuleFilter::Passing.matches(verdict));
            assert!(SoaRuleFilter::All.matches(verdict));
        }
        assert!(SoaRuleFilter::Passing.matches(SoaRuleVerdictEvidence::Pass));
        assert!(!SoaRuleFilter::Violations.matches(SoaRuleVerdictEvidence::Pass));
        assert!(SoaRuleFilter::All.matches(SoaRuleVerdictEvidence::Pass));
    }

    #[test]
    fn evidence_parameters_map_to_stable_stress_waveform_names() {
        let cases = [
            (SoaParameterEvidence::GateSourceVoltage, "SOA_VGS(M1)"),
            (SoaParameterEvidence::DrainSourceVoltage, "SOA_VDS(M1)"),
            (SoaParameterEvidence::GateDrainVoltage, "SOA_VGD(M1)"),
            (SoaParameterEvidence::BaseEmitterVoltage, "SOA_VBE(M1)"),
            (SoaParameterEvidence::CollectorEmitterVoltage, "SOA_VCE(M1)"),
            (SoaParameterEvidence::BaseCollectorVoltage, "SOA_VBC(M1)"),
            (SoaParameterEvidence::DrainCurrent, "SOA_ID(M1)"),
            (SoaParameterEvidence::CollectorCurrent, "SOA_IC(M1)"),
            (SoaParameterEvidence::PowerDissipation, "SOA_PDISS(M1)"),
            (SoaParameterEvidence::Temperature, "SOA_TEMP(M1)"),
        ];
        for (parameter, expected) in cases {
            assert_eq!(
                crate::services::safety::soa_stress_waveform_name(
                    "M1",
                    runtime_parameter(parameter)
                ),
                expected
            );
        }
    }

    #[test]
    fn nearest_sample_selection_is_exact_and_deterministic() {
        let x = [0.0, 1.0, 2.0, 4.0];
        assert_eq!(nearest_sample_index(&x, -1.0), 0);
        assert_eq!(nearest_sample_index(&x, 1.5), 1);
        assert_eq!(nearest_sample_index(&x, 3.9), 3);
        assert_eq!(nearest_sample_index(&x, 9.0), 3);
    }

    #[test]
    fn worst_interval_expands_only_across_contiguous_active_samples() {
        let values = [0.1, 1.1, 1.3, 0.8, 1.4, 0.7];
        assert_eq!(active_interval_indices(&values, 2, 1.0), Some((1, 2)));
        assert_eq!(active_interval_indices(&values, 4, 1.0), Some((4, 4)));
        assert_eq!(active_interval_indices(&values, 3, 1.0), None);
    }
}
