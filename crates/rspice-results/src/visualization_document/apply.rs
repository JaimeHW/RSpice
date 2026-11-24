//! Applying an edit, and what an application is allowed to leave behind.
//!
//! A transaction applies its edits in place, keeping what each changed, and
//! checks what they changed before it commits; a refused edit puts back
//! what the transaction changed, so it changes nothing — there is no
//! partially-applied state. Removal cascades through the entities that depend
//! on the removed one and records a tombstone for each, which is why a page
//! can never keep a pane, axis, or trace whose parent is gone.

use super::lookup::{Entities, Lookup, require_made_in_order};
use super::*;

/// What one run of removals has gathered so far.
#[derive(Default)]
struct Removal {
    /// The entities removed, in order.
    order: Vec<EntityRef>,
    gone: HashSet<EntityRef>,
    /// What draws on each axis and what reads each trace, read once when a
    /// removal first asks.
    axis_dependents: Option<HashMap<AxisId, Vec<EntityRef>>>,
    trace_dependents: Option<HashMap<TraceId, Vec<EntityRef>>>,
}

/// Whether `now` reads what `before` read: the same rows of the same
/// dataset, on the same pane and axes, under the same name. A change of
/// anything else (whether it shows, its colour, style or lane) leaves what
/// a check of it found as it was.
fn reads_the_same(before: &Trace, now: &Trace) -> bool {
    before.binding == now.binding && reads_alike(before, now)
}

/// [`reads_the_same`], whatever content of a dataset each reads.
fn reads_alike(before: &Trace, now: &Trace) -> bool {
    before.pane_id == now.pane_id
        && before.signal_key == now.signal_key
        && before.coordinate_key == now.coordinate_key
        && before.row_predicates == now.row_predicates
        && before.x_axis_id == now.x_axis_id
        && before.y_axis_id == now.y_axis_id
        && before.label == now.label
        && before.missing == now.missing
}

/// A family policy's dimensions, each a coordinate of the dataset
/// `binding` names, of the type the policy declares.
pub(super) fn validate_family_source(
    entities: &dyn Entities,
    binding: DatasetBinding,
    policy: &FamilyPresentationPolicy,
) -> Result<(), VisualizationError> {
    let dataset = entities.dataset(binding)?;
    for dimension in
        std::iter::once(&policy.x_dimension.dimension).chain(policy.family_dimensions.iter())
    {
        let column = dataset
            .columns
            .iter()
            .find(|column| column.key == dimension.key)
            .ok_or_else(|| VisualizationError::InvalidValue {
                field: "pane.family-policy.dimension",
                message: format!(
                    "source dataset {} has no declared dimension '{}'",
                    binding.dataset_id, dimension.key
                ),
            })?;
        if column.role != ColumnRole::Coordinate || column.value_type != dimension.value_type {
            return Err(VisualizationError::InvalidValue {
                field: "pane.family-policy.dimension",
                message: format!(
                    "source dimension '{}' must be a {:?} coordinate, received {:?} {:?}",
                    dimension.key, dimension.value_type, column.value_type, column.role
                ),
            });
        }
    }
    Ok(())
}

/// What moving a document's consumers from one run onto another asks of
/// the run moved onto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum RebindMode {
    /// Every consumer moves, and a signal the next run lacks refuses.
    Exact,
    /// Every consumer moves; a trace whose signal the next run lacks stays,
    /// marked missing.
    KeepMissing,
    /// Consumers of an analysis the next run solves move, keeping missing
    /// traces; the rest stay on their run.
    Follow,
}

impl VisualizationDocument {
    fn validate_page_definition(page: &NewPage) -> Result<(), VisualizationError> {
        validate_label("page.title", &page.title)?;
        page.layout.validate()?;
        validate_key("page.template-id", &page.template_id)
    }

    pub(super) fn validate_pane_source(
        &self,
        entities: &dyn Entities,
        kind: PaneKind,
        viewer_id: &str,
        binding: Option<PaneDataBinding>,
    ) -> Result<(), VisualizationError> {
        validate_key("pane.viewer-id", viewer_id)?;
        let viewer =
            viewer_document(viewer_id).ok_or_else(|| VisualizationError::InvalidValue {
                field: "pane.viewer-id",
                message: format!("unknown viewer document '{viewer_id}'"),
            })?;
        let expected_kind = pane_kind_for_viewer_art(viewer.art);
        if kind != expected_kind {
            return Err(VisualizationError::InvalidValue {
                field: "pane.kind",
                message: format!(
                    "viewer '{viewer_id}' requires pane kind {expected_kind:?}, received {kind:?}"
                ),
            });
        }
        if let Some(binding) = binding {
            entities.dataset(binding.dataset)?;
        }
        Ok(())
    }

