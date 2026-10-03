//! Retained SOA rule projections, table, inspector and stress-history presentation.

use crate::{
    presentation::{PlotView, panel_note, stat_table, well_hint},
    strip::StripHeader,
    waveform::WaveformData,
};
use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};
use rspice_results::{
    analysis_payload::AnalysisResultPayload,
    family_metadata::AnalysisResultFamilyMetadata,
    result_presentation::AnalysisPresentationKey,
    safety::dynamic_active_interval_indices,
    soa_evidence::{SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence},
};
use rspice_ui_kit::{
    plot::{self, Axis, LimitLine, Marker, PlotSpec, Trace, XScale},
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::section_header,
};
type AnalysisResult = rspice_results::analysis_result::AnalysisResult<WaveformData>;
const ROW_HEIGHT: f32 = 29.0;

/// Which safe-operating-area rules the SOA table lists.
///
/// Attention and passing are complements, so a rule is in exactly one of them
/// and the two counts always sum to the evaluated total.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SoaRuleFilter {
    #[default]
    All,
    Violations,
    Passing,
}

/// One SOA rule picked out of the evidence table.
///
/// A rule is identified by its device and the stressed parameter, never by row
/// ordinal: the filter reorders the table and the analysis may be re-run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoaRuleSelection {
    pub analysis: AnalysisPresentationKey,
    pub device_id: String,
    pub parameter: SoaParameterEvidence,
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
pub struct SoaRuleFacts {
    /// Index into the analysis' waveforms of this rule's verified stress
    /// history, when one is retained.
    pub stress_waveform: Option<usize>,
    pub limit_waveform: Option<usize>,
    pub temperature_waveform: Option<usize>,
    /// The worst-interval cell, as the table prints it.
    pub interval_compact: String,
    /// The same interval, as the inspector prints it.
    pub interval_full: String,
    /// The stress card's padded axis extents. Both walk the whole history,
    /// and the card asked for them on every frame it was open.
    pub stress_axes: Option<((f64, f64), (f64, f64))>,
    /// Decimation-cache identity for the stress polyline. Without one the
    /// renderer re-reduces every retained sample per frame.
    pub stress_cache_key: u64,
}

/// Scanned stress facts and visible rule ordinals for one retained analysis.
#[derive(Debug, Clone, PartialEq)]
pub struct SoaPlan {
    rules: Vec<SoaRuleFacts>,
    visible: [Vec<usize>; SoaRuleFilter::ALL.len()],
}

impl SoaPlan {
    pub fn facts(&self, rule: usize) -> Option<&SoaRuleFacts> {
        self.rules.get(rule)
    }

    /// The rules one filter shows, in retained order.
    fn visible(&self, filter: SoaRuleFilter) -> &[usize] {
        &self.visible[filter.ordinal()]
    }
}

pub fn build_soa_plan(
    version: u64,
    analysis_key: AnalysisPresentationKey,
    analysis: &AnalysisResult,
    evaluations: &[SoaEvaluationEvidence],
    mut on_stress_scan: impl FnMut(),
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
            on_stress_scan();
            let stress = stress_waveform(analysis, evaluation);
            let limits = derating_waveform(analysis, evaluation, false);
            let temperature = derating_waveform(analysis, evaluation, true);
            SoaRuleFacts {
                limit_waveform: limits.map(|found| found.index),
                temperature_waveform: temperature.map(|found| found.index),
                stress_waveform: stress.map(|found| found.index),
                interval_compact: worst_interval_text(
                    stress,
                    limits.map(|w| w.y.as_slice()),
                    evaluation,
                    true,
                ),
                interval_full: worst_interval_text(
                    stress,
                    limits.map(|w| w.y.as_slice()),
                    evaluation,
                    false,
                ),
                stress_axes: stress.map(|waveform| {
                    let (x_min, x_max) = padded_range(waveform.x.iter().copied(), None);
                    let (y_min, y_max) = padded_range(
                        waveform
                            .y
                            .iter()
                            .copied()
                            .chain(limits.into_iter().flat_map(|wave| wave.y.iter().copied())),
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
    SoaPlan { rules, visible }
}

/// Borrowed validated SOA evidence and its cached presentation facts.
pub struct SoaSource<'a> {
    pub analysis: &'a AnalysisResult,
    pub analysis_key: AnalysisPresentationKey,
    pub evaluations: &'a [SoaEvaluationEvidence],
    pub violation_count: usize,
    pub plan: &'a SoaPlan,
}

pub struct SoaControls<'a> {
    pub selected: Option<&'a SoaRuleSelection>,
    pub filter: SoaRuleFilter,
    pub trace_open: bool,
    pub stress_view: PlotView,
}

pub struct SoaResponse {
    pub filter: SoaRuleFilter,
    pub trace_open: bool,
    pub selection: Option<SoaRuleSelection>,
    pub cross_probe: Option<SoaRuleSelection>,
    pub fit_clicked: bool,
    pub stress_view_change: Option<plot::ViewChange>,
}

struct StressPlot<'a> {
    view: PlotView,
    cache: &'a mut plot::DecimationCache,
}

pub struct SoaInspector<'a> {
    pub evaluation: &'a SoaEvaluationEvidence,
    pub facts: Option<&'a SoaRuleFacts>,
    pub event_count: usize,
    pub schematic_available: bool,
}

