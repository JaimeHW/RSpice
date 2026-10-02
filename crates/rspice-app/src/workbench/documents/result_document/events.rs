//! Retained digital and real-valued event history and its source attribution.

use rspice_results::events::projection::{
    BusRadix, BusTimeline, EventOrder, EventRow, EventSelectionSource, EventValue, bus_notes,
    bus_subtitle, event_row_at_name, event_row_from_entry,
};
use rspice_results_ui::presentation::{panel_note, stat_table, well_hint};
use std::sync::Arc;

use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::state::{
    AnalysisResult, AnalysisResultPayload, AnalysisType, RunHistoryRevision, WaveformData,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{SegmentedWidth, chip, section_header, segmented};
use crate::workbench::AppState;

use super::{AnalysisPresentationKey, SheetContext};
use rspice_results_ui::strip::StripHeader;

const ROW_HEIGHT: f32 = 28.0;
const HEADER_HEIGHT: f32 = 31.0;

#[derive(Clone, Copy)]
enum EventOrigin<'a> {
    Native,
    Imported(&'a crate::state::ResultImportSource),
    Unrecorded,
}

impl<'a> EventOrigin<'a> {
    fn active(state: &'a AppState) -> Self {
        if let Some(source) = state
            .simulation
            .active_analysis()
            .and_then(|analysis| analysis.import_source.as_ref())
        {
            Self::Imported(source)
        } else if state
            .simulation
            .active_analysis()
            .and_then(AnalysisResult::provenance)
            .is_some()
            && state
                .simulation
                .active_run()
                .is_some_and(|run| run.prepared_receipt().is_some())
        {
            Self::Native
        } else {
            Self::Unrecorded
        }
    }

    fn label(self) -> &'a str {
        match self {
            Self::Native => "RSpice prepared execution",
            Self::Imported(source) => source.format.canonical_id(),
            Self::Unrecorded => "Source not recorded",
        }
    }

    fn note(self) -> Option<&'static str> {
        match self {
            Self::Native => None,
            Self::Imported(source)
                if matches!(
                    source.format,
                    crate::state::ResultImportFormat::Vcd | crate::state::ResultImportFormat::Fst
                ) =>
            {
                Some(
                    "Imported digital values use four-state logic. Source drive strength was not retained; stored event codes use canonical strengths. File attribution does not identify or authenticate the source solver.",
                )
            }
            Self::Imported(_) => Some(
                "Imported result data. An encoded strength does not establish source drive strength. File attribution does not identify or authenticate the source solver.",
            ),
            Self::Unrecorded => Some(
                "Source not recorded. An encoded strength does not establish source drive strength; historical imports and native results cannot be distinguished from these records.",
            ),
        }
    }
}

#[cfg(test)]
mod source_tests;

/// Source ownership is checked by both the sheet and the inspector. Holding
/// the history revision prevents restored or edited results from reusing an
/// old merge even when their dataset identity and display counter match.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EventOrderCache {
    pub analysis: AnalysisPresentationKey,
    source: (RunHistoryRevision, u64),
    expanded: std::collections::BTreeSet<String>,
    order: Arc<EventOrder>,
}

fn event_order(state: &mut AppState) -> Option<Arc<EventOrder>> {
    let run = state.simulation.active_run()?;
    let analysis = state.simulation.active_analysis()?;
    let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
    let source = (
        state.simulation.runs.revision(),
        state.simulation.data_version,
    );
    let expanded = &state.ui.results.expanded_event_buses;
    if let Some(cache) = &state.ui.results.event_order_cache
        && cache.analysis == key
        && cache.source == source
        && &cache.expanded == expanded
    {
        return Some(Arc::clone(&cache.order));
    }
    if !analysis_is_renderable(analysis) || !super::retained_evidence_is_valid(state, key) {
        return None;
    }
    let order = Arc::new(build_event_order(analysis, expanded));
    state.ui.results.event_order_cache = Some(EventOrderCache {
        analysis: key,
        source,
        expanded: expanded.clone(),
        order: Arc::clone(&order),
    });
    Some(order)
}

/// One event picked out of the history.
///
/// Addressed by node name rather than trace ordinal so the selection survives
/// a re-run that registers event nodes in a different order. A `Bus` row's
/// `trace_name` is the bus name, for the same reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DigitalEventSelection {
    pub analysis: AnalysisPresentationKey,
    pub source: EventSelectionSource,
    pub trace_name: String,
    pub point_index: usize,
    time_bits: u64,
    initial: bool,
    value: SelectedEventValue,
}

/// Raw retained identity, independent of display radix and formatting. A bus
/// also carries its bit mapping: an unchanged word can name different nets.
#[derive(Debug, Clone, PartialEq, Eq)]
enum SelectedEventValue {
    Scalar(u64),
    Bus {
        label: String,
        members: Vec<String>,
        codes: Vec<Option<u8>>,
    },
}

