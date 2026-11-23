//! What one undo or redo step of a view holds: only what the step changed.
//!
//! A step holds the entities its edit made, changed or took away, each as it
//! stood before, and nothing of the rest of the view. So a hundred small
//! edits of a view of a hundred thousand traces hold about a hundred traces,
//! not a hundred views: the memory of undo grows with the edits made, not
//! with the size of the view times the steps.
//!
//! A document lists each kind of entity in the order its entities were made,
//! by ascending identity, so a step addresses each entity by identity and
//! puts it back in its place in one pass. The few lists without identities
//! (per-pane settings, tombstones, comparison receipts) keep the one stretch
//! of the list the step changed.

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
    /// The step that puts the view `now` back as `then`, an earlier view of
    /// the same document, held it. The datasets a view reads are not its
    /// own and the title is the project's name for it, so neither is held.
    pub(super) fn back_to(then: &VisualizationDocument, now: &VisualizationDocument) -> Self {
        Self {
            document_id: now.id,
            tracking: changed(then.tracking, now.tracking),
            presentation: changed(then.presentation, now.presentation),
            working_view: changed(then.working_view, now.working_view),
            pages: Changed::back_to(&then.pages, &now.pages),
            panes: Changed::back_to(&then.panes, &now.panes),
            axes: Changed::back_to(&then.axes, &now.axes),
            traces: Changed::back_to(&then.traces, &now.traces),
            cursors: Changed::back_to(&then.cursors, &now.cursors),
            markers: Changed::back_to(&then.markers, &now.markers),
            measurements: Changed::back_to(&then.measurements, &now.measurements),
            annotations: Changed::back_to(&then.annotations, &now.annotations),
            link_groups: Changed::back_to(&then.link_groups, &now.link_groups),
            tombstones: Splice::back_to(&then.tombstones, &now.tombstones),
            comparisons: Splice::back_to(&then.comparisons, &now.comparisons),
            user_buses: Splice::back_to(&then.user_buses, &now.user_buses),
            event_lane_orders: Splice::back_to(&then.event_lane_orders, &now.event_lane_orders),
            expression_traces: Splice::back_to(&then.expression_traces, &now.expression_traces),
            xy_plots: Splice::back_to(&then.xy_plots, &now.xy_plots),
            readout_pins: Splice::back_to(&then.readout_pins, &now.readout_pins),
            sheet_settings: Splice::back_to(&then.sheet_settings, &now.sheet_settings),
            spectrum_settings: Splice::back_to(&then.spectrum_settings, &now.spectrum_settings),
            coefficient_settings: Splice::back_to(
                &then.coefficient_settings,
                &now.coefficient_settings,
            ),
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
        self.pages.put_back(&mut document.pages);
        self.panes.put_back(&mut document.panes);
        self.axes.put_back(&mut document.axes);
        self.traces.put_back(&mut document.traces);
        self.cursors.put_back(&mut document.cursors);
        self.markers.put_back(&mut document.markers);
        self.measurements.put_back(&mut document.measurements);
        self.annotations.put_back(&mut document.annotations);
        self.link_groups.put_back(&mut document.link_groups);
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
struct Changed<T> {
    out: Vec<u64>,
    back: Vec<T>,
}

impl<T: Identified + Clone + PartialEq> Changed<T> {
    /// What puts `now` back as `then`: both in ascending identity, read
    /// once each.
    fn back_to(then: &[T], now: &[T]) -> Self {
        let (mut out, mut back) = (Vec::new(), Vec::new());
        let (mut before, mut after) = (then.iter().peekable(), now.iter().peekable());
        loop {
            match (before.peek(), after.peek()) {
                (Some(was), Some(is)) if was.identity() == is.identity() => {
                    if was != is {
                        out.push(is.identity());
                        back.push((*was).clone());
                    }
                    before.next();
                    after.next();
                }
                (Some(was), Some(is)) if was.identity() < is.identity() => {
                    back.push((*was).clone());
                    before.next();
                }
                (Some(was), None) => {
                    back.push((*was).clone());
                    before.next();
                }
                (_, Some(is)) => {
                    out.push(is.identity());
                    after.next();
                }
                (None, None) => break,
            }
        }
        Self { out, back }
    }

    /// Take out of `list` what the step takes out and put back what it
    /// holds, each in its place by identity. An entity another edit took
    /// out since is not missed; one another edit made since stays.
    fn put_back(self, list: &mut Vec<T>) {
        if self.out.is_empty() && self.back.is_empty() {
            return;
        }
        let mut out = self.out.into_iter().peekable();
        let mut back = self.back.into_iter().peekable();
        let mut kept = Vec::with_capacity(list.len() + back.len());
        for item in list.drain(..) {
            let identity = item.identity();
            while out.next_if(|taken| *taken < identity).is_some() {}
            while let Some(earlier) = back.next_if(|held| held.identity() < identity) {
                kept.push(earlier);
            }
            let taken = out.next_if_eq(&identity).is_some();
            match back.next_if(|held| held.identity() == identity) {
                Some(held) => kept.push(held),
                None if taken => {}
                None => kept.push(item),
            }
        }
        kept.extend(back);
        *list = kept;
    }
}

/// The one stretch of a list without identities that a step changed: the
/// items now at `at..at + out`, and what stood there before.
#[derive(Debug, Clone, PartialEq)]
struct Splice<T> {
    at: usize,
    out: usize,
    back: Vec<T>,
}

impl<T: Clone + PartialEq> Splice<T> {
    fn back_to(then: &[T], now: &[T]) -> Self {
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

    fn put_back(self, list: &mut Vec<T>) {
        if self.out == 0 && self.back.is_empty() {
            return;
        }
        let at = self.at.min(list.len());
        let end = at.saturating_add(self.out).min(list.len());
        list.splice(at..end, self.back);
    }
}