pub struct SoaInspectorResponse {
    pub open_trace: bool,
    pub locate_device: bool,
}

pub enum SoaSelectionAbsence {
    Unselected,
    Unavailable,
    Removed,
}

pub fn selection_note(ui: &mut Ui, reason: SoaSelectionAbsence) {
    section_header(ui, "SOA rule selection", None);
    panel_note(
        ui,
        match reason {
            SoaSelectionAbsence::Unselected => {
                "Select a rule row to inspect its exact worst point, limit, and coverage."
            }
            SoaSelectionAbsence::Unavailable => {
                "Select the rule's analysis with valid, successful SOA evidence to inspect it."
            }
            SoaSelectionAbsence::Removed => "The selected SOA rule is no longer retained.",
        },
    );
}

pub fn show(
    ui: &mut Ui,
    source: SoaSource<'_>,
    controls: SoaControls<'_>,
    cache: &mut plot::DecimationCache,
    schematic_available: impl Fn(AnalysisPresentationKey, &str) -> bool,
) -> SoaResponse {
    let SoaSource {
        analysis,
        analysis_key,
        evaluations,
        violation_count,
        plan,
    } = source;
    let mut plot = StressPlot {
        view: controls.stress_view,
        cache,
    };
    let mut stress_view_change = None;
    let selected = controls.selected;
    let initial_filter = controls.filter;
    let mut filter = initial_filter;
    let mut trace_open = controls.trace_open;
    let mut requested = selected.cloned();
    let mut cross_probe = None;

    let stress_view = controls.stress_view;
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
        plot.view = PlotView::default();
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
                .and_then(|index| evidence_trace(analysis, index))
                .map(|waveform| (facts, waveform))
        }) {
            stress_view_change = stress_trace_card(
                ui,
                &mut plot,
                waveform,
                evaluation,
                facts,
                facts
                    .limit_waveform
                    .and_then(|i| evidence_trace(analysis, i)),
                facts
                    .temperature_waveform
                    .and_then(|i| evidence_trace(analysis, i)),
            );
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
                        // `rule` is an ordinal from the memoized plan and
                        // `evaluations` is the live retained evidence. They
                        // agree for every generation the plan was built
                        // against, and a panic is not how a Results sheet
                        // should report that they stopped agreeing: a row it
                        // cannot vouch for is a row it does not draw.
                        let (Some(evaluation), Some(facts)) =
                            (evaluations.get(rule), plan.facts(rule))
                        else {
                            return;
                        };
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
                                    schematic_available(analysis_key, &evaluation.device_id);
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

    SoaResponse {
        filter,
        trace_open,
        selection: requested,
        cross_probe,
        fit_clicked: header.fit_clicked,
        stress_view_change,
    }
}