impl DigitalEventSelection {
    fn matches(&self, row: &EventRow<'_>, buses: &[BusTimeline]) -> bool {
        if self.source != row.source()
            || self.trace_name != row.trace_name()
            || self.point_index != row.point_index()
            || self.time_bits != row.time_s().to_bits()
            || self.initial != row.initial()
        {
            return false;
        }
        match (&self.value, row.value()) {
            (
                SelectedEventValue::Scalar(bits),
                EventValue::Digital { .. } | EventValue::Real(_),
            ) => *bits == row.value().identity(),
            (
                SelectedEventValue::Bus {
                    label,
                    members,
                    codes,
                },
                EventValue::Bus(_),
            ) => buses
                .iter()
                .find(|bus| bus.name() == row.trace_name())
                .is_some_and(|bus| {
                    bus.label() == label.as_str()
                        && bus.members() == members.as_slice()
                        && bus
                            .events()
                            .get(row.point_index())
                            .is_some_and(|(_, current)| current == codes)
                }),
            _ => false,
        }
    }
}

const BUS_RADIX_OPTIONS: [&str; 4] = ["BIN", "HEX", "DEC", "±DEC"];

fn bus_radix_index(radix: BusRadix) -> usize {
    match radix {
        BusRadix::Binary => 0,
        BusRadix::Hex => 1,
        BusRadix::Unsigned => 2,
        BusRadix::Signed => 3,
    }
}

fn bus_radix_from_index(index: usize) -> BusRadix {
    match index {
        1 => BusRadix::Hex,
        2 => BusRadix::Unsigned,
        3 => BusRadix::Signed,
        _ => BusRadix::Binary,
    }
}

trait EventRowSelection {
    fn selection(
        &self,
        analysis: AnalysisPresentationKey,
        buses: &[BusTimeline],
    ) -> Option<DigitalEventSelection>;
}

impl EventRowSelection for EventRow<'_> {
    fn selection(
        &self,
        analysis: AnalysisPresentationKey,
        buses: &[BusTimeline],
    ) -> Option<DigitalEventSelection> {
        let value = match self.value() {
            EventValue::Bus(_) => {
                let bus = buses.iter().find(|bus| bus.name() == self.trace_name())?;
                SelectedEventValue::Bus {
                    label: bus.label().to_owned(),
                    members: bus.members().to_vec(),
                    codes: bus.events().get(self.point_index())?.1.clone(),
                }
            }
            _ => SelectedEventValue::Scalar(self.value().identity()),
        };
        Some(DigitalEventSelection {
            analysis,
            source: self.source(),
            trace_name: self.trace_name().to_owned(),
            point_index: self.point_index(),
            time_bits: self.time_s().to_bits(),
            initial: self.initial(),
            value,
        })
    }
}

fn waveform_is_event(waveform: &WaveformData) -> bool {
    rspice_results::events::projection::waveform_is_event(waveform, || {
        super::frame_work::note(super::frame_work::DatasetWalk::EventProjectionScan);
    })
}

/// Whether this analysis has an event schedule worth a sheet.
///
/// A live partial qualifies alongside a completed run: the events it carries
/// are the ones the engine has already committed, arriving as the run accepts
/// them, and the manifest states the run is still going. Withholding the sheet
/// until the run ends would hide evidence the result model already holds,
/// which is exactly what the waveform sheets stopped doing when live partial
/// analog landed.
pub(super) fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    (analysis.success || analysis.is_live_partial())
        && analysis.analysis_type == AnalysisType::Transient
        && (matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                current_impulses,
                ..
            }) if !digital_traces.is_empty() || !real_traces.is_empty() || current_impulses.is_some()
        ) || analysis.waveforms.iter().any(waveform_is_event))
}

/// Whether the tab strip offers an events sheet for the active analysis.
///
/// The validity half resolves the workspace memo rather than peeking into it.
/// Peeking and treating a miss as a pass meant evidence nobody had validated
/// yet — which is every analysis on the frame it first appears — was offered
/// whatever validation would have said about it, while the SOA, Smith and
/// Optimization gates beside it were failing closed on the same question.
/// Resolving it also fills the memo, so the walk happens once per dataset
/// generation rather than once per gate.
pub(super) fn active_analysis_is_renderable(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    let Some(analysis) = state.simulation.active_analysis() else {
        return false;
    };
    super::analysis_answers_structural_gate(
        state,
        run.dataset_id,
        analysis,
        super::StructuralGate::EventHistory,
    ) && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
}

fn build_event_order(
    analysis: &AnalysisResult,
    expanded: &std::collections::BTreeSet<String>,
) -> EventOrder {
    super::frame_work::note(super::frame_work::DatasetWalk::EventOrder);
    rspice_results::events::projection::build_event_order(analysis, expanded, || {
        super::frame_work::note(super::frame_work::DatasetWalk::EventProjectionScan);
    })
}

