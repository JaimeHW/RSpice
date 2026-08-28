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

/// The active SOA analysis, if it is one and its evidence validated.
///
/// The validation goes through the workspace memo rather than the validator:
/// the tab strip asks this on every frame to decide whether to offer the
/// sheet, and the validator walks every retained stress sample in the run.
fn active_soa(
    simulation: &crate::state::SimulationState,
    evidence_is_valid: bool,
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
    if !analysis.success || analysis.analysis_type != AnalysisType::Soa || !evidence_is_valid {
        return None;
    }
    Some((analysis, evaluations, violations.len()))
}

/// The memoized retained-evidence verdict for whichever analysis is active.
fn active_evidence_is_valid(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    state
        .simulation
        .active_analysis()
        .is_some_and(|analysis| super::analysis_evidence_is_valid(state, run.dataset_id, analysis))
}

pub(super) fn active_payload_is_valid(state: &AppState) -> bool {
    active_soa(&state.simulation, active_evidence_is_valid(state)).is_some()
}

/// What one rule's retained stress history says, once someone has read it.
///
/// Both facts cost a scan: locating the history means comparing a candidate
/// waveform's complete time axis and values against the rule's evidence, and
/// the interval means walking outward from the worst sample until the stress
/// drops back under the limit. The table asked for both on every row of every
/// frame, twice — once for the cell and once to decide whether the row's
/// button could be pressed.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct SoaRuleFacts {
    /// Index into the analysis' waveforms of this rule's verified stress
    /// history, when one is retained.
    stress_waveform: Option<usize>,
    /// The worst-interval cell, as the table prints it.
    interval_compact: String,
    /// The same interval, as the inspector prints it.
    interval_full: String,
    /// The stress card's padded axis extents. Both walk the whole history,
    /// and the card asked for them on every frame it was open.
    stress_axes: Option<((f64, f64), (f64, f64))>,
    /// Decimation-cache identity for the stress polyline. Without one the
    /// renderer re-reduces every retained sample per frame.
    stress_cache_key: u64,
}

/// Every rule's scanned facts, built once per retained SOA analysis.
///
/// The visible-row lists are here too, one per filter. They cost no scan,
/// but the table reads one of them by ordinal every frame and building them
/// alongside the facts keeps the row a viewport names addressable in O(1).
#[derive(Debug, Clone, PartialEq)]
pub(super) struct SoaPlan {
    version: u64,
    analysis: AnalysisPresentationKey,
    rules: Vec<SoaRuleFacts>,
    visible: [Vec<usize>; SoaRuleFilter::ALL.len()],
}

impl SoaPlan {
    fn facts(&self, rule: usize) -> Option<&SoaRuleFacts> {
        self.rules.get(rule)
    }

    /// The rules one filter shows, in retained order.
    fn visible(&self, filter: SoaRuleFilter) -> &[usize] {
        &self.visible[filter.ordinal()]
    }
}

fn build_soa_plan(
    version: u64,
    analysis_key: AnalysisPresentationKey,
    analysis: &AnalysisResult,
    evaluations: &[SoaEvaluationEvidence],
) -> SoaPlan {
    let identity = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        version.hash(&mut hasher);
        analysis_key.hash(&mut hasher);
        hasher.finish()
    };
    let rules: Vec<SoaRuleFacts> = evaluations
        .iter()
        .enumerate()
        .map(|(rule, evaluation)| {
            let stress = stress_waveform(analysis, evaluation);
            SoaRuleFacts {
                stress_waveform: stress.and_then(|found| {
                    analysis
                        .waveforms
                        .iter()
                        .position(|w| std::ptr::eq(w, found))
                }),
                interval_compact: worst_interval_text(stress, evaluation, true),
                interval_full: worst_interval_text(stress, evaluation, false),
                stress_axes: stress.map(|waveform| {
                    let (x_min, x_max) = padded_range(waveform.x.iter().copied(), None);
                    let (y_min, y_max) = padded_range(
                        waveform.y.iter().copied(),
                        Some([0.0, evaluation.limit_value, evaluation.worst_actual_value]),
                    );
                    ((x_min.max(0.0), x_max), (y_min.max(0.0), y_max))
                }),
                stress_cache_key: identity ^ (rule as u64).wrapping_mul(0x9E37_79B9_7F4A_7C15),
            }
        })
        .collect();
    let visible = SoaRuleFilter::ALL.map(|filter| {
        evaluations
            .iter()
            .enumerate()
            .filter(|(_, evaluation)| filter.matches(evaluation.verdict))
            .map(|(index, _)| index)
            .collect()
    });
    SoaPlan {
        version,
        analysis: analysis_key,
        rules,
        visible,
    }
}

