//! Applying an edit, and what an application is allowed to leave behind.
//!
//! A transaction validates every edit against the document it would produce
//! before any of it is kept, so a rejected edit changes nothing — there is no
//! partially-applied state. Removal cascades through the entities that depend
//! on the removed one and records a tombstone for each, which is why a page
//! can never keep a pane, axis, or trace whose parent is gone.

use super::*;

impl VisualizationDocument {
    fn validate_page_definition(page: &NewPage) -> Result<(), VisualizationError> {
        validate_label("page.title", &page.title)?;
        page.layout.validate()?;
        validate_key("page.template-id", &page.template_id)
    }

    pub(super) fn validate_pane_source(
        &self,
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
            self.dataset_for_binding(binding.dataset)?;
        }
        Ok(())
    }

    fn validate_pane_family_policy(
        &self,
        pane_id: PaneId,
        policy: &FamilyPresentationPolicy,
    ) -> Result<(), VisualizationError> {
        policy.validate()?;
        let pane = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .ok_or(VisualizationError::EntityNotFound(EntityRef::Pane(pane_id)))?;
        let mut bindings = Vec::<DatasetBinding>::new();
        if let Some(binding) = pane.binding.map(|binding| binding.dataset) {
            bindings.push(binding);
        }
        for binding in self
            .traces
            .iter()
            .filter(|trace| trace.pane_id == pane_id)
            .map(|trace| trace.binding)
        {
            if !bindings.contains(&binding) {
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
            let dataset = self.dataset_for_binding(binding)?;
            for dimension in std::iter::once(&policy.x_dimension.dimension)
                .chain(policy.family_dimensions.iter())
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
                if column.role != ColumnRole::Coordinate
                    || column.value_type != dimension.value_type
                {
                    return Err(VisualizationError::InvalidValue {
                        field: "pane.family-policy.dimension",
                        message: format!(
                            "source dimension '{}' must be a {:?} coordinate, received {:?} {:?}",
                            dimension.key, dimension.value_type, column.value_type, column.role
                        ),
                    });
                }
            }
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

    fn create_pane(&mut self, pane: NewPane) -> Result<PaneId, VisualizationError> {
        validate_label("pane.title", &pane.title)?;
        self.validate_pane_source(pane.kind, &pane.viewer_id, pane.binding)?;
        let order = self.placement_order(pane.page_id, pane.placement, None)?;
        for existing in &mut self.panes {
            if existing.page_id == pane.page_id && existing.order >= order {
                existing.order = existing
                    .order
                    .checked_add(1)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)?;
            }
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
        });
        Ok(id)
    }

    /// Materialize the canonical axes and display traces for a newly bound
    /// pane. A bound pane without these entities can be shown by the live
    /// Results renderer, but cannot participate in deterministic export,
    /// marker/measurement authoring, trace visibility, or link groups.
    pub(super) fn provision_bound_pane(
        &mut self,
        pane_id: PaneId,
        viewer_id: &str,
        binding: PaneDataBinding,
        created: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
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

        if self.traces.iter().any(|trace| trace.pane_id == pane_id) {
            return Ok(());
        }
        let (coordinate_key, display_signals) = {
            let dataset = self.dataset_for_binding(binding.dataset)?;
            let column_index =
                |key: &str| dataset.columns.iter().position(|column| column.key == key);
            let analysis_identity = binding.analysis_id.to_string();
            let analysis_id = column_index("analysis-id");
            if let (Some(trace_index), Some(trace_name), Some(component), Some(x), Some(y)) = (
                column_index("trace-index"),
                column_index("trace-name"),
                column_index("component"),
                column_index("x"),
                column_index("y"),
            ) && dataset.columns[trace_index].role == ColumnRole::Coordinate
                && dataset.columns[trace_name].role == ColumnRole::Coordinate
                && dataset.columns[component].role == ColumnRole::Coordinate
                && dataset.columns[x].role == ColumnRole::Coordinate
                && dataset.columns[y].role == ColumnRole::Signal
            {
                let mut signals = Vec::new();
                let mut seen_trace_indices = HashSet::new();
                for row in &dataset.rows {
                    if analysis_id.is_some_and(|analysis_id| {
                        !matches!(
                            row.values.get(analysis_id),
                            Some(TypedValue::Text(value)) if value == &analysis_identity
                        )
                    }) {
                        continue;
                    }
                    let (
                        TypedValue::Integer(index),
                        TypedValue::Text(name),
                        TypedValue::Text(component_name),
                    ) = (
                        &row.values[trace_index],
                        &row.values[trace_name],
                        &row.values[component],
                    )
                    else {
                        continue;
                    };
                    if component_name != "display" || !seen_trace_indices.insert(*index) {
                        continue;
                    }
                    let mut row_predicates = vec![
                        QueryCoordinate {
                            column: "trace-index".to_owned(),
                            value: TypedValue::Integer(*index),
                        },
                        QueryCoordinate {
                            column: "trace-name".to_owned(),
                            value: TypedValue::Text(name.clone()),
                        },
                        QueryCoordinate {
                            column: "component".to_owned(),
                            value: TypedValue::Text("display".to_owned()),
                        },
                    ];
                    if analysis_id.is_some() {
                        row_predicates.push(QueryCoordinate {
                            column: "analysis-id".to_owned(),
                            value: TypedValue::Text(analysis_identity.clone()),
                        });
                    }
                    signals.push(("y".to_owned(), name.clone(), row_predicates));
                }
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
            for group in &mut self.link_groups {
                let compatible = matches!(
                    (group.kind, orientation),
                    (LinkKind::HorizontalViewport, AxisOrientation::Horizontal)
                        | (
                            LinkKind::VerticalViewport,
                            AxisOrientation::VerticalLeft | AxisOrientation::VerticalRight
                        )
                );
                if compatible
                    && group.members.contains(&EntityRef::Axis(anchor_axis))
                    && !group.members.contains(&EntityRef::Axis(new_axis))
                {
                    if group.members.len() >= MAX_ENTITY_REFERENCES {
                        return Err(VisualizationError::InvalidValue {
                            field: "link-group.members",
                            message: format!(
                                "a link group supports at most {MAX_ENTITY_REFERENCES} members"
                            ),
                        });
                    }
                    group.members.push(EntityRef::Axis(new_axis));
                }
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
                .iter_mut()
                .find(|pane| pane.id == pane_id)
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
        for pane in &mut self.panes {
            if pane.id != pane_id && pane.page_id == page_id && pane.order >= order {
                pane.order = pane
                    .order
                    .checked_add(1)
                    .ok_or(VisualizationError::IdentitySpaceExhausted)?;
            }
        }
        let pane = self.pane_mut(pane_id)?;
        pane.page_id = page_id;
        pane.placement = placement;
        pane.order = order;
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
                })?;
                created.push(EntityRef::Pane(id));
                Ok(())
            }
            DocumentEdit::AddBoundPane(pane) => {
                let viewer_id = pane.viewer_id.clone();
                let binding = pane.binding;
                let placement = pane.placement;
                let id = self.create_pane(pane)?;
                created.push(EntityRef::Pane(id));
                if let Some(binding) = binding {
                    self.provision_bound_pane(id, &viewer_id, binding, created)?;
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
                })?;
                created.push(EntityRef::Pane(pane_id));
                if let Some(binding) = binding {
                    self.provision_bound_pane(pane_id, &viewer_id, binding, created)?;
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
                self.validate_pane_source(kind, &viewer_id, binding)?;
                let pane = self.pane_mut(pane_id)?;
                pane.viewer_id = viewer_id.clone();
                pane.binding = binding;
                if let Some(binding) = binding {
                    self.provision_bound_pane(pane_id, &viewer_id, binding, created)?;
                }
                Ok(())
            }
            DocumentEdit::SetPaneFamilyPresentation { pane_id, policy } => {
                if let Some(policy) = &policy {
                    self.validate_pane_family_policy(pane_id, policy)?;
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
                    self.remove(EntityRef::Page(source_page_id), revision, tombstoned)?;
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
                if !trace_contains_exact_coordinate(self, trace_id, &coordinate)? {
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
                if !trace_contains_exact_coordinate(self, trace_id, &coordinate)? {
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
                if trace_ids.is_empty() || trace_ids.len() > MAX_ENTITY_REFERENCES {
                    return Err(VisualizationError::InvalidValue {
                        field: "measurement.traces",
                        message: format!(
                            "a measurement requires 1 to {MAX_ENTITY_REFERENCES} traces"
                        ),
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
                validate_annotation(self, pane_id, &anchor, &text)?;
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
                validate_link_members(self, kind, &members)?;
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
            DocumentEdit::SetTraceVisibility { trace_id, visible } => {
                self.trace_mut(trace_id)?.visible = visible;
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
                if !trace_contains_exact_coordinate(self, trace_id, &coordinate)? {
                    return Err(VisualizationError::InterpolationRequired);
                }
                self.marker_mut(marker_id)?.coordinate = coordinate;
                Ok(())
            }
            DocumentEdit::AddScalarMeasurement {
                pane_id,
                trace_ids,
                expression,
                value,
            } => {
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
                if trace_ids.is_empty() || trace_ids.len() > MAX_ENTITY_REFERENCES {
                    return Err(VisualizationError::InvalidValue {
                        field: "measurement.traces",
                        message: format!(
                            "a measurement requires 1 to {MAX_ENTITY_REFERENCES} traces"
                        ),
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
                    kind: MeasurementKind::Point,
                    label: format!("Measurement {}", id.get()),
                    expression: Some(expression),
                    value: Some(value),
                });
                created.push(EntityRef::Measurement(id));
                Ok(())
            }
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
                if !trace_contains_exact_coordinate(self, trace_id, &coordinate)? {
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
                validate_annotation(self, pane_id, &anchor, &text)?;
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
                validate_link_members(self, kind, &members)?;
                self.link_group_mut(link_group_id)?.members = members;
                Ok(())
            }
            DocumentEdit::ClearMarkers { pane_id } => {
                if let Some(pane_id) = pane_id {
                    self.require_pane(pane_id)?;
                }
                let marker_ids = self
                    .markers
                    .iter()
                    .filter(|marker| pane_id.is_none_or(|pane_id| marker.pane_id == pane_id))
                    .map(|marker| marker.id)
                    .collect::<Vec<_>>();
                for marker_id in marker_ids {
                    self.remove(EntityRef::Marker(marker_id), revision, tombstoned)?;
                }
                Ok(())
            }
            DocumentEdit::Remove(entity) => self.remove(entity, revision, tombstoned),
            DocumentEdit::RecordComparison(receipt) => {
                self.validate_comparison_receipt(&receipt)?;
                self.comparisons.push(receipt);
                Ok(())
            }
        }
    }

    fn attach_dataset(&mut self, dataset: SourceDataset) -> Result<(), VisualizationError> {
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

    fn merge_dataset_projection(
        &mut self,
        projection: SourceDataset,
    ) -> Result<(), VisualizationError> {
        projection.validate()?;
        let existing = self
            .datasets
            .iter_mut()
            .find(|dataset| dataset.binding.dataset_id == projection.binding.dataset_id)
            .ok_or(VisualizationError::DatasetNotFound(
                projection.binding.dataset_id,
            ))?;
        if existing.binding.content_digest != projection.binding.content_digest {
            return Err(VisualizationError::SourceDigestMismatch {
                dataset_id: projection.binding.dataset_id,
                bound: existing.binding.content_digest,
                requested: projection.binding.content_digest,
            });
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
        *existing = merged;
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

        let long_form_trace_predicates = {
            let dataset = self.dataset_for_binding(next_binding)?;
            let find = |key: &str| dataset.columns.iter().position(|column| column.key == key);
            let analysis_identity = analysis_id.to_string();
            let analysis_identity_column = find("analysis-id");
            match (
                find("trace-index"),
                find("trace-name"),
                find("component"),
                find("x"),
                find("y"),
            ) {
                (Some(trace_index), Some(trace_name), Some(component), Some(_), Some(_)) => {
                    let mut by_name = HashMap::<String, Vec<QueryCoordinate>>::new();
                    let mut seen_indices = HashSet::new();
                    for row in &dataset.rows {
                        if analysis_identity_column.is_some_and(|analysis_id| {
                            !matches!(
                                row.values.get(analysis_id),
                                Some(TypedValue::Text(value)) if value == &analysis_identity
                            )
                        }) {
                            continue;
                        }
                        let (
                            TypedValue::Integer(index),
                            TypedValue::Text(name),
                            TypedValue::Text(component_name),
                        ) = (
                            &row.values[trace_index],
                            &row.values[trace_name],
                            &row.values[component],
                        )
                        else {
                            continue;
                        };
                        if component_name != "display" || !seen_indices.insert(*index) {
                            continue;
                        }
                        if by_name.contains_key(name) {
                            return Err(VisualizationError::InvalidValue {
                                field: "trace.row-predicates",
                                message: format!(
                                    "the newer tracked dataset has more than one display trace named {name:?}"
                                ),
                            });
                        }
                        let mut row_predicates = vec![
                            QueryCoordinate {
                                column: "trace-index".to_owned(),
                                value: TypedValue::Integer(*index),
                            },
                            QueryCoordinate {
                                column: "trace-name".to_owned(),
                                value: TypedValue::Text(name.clone()),
                            },
                            QueryCoordinate {
                                column: "component".to_owned(),
                                value: TypedValue::Text("display".to_owned()),
                            },
                        ];
                        if analysis_identity_column.is_some() {
                            row_predicates.push(QueryCoordinate {
                                column: "analysis-id".to_owned(),
                                value: TypedValue::Text(analysis_identity.clone()),
                            });
                        }
                        by_name.insert(name.clone(), row_predicates);
                    }
                    Some(by_name)
                }
                _ => None,
            }
        };

        let mut rebound = 0usize;
        for pane in &mut self.panes {
            if let Some(binding) = pane.binding.as_mut()
                && binding.dataset == previous
            {
                binding.dataset = next_binding;
                binding.analysis_id = analysis_id;
                rebound = rebound.saturating_add(1);
            }
        }
        for trace in &mut self.traces {
            if trace.binding == previous {
                if let Some(predicates) = &long_form_trace_predicates {
                    trace.row_predicates = predicates.get(&trace.label).cloned().ok_or_else(
                        || VisualizationError::InvalidValue {
                            field: "trace.row-predicates",
                            message: format!(
                                "the newer tracked dataset has no exact display trace named {:?}",
                                trace.label
                            ),
                        },
                    )?;
                    trace.signal_key = "y".to_owned();
                    trace.coordinate_key = "x".to_owned();
                }
                trace.binding = next_binding;
                rebound = rebound.saturating_add(1);
            }
        }
        if rebound == 0 {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.datasets",
                message: format!(
                    "dataset {} is attached but no pane or trace is bound to it",
                    previous.dataset_id
                ),
            });
        }
        Ok(())
    }

    fn add_trace(
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
        let signal = find_column(dataset, &trace.signal_key)?;
        let coordinate = find_column(dataset, &trace.coordinate_key)?;
        if signal.role != ColumnRole::Signal || coordinate.role != ColumnRole::Coordinate {
            return Err(VisualizationError::InvalidValue {
                field: "trace.source-columns",
                message: "trace requires one signal column and one coordinate column".to_owned(),
            });
        }
        validate_trace_row_predicates(dataset, &trace.row_predicates)?;
        let id = TraceId::allocate(self.allocate_serial()?)?;
        self.traces.push(Trace {
            id,
            pane_id: trace.pane_id,
            binding: trace.binding,
            signal_key: trace.signal_key,
            coordinate_key: trace.coordinate_key,
            row_predicates: trace.row_predicates,
            x_axis_id: trace.x_axis_id,
            y_axis_id: trace.y_axis_id,
            label: trace.label,
            visible: true,
        });
        created.push(EntityRef::Trace(id));
        Ok(())
    }

    fn rename(&mut self, entity: EntityRef, value: String) -> Result<(), VisualizationError> {
        validate_label("entity.label", &value)?;
        match entity {
            EntityRef::Page(id) => self.page_mut(id)?.title = value,
            EntityRef::Pane(id) => self.pane_mut(id)?.title = value,
            EntityRef::Trace(id) => self.trace_mut(id)?.label = value,
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

    fn remove(
        &mut self,
        entity: EntityRef,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) -> Result<(), VisualizationError> {
        if let Some(group) = self
            .link_groups
            .iter()
            .find(|group| group.members.contains(&entity))
        {
            return Err(VisualizationError::EntityInUse {
                entity,
                dependent: EntityRef::LinkGroup(group.id),
            });
        }
        match entity {
            EntityRef::Page(id) => {
                self.require_page(id)?;
                let pane_ids: Vec<_> = self
                    .panes
                    .iter()
                    .filter_map(|pane| (pane.page_id == id).then_some(pane.id))
                    .collect();
                for pane_id in pane_ids {
                    self.remove_pane_cascade(pane_id, revision, tombstoned);
                }
                self.pages.retain(|page| page.id != id);
            }
            EntityRef::Pane(id) => {
                self.require_pane(id)?;
                self.remove_pane_cascade(id, revision, tombstoned);
                return Ok(());
            }
            EntityRef::Axis(id) => {
                self.require_entity(entity)?;
                if let Some(trace) = self
                    .traces
                    .iter()
                    .find(|trace| trace.x_axis_id == id || trace.y_axis_id == id)
                {
                    return Err(VisualizationError::EntityInUse {
                        entity,
                        dependent: EntityRef::Trace(trace.id),
                    });
                }
                if let Some(cursor) = self.cursors.iter().find(|cursor| cursor.axis_id == id) {
                    return Err(VisualizationError::EntityInUse {
                        entity,
                        dependent: EntityRef::Cursor(cursor.id),
                    });
                }
                self.axes.retain(|axis| axis.id != id);
            }
            EntityRef::Trace(id) => {
                self.require_entity(entity)?;
                let dependent = self
                    .markers
                    .iter()
                    .find(|marker| marker.trace_id == id)
                    .map(|marker| EntityRef::Marker(marker.id))
                    .or_else(|| {
                        self.measurements
                            .iter()
                            .find(|measurement| measurement.trace_ids.contains(&id))
                            .map(|measurement| EntityRef::Measurement(measurement.id))
                    })
                    .or_else(|| {
                        self.annotations
                            .iter()
                            .find_map(|annotation| match &annotation.anchor {
                                AnnotationAnchor::Trace { trace_id, .. } if *trace_id == id => {
                                    Some(EntityRef::Annotation(annotation.id))
                                }
                                _ => None,
                            })
                    });
                if let Some(dependent) = dependent {
                    return Err(VisualizationError::EntityInUse { entity, dependent });
                }
                self.traces.retain(|trace| trace.id != id);
            }
            EntityRef::Cursor(id) => {
                self.require_entity(entity)?;
                self.cursors.retain(|cursor| cursor.id != id);
            }
            EntityRef::Marker(id) => {
                self.require_entity(entity)?;
                self.markers.retain(|marker| marker.id != id);
            }
            EntityRef::Measurement(id) => {
                self.require_entity(entity)?;
                self.measurements.retain(|measurement| measurement.id != id);
            }
            EntityRef::Annotation(id) => {
                self.require_entity(entity)?;
                self.annotations.retain(|annotation| annotation.id != id);
            }
            EntityRef::LinkGroup(id) => {
                self.require_entity(entity)?;
                self.link_groups.retain(|group| group.id != id);
            }
        }
        self.record_tombstone(entity, revision, tombstoned);
        Ok(())
    }

    fn remove_pane_cascade(
        &mut self,
        pane_id: PaneId,
        revision: ObjectRevision,
        tombstoned: &mut Vec<EntityRef>,
    ) {
        let mut removed = Vec::new();
        removed.extend(
            self.axes
                .iter()
                .filter_map(|axis| (axis.pane_id == pane_id).then_some(EntityRef::Axis(axis.id))),
        );
        removed.extend(
            self.traces.iter().filter_map(|trace| {
                (trace.pane_id == pane_id).then_some(EntityRef::Trace(trace.id))
            }),
        );
        removed.extend(self.cursors.iter().filter_map(|cursor| {
            (cursor.pane_id == pane_id).then_some(EntityRef::Cursor(cursor.id))
        }));
        removed.extend(self.markers.iter().filter_map(|marker| {
            (marker.pane_id == pane_id).then_some(EntityRef::Marker(marker.id))
        }));
        removed.extend(self.measurements.iter().filter_map(|measurement| {
            (measurement.pane_id == pane_id).then_some(EntityRef::Measurement(measurement.id))
        }));
        removed.extend(self.annotations.iter().filter_map(|annotation| {
            (annotation.pane_id == pane_id).then_some(EntityRef::Annotation(annotation.id))
        }));
        self.axes.retain(|axis| axis.pane_id != pane_id);
        self.traces.retain(|trace| trace.pane_id != pane_id);
        self.cursors.retain(|cursor| cursor.pane_id != pane_id);
        self.markers.retain(|marker| marker.pane_id != pane_id);
        self.measurements
            .retain(|measurement| measurement.pane_id != pane_id);
        self.annotations
            .retain(|annotation| annotation.pane_id != pane_id);
        let page_id = self
            .panes
            .iter()
            .find(|pane| pane.id == pane_id)
            .map(|pane| pane.page_id);
        self.panes.retain(|pane| pane.id != pane_id);
        if let Some(page_id) = page_id {
            self.normalize_page_pane_layout(page_id, None);
        }
        for entity in removed {
            self.record_tombstone(entity, revision, tombstoned);
        }
        self.record_tombstone(EntityRef::Pane(pane_id), revision, tombstoned);
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
        receipt: &ComparisonReceipt,
    ) -> Result<(), VisualizationError> {
        self.dataset_for_binding(receipt.baseline)?;
        self.dataset_for_binding(receipt.candidate)?;
        receipt.validate_structure()
    }

    pub(super) fn validate(&self) -> Result<(), VisualizationError> {
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
        ensure_maximum_len(
            "visualization-document.pages",
            self.pages.len(),
            MAX_VISUALIZATION_PAGES,
        )?;
        ensure_maximum_len(
            "visualization-document.panes",
            self.panes.len(),
            MAX_VISUALIZATION_PANES,
        )?;
        ensure_maximum_len(
            "visualization-document.axes",
            self.axes.len(),
            MAX_VISUALIZATION_AXES,
        )?;
        ensure_maximum_len(
            "visualization-document.traces",
            self.traces.len(),
            MAX_VISUALIZATION_TRACES,
        )?;
        ensure_maximum_len(
            "visualization-document.cursors",
            self.cursors.len(),
            MAX_VISUALIZATION_CURSORS,
        )?;
        ensure_maximum_len(
            "visualization-document.markers",
            self.markers.len(),
            MAX_VISUALIZATION_MARKERS,
        )?;
        ensure_maximum_len(
            "visualization-document.measurements",
            self.measurements.len(),
            MAX_VISUALIZATION_MEASUREMENTS,
        )?;
        ensure_maximum_len(
            "visualization-document.annotations",
            self.annotations.len(),
            MAX_VISUALIZATION_ANNOTATIONS,
        )?;
        ensure_maximum_len(
            "visualization-document.link-groups",
            self.link_groups.len(),
            MAX_VISUALIZATION_LINK_GROUPS,
        )?;
        ensure_maximum_len(
            "visualization-document.tombstones",
            self.tombstones.len(),
            MAX_VISUALIZATION_TOMBSTONES,
        )?;
        ensure_maximum_len(
            "visualization-document.comparisons",
            self.comparisons.len(),
            MAX_VISUALIZATION_COMPARISONS,
        )?;
        validate_aggregate_nested_resources(
            "visualization-document.measurement-trace-references",
            &self.measurements,
            MAX_VISUALIZATION_MEASUREMENT_TRACE_REFERENCES_TOTAL,
        )?;
        validate_aggregate_nested_resources(
            "visualization-document.link-member-references",
            &self.link_groups,
            MAX_VISUALIZATION_LINK_MEMBER_REFERENCES_TOTAL,
        )?;
        validate_aggregate_nested_resources(
            "visualization-document.comparison-signals",
            &self.comparisons,
            MAX_VISUALIZATION_COMPARISON_SIGNALS_TOTAL,
        )?;
        if self.pages.is_empty() {
            return Err(VisualizationError::InvalidValue {
                field: "visualization-document.pages",
                message: "at least one page is required".to_owned(),
            });
        }
        let mut identities = HashSet::new();
        for page in &self.pages {
            validate_label("page.title", &page.title)?;
            page.layout.validate()?;
            validate_key("page.template-id", &page.template_id)?;
            if page.layout == PageLayout::SinglePane
                && self
                    .panes
                    .iter()
                    .filter(|pane| pane.page_id == page.id)
                    .count()
                    > 1
            {
                return Err(VisualizationError::InvalidValue {
                    field: "page.layout",
                    message: "single-pane layout cannot contain more than one pane".to_owned(),
                });
            }
            ensure_identity(&mut identities, EntityRef::Page(page.id))?;
        }
        for pane in &self.panes {
            validate_label("pane.title", &pane.title)?;
            self.require_page(pane.page_id)?;
            self.validate_pane_source(pane.kind, &pane.viewer_id, pane.binding)?;
            if let Some(policy) = &pane.family_policy {
                self.validate_pane_family_policy(pane.id, policy)?;
            }
            ensure_identity(&mut identities, EntityRef::Pane(pane.id))?;
        }
        for page in &self.pages {
            let mut panes: Vec<_> = self
                .panes
                .iter()
                .filter(|pane| pane.page_id == page.id)
                .collect();
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
                            message:
                                "pane placement must reference an earlier pane on the same page"
                                    .to_owned(),
                        });
                    }
                }
                preceding.insert(pane.id);
            }
        }
        for axis in &self.axes {
            validate_label("axis.label", &axis.label)?;
            validate_optional_unit("axis.unit", axis.unit.as_deref())?;
            self.require_pane(axis.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Axis(axis.id))?;
        }
        for trace in &self.traces {
            validate_label("trace.label", &trace.label)?;
            validate_key("trace.signal-key", &trace.signal_key)?;
            validate_key("trace.coordinate-key", &trace.coordinate_key)?;
            self.require_pane(trace.pane_id)?;
            self.require_axis_in_pane(trace.x_axis_id, trace.pane_id)?;
            self.require_axis_in_pane(trace.y_axis_id, trace.pane_id)?;
            let dataset = self.dataset_for_binding(trace.binding)?;
            if find_column(dataset, &trace.signal_key)?.role != ColumnRole::Signal
                || find_column(dataset, &trace.coordinate_key)?.role != ColumnRole::Coordinate
            {
                return Err(VisualizationError::InvalidValue {
                    field: "trace.source-columns",
                    message: "invalid signal or coordinate column role".to_owned(),
                });
            }
            validate_trace_row_predicates(dataset, &trace.row_predicates)?;
            ensure_identity(&mut identities, EntityRef::Trace(trace.id))?;
        }
        for cursor in &self.cursors {
            validate_label("cursor.label", &cursor.label)?;
            cursor.position.validate("cursor.position")?;
            self.require_axis_in_pane(cursor.axis_id, cursor.pane_id)?;
            ensure_identity(&mut identities, EntityRef::Cursor(cursor.id))?;
        }
        for marker in &self.markers {
            validate_label("marker.label", &marker.label)?;
            if let Some(source) = marker.source_specification.as_deref() {
                validate_label("marker.source-specification", source)?;
            }
            marker.coordinate.validate("marker.coordinate")?;
            self.require_trace_in_pane(marker.trace_id, marker.pane_id)?;
            if !trace_contains_exact_coordinate(self, marker.trace_id, &marker.coordinate)? {
                return Err(VisualizationError::InterpolationRequired);
            }
            ensure_identity(&mut identities, EntityRef::Marker(marker.id))?;
        }
        for measurement in &self.measurements {
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
            if measurement.trace_ids.is_empty()
                || measurement.trace_ids.len() > MAX_ENTITY_REFERENCES
            {
                return Err(VisualizationError::InvalidValue {
                    field: "measurement.traces",
                    message: format!("a measurement requires 1 to {MAX_ENTITY_REFERENCES} traces"),
                });
            }
            for trace in &measurement.trace_ids {
                self.require_trace_in_pane(*trace, measurement.pane_id)?;
            }
            ensure_identity(&mut identities, EntityRef::Measurement(measurement.id))?;
        }
        for annotation in &self.annotations {
            validate_annotation(
                self,
                annotation.pane_id,
                &annotation.anchor,
                &annotation.text,
            )?;
            ensure_identity(&mut identities, EntityRef::Annotation(annotation.id))?;
        }
        for group in &self.link_groups {
            validate_label("link-group.label", &group.label)?;
            validate_link_members(self, group.kind, &group.members)?;
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
            self.validate_comparison_receipt(receipt)?;
        }
        Ok(())
    }
}