pub fn right_panel(ui: &mut Ui, source: SoaInspector<'_>) -> SoaInspectorResponse {
    let SoaInspector {
        evaluation,
        facts,
        event_count,
        ..
    } = source;
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
                    || worst_interval_text(None, None, evaluation, false),
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
            (
                "Warning above",
                threshold_text(evaluation.thresholds.warning_fraction),
                false,
            ),
            (
                "Critical above",
                threshold_text(evaluation.thresholds.critical_fraction),
                false,
            ),
        ],
    );
    if let Some(duration) = evaluation.duration {
        stat_table(
            ui,
            &[
                (
                    "Duration threshold",
                    format!("{:.17e} s", duration.minimum_duration_s),
                    false,
                ),
                (
                    "Longest excursion",
                    format!("{:.17e} s", duration.longest_excursion_s),
                    true,
                ),
                (
                    "Total time above limit",
                    format!("{:.17e} s", duration.total_exceedance_s),
                    false,
                ),
                (
                    "Qualified excursions",
                    duration.qualified_excursions.to_string(),
                    true,
                ),
                (
                    "Unqualified excursions",
                    duration.rejected_excursions.to_string(),
                    false,
                ),
                (
                    "Window-clipped excursions",
                    duration.clipped_excursions.to_string(),
                    false,
                ),
            ],
        );
        if let Some(cumulative) = duration.cumulative {
            stat_table(
                ui,
                &[
                    ("Duration policy", "Cumulative exposure".into(), false),
                    (
                        "Recovery time constant",
                        cumulative
                            .recovery_time_s
                            .map(|value| format!("{value:.17e} s"))
                            .unwrap_or_else(|| "No recovery".into()),
                        false,
                    ),
                    (
                        "Peak exposure",
                        format!("{:.17e} s", cumulative.peak_exposure_s),
                        true,
                    ),
                    (
                        "Exposure at window end",
                        format!("{:.17e} s", cumulative.final_exposure_s),
                        false,
                    ),
                ],
            );
            panel_note(
                ui,
                "Exposure starts at zero, accumulates above the limit, and optionally decays between excursions. Each excursion's ending exposure determines its verdict; earlier excursions keep their own verdicts.",
            );
        }
        panel_note(
            ui,
            "The worst point is selected by reported severity, then utilization. Excursion widths use linear threshold crossings inside the checked window; the full stress history includes unqualified excursions.",
        );
    }
    panel_note(ui, &evaluation.description);

    let trace_available = facts.is_some_and(|facts| facts.stress_waveform.is_some());
    let schematic_available = source.schematic_available;
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
    SoaInspectorResponse {
        open_trace,
        locate_device,
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

fn threshold_text(fraction: Option<f64>) -> String {
    fraction
        .map(|value| format!("{}% of limit", value * 100.0))
        .unwrap_or_else(|| "Disabled".into())
}

/// Borrow either the complete SOA observations or a legacy full-grid trace.
#[derive(Clone, Copy)]
struct SoaTrace<'a> {
    index: usize,
    name: &'a str,
    x: &'a Vec<f64>,
    y: &'a Vec<f64>,
}

impl<'a> From<&'a WaveformData> for SoaTrace<'a> {
    fn from(wave: &'a WaveformData) -> Self {
        Self {
            index: usize::MAX,
            name: &wave.name,
            x: &wave.x,
            y: &wave.y,
        }
    }
}

fn evidence_trace(analysis: &AnalysisResult, index: usize) -> Option<SoaTrace<'_>> {
    if let Some(AnalysisResultPayload::Soa {
        source_history: Some(source),
        ..
    }) = &analysis.result_payload
    {
        let wave = source.waveforms.get(index)?;
        Some(SoaTrace {
            index,
            name: &wave.name,
            x: &source.time,
            y: &wave.values,
        })
    } else {
        let mut trace = SoaTrace::from(analysis.waveforms.get(index)?);
        trace.index = index;
        Some(trace)
    }
}

fn evidence_traces(analysis: &AnalysisResult) -> impl Iterator<Item = SoaTrace<'_>> {
    let count = match &analysis.result_payload {
        Some(AnalysisResultPayload::Soa {
            source_history: Some(source),
            ..
        }) => source.waveforms.len(),
        _ => analysis.waveforms.len(),
    };
    (0..count).filter_map(|index| evidence_trace(analysis, index))
}

