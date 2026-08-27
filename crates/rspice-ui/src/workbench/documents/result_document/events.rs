//! Exact committed XSPICE digital and real-valued event history.

use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType, WaveformData};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::section_header;
use crate::workbench::AppState;

use super::strip::StripHeader;
use super::{
    AnalysisPresentationKey, DigitalEventSelection, EventOrderCache, EventOrderEntry,
    EventSelectionSource, panel_note, stat_table, well_hint,
};

const ROW_HEIGHT: f32 = 28.0;
const HEADER_HEIGHT: f32 = 31.0;

#[derive(Clone, Copy)]
struct LogicCode {
    value: &'static str,
    strength: &'static str,
}

#[derive(Clone, Copy)]
enum EventValue {
    Digital { code: u8, decoded: LogicCode },
    Real(f64),
}

impl EventValue {
    fn identity(self) -> u64 {
        match self {
            Self::Digital { code, .. } => u64::from(code),
            Self::Real(value) => value.to_bits(),
        }
    }

    fn display(self) -> String {
        match self {
            Self::Digital { decoded, .. } => decoded.value.to_owned(),
            Self::Real(value) => format!("{value:.17e}"),
        }
    }

    const fn strength(self) -> &'static str {
        match self {
            Self::Digital { decoded, .. } => decoded.strength,
            Self::Real(_) => "real-valued",
        }
    }

    const fn domain(self) -> &'static str {
        match self {
            Self::Digital { .. } => "digital",
            Self::Real(_) => "real",
        }
    }
}

#[derive(Clone, Copy)]
struct EventRow<'a> {
    source: EventSelectionSource,
    trace_name: &'a str,
    signal_name: &'a str,
    point_index: usize,
    event_ordinal: usize,
    time_s: f64,
    value: EventValue,
    initial: bool,
}

impl EventRow<'_> {
    const fn exact(&self) -> bool {
        matches!(
            self.source,
            EventSelectionSource::ExactDigital | EventSelectionSource::ExactReal
        )
    }
}

fn logic_code(value: f64) -> Option<(u8, LogicCode)> {
    if !value.is_finite() || value.fract() != 0.0 || !(0.0..=12.0).contains(&value) {
        return None;
    }
    let code = value as u8;
    let decoded = match code {
        0 => LogicCode {
            value: "0",
            strength: "strong",
        },
        1 => LogicCode {
            value: "1",
            strength: "strong",
        },
        2 => LogicCode {
            value: "X",
            strength: "strong",
        },
        3 => LogicCode {
            value: "0",
            strength: "resistive",
        },
        4 => LogicCode {
            value: "1",
            strength: "resistive",
        },
        5 => LogicCode {
            value: "X",
            strength: "resistive",
        },
        6 => LogicCode {
            value: "0",
            strength: "high-Z",
        },
        7 => LogicCode {
            value: "1",
            strength: "high-Z",
        },
        8 => LogicCode {
            value: "X",
            strength: "high-Z",
        },
        9 => LogicCode {
            value: "0",
            strength: "undetermined",
        },
        10 => LogicCode {
            value: "1",
            strength: "undetermined",
        },
        11 => LogicCode {
            value: "X",
            strength: "undetermined",
        },
        12 => LogicCode {
            value: "Z",
            strength: "high-Z",
        },
        _ => return None,
    };
    Some((code, decoded))
}

fn logic_code_u8(code: u8) -> Option<LogicCode> {
    logic_code(f64::from(code)).map(|(_, decoded)| decoded)
}

fn event_signal_name(name: &str) -> Option<(&str, bool)> {
    if let Some(signal) = name
        .strip_prefix("D(")
        .and_then(|name| name.strip_suffix(')'))
    {
        Some((signal, true))
    } else {
        name.strip_prefix("E(")
            .and_then(|name| name.strip_suffix(')'))
            .map(|signal| (signal, false))
    }
}

