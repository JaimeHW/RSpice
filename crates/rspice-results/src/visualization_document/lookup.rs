//! What a check of one entity reads of the rest of its document: the other
//! entities, by identity. A document lists each kind of entity in the order
//! its entities were made, so an edit finds the few entities it touches by a
//! binary search of the document's lists; a whole document is checked
//! through a [`Lookup`], which indexes every entity once. Either way a check
//! costs in proportion to what it reads, and no count of anything needs a
//! ceiling to keep it quick.

use super::query::RowIndex;
use super::undo::Identified;
use super::*;

/// Where the entity `identity` stands in `list`, a list in ascending
/// identity.
pub(super) fn position<T: Identified>(list: &[T], identity: u64) -> Option<usize> {
    list.binary_search_by_key(&identity, Identified::identity)
        .ok()
}

/// Refuse a list of `kind` not in the order its entities were made:
/// ascending identities, each below `next_serial`, the identity the document
/// makes next.
pub(super) fn require_made_in_order<T: Identified>(
    kind: &'static str,
    list: &[T],
    next_serial: u64,
) -> Result<(), VisualizationError> {
    let mut previous = 0;
    for identity in list.iter().map(Identified::identity) {
        if identity <= previous || identity >= next_serial {
            return Err(VisualizationError::InvalidValue {
                field: kind,
                message: format!(
                    "the document lists {} out of the order they were made",
                    kind.rsplit('.').next().unwrap_or(kind)
                ),
            });
        }
        previous = identity;
    }
    Ok(())
}

/// The entities a check reads, by identity.
pub(super) trait Entities {
    fn pane(&self, id: PaneId) -> Result<&Pane, VisualizationError>;
    fn trace(&self, id: TraceId) -> Result<&Trace, VisualizationError>;
    fn require_page(&self, id: PageId) -> Result<(), VisualizationError>;
    fn require_axis_in_pane(&self, id: AxisId, pane_id: PaneId) -> Result<(), VisualizationError>;
    fn require_entity(&self, entity: EntityRef) -> Result<(), VisualizationError>;
    fn dataset(&self, binding: DatasetBinding) -> Result<&SourceDataset, VisualizationError>;
    /// The traces drawn on `pane_id`, in the document's order.
    fn traces_in_pane(&self, pane_id: PaneId) -> Vec<&Trace>;
    /// The rows of the embedded tables read so far by this check.
    fn rows(&self) -> &RowIndex;

    fn require_pane(&self, id: PaneId) -> Result<(), VisualizationError> {
        self.pane(id).map(|_| ())
    }

    fn require_trace_in_pane(
        &self,
        id: TraceId,
        pane_id: PaneId,
    ) -> Result<(), VisualizationError> {
        match self.trace(id) {
            Ok(trace) if trace.pane_id == pane_id => Ok(()),
            _ => Err(VisualizationError::EntityNotFound(EntityRef::Trace(id))),
        }
    }
}

impl Entities for VisualizationDocument {
    fn pane(&self, id: PaneId) -> Result<&Pane, VisualizationError> {
        position(&self.panes, id.get())
            .map(|index| &self.panes[index])
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(id)))
    }

    fn trace(&self, id: TraceId) -> Result<&Trace, VisualizationError> {
        position(&self.traces, id.get())
            .map(|index| &self.traces[index])
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Trace(id)))
    }

    fn require_page(&self, id: PageId) -> Result<(), VisualizationError> {
        Self::require_page(self, id)
    }

    fn require_axis_in_pane(&self, id: AxisId, pane_id: PaneId) -> Result<(), VisualizationError> {
        Self::require_axis_in_pane(self, id, pane_id)
    }

    fn require_entity(&self, entity: EntityRef) -> Result<(), VisualizationError> {
        Self::require_entity(self, entity)
    }

    fn dataset(&self, binding: DatasetBinding) -> Result<&SourceDataset, VisualizationError> {
        self.dataset_for_binding(binding)
    }

    fn traces_in_pane(&self, pane_id: PaneId) -> Vec<&Trace> {
        self.traces
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .collect()
    }

    fn rows(&self) -> &RowIndex {
        &self.row_index
    }
}

/// Every entity of one document by identity, read once.
pub(super) struct Lookup<'a> {
    pages: HashSet<PageId>,
    panes: HashMap<PaneId, &'a Pane>,
    axes: HashMap<AxisId, PaneId>,
    traces: HashMap<TraceId, &'a Trace>,
    traces_by_pane: HashMap<PaneId, Vec<&'a Trace>>,
    others: HashSet<EntityRef>,
    datasets: HashMap<DatasetId, &'a SourceDataset>,
    rows: &'a RowIndex,
}

