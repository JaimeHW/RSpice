//! Retained event tables, bus controls and source-aware selection readouts.

use crate::{
    chrome::bars::bar_label,
    fft::view::columns::fitted_to,
    presentation::{panel_note, stat_table, table_row_menu, well_hint, with_table_state},
    waves::logic::RunMarkers,
};
use egui::{RichText, Ui};
use rspice_results::{
    analysis_payload::AnalysisResultPayload,
    analysis_result::{AnalysisResult, RunChecks},
    events::projection::{
        BusRadix, BusTimeline, EventOrder, EventRow, EventSelectionSource, EventValue, bus_notes,
        bus_subtitle, event_row_at_name, event_row_from_entry,
    },
    result_presentation::AnalysisPresentationKey,
    waveform::RetainedWaveform,
};
use rspice_ui_kit::{
    panels::section_header,
    plot::{fmt_column_reading, fmt_si_significant},
    theme::{self, FontWeight},
    tokens::{self, Tokens},
    widgets::{
        Button, SegmentedWidth, segmented,
        table::{Align, Cell, Column, DataTable, SortKey},
    },
};
use rspice_vocabulary::quantity::{
    EngineeringPrecision, QuantityPresentationPolicy, format_engineering_value_with,
    parse_engineering_value,
};
use std::{collections::BTreeSet, sync::Arc};

/// The reader's significant digits and copied-value format, for every time,
/// charge and real value the event sheet and its inspector state.
///
/// The sheet states a number at the digits the reader chose, as every other
/// result sheet does. The exact value stays one hover or one copy away, in the
/// notation Copied values chose, as the cursor readout's copy is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventNumbers {
    pub significant_digits: usize,
    pub quantity_policy: QuantityPresentationPolicy,
}

impl EventNumbers {
    /// `value` in `unit` at the reader's significant digits, zeros kept, as
    /// a table cell or a readout row states it.
    fn shown(self, value: f64, unit: &str) -> String {
        fmt_column_reading(value, unit, self.significant_digits)
    }

    /// `value` in `unit` as a sentence states it: at most the reader's
    /// digits, the zeros that only pad it dropped.
    fn said(self, value: f64, unit: &str) -> String {
        fmt_si_significant(value, unit, self.significant_digits)
    }

    /// `value` in `unit` as Copied values writes it: every digit the double
    /// holds.
    fn exact(self, value: f64, unit: &str) -> String {
        self.quantity_policy
            .copy_si_value(value, unit)
            .trim_end()
            .to_owned()
    }

    /// An event's value as the table states it: a real value at the reader's
    /// digits, a logic level or bus word as it stands.
    fn value(self, value: &EventValue) -> String {
        match value {
            EventValue::Real(real) => self.shown(*real, ""),
            _ => value.display(),
        }
    }

    /// An event's value as a copy writes it.
    fn exact_value(self, value: &EventValue) -> String {
        match value {
            EventValue::Real(real) => self.exact(*real, ""),
            _ => value.display(),
        }
    }
}
#[derive(Clone, Copy)]
pub enum EventOrigin<'a> {
    Native,
    Imported(&'a rspice_results::result_import::ResultImportSource),
    Unrecorded,
}
impl EventOrigin<'_> {
    /// What the reader should know of where the events came from, said once,
    /// beside the selected event.
    fn note(self) -> Option<&'static str> {
        match self {
            Self::Native => None,
            Self::Imported(source)
                if matches!(
                    source.format,
                    rspice_results::result_import::ResultImportFormat::Vcd
                        | rspice_results::result_import::ResultImportFormat::Fst
                ) =>
            {
                Some(
                    "Read from the file as four-state logic. The import keeps no drive strength, so standard strengths are shown. The file does not prove which simulator wrote it.",
                )
            }
            Self::Imported(_) => Some(
                "Read from an imported file. A strength shown here is not the source's drive strength, and the file does not prove which simulator wrote it.",
            ),
            Self::Unrecorded => Some(
                "Where these events came from was not recorded, so a strength shown here is not a drive strength.",
            ),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalEventSelection {
    pub analysis: AnalysisPresentationKey,
    pub source: EventSelectionSource,
    pub trace_name: String,
    pub point_index: usize,
    time_bits: u64,
    initial: bool,
    value: SelectedEventValue,
}
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
impl DigitalEventSelection {
    #[cfg(any(test, feature = "test-support"))]
    pub fn initial(&self) -> bool {
        self.initial
    }
    #[cfg(any(test, feature = "test-support"))]
    pub fn time_bits(&self) -> u64 {
        self.time_bits
    }
}
pub trait EventRowSelection {
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

pub fn event_row_for_selection<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
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
pub enum EventAbsence {
    NoDataset,
    NoEvents,
    InvalidEvidence,
}
pub fn show_absent(ui: &mut Ui, absence: EventAbsence) {
    well_hint(
        ui,
        match absence {
            EventAbsence::NoDataset => "Select a dataset with retained event traces",
            EventAbsence::NoEvents => "The active analysis has no valid retained event traces",
            EventAbsence::InvalidEvidence => "The retained event evidence is invalid",
        },
    );
}

/// The Events sheet's two lists.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum EventPage {
    /// The signals' events.
    #[default]
    Events,
    /// What the run's checks reported: the HDL's severity records, the task
    /// that ended the run and the assertions' violations.
    Messages,
}

/// What the Events sheet lists, and which of its rows: those whose words
/// contain the search, at times inside the window.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventFilter {
    pub page: EventPage,
    pub search: String,
    /// The window's first time; `None` leaves it open.
    pub from: Option<f64>,
    /// The window's last time; `None` leaves it open.
    pub to: Option<f64>,
}

impl EventFilter {
    fn keeps_time(&self, time: f64) -> bool {
        self.from.is_none_or(|from| time >= from) && self.to.is_none_or(|to| time <= to)
    }

    /// Whether any of `words` contains the search, in any case.
    fn keeps_words<'w>(&self, words: impl IntoIterator<Item = &'w str>) -> bool {
        let search = self.search.trim();
        if search.is_empty() {
            return true;
        }
        let search = search.to_lowercase();
        words
            .into_iter()
            .any(|word| word.to_lowercase().contains(&search))
    }

    /// The window as the sheet's title states it, when one is set.
    fn window_words(&self, numbers: EventNumbers) -> Option<String> {
        let at = |time: f64| numbers.said(time, "s");
        match (self.from, self.to) {
            (None, None) => None,
            (Some(from), None) => Some(format!("from {}", at(from))),
            (None, Some(to)) => Some(format!("until {}", at(to))),
            (Some(from), Some(to)) => Some(format!("{} to {}", at(from), at(to))),
        }
    }
}