fn event_value(waveform: &WaveformData, value: f64) -> Option<EventValue> {
    let (_, digital) = event_signal_name(&waveform.name)?;
    if digital {
        logic_code(value).map(|(code, decoded)| EventValue::Digital { code, decoded })
    } else {
        value.is_finite().then_some(EventValue::Real(value))
    }
}

fn event_time_axis_is_valid(values: &[f64]) -> bool {
    !values.is_empty()
        && values.iter().all(|value| value.is_finite())
        && values.windows(2).all(|pair| pair[0] < pair[1])
}

fn real_event_values_are_valid(values: &[f64]) -> bool {
    let Some(first_value) = values.iter().position(|value| value.is_finite()) else {
        return false;
    };
    values[..first_value].iter().all(|value| value.is_nan())
        && values[first_value..].iter().all(|value| value.is_finite())
}

fn waveform_is_event(waveform: &WaveformData) -> bool {
    let Some((_, digital)) = event_signal_name(&waveform.name) else {
        return false;
    };
    if waveform.x.len() != waveform.y.len() || !event_time_axis_is_valid(&waveform.x) {
        return false;
    }
    if digital {
        waveform
            .y
            .iter()
            .copied()
            .all(|value| logic_code(value).is_some())
    } else {
        real_event_values_are_valid(&waveform.y)
    }
}

fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    analysis.success
        && analysis.analysis_type == AnalysisType::Transient
        && (matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
            }) if !digital_traces.is_empty() || !real_traces.is_empty()
        ) || analysis.waveforms.iter().any(waveform_is_event))
}

pub(super) fn active_analysis_is_renderable(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    let Some(analysis) = state.simulation.active_analysis() else {
        return false;
    };
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    analysis_is_renderable(analysis)
        && state
            .ui
            .results
            .retained_evidence_validity
            .get(&key)
            .copied()
            .unwrap_or(true)
}

fn build_event_order(
    analysis: &AnalysisResult,
    analysis_key: AnalysisPresentationKey,
) -> EventOrderCache {
    if let Some(AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
    }) = analysis.result_payload.as_ref()
    {
        let mut rows = Vec::new();
        for (trace_index, trace) in digital_traces.iter().enumerate() {
            for (point_index, point) in trace.points.iter().enumerate() {
                rows.push(EventOrderEntry {
                    source: EventSelectionSource::ExactDigital,
                    trace_index,
                    point_index,
                    time_s: point.time_s,
                    initial: point_index == 0,
                });
            }
        }
        for (trace_index, trace) in real_traces.iter().enumerate() {
            for (point_index, point) in trace.points.iter().enumerate() {
                rows.push(EventOrderEntry {
                    source: EventSelectionSource::ExactReal,
                    trace_index,
                    point_index,
                    time_s: point.time_s,
                    initial: point_index == 0,
                });
            }
        }
        sort_event_order(analysis, &mut rows);
        return EventOrderCache {
            analysis: analysis_key,
            exact: true,
            rows,
        };
    }

    // Legacy fallback: preserve access to old project files, but label these
    // rows as projections because the accepted transient grid is not the
    // original sparse event schedule.
    let mut rows = Vec::new();
    for (waveform_index, waveform) in analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform_is_event(waveform))
    {
        if event_signal_name(&waveform.name).is_none() {
            continue;
        }
        let mut previous = None;
        for (sample_index, (&time_s, &raw_value)) in
            waveform.x.iter().zip(waveform.y.iter()).enumerate()
        {
            let Some(value) = event_value(waveform, raw_value) else {
                continue;
            };
            let identity = value.identity();
            if previous == Some(identity) {
                continue;
            }
            rows.push(EventOrderEntry {
                source: if matches!(value, EventValue::Digital { .. }) {
                    EventSelectionSource::ProjectedDigital
                } else {
                    EventSelectionSource::ProjectedReal
                },
                trace_index: waveform_index,
                point_index: sample_index,
                time_s,
                initial: previous.is_none(),
            });
            previous = Some(identity);
        }
    }
    sort_event_order(analysis, &mut rows);
    EventOrderCache {
        analysis: analysis_key,
        exact: false,
        rows,
    }
}

