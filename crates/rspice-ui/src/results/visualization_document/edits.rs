//! The edit vocabulary: every change a visualization document can be asked to make.
//!
//! An edit names what it adds, changes, or removes and nothing about how it is
//! applied, so the whole set of legal document mutations is readable in one
//! place. Removal is a tombstone rather than a deletion, which is what lets a
//! document's history stay complete after an entity is taken off a page.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tombstone {
    pub entity: EntityRef,
    pub deleted_at_revision: ObjectRevision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewAxis {
    pub pane_id: PaneId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
    pub orientation: AxisOrientation,
    pub scale: AxisScale,
    #[serde(deserialize_with = "deserialize_unit_string")]
    pub unit: Option<String>,
    pub range: Option<AxisRange>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewTrace {
    pub pane_id: PaneId,
    pub binding: DatasetBinding,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub signal_key: String,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub coordinate_key: String,
    pub x_axis_id: AxisId,
    pub y_axis_id: AxisId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewPage {
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub layout: PageLayout,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub template_id: String,
    pub update_policy: PageUpdatePolicy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewPane {
    pub page_id: PageId,
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub kind: PaneKind,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub viewer_id: String,
    pub binding: Option<PaneDataBinding>,
    pub placement: PanePlacement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewPagePane {
    #[serde(deserialize_with = "deserialize_label_string")]
    pub title: String,
    pub kind: PaneKind,
    #[serde(deserialize_with = "deserialize_key_string")]
    pub viewer_id: String,
    pub binding: Option<PaneDataBinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DocumentEdit {
    SetTracking(ResultDocumentTracking),
    AttachDataset(SourceDataset),
    /// Attach a newer immutable source snapshot and move presentation
    /// entities that currently consume `previous` to it. The prior snapshot
    /// remains attached so comparison receipts and review evidence stay
    /// reproducible.
    RetargetTrackedDataset {
        previous: DatasetBinding,
        next: SourceDataset,
        analysis_id: AnalysisInstanceId,
    },
    AddPage {
        title: String,
    },
    AddComposedPage(NewPage),
    AddPane {
        page_id: PageId,
        title: String,
        kind: PaneKind,
    },
    AddBoundPane(NewPane),
    AddPaneOnNewPage {
        page: NewPage,
        pane: NewPagePane,
    },
    SetPageComposition {
        page_id: PageId,
        layout: PageLayout,
        template_id: String,
        update_policy: PageUpdatePolicy,
    },
    SetPaneSource {
        pane_id: PaneId,
        viewer_id: String,
        binding: Option<PaneDataBinding>,
    },
    SetPaneFamilyPresentation {
        pane_id: PaneId,
        policy: Option<FamilyPresentationPolicy>,
    },
    PlacePane {
        pane_id: PaneId,
        page_id: PageId,
        placement: PanePlacement,
    },
    AddAxis(NewAxis),
    AddTrace(NewTrace),
    AddCursor {
        pane_id: PaneId,
        axis_id: AxisId,
        position: TypedValue,
        label: String,
    },
    AddMarker {
        pane_id: PaneId,
        trace_id: TraceId,
        coordinate: TypedValue,
        label: String,
    },
    AddTypedMarker {
        pane_id: PaneId,
        trace_id: TraceId,
        coordinate: TypedValue,
        label: String,
        kind: PlotMarkerKind,
        scope: PlotMarkerScope,
        source_specification: Option<String>,
    },
    AddMeasurement {
        pane_id: PaneId,
        trace_ids: Vec<TraceId>,
        kind: MeasurementKind,
        label: String,
    },
    AddAnnotation {
        pane_id: PaneId,
        anchor: AnnotationAnchor,
        text: String,
    },
    AddLinkGroup {
        label: String,
        kind: LinkKind,
        members: Vec<EntityRef>,
    },
    Rename {
        entity: EntityRef,
        value: String,
    },
    SetAxisRange {
        axis_id: AxisId,
        range: Option<AxisRange>,
    },
    SetTraceVisibility {
        trace_id: TraceId,
        visible: bool,
    },
    MoveCursor {
        cursor_id: CursorId,
        position: TypedValue,
    },
    MoveMarker {
        marker_id: MarkerId,
        coordinate: TypedValue,
    },
    SetMarker {
        marker_id: MarkerId,
        coordinate: TypedValue,
        label: String,
        kind: PlotMarkerKind,
        scope: PlotMarkerScope,
        source_specification: Option<String>,
    },
    SetAnnotation {
        annotation_id: AnnotationId,
        anchor: AnnotationAnchor,
        text: String,
    },
    SetLinkMembers {
        link_group_id: LinkGroupId,
        members: Vec<EntityRef>,
    },
    Remove(EntityRef),
    RecordComparison(ComparisonReceipt),
}