/// The whole sheet as rows, for the tests that read it without drawing.
#[cfg(test)]
fn event_rows_with(
    analysis: &AnalysisResult,
    radix: BusRadix,
    expanded: &std::collections::BTreeSet<String>,
) -> (EventOrder, Vec<String>) {
    let cache = build_event_order(analysis, expanded);
    let rows = cache
        .rows()
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, entry)| {
            event_row_from_entry(analysis, cache.buses(), radix, entry, index + 1).map(|row| {
                format!(
                    "{} {} {}",
                    row.signal_name(),
                    row.value().display(),
                    row.value().domain()
                )
            })
        })
        .collect();
    (cache, rows)
}

/// The scalar rows of a history, for the tests that read one without drawing.
///
/// No declaration is resolved here — a bus row needs the reassembled table,
/// which `event_rows_with` returns beside its rows. A bussed history read
/// through this helper reports its member rows only, which is what every
/// caller of it is asking about.
#[cfg(test)]
fn event_rows(analysis: &AnalysisResult) -> Vec<EventRow<'_>> {
    build_event_order(analysis, &std::collections::BTreeSet::new())
        .rows()
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, entry)| {
            event_row_from_entry(analysis, &[], BusRadix::Binary, entry, index + 1)
        })
        .collect()
}

/// Resolve one row after the caller has validated the history through
/// `event_order`. Only the selected sample and its immediate predecessor are
/// read; validating a complete legacy trace here would repeat it every frame.
fn event_row_for_selection<'a>(
    analysis: &'a AnalysisResult,
    buses: &'a [BusTimeline],
    radix: BusRadix,
    selection: &DigitalEventSelection,
) -> Option<EventRow<'a>> {
    let row = event_row_at_name(
        analysis,
        buses,
        radix,
        selection.source,
        &selection.trace_name,
        selection.point_index,
    )?;
    selection.matches(&row, buses).then_some(row)
}

fn show_current_impulses(
    ui: &mut Ui,
    history: &crate::state::CurrentImpulseHistoryEvidence,
    order: &EventOrder,
) {
    section_header(ui, "Current impulses", None);
    panel_note(
        ui,
        "Signed charge is reported in coulombs at the exact event time. Finite current waveforms are reported separately in amperes.",
    );
    mono(
        ui,
        &format!(
            "Retained interval: {:.17e} to {:.17e} s",
            history.start_time_s, history.stop_time_s
        ),
    );
    if !history.delivery_complete {
        panel_note(
            ui,
            "This preview did not retain the complete impulse history.",
        );
    }
    let min_width = ui.available_width().max(720.0);
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.current-impulses-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(min_width);
            show_current_impulse_tables(ui, history, order);
        });
}