    fn validate_pane_family_policy(
        &self,
        entities: &dyn Entities,
        pane_id: PaneId,
        policy: &FamilyPresentationPolicy,
    ) -> Result<(), VisualizationError> {
        policy.validate()?;
        let pane = entities.pane(pane_id)?;
        let mut bindings = Vec::<DatasetBinding>::new();
        let mut held = HashSet::new();
        for binding in pane
            .binding
            .map(|binding| binding.dataset)
            .into_iter()
            .chain(
                entities
                    .traces_in_pane(pane_id)
                    .into_iter()
                    .map(|trace| trace.binding),
            )
        {
            if held.insert(binding) {
                bindings.push(binding);
            }
        }
        if bindings.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "pane.family-policy",
                message: "a family policy requires at least one immutable pane or trace source"
                    .to_owned(),
            });
        }
        for binding in bindings {
            validate_family_source(entities, binding, policy)?;
        }
        Ok(())
    }

    fn create_page(&mut self, page: NewPage) -> Result<PageId, VisualizationError> {
        Self::validate_page_definition(&page)?;
        let id = PageId::allocate(self.allocate_serial()?)?;
        self.pages.push(Page {
            id,
            title: page.title,
            layout: page.layout,
            template_id: page.template_id,
            update_policy: page.update_policy,
        });
        Ok(id)
    }

    fn placement_order(
        &self,
        page_id: PageId,
        placement: PanePlacement,
        excluded: Option<PaneId>,
    ) -> Result<u32, VisualizationError> {
        self.require_page(page_id)?;
        let pane_count = self
            .panes
            .iter()
            .filter(|pane| pane.page_id == page_id && Some(pane.id) != excluded)
            .count();
        match placement {
            PanePlacement::Primary if pane_count == 0 => Ok(0),
            PanePlacement::Primary => Err(VisualizationError::InvalidValue {
                field: "pane.placement",
                message: "primary placement requires an empty page".to_owned(),
            }),
            PanePlacement::Below { anchor_pane_id } | PanePlacement::RightOf { anchor_pane_id } => {
                if Some(anchor_pane_id) == excluded {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.placement",
                        message: "a pane cannot be placed relative to itself".to_owned(),
                    });
                }
                let anchor = self
                    .panes
                    .iter()
                    .find(|pane| pane.id == anchor_pane_id && Some(pane.id) != excluded)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(
                        anchor_pane_id,
                    )))?;
                if anchor.page_id != page_id {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.placement",
                        message: "placement anchor must belong to the destination page".to_owned(),
                    });
                }
                anchor
                    .order
                    .checked_add(1)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)
            }
        }
    }

    pub(super) fn create_pane(&mut self, pane: NewPane) -> Result<PaneId, VisualizationError> {
        validate_label("pane.title", &pane.title)?;
        self.validate_pane_source(&*self, pane.kind, &pane.viewer_id, pane.binding)?;
        let order = self.placement_order(pane.page_id, pane.placement, None)?;
        for existing in self
            .panes
            .iter_mut_where(|existing| existing.page_id == pane.page_id && existing.order >= order)
        {
            existing.order = existing
                .order
                .checked_add(1)
                .ok_or(VisualizationError::IdentitySpaceExhausted)?;
        }
        let id = PaneId::allocate(self.allocate_serial()?)?;
        self.panes.push(Pane {
            id,
            page_id: pane.page_id,
            title: pane.title,
            kind: pane.kind,
            viewer_id: pane.viewer_id,
            binding: pane.binding,
            placement: pane.placement,
            order,
            family_policy: None,
            hidden: false,
            strips: Vec::new(),
            maximized: false,
            shown: false,
        });
        Ok(id)
    }

    /// Materialize the canonical axes and display traces for a newly bound
    /// pane. A bound pane without these entities can be shown by the live
    /// Results renderer, but cannot participate in deterministic export,
    /// marker/measurement authoring, trace visibility, or link groups.
    ///
    /// A pane bound to a table plots every signal the table holds. A run
    /// reference holds no list of its run's signals, so a pane bound to one
    /// plots the signals `plotted` names, which the host read from the run.
    pub(super) fn provision_bound_pane(
        &mut self,
        pane_id: PaneId,
        viewer_id: &str,
        binding: PaneDataBinding,
        plotted: &[String],
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        let (x_axis_id, y_axis_id) = self.provision_axes(pane_id, viewer_id, created)?;
        if self.traces.iter().any(|trace| trace.pane_id == pane_id) {
            return Ok(());
        }
        if self.dataset_for_binding(binding.dataset)?.run.is_some() {
            return self.set_signals_plotted(pane_id, plotted, true, created);
        }
        self.provision_display_traces(pane_id, binding, (x_axis_id, y_axis_id), created)
    }

    /// The pane's horizontal and first vertical axis, made as its viewer
    /// labels and scales them when the pane has none.
    pub(super) fn provision_axes(
        &mut self,
        pane_id: PaneId,
        viewer_id: &str,
        created: &mut Vec<EntityRef>,
    ) -> Result<(AxisId, AxisId), VisualizationError> {
        self.require_pane(pane_id)?;
        let viewer =
            viewer_document(viewer_id).ok_or_else(|| VisualizationError::InvalidValue {
                field: "pane.viewer-id",
                message: format!("unknown viewer document '{viewer_id}'"),
            })?;
        let x_label = viewer.x_axis.to_owned();
        let y_label = viewer.y_axis.to_owned();
        let x_scale = if viewer.x_axis.to_ascii_lowercase().contains("frequency") {
            AxisScale::Logarithmic
        } else {
            AxisScale::Linear
        };
        let y_scale = if viewer.y_axis.to_ascii_lowercase().contains("db") {
            AxisScale::Decibels
        } else {
            AxisScale::Linear
        };

        let x_axis_id = if let Some(id) = self
            .axes
            .iter()
            .find(|axis| axis.pane_id == pane_id && axis.orientation == AxisOrientation::Horizontal)
            .map(|axis| axis.id)
        {
            id
        } else {
            let id = AxisId::allocate(self.allocate_serial()?)?;
            self.axes.push(Axis {
                id,
                pane_id,
                label: x_label,
                orientation: AxisOrientation::Horizontal,
                scale: x_scale,
                unit: None,
                range: None,
            });
            created.push(EntityRef::Axis(id));
            id
        };
        let y_axis_id = if let Some(id) = self
            .axes
            .iter()
            .find(|axis| {
                axis.pane_id == pane_id && axis.orientation == AxisOrientation::VerticalLeft
            })
            .map(|axis| axis.id)
        {
            id
        } else {
            let id = AxisId::allocate(self.allocate_serial()?)?;
            self.axes.push(Axis {
                id,
                pane_id,
                label: y_label,
                orientation: AxisOrientation::VerticalLeft,
                scale: y_scale,
                unit: None,
                range: None,
            });
            created.push(EntityRef::Axis(id));
            id
        };
        Ok((x_axis_id, y_axis_id))
    }

    /// A trace for every display signal of the analysis a new pane draws.
    fn provision_display_traces(
        &mut self,
        pane_id: PaneId,
        binding: PaneDataBinding,
        (x_axis_id, y_axis_id): (AxisId, AxisId),
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        let (coordinate_key, display_signals) = {
            let dataset = self.dataset_for_binding(binding.dataset)?;
            if let Some((signals, distinguishes_analyses)) =
                long_form_display_signals(dataset, binding.analysis_id)
            {
                let analysis = distinguishes_analyses.then(|| binding.analysis_id.to_string());
                let signals = signals
                    .into_iter()
                    .map(|(index, name)| {
                        let predicates =
                            display_signal_predicates(index, &name, analysis.as_deref());
                        ("y".to_owned(), name, predicates)
                    })
                    .collect::<Vec<_>>();
                ("x".to_owned(), signals)
            } else {
                let mut coordinates = dataset
                    .columns
                    .iter()
                    .filter(|column| column.role == ColumnRole::Coordinate);
                let Some(coordinate) = coordinates.next() else {
                    return Ok(());
                };
                if coordinates.next().is_some()
                    || !matches!(coordinate.value_type, ValueType::Real | ValueType::Integer)
                {
                    // Multi-dimensional family and table sources use their own
                    // typed renderers. They still receive canonical axes, but do
                    // not invent a one-dimensional Trace projection.
                    return Ok(());
                }
                let signals = dataset
                    .columns
                    .iter()
                    .filter(|column| {
                        column.role == ColumnRole::Signal
                            && !column.key.ends_with(":real")
                            && !column.key.ends_with(":imag")
                    })
                    .map(|column| (column.key.clone(), column.label.clone(), Vec::new()))
                    .collect::<Vec<_>>();
                (coordinate.key.clone(), signals)
            }
        };
        if display_signals.is_empty() {
            return Ok(());
        }
        for (signal_key, label, row_predicates) in display_signals {
            self.add_trace(
                NewTrace {
                    pane_id,
                    binding: binding.dataset,
                    signal_key,
                    coordinate_key: coordinate_key.clone(),
                    row_predicates,
                    x_axis_id,
                    y_axis_id,
                    label,
                },
                created,
            )?;
        }
        Ok(())
    }

    fn inherit_axis_link_groups(
        &mut self,
        pane_id: PaneId,
        placement: PanePlacement,
    ) -> Result<(), VisualizationError> {
        let anchor_pane_id = match placement {
            PanePlacement::Below { anchor_pane_id } | PanePlacement::RightOf { anchor_pane_id } => {
                anchor_pane_id
            }
            PanePlacement::Primary => return Ok(()),
        };
        let axis_pairs = self
            .axes
            .iter()
            .filter(|axis| axis.pane_id == anchor_pane_id)
            .filter_map(|anchor| {
                self.axes
                    .iter()
                    .find(|candidate| {
                        candidate.pane_id == pane_id && candidate.orientation == anchor.orientation
                    })
                    .map(|candidate| (anchor.id, candidate.id, anchor.orientation))
            })
            .collect::<Vec<_>>();
        for (anchor_axis, new_axis, orientation) in axis_pairs {
            for group in self.link_groups.iter_mut_where(|group| {
                let compatible = matches!(
                    (group.kind, orientation),
                    (LinkKind::HorizontalViewport, AxisOrientation::Horizontal)
                        | (
                            LinkKind::VerticalViewport,
                            AxisOrientation::VerticalLeft | AxisOrientation::VerticalRight
                        )
                );
                compatible
                    && group.members.contains(&EntityRef::Axis(anchor_axis))
                    && !group.members.contains(&EntityRef::Axis(new_axis))
            }) {
                group.members.push(EntityRef::Axis(new_axis));
            }
        }
        Ok(())
    }

    fn default_append_placement(&self, page_id: PageId) -> PanePlacement {
        self.panes
            .iter()
            .filter(|pane| pane.page_id == page_id)
            .max_by_key(|pane| pane.order)
            .map_or(PanePlacement::Primary, |pane| PanePlacement::Below {
                anchor_pane_id: pane.id,
            })
    }

    fn normalize_page_pane_layout(&mut self, page_id: PageId, excluded: Option<PaneId>) {
        let mut ordered: Vec<_> = self
            .panes
            .iter()
            .filter(|pane| pane.page_id == page_id && Some(pane.id) != excluded)
            .map(|pane| (pane.id, pane.order, pane.placement))
            .collect();
        ordered.sort_by_key(|(id, order, _)| (*order, *id));
        let mut preceding = HashSet::new();
        let mut previous = None;
        for (order, (pane_id, _, placement)) in ordered.into_iter().enumerate() {
            let normalized_placement = if order == 0 {
                PanePlacement::Primary
            } else {
                match placement {
                    PanePlacement::Below { anchor_pane_id }
                        if preceding.contains(&anchor_pane_id) =>
                    {
                        placement
                    }
                    PanePlacement::RightOf { anchor_pane_id }
                        if preceding.contains(&anchor_pane_id) =>
                    {
                        placement
                    }
                    _ => PanePlacement::Below {
                        anchor_pane_id: previous.expect("a non-primary pane has a predecessor"),
                    },
                }
            };
            let pane = self
                .panes
                .get_mut(pane_id.get())
                .expect("pane identity was projected from this document");
            pane.order = u32::try_from(order).expect("pane count exceeds u32 address space");
            pane.placement = normalized_placement;
            preceding.insert(pane_id);
            previous = Some(pane_id);
        }
    }

    fn place_pane(
        &mut self,
        pane_id: PaneId,
        page_id: PageId,
        placement: PanePlacement,
    ) -> Result<(), VisualizationError> {
        let source_page_id = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| pane.page_id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
        self.require_page(page_id)?;
        self.normalize_page_pane_layout(source_page_id, Some(pane_id));
        let order = self.placement_order(page_id, placement, Some(pane_id))?;
        for pane in self.panes.iter_mut_where(|pane| {
            pane.id != pane_id && pane.page_id == page_id && pane.order >= order
        }) {
            pane.order = pane
                .order
                .checked_add(1)
                .ok_or(VisualizationError::IdentitySpaceExhausted)?;
        }
        let pane = self.pane_mut(pane_id)?;
        pane.page_id = page_id;
        pane.placement = placement;
        pane.order = order;
        Ok(())
    }

    /// Keep one authored measurement: a single evaluation, or the
    /// distribution of one evaluated on every member of a family. Its
    /// expression names the signals it reads; it references no trace.
    fn add_scalar_measurement(
        &mut self,
        pane_id: PaneId,
        expression: String,
        value: f64,
        family: Option<FamilyMeasurementSummary>,
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        if expression.trim().is_empty()
            || expression.len() > MAX_SOURCE_TEXT_BYTES
            || expression.chars().any(char::is_control)
            || !value.is_finite()
        {
            return Err(VisualizationError::InvalidValue {
                field: "measurement.expression",
                message: format!(
                    "a scalar measurement requires a finite value and 1 to {MAX_SOURCE_TEXT_BYTES} non-control UTF-8 expression bytes"
                ),
            });
        }
        self.require_pane(pane_id)?;
        if let Some(family) = &family {
            family.validate(Some(value))?;
        }
        let id = MeasurementId::allocate(self.allocate_serial()?)?;
        self.measurements.push(Measurement {
            id,
            pane_id,
            trace_ids: Vec::new(),
            kind: MeasurementKind::Point,
            label: format!("Measurement {}", id.get()),
            expression: Some(expression),
            value: Some(value),
            family,
        });
        created.push(EntityRef::Measurement(id));
        Ok(())
    }

    /// Apply `edits` in order; a run of removals is taken out together.
    pub(super) fn apply_edits(
        &mut self,
        edits: Vec<DocumentEdit>,
        revision: ObjectRevision,
        created: &mut Vec<EntityRef>,
        tombstoned: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        let mut removals = Vec::new();
        for edit in edits {
            if let DocumentEdit::Remove(entity) = edit {
                removals.push(entity);
                continue;
            }
            if !removals.is_empty() {
                self.remove_all(std::mem::take(&mut removals), revision, tombstoned)?;
            }
            self.apply_edit(edit, revision, created, tombstoned)?;
        }
        if !removals.is_empty() {
            self.remove_all(removals, revision, tombstoned)?;
        }
        Ok(())
    }

    pub(super) fn apply_edit(
        &mut self,
        edit: DocumentEdit,
        revision: ObjectRevision,
        created: &mut Vec<EntityRef>,
        tombstoned: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        match edit {
            DocumentEdit::SetTracking(tracking) => {
                tracking.validate()?;
                self.tracking = tracking;
                Ok(())
            }
            DocumentEdit::SetPresentation(presentation) => {
                presentation.validate()?;
                self.presentation = presentation;
                Ok(())
            }
            DocumentEdit::AttachDataset(dataset) => self.attach_dataset(dataset),
            DocumentEdit::MergeDatasetProjection(dataset) => self.merge_dataset_projection(dataset),
            DocumentEdit::ArchiveDataset { binding, archive } => {
                self.dataset_for_binding(binding)?;
                self.datasets
                    .edit()
                    .iter_mut()
                    .find(|dataset| dataset.binding == binding)
                    .and_then(|dataset| dataset.run.as_mut())
                    .ok_or_else(|| VisualizationError::InvalidValue {
                        field: "run-source.archive",
                        message: "only a dataset that references its run is archived".to_owned(),
                    })?
                    .set_archive(archive)
            }
            DocumentEdit::RepointDataset {
                previous,
                next,
                analyses,
            } => self.repoint_dataset(previous, next, &analyses),
            DocumentEdit::RetargetTrackedDataset {
                previous,
                next,
                analysis_id,
            } => self.retarget_tracked_dataset(previous, next, analysis_id),
            DocumentEdit::AddPage { title } => {
                let id = self.create_page(NewPage {
                    title,
                    layout: PageLayout::default(),
                    template_id: default_page_template_id(),
                    update_policy: PageUpdatePolicy::default(),
                })?;
                created.push(EntityRef::Page(id));
                Ok(())
            }
            DocumentEdit::AddComposedPage(page) => {
                let id = self.create_page(page)?;
                created.push(EntityRef::Page(id));
                Ok(())
            }
            DocumentEdit::AddPane {
                page_id,
                title,
                kind,
            } => {
                let placement = self.default_append_placement(page_id);
                let id = self.create_pane(NewPane {
                    page_id,
                    title,
                    kind,
                    viewer_id: default_viewer_id_for_kind(kind).to_owned(),
                    binding: None,
                    placement,
                    plotted: Vec::new(),
                })?;
                created.push(EntityRef::Pane(id));
                Ok(())
            }
            DocumentEdit::FollowRun { next } => self.follow_run(next),
            DocumentEdit::AddViewPane(pane) => self.add_view_pane(pane, created).map(|_| ()),
            DocumentEdit::SetSignalsPlotted {
                pane_id,
                signals,
                plotted,
            } => self.set_signals_plotted(pane_id, &signals, plotted, created),
            DocumentEdit::PlotPrepared(prepared) => self.plot_prepared(prepared, created),
            DocumentEdit::SetPaneHidden { pane_id, hidden } => {
                self.pane_mut(pane_id)?.hidden = hidden;
                Ok(())
            }
            DocumentEdit::SetPaneStrips { pane_id, strips } => {
                super::strips::validate_strips(&strips)?;
                self.pane_mut(pane_id)?.strips = strips;
                Ok(())
            }
            DocumentEdit::SetPaneMaximized { pane_id, maximized } => {
                self.pane_mut(pane_id)?.maximized = maximized;
                Ok(())
            }
            DocumentEdit::ShowPaneAnalysis { pane_id } => {
                self.pane_mut(pane_id)?;
                for pane in self
                    .panes
                    .iter_mut_where(|pane| pane.shown != (pane.id == pane_id))
                {
                    pane.shown = pane.id == pane_id;
                }
                Ok(())
            }
            DocumentEdit::AddBoundPane(mut pane) => {
                let viewer_id = pane.viewer_id.clone();
                let binding = pane.binding;
                let placement = pane.placement;
                let plotted = std::mem::take(&mut pane.plotted);
                let id = self.create_pane(pane)?;
                created.push(EntityRef::Pane(id));
                if let Some(binding) = binding {
                    self.provision_bound_pane(id, &viewer_id, binding, &plotted, created)?;
                }
                self.inherit_axis_link_groups(id, placement)?;
                Ok(())
            }
            DocumentEdit::AddPaneOnNewPage { page, pane } => {
                let page_id = self.create_page(page)?;
                created.push(EntityRef::Page(page_id));
                let viewer_id = pane.viewer_id.clone();
                let binding = pane.binding;
                let pane_id = self.create_pane(NewPane {
                    page_id,
                    title: pane.title,
                    kind: pane.kind,
                    viewer_id: pane.viewer_id,
                    binding: pane.binding,
                    placement: PanePlacement::Primary,
                    plotted: Vec::new(),
                })?;
                created.push(EntityRef::Pane(pane_id));
                if let Some(binding) = binding {
                    self.provision_bound_pane(
                        pane_id,
                        &viewer_id,
                        binding,
                        &pane.plotted,
                        created,
                    )?;
                }
                Ok(())
            }
            DocumentEdit::SetPageComposition {
                page_id,
                layout,
                template_id,
                update_policy,
            } => {
                Self::validate_page_definition(&NewPage {
                    title: self.page_mut(page_id)?.title.clone(),
                    layout,
                    template_id: template_id.clone(),
                    update_policy,
                })?;
                let page = self.page_mut(page_id)?;
                page.layout = layout;
                page.template_id = template_id;
                page.update_policy = update_policy;
                Ok(())
            }
            DocumentEdit::SetPaneSource {
                pane_id,
                viewer_id,
                binding,
            } => {
                let kind = self
                    .panes
                    .iter()
                    .find(|pane| pane.id == pane_id)
                    .map(|pane| pane.kind)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
                self.validate_pane_source(&*self, kind, &viewer_id, binding)?;
                let pane = self.pane_mut(pane_id)?;
                pane.viewer_id = viewer_id.clone();
                pane.binding = binding;
                if let Some(binding) = binding {
                    self.provision_bound_pane(pane_id, &viewer_id, binding, &[], created)?;
                }
                Ok(())
            }
            DocumentEdit::SetPaneFamilyPresentation { pane_id, policy } => {
                if let Some(policy) = &policy {
                    self.validate_pane_family_policy(&*self, pane_id, policy)?;
                } else {
                    self.require_pane(pane_id)?;
                }
                self.pane_mut(pane_id)?.family_policy = policy;
                Ok(())
            }
            DocumentEdit::PlacePane {
                pane_id,
                page_id,
                placement,
            } => self.place_pane(pane_id, page_id, placement),
            DocumentEdit::AssignPaneToReportPage {
                pane_id,
                page_title,
                template_id,
                update_policy,
            } => {
                validate_label("page.title", &page_title)?;
                validate_key("page.template-id", &template_id)?;
                let source_page_id = self
                    .panes
                    .iter()
                    .find(|pane| pane.id == pane_id)
                    .map(|pane| pane.page_id)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
                let target_page_id = if let Some(page_id) = self
                    .pages
                    .iter()
                    .find(|page| page.title == page_title)
                    .map(|page| page.id)
                {
                    page_id
                } else if self
                    .panes
                    .iter()
                    .filter(|pane| pane.page_id == source_page_id)
                    .count()
                    == 1
                {
                    self.page_mut(source_page_id)?.title = page_title;
                    source_page_id
                } else {
                    let page_id = self.create_page(NewPage {
                        title: page_title,
                        layout: PageLayout::Rows,
                        template_id: template_id.clone(),
                        update_policy,
                    })?;
                    created.push(EntityRef::Page(page_id));
                    page_id
                };
                if target_page_id != source_page_id {
                    let placement = self.default_append_placement(target_page_id);
                    self.place_pane(pane_id, target_page_id, placement)?;
                }
                let target = self.page_mut(target_page_id)?;
                target.template_id = template_id;
                target.update_policy = update_policy;
                if target_page_id != source_page_id
                    && !self.panes.iter().any(|pane| pane.page_id == source_page_id)
                {
                    self.remove_all(vec![EntityRef::Page(source_page_id)], revision, tombstoned)?;
                }
                Ok(())
            }
            DocumentEdit::ReorderPagePanes { page_id, pane_ids } => {
                self.require_page(page_id)?;
                let current = self
                    .panes
                    .iter()
                    .filter(|pane| pane.page_id == page_id)
                    .map(|pane| pane.id)
                    .collect::<HashSet<_>>();
                let requested = pane_ids.iter().copied().collect::<HashSet<_>>();
                if pane_ids.is_empty() || requested.len() != pane_ids.len() || requested != current
                {
                    return Err(VisualizationError::InvalidValue {
                        field: "page.pane-order",
                        message: "pane order must contain every pane on the page exactly once"
                            .to_owned(),
                    });
                }
                let mut preceding = HashSet::new();
                let mut previous = None;
                for (order, pane_id) in pane_ids.into_iter().enumerate() {
                    let pane = self.pane_mut(pane_id)?;
                    let placement = if order == 0 {
                        PanePlacement::Primary
                    } else {
                        match pane.placement {
                            PanePlacement::Below { anchor_pane_id }
                                if preceding.contains(&anchor_pane_id) =>
                            {
                                pane.placement
                            }
                            PanePlacement::RightOf { anchor_pane_id }
                                if preceding.contains(&anchor_pane_id) =>
                            {
                                pane.placement
                            }
                            _ => PanePlacement::Below {
                                anchor_pane_id: previous
                                    .expect("a non-primary pane has a predecessor"),
                            },
                        }
                    };
                    pane.order = u32::try_from(order)
                        .map_err(|_| VisualizationError::IdentitySpaceExhausted)?;
                    pane.placement = placement;
                    preceding.insert(pane_id);
                    previous = Some(pane_id);
                }
                Ok(())
            }
            DocumentEdit::AddAxis(axis) => {
                self.require_pane(axis.pane_id)?;
                validate_label("axis.label", &axis.label)?;
                validate_optional_unit("axis.unit", axis.unit.as_deref())?;
                let id = AxisId::allocate(self.allocate_serial()?)?;
                self.axes.push(Axis {
                    id,
                    pane_id: axis.pane_id,
                    label: axis.label,
                    orientation: axis.orientation,
                    scale: axis.scale,
                    unit: axis.unit,
                    range: axis.range,
                });
                created.push(EntityRef::Axis(id));
                Ok(())
            }
            DocumentEdit::AddTrace(trace) => self.add_trace(trace, created),
            DocumentEdit::AddCursor {
                pane_id,
                axis_id,
                position,
                label,
            } => {
                validate_label("cursor.label", &label)?;
                position.validate("cursor.position")?;
                self.require_axis_in_pane(axis_id, pane_id)?;
                let id = CursorId::allocate(self.allocate_serial()?)?;
                self.cursors.push(Cursor {
                    id,
                    pane_id,
                    axis_id,
                    position,
                    label,
                });
                created.push(EntityRef::Cursor(id));
                Ok(())
            }
            DocumentEdit::AddMarker {
                pane_id,
                trace_id,
                coordinate,
                label,
            } => {
                validate_label("marker.label", &label)?;
                coordinate.validate("marker.coordinate")?;
                self.require_trace_in_pane(trace_id, pane_id)?;
                if !trace_contains_exact_coordinate(&*self, trace_id, &coordinate)? {
                    return Err(VisualizationError::InterpolationRequired);
                }
                let id = MarkerId::allocate(self.allocate_serial()?)?;
                self.markers.push(Marker {
                    id,
                    pane_id,
                    trace_id,
                    coordinate,
                    label,
                    kind: PlotMarkerKind::PointNote,
                    scope: PlotMarkerScope::Document,
                    source_specification: None,
                });
                created.push(EntityRef::Marker(id));
                Ok(())
            }
            DocumentEdit::AddTypedMarker {
                pane_id,
                trace_id,
                coordinate,
                label,
                kind,
                scope,
                source_specification,
            } => {
                validate_label("marker.label", &label)?;
                if let Some(source) = source_specification.as_deref() {
                    validate_label("marker.source-specification", source)?;
                }
                coordinate.validate("marker.coordinate")?;
                self.require_trace_in_pane(trace_id, pane_id)?;
                if !trace_contains_exact_coordinate(&*self, trace_id, &coordinate)? {
                    return Err(VisualizationError::InterpolationRequired);
                }
                let id = MarkerId::allocate(self.allocate_serial()?)?;
                self.markers.push(Marker {
                    id,
                    pane_id,
                    trace_id,
                    coordinate,
                    label,
                    kind,
                    scope,
                    source_specification,
                });
                created.push(EntityRef::Marker(id));
                Ok(())
            }
            DocumentEdit::AddMeasurement {
                pane_id,
                trace_ids,
                kind,
                label,
            } => {
                validate_label("measurement.label", &label)?;
                if trace_ids.is_empty() {
                    return Err(VisualizationError::InvalidValue {
                        field: "measurement.traces",
                        message: "a measurement reads at least one trace".to_owned(),
                    });
                }
                for trace_id in &trace_ids {
                    self.require_trace_in_pane(*trace_id, pane_id)?;
                }
                let id = MeasurementId::allocate(self.allocate_serial()?)?;
                self.measurements.push(Measurement {
                    id,
                    pane_id,
                    trace_ids,
                    kind,
                    label,
                    expression: None,
                    value: None,
                    family: None,
                });
                created.push(EntityRef::Measurement(id));
                Ok(())
            }
            DocumentEdit::AddAnnotation {
                pane_id,
                anchor,
                text,
            } => {
                self.require_pane(pane_id)?;
                validate_annotation(&*self, pane_id, &anchor, &text)?;
                let id = AnnotationId::allocate(self.allocate_serial()?)?;
                self.annotations.push(Annotation {
                    id,
                    pane_id,
                    anchor,
                    text,
                });
                created.push(EntityRef::Annotation(id));
                Ok(())
            }
            DocumentEdit::AddLinkGroup {
                label,
                kind,
                members,
            } => {
                validate_label("link-group.label", &label)?;
                validate_link_members(&*self, kind, &members)?;
                let id = LinkGroupId::allocate(self.allocate_serial()?)?;
                self.link_groups.push(LinkGroup {
                    id,
                    label,
                    kind,
                    members,
                });
                created.push(EntityRef::LinkGroup(id));
                Ok(())
            }
            DocumentEdit::Rename { entity, value } => self.rename(entity, value),
            DocumentEdit::SetAxisRange { axis_id, range } => {
                self.axis_mut(axis_id)?.range = range;
                Ok(())
            }
            DocumentEdit::SetAxisScale { axis_id, scale } => {
                let axis = self.axis_mut(axis_id)?;
                if let Some(range) = axis.range
                    && scale == AxisScale::Logarithmic
                    && range.minimum <= 0.0
                {
                    // A range read on the linear axis is no logarithmic one.
                    axis.range = None;
                }
                axis.scale = scale;
                Ok(())
            }
            DocumentEdit::SetTraceVisibility { trace_id, visible } => {
                self.update_trace(trace_id, |trace| trace.visible = visible)
            }
            DocumentEdit::SetTraceColor { trace_id, color } => {
                self.update_trace(trace_id, |trace| trace.color = color)
            }
            DocumentEdit::SetTraceStyle { trace_id, style } => {
                self.update_trace(trace_id, |trace| trace.style = style)
            }
            DocumentEdit::SetTraceLane { trace_id, lane } => {
                self.update_trace(trace_id, |trace| trace.lane = lane)
            }
            DocumentEdit::DefineUserBus(bus) => {
                self.validate_user_bus(&*self, &bus)?;
                match self
                    .user_buses
                    .iter()
                    .position(|held| held.pane_id == bus.pane_id && held.name == bus.name)
                {
                    Some(index) => self.user_buses.splice(index..index + 1, vec![bus]),
                    None => self.user_buses.push(bus),
                }
                Ok(())
            }
            DocumentEdit::RemoveUserBus { pane_id, name } => {
                let before = self.user_buses.len();
                self.user_buses
                    .retain(|bus| bus.pane_id != pane_id || bus.name != name);
                if self.user_buses.len() == before {
                    return Err(VisualizationError::InvalidValue {
                        field: "user-bus.name",
                        message: format!("pane {} groups no bus named {name:?}", pane_id.get()),
                    });
                }
                let ungrouped = EventLaneRef::Bus { name };
                self.retain_event_lanes(|order, lane| {
                    order.pane_id != pane_id || *lane != ungrouped
                });
                Ok(())
            }
            DocumentEdit::AddExpressionTrace(trace) => {
                self.validate_expression_trace(&*self, &trace)?;
                match self
                    .expression_traces
                    .iter()
                    .position(|held| held.pane_id == trace.pane_id && held.text == trace.text)
                {
                    Some(index) => self.expression_traces.splice(index..index + 1, vec![trace]),
                    None => self.expression_traces.push(trace),
                }
                Ok(())
            }
            DocumentEdit::RemoveExpressionTrace { pane_id, text } => {
                let before = self.expression_traces.len();
                self.expression_traces
                    .retain(|trace| trace.pane_id != pane_id || trace.text != text);
                if self.expression_traces.len() == before {
                    return Err(VisualizationError::InvalidValue {
                        field: "expression-trace.text",
                        message: format!("pane {} plots no expression {text:?}", pane_id.get()),
                    });
                }
                Ok(())
            }
            DocumentEdit::SetXyPlots { pane_id, plots } => {
                self.require_pane(pane_id)?;
                for (index, plot) in plots.iter().enumerate() {
                    if plot.pane_id != pane_id {
                        return Err(VisualizationError::InvalidValue {
                            field: "xy-plot.pane-id",
                            message: format!(
                                "a plot of pane {} is listed among pane {}'s",
                                plot.pane_id.get(),
                                pane_id.get()
                            ),
                        });
                    }
                    self.validate_xy_plot(&*self, plot)?;
                    if plots[..index].iter().any(|held| held.same_plot(plot)) {
                        return Err(duplicate_xy_plot(plot));
                    }
                }
                self.xy_plots.retain(|held| held.pane_id != pane_id);
                self.xy_plots.edit().extend(plots);
                Ok(())
            }
            DocumentEdit::SetReadoutPin { pane_id, sheet, x } => {
                self.require_pane(pane_id)?;
                self.readout_pins
                    .retain(|pin| pin.pane_id != pane_id || pin.sheet != sheet);
                if let Some(x) = x {
                    let pin = ReadoutPin { pane_id, sheet, x };
                    validate_readout_pin(&pin)?;
                    self.readout_pins.push(pin);
                }
                Ok(())
            }
            DocumentEdit::SetSheetSetting {
                pane_id,
                sheet,
                setting,
            } => {
                self.require_pane(pane_id)?;
                self.sheet_settings
                    .retain(|held| held.pane_id != pane_id || held.setting.sheet() != sheet);
                if let Some(setting) = setting {
                    let setting = SheetSetting { pane_id, setting };
                    validate_sheet_setting(&setting)?;
                    if setting.setting.sheet() != sheet {
                        return Err(VisualizationError::InvalidValue {
                            field: "sheet-setting.sheet",
                            message: format!(
                                "settings for {:?} cannot be kept as {sheet:?}'s",
                                setting.setting.sheet()
                            ),
                        });
                    }
                    self.sheet_settings.push(setting);
                }
                Ok(())
            }
            DocumentEdit::SetSpectrumSettings { pane_id, settings } => {
                self.require_pane(pane_id)?;
                self.spectrum_settings
                    .retain(|held| held.pane_id != pane_id);
                if let Some(settings) = settings {
                    validate_spectrum_settings(&settings)?;
                    self.spectrum_settings
                        .push(PaneSpectrumSettings { pane_id, settings });
                }
                Ok(())
            }
            DocumentEdit::SetCoefficientSettings { pane_id, settings } => {
                self.require_pane(pane_id)?;
                self.coefficient_settings
                    .retain(|held| held.pane_id != pane_id);
                if let Some(settings) = settings {
                    validate_coefficient_settings(&settings)?;
                    self.coefficient_settings
                        .push(PaneCoefficientSettings { pane_id, settings });
                }
                Ok(())
            }
            DocumentEdit::SetEventLaneOrder(order) => {
                self.validate_event_lane_order(&*self, &order)?;
                self.event_lane_orders
                    .retain(|held| held.pane_id != order.pane_id);
                if !order.lanes.is_empty() {
                    self.event_lane_orders.push(order);
                }
                Ok(())
            }
            DocumentEdit::MoveCursor {
                cursor_id,
                position,
            } => {
                position.validate("cursor.position")?;
                self.cursor_mut(cursor_id)?.position = position;
                Ok(())
            }
            DocumentEdit::MoveMarker {
                marker_id,
                coordinate,
            } => {
                coordinate.validate("marker.coordinate")?;
                let trace_id = self.marker_mut(marker_id)?.trace_id;
                if !trace_contains_exact_coordinate(&*self, trace_id, &coordinate)? {
                    return Err(VisualizationError::InterpolationRequired);
                }
                self.marker_mut(marker_id)?.coordinate = coordinate;
                Ok(())
            }
            DocumentEdit::AddScalarMeasurement {
                pane_id,
                expression,
                value,
            } => self.add_scalar_measurement(pane_id, expression, value, None, created),
            DocumentEdit::AddFamilyMeasurement {
                pane_id,
                expression,
                value,
                family,
            } => self.add_scalar_measurement(pane_id, expression, value, Some(family), created),
            DocumentEdit::SetMarker {
                marker_id,
                coordinate,
                label,
                kind,
                scope,
                source_specification,
            } => {
                validate_label("marker.label", &label)?;
                if let Some(source) = source_specification.as_deref() {
                    validate_label("marker.source-specification", source)?;
                }
                coordinate.validate("marker.coordinate")?;
                let trace_id = self.marker_mut(marker_id)?.trace_id;
                if !trace_contains_exact_coordinate(&*self, trace_id, &coordinate)? {
                    return Err(VisualizationError::InterpolationRequired);
                }
                let marker = self.marker_mut(marker_id)?;
                marker.coordinate = coordinate;
                marker.label = label;
                marker.kind = kind;
                marker.scope = scope;
                marker.source_specification = source_specification;
                Ok(())
            }
            DocumentEdit::SetAnnotation {
                annotation_id,
                anchor,
                text,
            } => {
                let pane_id = self
                    .annotations
                    .iter()
                    .find(|annotation| annotation.id == annotation_id)
                    .map(|annotation| annotation.pane_id)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::Annotation(
                        annotation_id,
                    )))?;
                validate_annotation(&*self, pane_id, &anchor, &text)?;
                let annotation = self.annotation_mut(annotation_id)?;
                annotation.anchor = anchor;
                annotation.text = text;
                Ok(())
            }
            DocumentEdit::SetLinkMembers {
                link_group_id,
                members,
            } => {
                let kind = self
                    .link_groups
                    .iter()
                    .find(|group| group.id == link_group_id)
                    .map(|group| group.kind)
                    .ok_or(VisualizationError::EntityNotFound(EntityRef::LinkGroup(
                        link_group_id,
                    )))?;
                validate_link_members(&*self, kind, &members)?;
                self.link_group_mut(link_group_id)?.members = members;
                Ok(())
            }
            DocumentEdit::ClearMarkers { pane_id } => {
                if let Some(pane_id) = pane_id {
                    self.require_pane(pane_id)?;
                }
                let markers = self
                    .markers
                    .iter()
                    .filter(|marker| pane_id.is_none_or(|pane_id| marker.pane_id == pane_id))
                    .map(|marker| EntityRef::Marker(marker.id))
                    .collect::<Vec<_>>();
                self.remove_all(markers, revision, tombstoned)
            }
            DocumentEdit::Remove(entity) => self.remove_all(vec![entity], revision, tombstoned),
            DocumentEdit::RecordComparison(receipt) => {
                self.validate_comparison_receipt(&*self, &receipt)?;
                self.comparisons.push(receipt);
                Ok(())
            }
        }
    }

    pub(super) fn attach_dataset(
        &mut self,
        dataset: SourceDataset,
    ) -> Result<(), VisualizationError> {
        dataset.validate()?;
        if let Some(existing) = self
            .datasets
            .iter()
            .find(|existing| existing.binding.dataset_id == dataset.binding.dataset_id)
        {
            if existing.binding.content_digest != dataset.binding.content_digest {
                return Err(VisualizationError::SourceDigestMismatch {
                    dataset_id: dataset.binding.dataset_id,
                    bound: existing.binding.content_digest,
                    requested: dataset.binding.content_digest,
                });
            }
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: format!("dataset {} is already attached", dataset.binding.dataset_id),
            });
        }
        self.datasets.push(dataset);
        Ok(())
    }

    pub(super) fn merge_dataset_projection(
        &mut self,
        projection: SourceDataset,
    ) -> Result<(), VisualizationError> {
        projection.validate()?;
        let index = self
            .datasets
            .iter()
            .position(|dataset| dataset.binding.dataset_id == projection.binding.dataset_id)
            .ok_or(VisualizationError::DatasetNotFound(
                projection.binding.dataset_id,
            ))?;
        let existing = &self.datasets[index];
        if existing.binding.content_digest != projection.binding.content_digest {
            return Err(VisualizationError::SourceDigestMismatch {
                dataset_id: projection.binding.dataset_id,
                bound: existing.binding.content_digest,
                requested: projection.binding.content_digest,
            });
        }
        match (existing.run.is_some(), projection.run) {
            (true, Some(projected)) => {
                return self.datasets.edit()[index]
                    .run
                    .as_mut()
                    .expect("the dataset references its run")
                    .merge(projected);
            }
            (false, None) => {}
            _ => {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.datasets",
                    message: "a run reference and an embedded table cannot share one binding"
                        .to_owned(),
                });
            }
        }
        if existing.columns != projection.columns {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "analysis projections for one immutable dataset must use identical typed columns"
                    .to_owned(),
            });
        }
        let analysis_index = existing
            .columns
            .iter()
            .position(|column| column.key == "analysis-id")
            .ok_or_else(|| VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "mergeable analysis projections require an analysis-id coordinate"
                    .to_owned(),
            })?;
        if existing.columns[analysis_index].role != ColumnRole::Coordinate
            || existing.columns[analysis_index].value_type != ValueType::Text
        {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "analysis-id must be a text coordinate".to_owned(),
            });
        }
        let mut projection_analysis = None::<&str>;
        for row in &projection.rows {
            let Some(TypedValue::Text(analysis_id)) = row.values.get(analysis_index) else {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.datasets",
                    message: "every projected source row must retain its analysis identity"
                        .to_owned(),
                });
            };
            if projection_analysis.is_some_and(|existing| existing != analysis_id) {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.datasets",
                    message: "one merge edit may carry exactly one analysis projection".to_owned(),
                });
            }
            projection_analysis = Some(analysis_id);
        }
        let projection_analysis =
            projection_analysis.ok_or_else(|| VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "an analysis projection must retain at least one exact row".to_owned(),
            })?;
        let retained_rows = existing
            .rows
            .iter()
            .filter(|row| {
                matches!(
                    row.values.get(analysis_index),
                    Some(TypedValue::Text(analysis_id)) if analysis_id == projection_analysis
                )
            })
            .collect::<Vec<_>>();
        if !retained_rows.is_empty() {
            if retained_rows.len() != projection.rows.len()
                || retained_rows
                    .iter()
                    .zip(&projection.rows)
                    .any(|(retained, projected)| *retained != projected)
            {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.datasets",
                    message: "an attached analysis projection cannot be rewritten under the same immutable binding"
                        .to_owned(),
                });
            }
            return Ok(());
        }
        let mut merged = existing.clone();
        merged.rows.extend(projection.rows);
        merged.validate()?;
        self.datasets.splice(index..index + 1, vec![merged]);
        Ok(())
    }

    fn retarget_tracked_dataset(
        &mut self,
        previous: DatasetBinding,
        next: SourceDataset,
        analysis_id: AnalysisInstanceId,
    ) -> Result<(), VisualizationError> {
        if self.tracking.mode != ResultDocumentTrackingMode::Latest {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.tracking",
                message: "only a latest-bound document may advance to a newer dataset".to_owned(),
            });
        }
        self.dataset_for_binding(previous)?;
        let next_binding = next.binding();
        if next_binding == previous {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "the replacement dataset is identical to the current binding".to_owned(),
            });
        }
        self.attach_dataset(next)?;
        self.rebind_consumers(
            previous,
            next_binding,
            |_| Some(analysis_id),
            "newer tracked",
            RebindMode::KeepMissing,
        )?;
        // Nothing reads the run left behind once its panes and traces have
        // moved, so it goes as it does on a re-point: a document that follows
        // the latest run holds one run, not one per run it has ever followed.
        self.drop_unreferenced_dataset(previous);
        Ok(())
    }

    /// Drop `binding` unless a comparison receipt still names it.
    pub(super) fn drop_unreferenced_dataset(&mut self, binding: DatasetBinding) {
        if !self
            .comparisons
            .iter()
            .any(|receipt| receipt.baseline == binding || receipt.candidate == binding)
        {
            self.datasets.retain(|dataset| dataset.binding != binding);
        }
    }

    /// Read another retained run with the same signals in place of one this
    /// document references, keeping every pane, trace and presentation
    /// entity. The previous reference stays only while a comparison receipt
    /// still names it.
    fn repoint_dataset(
        &mut self,
        previous: DatasetBinding,
        next: SourceDataset,
        analyses: &[AnalysisRepoint],
    ) -> Result<(), VisualizationError> {
        self.dataset_for_binding(previous)?;
        let next_binding = next.binding();
        if next_binding.dataset_id == previous.dataset_id {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "a dataset is re-pointed at a different run".to_owned(),
            });
        }
        if next.run.is_none() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: "a dataset is re-pointed at a run reference".to_owned(),
            });
        }
        if self
            .datasets
            .iter()
            .any(|dataset| dataset.binding.dataset_id == next_binding.dataset_id)
        {
            self.merge_dataset_projection(next)?;
        } else {
            self.attach_dataset(next)?;
        }
        let mapped = |analysis: Option<AnalysisInstanceId>| {
            let analysis = analysis?;
            analyses
                .iter()
                .find(|mapping| mapping.previous == analysis)
                .map(|mapping| mapping.next)
        };
        self.rebind_consumers(previous, next_binding, mapped, "chosen", RebindMode::Exact)?;
        self.drop_unreferenced_dataset(previous);
        Ok(())
    }

    /// Move every pane and trace that reads `previous` onto `next`, each
    /// onto the analysis `analysis` maps its own to, and each trace onto the
    /// display signal of the same name there. With `keep_missing`, a trace
    /// whose signal `next` lacks stays, marked missing; without it, only a
    /// trace already marked missing may.
    pub(super) fn rebind_consumers(
        &mut self,
        previous: DatasetBinding,
        next: DatasetBinding,
        analysis: impl Fn(Option<AnalysisInstanceId>) -> Option<AnalysisInstanceId>,
        describe_next: &str,
        mode: RebindMode,
    ) -> Result<(), VisualizationError> {
        let keep_missing = mode != RebindMode::Exact;
        let follow = mode == RebindMode::Follow;
        let unmapped = |from: Option<AnalysisInstanceId>| {
            VisualizationError::InvalidValue {
            field: "visualization-document.datasets",
            message: from.map_or_else(
                || format!("a trace's analysis is unknown, so the {describe_next} run cannot stand in for it"),
                |from| format!("the {describe_next} run has no analysis in place of {from}"),
            ),
        }
        };
        let mut panes = Vec::new();
        for (index, pane) in self.panes.iter().enumerate() {
            if let Some(binding) = pane.binding
                && binding.dataset == previous
            {
                let from = Some(binding.analysis_id);
                let to = match analysis(from) {
                    Some(to) => to,
                    None if follow => continue,
                    None => return Err(unmapped(from)),
                };
                panes.push((index, to));
            }
        }
        let mut traces = Vec::new();
        {
            let dataset = self.dataset_for_binding(next)?;
            // Each analysis's signal names, and its identity spelled as a
            // predicate spells it.
            let mut signals_by_analysis =
                HashMap::<AnalysisInstanceId, (SignalNames, String)>::new();
            // The analysis the last trace's predicates named, as spelled
            // there: the traces of one analysis spell it alike, so it is
            // read once, not once a trace.
            let mut spelled: Option<(&str, Option<AnalysisInstanceId>)> = None;
            for (index, trace) in self.traces.iter().enumerate() {
                if trace.binding != previous {
                    continue;
                }
                let named =
                    trace
                        .row_predicates
                        .iter()
                        .find_map(|coordinate| match &coordinate.value {
                            TypedValue::Text(analysis)
                                if coordinate.column == ANALYSIS_ID_COLUMN =>
                            {
                                Some(analysis.as_str())
                            }
                            _ => None,
                        });
                let source = named.and_then(|named| match spelled {
                    Some((held, analysis)) if held == named => analysis,
                    _ => {
                        let analysis = named.parse().ok();
                        spelled = Some((named, analysis));
                        analysis
                    }
                });
                let from = source.or_else(|| {
                    let mut bound = self
                        .panes
                        .iter()
                        .filter_map(|pane| pane.binding)
                        .filter(|binding| binding.dataset == previous)
                        .map(|binding| binding.analysis_id);
                    let first = bound.next()?;
                    bound.all(|other| other == first).then_some(first)
                });
                let to = match analysis(from) {
                    Some(to) => to,
                    None if follow => continue,
                    None => return Err(unmapped(from)),
                };
                if let std::collections::hash_map::Entry::Vacant(entry) =
                    signals_by_analysis.entry(to)
                {
                    entry.insert((SignalNames::of(dataset, to, describe_next)?, to.to_string()));
                }
                let (names, spelled_to) = &signals_by_analysis[&to];
                let reading = if names.is_wide() {
                    Reading::Unread
                } else {
                    // The vector the trace was provisioned from, not what it
                    // is called: a reader who renames a trace has not
                    // repointed it at another signal. A signal a table
                    // lacks keeps its trace, marked missing, naming the
                    // analysis it now stands in; a run says whether it has
                    // the signal when the trace is drawn. Predicates that
                    // name the signal in a run already are kept.
                    let signal = trace.source_signal().unwrap_or(&trace.label);
                    let component = trace_component(trace);
                    if matches!(names, SignalNames::Run(_))
                        && super::run_source::are_run_signal_predicates(
                            &trace.row_predicates,
                            spelled_to,
                            signal,
                            component,
                        )
                    {
                        Reading::Kept
                    } else {
                        match names.predicates(signal, component) {
                            Some(predicates) => Reading::Read(predicates, false),
                            None if keep_missing || trace.missing => Reading::Read(
                                missing_signal_predicates(&trace.row_predicates, to),
                                true,
                            ),
                            None => {
                                return Err(VisualizationError::InvalidValue {
                                    field: "trace.row-predicates",
                                    message: format!(
                                        "the {describe_next} dataset has no exact display trace named {signal:?}"
                                    ),
                                });
                            }
                        }
                    }
                };
                traces.push((index, reading));
            }
        }
        if panes.is_empty() && traces.is_empty() && !follow {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: format!(
                    "dataset {} is attached but no pane or trace is bound to it",
                    previous.dataset_id
                ),
            });
        }
        let moved: HashSet<PaneId> = panes
            .iter()
            .map(|(index, _)| self.panes[*index].id)
            .collect();
        for (index, to) in panes {
            if let Some(binding) = self.panes.at_mut(index).binding.as_mut() {
                binding.dataset = next;
                binding.analysis_id = to;
            }
        }
        for expression in self.expression_traces.edit().iter_mut() {
            // Following moves only the panes of the analyses the new run
            // solves, and an expression reads the run its pane does.
            if follow && !moved.contains(&expression.pane_id) {
                continue;
            }
            for binding in &mut expression.datasets {
                if *binding == previous {
                    *binding = next;
                }
            }
            expression.datasets.dedup();
        }
        for (index, reading) in traces {
            match reading {
                Reading::Read(predicates, missing) => self.traces.update_at(index, |trace| {
                    trace.row_predicates = predicates.into();
                    trace.signal_key = "y".into();
                    trace.coordinate_key = "x".into();
                    trace.missing = missing;
                    trace.binding = next;
                }),
                // Its predicates, and so its signal, stand: it stays under
                // its key.
                Reading::Kept => self.traces.update_unkeyed_at(index, |trace| {
                    trace.missing = false;
                    trace.binding = next;
                }),
                Reading::Unread => self.traces.update_unkeyed_at(index, |trace| {
                    trace.binding = next;
                }),
            }
        }
        Ok(())
    }

    pub(super) fn add_trace(
        &mut self,
        trace: NewTrace,
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        validate_label("trace.label", &trace.label)?;
        validate_key("trace.signal-key", &trace.signal_key)?;
        validate_key("trace.coordinate-key", &trace.coordinate_key)?;
        self.require_pane(trace.pane_id)?;
        self.require_axis_in_pane(trace.x_axis_id, trace.pane_id)?;
        self.require_axis_in_pane(trace.y_axis_id, trace.pane_id)?;
        let dataset = self.dataset_for_binding(trace.binding)?;
        if !trace_columns_are_drawable(dataset, &trace.signal_key, &trace.coordinate_key)? {
            return Err(VisualizationError::InvalidValue {
                field: "trace.source-columns",
                message: "trace requires one signal column, drawn against a coordinate column \
                          or against another signal column"
                    .to_owned(),
            });
        }
        validate_trace_row_predicates(dataset, &trace.row_predicates, &self.row_index)?;
        let id = TraceId::allocate(self.allocate_serial()?)?;
        self.traces.push(Trace {
            id,
            pane_id: trace.pane_id,
            binding: trace.binding,
            signal_key: trace.signal_key.into(),
            coordinate_key: trace.coordinate_key.into(),
            row_predicates: trace.row_predicates.into(),
            x_axis_id: trace.x_axis_id,
            y_axis_id: trace.y_axis_id,
            label: trace.label,
            visible: true,
            color: None,
            lane: TraceLane::default(),
            style: TraceStyle::default(),
            missing: false,
        });
        created.push(EntityRef::Trace(id));
        Ok(())
    }

    fn rename(&mut self, entity: EntityRef, value: String) -> Result<(), VisualizationError> {
        validate_label("entity.label", &value)?;
        match entity {
            EntityRef::Page(id) => self.page_mut(id)?.title = value,
            EntityRef::Pane(id) => self.pane_mut(id)?.title = value,
            EntityRef::Trace(id) => self.update_trace(id, |trace| trace.label = value)?,
            EntityRef::Axis(id) => self.axis_mut(id)?.label = value,
            EntityRef::Cursor(id) => self.cursor_mut(id)?.label = value,
            EntityRef::Marker(id) => self.marker_mut(id)?.label = value,
            EntityRef::Measurement(id) => self.measurement_mut(id)?.label = value,
            EntityRef::Annotation(_) => {
                return Err(VisualizationError::InvalidValue {
                    field: "entity.label",
                    message: "annotations are edited with SetAnnotation".to_owned(),
                });
            }
            EntityRef::LinkGroup(id) => self.link_group_mut(id)?.label = value,
        }
        Ok(())
    }

    /// Take out each of `entities` in order, as removing each on its own
    /// would, refusing the first that cannot go: one in use, or one not
    /// there. Each list is passed once for all the entities it loses, and
    /// what depends on traces and axes is read once, so removing k entities
    /// costs about k look-ups, not k passes of the view. A page or a pane
    /// takes out everything on it.
    pub(super) fn remove_all(
        &mut self,
        entities: Vec<EntityRef>,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        let mut removal = Removal::default();
        // Panes going one after another, taken out together.
        let mut panes: Vec<PaneId> = Vec::new();
        for entity in entities {
            if let Some(group) = self.link_groups.iter().find(|group| {
                group.members.contains(&entity)
                    && !removal.gone.contains(&EntityRef::LinkGroup(group.id))
            }) {
                return Err(VisualizationError::EntityInUse {
                    entity,
                    dependent: EntityRef::LinkGroup(group.id),
                });
            }
            match entity {
                EntityRef::Page(id) => {
                    self.take_out(std::mem::take(&mut removal), revision, tombstoned);
                    self.require_page(id)?;
                    for pane in self.panes.iter().filter(|pane| pane.page_id == id) {
                        if !panes.contains(&pane.id) {
                            panes.push(pane.id);
                        }
                    }
                    self.remove_panes_cascade(std::mem::take(&mut panes), revision, tombstoned);
                    self.pages.retain(|page| page.id != id);
                    self.record_tombstone(entity, revision, tombstoned);
                }
                EntityRef::Pane(id) => {
                    self.take_out(std::mem::take(&mut removal), revision, tombstoned);
                    self.require_pane(id)?;
                    if panes.contains(&id) {
                        return Err(VisualizationError::EntityNotFound(entity));
                    }
                    panes.push(id);
                }
                _ => {
                    self.remove_panes_cascade(std::mem::take(&mut panes), revision, tombstoned);
                    if removal.gone.contains(&entity) {
                        return Err(VisualizationError::EntityNotFound(entity));
                    }
                    self.require_entity(entity)?;
                    if let Some(dependent) = self.dependent(entity, &mut removal) {
                        return Err(VisualizationError::EntityInUse { entity, dependent });
                    }
                    removal.gone.insert(entity);
                    removal.order.push(entity);
                }
            }
        }
        self.remove_panes_cascade(panes, revision, tombstoned);
        self.take_out(removal, revision, tombstoned);
        Ok(())
    }

    /// The first entity still in the view that keeps `entity` from going:
    /// a trace or a cursor on an axis; a marker, a measurement or an
    /// annotation on a trace.
    fn dependent(&self, entity: EntityRef, removal: &mut Removal) -> Option<EntityRef> {
        let dependents = match entity {
            EntityRef::Axis(id) => removal
                .axis_dependents
                .get_or_insert_with(|| self.axis_dependents())
                .get(&id),
            EntityRef::Trace(id) => removal
                .trace_dependents
                .get_or_insert_with(|| self.trace_dependents())
                .get(&id),
            _ => None,
        }?;
        dependents
            .iter()
            .find(|dependent| !removal.gone.contains(dependent))
            .copied()
    }

    /// What draws on each axis: its traces, then its cursors, each in the
    /// document's order.
    fn axis_dependents(&self) -> HashMap<AxisId, Vec<EntityRef>> {
        let mut dependents: HashMap<AxisId, Vec<EntityRef>> = HashMap::new();
        for trace in &self.traces {
            dependents
                .entry(trace.x_axis_id)
                .or_default()
                .push(EntityRef::Trace(trace.id));
            if trace.y_axis_id != trace.x_axis_id {
                dependents
                    .entry(trace.y_axis_id)
                    .or_default()
                    .push(EntityRef::Trace(trace.id));
            }
        }
        for cursor in &self.cursors {
            dependents
                .entry(cursor.axis_id)
                .or_default()
                .push(EntityRef::Cursor(cursor.id));
        }
        dependents
    }

    /// What reads each trace: its markers, then its measurements, then the
    /// annotations anchored on it, each in the document's order.
    fn trace_dependents(&self) -> HashMap<TraceId, Vec<EntityRef>> {
        let mut dependents: HashMap<TraceId, Vec<EntityRef>> = HashMap::new();
        for marker in &self.markers {
            dependents
                .entry(marker.trace_id)
                .or_default()
                .push(EntityRef::Marker(marker.id));
        }
        for measurement in &self.measurements {
            for trace_id in &measurement.trace_ids {
                dependents
                    .entry(*trace_id)
                    .or_default()
                    .push(EntityRef::Measurement(measurement.id));
            }
        }
        for annotation in &self.annotations {
            if let AnnotationAnchor::Trace { trace_id, .. } = &annotation.anchor {
                dependents
                    .entry(*trace_id)
                    .or_default()
                    .push(EntityRef::Annotation(annotation.id));
            }
        }
        dependents
    }

    /// Take out what `removal` gathered, each list in one pass, and
    /// tombstone each entity in the order it was removed. A user bus is a
    /// grouping of lanes, not a dependent of them: removing a member
    /// narrows the word, and a bus left with no member is no bus.
    fn take_out(
        &mut self,
        removal: Removal,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) {
        if removal.order.is_empty() {
            return;
        }
        let gone = |kind: fn(&EntityRef) -> Option<u64>| -> HashSet<u64> {
            removal.order.iter().filter_map(kind).collect()
        };
        let traces = gone(|entity| match entity {
            EntityRef::Trace(id) => Some(id.get()),
            _ => None,
        });
        let axes = gone(|entity| match entity {
            EntityRef::Axis(id) => Some(id.get()),
            _ => None,
        });
        let cursors = gone(|entity| match entity {
            EntityRef::Cursor(id) => Some(id.get()),
            _ => None,
        });
        let markers = gone(|entity| match entity {
            EntityRef::Marker(id) => Some(id.get()),
            _ => None,
        });
        let measurements = gone(|entity| match entity {
            EntityRef::Measurement(id) => Some(id.get()),
            _ => None,
        });
        let annotations = gone(|entity| match entity {
            EntityRef::Annotation(id) => Some(id.get()),
            _ => None,
        });
        let link_groups = gone(|entity| match entity {
            EntityRef::LinkGroup(id) => Some(id.get()),
            _ => None,
        });
        if !axes.is_empty() {
            self.axes.retain(|axis| !axes.contains(&axis.id.get()));
        }
        if !cursors.is_empty() {
            self.cursors
                .retain(|cursor| !cursors.contains(&cursor.id.get()));
        }
        if !markers.is_empty() {
            self.markers
                .retain(|marker| !markers.contains(&marker.id.get()));
        }
        if !measurements.is_empty() {
            self.measurements
                .retain(|measurement| !measurements.contains(&measurement.id.get()));
        }
        if !annotations.is_empty() {
            self.annotations
                .retain(|annotation| !annotations.contains(&annotation.id.get()));
        }
        if !link_groups.is_empty() {
            self.link_groups
                .retain(|group| !link_groups.contains(&group.id.get()));
        }
        if !traces.is_empty() {
            self.traces
                .retain(|trace| !traces.contains(&trace.id.get()));
            let removed = |trace_id: &TraceId| traces.contains(&trace_id.get());
            if self
                .user_buses
                .iter()
                .any(|bus| bus.members.iter().any(removed))
            {
                self.user_buses.edit().retain_mut(|bus| {
                    if !bus.members.iter().any(removed) {
                        return true;
                    }
                    bus.members.retain(|member| !removed(member));
                    !bus.members.is_empty()
                });
            }
            let removed_lane = |lane: &EventLaneRef| matches!(lane, EventLaneRef::Trace { trace_id } if removed(trace_id));
            if self
                .event_lane_orders
                .iter()
                .any(|order| order.lanes.iter().any(removed_lane))
            {
                self.retain_event_lanes(|_, lane| !removed_lane(lane));
            }
        }
        for entity in removal.order {
            self.record_tombstone(entity, revision, tombstoned);
        }
    }

    /// Take out the panes `pane_ids`, in that order, and everything on
    /// them: each list is passed once for them all.
    fn remove_panes_cascade(
        &mut self,
        pane_ids: Vec<PaneId>,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) {
        if pane_ids.is_empty() {
            return;
        }
        let gone: HashSet<PaneId> = pane_ids.iter().copied().collect();
        // What each pane loses, kind by kind, in the order each list holds it.
        let mut removed: HashMap<PaneId, Vec<EntityRef>> = HashMap::new();
        let mut note = |pane_id: PaneId, entity: EntityRef| {
            if gone.contains(&pane_id) {
                removed.entry(pane_id).or_default().push(entity);
            }
        };
        for axis in &self.axes {
            note(axis.pane_id, EntityRef::Axis(axis.id));
        }
        for trace in &self.traces {
            note(trace.pane_id, EntityRef::Trace(trace.id));
        }
        for cursor in &self.cursors {
            note(cursor.pane_id, EntityRef::Cursor(cursor.id));
        }
        for marker in &self.markers {
            note(marker.pane_id, EntityRef::Marker(marker.id));
        }
        for measurement in &self.measurements {
            note(measurement.pane_id, EntityRef::Measurement(measurement.id));
        }
        for annotation in &self.annotations {
            note(annotation.pane_id, EntityRef::Annotation(annotation.id));
        }
        let stays = |pane_id: &PaneId| !gone.contains(pane_id);
        self.axes.retain(|axis| stays(&axis.pane_id));
        self.traces.retain(|trace| stays(&trace.pane_id));
        self.cursors.retain(|cursor| stays(&cursor.pane_id));
        self.markers.retain(|marker| stays(&marker.pane_id));
        self.measurements
            .retain(|measurement| stays(&measurement.pane_id));
        self.annotations
            .retain(|annotation| stays(&annotation.pane_id));
        self.user_buses.retain(|bus| stays(&bus.pane_id));
        self.event_lane_orders.retain(|order| stays(&order.pane_id));
        self.expression_traces.retain(|trace| stays(&trace.pane_id));
        self.xy_plots.retain(|plot| stays(&plot.pane_id));
        self.readout_pins.retain(|pin| stays(&pin.pane_id));
        self.sheet_settings
            .retain(|setting| stays(&setting.pane_id));
        self.spectrum_settings.retain(|held| stays(&held.pane_id));
        self.coefficient_settings
            .retain(|held| stays(&held.pane_id));
        let mut pages = Vec::new();
        for pane in self.panes.iter().filter(|pane| !stays(&pane.id)) {
            if !pages.contains(&pane.page_id) {
                pages.push(pane.page_id);
            }
        }
        self.panes.retain(|pane| stays(&pane.id));
        for page_id in pages {
            self.normalize_page_pane_layout(page_id, None);
        }
        for pane_id in pane_ids {
            for entity in removed.remove(&pane_id).unwrap_or_default() {
                self.record_tombstone(entity, revision, tombstoned);
            }
            self.record_tombstone(EntityRef::Pane(pane_id), revision, tombstoned);
        }
    }

    fn record_tombstone(
        &mut self,
        entity: EntityRef,
        revision: ObjectRevision,
        receipt: &mut Vec<EntityRef>,
    ) {
        self.tombstones.push(Tombstone {
            entity,
            deleted_at_revision: revision,
        });
        receipt.push(entity);
    }

    fn validate_comparison_receipt(
        &self,
        entities: &dyn Entities,
        receipt: &ComparisonReceipt,
    ) -> Result<(), VisualizationError> {
        entities.dataset(receipt.baseline)?;
        entities.dataset(receipt.candidate)?;
        receipt.validate_structure()
    }

    /// Check what the running transaction changed. The rows a check reads
    /// of its embedded tables are let go once it ends, whatever it finds.
    pub(super) fn check_changes(&self) -> Result<(), VisualizationError> {
        let checked = self.check_touched();
        self.row_index.clear();
        checked
    }

    /// Check each entity the running transaction made or changed, the
    /// groups those belong to (the panes of a page, the settings of a
    /// pane's sheets), and what reads anything it took out or changed the
    /// reading of. Every other entity held before the transaction and still
    /// holds: nothing it reads changed. A transaction that changes datasets
    /// the view reads has the set checked, and what reads those datasets.
    fn check_touched(&self) -> Result<(), VisualizationError> {
        let datasets = self.changed_datasets();
        if self.datasets.is_touched() {
            validate_dataset_set(&self.datasets)?;
        }
        self.tracking.validate()?;
        self.presentation.validate()?;
        if self.pages.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.pages",
                message: "at least one page is required".to_owned(),
            });
        }
        let entities: &dyn Entities = self;
        let mut pages = HashSet::new();
        for page in self.pages.touched() {
            validate_label("page.title", &page.title)?;
            page.layout.validate()?;
            validate_key("page.template-id", &page.template_id)?;
            pages.insert(page.id);
        }
        let mut panes = self.panes.touched();
        if !datasets.is_empty() {
            panes.extend(self.panes.iter().filter(|pane| {
                pane.binding
                    .is_some_and(|binding| datasets.contains(&binding.dataset))
                    && !self.panes.touches(pane.id.get())
            }));
        }
        for pane in &panes {
            pages.insert(pane.page_id);
            if let Some(before) = self.panes.before(pane.id.get()) {
                pages.insert(before.page_id);
            }
        }
        for identity in self.panes.taken_out() {
            if let Some(before) = self.panes.before(identity) {
                pages.insert(before.page_id);
            }
        }
        let mut pages: Vec<&Page> = pages
            .into_iter()
            .filter_map(|page_id| {
                self.pages
                    .position(page_id.get())
                    .map(|index| &self.pages[index])
            })
            .collect();
        pages.sort_by_key(|page| page.id);
        for page in pages {
            let on_page = self
                .panes
                .iter()
                .filter(|pane| pane.page_id == page.id)
                .collect();
            Self::check_page_panes(page, on_page)?;
        }
        for pane in panes {
            validate_label("pane.title", &pane.title)?;
            entities.require_page(pane.page_id)?;
            self.validate_pane_source(entities, pane.kind, &pane.viewer_id, pane.binding)?;
            super::strips::validate_strips(&pane.strips)?;
            if let Some(policy) = &pane.family_policy
                && self.panes.before(pane.id.get()).is_none_or(|before| {
                    before.family_policy != pane.family_policy || before.binding != pane.binding
                })
            {
                self.validate_pane_family_policy(entities, pane.id, policy)?;
            }
        }
        for axis in self.axes.touched() {
            validate_label("axis.label", &axis.label)?;
            validate_optional_unit("axis.unit", axis.unit.as_deref())?;
            entities.require_pane(axis.pane_id)?;
        }
        // A trace whose reading did not change holds as it held; one made,
        // or one that reads something else now, is checked, and so is what
        // reads it. One moved onto a newer content of its run, which a trace
        // reads as it read the content before, reads the same.
        let grown = self.grown_runs();
        let mut reread = HashSet::new();
        let mut relaned = false;
        let check_trace = |trace: &Trace| -> Result<(), VisualizationError> {
            Self::validate_trace(entities, trace)?;
            if let Some(policy) = &entities.pane(trace.pane_id)?.family_policy {
                validate_family_source(entities, trace.binding, policy)?;
            }
            Ok(())
        };
        for (trace, before) in self.traces.touched_with_before() {
            relaned |= before.is_some_and(|before| before.lane.kind != trace.lane.kind);
            if !datasets.contains(&trace.binding)
                && (before.is_some_and(|before| reads_the_same(before, trace))
                    || self.traces.made_checked(trace.id.get()))
            {
                continue;
            }
            if before.is_some_and(|before| {
                grown.contains(&(before.binding, trace.binding)) && reads_alike(before, trace)
            }) {
                continue;
            }
            if before.is_some() {
                reread.insert(trace.id);
            }
            check_trace(trace)?;
        }
        // A trace the transaction left as it was reads otherwise when its
        // dataset changed.
        if !datasets.is_empty() {
            for trace in self.traces.iter().filter(|trace| {
                datasets.contains(&trace.binding) && !self.traces.touches(trace.id.get())
            }) {
                reread.insert(trace.id);
                check_trace(trace)?;
            }
        }
        for cursor in self.cursors.touched() {
            validate_label("cursor.label", &cursor.label)?;
            cursor.position.validate("cursor.position")?;
            entities.require_axis_in_pane(cursor.axis_id, cursor.pane_id)?;
        }
        let markers = self.markers.touched();
        let markers = if reread.is_empty() {
            markers
        } else {
            let touched: HashSet<MarkerId> = markers.iter().map(|marker| marker.id).collect();
            self.markers
                .iter()
                .filter(|marker| touched.contains(&marker.id) || reread.contains(&marker.trace_id))
                .collect()
        };
        for marker in markers {
            Self::validate_marker(entities, marker)?;
        }
        for measurement in self.measurements.touched() {
            Self::validate_measurement(entities, measurement)?;
        }
        let taken = self.taken_out();
        let annotations = self.annotations.touched();
        let annotations = if reread.is_empty() && taken.is_empty() {
            annotations
        } else {
            let touched: HashSet<AnnotationId> =
                annotations.iter().map(|annotation| annotation.id).collect();
            self.annotations
                .iter()
                .filter(|annotation| {
                    touched.contains(&annotation.id)
                        || taken.contains(&EntityRef::Pane(annotation.pane_id))
                        || matches!(&annotation.anchor, AnnotationAnchor::Trace { trace_id, .. }
                            if reread.contains(trace_id) || taken.contains(&EntityRef::Trace(*trace_id)))
                })
                .collect()
        };
        for annotation in annotations {
            validate_annotation(
                entities,
                annotation.pane_id,
                &annotation.anchor,
                &annotation.text,
            )?;
        }
        let groups = if taken.is_empty() {
            self.link_groups.touched()
        } else {
            self.link_groups.iter().collect()
        };
        for group in groups {
            validate_label("link-group.label", &group.label)?;
            validate_link_members(entities, group.kind, &group.members)?;
        }
        let mut deleted = HashSet::new();
        for tombstone in self.tombstones.added() {
            if !deleted.insert(tombstone.entity)
                || entities.require_entity(tombstone.entity).is_ok()
            {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.tombstones",
                    message: "tombstones must be unique and must not identify live entities"
                        .to_owned(),
                });
            }
        }
        // The lists without identities are each a few per pane, and are
        // checked whole when changed, or when a dataset they may read did.
        let reread_lists = !datasets.is_empty();
        if self.comparisons.is_touched() || reread_lists {
            for receipt in &self.comparisons {
                self.validate_comparison_receipt(entities, receipt)?;
            }
        }
        if self.event_lane_orders.is_touched() || relaned || reread_lists {
            self.check_event_lane_orders(entities)?;
        }
        if self.expression_traces.is_touched() || reread_lists {
            self.check_expression_traces(entities)?;
        }
        if self.xy_plots.is_touched() || reread_lists {
            self.check_xy_plots(entities)?;
        }
        if self.readout_pins.is_touched() || reread_lists {
            self.check_readout_pins(entities)?;
        }
        if self.sheet_settings.is_touched() || reread_lists {
            self.check_sheet_settings(entities)?;
        }
        if self.spectrum_settings.is_touched() || reread_lists {
            self.check_spectrum_settings(entities)?;
        }
        if self.coefficient_settings.is_touched() || reread_lists {
            self.check_coefficient_settings(entities)?;
        }
        if self.user_buses.is_touched() || relaned || reread_lists {
            self.check_user_buses(entities)?;
        }
        Ok(())
    }

    /// The datasets the running transaction attached, changed or took out:
    /// what reads one of them reads otherwise.
    /// Each run reference the running transaction replaced with a newer
    /// content of the same run that a trace reads alike: the same analyses,
    /// through the same columns. Before and after.
    fn grown_runs(&self) -> Vec<(DatasetBinding, DatasetBinding)> {
        let (before, after) = self.datasets.changed();
        before
            .iter()
            .filter_map(|held| {
                let now = after.iter().find(|now| {
                    now.binding.dataset_id == held.binding.dataset_id && now.binding != held.binding
                })?;
                let (Some(was), Some(is)) = (&held.run, &now.run) else {
                    return None;
                };
                (was.analyses() == is.analyses() && held.columns == now.columns)
                    .then_some((held.binding, now.binding))
            })
            .collect()
    }

    fn changed_datasets(&self) -> HashSet<DatasetBinding> {
        let (before, after) = self.datasets.changed();
        before
            .iter()
            .filter(|held| !after.contains(held))
            .chain(after.iter().filter(|now| !before.contains(now)))
            .map(|dataset| dataset.binding)
            .collect()
    }

    /// Every entity the running transaction took out.
    fn taken_out(&self) -> HashSet<EntityRef> {
        let mut taken = HashSet::new();
        let mut take = |identities: Vec<u64>, entity: fn(u64) -> Option<EntityRef>| {
            taken.extend(identities.into_iter().filter_map(entity));
        };
        take(self.pages.taken_out(), |id| {
            PageId::allocate(id).ok().map(EntityRef::Page)
        });
        take(self.panes.taken_out(), |id| {
            PaneId::allocate(id).ok().map(EntityRef::Pane)
        });
        take(self.axes.taken_out(), |id| {
            AxisId::allocate(id).ok().map(EntityRef::Axis)
        });
        take(self.traces.taken_out(), |id| {
            TraceId::allocate(id).ok().map(EntityRef::Trace)
        });
        take(self.cursors.taken_out(), |id| {
            CursorId::allocate(id).ok().map(EntityRef::Cursor)
        });
        take(self.markers.taken_out(), |id| {
            MarkerId::allocate(id).ok().map(EntityRef::Marker)
        });
        take(self.measurements.taken_out(), |id| {
            MeasurementId::allocate(id).ok().map(EntityRef::Measurement)
        });
        take(self.annotations.taken_out(), |id| {
            AnnotationId::allocate(id).ok().map(EntityRef::Annotation)
        });
        take(self.link_groups.taken_out(), |id| {
            LinkGroupId::allocate(id).ok().map(EntityRef::LinkGroup)
        });
        taken
    }

    /// The panes of one page: one at most on a single-pane page, their
    /// orders contiguous from zero, the first primary and each other placed
    /// against an earlier one.
    fn check_page_panes(page: &Page, mut panes: Vec<&Pane>) -> Result<(), VisualizationError> {
        if page.layout == PageLayout::SinglePane && panes.len() > 1 {
            return Err(VisualizationError::InvalidValue {
                field: "page.layout",
                message: "single-pane layout cannot contain more than one pane".to_owned(),
            });
        }
        panes.sort_by_key(|pane| (pane.order, pane.id));
        let mut preceding = HashSet::new();
        for (expected_order, pane) in panes.into_iter().enumerate() {
            if pane.order != u32::try_from(expected_order).unwrap_or(u32::MAX) {
                return Err(VisualizationError::InvalidValue {
                    field: "pane.order",
                    message: format!(
                        "pane orders on page {} must be unique and contiguous from zero",
                        page.id.get()
                    ),
                });
            }
            match pane.placement {
                PanePlacement::Primary if expected_order == 0 => {}
                PanePlacement::Primary => {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.placement",
                        message: "only the first pane on a page may be primary".to_owned(),
                    });
                }
                PanePlacement::Below { anchor_pane_id }
                | PanePlacement::RightOf { anchor_pane_id }
                    if preceding.contains(&anchor_pane_id) => {}
                PanePlacement::Below { .. } | PanePlacement::RightOf { .. } => {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.placement",
                        message: "pane placement must reference an earlier pane on the same page"
                            .to_owned(),
                    });
                }
            }
            preceding.insert(pane.id);
        }
        Ok(())
    }

    /// One trace on its own: its names, its pane and axes, and the rows it
    /// reads of its dataset.
    fn validate_trace(entities: &dyn Entities, trace: &Trace) -> Result<(), VisualizationError> {
        validate_label("trace.label", &trace.label)?;
        validate_key("trace.signal-key", &trace.signal_key)?;
        validate_key("trace.coordinate-key", &trace.coordinate_key)?;
        entities.require_pane(trace.pane_id)?;
        entities.require_axis_in_pane(trace.x_axis_id, trace.pane_id)?;
        entities.require_axis_in_pane(trace.y_axis_id, trace.pane_id)?;
        let dataset = entities.dataset(trace.binding)?;
        if !trace_columns_are_drawable(dataset, &trace.signal_key, &trace.coordinate_key)? {
            return Err(VisualizationError::InvalidValue {
                field: "trace.source-columns",
                message: "invalid signal or coordinate column role".to_owned(),
            });
        }
        if !trace.missing {
            validate_trace_row_predicates(dataset, &trace.row_predicates, entities.rows())?;
        }
        Ok(())
    }

    /// One marker on its own: its names, and a coordinate its trace holds.
    fn validate_marker(entities: &dyn Entities, marker: &Marker) -> Result<(), VisualizationError> {
        validate_label("marker.label", &marker.label)?;
        if let Some(source) = marker.source_specification.as_deref() {
            validate_label("marker.source-specification", source)?;
        }
        marker.coordinate.validate("marker.coordinate")?;
        entities.require_trace_in_pane(marker.trace_id, marker.pane_id)?;
        if !trace_contains_exact_coordinate(entities, marker.trace_id, &marker.coordinate)? {
            return Err(VisualizationError::InterpolationRequired);
        }
        Ok(())
    }

    /// One measurement on its own: what it evaluates, on its pane's traces.
    fn validate_measurement(
        entities: &dyn Entities,
        measurement: &Measurement,
    ) -> Result<(), VisualizationError> {
        validate_label("measurement.label", &measurement.label)?;
        match (measurement.expression.as_deref(), measurement.value) {
            (Some(expression), Some(value))
                if !expression.trim().is_empty()
                    && expression.len() <= MAX_SOURCE_TEXT_BYTES
                    && !expression.chars().any(char::is_control)
                    && value.is_finite() => {}
            (None, None) => {}
            _ => {
                return Err(VisualizationError::InvalidValue {
                    field: "measurement.expression",
                    message: format!(
                        "an authored scalar measurement requires both a finite value and 1 to {MAX_SOURCE_TEXT_BYTES} non-control UTF-8 expression bytes"
                    ),
                });
            }
        }
        if let Some(family) = &measurement.family {
            family.validate(measurement.value)?;
        }
        validate_measurement_traces(
            measurement.trace_ids.len(),
            measurement.expression.is_some(),
        )?;
        entities.require_pane(measurement.pane_id)?;
        for trace in &measurement.trace_ids {
            entities.require_trace_in_pane(*trace, measurement.pane_id)?;
        }
        Ok(())
    }

    fn check_event_lane_orders(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut ordered_panes = HashSet::new();
        for order in &self.event_lane_orders {
            self.validate_event_lane_order(entities, order)?;
            if order.lanes.is_empty() || !ordered_panes.insert(order.pane_id) {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.event-lane-orders",
                    message: format!(
                        "pane {} holds an empty or second event-lane order",
                        order.pane_id.get()
                    ),
                });
            }
        }
        Ok(())
    }

    fn check_expression_traces(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut expressions = HashSet::new();
        for trace in &self.expression_traces {
            self.validate_expression_trace(entities, trace)?;
            if !expressions.insert((trace.pane_id, trace.text.as_str())) {
                return Err(VisualizationError::InvalidValue {
                    field: "expression-trace.text",
                    message: format!(
                        "pane {} plots the expression {:?} twice",
                        trace.pane_id.get(),
                        trace.text
                    ),
                });
            }
        }
        Ok(())
    }

    fn check_xy_plots(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut plots = HashSet::new();
        for plot in &self.xy_plots {
            self.validate_xy_plot(entities, plot)?;
            if !plots.insert((plot.pane_id, plot.unit.as_str(), &plot.abscissa)) {
                return Err(duplicate_xy_plot(plot));
            }
        }
        Ok(())
    }

    fn check_readout_pins(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut pins = HashSet::new();
        for pin in &self.readout_pins {
            entities.require_pane(pin.pane_id)?;
            validate_readout_pin(pin)?;
            if !pins.insert((pin.pane_id, pin.sheet)) {
                return Err(VisualizationError::InvalidValue {
                    field: "readout-pin.sheet",
                    message: format!(
                        "pane {} pins its {:?} readout twice",
                        pin.pane_id.get(),
                        pin.sheet
                    ),
                });
            }
        }
        Ok(())
    }

    fn check_sheet_settings(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut settings = HashSet::new();
        for setting in &self.sheet_settings {
            entities.require_pane(setting.pane_id)?;
            validate_sheet_setting(setting)?;
            if !settings.insert((setting.pane_id, setting.setting.sheet())) {
                return Err(VisualizationError::InvalidValue {
                    field: "sheet-setting.sheet",
                    message: format!(
                        "pane {} keeps its {:?} settings twice",
                        setting.pane_id.get(),
                        setting.setting.sheet()
                    ),
                });
            }
        }
        Ok(())
    }

    fn check_spectrum_settings(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut spectra = HashSet::new();
        for held in &self.spectrum_settings {
            entities.require_pane(held.pane_id)?;
            validate_spectrum_settings(&held.settings)?;
            if !spectra.insert(held.pane_id) {
                return Err(VisualizationError::InvalidValue {
                    field: "spectrum-settings.pane-id",
                    message: format!(
                        "pane {} keeps its spectrum settings twice",
                        held.pane_id.get()
                    ),
                });
            }
        }
        Ok(())
    }

    fn check_coefficient_settings(
        &self,
        entities: &dyn Entities,
    ) -> Result<(), VisualizationError> {
        let mut coefficients = HashSet::new();
        for held in &self.coefficient_settings {
            entities.require_pane(held.pane_id)?;
            validate_coefficient_settings(&held.settings)?;
            if !coefficients.insert(held.pane_id) {
                return Err(VisualizationError::InvalidValue {
                    field: "coefficient-settings.pane-id",
                    message: format!(
                        "pane {} keeps its coefficient settings twice",
                        held.pane_id.get()
                    ),
                });
            }
        }
        Ok(())
    }

    fn check_user_buses(&self, entities: &dyn Entities) -> Result<(), VisualizationError> {
        let mut bus_names = HashSet::new();
        for bus in &self.user_buses {
            self.validate_user_bus(entities, bus)?;
            if !bus_names.insert((bus.pane_id, bus.name.as_str())) {
                return Err(VisualizationError::InvalidValue {
                    field: "user-bus.name",
                    message: format!(
                        "pane {} groups two buses named {:?}",
                        bus.pane_id.get(),
                        bus.name
                    ),
                });
            }
        }
        Ok(())
    }

    /// Check the whole document. The rows a check reads of its embedded
    /// tables are let go once it ends, whatever it finds.
    pub(super) fn validate(&self) -> Result<(), VisualizationError> {
        let checked = self.check_whole();
        self.row_index.clear();
        checked
    }

    fn check_whole(&self) -> Result<(), VisualizationError> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.schema-version",
                message: format!("unsupported schema version {}", self.schema_version),
            });
        }
        validate_label("visualization-document.title", &self.title)?;
        self.tracking.validate()?;
        self.presentation.validate()?;
        validate_dataset_set(&self.datasets)?;
        if self.pages.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.pages",
                message: "at least one page is required".to_owned(),
            });
        }
        let next = self.next_serial;
        require_made_in_order("visualization-document.pages", &self.pages, next)?;
        require_made_in_order("visualization-document.panes", &self.panes, next)?;
        require_made_in_order("visualization-document.axes", &self.axes, next)?;
        require_made_in_order("visualization-document.traces", &self.traces, next)?;
        require_made_in_order("visualization-document.cursors", &self.cursors, next)?;
        require_made_in_order("visualization-document.markers", &self.markers, next)?;
        require_made_in_order(
            "visualization-document.measurements",
            &self.measurements,
            next,
        )?;
        require_made_in_order(
            "visualization-document.annotations",
            &self.annotations,
            next,
        )?;
        require_made_in_order(
            "visualization-document.link-groups",
            &self.link_groups,
            next,
        )?;
        let mut identities = HashSet::new();
        let lookup = Lookup::of(self);
        let mut panes_by_page: HashMap<PageId, Vec<&Pane>> = HashMap::new();
        for pane in &self.panes {
            panes_by_page.entry(pane.page_id).or_default().push(pane);
        }
        for page in &self.pages {
            validate_label("page.title", &page.title)?;
            page.layout.validate()?;
            validate_key("page.template-id", &page.template_id)?;
            ensure_identity(&mut identities, EntityRef::Page(page.id))?;
        }
        for pane in &self.panes {
            validate_label("pane.title", &pane.title)?;
            lookup.require_page(pane.page_id)?;
            self.validate_pane_source(&lookup, pane.kind, &pane.viewer_id, pane.binding)?;
            super::strips::validate_strips(&pane.strips)?;
            if let Some(policy) = &pane.family_policy {
                self.validate_pane_family_policy(&lookup, pane.id, policy)?;
            }
            ensure_identity(&mut identities, EntityRef::Pane(pane.id))?;
        }
        for page in &self.pages {
            Self::check_page_panes(page, panes_by_page.remove(&page.id).unwrap_or_default())?;
        }
        for axis in &self.axes {
            validate_label("axis.label", &axis.label)?;
            validate_optional_unit("axis.unit", axis.unit.as_deref())?;
            lookup.require_pane(axis.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Axis(axis.id))?;
        }
        for trace in &self.traces {
            Self::validate_trace(&lookup, trace)?;
            ensure_identity(&mut identities, EntityRef::Trace(trace.id))?;
        }
        for cursor in &self.cursors {
            validate_label("cursor.label", &cursor.label)?;
            cursor.position.validate("cursor.position")?;
            lookup.require_axis_in_pane(cursor.axis_id, cursor.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Cursor(cursor.id))?;
        }
        for marker in &self.markers {
            Self::validate_marker(&lookup, marker)?;
            ensure_identity(&mut identities, EntityRef::Marker(marker.id))?;
        }
        for measurement in &self.measurements {
            Self::validate_measurement(&lookup, measurement)?;
            ensure_identity(&mut identities, EntityRef::Measurement(measurement.id))?;
        }
        for annotation in &self.annotations {
            validate_annotation(
                &lookup,
                annotation.pane_id,
                &annotation.anchor,
                &annotation.text,
            )?;
            ensure_identity(&mut identities, EntityRef::Annotation(annotation.id))?;
        }
        for group in &self.link_groups {
            validate_label("link-group.label", &group.label)?;
            validate_link_members(&lookup, group.kind, &group.members)?;
            ensure_identity(&mut identities, EntityRef::LinkGroup(group.id))?;
        }
        let mut deleted = HashSet::new();
        for tombstone in &self.tombstones {
            if !deleted.insert(tombstone.entity) || identities.contains(&tombstone.entity) {
                return Err(VisualizationError::InvalidValue {
                    field: "visualization-document.tombstones",
                    message: "tombstones must be unique and must not identify live entities"
                        .to_owned(),
                });
            }
        }
        for receipt in &self.comparisons {
            self.validate_comparison_receipt(&lookup, receipt)?;
        }
        self.check_event_lane_orders(&lookup)?;
        self.check_expression_traces(&lookup)?;
        self.check_xy_plots(&lookup)?;
        self.check_readout_pins(&lookup)?;
        self.check_sheet_settings(&lookup)?;
        self.check_spectrum_settings(&lookup)?;
        self.check_coefficient_settings(&lookup)?;
        self.check_user_buses(&lookup)
    }

    /// Drop the event-lane rows `keep` rejects, and any order left empty.
    fn retain_event_lanes(&mut self, keep: impl Fn(&EventLaneOrder, &EventLaneRef) -> bool) {
        for order in self.event_lane_orders.edit().iter_mut() {
            let held = std::mem::take(&mut order.lanes);
            order.lanes = held.into_iter().filter(|lane| keep(order, lane)).collect();
        }
        self.event_lane_orders
            .retain(|order| !order.lanes.is_empty());
    }

    /// One pane's event-lane order on its own: duplicate-free rows, each a
    /// logic trace of that pane or a named bus. A bus the run declared is no
    /// entity of the document, so a bus row is held to a valid name only.
    fn validate_event_lane_order(
        &self,
        entities: &dyn Entities,
        order: &EventLaneOrder,
    ) -> Result<(), VisualizationError> {
        entities.require_pane(order.pane_id)?;
        let mut seen = HashSet::new();
        for lane in &order.lanes {
            match lane {
                EventLaneRef::Trace { trace_id } => {
                    let trace = entities.trace(*trace_id)?;
                    if trace.pane_id != order.pane_id || trace.lane.kind != TraceLaneKind::Logic {
                        return Err(VisualizationError::InvalidValue {
                            field: "event-lane-order.lanes",
                            message: format!(
                                "trace {} is not a logic lane of pane {}",
                                trace_id.get(),
                                order.pane_id.get()
                            ),
                        });
                    }
                }
                EventLaneRef::Bus { name } => validate_label("event-lane-order.bus", name)?,
                EventLaneRef::Net { name } => validate_label("event-lane-order.net", name)?,
            }
            if !seen.insert(lane) {
                return Err(VisualizationError::InvalidValue {
                    field: "event-lane-order.lanes",
                    message: format!("pane {} orders one lane twice", order.pane_id.get()),
                });
            }
        }
        Ok(())
    }

    /// One expression trace on its own: non-blank text on a pane, reading
    /// attached datasets only, with a well-formed declared unit if any.
    fn validate_expression_trace(
        &self,
        entities: &dyn Entities,
        trace: &ExpressionTrace,
    ) -> Result<(), VisualizationError> {
        entities.require_pane(trace.pane_id)?;
        if trace.text.trim().is_empty()
            || trace.text.len() > MAX_SOURCE_TEXT_BYTES
            || trace.text.chars().any(char::is_control)
        {
            return Err(VisualizationError::InvalidValue {
                field: "expression-trace.text",
                message: format!(
                    "must be non-blank, contain no control characters, and not exceed {MAX_SOURCE_TEXT_BYTES} bytes"
                ),
            });
        }
        validate_optional_unit("expression-trace.unit", trace.unit.as_deref())?;
        if trace.datasets.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "expression-trace.datasets",
                message: "an expression reads at least one attached dataset".to_owned(),
            });
        }
        for binding in &trace.datasets {
            entities.dataset(*binding)?;
        }
        Ok(())
    }

    /// One plot on its own: a pane of the document, a unit axis named within
    /// the bound, a signal name or expression text a label or a calculator
    /// source can hold, and a sweep span that is an interval.
    fn validate_xy_plot(
        &self,
        entities: &dyn Entities,
        choice: &XyPlot,
    ) -> Result<(), VisualizationError> {
        entities.require_pane(choice.pane_id)?;
        if choice.unit.len() > MAX_UNIT_AXIS_BYTES || choice.unit.chars().any(char::is_control) {
            return Err(VisualizationError::InvalidValue {
                field: "xy-plot.unit",
                message: format!(
                    "must contain no control characters and not exceed {MAX_UNIT_AXIS_BYTES} bytes"
                ),
            });
        }
        if let Some(range) = choice.sweep_range {
            AxisRange::new(range.minimum, range.maximum).map_err(|_| {
                VisualizationError::InvalidValue {
                    field: "xy-plot.sweep-range",
                    message: "the sweep span's ends must be finite and its start before its end"
                        .to_owned(),
                }
            })?;
        }
        match &choice.abscissa {
            Abscissa::Signal { name } => validate_label("xy-plot.signal", name),
            Abscissa::Expression { text } => {
                if text.trim().is_empty()
                    || text.len() > MAX_SOURCE_TEXT_BYTES
                    || text.chars().any(char::is_control)
                {
                    return Err(VisualizationError::InvalidValue {
                        field: "xy-plot.expression",
                        message: format!(
                            "must be non-blank, contain no control characters, and not exceed {MAX_SOURCE_TEXT_BYTES} bytes"
                        ),
                    });
                }
                Ok(())
            }
        }
    }

    /// One user bus on its own: a named, non-empty, duplicate-free word of
    /// logic lanes that all sit in the bus's pane.
    fn validate_user_bus(
        &self,
        entities: &dyn Entities,
        bus: &UserBus,
    ) -> Result<(), VisualizationError> {
        validate_label("user-bus.name", &bus.name)?;
        entities.require_pane(bus.pane_id)?;
        if (bus.members.is_empty() && bus.nets.is_empty())
            || (!bus.members.is_empty() && !bus.nets.is_empty())
        {
            return Err(VisualizationError::InvalidValue {
                field: "user-bus.members",
                message: "a user bus groups logic lanes, all traces or all history-only nets"
                    .to_owned(),
            });
        }
        if bus
            .value_names
            .windows(2)
            .any(|pair| pair[0].value >= pair[1].value)
        {
            return Err(VisualizationError::InvalidValue {
                field: "user-bus.value-names",
                message: format!(
                    "bus {:?} names distinct words, in ascending order",
                    bus.name
                ),
            });
        }
        for name in &bus.value_names {
            validate_label("user-bus.value-names", &name.name)?;
        }
        let mut nets = HashSet::new();
        for net in &bus.nets {
            validate_label("user-bus.nets", net)?;
            if !nets.insert(net) {
                return Err(VisualizationError::InvalidValue {
                    field: "user-bus.nets",
                    message: format!("bus {:?} groups the net {net:?} twice", bus.name),
                });
            }
        }
        let mut seen = HashSet::new();
        for member in &bus.members {
            let trace = entities.trace(*member)?;
            if !seen.insert(*member)
                || trace.pane_id != bus.pane_id
                || trace.lane.kind != TraceLaneKind::Logic
            {
                return Err(VisualizationError::InvalidValue {
                    field: "user-bus.members",
                    message: format!(
                        "bus {:?} must group distinct logic lanes of its own pane; trace {} is not one",
                        bus.name,
                        member.get()
                    ),
                });
            }
        }
        Ok(())
    }
}

