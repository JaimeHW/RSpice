//! What one undo or redo step of a view holds: only what the step changed.
//!
//! A step holds the entities its edit made, changed or took away, each as it
//! stood before, and nothing of the rest of the view. So a hundred small
//! edits of a view of a hundred thousand traces hold about a hundred traces,
//! not a hundred views: the memory of undo grows with the edits made, not
//! with the size of the view times the steps.
//!
//! A step is read off what its transaction kept while it ran, so making it
//! costs what the transaction changed. A document lists each kind of entity
//! in the order its entities were made, by ascending identity, so a step
//! addresses each entity by identity and puts it back in its place. The few
//! lists without identities (per-pane settings, tombstones, comparison
//! receipts) keep the one stretch of the list the step changed.

use super::journal::{Began, SpliceList};
use super::*;

/// What putting a view back as it stood holds: the entities and settings
/// one step changed, as they stood before it. Taking it gives the step that
/// reverses it, so undo and redo stay exact.
#[derive(Debug, Clone, PartialEq)]
pub struct ViewStep {
    document_id: ResultDocumentId,
    tracking: Option<ResultDocumentTracking>,
    presentation: Option<VisualizationPresentationPolicy>,
    working_view: Option<Option<WorkingViewSource>>,
    pages: Changed<Page>,
    panes: Changed<Pane>,
    axes: Changed<Axis>,
    traces: Changed<Trace>,
    cursors: Changed<Cursor>,
    markers: Changed<Marker>,
    measurements: Changed<Measurement>,
    annotations: Changed<Annotation>,
    link_groups: Changed<LinkGroup>,
    tombstones: Splice<Tombstone>,
    comparisons: Splice<ComparisonReceipt>,
    user_buses: Splice<UserBus>,
    event_lane_orders: Splice<EventLaneOrder>,
    expression_traces: Splice<ExpressionTrace>,
    xy_plots: Splice<XyPlot>,
    readout_pins: Splice<ReadoutPin>,
    sheet_settings: Splice<SheetSetting>,
    spectrum_settings: Splice<PaneSpectrumSettings>,
    coefficient_settings: Splice<PaneCoefficientSettings>,
}

impl ViewStep {
    /// The step that puts `document` back as it stood when the transaction
    /// ending now `began`, read off what the transaction kept. The datasets
    /// a view reads are not its own and the title is the project's name for
    /// it, so neither is held.
    pub(super) fn back_to(document: &mut VisualizationDocument, began: &Began) -> Self {
        Self {
            document_id: document.id,
            tracking: changed(began.tracking, document.tracking),
            presentation: changed(began.presentation, document.presentation),
            working_view: changed(began.working_view, document.working_view),
            pages: document.pages.finish(),
            panes: document.panes.finish(),
            axes: document.axes.finish(),
            traces: document.traces.finish(),
            cursors: document.cursors.finish(),
            markers: document.markers.finish(),
            measurements: document.measurements.finish(),
            annotations: document.annotations.finish(),
            link_groups: document.link_groups.finish(),
            tombstones: document.tombstones.finish(),
            comparisons: document.comparisons.finish(),
            user_buses: document.user_buses.finish(),
            event_lane_orders: document.event_lane_orders.finish(),
            expression_traces: document.expression_traces.finish(),
            xy_plots: document.xy_plots.finish(),
            readout_pins: document.readout_pins.finish(),
            sheet_settings: document.sheet_settings.finish(),
            spectrum_settings: document.spectrum_settings.finish(),
            coefficient_settings: document.coefficient_settings.finish(),
        }
    }

    /// How many traces the step holds.
    #[cfg(test)]
    pub(super) fn traces_held(&self) -> usize {
        self.traces.back.len()
    }

    /// The document the step belongs to.
    #[must_use]
    pub const fn document_id(&self) -> ResultDocumentId {
        self.document_id
    }