/// The scanned facts for the active SOA analysis, rebuilding them only when
/// the dataset generation or the selected analysis changes.
///
/// The build reads whichever analysis is active, so a key naming a different
/// one is refused rather than stamped onto the active analysis' facts: a memo
/// entry whose key and content disagree is a wrong number waiting for the
/// caller that reads it. The right panel asks with the selection's key before
/// it has checked the selection against the active analysis, and that is
/// exactly the case this closes.
fn soa_plan(
    state: &mut AppState,
    analysis_key: AnalysisPresentationKey,
    evidence_is_valid: bool,
) -> Option<std::sync::Arc<SoaPlan>> {
    let version = state.simulation.data_version;
    if let Some(plan) = state.ui.results.plans.soa.as_ref()
        && plan.version == version
        && plan.analysis == analysis_key
    {
        return Some(std::sync::Arc::clone(plan));
    }
    let dataset_id = state.simulation.active_run()?.dataset_id;
    let (analysis, evaluations, _) = active_soa(&state.simulation, evidence_is_valid)?;
    if AnalysisPresentationKey::new(dataset_id, analysis) != analysis_key {
        return None;
    }
    let built = std::sync::Arc::new(build_soa_plan(version, analysis_key, analysis, evaluations));
    state.ui.results.plans.soa = Some(std::sync::Arc::clone(&built));
    Some(built)
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some(dataset_id) = state.simulation.active_run().map(|run| run.dataset_id) else {
        well_hint(ui, "Select a dataset with retained SOA evidence");
        return;
    };
    let evidence_is_valid = active_evidence_is_valid(state);
    let Some(analysis_key) = state
        .simulation
        .active_analysis()
        .map(|analysis| AnalysisPresentationKey::new(dataset_id, analysis))
    else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
    // Built before the retained evidence is borrowed, so the scan happens at
    // most once per dataset generation rather than once per row per frame.
    let Some(plan) = soa_plan(state, analysis_key, evidence_is_valid) else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
    let Some((analysis, evaluations, violation_count)) =
        active_soa(&state.simulation, evidence_is_valid)
    else {
        well_hint(ui, "Select a validated safe-operating-area analysis");
        return;
    };
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

    let attention_count = plan.visible(SoaRuleFilter::Violations).len();
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
        && let Some((rule, evaluation)) = evaluations.iter().enumerate().find(|(_, evaluation)| {
            selection.device_id == evaluation.device_id
                && selection.parameter == evaluation.parameter
        })
    {
        // The plan already located and verified this rule's history; drawing
        // it must not repeat that scan on every frame the card is open.
        if let Some((facts, waveform)) = plan.facts(rule).and_then(|facts| {
            facts
                .stress_waveform
                .and_then(|index| analysis.waveforms.get(index))
                .map(|waveform| (facts, waveform))
        }) {
            stress_trace_card(ui, &mut state.ui.results, waveform, evaluation, facts);
        } else {
            legacy_stress_history_note(ui);
        }
    }

    let visible_rules = plan.visible(filter);

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
                // Only the rows the viewport can show. A real block SOAs
                // thousands of rules, and every row here costs a schematic
                // lookup for its cross-probe button.
                .body(|body| {
                    body.rows(ROW_HEIGHT, visible_rules.len(), |mut row| {
                        let rule = visible_rules[row.index()];
                        let evaluation = &evaluations[rule];
                        let facts = plan.facts(rule).expect("one fact set per retained rule");
                        let is_selected = selected.as_ref().is_some_and(|selection| {
                            selection.analysis == analysis_key
                                && selection.device_id == evaluation.device_id
                                && selection.parameter == evaluation.parameter
                        });
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
                        row.col(|ui| exact_quantity(ui, evaluation.limit_value, &evaluation.unit));
                        row.col(|ui| {
                            exact_quantity(
                                ui,
                                evaluation.limit_value - evaluation.worst_actual_value,
                                &evaluation.unit,
                            )
                        });
                        row.col(|ui| mono(ui, &facts.interval_compact));
                        row.col(|ui| verdict(ui, evaluation.verdict));
                        row.col(|ui| {
                            ui.horizontal(|ui| {
                                let response = ui
                                    .add_enabled(
                                        facts.stress_waveform.is_some(),
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

                                let schematic_available =
                                    soa_device_target(state, analysis_key, &evaluation.device_id)
                                        .is_some();
                                let response = ui
                                    .add_enabled(schematic_available, egui::Button::new("Schematic"))
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
                });
        });
    if visible_rules.is_empty() {
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
    // The panel reads the same scanned facts the table does, so opening it
    // does not re-scan the histories the sheet already walked.
    let plan = soa_plan(state, selection.analysis, active_evidence_is_valid(state));
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
    let Some((rule, evaluation)) = evaluations.iter().enumerate().find(|(_, evaluation)| {
        evaluation.device_id == selection.device_id && evaluation.parameter == selection.parameter
    }) else {
        state.ui.results.selected_soa_rule = None;
        section_header(ui, "SOA rule selection", None);
        panel_note(ui, "The selected SOA rule is no longer retained.");
        return;
    };
    let facts = plan.as_ref().and_then(|plan| plan.facts(rule));
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
                facts.map_or_else(
                    || worst_interval_text(None, evaluation, false),
                    |facts| facts.interval_full.clone(),
                ),
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

    let trace_available = facts.is_some_and(|facts| facts.stress_waveform.is_some());
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

    /// Position in [`Self::ALL`], which indexes the plan's visible-row lists.
    const fn ordinal(self) -> usize {
        match self {
            Self::Violations => 0,
            Self::Passing => 1,
            Self::All => 2,
        }
    }

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
    stress: Option<&WaveformData>,
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
    let Some(waveform) = stress else {
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
    facts: &SoaRuleFacts,
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

            // The card's own extents are resolved with the plan; deriving
            // them here walked the whole history twice per frame.
            let ((x_min, x_max), (y_min, y_max)) =
                facts.stress_axes.unwrap_or(((0.0, 1.0), (0.0, 1.0)));
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
            spec.traces.push(
                Trace::new(
                    waveform.x.as_slice(),
                    waveform.y.as_slice(),
                    t.color.traces[0],
                )
                // Without an identity the renderer re-reduces every retained
                // sample on every frame the card is open.
                .cache_key(facts.stress_cache_key),
            );
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

    /// A retained SOA analysis with `rules` warning rules, each carrying its
    /// own verified stress history whose last sample is the worst.
    fn soa_state(rules: usize, samples: usize, peak: f64) -> AppState {
        use crate::state::{SoaViolationEvidence, SoaViolationSeverityEvidence};

        let time: Vec<f64> = (0..samples).map(|index| index as f64 * 1.0e-12).collect();
        let mut waveforms = Vec::new();
        let mut evaluations = Vec::new();
        let mut violations = Vec::new();
        for rule in 0..rules {
            let device_id = format!("M{rule:04}");
            let y: Vec<f64> = (0..samples)
                .map(|index| peak * (index as f64 + 1.0) / samples as f64)
                .collect();
            let worst_actual_value = y[samples - 1];
            let worst_time_s = time[samples - 1];
            waveforms.push(WaveformData::new(
                crate::services::safety::soa_stress_waveform_name(
                    &device_id,
                    crate::services::safety::SoAParameter::Vds,
                ),
                time.clone(),
                y,
                "#00aaff",
            ));
            evaluations.push(SoaEvaluationEvidence {
                device_id: device_id.clone(),
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                worst_actual_value,
                worst_time_s,
                sample_count: samples as u64,
                unit: "V".to_owned(),
                description: "Maximum drain-source voltage".to_owned(),
                verdict: SoaRuleVerdictEvidence::Warning,
            });
            violations.push(SoaViolationEvidence {
                device_id,
                parameter: SoaParameterEvidence::DrainSourceVoltage,
                limit_value: 3.3,
                actual_value: worst_actual_value,
                time_s: worst_time_s,
                severity: SoaViolationSeverityEvidence::Warning,
            });
        }

        let analysis = AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
            .with_waveforms(waveforms)
            .with_result_payload(AnalysisResultPayload::Soa {
                evaluations,
                violations,
            });
        let mut state = AppState::default();
        let mut run = crate::state::SimulationRun::new(1);
        run.add_analysis(analysis);
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state
    }

    fn active_key(state: &AppState) -> AnalysisPresentationKey {
        let run = state.simulation.active_run().expect("retained run");
        AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0])
    }

    /// The plan is a memo, so it must say exactly what the scan it replaced
    /// said — for every rule, not just the one the fixture looks at.
    #[test]
    fn the_plan_states_what_a_direct_scan_of_each_rule_states() {
        let mut state = soa_state(6, 32, 3.0);
        let key = active_key(&state);
        let plan = soa_plan(&mut state, key, true).expect("a validated SOA plan");

        let analysis = &state.simulation.runs[0].analyses[0];
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = analysis.result_payload.as_ref()
        else {
            panic!("the fixture retains an SOA payload");
        };
        for (rule, evaluation) in evaluations.iter().enumerate() {
            let scanned = stress_waveform(analysis, evaluation);
            let facts = plan.facts(rule).expect("one fact set per rule");
            assert_eq!(facts.stress_waveform.is_some(), scanned.is_some(), "{rule}");
            assert_eq!(
                facts.interval_compact,
                worst_interval_text(scanned, evaluation, true),
                "{rule}"
            );
            assert_eq!(
                facts.interval_full,
                worst_interval_text(scanned, evaluation, false),
                "{rule}"
            );
            let expected_axes = scanned.map(|waveform| {
                let (x_min, x_max) = padded_range(waveform.x.iter().copied(), None);
                let (y_min, y_max) = padded_range(
                    waveform.y.iter().copied(),
                    Some([0.0, evaluation.limit_value, evaluation.worst_actual_value]),
                );
                ((x_min.max(0.0), x_max), (y_min.max(0.0), y_max))
            });
            assert_eq!(facts.stress_axes, expected_axes, "{rule}");
        }
        // Every rule's stress polyline needs its own decimation identity, or
        // one rule's reduction is served under another rule's name.
        let keys: std::collections::HashSet<u64> = plan
            .rules
            .iter()
            .map(|facts| facts.stress_cache_key)
            .collect();
        assert_eq!(keys.len(), plan.rules.len());
        assert_eq!(plan.visible(SoaRuleFilter::Violations).len(), 6);
        assert_eq!(plan.visible(SoaRuleFilter::Passing).len(), 0);
        assert_eq!(plan.visible(SoaRuleFilter::All).len(), 6);
    }

    /// A memo that survives its dataset is worse than the scan it saved.
    #[test]
    fn a_new_dataset_generation_rebuilds_the_stress_facts() {
        let mut state = soa_state(2, 32, 3.0);
        let key = active_key(&state);
        let before = soa_plan(&mut state, key, true).expect("a validated SOA plan");
        let before_interval = before.facts(0).expect("rule 0").interval_compact.clone();

        // Replace the stress history with one that never crosses the warning
        // band, so the interval the plan reports has to change.
        let flattened: Vec<f64> = vec![0.1; 32];
        state.simulation.runs[0].analyses[0].waveforms[0].y = std::sync::Arc::new(flattened);
        state.simulation.data_version = state.simulation.data_version.wrapping_add(1);

        let after = soa_plan(&mut state, key, true).expect("a validated SOA plan");
        assert_ne!(
            after.facts(0).expect("rule 0").interval_compact,
            before_interval,
            "the plan reported the previous dataset generation's interval"
        );
        assert!(
            after.facts(0).expect("rule 0").stress_waveform.is_none(),
            "the replaced history no longer reproduces the rule, so no trace is offered"
        );
    }

    /// The plan is stamped with an analysis key, and the build reads whichever
    /// analysis is active. If the two are allowed to disagree, the memo holds
    /// one analysis' stress facts under another analysis' name — today nothing
    /// reads that entry, which is exactly the kind of latency a refactor turns
    /// into a wrong number on screen.
    #[test]
    fn a_plan_is_never_keyed_to_an_analysis_it_did_not_read() {
        let mut state = soa_state(2, 32, 3.0);
        let mut donor = soa_state(1, 8, 3.0);
        let second = donor.simulation.runs[0].analyses.remove(0);
        state.simulation.runs[0].add_analysis(second);
        assert_eq!(
            state.simulation.active_analysis_idx,
            Some(0),
            "the first analysis stays active"
        );

        let inactive = {
            let run = state.simulation.active_run().expect("retained run");
            AnalysisPresentationKey::new(run.dataset_id, &run.analyses[1])
        };
        assert_ne!(
            inactive,
            active_key(&state),
            "the fixture retains two distinguishable analyses"
        );

        let plan = soa_plan(&mut state, inactive, true);
        assert!(
            plan.is_none(),
            "a plan was served for an analysis whose evidence the build never read"
        );
        assert!(
            state.ui.results.plans.soa.is_none(),
            "the memo kept an entry stamped with an analysis it did not read"
        );

        // A key check, not a shutdown: the analysis that is active still gets
        // its plan, and it is stamped with its own key.
        let active = active_key(&state);
        let served = soa_plan(&mut state, active, true).expect("a validated SOA plan");
        assert_eq!(served.analysis, active);
    }

    /// The table lays out what the viewport can show, not what the dataset
    /// retains. Counted through the real sheet's accessibility tree, because
    /// that is what a screen reader — and a frame — actually pays for.
    #[test]
    fn the_rule_table_lays_out_only_the_rows_the_viewport_shows() {
        let mut state = soa_state(400, 8, 3.0);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1_680.0, 1_020.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut state));
            },
        );
        let drawn = output
            .platform_output
            .accesskit_update
            .expect("the SOA sheet publishes an accessibility tree")
            .nodes
            .iter()
            .filter(|(_, node)| node.label() == Some("Stress trace"))
            .count();
        assert!(drawn > 0, "the sheet drew no rules at all");
        assert!(
            drawn < 100,
            "the sheet laid out {drawn} of 400 retained rules for a 1020 px viewport"
        );
    }
}