/// The display signals of one analysis a long-form table holds, as (trace
/// index, name) in source order, and whether the table tells its analyses
/// apart. `None` when the dataset is not a long-form table.
pub(super) fn long_form_display_signals(
    dataset: &SourceDataset,
    analysis_id: AnalysisInstanceId,
) -> Option<(Vec<(i64, String)>, bool)> {
    if dataset.run.is_some() {
        return None;
    }
    let column_index = |key: &str| dataset.columns.iter().position(|column| column.key == key);
    let analysis_identity = analysis_id.to_string();
    let analysis_column = column_index(ANALYSIS_ID_COLUMN);
    let (Some(trace_index), Some(trace_name), Some(component), Some(x), Some(y)) = (
        column_index(TRACE_INDEX_COLUMN),
        column_index(TRACE_NAME_COLUMN),
        column_index(COMPONENT_COLUMN),
        column_index("x"),
        column_index("y"),
    ) else {
        return None;
    };
    if dataset.columns[trace_index].role != ColumnRole::Coordinate
        || dataset.columns[trace_name].role != ColumnRole::Coordinate
        || dataset.columns[component].role != ColumnRole::Coordinate
        || dataset.columns[x].role != ColumnRole::Coordinate
        || dataset.columns[y].role != ColumnRole::Signal
    {
        return None;
    }
    let mut signals = Vec::new();
    let mut seen_trace_indices = HashSet::new();
    for row in &dataset.rows {
        if analysis_column.is_some_and(|analysis_column| {
            !matches!(
                row.values.get(analysis_column),
                Some(TypedValue::Text(value)) if value == &analysis_identity
            )
        }) {
            continue;
        }
        let (TypedValue::Integer(index), TypedValue::Text(name), TypedValue::Text(component_name)) = (
            &row.values[trace_index],
            &row.values[trace_name],
            &row.values[component],
        ) else {
            continue;
        };
        if component_name == SignalComponent::Display.key() && seen_trace_indices.insert(*index) {
            signals.push((*index, name.clone()));
        }
    }
    Some((signals, analysis_column.is_some()))
}