fn show_current_impulse_tables(
    ui: &mut Ui,
    history: &crate::state::CurrentImpulseHistoryEvidence,
    order: &EventOrder,
) {
    ui.collapsing("Current coverage", |ui| {
        TableBuilder::new(ui)
            .id_salt("rspice.results.current-coverage")
            .striped(true)
            .max_scroll_height(180.0)
            .column(Column::remainder().clip(true))
            .column(Column::initial(100.0).clip(true))
            .column(Column::initial(120.0).clip(true))
            .header(HEADER_HEIGHT, |mut header| {
                for label in ["CURRENT", "EVENTS", "COVERAGE"] {
                    header.col(|ui| table_header(ui, label));
                }
            })
            .body(|body| {
                body.rows(ROW_HEIGHT, history.traces.len(), |mut row| {
                    let index = row.index();
                    let trace = &history.traces[index];
                    row.col(|ui| {
                        mono(ui, &order.current_names()[index]);
                    });
                    row.col(|ui| {
                        mono(ui, &trace.points.len().to_string());
                    });
                    row.col(|ui| {
                        ui.label(if trace.complete && history.delivery_complete {
                            "Complete"
                        } else {
                            "Incomplete"
                        });
                    });
                })
            });
    });
    if order.current_rows().is_empty() {
        panel_note(
            ui,
            if history.traces.is_empty() {
                "No current coverage was retained. Absence of events does not establish zero impulse charge."
            } else {
                "No charge events were retained in this time window. See Current coverage for the currents whose histories are complete."
            },
        );
        return;
    }
    let height = (ui.available_height() * 0.45).max(120.0);
    TableBuilder::new(ui)
        .id_salt("rspice.results.current-impulses")
        .striped(true)
        .max_scroll_height(height)
        .column(Column::initial(180.0).clip(true))
        .column(Column::remainder().at_least(100.0).clip(true))
        .column(Column::initial(210.0).clip(true))
        .header(HEADER_HEIGHT, |mut header| {
            for label in ["TIME (s)", "CURRENT", "SIGNED CHARGE (C)"] {
                header.col(|ui| table_header(ui, label));
            }
        })
        .body(|body| {
            body.rows(ROW_HEIGHT, order.current_rows().len(), |mut row| {
                let (trace_index, point_index) = order.current_rows()[row.index()];
                let point = &history.traces[trace_index].points[point_index];
                row.col(|ui| {
                    mono(ui, &format!("{:.17e}", point.time));
                });
                row.col(|ui| {
                    mono(ui, &order.current_names()[trace_index]);
                });
                row.col(|ui| {
                    mono(ui, &format!("{:.17e}", point.charge_coulombs));
                });
            })
        });
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some((analysis_key, structurally_renderable)) =
        state.simulation.active_run().and_then(|run| {
            state.simulation.active_analysis().map(|analysis| {
                (
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    super::analysis_answers_structural_gate(
                        state,
                        run.dataset_id,
                        analysis,
                        super::StructuralGate::EventHistory,
                    ),
                )
            })
        })
    else {
        well_hint(ui, "Select a dataset with retained event traces");
        return;
    };
    if !structurally_renderable {
        well_hint(ui, "The active analysis has no valid retained event traces");
        return;
    }
    if !super::retained_evidence_is_valid(state, analysis_key) {
        well_hint(ui, "The retained event evidence is invalid");
        return;
    }
    let Some(cache) = event_order(state) else {
        return;
    };
    let analysis = state
        .simulation
        .active_analysis()
        .expect("active analysis resolved above");
    let radix = state.ui.results.event_bus_radix;
    let exact = cache.exact();
    let expanded = &state.ui.results.expanded_event_buses;
    StripHeader::new(
        "EVENTS",
        &format!(
            "{} · {} {}{}",
            analysis.label,
            cache.rows().len() + cache.current_rows().len(),
            if exact {
                "retained events"
            } else {
                "projected changes"
            },
            bus_subtitle(cache.buses()),
        ),
        &[],
    )
    .show(ui);
    if !cache.rows().is_empty()
        && let Some(note) = EventOrigin::active(state).note()
    {
        panel_note(ui, note);
    }
    if let Some(AnalysisResultPayload::TransientEvents {
        current_impulses: Some(history),
        ..
    }) = analysis.result_payload.as_ref()
    {
        show_current_impulses(ui, history, &cache);
        if cache.rows().is_empty() {
            return;
        }
    }
    if !exact {
        panel_note(
            ui,
            "Sampled waveform projection. Original sparse event timestamps are unavailable.",
        );
    }
    for note in bus_notes(cache.buses(), radix) {
        panel_note(ui, &note);
    }

    let selected = state.ui.results.selected_digital_event.clone();
    let mut requested = None;
    let mut toggled_bus = None;
    let min_width = 908.0_f32.max(ui.available_width());
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.events-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(min_width);
            TableBuilder::new(ui)
                .id_salt("rspice.results.events")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                // Every column clips. `egui_extras` grows a cell that does not
                // fit and pushes the rest of its row along with it, so one
                // long value used to shift four columns on that row alone and
                // the table stopped being a table wherever the numbers were
                // widest. The time column is sized so its own widest content
                // — a `{:.17e}` second with a three-digit exponent — never
                // reaches the clip.
                .column(Column::initial(84.0).clip(true))
                .column(Column::initial(176.0).clip(true))
                .column(Column::remainder().at_least(190.0).clip(true))
                .column(Column::initial(152.0).clip(true))
                .column(Column::initial(128.0).clip(true))
                .column(Column::initial(112.0).clip(true))
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
                    body.rows(ROW_HEIGHT, cache.rows().len(), |mut row| {
                        let row_index = row.index();
                        let Some(row_data) =
                            cache.rows().get(row_index).copied().and_then(|entry| {
                                event_row_from_entry(
                                    analysis,
                                    cache.buses(),
                                    radix,
                                    entry,
                                    row_index + 1,
                                )
                            })
                        else {
                            return;
                        };
                        let is_selected = selected.as_ref().is_some_and(|selection| {
                            selection.analysis == analysis_key
                                && selection.matches(&row_data, cache.buses())
                        });
                        row.set_selected(is_selected);
                        row.col(|ui| {
                            if ui
                                .selectable_label(
                                    is_selected,
                                    format!("#{:04}", row_data.event_ordinal()),
                                )
                                .clicked()
                            {
                                requested = row_data.selection(analysis_key, cache.buses());
                            }
                        });
                        row.col(|ui| {
                            mono(ui, &format!("{:.17e} s", row_data.time_s()));
                        });
                        row.col(|ui| {
                            mono(ui, row_data.signal_name());
                            // The disclosure rides the bus row rather than a
                            // seventh column: it belongs to the one row it
                            // acts on, and a column that is blank on every
                            // scalar row would cost the table width to say
                            // nothing about them.
                            if row_data.source() == EventSelectionSource::Bus {
                                let open = expanded.contains(row_data.trace_name());
                                let response = chip(ui, "BITS", open);
                                // The chip reads `BITS` on every bus row, so
                                // its published name says which bus it opens
                                // — two declarations on screen are otherwise
                                // the same word to anything not looking at
                                // the column beside it.
                                let name = format!("Bits of {}", row_data.signal_name());
                                response.widget_info(|| {
                                    egui::WidgetInfo::selected(
                                        egui::WidgetType::SelectableLabel,
                                        ui.is_enabled(),
                                        open,
                                        &name,
                                    )
                                });
                                let response = response.on_hover_text(if open {
                                    "Hide the member rows this word is reassembled from"
                                } else {
                                    "Show the member rows this word is reassembled from"
                                });
                                if response.clicked() {
                                    toggled_bus = Some(row_data.trace_name().to_owned());
                                }
                            }
                        });
                        row.col(|ui| {
                            let text = row_data.value().display();
                            let response = mono(ui, &text);
                            // A word is as wide as the bus is: the column
                            // clips rather than shoving the table along, so
                            // the whole word has to be reachable without
                            // selecting the row.
                            if row_data.source() == EventSelectionSource::Bus {
                                response.on_hover_text(text);
                            }
                        });
                        row.col(|ui| {
                            ui.label(row_data.value().domain());
                        });
                        row.col(|ui| {
                            ui.label(if row_data.initial() {
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
    if let Some(bus) = toggled_bus
        && !state.ui.results.expanded_event_buses.remove(&bus)
    {
        state.ui.results.expanded_event_buses.insert(bus);
    }
}

/// The Radix control, in the sheet's own domain bar.
///
/// One setting for the sheet rather than one per bus: a radix is how the
/// reader is reading, not a property of any one declaration, and the sheets
/// beside this one keep their domain controls the same way.
pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let declares_bus = context
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.result_payload.as_ref())
        .is_some_and(|payload| {
            matches!(
                payload,
                AnalysisResultPayload::TransientEvents { digital_buses, .. }
                    if !digital_buses.is_empty()
            )
        });
    if !declares_bus {
        return false;
    }
    let mut index = bus_radix_index(context.results.event_bus_radix);
    if segmented(
        ui,
        "rspice.results.events.radix",
        &BUS_RADIX_OPTIONS,
        &mut index,
        SegmentedWidth::Natural,
    ) {
        context.results.event_bus_radix = bus_radix_from_index(index);
    }
    true
}

/// Why the event inspector has nothing to show for a selection that exists.
const EVENT_SELECTION_NO_DATASET: &str = "No dataset is open, so the selected event has nothing to resolve against. \
     Open the dataset it was taken from, or select an event row here.";
const EVENT_SELECTION_UNRETAINED_ANALYSIS: &str = "The analysis this event was selected from is no longer retained in the \
     active dataset. Select an event row again.";
const EVENT_SELECTION_OTHER_ANALYSIS: &str = "Select an event row in the active analysis.";
const EVENT_SELECTION_UNRETAINED_ROW: &str =
    "The selected event has changed or is no longer retained. Select an event row again.";
const EVENT_SELECTION_OTHER_DATASET: &str =
    "Open the dataset this event was selected from, or select an event in the current dataset.";
const EVENT_SELECTION_INVALID_EVIDENCE: &str = "The selected analysis does not contain valid, available event evidence. Repair or replace the results to inspect this event.";

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

/// Resolve the selection independently of whether a sheet was painted.
/// Navigation and invalid evidence can recover; changed or removed events
/// require a new selection instead of silently adopting their replacements.
fn event_selection_block(
    state: &mut AppState,
    selection: &DigitalEventSelection,
) -> Option<EventSelectionBlock> {
    let block = |note, stale| Some(EventSelectionBlock { note, stale });
    let Some(run) = state.simulation.active_run() else {
        return block(EVENT_SELECTION_NO_DATASET, false);
    };
    let Some((analysis_index, _)) = selection.analysis.resolve(run) else {
        if state
            .simulation
            .runs
            .iter()
            .any(|run| selection.analysis.resolve(run).is_some())
        {
            return block(EVENT_SELECTION_OTHER_DATASET, false);
        }
        return block(EVENT_SELECTION_UNRETAINED_ANALYSIS, true);
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        return block(EVENT_SELECTION_OTHER_ANALYSIS, false);
    }
    let Some(order) = event_order(state) else {
        return block(EVENT_SELECTION_INVALID_EVIDENCE, false);
    };
    let analysis = state.simulation.active_analysis()?;
    if event_row_for_selection(
        analysis,
        order.buses(),
        state.ui.results.event_bus_radix,
        selection,
    )
    .is_none()
    {
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
    let radix = state.ui.results.event_bus_radix;
    let Some(order) = event_order(state) else {
        return;
    };
    let buses = order.buses();
    let Some(event) = state
        .simulation
        .active_run()
        .and_then(|run| selection.analysis.resolve(run))
        .and_then(|(_, analysis)| event_row_for_selection(analysis, buses, radix, &selection))
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
        ("Signal", event.signal_name().to_owned(), true),
        ("Physical time", format!("{:.17e} s", event.time_s()), true),
        (
            if event.exact() {
                "Trace event"
            } else {
                "Source sample"
            },
            format!("#{}", selection.point_index + 1),
            false,
        ),
        ("Value", event.value().display(), true),
        ("Domain", event.value().domain().to_owned(), false),
    ];
    let origin = EventOrigin::active(state);
    stats.push(("Source", origin.label().to_owned(), false));
    if let EventOrigin::Imported(source) = origin {
        stats.push(("Import file", source.source_name.clone(), false));
    }
    let mut bus_fallback = None;
    match event.value() {
        EventValue::Digital { code, .. } => {
            stats.push(("Retained code", code.to_string(), false));
            stats.push((
                if matches!(origin, EventOrigin::Native) {
                    "Drive strength"
                } else {
                    "Encoded strength"
                },
                event.value().strength().to_owned(),
                false,
            ));
        }
        EventValue::Bus(word) => {
            stats.push(("Radix", radix.label().to_owned(), false));
            if let Some(bus) = buses.iter().find(|bus| bus.name() == selection.trace_name) {
                stats.push(("Members", bus.members().len().to_string(), false));
                stats.push(("Bit order", bus.members().join(" "), false));
            }
            bus_fallback = word.fallback();
        }
        EventValue::Real(_) => {}
    }
    stat_table(ui, &stats);
    if let Some(note) = origin.note() {
        panel_note(ui, note);
    }
    if let Some(reason) = bus_fallback {
        panel_note(ui, &format!("Not shown in {}: {reason}.", radix.label()));
    }
    panel_note(
        ui,
        match event.source() {
            EventSelectionSource::Bus => {
                "This word is reassembled from the member histories beside it, at the exact time one of them changed. Every member keeps its own retained event code; the word holds no value they do not."
            }
            _ if event.exact() => {
                "This is a retained sparse event timestamp. Same-time transitions are preserved in per-trace order; cross-node delta-cycle ordering is not retained by the result contract."
            }
            _ => {
                "This row was reconstructed from sampled waveform values. Its original sparse event timestamp is unavailable."
            }
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

fn mono(ui: &mut Ui, text: &str) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_impulses_offer_an_exact_event_sheet_without_digital_nodes() {
        let mut history = crate::state::current_impulse_history_fixture();
        let mut second = history.traces[0].clone();
        second.owner = rspice_core::CurrentImpulseOwner::DeviceLead {
            device_name: "Q1".into(),
            parameter: "ic".into(),
        };
        second.points[0].time = 0.2;
        history.traces.push(second);
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces: vec![],
                real_traces: vec![],
                digital_buses: vec![],
                current_impulses: Some(history),
            },
        );
        assert!(analysis_is_renderable(&analysis));
        let order = build_event_order(&analysis, &Default::default());
        assert!(order.exact());
        assert_eq!(order.current_names(), ["I(V1)", "@Q1[ic]"]);
        assert_eq!(order.current_rows(), [(1, 0), (0, 0)]);
        assert!(order.rows().is_empty());
    }

    pub(super) fn committed_events(digital: &[(f64, u8)]) -> AnalysisResultPayload {
        AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: digital
                    .iter()
                    .map(
                        |(time_s, value_code)| crate::state::DigitalEventPointEvidence {
                            time_s: *time_s,
                            value_code: *value_code,
                        },
                    )
                    .collect(),
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        }
    }

    #[test]
    fn a_running_analysis_offers_the_events_it_has_already_committed() {
        let running = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        assert!(
            running.is_live_partial(),
            "the fixture must be the provisional result the controller publishes"
        );
        assert!(
            analysis_is_renderable(&running),
            "a run still accepting points has committed events worth showing"
        );

        let failed = AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "converge")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        assert!(
            !analysis_is_renderable(&failed),
            "only a live partial joins successful results; a failure stays out"
        );
    }

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
        assert!(rows[0].initial());
        assert_eq!(rows[1].point_index(), 2);
        assert!(matches!(
            rows[1].value(),
            EventValue::Digital { code: 1, .. }
        ));
        assert!(!rows[0].exact());
    }

    #[test]
    fn real_event_rows_preserve_projected_value_changes() {
        let waveform = WaveformData::new("E(control)", vec![1.0, 2.0], vec![0.25, 0.5], "#fff");
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].initial());
        assert_eq!(rows[0].time_s(), 1.0);
        assert!(matches!(rows[1].value(), EventValue::Real(value) if *value == 0.5));
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
    fn exact_event_rows_preserve_between_sample_and_same_time_transitions() {
        let payload = AnalysisResultPayload::TransientEvents {
            current_impulses: None,
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
            digital_buses: Vec::new(),
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
        assert_eq!(rows[1].time_s().to_bits(), 0.5_f64.to_bits());
        assert_eq!(rows[2].time_s().to_bits(), 0.5_f64.to_bits());
        assert_eq!(rows[1].point_index(), 1);
        assert_eq!(rows[2].point_index(), 2);
        assert!(rows.iter().all(EventRow::exact));
    }

    #[test]
    fn exact_event_order_is_deterministic_without_claiming_cross_node_delta_order() {
        let payload = AnalysisResultPayload::TransientEvents {
            current_impulses: None,
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
            digital_buses: Vec::new(),
        };
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(payload);
        let rows = event_rows(&analysis);

        assert_eq!(
            rows.iter().map(|row| row.signal_name()).collect::<Vec<_>>(),
            ["a", "a", "z"]
        );
        assert_eq!(rows[0].point_index(), 0);
        assert_eq!(rows[1].point_index(), 1);
    }

    #[test]
    fn selection_resolution_rejects_a_projected_non_change_sample() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let analysis_key =
            AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
        let changed = event_rows(&analysis)[1]
            .selection(analysis_key, &[])
            .unwrap();
        let stale = DigitalEventSelection {
            point_index: 1,
            ..changed.clone()
        };

        assert!(event_row_for_selection(&analysis, &[], BusRadix::Binary, &stale).is_none());
        assert_eq!(
            event_row_for_selection(&analysis, &[], BusRadix::Binary, &changed)
                .expect("changed projected sample")
                .point_index(),
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
        let selection = event_rows(&analysis)[1]
            .selection(AnalysisPresentationKey::new(dataset_id, &analysis), &[])
            .unwrap();
        assert_eq!(
            event_selection_block(&mut state, &selection),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_NO_DATASET,
                stale: false,
            })
        );

        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        // The row resolves: nothing blocks the inspector.
        assert_eq!(event_selection_block(&mut state, &selection), None);

        // A selection naming an analysis this dataset never retained.
        let unretained = DigitalEventSelection {
            analysis: AnalysisPresentationKey::new(
                dataset_id,
                &AnalysisResult::new(99, AnalysisType::Transient, "GONE"),
            ),
            ..selection.clone()
        };
        assert_eq!(
            event_selection_block(&mut state, &unretained),
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
            event_selection_block(&mut state, &past_the_end),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_UNRETAINED_ROW,
                stale: true,
            })
        );

        // A selection on a retained analysis that is not the active one.
        assert!(state.simulation.select_analysis(1));
        assert_eq!(
            event_selection_block(&mut state, &selection),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_OTHER_ANALYSIS,
                stale: false,
            })
        );
    }
}