/// One message the run's checks reported, as the Messages list states it.
#[derive(Debug, Clone, PartialEq)]
pub struct EventMessage {
    pub time: f64,
    pub severity: &'static str,
    /// The message, on one line.
    pub text: String,
    /// The instance, or the check, that reported it.
    pub scope: String,
    /// The `file:line` it was written at, when the run names one.
    pub source: String,
}

/// One line of `text`, its lines joined by a space.
fn one_line(text: &str) -> String {
    text.lines().map(str::trim).collect::<Vec<_>>().join(" ")
}

/// The messages `checks` hold on an axis whose first and last instants are
/// `axis`, in time order: one row for each record and violation the
/// waveform's Checks strip marks.
/// The first and last instants of the analysis' waveforms, where the
/// messages of a run that kept no events stand.
pub fn waveform_axis<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
) -> Option<(f64, f64)> {
    analysis
        .waveforms
        .first()
        .and_then(|waveform| waveform.as_ref().x.ends())
}

pub fn event_messages(checks: &RunChecks, axis: (f64, f64)) -> Vec<EventMessage> {
    let Some(markers) = RunMarkers::of(checks, axis) else {
        return Vec::new();
    };
    let records = markers.hdl.iter().map(|marker| EventMessage {
        time: marker.time,
        severity: marker.severity.label(),
        text: one_line(&marker.text),
        scope: marker.scope.clone(),
        source: marker
            .source
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    });
    let violations = markers.assertions.iter().map(|span| EventMessage {
        time: span.start,
        severity: if span.error {
            "Assertion error"
        } else {
            "Assertion"
        },
        text: one_line(span.message.as_deref().unwrap_or(&span.check)),
        scope: span.label.clone(),
        source: span
            .source
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    });
    let unchecked = markers.stop.iter().map(|stop| EventMessage {
        time: stop.time,
        severity: "Unchecked",
        text: format!("{} expected outputs fall after the stop time", stop.beyond),
        scope: String::new(),
        source: stop
            .source
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
    });
    let mut messages: Vec<EventMessage> = records.chain(violations).chain(unchecked).collect();
    messages.sort_by(|a, b| a.time.total_cmp(&b.time));
    messages
}

/// The first and last instants of the analysis' axis, where its messages
/// stand: its waveforms', else its first and last events'.
pub fn event_axis<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    order: &EventOrder,
) -> (f64, f64) {
    if let Some(axis) = waveform_axis(analysis) {
        return axis;
    }
    let time = |index: usize| {
        order.rows().get(index).copied().and_then(|entry| {
            event_row_from_entry(
                analysis,
                order.buses(),
                BusRadix::default(),
                entry,
                index + 1,
            )
            .map(|row| row.time_s())
        })
    };
    match (time(0), time(order.rows().len().saturating_sub(1))) {
        (Some(first), Some(last)) => (first, last),
        _ => (0.0, 0.0),
    }
}

/// The rows of an event order a filter keeps, and, for each column, the
/// kept row whose words are the widest, which the columns are sized from.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeptEvents {
    /// Indices into the order's rows.
    pub rows: Vec<usize>,
    /// Positions in `rows`.
    widest: Vec<usize>,
}

/// The cells of one event row.
fn event_cells(
    row: &EventRow<'_>,
    numbers: EventNumbers,
    expanded: &BTreeSet<String>,
) -> Vec<Cell> {
    let mut signal = Cell::text(row.signal_name());
    if row.source() == EventSelectionSource::Bus {
        signal = signal
            .twist(expanded.contains(row.trace_name()))
            .hint("Show or hide the member rows this word is reassembled from");
    }
    let mut value = Cell::quantity(numbers.value(row.value()));
    match row.value() {
        // A word is as wide as the bus: the whole word, and a real
        // value's every digit, are one hover away.
        EventValue::Bus(_) => value = value.hint(numbers.exact_value(row.value())),
        EventValue::Real(real) => {
            value = value
                .hint(numbers.exact_value(row.value()))
                .sort_by(SortKey::Number(*real));
        }
        _ => {}
    }
    #[allow(clippy::cast_precision_loss)]
    let ordinal = row.event_ordinal() as f64;
    vec![
        Cell::number(format!("#{:04}", row.event_ordinal())).sort_by(SortKey::Number(ordinal)),
        Cell::quantity(numbers.shown(row.time_s(), "s"))
            .hint(numbers.exact(row.time_s(), "s"))
            .sort_by(SortKey::Number(row.time_s())),
        signal,
        value,
        Cell::text(row.value().domain()),
        Cell::text(if row.initial() { "initial" } else { "change" }),
    ]
}

/// The event sheet's heads: where each column's words sit, and whether in
/// mono.
const EVENT_HEADS: [(&str, Align, bool); 6] = [
    ("Event", Align::Right, true),
    ("Physical time", Align::Right, true),
    ("Signal", Align::Left, false),
    ("Value", Align::Right, true),
    ("Domain", Align::Left, false),
    ("Kind", Align::Left, false),
];

/// The rows of `order` that `filter` keeps, searching each row's words as
/// the table states them.
pub fn kept_events<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    order: &EventOrder,
    radix: BusRadix,
    numbers: EventNumbers,
    filter: &EventFilter,
) -> KeptEvents {
    let expanded = BTreeSet::new();
    let mut kept = KeptEvents {
        rows: Vec::new(),
        widest: vec![0; EVENT_HEADS.len()],
    };
    let mut widths = [0_usize; EVENT_HEADS.len()];
    for (index, entry) in order.rows().iter().copied().enumerate() {
        let Some(row) = event_row_from_entry(analysis, order.buses(), radix, entry, index + 1)
        else {
            continue;
        };
        if !filter.keeps_time(row.time_s()) {
            continue;
        }
        let cells = event_cells(&row, numbers, &expanded);
        if !filter.keeps_words(cells.iter().skip(2).map(|cell| cell.text.as_str())) {
            continue;
        }
        for (column, cell) in cells.iter().enumerate() {
            let width = cell.text.chars().count();
            if width > widths[column] {
                widths[column] = width;
                kept.widest[column] = kept.rows.len();
            }
        }
        kept.rows.push(index);
    }
    kept
}

/// Where the sheet keeps the rows its filter kept, with what they were
/// kept for.
fn kept_id() -> egui::Id {
    egui::Id::new("rspice.results.events.kept")
}

/// Where the list states how much of it is shown, for the bar to read out.
fn status_id() -> egui::Id {
    egui::Id::new("rspice.results.events.status")
}

/// What the bar reads out right: the whole count with its window, and the
/// count alone for a bar too narrow for both.
#[derive(Debug, Clone, Default)]
struct EventStatus {
    pass: u64,
    full: String,
    count: String,
}