fn sort_event_order(analysis: &AnalysisResult, rows: &mut [EventOrderEntry]) {
    rows.sort_by(|left, right| {
        left.time_s
            .total_cmp(&right.time_s)
            .then_with(|| {
                event_entry_signal_name(analysis, *left)
                    .cmp(event_entry_signal_name(analysis, *right))
            })
            .then_with(|| left.point_index.cmp(&right.point_index))
    });
}

fn event_entry_signal_name(analysis: &AnalysisResult, entry: EventOrderEntry) -> &str {
    match entry.source {
        EventSelectionSource::ExactDigital => analysis
            .result_payload
            .as_ref()
            .and_then(|payload| match payload {
                AnalysisResultPayload::TransientEvents { digital_traces, .. } => {
                    digital_traces.get(entry.trace_index)
                }
                _ => None,
            })
            .map_or("", |trace| trace.node_name.as_str()),
        EventSelectionSource::ExactReal => analysis
            .result_payload
            .as_ref()
            .and_then(|payload| match payload {
                AnalysisResultPayload::TransientEvents { real_traces, .. } => {
                    real_traces.get(entry.trace_index)
                }
                _ => None,
            })
            .map_or("", |trace| trace.node_name.as_str()),
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => analysis
            .waveforms
            .get(entry.trace_index)
            .and_then(|waveform| event_signal_name(&waveform.name).map(|(name, _)| name))
            .unwrap_or(""),
    }
}

fn event_row_from_entry(
    analysis: &AnalysisResult,
    entry: EventOrderEntry,
    event_ordinal: usize,
) -> Option<EventRow<'_>> {
    match entry.source {
        EventSelectionSource::ExactDigital => {
            let AnalysisResultPayload::TransientEvents { digital_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let trace = digital_traces.get(entry.trace_index)?;
            let point = trace.points.get(entry.point_index)?;
            Some(EventRow {
                source: entry.source,
                trace_name: &trace.node_name,
                signal_name: &trace.node_name,
                point_index: entry.point_index,
                event_ordinal,
                time_s: point.time_s,
                value: EventValue::Digital {
                    code: point.value_code,
                    decoded: logic_code_u8(point.value_code)?,
                },
                initial: entry.initial,
            })
        }
        EventSelectionSource::ExactReal => {
            let AnalysisResultPayload::TransientEvents { real_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let trace = real_traces.get(entry.trace_index)?;
            let point = trace.points.get(entry.point_index)?;
            Some(EventRow {
                source: entry.source,
                trace_name: &trace.node_name,
                signal_name: &trace.node_name,
                point_index: entry.point_index,
                event_ordinal,
                time_s: point.time_s,
                value: EventValue::Real(point.value),
                initial: entry.initial,
            })
        }
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => {
            let waveform = analysis.waveforms.get(entry.trace_index)?;
            let (signal_name, _) = event_signal_name(&waveform.name)?;
            let (&time_s, &raw_value) = waveform
                .x
                .get(entry.point_index)
                .zip(waveform.y.get(entry.point_index))?;
            Some(EventRow {
                source: entry.source,
                trace_name: &waveform.name,
                signal_name,
                point_index: entry.point_index,
                event_ordinal,
                time_s,
                value: event_value(waveform, raw_value)?,
                initial: entry.initial,
            })
        }
    }
}

#[cfg(test)]
fn event_rows(analysis: &AnalysisResult) -> Vec<EventRow<'_>> {
    build_event_order(
        analysis,
        AnalysisPresentationKey::new(crate::product::DatasetId::new(), analysis),
    )
    .rows
    .iter()
    .copied()
    .enumerate()
    .filter_map(|(index, entry)| event_row_from_entry(analysis, entry, index + 1))
    .collect()
}