#[cfg(test)]
mod availability_tests {
    use super::*;
    use crate::state::SimulationRun;

    /// Availability fails closed on evidence nobody has validated yet.
    ///
    /// The gate read the workspace validity memo and treated a miss as a
    /// pass, so an analysis whose retained evidence has never been checked —
    /// which is every analysis on the frame it first appears — was offered
    /// regardless of what validation would have said. Its siblings (SOA,
    /// Smith, Optimization) resolve the memo instead of peeking at it, and
    /// therefore close.
    #[test]
    fn never_validated_evidence_is_not_offered_as_an_events_sheet() {
        let payload = AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![crate::state::DigitalEventPointEvidence {
                    time_s: 0.25,
                    value_code: 0,
                }],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        };
        // A waveform with more coordinates than values is exactly what
        // `validate_retained_evidence` exists to refuse.
        let mut corrupt = WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
        corrupt.y = std::sync::Arc::new(vec![0.0]);
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![corrupt])
            .with_result_payload(payload);
        assert!(
            analysis.validate_retained_evidence().is_err(),
            "the fixture has to be invalid for the gate to have anything to refuse"
        );
        assert!(
            analysis_is_renderable(&analysis),
            "the fixture has to be renderable so only validity can close the gate"
        );

        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run].into();
        assert!(state.simulation.select_run(0));
        assert!(
            !active_analysis_is_renderable(&state),
            "an unvalidated, invalid analysis was offered its sheet"
        );
    }
}

