//! Session state shared by document engines.
//!
//! Top-level navigation, responsive layout, docks, and drawers belong to
//! [`crate::workbench::WorkbenchState`]. This module contains only cross-frame document
//! interaction state and durable presentation preferences.

use serde::{Deserialize, Serialize};

use crate::state::{GridStyle, SchematicVisibilityPolicy, SchematicWireRoutingStyle};
use crate::ui::Theme;
use crate::ui::widgets::Toasts;

/// Object family searched by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkObjectKind {
    #[default]
    InstancesAndParameters,
    NetsAndLabels,
    PortsAndPins,
    GraphicsAndNotes,
}

impl SelectionBulkObjectKind {
    pub const ALL: [Self; 4] = [
        Self::InstancesAndParameters,
        Self::NetsAndLabels,
        Self::PortsAndPins,
        Self::GraphicsAndNotes,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::InstancesAndParameters => "Instances + parameters",
            Self::NetsAndLabels => "Nets and labels",
            Self::PortsAndPins => "Ports and pins",
            Self::GraphicsAndNotes => "Graphics and notes",
        }
    }
}

/// Project extent inspected by the schematic bulk-edit workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SelectionBulkHierarchyScope {
    #[default]
    ActiveHierarchyPath,
    CurrentSheet,
    CompleteProject,
}

impl SelectionBulkHierarchyScope {
    pub const ALL: [Self; 3] = [
        Self::ActiveHierarchyPath,
        Self::CurrentSheet,
        Self::CompleteProject,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ActiveHierarchyPath => "Active hierarchy path",
            Self::CurrentSheet => "Current sheet",
            Self::CompleteProject => "Complete project",
        }
    }
}

/// Device-local, reusable query used by the mockup-owned bulk editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SelectionBulkFilter {
    pub query: String,
    pub object_kind: SelectionBulkObjectKind,
    pub hierarchy_scope: SelectionBulkHierarchyScope,
    pub model_cell: String,
    pub current_property: String,
}

