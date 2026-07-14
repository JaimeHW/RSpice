//! Session state shared by document engines.
//!
//! Top-level navigation, responsive layout, docks, and drawers belong to
//! [`super::WorkbenchState`]. This module contains only cross-frame document
//! interaction state and durable presentation preferences.

use serde::{Deserialize, Serialize};

use crate::ui::Theme;
use crate::ui::widgets::Toasts;

/// Canvas grid rendering style. The toolbar grid button and the View
/// menu cycle Dots → Lines → Off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum GridStyle {
    /// One dot per snap point (default).
    #[default]
    Dots,
    /// Hairline rules per snap point.
    Lines,
    /// No grid.
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolTool {
    #[default]
    Select,
    PlacePin,
    Polyline,
    Circle,
    Arc,
    Arrow,
    Dot,
}

impl SymbolTool {
    pub fn label(self) -> &'static str {
        match self {
            SymbolTool::Select => "Select",
            SymbolTool::PlacePin => "Place pin",
            SymbolTool::Polyline => "Polyline",
            SymbolTool::Circle => "Circle",
            SymbolTool::Arc => "Arc",
            SymbolTool::Arrow => "Arrow",
            SymbolTool::Dot => "Dot",
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolSelection {
    pub pins: std::collections::BTreeSet<String>,
    pub shapes: std::collections::BTreeSet<usize>,
}

impl SymbolSelection {
    pub fn all_in(document: &crate::state::SymbolDocument) -> Self {
        Self {
            pins: document.pins.iter().map(|pin| pin.name.clone()).collect(),
            shapes: (0..document.body.len()).collect(),
        }
    }

    pub fn in_rect(
        document: &crate::state::SymbolDocument,
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
        Self { pins, shapes }
    }

    pub fn single_pin(name: impl Into<String>) -> Self {
        Self {
            pins: [name.into()].into_iter().collect(),
            shapes: std::collections::BTreeSet::new(),
        }
    }

    pub fn single_shape(index: usize) -> Self {
        Self {
            pins: std::collections::BTreeSet::new(),
            shapes: [index].into_iter().collect(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pins.is_empty() && self.shapes.is_empty()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolClipboard {
    pub pins: Vec<crate::state::SymbolPin>,
    pub shapes: Vec<crate::state::SymbolShape>,
}

impl SymbolClipboard {
    pub fn is_empty(&self) -> bool {
        self.pins.is_empty() && self.shapes.is_empty()
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
    pub generated_metadata: Option<String>,
    pub ports_metadata: Option<String>,
}

impl SymbolDocumentSnapshot {
    pub fn from_document(document: &crate::state::SymbolDocument) -> Self {
        Self {
            document: document.clone(),
            symbol_document_metadata: None,
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
    pub dragging_pin: Option<String>,
    pub dragging_shape: Option<(usize, crate::state::Point)>,
    pub dragging_label: Option<String>,
    pub dragging_origin: bool,
    pub drag_undo_recorded: bool,
    pub marquee_start: Option<crate::state::Point>,
    pub marquee_current: Option<crate::state::Point>,
    pub zoom: f32,
    pub pan: (f32, f32),
    pub needs_fit: bool,
    pub pending_polyline: Vec<crate::state::Point>,
    pub shape_start: Option<crate::state::Point>,
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
            dragging_pin: None,
            dragging_shape: None,
            dragging_label: None,
            dragging_origin: false,
            drag_undo_recorded: false,
            marquee_start: None,
            marquee_current: None,
            zoom: 4.0,
            pan: (0.0, 0.0),
            needs_fit: true,
            pending_polyline: Vec::new(),
            shape_start: None,
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
    }

    pub fn set_selection(&mut self, selection: SymbolSelection) {
        self.selected_pin = selection.pins.iter().next().cloned();
        self.selected_shape = selection.shapes.iter().next().copied();
        self.selection = selection;
    }

    pub fn select_pin(&mut self, name: impl Into<String>) {
        self.set_selection(SymbolSelection::single_pin(name));
    }

    pub fn select_shape(&mut self, index: usize) {
        self.set_selection(SymbolSelection::single_shape(index));
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
        selection
    }

    pub fn clear_drag_state(&mut self) {
        self.dragging_pin = None;
        self.dragging_shape = None;
        self.dragging_label = None;
        self.dragging_origin = false;
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
    /// All styles in cycle order.
    pub const ALL: [GridStyle; 3] = [GridStyle::Dots, GridStyle::Lines, GridStyle::Off];

    /// The next style in the Dots → Lines → Off cycle.
    pub fn cycled(self) -> Self {
        match self {
            GridStyle::Dots => GridStyle::Lines,
            GridStyle::Lines => GridStyle::Off,
            GridStyle::Off => GridStyle::Dots,
        }
    }

    /// Menu / preferences label.
    pub fn label(self) -> &'static str {
        match self {
            GridStyle::Dots => "Dots",
            GridStyle::Lines => "Lines",
            GridStyle::Off => "Off",
        }
    }

    /// Whether any grid renders.
    pub fn visible(self) -> bool {
        self != GridStyle::Off
    }
}

/// An in-flight inspector edit session: the inspected component and the
/// pre-edit snapshot. One undo entry commits when the session ends (focus
/// leaves the fields or the selection moves) — not per keystroke.
#[derive(Debug, Clone)]
pub struct InspectorEdit {
    /// Component being edited.
    pub component_id: u64,
    /// Design state captured when the first keystroke landed.
    pub before: crate::state::SchematicSnapshot,
}

/// Persistent and per-frame state shared by workbench document engines.
#[derive(Debug, Clone, Default)]
pub struct UiSessionState {
    /// Theme selection (direction / mode / density).
    pub theme: Theme,
    /// Autosave checkpoint interval in minutes; 0 = off. Checkpoints write
    /// next to the project file and never touch it.
    pub autosave_minutes: u8,
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
    /// One-shot request to export the visible waveforms as CSV (needs the
    /// app's IO backend, so it is handled at the workbench boundary).
    pub export_csv_requested: bool,
    /// One-shot request to capture the window as a PNG (native: viewport
    /// screenshot + save dialog; the web build offers the browser's own
    /// capture instead).
    pub export_png_requested: bool,
    /// Results workspace state (viewer, cursors, plot caches).
    pub results: super::result_document::ResultsState,
    /// Netlist editor state (diagnostics, diff pips, tuner mode).
    pub netlist: super::netlist_document::NetlistDocumentState,
    /// Runtime state for the Schematic-family symbol editor surface.
    pub symbol: SymbolUiState,
    /// In-flight inspector edit session, if any.
    pub inspector_edit: Option<InspectorEdit>,
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
}

/// Serialized subset of [`UiSessionState`]. Responsive layout is persisted by
/// [`super::WorkbenchState`]; transient document interaction signals are not.
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
    #[serde(default = "default_autosave_minutes")]
    autosave_minutes: u8,
    #[serde(default)]
    browser_spoken_feedback: bool,
    #[serde(default)]
    result_viewer: super::result_document::ResultViewer,
}

fn default_autosave_minutes() -> u8 {
    5
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
            autosave_minutes: session.autosave_minutes,
            browser_spoken_feedback: session.browser_spoken_feedback,
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
        let mut session = Self {
            theme: ser.theme,
            grid,
            autosave_minutes: ser.autosave_minutes,
            browser_spoken_feedback: ser.browser_spoken_feedback,
            ..Self::new()
        };
        session.results.viewer = ser.result_viewer;
        session
    }
}

#[cfg(test)]
mod symbol_selection_tests {
    use super::*;
    use crate::state::{Point, PortDirection, SymbolDocument, SymbolPin, SymbolShape};

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
}
