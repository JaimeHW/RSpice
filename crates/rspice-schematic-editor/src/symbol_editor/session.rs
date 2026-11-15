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

    /// Whether the tool draws artwork. A drawing tool never selects or moves
    /// what is already drawn.
    pub fn draws(self) -> bool {
        !matches!(self, SymbolTool::Select | SymbolTool::PlacePin)
    }
}

/// The arc tool's second point, and how far the pointer has turned about the
/// center since it was placed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SymbolArcSweep {
    pub start: rspice_design_model::Point,
    /// Degrees turned from `start`, clockwise positive on a screen whose y
    /// runs down. It accumulates as the pointer moves, so the arc runs the way
    /// the pointer went and past half a turn keeps its direction.
    pub degrees: f64,
    /// The pointer's bearing from the center when last seen.
    bearing: f64,
}

impl SymbolArcSweep {
    pub fn new(center: rspice_design_model::Point, start: rspice_design_model::Point) -> Self {
        Self {
            start,
            degrees: 0.0,
            bearing: bearing(center, start),
        }
    }

    /// Follow the pointer to `point`. A point on the center has no bearing
    /// and leaves the sweep as it was.
    pub fn follow(
        &mut self,
        center: rspice_design_model::Point,
        point: rspice_design_model::Point,
    ) {
        if point == center {
            return;
        }
        let now = bearing(center, point);
        let turned = (now - self.bearing + 540.0).rem_euclid(360.0) - 180.0;
        self.degrees = (self.degrees + turned).clamp(-360.0, 360.0);
        self.bearing = now;
    }
}

/// The bearing of `point` from `center` in degrees from +x, clockwise on a
/// screen whose y runs down: the convention an authored arc's angles use.
pub fn bearing(center: rspice_design_model::Point, point: rspice_design_model::Point) -> f64 {
    f64::from(point.y - center.y)
        .atan2(f64::from(point.x - center.x))
        .to_degrees()
}

/// Text being typed where the text tool was clicked. Nothing is in the
/// document until Enter places it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolTextDraft {
    pub anchor: rspice_design_model::Point,
    pub text: String,
    /// Whether the entry has taken the caret, which it does once, on opening.
    pub caret_placed: bool,
}

/// The name of the pin the Pin tool has just placed, open for typing where
/// it stands. The pin is in the document already, under the name the tool
/// offered; Enter renames it to what was typed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolPinNameDraft {
    /// The pin's name in the document.
    pub pin: String,
    pub text: String,
    /// Whether the entry has taken the caret, which it does once, on opening.
    pub caret_placed: bool,
}

/// Display lattice of the symbol canvas.
///
/// This governs body artwork only. A terminal always lands on
/// [`rspice_design::symbol::SYMBOL_TERMINAL_GRID`] whatever is selected here, so a
/// finer display grid can never produce a pin a parent schematic cannot
/// wire to. The default is the terminal pitch itself, so the lattice a new
/// author draws against is the one their pins will snap to.
///
/// Symbol coordinates are whole units, so every pitch offered is a whole
/// number of them: a point snapped to it lands on the lattice drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SymbolGridSpacing {
    #[default]
    Ten,
    Five,
    Two,
    One,
}