fn set_status(ui: &Ui, count: String, rest: String) {
    let status = EventStatus {
        pass: ui.ctx().cumulative_pass_nr(),
        full: format!("{count}{rest}"),
        count,
    };
    ui.data_mut(|data| data.insert_temp(status_id(), status));
}

pub struct EventView<'a, W: AsRef<RetainedWaveform>> {
    pub analysis_key: AnalysisPresentationKey,
    pub analysis: &'a AnalysisResult<W>,
    pub order: &'a EventOrder,
    pub origin: EventOrigin<'a>,
    pub radix: BusRadix,
    pub expanded: &'a BTreeSet<String>,
    pub selected: Option<&'a DigitalEventSelection>,
    pub numbers: EventNumbers,
    pub filter: &'a EventFilter,
    /// The run's check messages, from [`event_messages`].
    pub messages: &'a [EventMessage],
}
#[derive(Default)]
pub struct EventResponse {
    pub requested: Option<DigitalEventSelection>,
    pub toggled_bus: Option<String>,
    /// A message row was clicked: the cursor goes to its time.
    pub place_cursor: Option<f64>,
    /// A message row was opened: the cursor goes to its time, on Waves.
    pub show_in_waves: Option<f64>,
}
fn show_current_impulses(
    ui: &mut Ui,
    history: &rspice_results::current_impulses::CurrentImpulseHistoryEvidence,
    order: &EventOrder,
    numbers: EventNumbers,
) {
    section_header(ui, "Current impulses", None);
    panel_note(
        ui,
        "Signed charge is reported in coulombs at the exact event time. Finite current waveforms are reported separately in amperes.",
    );
    mono(
        ui,
        &format!(
            "Retained interval: {} to {}",
            numbers.said(history.start_time_s, "s"),
            numbers.said(history.stop_time_s, "s")
        ),
    )
    .on_hover_text(format!(
        "{} to {}",
        numbers.exact(history.start_time_s, "s"),
        numbers.exact(history.stop_time_s, "s")
    ));
    if !history.delivery_complete {
        panel_note(
            ui,
            "This preview did not retain the complete impulse history.",
        );
    }
    show_current_impulse_tables(ui, history, order, numbers);
}
fn show_current_impulse_tables(
    ui: &mut Ui,
    history: &rspice_results::current_impulses::CurrentImpulseHistoryEvidence,
    order: &EventOrder,
    numbers: EventNumbers,
) {
    ui.collapsing("Current coverage", |ui| {
        let columns = [
            Column::name("Current", 180.0),
            Column::number("Events", 100.0),
            Column::words("Coverage", 120.0),
        ];
        with_table_state(ui, "current-coverage", |ui, state| {
            DataTable::new(
                "rspice.results.current-coverage",
                &columns,
                history.traces.len(),
            )
            .state(state)
            .max_height(180.0)
            .show(ui, |index| {
                let trace = &history.traces[index];
                let count = trace.points.len();
                vec![
                    Cell::text(&order.current_names()[index]),
                    Cell::number(count.to_string()).sort_by(SortKey::Number(count as f64)),
                    Cell::text(if trace.complete && history.delivery_complete {
                        "Complete"
                    } else {
                        "Incomplete"
                    }),
                ]
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
    let columns = [
        Column::number("Time", 180.0),
        Column::name("Current", 100.0),
        Column::number("Signed charge", 210.0),
    ];
    let point = |row: usize| {
        let (trace_index, point_index) = order.current_rows()[row];
        (
            &history.traces[trace_index].points[point_index],
            &order.current_names()[trace_index],
        )
    };
    let response = with_table_state(ui, "current-impulses", |ui, state| {
        DataTable::new(
            "rspice.results.current-impulses",
            &columns,
            order.current_rows().len(),
        )
        .state(state)
        .multi_select()
        .freeze(1)
        .max_height(height)
        .revision(egui::Id::new(("current-impulses", order.current_rows().len())).value())
        .show(ui, |row| {
            let (point, name) = point(row);
            vec![
                Cell::quantity(numbers.shown(point.time, "s"))
                    .hint(numbers.exact(point.time, "s"))
                    .sort_by(SortKey::Number(point.time)),
                Cell::text(name),
                Cell::quantity(numbers.shown(point.charge_coulombs, "C"))
                    .hint(numbers.exact(point.charge_coulombs, "C"))
                    .sort_by(SortKey::Number(point.charge_coulombs)),
            ]
        })
    });
    table_row_menu(ui, "current-impulses", response.context_menu, |ui, row| {
        if ui.button("Copy exact row").clicked() {
            let (point, name) = point(row);
            ui.ctx().copy_text(format!(
                "{}\t{name}\t{}",
                numbers.exact(point.time, "s"),
                numbers.exact(point.charge_coulombs, "C")
            ));
            ui.close();
        }
    });
}
/// A number or name in the sheet's mono face. It is not selectable text: a
/// drag would copy the digits the cell shows, and the exact value is what
/// the row's Copy exact row and the cell's hover give instead.
fn mono(ui: &mut Ui, text: &str) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            RichText::new(text)
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text),
        )
        .selectable(false),
    )
}
pub fn show<W: AsRef<RetainedWaveform>>(ui: &mut Ui, source: &EventView<'_, W>) -> EventResponse {
    let analysis_key = source.analysis_key;
    let analysis = source.analysis;
    let cache = source.order;
    let radix = source.radix;
    let expanded = source.expanded;
    let selected = source.selected;
    let numbers = source.numbers;
    let filter = source.filter;
    let exact = cache.exact();
    if filter.page == EventPage::Messages && !source.messages.is_empty() {
        return show_messages(ui, analysis, source.messages, filter, numbers);
    }
    // The rows the search and the window keep, found again only when the
    // rows' words or the filter change.
    let revision = egui::Id::new((
        "events",
        analysis_key,
        cache.rows().len(),
        bus_radix_index(radix),
        expanded,
    ))
    .value();
    let kept_key = egui::Id::new((
        revision,
        filter.search.trim(),
        filter.from.map(f64::to_bits),
        filter.to.map(f64::to_bits),
        numbers.significant_digits,
    ))
    .value();
    let kept = match ui.data(|data| data.get_temp::<(u64, Arc<KeptEvents>)>(kept_id())) {
        Some((key, kept)) if key == kept_key => kept,
        _ => {
            let kept = Arc::new(kept_events(analysis, cache, radix, numbers, filter));
            ui.data_mut(|data| data.insert_temp(kept_id(), (kept_key, Arc::clone(&kept))));
            kept
        }
    };
    let total = cache.rows().len() + cache.current_rows().len();
    let shown = kept.rows.len() + cache.current_rows().len();
    let noun = if exact {
        "retained events"
    } else {
        "projected changes"
    };
    let counted = if shown == total {
        format!("{total} {noun}")
    } else {
        format!("{shown} of {total} {noun}")
    };
    let window = filter
        .window_words(numbers)
        .map(|window| format!(" · {window}"))
        .unwrap_or_default();
    set_status(
        ui,
        counted,
        format!("{window}{}", bus_subtitle(cache.buses())),
    );
    if let Some(AnalysisResultPayload::TransientEvents {
        current_impulses: Some(history),
        ..
    }) = analysis.result_payload.as_ref()
    {
        show_current_impulses(ui, history, cache, numbers);
        if cache.rows().is_empty() {
            return EventResponse::default();
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

    let mut requested = None;
    let mut toggled_bus = None;
    // A table row is a kept row; the event keeps its place in the whole
    // order, so its number does not change as the filter narrows.
    let row_at = |row: usize| {
        let index = *kept.rows.get(row)?;
        cache.rows().get(index).copied().and_then(|entry| {
            event_row_from_entry(analysis, cache.buses(), radix, entry, index + 1)
        })
    };
    let cells_at = |row: usize| {
        row_at(row).map_or_else(
            || vec![Cell::absent(); EVENT_HEADS.len()],
            |event| event_cells(&event, numbers, expanded),
        )
    };
    // Each column is as wide as its head, the widest of its kept rows and
    // the rows at either end.
    let count = kept.rows.len();
    let sizing: Vec<Vec<Cell>> = (0..count.min(SIZING_ROWS))
        .chain(count.saturating_sub(SIZING_ROWS)..count)
        .chain(kept.widest.iter().copied().filter(|row| *row < count))
        .map(cells_at)
        .collect();
    let columns = fitted_to(ui, &EVENT_HEADS, &sizing);
    if count == 0 && !cache.rows().is_empty() {
        well_hint(ui, "No event matches the search and window");
    }
    let response = with_table_state(ui, "events", |ui, state| {
        // Rows the filter kept are numbered afresh, so the table looks for
        // the page's event again whenever the filter changes.
        let kept_seen = ui.id().with("rspice.results.events-kept");
        if ui.data(|data| data.get_temp::<u64>(kept_seen)) != Some(kept_key) {
            let remembered = page_selection_id(ui);
            ui.data_mut(|data| {
                data.insert_temp(kept_seen, kept_key);
                data.remove::<Option<DigitalEventSelection>>(remembered);
            });
        }
        follow_page_selection(ui, state, selected, analysis_key, count, |row| {
            row_at(row).is_some_and(|row| {
                selected.is_some_and(|selection| selection.matches(&row, cache.buses()))
            })
        });
        let response = DataTable::new("rspice.results.events", &columns, count)
            .state(state)
            .freeze(1)
            .revision(kept_key)
            .max_height(ui.available_height())
            .show(ui, cells_at);
        if response.clicked.is_some() || response.selection_changed {
            requested = state
                .selected_row()
                .and_then(row_at)
                .and_then(|row| row.selection(analysis_key, cache.buses()));
            remember_page_selection(ui, requested.as_ref());
        }
        response
    });
    if let Some((index, _)) = response.expand
        && let Some(row) = row_at(index)
    {
        toggled_bus = Some(row.trace_name().to_owned());
    }
    table_row_menu(ui, "events", response.context_menu, |ui, index| {
        if ui.button("Copy exact row").clicked() {
            if let Some(row) = row_at(index) {
                ui.ctx().copy_text(format!(
                    "#{:04}\t{}\t{}\t{}\t{}\t{}",
                    row.event_ordinal(),
                    numbers.exact(row.time_s(), "s"),
                    row.signal_name(),
                    numbers.exact_value(row.value()),
                    row.value().domain(),
                    if row.initial() { "initial" } else { "change" },
                ));
            }
            ui.close();
        }
    });
    EventResponse {
        requested,
        toggled_bus,
        ..EventResponse::default()
    }
}

/// Rows read from each end of a list to size its columns, beside the
/// widest of its rows.
const SIZING_ROWS: usize = 32;

/// The tone a message's severity is set in: errors and fatal ends in the
/// error tone, warnings and unchecked outputs in the warning tone.
fn severity_tone(severity: &str) -> rspice_ui_kit::widgets::table::Tone {
    use rspice_ui_kit::widgets::table::Tone;
    match severity {
        "Error" | "Fatal" | "Assertion error" => Tone::Error,
        "Warning" | "Assertion" | "Unchecked" => Tone::Caution,
        _ => Tone::Neutral,
    }
}

/// The Messages list: every record and violation of the run's checks the
/// search and the window keep. A click puts the cursor on its time; opening
/// a row, or Show in Waves, goes there on the waveform sheet.
/// The Messages list of a run that kept no events, which is the whole sheet.
pub fn show_message_list<W: AsRef<RetainedWaveform>>(
    ui: &mut Ui,
    analysis: &AnalysisResult<W>,
    messages: &[EventMessage],
    filter: &EventFilter,
    numbers: EventNumbers,
) -> EventResponse {
    show_messages(ui, analysis, messages, filter, numbers)
}

fn show_messages<W: AsRef<RetainedWaveform>>(
    ui: &mut Ui,
    analysis: &AnalysisResult<W>,
    messages: &[EventMessage],
    filter: &EventFilter,
    numbers: EventNumbers,
) -> EventResponse {
    let kept: Vec<usize> = messages
        .iter()
        .enumerate()
        .filter(|(_, message)| {
            filter.keeps_time(message.time)
                && filter.keeps_words([
                    message.severity,
                    message.text.as_str(),
                    message.scope.as_str(),
                    message.source.as_str(),
                ])
        })
        .map(|(index, _)| index)
        .collect();
    let counted = if kept.len() == messages.len() {
        format!("{} messages", messages.len())
    } else {
        format!("{} of {} messages", kept.len(), messages.len())
    };
    let window = filter
        .window_words(numbers)
        .map(|window| format!(" · {window}"))
        .unwrap_or_default();
    set_status(ui, counted, window);
    if kept.is_empty() {
        well_hint(ui, "No message matches the search and window");
        return EventResponse::default();
    }
    let rows: Vec<Vec<Cell>> = kept
        .iter()
        .map(|&index| {
            let message = &messages[index];
            let words = |text: &str| {
                if text.is_empty() {
                    Cell::empty()
                } else {
                    Cell::text(text)
                }
            };
            vec![
                Cell::quantity(numbers.shown(message.time, "s"))
                    .hint(numbers.exact(message.time, "s"))
                    .sort_by(SortKey::Number(message.time)),
                Cell::text(message.severity).toned(severity_tone(message.severity)),
                words(&message.text),
                words(&message.scope),
                if message.source.is_empty() {
                    Cell::empty()
                } else {
                    Cell::number(message.source.as_str())
                },
            ]
        })
        .collect();
    let columns = fitted_to(
        ui,
        &[
            ("Time", Align::Right, true),
            ("Severity", Align::Left, false),
            ("Message", Align::Left, false),
            ("Instance", Align::Left, false),
            ("Source", Align::Left, true),
        ],
        &rows,
    );
    let revision = egui::Id::new((
        "rspice.results.event-messages",
        analysis.id,
        &kept,
        numbers.significant_digits,
    ))
    .value();
    let response = with_table_state(ui, "event-messages", |ui, state| {
        DataTable::new("rspice.results.event-messages", &columns, kept.len())
            .state(state)
            .freeze(1)
            .revision(revision)
            .max_height(ui.available_height())
            .show(ui, |row| rows[row].clone())
    });
    let time_of = |row: usize| kept.get(row).map(|&index| messages[index].time);
    let mut out = EventResponse {
        place_cursor: response.clicked.and_then(time_of),
        show_in_waves: response.activate.and_then(time_of),
        ..EventResponse::default()
    };
    table_row_menu(ui, "event-messages", response.context_menu, |ui, row| {
        let Some(message) = kept.get(row).map(|&index| &messages[index]) else {
            return;
        };
        if ui.button("Show in Waves").clicked() {
            out.show_in_waves = Some(message.time);
            ui.close();
        }
        if ui.button("Copy message").clicked() {
            ui.ctx().copy_text(format!(
                "{}\t{}\t{}\t{}\t{}",
                numbers.exact(message.time, "s"),
                message.severity,
                message.text,
                message.scope,
                message.source,
            ));
            ui.close();
        }
    });
    out
}

fn page_selection_id(ui: &Ui) -> egui::Id {
    ui.id().with("rspice.results.events-page-selection")
}

/// Note the event the table last handed the page, so the table does not
/// look for it again.
fn remember_page_selection(ui: &Ui, selection: Option<&DigitalEventSelection>) {
    let id = page_selection_id(ui);
    let selection = selection.cloned();
    ui.data_mut(|data| data.insert_temp(id, selection));
}

/// Put the table's selection on the event the page has selected, when the
/// page chose it elsewhere (on a plot, in the inspector) or the rows
/// changed under it. The rows are searched once per change, never on a
/// frame where the table and the page already agree.
fn follow_page_selection(
    ui: &Ui,
    state: &mut rspice_ui_kit::widgets::table::TableState,
    selected: Option<&DigitalEventSelection>,
    analysis: AnalysisPresentationKey,
    rows: usize,
    matches: impl Fn(usize) -> bool,
) {
    let id = page_selection_id(ui);
    let seen = ui.data(|data| data.get_temp::<Option<DigitalEventSelection>>(id));
    let selected = selected.filter(|selection| selection.analysis == analysis);
    if seen.as_ref().map(Option::as_ref) == Some(selected)
        && state.selected_row().is_none_or(|row| row < rows)
    {
        return;
    }
    remember_page_selection(ui, selected);
    match selected {
        None => state.selection = None,
        Some(_) => {
            if state.selected_row().is_some_and(&matches) {
                return;
            }
            match (0..rows).find(|row| matches(*row)) {
                Some(row) => state.select(row, None),
                None => state.selection = None,
            }
        }
    }
}
const BUS_RADIX_OPTIONS: [&str; 6] = [
    BusRadix::ALL[0].short_label(),
    BusRadix::ALL[1].short_label(),
    BusRadix::ALL[2].short_label(),
    BusRadix::ALL[3].short_label(),
    BusRadix::ALL[4].short_label(),
    BusRadix::ALL[5].short_label(),
];
fn bus_radix_index(radix: BusRadix) -> usize {
    BusRadix::ALL
        .iter()
        .position(|candidate| *candidate == radix)
        .unwrap_or(0)
}
fn bus_radix_from_index(index: usize) -> BusRadix {
    BusRadix::ALL.get(index).copied().unwrap_or_default()
}
/// What the Events sheet's bar sets, and what it shows beside its controls.
pub struct EventBar<'a> {
    /// The bus radix, when the run declares a bus.
    pub radix: Option<&'a mut BusRadix>,
    pub filter: &'a mut EventFilter,
    /// How many messages the run's checks reported.
    pub messages: usize,
    /// Whether the analysis kept events. A run that kept none and whose
    /// checks reported messages shows its Messages list alone.
    pub events: bool,
}

/// What the reader asked of the bar beyond its settings.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct EventBarResponse {
    /// Open Verify's Checks view.
    pub open_checks: bool,
}

/// Width of a time typed in the Display menu.
const TIME_FIELD_WIDTH: f32 = 84.0;
/// Width of the search field.
const SEARCH_WIDTH: f32 = 140.0;

/// The Events sheet's bar: the list it shows, the search, and in Display
/// the time window and the bus radix; on the Messages list, the way to
/// Verify's Checks view.
pub fn domain_bar(ui: &mut Ui, bar: EventBar<'_>) -> EventBarResponse {
    let mut out = EventBarResponse::default();
    let filter = bar.filter;
    if bar.messages > 0 && bar.events {
        let pages = ["Events".to_owned(), format!("Messages {}", bar.messages)];
        let pages: Vec<&str> = pages.iter().map(String::as_str).collect();
        let mut index = usize::from(filter.page == EventPage::Messages);
        if segmented(
            ui,
            "rspice.results.events.page",
            &pages,
            &mut index,
            SegmentedWidth::Natural,
        ) {
            filter.page = if index == 1 {
                EventPage::Messages
            } else {
                EventPage::Events
            };
        }
    }
    // The field stands at the bar's control height, its words on its centre.
    let height = crate::chrome::ResultBarMetrics::of(ui).instrument_control;
    ui.add_sized(
        [SEARCH_WIDTH, height],
        egui::TextEdit::singleline(&mut filter.search)
            .id(egui::Id::new("rspice.results.events.search"))
            .font(theme::field_mono())
            .vertical_align(egui::Align::Center)
            .hint_text("Search…"),
    );
    let messages_shown = bar.messages > 0 && (filter.page == EventPage::Messages || !bar.events);
    display_menu(ui, filter, if messages_shown { None } else { bar.radix });
    if messages_shown
        && Button::new("Checks")
            .accessible_label("Open the run's checks in Verify")
            .show(ui)
            .clicked()
    {
        out.open_checks = true;
    }
    status(ui);
    out
}

/// How much of the list is shown, at the bar's right end: with its window
/// and buses when the bar holds them, the count alone when it holds only
/// that. The list states it as it draws, so the bar reads the last frame's.
fn status(ui: &mut Ui) {
    let Some(status) = ui.data(|data| data.get_temp::<EventStatus>(status_id())) else {
        return;
    };
    if status.pass + 1 < ui.ctx().cumulative_pass_nr() {
        return;
    }
    let font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let room = ui.available_width() - ui.spacing().item_spacing.x;
    let width = |text: &str| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::PLACEHOLDER)
            .size()
            .x
    };
    let text = if width(&status.full) <= room {
        status.full
    } else if width(&status.count) <= room {
        status.count
    } else {
        return;
    };
    let faint = Tokens::get(ui.ctx()).color.text_faint;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.label(RichText::new(text).font(font).color(faint));
    });
}