fn derating_waveform<'a>(
    analysis: &'a AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
    temperature: bool,
) -> Option<SoaTrace<'a>> {
    let name = if evaluation.envelope.is_some() && !temperature {
        rspice_results::safety::soa_envelope_limit_waveform_name(
            &evaluation.device_id,
            evaluation.parameter.runtime_parameter(),
        )
    } else if temperature {
        evaluation.derating?;
        rspice_results::safety::soa_derating_temperature_waveform_name(&evaluation.device_id)
    } else {
        evaluation.derating?;
        rspice_results::safety::soa_power_limit_waveform_name(&evaluation.device_id)
    };
    let AnalysisResultFamilyMetadata::Soa { time } = analysis.family_metadata.as_ref()? else {
        return None;
    };
    evidence_traces(analysis).find(|wave| {
        wave.name == name && wave.x.as_slice() == time.as_slice() && wave.y.len() == time.len()
    })
}

fn stress_waveform<'a>(
    analysis: &'a AnalysisResult,
    evaluation: &SoaEvaluationEvidence,
) -> Option<SoaTrace<'a>> {
    let expected_samples = usize::try_from(evaluation.sample_count).ok()?;
    if expected_samples == 0 {
        return None;
    }
    let AnalysisResultFamilyMetadata::Soa { time } = analysis.family_metadata.as_ref()? else {
        return None;
    };
    let name = rspice_results::safety::soa_stress_waveform_name(
        &evaluation.device_id,
        evaluation.parameter.runtime_parameter(),
    );
    let limits = derating_waveform(analysis, evaluation, false);
    if (evaluation.derating.is_some() || evaluation.envelope.is_some()) && limits.is_none() {
        return None;
    }
    evidence_traces(analysis).find(|waveform| {
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
            && (evaluation.duration.is_some()
                || waveform.y.iter().enumerate().all(|(index, value)| {
                    rspice_results::safety::compare_soa_stress(
                        *value,
                        limits.map_or(evaluation.limit_value, |wave| wave.y[index]),
                        evaluation.worst_actual_value,
                        evaluation.limit_value,
                    )
                    .is_le()
                }))
    })
}