impl SymbolGridSpacing {
    pub const ALL: [Self; 4] = [Self::Ten, Self::Five, Self::Two, Self::One];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Ten => "grid 10",
            Self::Five => "grid 5",
            Self::Two => "grid 2",
            Self::One => "grid 1",
        }
    }

    /// The pitch in symbol units.
    pub const fn model_step(self) -> i32 {
        match self {
            Self::Ten => 10,
            Self::Five => 5,
            Self::Two => 2,
            Self::One => 1,
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
    /// The handle of a selected shape being dragged.
    pub dragging_handle: Option<super::handles::SymbolHandleDrag>,
    pub drag_undo_recorded: bool,
    /// The point of a selected line or polygon that Delete removes: the
    /// one whose handle was clicked last, as `(shape, point)`.
    pub selected_vertex: Option<(usize, usize)>,
    /// The handle under the pointer, which Stretch takes hold of.
    pub hover_handle: Option<super::handles::SymbolHandleRef>,
    /// The handle Stretch armed: it follows the pointer, a click puts it
    /// down as one edit, and Escape lets it go.
    pub stretch: Option<super::handles::SymbolHandleRef>,
    /// What the open context menu was opened on, and where, in symbol
    /// coordinates.
    pub menu: Option<(
        super::context_menu::SymbolMenuTarget,
        rspice_design_model::Point,
    )>,
    /// Field drafts and undo grouping, scoped to the active symbol and selection.
    pub inspector: super::inspector::SymbolInspectorSession,
    pub marquee_start: Option<rspice_design_model::Point>,
    pub marquee_current: Option<rspice_design_model::Point>,
    pub zoom: f32,
    pub pan: (f32, f32),
    /// The views the canvas showed before this one, for Zoom previous.
    pub view_history: crate::view::navigation::ViewHistory<(f32, (f32, f32))>,
    pub needs_fit: bool,
    /// A change of view a command asked for, carried out on the canvas's
    /// next frame.
    pub zoom_request: Option<crate::view::navigation::ZoomRequest>,
    pub zoom_box: crate::view::navigation::ZoomBox,
    pub pending_polyline: Vec<rspice_design_model::Point>,
    /// A rectangle's first corner, or a circle's or an arc's center.
    pub shape_start: Option<rspice_design_model::Point>,
    pub arc_sweep: Option<SymbolArcSweep>,
    pub text_draft: Option<SymbolTextDraft>,
    /// The name of the pin just placed, while it is being typed.
    pub pin_name_draft: Option<SymbolPinNameDraft>,
    /// The side R turned the pin tool's pin to; `None` follows the artwork
    /// nearest the pointer.
    pub pin_side: Option<rspice_design_model::symbol_pin::SymbolPinSide>,
    /// The ports the symbol's interface declares, as the Pin tool was armed
    /// with them: those without a pin are the pins it offers first.
    pub declared_ports: Vec<rspice_design_model::port::PortSpec>,
    /// Whether the press-drag in progress began the shape being drawn.
    pub drawing_drag: bool,
    /// Where a click would land on the canvas, snapped as the armed tool
    /// snaps; `None` while the pointer is elsewhere.
    pub pointer: Option<rspice_design_model::Point>,
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
            dragging_handle: None,
            drag_undo_recorded: false,
            selected_vertex: None,
            hover_handle: None,
            stretch: None,
            menu: None,
            inspector: super::inspector::SymbolInspectorSession::default(),
            marquee_start: None,
            marquee_current: None,
            zoom: 4.0,
            pan: (0.0, 0.0),
            view_history: Default::default(),
            needs_fit: true,
            zoom_request: None,
            zoom_box: Default::default(),
            pending_polyline: Vec::new(),
            shape_start: None,
            arc_sweep: None,
            text_draft: None,
            pin_name_draft: None,
            pin_side: None,
            declared_ports: Vec::new(),
            drawing_drag: false,
            pointer: None,
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
        self.selected_vertex = None;
        self.stretch = None;
    }

    pub fn set_selection(&mut self, selection: SymbolSelection) {
        // A point picked on a shape stays picked only while its shape is
        // still the whole selection.
        if self
            .selected_vertex
            .is_some_and(|(shape, _)| selection != SymbolSelection::single_shape(shape))
        {
            self.selected_vertex = None;
        }
        if self
            .stretch
            .is_some_and(|stretch| !selection.shapes.contains(&stretch.shape))
        {
            self.stretch = None;
        }
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
        self.dragging_handle = None;
        self.drag_undo_recorded = false;
    }

    /// Whether a shape or a text is part drawn, or a placed pin's name is
    /// being typed: what Escape lets go of before it puts the tool down.
    pub fn has_draft(&self) -> bool {
        !self.pending_polyline.is_empty()
            || self.shape_start.is_some()
            || self.arc_sweep.is_some()
            || self.text_draft.is_some()
            || self.pin_name_draft.is_some()
    }

    /// What is in flight, as a command refused meanwhile names it: "a pin
    /// is being named", "text is being typed" or "a shape is being drawn".
    pub fn draft_activity(&self) -> Option<&'static str> {
        if self.pin_name_draft.is_some() {
            Some("a pin is being named")
        } else if self.text_draft.is_some() {
            Some("text is being typed")
        } else if self.has_draft() {
            Some("a shape is being drawn")
        } else {
            None
        }
    }

    /// Let go of the shape or text being drawn. The document never held it.
    pub fn clear_draft(&mut self) {
        self.pending_polyline.clear();
        self.shape_start = None;
        self.arc_sweep = None;
        self.text_draft = None;
        self.pin_name_draft = None;
        self.drawing_drag = false;
    }

    /// What the armed drawing tool takes next, for the status bar.
    pub fn next_step(&self) -> Option<&'static str> {
        let placed = self.pending_polyline.len();
        Some(match self.tool {
            SymbolTool::Select => return None,
            SymbolTool::PlacePin if self.pin_name_draft.is_some() => {
                "Pin: type its name, Enter to keep it"
            }
            SymbolTool::PlacePin => "Pin: click where its tip goes, R to turn it",
            SymbolTool::Line if placed == 0 => "Line: click the first point",
            SymbolTool::Line => "Line: click the next point, Enter to finish",
            SymbolTool::Polygon if placed == 0 => "Polygon: click the first corner",
            SymbolTool::Polygon if placed < 3 => "Polygon: click the next corner",
            SymbolTool::Polygon => "Polygon: click the next corner, Enter to close",
            SymbolTool::Rectangle if self.shape_start.is_none() => {
                "Rectangle: click a corner, or drag to the opposite one"
            }
            SymbolTool::Rectangle => "Rectangle: click the opposite corner",
            SymbolTool::Circle if self.shape_start.is_none() => {
                "Circle: click the center, or drag out the radius"
            }
            SymbolTool::Circle => "Circle: click a point on the circle",
            SymbolTool::Arc if self.shape_start.is_none() => "Arc: click the center",
            SymbolTool::Arc if self.arc_sweep.is_none() => "Arc: click where the arc starts",
            SymbolTool::Arc => "Arc: click where the arc ends",
            SymbolTool::Text if self.text_draft.is_none() => "Text: click where the text goes",
            SymbolTool::Text => "Text: type it, Enter to place",
        })
    }
}