/// A missing trace's predicates: its own, naming `analysis` in place of the
/// analysis it read before, so a later run is matched to it as to any trace.
fn missing_signal_predicates(
    predicates: &[QueryCoordinate],
    analysis: AnalysisInstanceId,
) -> Vec<QueryCoordinate> {
    predicates
        .iter()
        .map(|coordinate| {
            if coordinate.column == ANALYSIS_ID_COLUMN {
                QueryCoordinate {
                    column: ANALYSIS_ID_COLUMN.to_owned(),
                    value: TypedValue::Text(analysis.to_string()),
                }
            } else {
                coordinate.clone()
            }
        })
        .collect()
}

/// The exact predicates that select one display signal of a long-form source.
pub(super) fn display_signal_predicates(
    index: i64,
    name: &str,
    analysis: Option<&str>,
) -> Vec<QueryCoordinate> {
    let mut predicates = vec![
        QueryCoordinate {
            column: TRACE_INDEX_COLUMN.to_owned(),
            value: TypedValue::Integer(index),
        },
        QueryCoordinate {
            column: TRACE_NAME_COLUMN.to_owned(),
            value: TypedValue::Text(name.to_owned()),
        },
        QueryCoordinate {
            column: COMPONENT_COLUMN.to_owned(),
            value: TypedValue::Text(SignalComponent::Display.key().to_owned()),
        },
    ];
    if let Some(analysis) = analysis {
        predicates.push(QueryCoordinate {
            column: ANALYSIS_ID_COLUMN.to_owned(),
            value: TypedValue::Text(analysis.to_owned()),
        });
    }
    predicates
}