fn event_row_for_selection<'a>(
    analysis: &'a AnalysisResult,
    selection: &DigitalEventSelection,
) -> Option<EventRow<'a>> {
    let entry = match selection.source {
        EventSelectionSource::ExactDigital => {
            let AnalysisResultPayload::TransientEvents { digital_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let (trace_index, trace) = digital_traces
                .iter()
                .enumerate()
                .find(|(_, trace)| trace.node_name == selection.trace_name)?;
            let point = trace.points.get(selection.point_index)?;
            EventOrderEntry {
                source: selection.source,
                trace_index,
                point_index: selection.point_index,
                time_s: point.time_s,
                initial: selection.point_index == 0,
            }
        }
        EventSelectionSource::ExactReal => {
            let AnalysisResultPayload::TransientEvents { real_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let (trace_index, trace) = real_traces
                .iter()
                .enumerate()
                .find(|(_, trace)| trace.node_name == selection.trace_name)?;
            let point = trace.points.get(selection.point_index)?;
            EventOrderEntry {
                source: selection.source,
                trace_index,
                point_index: selection.point_index,
                time_s: point.time_s,
                initial: selection.point_index == 0,
            }
        }
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => {
            let (trace_index, waveform) = analysis
                .waveforms
                .iter()
                .enumerate()
                .find(|(_, waveform)| waveform.name == selection.trace_name)?;
            if !waveform_is_event(waveform) {
                return None;
            }
            let (&time_s, &raw_value) = waveform
                .x
                .get(selection.point_index)
                .zip(waveform.y.get(selection.point_index))?;
            let current = event_value(waveform, raw_value)?;
            let expected_source = if matches!(current, EventValue::Digital { .. }) {
                EventSelectionSource::ProjectedDigital
            } else {
                EventSelectionSource::ProjectedReal
            };
            if selection.source != expected_source {
                return None;
            }
            let previous = waveform.y[..selection.point_index]
                .iter()
                .filter_map(|value| event_value(waveform, *value))
                .next_back()
                .map(EventValue::identity);
            if previous == Some(current.identity()) {
                return None;
            }
            EventOrderEntry {
                source: selection.source,
                trace_index,
                point_index: selection.point_index,
                time_s,
                initial: previous.is_none(),
            }
        }
    };
    event_row_from_entry(analysis, entry, 0)
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some((analysis_key, structurally_renderable)) =
        state.simulation.active_run().and_then(|run| {
            state.simulation.active_analysis().map(|analysis| {
                (
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    analysis_is_renderable(analysis),
                )
            })
        })
    else {
        well_hint(ui, "Select a dataset with retained event traces");
        return;
    };
    if !structurally_renderable {
        well_hint(
            ui,
            "The active analysis has no valid retained XSPICE event traces",
        );
        return;
    }
    if !super::retained_evidence_is_valid(state, analysis_key) {
        well_hint(ui, "The retained XSPICE event evidence is invalid");
        return;
    }
    let analysis = state
        .simulation
        .active_analysis()
        .expect("active analysis was resolved above");

    if state
        .ui
        .results
        .event_order_cache
        .as_ref()
        .is_none_or(|cache| cache.analysis != analysis_key)
    {
        state.ui.results.event_order_cache = Some(build_event_order(analysis, analysis_key));
    }
    let cache = state
        .ui
        .results
        .event_order_cache
        .as_ref()
        .expect("event order cache was initialized above");
    let exact = cache.exact;
    StripHeader::new(
        "EVENTS",
        &format!(
            "{} · {} {}",
            analysis.label,
            cache.rows.len(),
            if exact {
                "committed events"
            } else {
                "projected changes"
            }
        ),
        &[],
    )
    .show(ui);
    if !exact {
        panel_note(
            ui,
            "Legacy accepted-sample projection. Re-run this analysis to retain exact sparse event timestamps.",
        );
    }

    let selected = state.ui.results.selected_digital_event.clone();
    let mut requested = None;
    let min_width = 880.0_f32.max(ui.available_width());
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.events-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(min_width);
            TableBuilder::new(ui)
                .id_salt("rspice.results.events")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::initial(84.0))
                .column(Column::initial(158.0))
                .column(Column::remainder().at_least(190.0))
                .column(Column::initial(138.0))
                .column(Column::initial(128.0))
                .column(Column::initial(112.0))
                .header(HEADER_HEIGHT, |mut header| {
                    for label in [
                        "EVENT",
                        "PHYSICAL TIME",
                        "SIGNAL",
                        "VALUE",
                        "DOMAIN",
                        "KIND",
                    ] {
                        header.col(|ui| table_header(ui, label));
                    }
                })
                .body(|body| {
                    body.rows(ROW_HEIGHT, cache.rows.len(), |mut row| {
                        let row_index = row.index();
                        let Some(row_data) =
                            cache.rows.get(row_index).copied().and_then(|entry| {
                                event_row_from_entry(analysis, entry, row_index + 1)
                            })
                        else {
                            return;
                        };
                        let is_selected = selected.as_ref().is_some_and(|selection| {
                            selection.analysis == analysis_key
                                && selection.source == row_data.source
                                && selection.trace_name == row_data.trace_name
                                && selection.point_index == row_data.point_index
                        });
                        row.set_selected(is_selected);
                        row.col(|ui| {
                            if ui
                                .selectable_label(
                                    is_selected,
                                    format!("#{:04}", row_data.event_ordinal),
                                )
                                .clicked()
                            {
                                requested = Some(DigitalEventSelection {
                                    analysis: analysis_key,
                                    source: row_data.source,
                                    trace_name: row_data.trace_name.to_owned(),
                                    point_index: row_data.point_index,
                                });
                            }
                        });
                        row.col(|ui| mono(ui, &format!("{:.17e} s", row_data.time_s)));
                        row.col(|ui| mono(ui, row_data.signal_name));
                        row.col(|ui| mono(ui, &row_data.value.display()));
                        row.col(|ui| {
                            ui.label(row_data.value.domain());
                        });
                        row.col(|ui| {
                            ui.label(if row_data.initial {
                                "initial"
                            } else {
                                "change"
                            });
                        });
                    });
                });
        });
    if let Some(selection) = requested {
        state.ui.results.selected_digital_event = Some(selection);
    }
}