impl<'a> Lookup<'a> {
    pub(super) fn of(document: &'a VisualizationDocument) -> Self {
        let mut traces_by_pane: HashMap<PaneId, Vec<&Trace>> = HashMap::new();
        for trace in &document.traces {
            traces_by_pane.entry(trace.pane_id).or_default().push(trace);
        }
        let others = document
            .cursors
            .iter()
            .map(|cursor| EntityRef::Cursor(cursor.id))
            .chain(
                document
                    .markers
                    .iter()
                    .map(|marker| EntityRef::Marker(marker.id)),
            )
            .chain(
                document
                    .measurements
                    .iter()
                    .map(|measurement| EntityRef::Measurement(measurement.id)),
            )
            .chain(
                document
                    .annotations
                    .iter()
                    .map(|annotation| EntityRef::Annotation(annotation.id)),
            )
            .chain(
                document
                    .link_groups
                    .iter()
                    .map(|group| EntityRef::LinkGroup(group.id)),
            )
            .collect();
        // A document that holds one identity or one dataset twice is refused
        // by its own check; the first of each is the one read meanwhile.
        let mut datasets = HashMap::new();
        for dataset in &document.datasets {
            datasets
                .entry(dataset.binding.dataset_id)
                .or_insert(dataset);
        }
        let mut panes = HashMap::new();
        for pane in &document.panes {
            panes.entry(pane.id).or_insert(pane);
        }
        let mut traces = HashMap::new();
        for trace in &document.traces {
            traces.entry(trace.id).or_insert(trace);
        }
        let mut axes = HashMap::new();
        for axis in &document.axes {
            axes.entry(axis.id).or_insert(axis.pane_id);
        }
        Self {
            pages: document.pages.iter().map(|page| page.id).collect(),
            panes,
            axes,
            traces,
            traces_by_pane,
            others,
            datasets,
            rows: &document.row_index,
        }
    }
}

impl Entities for Lookup<'_> {
    fn pane(&self, id: PaneId) -> Result<&Pane, VisualizationError> {
        self.panes
            .get(&id)
            .copied()
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(id)))
    }

    fn trace(&self, id: TraceId) -> Result<&Trace, VisualizationError> {
        self.traces
            .get(&id)
            .copied()
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Trace(id)))
    }

    fn require_page(&self, id: PageId) -> Result<(), VisualizationError> {
        self.pages
            .contains(&id)
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Page(id)))
    }

    fn require_axis_in_pane(&self, id: AxisId, pane_id: PaneId) -> Result<(), VisualizationError> {
        (self.axes.get(&id) == Some(&pane_id))
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Axis(id)))
    }

    fn require_entity(&self, entity: EntityRef) -> Result<(), VisualizationError> {
        let exists = match entity {
            EntityRef::Page(id) => self.pages.contains(&id),
            EntityRef::Pane(id) => self.panes.contains_key(&id),
            EntityRef::Trace(id) => self.traces.contains_key(&id),
            EntityRef::Axis(id) => self.axes.contains_key(&id),
            EntityRef::Cursor(_)
            | EntityRef::Marker(_)
            | EntityRef::Measurement(_)
            | EntityRef::Annotation(_)
            | EntityRef::LinkGroup(_) => self.others.contains(&entity),
        };
        exists
            .then_some(())
            .ok_or(VisualizationError::EntityNotFound(entity))
    }

    fn dataset(&self, binding: DatasetBinding) -> Result<&SourceDataset, VisualizationError> {
        let dataset = self
            .datasets
            .get(&binding.dataset_id)
            .copied()
            .ok_or(VisualizationError::DatasetNotFound(binding.dataset_id))?;
        if dataset.binding.content_digest != binding.content_digest {
            return Err(VisualizationError::SourceDigestMismatch {
                dataset_id: binding.dataset_id,
                bound: dataset.binding.content_digest,
                requested: binding.content_digest,
            });
        }
        Ok(dataset)
    }

    fn traces_in_pane(&self, pane_id: PaneId) -> Vec<&Trace> {
        self.traces_by_pane
            .get(&pane_id)
            .cloned()
            .unwrap_or_default()
    }

    fn rows(&self) -> &RowIndex {
        self.rows
    }
}