/// The Display menu: the time window, and the bus radix when there is one.
fn display_menu(ui: &mut Ui, filter: &mut EventFilter, radix: Option<&mut BusRadix>) {
    let display = Button::new("Display")
        .menu()
        .accessible_label("Event display options")
        .show(ui);
    egui::Popup::menu(&display)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            egui::Grid::new("rspice.results.events.display")
                .num_columns(2)
                .spacing([12.0, 6.0])
                .show(ui, |ui| {
                    bar_label(ui, "From");
                    ui.horizontal(|ui| {
                        if let Some(from) =
                            time_field(ui, "rspice.results.events.from", filter.from)
                        {
                            filter.from = from;
                        }
                    });
                    ui.end_row();
                    bar_label(ui, "To");
                    ui.horizontal(|ui| {
                        if let Some(to) = time_field(ui, "rspice.results.events.to", filter.to) {
                            filter.to = to;
                        }
                    });
                    ui.end_row();
                    if let Some(radix) = radix {
                        bar_label(ui, "Radix");
                        let mut index = bus_radix_index(*radix);
                        if segmented(
                            ui,
                            "rspice.results.events.radix",
                            &BUS_RADIX_OPTIONS,
                            &mut index,
                            SegmentedWidth::Natural,
                        ) {
                            *radix = bus_radix_from_index(index);
                        }
                        ui.end_row();
                    }
                });
        });
}