fn worst_interval_text(
    stress: Option<SoaTrace<'_>>,
    limits: Option<&[f64]>,
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
    if let Some(duration) = evaluation.duration {
        use rspice_results::safety::{SoaLimitTrace, scan_soa_duration_with_mode};
        let limit_trace = limits.map_or(
            SoaLimitTrace::Constant(evaluation.limit_value),
            SoaLimitTrace::Samples,
        );
        if let Ok(scan) = scan_soa_duration_with_mode(
            waveform.x.as_slice(),
            waveform.y.as_slice(),
            limit_trace,
            duration.minimum_duration_s,
            duration.mode(),
            || false,
        ) && let Some(excursion) = scan.excursions.iter().find(|excursion| {
            (excursion.first_sample..=excursion.last_sample).contains(&worst_index)
        }) {
            return if compact {
                format!("{:.5e}…{:.5e} s", excursion.start_s, excursion.end_s)
            } else {
                format!(
                    "{} excursion: {:.17e} s to {:.17e} s (linear threshold crossings)",
                    if excursion.qualified {
                        "Qualified"
                    } else {
                        "Unqualified"
                    },
                    excursion.start_s,
                    excursion.end_s
                )
            };
        }
    }
    let threshold = |index: usize| {
        let limit = limits.map_or(evaluation.limit_value, |limits| limits[index]);
        if evaluation.verdict == SoaRuleVerdictEvidence::Warning {
            limit
                * evaluation
                    .thresholds
                    .warning_fraction
                    .expect("validated warning threshold")
        } else {
            limit
        }
    };
    let Some((start, end)) =
        dynamic_active_interval_indices(waveform.y.as_slice(), worst_index, threshold)
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
    plot: &mut StressPlot<'_>,
    waveform: SoaTrace<'_>,
    evaluation: &SoaEvaluationEvidence,
    facts: &SoaRuleFacts,
    limits: Option<SoaTrace<'_>>,
    temperatures: Option<SoaTrace<'_>>,
) -> Option<plot::ViewChange> {
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
            let view = plot.view;
            let (x_min, x_max) = view.x.unwrap_or((x_min, x_max));
            let (y_min, y_max) = view.y.unwrap_or((y_min, y_max));
            let title = format!(
                "SOA stress history for {} {}",
                evaluation.device_id,
                parameter_label(evaluation.parameter)
            );
            let detail = format!(
                "Exact retained stress samples; limit at the worst point is {:.17e} {}. Worst point {:.17e} {} at {:.17e} seconds. A varying limit uses its retained trace at each time.",
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
            if let Some(limits) = limits {
                spec.traces.push(Trace::new(limits.x.as_slice(), limits.y.as_slice(), t.color.err).cache_key(facts.stress_cache_key ^ 0x7BD3_815F_A904_260C));
                mono(ui, if evaluation.duration.is_some() { "Red trace: allowed limit at each sample. Worst point follows duration-qualified severity, then utilization." } else { "Red trace: allowed limit at each sample. Worst point: highest stress / allowed-limit ratio." });
            } else {
                spec.limit_lines.push(LimitLine {
                    y: evaluation.limit_value,
                    color: t.color.err,
                    label: format!("LIMIT {:.6e} {}", evaluation.limit_value, evaluation.unit),
                });
            }
            spec.markers.push(Marker::point(
                evaluation.worst_time_s,
                evaluation.worst_actual_value,
                t.color.accent,
                "WORST",
            ));
            let readout = |hover_x: f64| {
                let index = nearest_sample_index(waveform.x.as_slice(), hover_x);
                let mut readout = vec![
                    ("t".to_owned(), format!("{:.17e} s", waveform.x[index])),
                    (
                        parameter_label(evaluation.parameter).to_owned(),
                        format!("{:.17e} {}", waveform.y[index], evaluation.unit),
                    ),
                    (
                        "Margin".to_owned(),
                        format!(
                            "{:.17e} {}",
                            limits.map_or(evaluation.limit_value, |wave| wave.y[index]) - waveform.y[index],
                            evaluation.unit
                        ),
                    ),
                ];
                if let Some(temperatures) = temperatures {
                    readout.push(("Device temperature".into(), format!("{:.17e} K", temperatures.y[index])));
                    readout.push(("Allowed power".into(), format!("{:.17e} W", limits.expect("derated limit").y[index])));
                }
                readout
            };
            let response = ui
                .allocate_ui(egui::vec2(ui.available_width(), 210.0), |ui| {
                    rspice_ui_kit::plot::show(
                        ui,
                        &spec,
                        plot.cache,
                        None,
                        Some(&readout),
                    )
                })
                .inner;
            response.view.any().then_some(response.view)
        }).inner
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
        SoaParameterEvidence::GateSourceVoltagePositive => "Gate-source voltage · positive",
        SoaParameterEvidence::GateSourceVoltageNegative => "Gate-source voltage · negative",
        SoaParameterEvidence::DrainSourceVoltagePositive => "Drain-source voltage · positive",
        SoaParameterEvidence::DrainSourceVoltageNegative => "Drain-source voltage · negative",
        SoaParameterEvidence::GateDrainVoltagePositive => "Gate-drain voltage · positive",
        SoaParameterEvidence::GateDrainVoltageNegative => "Gate-drain voltage · negative",
        SoaParameterEvidence::BaseEmitterVoltagePositive => "Base-emitter voltage · positive",
        SoaParameterEvidence::BaseEmitterVoltageNegative => "Base-emitter voltage · negative",
        SoaParameterEvidence::CollectorEmitterVoltagePositive => {
            "Collector-emitter voltage · positive"
        }
        SoaParameterEvidence::CollectorEmitterVoltageNegative => {
            "Collector-emitter voltage · negative"
        }
        SoaParameterEvidence::BaseCollectorVoltagePositive => "Base-collector voltage · positive",
        SoaParameterEvidence::BaseCollectorVoltageNegative => "Base-collector voltage · negative",
        SoaParameterEvidence::DrainCurrentPositive => "Drain current · positive",
        SoaParameterEvidence::DrainCurrentNegative => "Drain current · negative",
        SoaParameterEvidence::CollectorCurrentPositive => "Collector current · positive",
        SoaParameterEvidence::CollectorCurrentNegative => "Collector current · negative",
        SoaParameterEvidence::GateCurrent => "Gate current",
        SoaParameterEvidence::GateCurrentPositive => "Gate current · positive",
        SoaParameterEvidence::GateCurrentNegative => "Gate current · negative",
        SoaParameterEvidence::SourceCurrent => "Source current",
        SoaParameterEvidence::SourceCurrentPositive => "Source current · positive",
        SoaParameterEvidence::SourceCurrentNegative => "Source current · negative",
        SoaParameterEvidence::BaseCurrent => "Base current",
        SoaParameterEvidence::BaseCurrentPositive => "Base current · positive",
        SoaParameterEvidence::BaseCurrentNegative => "Base current · negative",
        SoaParameterEvidence::EmitterCurrent => "Emitter current",
        SoaParameterEvidence::EmitterCurrentPositive => "Emitter current · positive",
        SoaParameterEvidence::EmitterCurrentNegative => "Emitter current · negative",
        SoaParameterEvidence::CollectorSubstrateVoltage => "Collector to substrate voltage",
        SoaParameterEvidence::CollectorSubstrateVoltagePositive => {
            "Collector to substrate voltage positive"
        }
        SoaParameterEvidence::CollectorSubstrateVoltageNegative => {
            "Collector to substrate voltage negative"
        }
        SoaParameterEvidence::BaseSubstrateVoltage => "Base to substrate voltage",
        SoaParameterEvidence::BaseSubstrateVoltagePositive => "Base to substrate voltage positive",
        SoaParameterEvidence::BaseSubstrateVoltageNegative => "Base to substrate voltage negative",
        SoaParameterEvidence::EmitterSubstrateVoltage => "Emitter to substrate voltage",
        SoaParameterEvidence::EmitterSubstrateVoltagePositive => {
            "Emitter to substrate voltage positive"
        }
        SoaParameterEvidence::EmitterSubstrateVoltageNegative => {
            "Emitter to substrate voltage negative"
        }
        SoaParameterEvidence::SubstrateCurrent => "Substrate current",
        SoaParameterEvidence::SubstrateCurrentPositive => "Substrate current positive",
        SoaParameterEvidence::SubstrateCurrentNegative => "Substrate current negative",
        SoaParameterEvidence::AnodeCathodeVoltage => "Diode anode to cathode voltage",
        SoaParameterEvidence::AnodeCathodeVoltagePositive => {
            "Diode anode to cathode voltage positive"
        }
        SoaParameterEvidence::AnodeCathodeVoltageNegative => {
            "Diode anode to cathode voltage negative"
        }
        SoaParameterEvidence::AnodeCurrent => "Diode anode current",
        SoaParameterEvidence::AnodeCurrentPositive => "Diode anode current positive",
        SoaParameterEvidence::AnodeCurrentNegative => "Diode anode current negative",

        SoaParameterEvidence::BodySourceVoltage => "Vbs",
        SoaParameterEvidence::BodySourceVoltagePositive => "Vbs positive",
        SoaParameterEvidence::BodySourceVoltageNegative => "Vbs negative",
        SoaParameterEvidence::BodyDrainVoltage => "Vbd",
        SoaParameterEvidence::BodyDrainVoltagePositive => "Vbd positive",
        SoaParameterEvidence::BodyDrainVoltageNegative => "Vbd negative",
        SoaParameterEvidence::GateBodyVoltage => "Vgb",
        SoaParameterEvidence::GateBodyVoltagePositive => "Vgb positive",
        SoaParameterEvidence::GateBodyVoltageNegative => "Vgb negative",
        SoaParameterEvidence::BulkCurrent => "Bulk / body-contact current",
        SoaParameterEvidence::BulkCurrentPositive => "Bulk / body-contact current positive",
        SoaParameterEvidence::BulkCurrentNegative => "Bulk / body-contact current negative",
        SoaParameterEvidence::BackgateSourceVoltage => "Ves (back gate to source)",
        SoaParameterEvidence::BackgateSourceVoltagePositive => "Ves (back gate to source) positive",
        SoaParameterEvidence::BackgateSourceVoltageNegative => "Ves (back gate to source) negative",
        SoaParameterEvidence::BackgateDrainVoltage => "Ved (back gate to drain)",
        SoaParameterEvidence::BackgateDrainVoltagePositive => "Ved (back gate to drain) positive",
        SoaParameterEvidence::BackgateDrainVoltageNegative => "Ved (back gate to drain) negative",
        SoaParameterEvidence::GateBackgateVoltage => "Vge (gate to back gate)",
        SoaParameterEvidence::GateBackgateVoltagePositive => "Vge (gate to back gate) positive",
        SoaParameterEvidence::GateBackgateVoltageNegative => "Vge (gate to back gate) negative",
        SoaParameterEvidence::BackgateCurrent => "Back-gate current",
        SoaParameterEvidence::BackgateCurrentPositive => "Back-gate current positive",
        SoaParameterEvidence::BackgateCurrentNegative => "Back-gate current negative",
        SoaParameterEvidence::BodyBackgateVoltage => "Body contact to back gate voltage",
        SoaParameterEvidence::BodyBackgateVoltagePositive => {
            "Body contact to back gate voltage positive"
        }
        SoaParameterEvidence::BodyBackgateVoltageNegative => {
            "Body contact to back gate voltage negative"
        }
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