    /// The dataset bindings the panes, traces and expressions the step puts
    /// back read.
    pub(super) fn bindings(&self) -> impl Iterator<Item = DatasetBinding> + '_ {
        self.panes
            .back
            .iter()
            .filter_map(|pane| pane.binding.map(|binding| binding.dataset))
            .chain(self.traces.back.iter().map(|trace| trace.binding))
            .chain(
                self.expression_traces
                    .back
                    .iter()
                    .flat_map(|expression| expression.datasets.iter().copied()),
            )
    }

    /// Put back on `document` what the step holds.
    pub(super) fn put_back(self, document: &mut VisualizationDocument) {
        if let Some(tracking) = self.tracking {
            document.tracking = tracking;
        }
        if let Some(presentation) = self.presentation {
            document.presentation = presentation;
        }
        if let Some(working_view) = self.working_view {
            document.working_view = working_view;
        }
        document.pages.put_back(self.pages);
        document.panes.put_back(self.panes);
        document.axes.put_back(self.axes);
        document.traces.put_back(self.traces);
        document.cursors.put_back(self.cursors);
        document.markers.put_back(self.markers);
        document.measurements.put_back(self.measurements);
        document.annotations.put_back(self.annotations);
        document.link_groups.put_back(self.link_groups);
        self.tombstones.put_back(&mut document.tombstones);
        self.comparisons.put_back(&mut document.comparisons);
        self.user_buses.put_back(&mut document.user_buses);
        self.event_lane_orders
            .put_back(&mut document.event_lane_orders);
        self.expression_traces
            .put_back(&mut document.expression_traces);
        self.xy_plots.put_back(&mut document.xy_plots);
        self.readout_pins.put_back(&mut document.readout_pins);
        self.sheet_settings.put_back(&mut document.sheet_settings);
        self.spectrum_settings
            .put_back(&mut document.spectrum_settings);
        self.coefficient_settings
            .put_back(&mut document.coefficient_settings);
    }
}

/// `before`, when `after` differs from it.
fn changed<T: PartialEq>(before: T, after: T) -> Option<T> {
    (before != after).then_some(before)
}

/// An entity a document lists by identity.
pub(super) trait Identified {
    fn identity(&self) -> u64;
}

macro_rules! identified {
    ($($entity:ty),* $(,)?) => {
        $(impl Identified for $entity {
            fn identity(&self) -> u64 {
                self.id.get()
            }
        })*
    };
}

identified!(
    Page,
    Pane,
    Axis,
    Trace,
    Cursor,
    Marker,
    Measurement,
    Annotation,
    LinkGroup
);

/// The entities of one kind a step changed: the identities it takes out
/// (those the edit made or changed) and the entities it puts back as they
/// stood (those the edit changed or took away), each in ascending identity.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Changed<T> {
    pub(super) out: Vec<u64>,
    pub(super) back: Vec<T>,
}

impl<T> Default for Changed<T> {
    fn default() -> Self {
        Self {
            out: Vec::new(),
            back: Vec::new(),
        }
    }
}

/// The one stretch of a list without identities that a step changed: the
/// items now at `at..at + out`, and what stood there before.
#[derive(Debug, Clone, PartialEq)]
pub(super) struct Splice<T> {
    pub(super) at: usize,
    pub(super) out: usize,
    pub(super) back: Vec<T>,
}

impl<T> Default for Splice<T> {
    fn default() -> Self {
        Self {
            at: 0,
            out: 0,
            back: Vec::new(),
        }
    }
}

impl<T: Clone + PartialEq> Splice<T> {
    /// What puts `now` back as `then`: the stretch between the longest
    /// common head and tail.
    pub(super) fn back_to(then: &[T], now: &[T]) -> Self {
        let same = |(was, is): (&T, &T)| was == is;
        let head = then.iter().zip(now).take_while(|pair| same(*pair)).count();
        let tail = then[head..]
            .iter()
            .rev()
            .zip(now[head..].iter().rev())
            .take_while(|pair| same(*pair))
            .count();
        Self {
            at: head,
            out: now.len() - head - tail,
            back: then[head..then.len() - tail].to_vec(),
        }
    }

    fn put_back(self, list: &mut SpliceList<T>) {
        if self.out == 0 && self.back.is_empty() {
            return;
        }
        list.splice(self.at..self.at.saturating_add(self.out), self.back);
    }
}