pub fn rotate_point_cw_about(
    point: rspice_design_model::Point,
    origin: rspice_design_model::Point,
) -> rspice_design_model::Point {
    let relative = point - origin;
    origin + rspice_design_model::Point::new(-relative.y, relative.x)
}

/// `point` turned a quarter counter-clockwise about `origin`: the inverse
/// of [`rotate_point_cw_about`].
pub fn rotate_point_ccw_about(
    point: rspice_design_model::Point,
    origin: rspice_design_model::Point,
) -> rspice_design_model::Point {
    let relative = point - origin;
    origin + rspice_design_model::Point::new(relative.y, -relative.x)
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

/// `shape` turned a quarter counter-clockwise about `origin`: three
/// clockwise quarters.
pub fn rotate_shape_ccw_about(
    shape: &mut rspice_design::symbol::SymbolShape,
    origin: rspice_design_model::Point,
) {
    translate_shape_to_origin(shape, origin);
    shape.rotate_cw();
    shape.rotate_cw();
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
        SymbolShape::Rectangle { min, max, .. } => (*min, *max),
        SymbolShape::Circle { center, radius, .. } | SymbolShape::Dot { center, radius } => (
            rspice_design_model::Point::new(center.x - radius, center.y - radius),
            rspice_design_model::Point::new(center.x + radius, center.y + radius),
        ),
        SymbolShape::Arc { center, radius, .. } => (
            rspice_design_model::Point::new(center.x - radius, center.y - radius),
            rspice_design_model::Point::new(center.x + radius, center.y + radius),
        ),
        SymbolShape::Curve { curve, .. } => curve.whole_unit_bounds(),
        SymbolShape::Arrow { tip, .. } => (
            rspice_design_model::Point::new(tip.x - 10, tip.y - 10),
            rspice_design_model::Point::new(tip.x + 10, tip.y + 10),
        ),
        SymbolShape::Text {
            anchor,
            text,
            size,
            align,
            rotation,
        } => rspice_design::symbol::symbol_text_bounds(*anchor, text, *size, *align, *rotation),
        SymbolShape::Label {
            anchor,
            parameter,
            default,
            size,
            align,
            rotation,
            ..
        } => rspice_design::symbol::symbol_text_bounds(
            *anchor,
            &rspice_design::symbol::symbol_label_sample(parameter, default),
            *size,
            *align,
            *rotation,
        ),
    }
}