/// Each display signal of one analysis by name, refusing a name two signals
/// share, and whether the dataset is long-form at all.
pub(super) fn display_predicates_by_name(
    dataset: &SourceDataset,
    analysis_id: AnalysisInstanceId,
    describe: &str,
) -> Result<(HashMap<String, Vec<QueryCoordinate>>, bool), VisualizationError> {
    let Some((signals, distinguishes_analyses)) = long_form_display_signals(dataset, analysis_id)
    else {
        return Ok((HashMap::new(), false));
    };
    let analysis = distinguishes_analyses.then(|| analysis_id.to_string());
    let mut by_name = HashMap::with_capacity(signals.len());
    for (index, name) in signals {
        let predicates = display_signal_predicates(index, &name, analysis.as_deref());
        if by_name.insert(name.clone(), predicates).is_some() {
            return Err(VisualizationError::InvalidValue {
                field: "trace.row-predicates",
                message: format!(
                    "the {describe} dataset has more than one display trace named {name:?}"
                ),
            });
        }
    }
    Ok((by_name, true))
}

/// How the traces of one analysis of a dataset name the signals they draw.
pub(super) enum SignalNames {
    /// A run reference: a trace names its signal, and the run says whether
    /// it has it when the trace is drawn.
    Run(AnalysisInstanceId),
    /// A long-form table: each display signal's exact predicates by name.
    LongForm(HashMap<String, Vec<QueryCoordinate>>),
    /// A wide table: one signal column per signal.
    Wide,
}

