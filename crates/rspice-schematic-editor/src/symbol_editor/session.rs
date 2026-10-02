//! Local symbol editor state and selection geometry.

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

/// Display lattice of the symbol canvas.
///
/// This governs body artwork only. A terminal always lands on
/// [`rspice_design::symbol::SYMBOL_TERMINAL_GRID`] whatever is selected here, so a
/// finer display grid can never produce a pin a parent schematic cannot
/// wire to. The default is the terminal pitch itself, so the lattice a new
/// author draws against is the one their pins will snap to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolGridSpacing {
    #[default]
    Ten,
    Five,
    TwoPointFive,
}

impl SymbolGridSpacing {
    pub const ALL: [Self; 3] = [Self::Ten, Self::Five, Self::TwoPointFive];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ten => "grid 10",
            Self::Five => "grid 5",
            Self::TwoPointFive => "grid 2.5 \u{00b7} fine",
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
    /// Body shapes by index. Authored text is a body shape, so Select All and
    /// every canvas transform reach it without a category of its own.
    pub shapes: std::collections::BTreeSet<usize>,
    pub attributes: std::collections::BTreeSet<rspice_design::symbol::SymbolAttributeKind>,
}

impl SymbolSelection {
    pub fn all_in(document: &rspice_design::symbol::SymbolDocument) -> Self {
        Self {
            pins: document.pins.iter().map(|pin| pin.name.clone()).collect(),
            shapes: (0..document.body.len()).collect(),
            attributes: rspice_design::symbol::SymbolAttributeKind::ALL
                .into_iter()
                .collect(),
        }
    }

    pub fn in_rect(
        document: &rspice_design::symbol::SymbolDocument,
        metadata: &rspice_design::symbol::SymbolEditorMetadata,
        start: rspice_design_model::Point,
        end: rspice_design_model::Point,
    ) -> Self {
        let min = rspice_design_model::Point::new(start.x.min(end.x), start.y.min(end.y));
        let max = rspice_design_model::Point::new(start.x.max(end.x), start.y.max(end.y));
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
        Self {
            pins,
            shapes,
            attributes,
        }
    }