/// What a body shape is, in a word: "Rectangle", "Line", "Text". Only the
/// rectangle kind reads "Rectangle"; a closed outline of four square corners
/// is a polygon, as an imported drawing's box is.
pub fn shape_kind(shape: &rspice_design::symbol::SymbolShape) -> &'static str {
    use rspice_design::symbol::SymbolShape;
    match shape {
        SymbolShape::Polyline { closed: true, .. } => "Polygon",
        SymbolShape::Polyline { .. } => "Line",
        SymbolShape::Rectangle { .. } => "Rectangle",
        SymbolShape::Circle { .. } => "Circle",
        SymbolShape::Arc { .. } => "Arc",
        SymbolShape::Curve { curve, .. } if curve.radii[0] != curve.radii[1] => {
            if curve.sweep_degrees >= 360.0 {
                "Ellipse"
            } else {
                "Elliptical arc"
            }
        }
        SymbolShape::Curve { curve, .. } if curve.sweep_degrees >= 360.0 => "Circle",
        SymbolShape::Curve { .. } => "Arc",
        SymbolShape::Arrow { .. } => "Arrow",
        SymbolShape::Dot { .. } => "Dot",
        SymbolShape::Text { .. } => "Text",
        SymbolShape::Label { .. } => "Label",
    }
}

/// A body shape told apart from its neighbours: "Rectangle 60 × 40",
/// "Line, 3 points", "Circle r 10", "Text \"+\"".
pub fn shape_summary(shape: &rspice_design::symbol::SymbolShape) -> String {
    use rspice_design::symbol::SymbolShape;
    let kind = shape_kind(shape);
    match shape {
        SymbolShape::Polyline { points, .. } => format!("{kind}, {} points", points.len()),
        SymbolShape::Rectangle { min, max, .. } => {
            format!("{kind} {} \u{00d7} {}", max.x - min.x, max.y - min.y)
        }
        SymbolShape::Circle { radius, .. } => format!("{kind} r {radius}"),
        SymbolShape::Arc {
            radius,
            sweep_degrees,
            ..
        } => format!("{kind} r {radius}, {sweep_degrees}\u{00b0}"),
        SymbolShape::Curve { curve, .. } => {
            let [rx, ry] = curve.radii;
            let sweep = whole_or_tenths(curve.sweep_degrees);
            match (rx == ry, curve.sweep_degrees >= 360.0) {
                (true, true) => format!("{kind} r {}", whole_or_tenths(rx)),
                (true, false) => format!("{kind} r {}, {sweep}\u{00b0}", whole_or_tenths(rx)),
                (false, true) => format!(
                    "{kind} {} \u{00d7} {}",
                    whole_or_tenths(2.0 * rx),
                    whole_or_tenths(2.0 * ry)
                ),
                (false, false) => format!("{kind}, {sweep}\u{00b0}"),
            }
        }
        SymbolShape::Arrow { .. } => kind.to_owned(),
        SymbolShape::Dot { radius, .. } => format!("{kind} r {radius}"),
        SymbolShape::Text { text, .. } => match text.trim() {
            "" => kind.to_owned(),
            words => format!("{kind} \"{words}\""),
        },
        SymbolShape::Label { parameter, .. } => format!("{kind} {parameter}"),
    }
}