#[cfg(test)]
mod bus_tests {
    use super::*;
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence,
    };

    /// BUS-L2's counter: `count[1:0]` over `count#1` and `count#0`, counting
    /// `00 01 10 11` on a 5 ns tick — the history that lane pinned end to end.
    fn counter(members: &[(&str, &[(f64, u8)])], msb: i64, lsb: i64) -> AnalysisResult {
        let digital_traces = members
            .iter()
            .map(|(name, points)| DigitalEventTraceEvidence {
                node_name: (*name).to_owned(),
                points: points
                    .iter()
                    .map(|(time_s, value_code)| DigitalEventPointEvidence {
                        time_s: *time_s,
                        value_code: *value_code,
                    })
                    .collect(),
            })
            .collect();
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces: Vec::new(),
                current_impulses: None,
                digital_buses: vec![DigitalBusEvidence {
                    name: "count".to_owned(),
                    msb,
                    lsb,
                    members: members.iter().map(|(name, _)| (*name).to_owned()).collect(),
                    source: DigitalBusSourceEvidence::Engine,
                }],
            },
        )
    }

    pub(super) fn two_bit_counter() -> AnalysisResult {
        counter(
            &[
                ("count#1", &[(0.0, 0), (10.0e-9, 1)]),
                (
                    "count#0",
                    &[(0.0, 0), (5.0e-9, 1), (10.0e-9, 0), (15.0e-9, 1)],
                ),
            ],
            1,
            0,
        )
    }

    fn collapsed() -> std::collections::BTreeSet<String> {
        std::collections::BTreeSet::new()
    }

    #[test]
    fn a_declared_bus_replaces_its_member_rows_with_one_word_per_change() {
        let analysis = two_bit_counter();
        let (cache, rows) = event_rows_with(&analysis, BusRadix::Binary, &collapsed());
        assert_eq!(cache.buses().len(), 1);
        assert_eq!(
            rows,
            vec![
                "count[1:0] 00 digital bus".to_owned(),
                "count[1:0] 01 digital bus".to_owned(),
                "count[1:0] 10 digital bus".to_owned(),
                "count[1:0] 11 digital bus".to_owned(),
            ],
            "a collapsed bus lists its four words and none of its six member changes"
        );
    }

    #[test]
    fn expanding_a_bus_lists_its_members_beside_the_word() {
        let analysis = two_bit_counter();
        let expanded = std::collections::BTreeSet::from(["count".to_owned()]);
        let (_, rows) = event_rows_with(&analysis, BusRadix::Binary, &expanded);
        assert_eq!(
            rows.iter()
                .filter(|row| row.starts_with("count[1:0]"))
                .count(),
            4
        );
        assert_eq!(
            rows.iter().filter(|row| row.starts_with("count#")).count(),
            6,
            "every member change is listed once the bus is opened to its bits"
        );
    }

    #[test]
    fn each_radix_spells_the_same_word_and_says_when_it_cannot() {
        let analysis = two_bit_counter();
        let last = |radix| {
            event_rows_with(&analysis, radix, &collapsed())
                .1
                .last()
                .cloned()
                .expect("the counter has bus rows")
        };
        assert_eq!(last(BusRadix::Binary), "count[1:0] 11 digital bus");
        assert_eq!(last(BusRadix::Hex), "count[1:0] 0x3 digital bus");
        assert_eq!(last(BusRadix::Unsigned), "count[1:0] 3 digital bus");
        assert_eq!(
            last(BusRadix::Signed),
            "count[1:0] -1 digital bus",
            "two bits of ones is minus one, signed at the declared width"
        );

        // Code 12 is high impedance and code 2 is unknown: neither denotes a
        // digit, so every radix but binary falls back and says how many words.
        let unresolved = counter(
            &[
                ("count#1", &[(0.0, 0), (10.0e-9, 12)]),
                ("count#0", &[(0.0, 2)]),
            ],
            1,
            0,
        );
        let (cache, rows) = event_rows_with(&unresolved, BusRadix::Hex, &collapsed());
        assert_eq!(
            rows,
            vec![
                "count[1:0] 0x digital bus".to_owned(),
                "count[1:0] zx digital bus".to_owned(),
            ]
        );
        assert_eq!(
            bus_notes(cache.buses(), BusRadix::Hex),
            vec![
                "2 bus words are not shown in hexadecimal: the word carries unknown (x) or high-impedance (z) bits, which denote no integer."
            ]
        );
        assert!(
            bus_notes(cache.buses(), BusRadix::Binary).is_empty(),
            "binary spells every word a run can produce, so it never has a note"
        );
    }

    #[test]
    fn a_member_the_run_never_stated_reads_as_unknown_until_it_does() {
        // `count#1` has no point before 10 ns, so the run has not said what
        // the bit is — which VCD spells `x`, exactly as the dump does.
        let analysis = counter(
            &[
                ("count#1", &[(10.0e-9, 1)]),
                ("count#0", &[(0.0, 0), (10.0e-9, 1)]),
            ],
            1,
            0,
        );
        let (_, rows) = event_rows_with(&analysis, BusRadix::Binary, &collapsed());
        assert_eq!(
            rows,
            vec![
                "count[1:0] x0 digital bus".to_owned(),
                "count[1:0] 11 digital bus".to_owned(),
            ]
        );
    }

    #[test]
    fn a_result_with_no_declaration_paints_no_bus_row_and_no_bus_subtitle() {
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                current_impulses: None,
                digital_traces: vec![DigitalEventTraceEvidence {
                    node_name: "clk".to_owned(),
                    points: vec![
                        DigitalEventPointEvidence {
                            time_s: 0.0,
                            value_code: 0,
                        },
                        DigitalEventPointEvidence {
                            time_s: 1.0e-9,
                            value_code: 1,
                        },
                    ],
                }],
                real_traces: Vec::new(),
                digital_buses: Vec::new(),
            },
        );
        let (cache, rows) = event_rows_with(&analysis, BusRadix::Hex, &collapsed());
        assert!(cache.buses().is_empty());
        assert!(bus_subtitle(cache.buses()).is_empty());
        assert!(bus_notes(cache.buses(), BusRadix::Hex).is_empty());
        assert!(rows.iter().all(|row| row.ends_with("digital")));
    }

    #[test]
    fn the_subtitle_counts_the_declarations_and_their_members() {
        let analysis = two_bit_counter();
        let (cache, _) = event_rows_with(&analysis, BusRadix::Binary, &collapsed());
        assert_eq!(bus_subtitle(cache.buses()), " · 1 bus over 2 members");
    }
}