impl SignalNames {
    pub(super) fn of(
        dataset: &SourceDataset,
        analysis_id: AnalysisInstanceId,
        describe: &str,
    ) -> Result<Self, VisualizationError> {
        if dataset.run.is_some() {
            return Ok(Self::Run(analysis_id));
        }
        let (by_name, long_form) = display_predicates_by_name(dataset, analysis_id, describe)?;
        Ok(if long_form {
            Self::LongForm(by_name)
        } else {
            Self::Wide
        })
    }

    pub(super) const fn is_wide(&self) -> bool {
        matches!(self, Self::Wide)
    }

    /// The exact predicates that name the signal `name`, drawn as
    /// `component` where the source tells components apart; `None` when a
    /// table has no display signal by that name.
    pub(super) fn predicates(
        &self,
        name: &str,
        component: SignalComponent,
    ) -> Option<Vec<QueryCoordinate>> {
        match self {
            Self::Run(analysis) => Some(run_signal_predicates(*analysis, name, component)),
            Self::LongForm(by_name) => by_name.get(name).cloned(),
            Self::Wide => None,
        }
    }
}

/// The part of its signal a trace draws: the display values unless its
/// predicates name another.
/// What a trace moved onto another run reads there.
enum Reading {
    /// The predicates it holds, which name its signal there already.
    Kept,
    /// These predicates, and whether the run lacks the signal.
    Read(Vec<QueryCoordinate>, bool),
    /// The dataset names no signals: it reads as it read.
    Unread,
}