    pub fn single_pin(name: impl Into<String>) -> Self {
        Self {
            pins: [name.into()].into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn single_shape(index: usize) -> Self {
        Self {
            shapes: [index].into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn single_attribute(kind: rspice_design::symbol::SymbolAttributeKind) -> Self {
        Self {
            attributes: [kind].into_iter().collect(),
            ..Self::default()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.pins.is_empty() && self.shapes.is_empty() && self.attributes.is_empty()
    }

    pub fn contains_pin(&self, name: &str) -> bool {
        self.pins.contains(name)
    }

    /// Add the object to the selection, or drop it when it is already in.
    ///
    /// This is the shift-click gesture: it grows a group without discarding
    /// what the previous click selected, which is what makes a multi-object
    /// drag reachable at all.
    pub fn toggle_pin(&mut self, name: &str) {
        if !self.pins.remove(name) {
            self.pins.insert(name.to_owned());
        }
    }

    pub fn toggle_shape(&mut self, index: usize) {
        if !self.shapes.remove(&index) {
            self.shapes.insert(index);
        }
    }

    pub fn toggle_attribute(&mut self, kind: rspice_design::symbol::SymbolAttributeKind) {
        if !self.attributes.remove(&kind) {
            self.attributes.insert(kind);
        }
    }

    /// How many objects the selection holds, across all three classes.
    pub fn len(&self) -> usize {
        self.pins.len() + self.shapes.len() + self.attributes.len()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct SymbolClipboard {
    pub pins: Vec<rspice_design::symbol::SymbolPin>,
    pub shapes: Vec<rspice_design::symbol::SymbolShape>,
}

impl SymbolClipboard {
    pub fn is_empty(&self) -> bool {
        self.pins.is_empty() && self.shapes.is_empty()
    }

    pub fn bounds(&self) -> Option<(rspice_design_model::Point, rspice_design_model::Point)> {
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
            rspice_design_model::Point::new(xs.iter().min().copied()?, ys.iter().min().copied()?),
            rspice_design_model::Point::new(xs.iter().max().copied()?, ys.iter().max().copied()?),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct SymbolEditorSession {
    pub canvas_source: Option<super::interaction::SymbolRequestSource>,
    pub tool: SymbolTool,
    pub selection: SymbolSelection,
    pub selected_pin: Option<String>,
    pub selected_shape: Option<usize>,
    pub selected_attribute: Option<rspice_design::symbol::SymbolAttributeKind>,
    pub dragging_pin: Option<String>,
    pub dragging_shape: Option<(usize, rspice_design_model::Point)>,
    pub dragging_label: Option<rspice_design::symbol::SymbolAttributeKind>,
    pub dragging_origin: bool,
    /// Last drag point of a whole-selection move. Set instead of the
    /// single-object drags when the grabbed object belongs to a
    /// multi-object selection.
    pub dragging_group: Option<rspice_design_model::Point>,
    pub drag_undo_recorded: bool,
    /// `true` once the open inspector field has pushed its undo snapshot.
    /// Typing into a coordinate is one edit, not one per keystroke; the flag
    /// clears when the field loses focus.
    pub inspector_undo_recorded: bool,
    pub marquee_start: Option<rspice_design_model::Point>,
    pub marquee_current: Option<rspice_design_model::Point>,
    pub zoom: f32,
    pub pan: (f32, f32),
    pub needs_fit: bool,
    pub pending_polyline: Vec<rspice_design_model::Point>,
    pub shape_start: Option<rspice_design_model::Point>,
    pub show_grid: bool,
    pub snap_to_grid: bool,
    pub grid_spacing: SymbolGridSpacing,
    pub preview_as_placed: bool,
    pub clipboard: SymbolClipboard,
}

impl Default for SymbolEditorSession {
    fn default() -> Self {
        Self {
            canvas_source: None,
            tool: SymbolTool::Select,
            selection: SymbolSelection::default(),
            selected_pin: None,
            selected_shape: None,
            selected_attribute: None,
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
            grid_spacing: SymbolGridSpacing::Ten,
            preview_as_placed: false,
            clipboard: SymbolClipboard::default(),
        }
    }
}

impl SymbolEditorSession {
    pub fn clear_selection(&mut self) {
        self.selection = SymbolSelection::default();
        self.selected_pin = None;
        self.selected_shape = None;
        self.selected_attribute = None;
    }

    pub fn set_selection(&mut self, selection: SymbolSelection) {
        self.selected_pin = selection.pins.iter().next().cloned();
        self.selected_shape = selection.shapes.iter().next().copied();
        self.selected_attribute = selection.attributes.iter().next().copied();
        self.selection = selection;
    }

    pub fn select_pin(&mut self, name: impl Into<String>) {
        self.set_selection(SymbolSelection::single_pin(name));
    }

    pub fn select_shape(&mut self, index: usize) {
        self.set_selection(SymbolSelection::single_shape(index));
    }

    pub fn select_attribute(&mut self, kind: rspice_design::symbol::SymbolAttributeKind) {
        self.set_selection(SymbolSelection::single_attribute(kind));
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
    point: rspice_design_model::Point,
    origin: rspice_design_model::Point,
) -> rspice_design_model::Point {
    let relative = point - origin;
    origin + rspice_design_model::Point::new(-relative.y, relative.x)
}

pub fn mirror_point_h_about(
    point: rspice_design_model::Point,
    origin: rspice_design_model::Point,
) -> rspice_design_model::Point {
    let relative = point - origin;
    origin + rspice_design_model::Point::new(-relative.x, relative.y)
}

pub fn mirror_point_v_about(
    point: rspice_design_model::Point,
    origin: rspice_design_model::Point,
) -> rspice_design_model::Point {
    let relative = point - origin;
    origin + rspice_design_model::Point::new(relative.x, -relative.y)
}

pub fn rotate_shape_cw_about(
    shape: &mut rspice_design::symbol::SymbolShape,
    origin: rspice_design_model::Point,
) {
    translate_shape_to_origin(shape, origin);
    shape.rotate_cw();
    shape.translate(origin);
}

pub fn mirror_shape_h_about(
    shape: &mut rspice_design::symbol::SymbolShape,
    origin: rspice_design_model::Point,
) {
    translate_shape_to_origin(shape, origin);
    shape.mirror_h();
    shape.translate(origin);
}

pub fn mirror_shape_v_about(
    shape: &mut rspice_design::symbol::SymbolShape,
    origin: rspice_design_model::Point,
) {
    translate_shape_to_origin(shape, origin);
    shape.mirror_v();
    shape.translate(origin);
}

fn translate_shape_to_origin(
    shape: &mut rspice_design::symbol::SymbolShape,
    origin: rspice_design_model::Point,
) {
    shape.translate(rspice_design_model::Point::new(-origin.x, -origin.y));
}

pub fn symbol_shape_bounds(
    shape: &rspice_design::symbol::SymbolShape,
) -> (rspice_design_model::Point, rspice_design_model::Point) {
    use rspice_design::symbol::SymbolShape;
    match shape {
        SymbolShape::Polyline { points, .. } => {
            let min_x = points.iter().map(|point| point.x).min().unwrap_or(0);
            let max_x = points.iter().map(|point| point.x).max().unwrap_or(0);
            let min_y = points.iter().map(|point| point.y).min().unwrap_or(0);
            let max_y = points.iter().map(|point| point.y).max().unwrap_or(0);
            (
                rspice_design_model::Point::new(min_x, min_y),
                rspice_design_model::Point::new(max_x, max_y),
            )
        }
        SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => (
            rspice_design_model::Point::new(center.x - radius, center.y - radius),
            rspice_design_model::Point::new(center.x + radius, center.y + radius),
        ),
        SymbolShape::Arc { center, radius, .. } => (
            rspice_design_model::Point::new(center.x - radius, center.y - radius),
            rspice_design_model::Point::new(center.x + radius, center.y + radius),
        ),
        SymbolShape::Arrow { tip, .. } => (
            rspice_design_model::Point::new(tip.x - 10, tip.y - 10),
            rspice_design_model::Point::new(tip.x + 10, tip.y + 10),
        ),
        SymbolShape::Text {
            anchor,
            text,
            size,
            align,
        } => rspice_design::symbol::symbol_text_bounds(*anchor, text, *size, *align),
    }
}

fn point_in_bounds(
    point: rspice_design_model::Point,
    min: rspice_design_model::Point,
    max: rspice_design_model::Point,
) -> bool {
    (min.x..=max.x).contains(&point.x) && (min.y..=max.y).contains(&point.y)
}

fn bounds_intersect(
    a_min: rspice_design_model::Point,
    a_max: rspice_design_model::Point,
    b_min: rspice_design_model::Point,
    b_max: rspice_design_model::Point,
) -> bool {
    a_min.x <= b_max.x && a_max.x >= b_min.x && a_min.y <= b_max.y && a_max.y >= b_min.y
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::symbol::{SymbolDocument, SymbolPin, SymbolShape};
    use rspice_design_model::{Point, port::PortDirection};
    /// Select All reaches authored text because text is a body shape, not a
    /// category beside one.
    #[test]
    fn select_all_symbol_items_selects_pins_shapes_and_text() {
        let document = SymbolDocument {
            pins: vec![SymbolPin::new(
                "IN",
                PortDirection::In,
                Some(Point::new(-30, 0)),
            )],
            body: vec![
                SymbolShape::Dot {
                    center: Point::origin(),
                    radius: 2,
                },
                SymbolShape::Text {
                    anchor: Point::new(0, -20),
                    text: "AMP".to_owned(),
                    size: rspice_design::symbol::SymbolTextSize::Normal,
                    align: rspice_design::symbol::SymbolTextAlign::Center,
                },
            ],
            ..SymbolDocument::default()
        };

        let selection = SymbolSelection::all_in(&document);

        assert!(selection.pins.contains("IN"));
        assert_eq!(
            selection.shapes.iter().copied().collect::<Vec<usize>>(),
            vec![0, 1]
        );
    }
    /// A marquee over a text run takes it: its bounds are the run, not the
    /// anchor point, so a label is as selectable as the artwork beside it.
    #[test]
    fn a_marquee_over_a_text_run_selects_it() {
        let document = SymbolDocument {
            body: vec![SymbolShape::Text {
                anchor: Point::origin(),
                text: "AMP".to_owned(),
                size: rspice_design::symbol::SymbolTextSize::Normal,
                align: rspice_design::symbol::SymbolTextAlign::Left,
            }],
            ..SymbolDocument::default()
        };
        let metadata = rspice_design::symbol::SymbolEditorMetadata::for_document(&document);

        let over_the_run =
            SymbolSelection::in_rect(&document, &metadata, Point::new(10, -2), Point::new(12, 2));
        let clear_of_it =
            SymbolSelection::in_rect(&document, &metadata, Point::new(40, -2), Point::new(60, 2));

        assert!(over_the_run.shapes.contains(&0));
        assert!(clear_of_it.shapes.is_empty());
    }
    #[test]
    fn symbol_transforms_are_about_document_origin() {
        let origin = Point::new(10, 10);

        let point = rotate_point_cw_about(Point::new(20, 10), origin);

        assert_eq!(point, Point::new(10, 20));
    }
}