/// A measure as it reads: whole when it is, otherwise to a tenth.
fn whole_or_tenths(value: f64) -> String {
    let tenths = (value * 10.0).round() / 10.0;
    if tenths.fract() == 0.0 {
        format!("{tenths:.0}")
    } else {
        format!("{tenths:.1}")
    }
}

/// What a symbol label is called where one is named.
pub const fn label_name(kind: rspice_design::symbol::SymbolAttributeKind) -> &'static str {
    use rspice_design::symbol::SymbolAttributeKind;
    match kind {
        SymbolAttributeKind::Reference => "Reference",
        SymbolAttributeKind::Value => "Value",
        SymbolAttributeKind::Model => "Model",
    }
}

/// What `selection` holds, as an undo step names what it acted on:
/// "pin IN", "rectangle", "value label" or "3 items".
pub fn selection_subject(
    document: &rspice_design::symbol::SymbolDocument,
    selection: &SymbolSelection,
) -> String {
    let count = selection.pins.len() + selection.shapes.len() + selection.attributes.len();
    if count == 1 {
        if let Some(name) = selection.pins.first() {
            return format!("pin {name}");
        }
        if let Some(shape) = selection
            .shapes
            .first()
            .and_then(|index| document.body.get(*index))
        {
            return shape_kind(shape).to_lowercase();
        }
        if let Some(kind) = selection.attributes.first() {
            return format!("{} label", label_name(*kind).to_lowercase());
        }
    }
    rspice_ui_kit::accessibility::counted(count, "item", "items")
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

    /// A command refused while something is in flight names what it is, so
    /// a pin name being typed is not called a shape being drawn.
    #[test]
    fn the_draft_in_flight_is_named_for_what_it_is() {
        let mut session = SymbolEditorSession::default();
        assert_eq!(session.draft_activity(), None);
        session.pending_polyline.push(Point::origin());
        assert_eq!(session.draft_activity(), Some("a shape is being drawn"));
        session.clear_draft();
        session.text_draft = Some(SymbolTextDraft {
            anchor: Point::origin(),
            text: String::new(),
            caret_placed: false,
        });
        assert_eq!(session.draft_activity(), Some("text is being typed"));
        session.clear_draft();
        session.pin_name_draft = Some(SymbolPinNameDraft {
            pin: "PIN1".to_owned(),
            text: "PIN1".to_owned(),
            caret_placed: false,
        });
        assert_eq!(session.draft_activity(), Some("a pin is being named"));
    }

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
                    rotation: Default::default(),
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
                rotation: Default::default(),
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
    /// The sweep follows the pointer's turning, so an arc taken past half a
    /// turn keeps the direction it was drawn in, up to a whole turn.
    #[test]
    fn an_arc_sweep_accumulates_the_way_the_pointer_turned() {
        let center = Point::origin();
        let mut sweep = SymbolArcSweep::new(center, Point::new(10, 0));
        for point in [
            Point::new(0, 10),
            Point::new(-10, 0),
            Point::new(0, -10),
            Point::new(10, -1),
        ] {
            sweep.follow(center, point);
        }
        assert!((sweep.degrees - 354.3).abs() < 0.1, "{}", sweep.degrees);
        sweep.follow(center, Point::new(10, 1));
        sweep.follow(center, Point::new(0, 10));
        assert_eq!(sweep.degrees, 360.0, "never more than a whole turn");
        sweep.follow(center, center);
        assert_eq!(sweep.degrees, 360.0, "the center has no bearing");
    }

    /// The status bar's hint follows what the armed tool takes next.
    #[test]
    fn the_next_step_follows_the_shape_being_drawn() {
        let mut session = SymbolEditorSession::default();
        assert_eq!(session.next_step(), None);
        session.tool = SymbolTool::Line;
        assert_eq!(session.next_step(), Some("Line: click the first point"));
        session.pending_polyline.push(Point::origin());
        assert_eq!(
            session.next_step(),
            Some("Line: click the next point, Enter to finish")
        );
        session.tool = SymbolTool::Arc;
        session.clear_draft();
        session.shape_start = Some(Point::origin());
        assert_eq!(session.next_step(), Some("Arc: click where the arc starts"));
        session.arc_sweep = Some(SymbolArcSweep::new(Point::origin(), Point::new(10, 0)));
        assert_eq!(session.next_step(), Some("Arc: click where the arc ends"));
    }

    /// A body row tells one shape from the next: what it is and the measure
    /// that sets it apart, not just the kind.
    #[test]
    fn a_shape_summary_tells_shapes_apart() {
        use rspice_design::symbol::{SymbolTextAlign, SymbolTextSize};
        let summary = |shape: SymbolShape| shape_summary(&shape);
        assert_eq!(
            summary(super::super::interaction::rectangle_shape(
                Point::new(-30, -20),
                Point::new(30, 20)
            )),
            "Rectangle 60 \u{00d7} 40"
        );
        assert_eq!(
            summary(SymbolShape::Polyline {
                points: vec![Point::origin(), Point::new(10, 0), Point::new(10, 10)],
                closed: false,
                style: Default::default(),
            }),
            "Line, 3 points"
        );
        assert_eq!(
            summary(SymbolShape::Polyline {
                points: vec![Point::origin(), Point::new(10, 0), Point::new(10, 10)],
                closed: true,
                style: Default::default(),
            }),
            "Polygon, 3 points"
        );
        // Only the rectangle kind is a rectangle: a closed outline of four
        // square corners, as an imported drawing brings a box in, is a polygon.
        assert_eq!(
            summary(SymbolShape::Polyline {
                points: vec![
                    Point::origin(),
                    Point::new(10, 0),
                    Point::new(10, 10),
                    Point::new(0, 10)
                ],
                closed: true,
                style: Default::default(),
            }),
            "Polygon, 4 points"
        );
        assert_eq!(
            summary(SymbolShape::Polyline {
                points: vec![
                    Point::origin(),
                    Point::new(10, 0),
                    Point::new(10, 10),
                    Point::new(0, 10)
                ],
                closed: false,
                style: Default::default(),
            }),
            "Line, 4 points"
        );
        assert_eq!(
            summary(SymbolShape::Circle {
                center: Point::origin(),
                radius: 10,
                style: Default::default()
            }),
            "Circle r 10"
        );
        assert_eq!(
            summary(SymbolShape::Arc {
                center: Point::origin(),
                radius: 10,
                start_degrees: 0,
                sweep_degrees: 90,
                style: Default::default()
            }),
            "Arc r 10, 90\u{00b0}"
        );
        assert_eq!(
            summary(SymbolShape::Text {
                anchor: Point::origin(),
                text: "+".to_owned(),
                size: SymbolTextSize::Normal,
                align: SymbolTextAlign::Center,
                rotation: Default::default(),
            }),
            "Text \"+\""
        );
        assert_eq!(
            shape_kind(&super::super::interaction::rectangle_shape(
                Point::origin(),
                Point::new(10, 10)
            )),
            "Rectangle"
        );
    }

    #[test]
    fn symbol_transforms_are_about_document_origin() {
        let origin = Point::new(10, 10);

        let point = rotate_point_cw_about(Point::new(20, 10), origin);

        assert_eq!(point, Point::new(10, 20));
        assert_eq!(rotate_point_ccw_about(point, origin), Point::new(20, 10));
    }
}