pub(super) fn trace_component(trace: &Trace) -> SignalComponent {
    trace
        .row_predicates
        .iter()
        .find_map(|coordinate| match &coordinate.value {
            TypedValue::Text(key) if coordinate.column == COMPONENT_COLUMN => {
                [SignalComponent::Real, SignalComponent::Imag]
                    .into_iter()
                    .find(|component| component.key() == key)
            }
            _ => None,
        })
        .unwrap_or(SignalComponent::Display)
}

/// Spectrum settings stand when their record, samples and measurement do.
fn validate_spectrum_settings(
    settings: &crate::fft::settings::SpectrumSettings,
) -> Result<(), VisualizationError> {
    settings
        .validate()
        .map_err(|message| VisualizationError::InvalidValue {
            field: "spectrum-settings",
            message,
        })
}

/// Coefficient settings stand when their reference resistance does.
fn validate_coefficient_settings(
    settings: &crate::harmonic_spectrum::settings::CoefficientSettings,
) -> Result<(), VisualizationError> {
    settings
        .validate()
        .map_err(|message| VisualizationError::InvalidValue {
            field: "coefficient-settings",
            message,
        })
}

/// A pin sits at a finite X: the reader clicked somewhere on the axis.
fn validate_readout_pin(pin: &ReadoutPin) -> Result<(), VisualizationError> {
    if pin.x.is_finite() {
        Ok(())
    } else {
        Err(VisualizationError::InvalidValue {
            field: "readout-pin.x",
            message: format!("a readout cannot be pinned at {}", pin.x),
        })
    }
}