/// Why the event inspector has nothing to show for a selection that exists.
const EVENT_SELECTION_NO_DATASET: &str = "No dataset is open, so the selected event has nothing to resolve against. \
     Open the dataset it was taken from, or select an event row here.";
const EVENT_SELECTION_UNRETAINED_ANALYSIS: &str = "The analysis this event was selected from is no longer retained in the \
     active dataset. Select an event row again.";
const EVENT_SELECTION_OTHER_ANALYSIS: &str = "Select an event row in the active analysis.";
const EVENT_SELECTION_UNRETAINED_ROW: &str = "The retained trace or sample this event named is no longer present in the \
     analysis. Select an event row again.";

/// Why the inspector cannot show the event a selection names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventSelectionBlock {
    note: &'static str,
    /// Whether the selection itself is unrecoverable and should be dropped.
    /// A dataset that is merely not open right now is not: it can come back,
    /// and dropping the selection would lose the reader's place for a
    /// condition that is about the workspace rather than about the evidence.
    stale: bool,
}

/// A selection outlives the evidence it names: closing the dataset, or
/// re-running the analysis into a different retained shape, leaves the panel
/// holding an identity nothing resolves. Three of those four outcomes drew an
/// empty panel — no heading, no reason — which reads as a broken pane rather
/// than as a selection that has expired.
fn event_selection_block(
    state: &AppState,
    selection: &DigitalEventSelection,
) -> Option<EventSelectionBlock> {
    let block = |note, stale| Some(EventSelectionBlock { note, stale });
    let Some(run) = state.simulation.active_run() else {
        return block(EVENT_SELECTION_NO_DATASET, false);
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        return block(EVENT_SELECTION_UNRETAINED_ANALYSIS, true);
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        return block(EVENT_SELECTION_OTHER_ANALYSIS, true);
    }
    if event_row_for_selection(analysis, selection).is_none() {
        return block(EVENT_SELECTION_UNRETAINED_ROW, true);
    }
    None
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_digital_event.clone() else {
        section_header(ui, "Event selection", None);
        panel_note(
            ui,
            "Select an event row to inspect its retained value change.",
        );
        return;
    };
    if let Some(block) = event_selection_block(state, &selection) {
        if block.stale {
            state.ui.results.selected_digital_event = None;
        }
        section_header(ui, "Event selection", None);
        panel_note(ui, block.note);
        return;
    }
    let Some(event) = state
        .simulation
        .active_run()
        .and_then(|run| selection.analysis.resolve(run))
        .and_then(|(_, analysis)| event_row_for_selection(analysis, &selection))
    else {
        // Unreachable: the block above resolved this exact row over the same
        // unchanged state. Stated rather than asserted, because a panel that
        // says nothing is the defect this function is fixing.
        section_header(ui, "Event selection", None);
        panel_note(ui, EVENT_SELECTION_UNRETAINED_ROW);
        return;
    };

    section_header(
        ui,
        "Selected event",
        Some(if event.exact() { "EXACT" } else { "PROJECTED" }),
    );
    let mut stats = vec![
        ("Signal", event.signal_name.to_owned(), true),
        ("Physical time", format!("{:.17e} s", event.time_s), true),
        (
            if event.exact() {
                "Trace event"
            } else {
                "Source sample"
            },
            format!("#{}", selection.point_index + 1),
            false,
        ),
        ("Value", event.value.display(), true),
        ("Domain", event.value.domain().to_owned(), false),
    ];
    if let EventValue::Digital { code, .. } = event.value {
        stats.push(("Retained code", code.to_string(), false));
        stats.push(("Drive strength", event.value.strength().to_owned(), false));
    }
    stat_table(ui, &stats);
    panel_note(
        ui,
        if event.exact() {
            "This is an exact committed sparse event. Same-time transitions are preserved in per-trace order; cross-node delta-cycle ordering is not retained by the engine result contract."
        } else {
            "This row was reconstructed from a legacy accepted-sample projection. Its original sparse event timestamp is unavailable; re-run the analysis for exact evidence."
        },
    );
}

fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
}

fn mono(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn event_rows_keep_initial_value_and_only_projected_changes() {
        let waveform = WaveformData::new(
            "D(clk)",
            vec![0.0, 1.0, 2.0, 3.0],
            vec![0.0, 0.0, 1.0, 1.0],
            "#fff",
        );
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].initial);
        assert_eq!(rows[1].point_index, 2);
        assert!(matches!(rows[1].value, EventValue::Digital { code: 1, .. }));
        assert!(!rows[0].exact());
    }

    #[test]
    fn real_event_rows_preserve_projected_value_changes() {
        let waveform = WaveformData::new(
            "E(control)",
            vec![0.0, 1.0, 2.0],
            vec![f64::NAN, 0.25, 0.5],
            "#fff",
        );
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].initial);
        assert!(matches!(rows[1].value, EventValue::Real(value) if value == 0.5));
    }

    #[test]
    fn real_event_trace_rejects_an_unknown_after_the_first_committed_value() {
        let waveform = WaveformData::new(
            "E(control)",
            vec![0.0, 1.0, 2.0],
            vec![f64::NAN, 0.25, f64::NAN],
            "#fff",
        );
        assert!(!waveform_is_event(&waveform));
    }

    #[test]
    fn digital_code_contract_rejects_non_integral_values() {
        assert!(logic_code(12.0).is_some());
        assert!(logic_code(2.5).is_none());
        assert!(logic_code(13.0).is_none());
    }

    #[test]
    fn exact_event_rows_preserve_between_sample_and_same_time_transitions() {
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.25,
                        value_code: 0,
                    },
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 1,
                    },
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 0,
                    },
                ],
            }],
            real_traces: Vec::new(),
        };
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new(
                "D(clk)",
                vec![0.0, 1.0],
                vec![2.0, 0.0],
                "#fff",
            )])
            .with_result_payload(payload);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1].time_s.to_bits(), 0.5_f64.to_bits());
        assert_eq!(rows[2].time_s.to_bits(), 0.5_f64.to_bits());
        assert_eq!(rows[1].point_index, 1);
        assert_eq!(rows[2].point_index, 2);
        assert!(rows.iter().all(EventRow::exact));
    }

    #[test]
    fn exact_event_order_is_deterministic_without_claiming_cross_node_delta_order() {
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces: vec![
                crate::state::DigitalEventTraceEvidence {
                    node_name: "a".to_owned(),
                    points: vec![
                        crate::state::DigitalEventPointEvidence {
                            time_s: 0.5,
                            value_code: 0,
                        },
                        crate::state::DigitalEventPointEvidence {
                            time_s: 0.5,
                            value_code: 1,
                        },
                    ],
                },
                crate::state::DigitalEventTraceEvidence {
                    node_name: "z".to_owned(),
                    points: vec![crate::state::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 1,
                    }],
                },
            ],
            real_traces: Vec::new(),
        };
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(payload);
        let rows = event_rows(&analysis);

        assert_eq!(
            rows.iter().map(|row| row.signal_name).collect::<Vec<_>>(),
            ["a", "a", "z"]
        );
        assert_eq!(rows[0].point_index, 0);
        assert_eq!(rows[1].point_index, 1);
    }

    #[test]
    fn selection_resolution_rejects_a_projected_non_change_sample() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let analysis_key =
            AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
        let stale = DigitalEventSelection {
            analysis: analysis_key,
            source: EventSelectionSource::ProjectedDigital,
            trace_name: "D(clk)".to_owned(),
            point_index: 1,
        };
        let changed = DigitalEventSelection {
            point_index: 2,
            ..stale.clone()
        };

        assert!(event_row_for_selection(&analysis, &stale).is_none());
        assert_eq!(
            event_row_for_selection(&analysis, &changed)
                .expect("changed projected sample")
                .point_index,
            2
        );
    }

    /// Every way the inspector can fail to resolve a selection says which one
    /// it was, and drops the selection only when the evidence is gone for
    /// good. Three of these four returned silently, drawing a panel with no
    /// heading and no text at all.
    #[test]
    fn an_unresolvable_event_selection_is_explained_rather_than_drawn_empty() {
        use crate::state::SimulationRun;
        use crate::workbench::app_state::AppState;

        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let other = AnalysisResult::new(2, AnalysisType::Transient, "TRAN2").with_waveforms(vec![
            WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
        ]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis.clone());
        run.add_analysis(other);
        let dataset_id = run.dataset_id;

        let mut state = AppState::default();

        // No dataset at all: explained, and the selection is kept because the
        // dataset can come back.
        let selection = DigitalEventSelection {
            analysis: AnalysisPresentationKey::new(dataset_id, &analysis),
            source: EventSelectionSource::ProjectedDigital,
            trace_name: "D(clk)".to_owned(),
            point_index: 2,
        };
        assert_eq!(
            event_selection_block(&state, &selection),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_NO_DATASET,
                stale: false,
            })
        );

        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        // The row resolves: nothing blocks the inspector.
        assert_eq!(event_selection_block(&state, &selection), None);

        // A selection naming an analysis this dataset never retained.
        let unretained = DigitalEventSelection {
            analysis: AnalysisPresentationKey::new(
                dataset_id,
                &AnalysisResult::new(99, AnalysisType::Transient, "GONE"),
            ),
            ..selection.clone()
        };
        assert_eq!(
            event_selection_block(&state, &unretained),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_UNRETAINED_ANALYSIS,
                stale: true,
            })
        );

        // A selection whose trace no longer carries that sample index.
        let past_the_end = DigitalEventSelection {
            point_index: 97,
            ..selection.clone()
        };
        assert_eq!(
            event_selection_block(&state, &past_the_end),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_UNRETAINED_ROW,
                stale: true,
            })
        );

        // A selection on a retained analysis that is not the active one.
        assert!(state.simulation.select_analysis(1));
        assert_eq!(
            event_selection_block(&state, &selection),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_OTHER_ANALYSIS,
                stale: true,
            })
        );
    }
}