impl Default for SelectionBulkFilter {
    fn default() -> Self {
        Self {
            query: String::new(),
            object_kind: SelectionBulkObjectKind::InstancesAndParameters,
            hierarchy_scope: SelectionBulkHierarchyScope::ActiveHierarchyPath,
            model_cell: String::new(),
            current_property: String::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SavedSelectionBulkFilter {
    pub name: String,
    pub filter: SelectionBulkFilter,
}

/// Persisted active query and named filters. This is user/session state, never
/// project-owned design data.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SelectionBulkFilterSession {
    pub active: SelectionBulkFilter,
    saved: Vec<SavedSelectionBulkFilter>,
}

impl SelectionBulkFilterSession {
    const MAX_SAVED_FILTERS: usize = 32;
    const MAX_FILTER_NAME_BYTES: usize = 96;

    pub fn saved(&self) -> &[SavedSelectionBulkFilter] {
        &self.saved
    }

    pub fn save_active(&mut self, name: &str) -> Result<(), String> {
        let name = name.trim();
        if name.is_empty() {
            return Err("A saved filter name is required.".to_owned());
        }
        if name.len() > Self::MAX_FILTER_NAME_BYTES || name.chars().any(char::is_control) {
            return Err("The saved filter name is invalid or too long.".to_owned());
        }
        if let Some(existing) = self
            .saved
            .iter_mut()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
        {
            existing.name = name.to_owned();
            existing.filter = self.active.clone();
            return Ok(());
        }
        if self.saved.len() >= Self::MAX_SAVED_FILTERS {
            return Err(format!(
                "At most {} selection filters may be saved.",
                Self::MAX_SAVED_FILTERS
            ));
        }
        self.saved.push(SavedSelectionBulkFilter {
            name: name.to_owned(),
            filter: self.active.clone(),
        });
        self.saved
            .sort_by(|left, right| left.name.to_lowercase().cmp(&right.name.to_lowercase()));
        Ok(())
    }

    pub fn load(&mut self, name: &str) -> bool {
        let Some(filter) = self
            .saved
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .map(|entry| entry.filter.clone())
        else {
            return false;
        };
        self.active = filter;
        true
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.saved.len();
        self.saved
            .retain(|entry| !entry.name.eq_ignore_ascii_case(name));
        self.saved.len() != before
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolTool {
    #[default]
    Select,
    PlacePin,
    Line,
    Rectangle,
    Circle,
    Arc,
    Polygon,
    Text,
}

impl SymbolTool {
    pub fn label(self) -> &'static str {
        match self {
            SymbolTool::Select => "Select",
            SymbolTool::PlacePin => "Place pin",
            SymbolTool::Line => "Line",
            SymbolTool::Rectangle => "Rectangle",
            SymbolTool::Circle => "Circle",
            SymbolTool::Arc => "Arc",
            SymbolTool::Polygon => "Polygon",
            SymbolTool::Text => "Text",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolGridSpacing {
    Ten,
    Five,
    #[default]
    TwoPointFive,
}

impl SymbolGridSpacing {
    pub const ALL: [Self; 3] = [Self::Ten, Self::Five, Self::TwoPointFive];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ten => "grid 10",
            Self::Five => "grid 5",
            Self::TwoPointFive => "grid 2.5",
        }
    }

    /// The persistent symbol model uses integer quarter-grid coordinates.
    /// The fine 2.5 display grid therefore maps to one two-unit/two-and-a-half
    /// authored pitch with deterministic nearest-integer placement.
    pub const fn model_step(self) -> f32 {
        match self {
            Self::Ten => 10.0,
            Self::Five => 5.0,
            Self::TwoPointFive => 2.5,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolSelection {
    pub pins: std::collections::BTreeSet<String>,
    pub shapes: std::collections::BTreeSet<usize>,
    pub attributes: std::collections::BTreeSet<crate::state::SymbolAttributeKind>,
    pub texts: std::collections::BTreeSet<u64>,
}

impl SymbolSelection {
    pub fn all_in(document: &crate::state::SymbolDocument) -> Self {
        Self {
            pins: document.pins.iter().map(|pin| pin.name.clone()).collect(),
            shapes: (0..document.body.len()).collect(),
            attributes: crate::state::SymbolAttributeKind::ALL.into_iter().collect(),
            texts: std::collections::BTreeSet::new(),
        }
    }

    pub fn in_rect(
        document: &crate::state::SymbolDocument,
        metadata: &crate::state::SymbolEditorMetadata,
        start: crate::state::Point,
        end: crate::state::Point,
    ) -> Self {
        let min = crate::state::Point::new(start.x.min(end.x), start.y.min(end.y));
        let max = crate::state::Point::new(start.x.max(end.x), start.y.max(end.y));
        let pins = document
            .pins
            .iter()
            .filter(|pin| {
                pin.position
                    .is_some_and(|point| point_in_bounds(point, min, max))
            })
            .map(|pin| pin.name.clone())
            .collect();
        let shapes = document
            .body
            .iter()
            .enumerate()
            .filter_map(|(index, shape)| {
                let (shape_min, shape_max) = symbol_shape_bounds(shape);
                bounds_intersect(min, max, shape_min, shape_max).then_some(index)
            })
            .collect();
        // Hidden labels are not on the canvas, so a canvas marquee cannot
        // take them.
        let attributes = metadata
            .attributes
            .iter()
            .filter(|attribute| attribute.shown && point_in_bounds(attribute.position, min, max))
            .map(|attribute| attribute.kind)
            .collect();
        let texts = metadata
            .texts
            .iter()
            .filter(|text| text.shown && point_in_bounds(text.position, min, max))
            .map(|text| text.id)
            .collect();
        Self {
            pins,
            shapes,
            attributes,
            texts,
        }
    }

    pub fn single_pin(name: impl Into<String>) -> Self {
        Self {
            pins: [name.into()].into_iter().collect(),
            shapes: std::collections::BTreeSet::new(),
            attributes: std::collections::BTreeSet::new(),
            texts: std::collections::BTreeSet::new(),
        }
    }

    pub fn single_shape(index: usize) -> Self {
        Self {
            pins: std::collections::BTreeSet::new(),
            shapes: [index].into_iter().collect(),
            attributes: std::collections::BTreeSet::new(),
            texts: std::collections::BTreeSet::new(),
        }
    }

    pub fn single_attribute(kind: crate::state::SymbolAttributeKind) -> Self {
        Self {
            attributes: [kind].into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn single_text(id: u64) -> Self {
        Self {
            texts: [id].into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pins.is_empty()
            && self.shapes.is_empty()
            && self.attributes.is_empty()
            && self.texts.is_empty()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolClipboard {
    pub pins: Vec<crate::state::SymbolPin>,
    pub shapes: Vec<crate::state::SymbolShape>,
    pub texts: Vec<crate::state::SymbolTextObject>,
}

impl SymbolClipboard {
    pub fn is_empty(&self) -> bool {
        self.pins.is_empty() && self.shapes.is_empty() && self.texts.is_empty()
    }

    pub fn bounds(&self) -> Option<(crate::state::Point, crate::state::Point)> {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for pin in &self.pins {
            if let Some(position) = pin.position {
                xs.push(position.x);
                ys.push(position.y);
            }
        }
        for shape in &self.shapes {
            let (min, max) = symbol_shape_bounds(shape);
            xs.extend([min.x, max.x]);
            ys.extend([min.y, max.y]);
        }
        for text in &self.texts {
            xs.push(text.position.x);
            ys.push(text.position.y);
        }
        Some((
            crate::state::Point::new(xs.iter().min().copied()?, ys.iter().min().copied()?),
            crate::state::Point::new(xs.iter().max().copied()?, ys.iter().max().copied()?),
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolDocumentSnapshot {
    pub document: crate::state::SymbolDocument,
    pub symbol_document_metadata: Option<String>,
    pub symbol_editor_metadata: Option<String>,
    pub generated_metadata: Option<String>,
    pub ports_metadata: Option<String>,
}

impl SymbolDocumentSnapshot {
    pub fn from_document(document: &crate::state::SymbolDocument) -> Self {
        Self {
            document: document.clone(),
            symbol_document_metadata: None,
            symbol_editor_metadata: None,
            generated_metadata: None,
            ports_metadata: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SymbolUiState {
    pub tool: SymbolTool,
    pub selection: SymbolSelection,
    pub selected_pin: Option<String>,
    pub selected_shape: Option<usize>,
    pub selected_attribute: Option<crate::state::SymbolAttributeKind>,
    pub selected_text: Option<u64>,
    pub dragging_pin: Option<String>,
    pub dragging_shape: Option<(usize, crate::state::Point)>,
    pub dragging_label: Option<String>,
    pub dragging_origin: bool,
    /// Last drag point of a whole-selection move. Set instead of the
    /// single-object drags when the grabbed object belongs to a
    /// multi-object selection.
    pub dragging_group: Option<crate::state::Point>,
    pub drag_undo_recorded: bool,
    /// `true` once the open inspector field has pushed its undo snapshot.
    /// Typing into a coordinate is one edit, not one per keystroke; the flag
    /// clears when the field loses focus.
    pub inspector_undo_recorded: bool,
    pub marquee_start: Option<crate::state::Point>,
    pub marquee_current: Option<crate::state::Point>,
    pub zoom: f32,
    pub pan: (f32, f32),
    pub needs_fit: bool,
    pub pending_polyline: Vec<crate::state::Point>,
    pub shape_start: Option<crate::state::Point>,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub grid_spacing: SymbolGridSpacing,
    pub preview_as_placed: bool,
    pub save_dialog_open: bool,
    pub save_revision_note: String,
    pub save_error: Option<String>,
    pub clipboard: SymbolClipboard,
    pub undo_stacks: std::collections::HashMap<String, Vec<SymbolDocumentSnapshot>>,
    pub redo_stacks: std::collections::HashMap<String, Vec<SymbolDocumentSnapshot>>,
}

impl Default for SymbolUiState {
    fn default() -> Self {
        Self {
            tool: SymbolTool::Select,
            selection: SymbolSelection::default(),
            selected_pin: None,
            selected_shape: None,
            selected_attribute: None,
            selected_text: None,
            dragging_pin: None,
            dragging_shape: None,
            dragging_label: None,
            dragging_origin: false,
            dragging_group: None,
            drag_undo_recorded: false,
            inspector_undo_recorded: false,
            marquee_start: None,
            marquee_current: None,
            zoom: 4.0,
            pan: (0.0, 0.0),
            needs_fit: true,
            pending_polyline: Vec::new(),
            shape_start: None,
            show_grid: true,
            snap_to_grid: true,
            grid_spacing: SymbolGridSpacing::TwoPointFive,
            preview_as_placed: false,
            save_dialog_open: false,
            save_revision_note: String::new(),
            save_error: None,
            clipboard: SymbolClipboard::default(),
            undo_stacks: std::collections::HashMap::new(),
            redo_stacks: std::collections::HashMap::new(),
        }
    }
}

impl SymbolUiState {
    pub fn clear_selection(&mut self) {
        self.selection = SymbolSelection::default();
        self.selected_pin = None;
        self.selected_shape = None;
        self.selected_attribute = None;
        self.selected_text = None;
    }

    pub fn set_selection(&mut self, selection: SymbolSelection) {
        self.selected_pin = selection.pins.iter().next().cloned();
        self.selected_shape = selection.shapes.iter().next().copied();
        self.selected_attribute = selection.attributes.iter().next().copied();
        self.selected_text = selection.texts.iter().next().copied();
        self.selection = selection;
    }

    pub fn select_pin(&mut self, name: impl Into<String>) {
        self.set_selection(SymbolSelection::single_pin(name));
    }

    pub fn select_shape(&mut self, index: usize) {
        self.set_selection(SymbolSelection::single_shape(index));
    }

    pub fn select_attribute(&mut self, kind: crate::state::SymbolAttributeKind) {
        self.set_selection(SymbolSelection::single_attribute(kind));
    }

    pub fn select_text(&mut self, id: u64) {
        self.set_selection(SymbolSelection::single_text(id));
    }

    pub fn effective_selection(&self) -> SymbolSelection {
        if !self.selection.is_empty() {
            return self.selection.clone();
        }
        let mut selection = SymbolSelection::default();
        if let Some(pin) = self.selected_pin.clone() {
            selection.pins.insert(pin);
        }
        if let Some(shape) = self.selected_shape {
            selection.shapes.insert(shape);
        }
        if let Some(attribute) = self.selected_attribute {
            selection.attributes.insert(attribute);
        }
        if let Some(text) = self.selected_text {
            selection.texts.insert(text);
        }
        selection
    }

    pub fn clear_drag_state(&mut self) {
        self.dragging_pin = None;
        self.dragging_shape = None;
        self.dragging_label = None;
        self.dragging_origin = false;
        self.dragging_group = None;
        self.drag_undo_recorded = false;
    }
}

pub fn rotate_point_cw_about(
    point: crate::state::Point,
    origin: crate::state::Point,
) -> crate::state::Point {
    let relative = point - origin;
    origin + crate::state::Point::new(-relative.y, relative.x)
}

pub fn mirror_point_h_about(
    point: crate::state::Point,
    origin: crate::state::Point,
) -> crate::state::Point {
    let relative = point - origin;
    origin + crate::state::Point::new(-relative.x, relative.y)
}

pub fn mirror_point_v_about(
    point: crate::state::Point,
    origin: crate::state::Point,
) -> crate::state::Point {
    let relative = point - origin;
    origin + crate::state::Point::new(relative.x, -relative.y)
}

pub fn rotate_shape_cw_about(shape: &mut crate::state::SymbolShape, origin: crate::state::Point) {
    translate_shape_to_origin(shape, origin);
    shape.rotate_cw();
    shape.translate(origin);
}

pub fn mirror_shape_h_about(shape: &mut crate::state::SymbolShape, origin: crate::state::Point) {
    translate_shape_to_origin(shape, origin);
    shape.mirror_h();
    shape.translate(origin);
}

pub fn mirror_shape_v_about(shape: &mut crate::state::SymbolShape, origin: crate::state::Point) {
    translate_shape_to_origin(shape, origin);
    shape.mirror_v();
    shape.translate(origin);
}

fn translate_shape_to_origin(shape: &mut crate::state::SymbolShape, origin: crate::state::Point) {
    shape.translate(crate::state::Point::new(-origin.x, -origin.y));
}

pub fn symbol_shape_bounds(
    shape: &crate::state::SymbolShape,
) -> (crate::state::Point, crate::state::Point) {
    use crate::state::SymbolShape;
    match shape {
        SymbolShape::Polyline { points, .. } => {
            let min_x = points.iter().map(|point| point.x).min().unwrap_or(0);
            let max_x = points.iter().map(|point| point.x).max().unwrap_or(0);
            let min_y = points.iter().map(|point| point.y).min().unwrap_or(0);
            let max_y = points.iter().map(|point| point.y).max().unwrap_or(0);
            (
                crate::state::Point::new(min_x, min_y),
                crate::state::Point::new(max_x, max_y),
            )
        }
        SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => (
            crate::state::Point::new(center.x - radius, center.y - radius),
            crate::state::Point::new(center.x + radius, center.y + radius),
        ),
        SymbolShape::Arc { center, radius, .. } => (
            crate::state::Point::new(center.x - radius, center.y - radius),
            crate::state::Point::new(center.x + radius, center.y + radius),
        ),
        SymbolShape::Arrow { tip, .. } => (
            crate::state::Point::new(tip.x - 10, tip.y - 10),
            crate::state::Point::new(tip.x + 10, tip.y + 10),
        ),
    }
}

fn point_in_bounds(
    point: crate::state::Point,
    min: crate::state::Point,
    max: crate::state::Point,
) -> bool {
    (min.x..=max.x).contains(&point.x) && (min.y..=max.y).contains(&point.y)
}

fn bounds_intersect(
    a_min: crate::state::Point,
    a_max: crate::state::Point,
    b_min: crate::state::Point,
    b_max: crate::state::Point,
) -> bool {
    a_min.x <= b_max.x && a_max.x >= b_min.x && a_min.y <= b_max.y && a_max.y >= b_min.y
}

impl GridStyle {
    /// Whether any grid renders.
    pub fn visible(self) -> bool {
        self != GridStyle::Off
    }
}

/// Transient, device-local recovery point for a governed Select All command.
///
/// It is intentionally excluded from [`UiSessionStateSer`]: selection is
/// runtime editor state, not project data or a durable user preference.
#[derive(Debug, Clone)]
pub(crate) struct SchematicSelectionRecovery {
    pub(crate) active_key: String,
    pub(crate) selections: std::collections::HashMap<String, crate::state::Selection>,
}

/// Transient, device-local recovery point for a visibility-policy change.
#[derive(Debug, Clone)]
pub(crate) struct SchematicVisibilityRecovery {
    pub(crate) view_path: String,
    pub(crate) policy: SchematicVisibilityPolicy,
    pub(crate) routing_mode: crate::state::WireRoutingMode,
    pub(crate) net_highlight: crate::state::NetHighlightState,
    pub(crate) selection: crate::state::Selection,
}

/// Persistent and per-frame state shared by workbench document engines.
#[derive(Debug, Clone, Default)]
pub struct UiSessionState {
    /// Theme selection (direction / mode / density).
    pub theme: Theme,
    /// Autosave checkpoint interval in minutes; 0 = off. Checkpoints write
    /// next to the project file and never touch it.
    pub autosave_minutes: u8,
    /// Canonical user/device preference overrides not already owned by the
    /// theme or autosave runtime.
    pub preferences: crate::workbench::preferences::UserPreferences,
    /// Decimal-mark information supplied by the native/web platform edge.
    /// `None` is deliberate: engineering code must not guess the host locale.
    pub number_locale: crate::quantity::UiNumberLocale,
    /// Device-local UI message locale. Engineering source, identifiers, and
    /// portable numeric syntax never consult this presentation setting.
    pub text_locale: crate::workbench::UiTextLocale,
    /// Browser-only opt-in for egui's Web Speech interaction feedback.
    /// Native assistive technology is negotiated automatically by AccessKit.
    pub browser_spoken_feedback: bool,
    /// Toast queue.
    pub toasts: Toasts,
    /// Schematic-space cursor position, reported by the canvas each frame the
    /// pointer hovers it; consumed by the status bar.
    pub canvas_hover: Option<(f64, f64)>,
    /// Schematic-space center of the visible canvas, reported each frame the
    /// canvas renders; paste targets fall back to it when the cursor is not
    /// over the canvas (menu-driven paste).
    pub canvas_view_center: Option<(f64, f64)>,
    /// `simulation.data_version` at the last time the Results view was shown;
    /// drives the "new results" badge on the Results tab.
    pub results_seen_version: u64,
    /// Canvas grid style (dots / lines / off).
    pub grid: GridStyle,
    /// Last non-Off display style retained for backward-compatible session
    /// restoration. The current `G` command cycles dots, lines, and off.
    pub grid_restore_style: GridStyle,
    /// Schematic editor selection classes. This is restored with the UI
    /// session but never written into project-owned schematic data.
    pub schematic_selection_filter: crate::state::SchematicSelectionFilter,
    /// Active and named filters for exact schematic bulk-edit queries.
    pub schematic_bulk_edit_filters: SelectionBulkFilterSession,
    /// Device-local schematic display policy. Presentation choices are
    /// restored with the session and are never written to project data.
    pub schematic_visibility: SchematicVisibilityPolicy,
    /// Optional drawing-sheet construction detail shown around the permanent
    /// authored paper boundary. Output inclusion remains independently owned
    /// by Print and Export.
    pub drawing_sheet_layers: crate::state::DrawingSheetLayerVisibility,
    /// Exact runtime snap-target configuration used by the Grid & snap
    /// popover. The document grid pitch remains project-owned.
    pub schematic_snap: crate::state::SnapEngine,
    /// Exact runtime conductor routing mode. This retains the horizontal- or
    /// vertical-first orthogonal preference that the display policy's
    /// user-facing three-way style intentionally abstracts.
    pub schematic_routing_mode: crate::state::WireRoutingMode,
    /// Last governed Select All selection snapshot, including visible
    /// edit-in-place hierarchy buffers.
    pub(crate) schematic_selection_recovery: Option<SchematicSelectionRecovery>,
    /// Last visibility-policy presentation snapshot for the active view.
    pub(crate) schematic_visibility_recovery: Option<SchematicVisibilityRecovery>,
    /// Device-local edge-triggered schematic selection synchronization.
    /// This is deliberately excluded from `UiSessionStateSer`: source-map
    /// and retained-result highlights are presentation state, not project
    /// data or a durable preference value.
    pub(crate) schematic_cross_probe: crate::workbench::cross_probe::SchematicCrossProbeRuntime,
    /// Device-local working and named engineering-table views. Project-owned
    /// named views live in `ProjectWorkspace`; this store is the personal
    /// preference authority restored on desktop, web, and tablet hosts.
    pub engineering_table_views: crate::state::engineering_table::EngineeringTableViewStore,
    /// One-shot request to export the visible waveforms as CSV (needs the
    /// app's IO backend, so it is handled at the workbench boundary).
    pub export_csv_requested: bool,
    /// One-shot request to export the active view as a publication figure.
    ///
    /// This used to capture the window and crop it, which handed the reader
    /// back the pixels they already had at whatever resolution the display
    /// happened to be. It opens the hardcopy export workflow instead — the
    /// one pipeline that renders a sheet as PDF/A, PDF, SVG or PNG at a
    /// chosen resolution, and the same one Print already used.
    pub export_figure_requested: bool,
    /// One-shot platform clipboard payload emitted by commands that copy
    /// engineering readouts. Kept transient so clipboard intent is never
    /// serialized into a project/session.
    pub clipboard_text_request: Option<String>,
    /// One-shot request to change platform fullscreen state. Keeping this
    /// transient prevents the web backend from receiving the same viewport
    /// command every frame and preserves the user's persisted preference in
    /// [`crate::workbench::WorkbenchState`].
    pub(crate) full_screen_request: Option<bool>,
    /// Results workspace state (viewer, cursors, plot caches).
    pub results: crate::workbench::documents::result_document::ResultsState,
    /// Netlist editor state (diagnostics, diff pips, tuner mode).
    pub netlist: crate::workbench::documents::netlist_document::NetlistDocumentState,
    /// Visible Code & Automation page and revision-bound operation receipts.
    pub code_workspace: crate::workbench::documents::code_workspace::CodeWorkspaceRuntimeState,
    /// Runtime state for the Schematic-family symbol editor surface.
    pub symbol: SymbolUiState,
}

impl UiSessionState {
    /// New document session with safe autosave defaults.
    pub fn new() -> Self {
        Self {
            // Crash recovery is not opt-in; Off stays available.
            autosave_minutes: 5,
            ..Self::default()
        }
    }

    /// Replace the platform-provided locale used by interactive UI quantity
    /// fields. Portable project and SPICE parsers never read this value.
    pub fn set_number_locale(&mut self, locale: crate::quantity::UiNumberLocale) {
        self.number_locale = locale;
    }

    #[must_use]
    pub(crate) const fn messages(&self) -> crate::workbench::MessageCatalog {
        crate::workbench::MessageCatalog::new(self.text_locale)
    }

    pub(crate) fn set_grid_style(&mut self, style: GridStyle) {
        if style.visible() {
            self.grid_restore_style = style;
        }
        self.grid = style;
    }

    /// Cycle the upgraded toolbar's exact presentation order.
    pub(crate) fn cycle_grid_style(&mut self) -> GridStyle {
        let next = match self.grid {
            GridStyle::Dots => GridStyle::Lines,
            GridStyle::Lines => GridStyle::Off,
            GridStyle::Off => GridStyle::Dots,
        };
        self.set_grid_style(next);
        next
    }

    pub(crate) fn request_full_screen(&mut self, enabled: bool) {
        self.full_screen_request = Some(enabled);
    }

    pub(crate) fn take_full_screen_request(&mut self) -> Option<bool> {
        self.full_screen_request.take()
    }
}

/// Serialized subset of [`UiSessionState`]. Responsive layout is persisted by
/// [`crate::workbench::WorkbenchState`]; transient document interaction signals are not.
#[derive(Serialize, Deserialize)]
pub struct UiSessionStateSer {
    #[serde(default)]
    theme: Theme,
    /// Legacy on/off flag, still written so older builds keep their
    /// setting; `grid_style` wins when present.
    #[serde(default = "default_true")]
    show_grid: bool,
    #[serde(default)]
    grid_style: Option<GridStyle>,
    #[serde(default)]
    grid_restore_style: Option<GridStyle>,
    #[serde(default)]
    schematic_selection_filter: crate::state::SchematicSelectionFilter,
    #[serde(default)]
    schematic_bulk_edit_filters: SelectionBulkFilterSession,
    #[serde(default)]
    schematic_visibility: SchematicVisibilityPolicy,
    #[serde(default)]
    drawing_sheet_layers: crate::state::DrawingSheetLayerVisibility,
    #[serde(default)]
    schematic_snap: crate::state::SnapEngine,
    #[serde(default)]
    schematic_routing_mode: Option<crate::state::WireRoutingMode>,
    #[serde(default)]
    engineering_table_views: crate::state::engineering_table::EngineeringTableViewStore,
    #[serde(default = "default_autosave_minutes")]
    autosave_minutes: u8,
    #[serde(default)]
    preferences: crate::workbench::preferences::UserPreferences,
    #[serde(default)]
    browser_spoken_feedback: bool,
    #[serde(default)]
    text_locale: crate::workbench::UiTextLocale,
    #[serde(default)]
    result_viewer: crate::workbench::documents::result_document::ResultViewer,
}

fn default_autosave_minutes() -> u8 {
    5
}

fn normalize_autosave_minutes(minutes: u8) -> u8 {
    match minutes {
        2 | 5 | 10 => minutes,
        0..=3 => 2,
        4..=7 => 5,
        _ => 10,
    }
}

fn default_true() -> bool {
    true
}

impl Default for UiSessionStateSer {
    fn default() -> Self {
        Self::from(&UiSessionState::new())
    }
}

impl From<&UiSessionState> for UiSessionStateSer {
    fn from(session: &UiSessionState) -> Self {
        Self {
            theme: session.theme,
            show_grid: session.grid.visible(),
            grid_style: Some(session.grid),
            grid_restore_style: Some(if session.grid_restore_style.visible() {
                session.grid_restore_style
            } else if session.grid.visible() {
                session.grid
            } else {
                GridStyle::Dots
            }),
            schematic_selection_filter: session.schematic_selection_filter,
            schematic_bulk_edit_filters: session.schematic_bulk_edit_filters.clone(),
            schematic_visibility: session.schematic_visibility,
            drawing_sheet_layers: session.drawing_sheet_layers,
            schematic_snap: session.schematic_snap.clone(),
            schematic_routing_mode: Some(session.schematic_routing_mode),
            engineering_table_views: session.engineering_table_views.clone(),
            autosave_minutes: normalize_autosave_minutes(session.autosave_minutes),
            preferences: session.preferences.clone(),
            browser_spoken_feedback: session.browser_spoken_feedback,
            text_locale: session.text_locale,
            result_viewer: session.results.viewer,
        }
    }
}

impl From<UiSessionStateSer> for UiSessionState {
    fn from(ser: UiSessionStateSer) -> Self {
        let grid = ser.grid_style.unwrap_or(if ser.show_grid {
            GridStyle::Dots
        } else {
            GridStyle::Off
        });
        let grid_restore_style = ser
            .grid_restore_style
            .filter(|style| style.visible())
            .unwrap_or(if grid.visible() {
                grid
            } else {
                GridStyle::Dots
            });
        let mut preferences = ser.preferences;
        preferences.normalize();
        let schematic_routing_mode =
            ser.schematic_routing_mode
                .unwrap_or(match ser.schematic_visibility.wire_routing {
                    SchematicWireRoutingStyle::Orthogonal => {
                        crate::state::WireRoutingMode::HorizontalFirst
                    }
                    SchematicWireRoutingStyle::FortyFiveDegree => {
                        crate::state::WireRoutingMode::FortyFiveDegree
                    }
                    SchematicWireRoutingStyle::FreeAngle => crate::state::WireRoutingMode::Diagonal,
                });
        let mut session = Self {
            theme: ser.theme,
            grid,
            grid_restore_style,
            schematic_selection_filter: ser.schematic_selection_filter,
            schematic_bulk_edit_filters: ser.schematic_bulk_edit_filters,
            schematic_visibility: ser.schematic_visibility,
            drawing_sheet_layers: ser.drawing_sheet_layers,
            schematic_snap: ser.schematic_snap,
            schematic_routing_mode,
            engineering_table_views: ser.engineering_table_views,
            autosave_minutes: normalize_autosave_minutes(ser.autosave_minutes),
            preferences,
            browser_spoken_feedback: ser.browser_spoken_feedback,
            text_locale: ser.text_locale,
            ..Self::new()
        };
        session.results.viewer = ser.result_viewer;
        session
    }
}

#[cfg(test)]
mod symbol_selection_tests {
    use super::*;
    use crate::state::{
        Point, PortDirection, SchematicAnnotationVisibility, SchematicBackAnnotationContent,
        SchematicHierarchyVisibility, SchematicNetHighlighting, SchematicParameterLabelVisibility,
        SchematicReviewMarkerVisibility, SymbolDocument, SymbolPin, SymbolShape,
    };

    #[test]
    fn select_all_symbol_items_selects_pins_and_shapes() {
        let document = SymbolDocument {
            pins: vec![SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(-30, 0)),
            )],
            body: vec![SymbolShape::Dot {
                center: Point::origin(),
                radius: 2,
            }],
            ..SymbolDocument::default()
        };

        let selection = SymbolSelection::all_in(&document);

        assert!(selection.pins.contains("IN"));
        assert!(selection.shapes.contains(&0));
    }

    #[test]
    fn symbol_transforms_are_about_document_origin() {
        let origin = Point::new(10, 10);

        let point = rotate_point_cw_about(Point::new(20, 10), origin);

        assert_eq!(point, Point::new(10, 20));
    }

    #[test]
    fn schematic_selection_filter_round_trips_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_selection_filter.instances = false;
        session.schematic_selection_filter.annotations = false;
        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let wire: UiSessionStateSer =
            serde_json::from_value(serialized).expect("deserialize session");
        let restored = UiSessionState::from(wire);

        assert_eq!(
            restored.schematic_selection_filter,
            session.schematic_selection_filter
        );
    }

    #[test]
    fn ui_text_locale_round_trips_and_legacy_sessions_default_to_english() {
        let mut session = UiSessionState::new();
        session.text_locale = crate::workbench::UiTextLocale::PseudoExpanded;
        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );
        assert_eq!(
            restored.text_locale,
            crate::workbench::UiTextLocale::PseudoExpanded
        );

        let legacy = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session"),
        );
        assert_eq!(
            legacy.text_locale,
            crate::workbench::UiTextLocale::EnglishUnitedStates
        );
    }

    #[test]
    fn legacy_ui_session_defaults_to_all_selection_classes() {
        let wire: UiSessionStateSer =
            serde_json::from_value(serde_json::json!({})).expect("legacy session without a filter");
        let restored = UiSessionState::from(wire);

        assert!(restored.schematic_selection_filter.all_enabled());
    }

    #[test]
    fn schematic_bulk_edit_filters_round_trip_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_bulk_edit_filters.active = SelectionBulkFilter {
            query: "OPA*".to_owned(),
            object_kind: SelectionBulkObjectKind::InstancesAndParameters,
            hierarchy_scope: SelectionBulkHierarchyScope::CompleteProject,
            model_cell: "OPA189*".to_owned(),
            current_property: "section=tt".to_owned(),
        };
        session
            .schematic_bulk_edit_filters
            .save_active("TT amplifiers")
            .expect("save filter");

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.schematic_bulk_edit_filters,
            session.schematic_bulk_edit_filters
        );
    }

    #[test]
    fn personal_engineering_table_views_round_trip_with_the_ui_session() {
        let mut session = UiSessionState::new();
        let dataset = crate::state::engineering_table::EngineeringDataset::active_schematic(
            &Default::default(),
        );
        let view = crate::state::engineering_table::EngineeringTableView::for_dataset(&dataset);
        session
            .engineering_table_views
            .save(
                "My device view",
                crate::state::engineering_table::EngineeringViewScope::Personal,
                view,
                true,
                &dataset,
            )
            .expect("valid personal view");

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(
            restored.engineering_table_views,
            session.engineering_table_views
        );
    }

    #[test]
    fn named_bulk_filters_update_load_and_delete_case_insensitively() {
        let mut filters = SelectionBulkFilterSession::default();
        filters.active.query = "R*".to_owned();
        filters.save_active("Passives").expect("save");
        filters.active.query = "C*".to_owned();
        filters.save_active("passives").expect("replace");
        assert_eq!(filters.saved().len(), 1);
        assert!(filters.load("PASSIVES"));
        assert_eq!(filters.active.query, "C*");
        assert!(filters.remove("pAsSiVeS"));
        assert!(filters.saved().is_empty());
    }

    #[test]
    fn legacy_ui_session_defaults_bulk_edit_filters_without_project_mutation() {
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session without bulk filters"),
        );
        assert_eq!(
            restored.schematic_bulk_edit_filters,
            SelectionBulkFilterSession::default()
        );
    }

    #[test]
    fn schematic_visibility_policy_round_trips_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_visibility = SchematicVisibilityPolicy {
            hierarchy: SchematicHierarchyVisibility::ActiveOnly,
            annotations: SchematicAnnotationVisibility::ViolationsOnly,
            back_annotation: SchematicBackAnnotationContent::VoltagesCurrentsAndPower,
            parameter_labels: SchematicParameterLabelVisibility::Hidden,
            wire_routing: SchematicWireRoutingStyle::FortyFiveDegree,
            net_highlighting: SchematicNetHighlighting::Off,
            review_markers: SchematicReviewMarkerVisibility::All,
        };

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.schematic_visibility, session.schematic_visibility);
    }

    #[test]
    fn grid_cycle_is_dots_then_lines_then_off_and_round_trips_exactly() {
        let mut session = UiSessionState::new();
        session.set_grid_style(GridStyle::Dots);
        assert_eq!(session.cycle_grid_style(), GridStyle::Lines);
        assert_eq!(session.cycle_grid_style(), GridStyle::Off);
        assert_eq!(session.grid, GridStyle::Off);
        assert_eq!(session.grid_restore_style, GridStyle::Lines);

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let mut restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.grid, GridStyle::Off);
        assert_eq!(restored.grid_restore_style, GridStyle::Lines);
        assert_eq!(restored.cycle_grid_style(), GridStyle::Dots);
        assert_eq!(restored.cycle_grid_style(), GridStyle::Lines);
    }

    #[test]
    fn legacy_ui_session_uses_safe_visibility_defaults() {
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session without visibility policy"),
        );

        assert_eq!(
            restored.schematic_visibility,
            SchematicVisibilityPolicy::default()
        );
    }

    #[test]
    fn schematic_snap_and_exact_routing_round_trip_with_the_ui_session() {
        let mut session = UiSessionState::new();
        session.schematic_snap.enabled = false;
        session.schematic_snap.snap_to_wire_segments = false;
        session.schematic_snap.snap_radius = 7;
        session.schematic_routing_mode = crate::state::WireRoutingMode::VerticalFirst;

        let serialized =
            serde_json::to_value(UiSessionStateSer::from(&session)).expect("serialize session");
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serialized).expect("deserialize session"),
        );

        assert_eq!(restored.schematic_snap, session.schematic_snap);
        assert_eq!(
            restored.schematic_routing_mode,
            crate::state::WireRoutingMode::VerticalFirst
        );
    }

    #[test]
    fn legacy_ui_session_uses_snap_and_routing_defaults() {
        let restored = UiSessionState::from(
            serde_json::from_value::<UiSessionStateSer>(serde_json::json!({}))
                .expect("legacy session without editor runtime settings"),
        );

        assert_eq!(restored.schematic_snap, crate::state::SnapEngine::default());
        assert_eq!(
            restored.schematic_routing_mode,
            crate::state::WireRoutingMode::HorizontalFirst
        );
    }
}

#[cfg(test)]
mod platform_request_tests {
    use crate::workbench::UiSessionState;

    #[test]
    fn full_screen_request_is_consumed_exactly_once() {
        let mut session = UiSessionState::new();
        session.request_full_screen(true);

        assert_eq!(session.take_full_screen_request(), Some(true));
        assert_eq!(session.take_full_screen_request(), None);
    }

    #[test]
    fn newest_full_screen_request_supersedes_an_unapplied_request() {
        let mut session = UiSessionState::new();
        session.request_full_screen(true);
        session.request_full_screen(false);

        assert_eq!(session.take_full_screen_request(), Some(false));
    }
}