/// A Smith marker names a locus and sits at a finite, positive frequency,
/// and the Bode card measures a response with a name.
fn validate_sheet_setting(setting: &SheetSetting) -> Result<(), VisualizationError> {
    let markers = match &setting.setting {
        SheetSettingValue::Smith { markers } => markers,
        SheetSettingValue::Bode { measured_signal } => {
            return validate_label("sheet-setting.measured-signal", measured_signal);
        }
        SheetSettingValue::NetworkMatrix { .. } | SheetSettingValue::PoleZero { .. } => {
            return Ok(());
        }
        SheetSettingValue::Histogram(reading) => return validate_histogram_reading(reading),
        SheetSettingValue::PhaseNoise { spots, band, .. } => {
            return validate_phase_noise_setting(spots, *band);
        }
        SheetSettingValue::NoiseContributors { pinned } => {
            return validate_pinned_contributors(pinned);
        }
    };
    for marker in markers {
        validate_label("sheet-setting.marker.trace", &marker.trace)?;
        if !(marker.frequency.is_finite() && marker.frequency > 0.0) {
            return Err(VisualizationError::InvalidValue {
                field: "sheet-setting.marker.frequency",
                message: format!("a marker cannot sit at {} Hz", marker.frequency),
            });
        }
    }
    Ok(())
}

/// A histogram names its measurement, bins into at least one and at most a
/// thousand bins over a finite, increasing range, and judges against finite
/// limits that leave room between them.
fn validate_histogram_reading(reading: &HistogramReading) -> Result<(), VisualizationError> {
    if let Some(measurement) = &reading.measurement {
        validate_label("sheet-setting.histogram.measurement", measurement)?;
    }
    if !(1..=1000).contains(&reading.bins) {
        return Err(VisualizationError::InvalidValue {
            field: "sheet-setting.histogram.bins",
            message: format!("a histogram cannot have {} bins", reading.bins),
        });
    }
    if let Some([low, high]) = reading.range
        && !(low.is_finite() && high.is_finite() && low < high)
    {
        return Err(VisualizationError::InvalidValue {
            field: "sheet-setting.histogram.range",
            message: format!("a histogram cannot bin {low} to {high}"),
        });
    }
    let finite = |limit: Option<f64>| limit.is_none_or(f64::is_finite);
    let ordered = match (reading.lower_limit, reading.upper_limit) {
        (Some(lower), Some(upper)) => lower < upper,
        _ => true,
    };
    if !(finite(reading.lower_limit) && finite(reading.upper_limit) && ordered) {
        return Err(VisualizationError::InvalidValue {
            field: "sheet-setting.histogram.limits",
            message: format!(
                "a histogram cannot judge against {:?} to {:?}",
                reading.lower_limit, reading.upper_limit
            ),
        });
    }
    Ok(())
}

/// Each pinned contributor names a device, and a mechanism when the run
/// gave one, in words a label can hold; none is pinned twice.
fn validate_pinned_contributors(
    pinned: &[crate::visualization_document::PinnedContributor],
) -> Result<(), VisualizationError> {
    let mut seen = std::collections::HashSet::with_capacity(pinned.len());
    for contributor in pinned {
        validate_label("sheet-setting.pinned.device", &contributor.device)?;
        if !contributor.mechanism.is_empty() {
            validate_label("sheet-setting.pinned.mechanism", &contributor.mechanism)?;
        }
        if !seen.insert(contributor) {
            return Err(VisualizationError::InvalidValue {
                field: "sheet-setting.pinned",
                message: format!(
                    "{} {} is pinned twice",
                    contributor.device, contributor.mechanism
                ),
            });
        }
    }
    Ok(())
}

/// Phase-noise spots are finite positive offsets, and the band runs from a
/// finite positive offset up to a higher one.
fn validate_phase_noise_setting(
    spots: &[f64],
    band: Option<[f64; 2]>,
) -> Result<(), VisualizationError> {
    if let Some(spot) = spots
        .iter()
        .find(|spot| !(spot.is_finite() && **spot > 0.0))
    {
        return Err(VisualizationError::InvalidValue {
            field: "sheet-setting.spots",
            message: format!("L(f) cannot be read at {spot} Hz"),
        });
    }
    if let Some([low, high]) = band
        && !(low.is_finite() && high.is_finite() && low > 0.0 && low < high)
    {
        return Err(VisualizationError::InvalidValue {
            field: "sheet-setting.band",
            message: format!("{low} Hz to {high} Hz is not a band of offsets"),
        });
    }
    Ok(())
}

fn duplicate_xy_plot(plot: &XyPlot) -> VisualizationError {
    VisualizationError::InvalidValue {
        field: "xy-plot.abscissa",
        message: format!(
            "pane {} plots its {:?} axis against {:?} twice",
            plot.pane_id.get(),
            plot.unit,
            plot.abscissa.text()
        ),
    }
}