/// A time typed in engineering notation, in seconds: `Some(None)` for an
/// emptied field, `Some(Some(time))` for a time read, `None` while nothing
/// was committed.
fn time_field(ui: &mut Ui, id: &str, value: Option<f64>) -> Option<Option<f64>> {
    let shown = value.map_or_else(String::new, |value| {
        format_engineering_value_with(value, EngineeringPrecision::UpTo(6))
    });
    let id = ui.make_persistent_id(id);
    let mut text = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| shown.clone());
    let response = ui.add_sized(
        [TIME_FIELD_WIDTH, ui.spacing().interact_size.y],
        egui::TextEdit::singleline(&mut text)
            .font(theme::field_mono())
            .hint_text("open"),
    );
    bar_label(ui, "s");
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return None;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    if !response.lost_focus() || text == shown {
        return None;
    }
    if text.trim().is_empty() {
        return Some(None);
    }
    parse_engineering_value(text.trim())
        .ok()
        .filter(|time| time.is_finite())
        .map(Some)
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionAbsence {
    Unselected,
    NoDataset,
    UnretainedAnalysis,
    OtherAnalysis,
    UnretainedRow,
    OtherDataset,
    InvalidEvidence,
}
pub fn selection_absent(ui: &mut Ui, absence: SelectionAbsence) {
    section_header(ui, "Event selection", None);
    panel_note(
        ui,
        match absence {
            SelectionAbsence::Unselected => "Select an event.",
            SelectionAbsence::NoDataset => EVENT_SELECTION_NO_DATASET,
            SelectionAbsence::UnretainedAnalysis => EVENT_SELECTION_UNRETAINED_ANALYSIS,
            SelectionAbsence::OtherAnalysis => EVENT_SELECTION_OTHER_ANALYSIS,
            SelectionAbsence::UnretainedRow => EVENT_SELECTION_UNRETAINED_ROW,
            SelectionAbsence::OtherDataset => EVENT_SELECTION_OTHER_DATASET,
            SelectionAbsence::InvalidEvidence => EVENT_SELECTION_INVALID_EVIDENCE,
        },
    );
}
/// The selected event's details. Returns `true` when the reader asked for
/// the event's signal on the waveform plot, at the event's instant.
pub fn right_panel(
    ui: &mut Ui,
    event: &EventRow<'_>,
    selection: &DigitalEventSelection,
    buses: &[BusTimeline],
    radix: BusRadix,
    origin: EventOrigin<'_>,
    numbers: EventNumbers,
) -> bool {
    // An exact timestamp is the normal case; a reconstructed one qualifies
    // the time printed under it, so only that one says so.
    section_header(
        ui,
        "Selected event",
        (!event.exact()).then_some("from samples"),
    );
    // The point index, the value's domain and the encoded form of the 0/1/X/Z
    // shown above it are bookkeeping; a native source is the normal case.
    let mut stats = vec![
        ("Signal", event.signal_name().to_owned(), true),
        ("Physical time", numbers.shown(event.time_s(), "s"), true),
        ("Value", numbers.value(event.value()), true),
    ];
    if let EventOrigin::Imported(source) = origin {
        stats.push(("Source", source.format.canonical_id().to_owned(), false));
        stats.push(("Import file", source.source_name.clone(), false));
    }
    let mut bus_fallback = None;
    match event.value() {
        EventValue::Digital { .. } => {
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
            if let Some(bus) = buses.iter().find(|bus| bus.name() == selection.trace_name) {
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
    ui.button("Plot in waveforms").clicked()
}
#[cfg(any(test, feature = "test-support"))]
pub mod test_support {
    use super::*;
    use rspice_results::waveform::RetainedTrace;
    use rspice_results::{
        analysis_type::AnalysisType,
        events::{
            DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
            DigitalEventTraceEvidence,
        },
    };
    type AnalysisResult = rspice_results::analysis_result::AnalysisResult<RetainedTrace>;
    pub fn committed_events(digital: &[(f64, u8)]) -> AnalysisResultPayload {
        AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            voltage_impulses: None,
            digital_traces: vec![rspice_results::events::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: digital
                    .iter()
                    .map(
                        |(time_s, value_code)| rspice_results::events::DigitalEventPointEvidence {
                            time_s: *time_s,
                            value_code: *value_code,
                        },
                    )
                    .collect(),
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
            hdl_sources: Vec::new(),
        }
    }
    pub(super) fn counter(members: &[(&str, &[(f64, u8)])], msb: i64, lsb: i64) -> AnalysisResult {
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
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces: Vec::new(),
                current_impulses: None,
                voltage_impulses: None,
                digital_buses: vec![DigitalBusEvidence {
                    name: "count".to_owned(),
                    msb,
                    lsb,
                    members: members.iter().map(|(name, _)| (*name).to_owned()).collect(),
                    source: DigitalBusSourceEvidence::Engine,
                }],
                hdl_sources: Vec::new(),
            },
        )
    }
    pub fn two_bit_counter() -> AnalysisResult {
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
}
#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::waveform::RetainedTrace;
    use rspice_results::{
        analysis_type::AnalysisType,
        events::{DigitalEventPointEvidence, DigitalEventTraceEvidence},
    };
    use test_support::{counter, two_bit_counter};
    type AnalysisResult = rspice_results::analysis_result::AnalysisResult<RetainedTrace>;
    fn event_rows(analysis: &AnalysisResult) -> Vec<EventRow<'_>> {
        rspice_results::events::projection::build_event_order(
            analysis,
            &std::collections::BTreeSet::new(),
            || {},
        )
        .rows()
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, entry)| {
            event_row_from_entry(analysis, &[], BusRadix::Binary, entry, index + 1)
        })
        .collect()
    }
    fn event_rows_with(
        analysis: &AnalysisResult,
        radix: BusRadix,
        expanded: &std::collections::BTreeSet<String>,
    ) -> (EventOrder, Vec<String>) {
        let cache =
            rspice_results::events::projection::build_event_order(analysis, expanded, || {});
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
    fn collapsed() -> std::collections::BTreeSet<String> {
        std::collections::BTreeSet::new()
    }

    /// One frame of the event sheet over `events`, and where it drew.
    fn event_sheet_frame(
        ctx: &egui::Context,
        analysis_key: AnalysisPresentationKey,
        analysis: &AnalysisResult,
        order: &EventOrder,
        selected: Option<&DigitalEventSelection>,
        events: Vec<egui::Event>,
    ) -> (
        egui::FullOutput,
        rspice_ui_kit::probe::Geometry,
        Option<DigitalEventSelection>,
    ) {
        let expanded = collapsed();
        let mut requested = None;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 700.0),
            )),
            events,
            ..Default::default()
        };
        let (output, geometry) = rspice_ui_kit::probe::pass(ctx, input, |ui| {
            requested = show(
                ui,
                &EventView {
                    analysis_key,
                    analysis,
                    order,
                    origin: EventOrigin::Native,
                    radix: BusRadix::Binary,
                    expanded: &expanded,
                    selected,
                    numbers: EventNumbers {
                        significant_digits: 4,
                        quantity_policy: QuantityPresentationPolicy::default(),
                    },
                    filter: &EventFilter::default(),
                    messages: &[],
                },
            )
            .requested;
        });
        (output, geometry, requested)
    }

    fn numbers() -> EventNumbers {
        EventNumbers {
            significant_digits: 4,
            quantity_policy: QuantityPresentationPolicy::default(),
        }
    }

    /// A clock with four committed values, at 0, 5, 10 and 15 ns.
    fn clock() -> AnalysisResult {
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_result_payload(
            test_support::committed_events(&[(0.0, 0), (5.0e-9, 1), (10.0e-9, 0), (15.0e-9, 1)]),
        )
    }

    /// The window keeps the events inside it, ends included; the search
    /// keeps the rows whose words hold it, in any case; and together they
    /// keep what both keep. Each row keeps its place in the whole order.
    #[test]
    fn a_filter_narrows_the_list_to_the_window_and_the_search() {
        let analysis = clock();
        let order =
            rspice_results::events::projection::build_event_order(&analysis, &collapsed(), || {});
        assert_eq!(order.rows().len(), 4);
        let kept = |filter: &EventFilter| {
            kept_events(&analysis, &order, BusRadix::Binary, numbers(), filter).rows
        };
        assert_eq!(kept(&EventFilter::default()), vec![0, 1, 2, 3]);
        let window = EventFilter {
            from: Some(5.0e-9),
            to: Some(10.0e-9),
            ..EventFilter::default()
        };
        assert_eq!(kept(&window), vec![1, 2]);
        let initial = EventFilter {
            search: "INITIAL".to_owned(),
            ..EventFilter::default()
        };
        assert_eq!(kept(&initial), vec![0]);
        let changes = EventFilter {
            search: "change".to_owned(),
            from: Some(6.0e-9),
            ..EventFilter::default()
        };
        assert_eq!(kept(&changes), vec![2, 3]);
        let nothing = EventFilter {
            search: "vctl".to_owned(),
            ..EventFilter::default()
        };
        assert!(kept(&nothing).is_empty());
    }

    fn warning_at(time: f64, text: &str) -> rspice_results::analysis_result::HdlDiagnostic {
        rspice_results::analysis_result::HdlDiagnostic {
            severity: rspice_results::engine_evidence::HdlSeverity::Warning,
            text: text.to_owned(),
            point: rspice_results::engine_evidence::HdlPoint::Transient { time },
            origin: rspice_results::engine_evidence::HdlOrigin {
                scope: "tb.dut".to_owned(),
                file: "dut.va".to_owned(),
                line: 12,
            },
        }
    }

    /// A run's checks have messages exactly when the Checks strip marks
    /// something, so the Events sheet is offered for what it lists.
    #[test]
    fn checks_have_messages_exactly_when_the_strip_marks_them() {
        let mut checks = RunChecks::default();
        let axis = (0.0, 20.0e-9);
        assert!(!checks.has_messages());
        assert!(RunMarkers::of(&checks, axis).is_none());
        let mut info = warning_at(2.0e-9, "hello");
        info.severity = rspice_results::engine_evidence::HdlSeverity::Info;
        checks.hdl.push(info);
        assert!(!checks.has_messages());
        assert!(RunMarkers::of(&checks, axis).is_none());
        checks.hdl.push(warning_at(3.0e-9, "glitch"));
        assert!(checks.has_messages());
        assert!(RunMarkers::of(&checks, axis).is_some());
    }

    /// The Messages list states every record the Checks strip marks, in
    /// time order, each on one line with its instance and source line; a
    /// click on a row hands back its time for the cursor.
    #[test]
    fn a_message_row_hands_its_time_to_the_cursor() {
        let mut checks = RunChecks::default();
        checks.hdl.push(warning_at(9.0e-9, "late\nedge"));
        checks.hdl.push(warning_at(3.0e-9, "glitch"));
        let messages = event_messages(&checks, (0.0, 20.0e-9));
        let times: Vec<f64> = messages.iter().map(|message| message.time).collect();
        assert_eq!(times, vec![3.0e-9, 9.0e-9]);
        assert_eq!(messages[1].text, "late edge");
        assert_eq!(messages[1].scope, "tb.dut");
        assert_eq!(messages[1].source, "dut.va:12");

        let analysis = clock();
        let order =
            rspice_results::events::projection::build_event_order(&analysis, &collapsed(), || {});
        let key =
            AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis);
        let filter = EventFilter {
            page: EventPage::Messages,
            ..EventFilter::default()
        };
        let expanded = collapsed();
        let frame = |ctx: &egui::Context, events: Vec<egui::Event>| {
            let mut response = EventResponse::default();
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1200.0, 700.0),
                )),
                events,
                ..Default::default()
            };
            let (_, geometry) = rspice_ui_kit::probe::pass(ctx, input, |ui| {
                response = show(
                    ui,
                    &EventView {
                        analysis_key: key,
                        analysis: &analysis,
                        order: &order,
                        origin: EventOrigin::Native,
                        radix: BusRadix::Binary,
                        expanded: &expanded,
                        selected: None,
                        numbers: numbers(),
                        filter: &filter,
                        messages: &messages,
                    },
                );
            });
            (geometry, response)
        };
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let (geometry, _) = frame(&ctx, Vec::new());
        let second = geometry
            .all("table.cell.0")
            .into_iter()
            .max_by(|a, b| a.top().total_cmp(&b.top()))
            .expect("two message rows");
        let pos = second.center();
        let (_, response) = frame(
            &ctx,
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
        assert_eq!(response.place_cursor, Some(9.0e-9));
    }

    /// The event sheet draws through the shared table: each head stands over
    /// its own cells (the heads used to sit 12 px left of them), and Ctrl+C
    /// on a selected row copies it as tab-separated text under the heads.
    #[test]
    fn the_event_table_aligns_heads_with_cells_and_copies_rows_as_tab_separated_text() {
        let analysis = two_bit_counter();
        let order =
            rspice_results::events::projection::build_event_order(&analysis, &collapsed(), || {});
        assert!(order.rows().len() >= 2, "the fixture has events to list");
        let key =
            AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis);
        let ctx = egui::Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let (_, geometry, _) = event_sheet_frame(&ctx, key, &analysis, &order, None, Vec::new());
        for column in 0..6 {
            let head = geometry
                .one(&format!("table.head.{column}"))
                .unwrap_or_else(|| panic!("column {column}'s head is drawn"));
            let cell = geometry
                .one(&format!("table.cell.{column}"))
                .unwrap_or_else(|| panic!("column {column} has cells"));
            assert!(
                (head.left() - cell.left()).abs() < 0.5
                    && (head.right() - cell.right()).abs() < 0.5,
                "column {column}: head {head:?} over cells {cell:?}"
            );
        }
        let first = geometry
            .all("table.cell.0")
            .into_iter()
            .min_by(|a, b| a.top().total_cmp(&b.top()))
            .expect("a first row");
        let click = |pos: egui::Pos2| {
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                },
            ]
        };
        let (_, _, requested) =
            event_sheet_frame(&ctx, key, &analysis, &order, None, click(first.center()));
        let requested = requested.expect("a click on a row selects its event");
        let _ = event_sheet_frame(&ctx, key, &analysis, &order, Some(&requested), Vec::new());
        let (output, _, _) = event_sheet_frame(
            &ctx,
            key,
            &analysis,
            &order,
            Some(&requested),
            vec![egui::Event::Copy],
        );
        let copied = output
            .platform_output
            .commands
            .iter()
            .find_map(|command| match command {
                egui::OutputCommand::CopyText(text) => Some(text.clone()),
                _ => None,
            })
            .expect("Ctrl+C copies the selected row");
        let mut lines = copied.lines();
        assert_eq!(
            lines.next(),
            Some("Event\tPhysical time\tSignal\tValue\tDomain\tKind")
        );
        let row = lines.next().expect("the selected row");
        assert!(row.starts_with("#0001\t"), "{row}");
        assert_eq!(row.split('\t').count(), 6, "{row}");
    }

    #[test]
    fn event_rows_keep_initial_value_and_only_projected_changes() {
        let waveform = RetainedTrace::new(
            "D(clk)",
            vec![0.0, 1.0, 2.0, 3.0],
            vec![0.0, 0.0, 1.0, 1.0],
            "#fff",
        );
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
            .with_waveforms(vec![waveform]);
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
        let waveform = RetainedTrace::new("E(control)", vec![1.0, 2.0], vec![0.25, 0.5], "#fff");
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
            .with_waveforms(vec![waveform]);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].initial());
        assert_eq!(rows[0].time_s(), 1.0);
        assert!(matches!(rows[1].value(), EventValue::Real(value) if *value == 0.5));
    }
    #[test]
    fn real_event_trace_rejects_an_unknown_after_the_first_committed_value() {
        let waveform = RetainedTrace::new(
            "E(control)",
            vec![0.0, 1.0, 2.0],
            vec![f64::NAN, 0.25, f64::NAN],
            "#fff",
        );
        assert!(!rspice_results::events::projection::waveform_is_event(
            &waveform,
            || {}
        ));
    }
    #[test]
    fn exact_event_rows_preserve_between_sample_and_same_time_transitions() {
        let payload = AnalysisResultPayload::TransientEvents {
            current_impulses: None,
            voltage_impulses: None,
            digital_traces: vec![rspice_results::events::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![
                    rspice_results::events::DigitalEventPointEvidence {
                        time_s: 0.25,
                        value_code: 0,
                    },
                    rspice_results::events::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 1,
                    },
                    rspice_results::events::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 0,
                    },
                ]
                .into(),
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
            hdl_sources: Vec::new(),
        };
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
            .with_waveforms(vec![RetainedTrace::new(
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
            voltage_impulses: None,
            digital_traces: vec![
                rspice_results::events::DigitalEventTraceEvidence {
                    node_name: "a".to_owned(),
                    points: vec![
                        rspice_results::events::DigitalEventPointEvidence {
                            time_s: 0.5,
                            value_code: 0,
                        },
                        rspice_results::events::DigitalEventPointEvidence {
                            time_s: 0.5,
                            value_code: 1,
                        },
                    ]
                    .into(),
                },
                rspice_results::events::DigitalEventTraceEvidence {
                    node_name: "z".to_owned(),
                    points: vec![rspice_results::events::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 1,
                    }]
                    .into(),
                },
            ],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
            hdl_sources: Vec::new(),
        };
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
            .with_result_payload(payload);
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
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0).with_waveforms(vec![
                RetainedTrace::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let analysis_key =
            AnalysisPresentationKey::new(rspice_vocabulary::product::DatasetId::new(), &analysis);
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
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
            .with_result_payload(AnalysisResultPayload::TransientEvents {
                current_impulses: None,
                voltage_impulses: None,
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
                    ]
                    .into(),
                }],
                real_traces: Vec::new(),
                digital_buses: Vec::new(),
                hdl_sources: Vec::new(),
            });
        let (cache, rows) = event_rows_with(&analysis, BusRadix::Hex, &collapsed());
        assert!(cache.buses().is_empty());
        assert!(bus_subtitle(cache.buses()).is_empty());
        assert!(bus_notes(cache.buses(), BusRadix::Hex).is_empty());
        assert!(rows.iter().all(|row| row.ends_with("digital")));
    }
}