#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use super::*;
    use rspice_results::soa_evidence::{SoaViolationEvidence, SoaViolationSeverityEvidence};

    pub fn soa_analysis(rules: usize, samples: usize, peak: f64) -> AnalysisResult {
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
                rspice_results::safety::soa_stress_waveform_name(
                    &device_id,
                    rspice_results::safety::SoAParameter::Vds,
                ),
                time.clone(),
                y,
                "#00aaff",
            ));
            evaluations.push(SoaEvaluationEvidence {
                duration: None,
                thresholds: Default::default(),
                envelope: None,
                derating: None,
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

        AnalysisResult::new(
            1,
            rspice_results::analysis_type::AnalysisType::Soa,
            "SOA",
            0.0,
        )
        .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
        .with_waveforms(waveforms)
        .with_result_payload(AnalysisResultPayload::Soa {
            source_history: None,
            evaluations,
            violations,
        })
    }
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
                rspice_results::safety::soa_stress_waveform_name(
                    "M1",
                    parameter.runtime_parameter()
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
    fn soa_thresholds_control_the_retained_warning_interval() {
        let analysis = test_support::soa_analysis(1, 4, 3.0);
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = &analysis.result_payload else {
            unreachable!()
        };
        let mut evaluation = evaluations[0].clone();
        evaluation.thresholds.warning_fraction = Some(0.5);
        let text = worst_interval_text(
            Some((&analysis.waveforms[0]).into()),
            None,
            &evaluation,
            false,
        );
        assert_eq!(
            text,
            format!("Warning band: {:.17e} s to {:.17e} s", 2e-12, 3e-12)
        );
        assert_eq!(threshold_text(None), "Disabled");
        assert_eq!(threshold_text(Some(1.5)), "150% of limit");
    }

    #[test]
    fn the_plan_states_what_a_direct_scan_of_each_rule_states() {
        let analysis = test_support::soa_analysis(6, 32, 3.0);
        let run = rspice_results::run::SimulationRun::<WaveformData>::new(
            1,
            0.0,
            rspice_results::run::ExecutionTarget::LocalDesktop,
        );
        let key = AnalysisPresentationKey::new(run.dataset_id, &analysis);
        let Some(AnalysisResultPayload::Soa { evaluations, .. }) = analysis.result_payload.as_ref()
        else {
            panic!("the fixture retains an SOA payload");
        };
        let plan = build_soa_plan(0, key, &analysis, evaluations, || {});
        for (rule, evaluation) in evaluations.iter().enumerate() {
            let scanned = stress_waveform(&analysis, evaluation);
            let facts = plan.facts(rule).expect("one fact set per rule");
            assert_eq!(facts.stress_waveform.is_some(), scanned.is_some(), "{rule}");
            assert_eq!(
                facts.interval_compact,
                worst_interval_text(scanned, None, evaluation, true),
                "{rule}"
            );
            assert_eq!(
                facts.interval_full,
                worst_interval_text(scanned, None, evaluation, false),
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
}
