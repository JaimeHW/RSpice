//! Isolated object-property drafts and form presentation over canonical design values.
//!
//! The host owns document context, live validation and commits; these drafts
//! retain expected values solely for semantic dirty state and validation.
//!
//! The form asks what the object has to be told and nothing else. It draws no
//! preview: the object is selected on the canvas behind the dialog, which is
//! already drawing it for real. The only fact it adds to the fields is how far
//! a rename reaches, stated under the field that causes it.

use rspice_design::schematic::{
    bus::{Bus, BusDeclaration, BusPropertyImpact, BusSlice, BusTap, BusTapOrientation},
    design_note::{DesignNote, DesignNoteKind, DesignReviewState},
    documentation_shape::{
        DocumentationShape, DocumentationShapeGeometry, DocumentationShapeKind, SymbolDash,
        SymbolFill, SymbolStroke, SymbolStyle, geometry_from_points,
    },
    net_label::NetLabel,
    owned::named_net::NamedNetTarget,
};
use rspice_design_model::Point;
use rspice_ui_kit::widgets::{FieldLine, FormRows};

pub const BUS_DECLARATION_FIELD: &str = "bus-declaration";
pub const TAP_SOURCE_FIELD: &str = "tap-source-bus";
pub const TAP_SLICE_FIELD: &str = "tap-slice";
pub const LABEL_NAME_FIELD: &str = "net-label-name";
pub const LABEL_X_FIELD: &str = "net-label-x";
pub const LABEL_Y_FIELD: &str = "net-label-y";
pub const NAMED_NET_NAME_FIELD: &str = "named-net-name";
const NOTE_KIND_FIELD: &str = "design-note-kind";
pub const NOTE_TEXT_FIELD: &str = "design-note-text";

/// Exact, isolated draft for a selected bus. The durable baseline is retained
/// to guard the eventual commit against stale-object overwrite.
#[derive(Debug, Clone)]
pub struct BusObjectPropertiesDraft {
    pub original: Bus,
    pub declaration: String,
}

/// Exact, isolated draft for a selected typed bus tap.
#[derive(Debug, Clone)]
pub struct BusTapObjectPropertiesDraft {
    pub original: BusTap,
    pub source_bus_id: u64,
    pub slice: String,
    pub orientation: BusTapOrientation,
}

/// Exact, isolated draft for a selected net label. Coordinates are retained as
/// text until Primary so incomplete or out-of-range edits never partially move
/// the electrical attachment point.
#[derive(Debug, Clone)]
pub struct NetLabelObjectPropertiesDraft {
    pub original: NetLabel,
    pub name: String,
    pub x: String,
    pub y: String,
}

/// Exact naming authority for a logical net selected through conductor
/// geometry. Unlike a label draft, this has no glyph position to edit: the
/// transaction updates every captured label/port name while retaining IDs.
#[derive(Debug, Clone)]
pub struct NamedNetObjectPropertiesDraft {
    pub original: NamedNetTarget,
    pub name: String,
}

/// Exact isolated draft for one non-electrical schematic documentation object.
#[derive(Debug, Clone)]
pub struct DesignNoteObjectPropertiesDraft {
    pub original: DesignNote,
    pub kind: DesignNoteKind,
    pub text: String,
    pub review_state: Option<DesignReviewState>,
}

/// A shape's fields as typed. A rectangle, and a callout's box, are told as
/// where their top-left corner stands and how wide and tall they are; a line,
/// an arc and a polygon by the points that define them.
#[derive(Debug, Clone)]
pub struct DocumentationShapeObjectPropertiesDraft {
    pub original: DocumentationShape,
    /// Each pair of fields: a point's X and Y, or a width and a height.
    pub points: Vec<(String, String)>,
    pub style: SymbolStyle,
}

/// What one pair of a shape draft's fields stands for.
enum ShapeFields {
    Point(String),
    Size,
}

impl DocumentationShapeObjectPropertiesDraft {
    pub fn new(shape: &DocumentationShape) -> Self {
        let pair = |x: i32, y: i32| (x.to_string(), y.to_string());
        let points = match &shape.geometry {
            DocumentationShapeGeometry::Rectangle { first, opposite } => {
                let (corner, width, height) = rect_fields(*first, *opposite);
                vec![pair(corner.x, corner.y), pair(width, height)]
            }
            DocumentationShapeGeometry::Callout {
                tip,
                elbow,
                box_corner,
            } => {
                let (corner, width, height) = rect_fields(*elbow, *box_corner);
                vec![
                    pair(tip.x, tip.y),
                    pair(corner.x, corner.y),
                    pair(width, height),
                ]
            }
            geometry => geometry
                .points()
                .into_iter()
                .map(|point| pair(point.x, point.y))
                .collect(),
        };
        Self {
            original: shape.clone(),
            points,
            style: shape.style,
        }
    }

    /// The geometry the fields describe, or what is wrong with them. Fields
    /// left as they opened keep the shape exactly as it was drawn.
    pub fn geometry(&self) -> Result<DocumentationShapeGeometry, String> {
        let kind = self.original.kind();
        let mut values = Vec::with_capacity(self.points.len());
        for ((x, y), fields) in self
            .points
            .iter()
            .zip(shape_fields(kind, self.points.len()))
        {
            let (x_name, y_name) = match &fields {
                ShapeFields::Point(name) => (format!("{name} X"), format!("{name} Y")),
                ShapeFields::Size => ("Width".to_owned(), "Height".to_owned()),
            };
            let whole = |text: &str, name: &str| {
                text.trim()
                    .parse::<i32>()
                    .map_err(|_| format!("{name} must be a whole number."))
            };
            let (x, y) = (whole(x, &x_name)?, whole(y, &y_name)?);
            if matches!(fields, ShapeFields::Size) && (x <= 0 || y <= 0) {
                return Err(format!(
                    "{} must be more than zero.",
                    if x <= 0 { "Width" } else { "Height" }
                ));
            }
            values.push(Point::new(x, y));
        }
        let opposite = |corner: Point, size: Point| {
            Point::new(
                corner.x.saturating_add(size.x),
                corner.y.saturating_add(size.y),
            )
        };
        let candidate = match (kind, values.as_slice()) {
            (DocumentationShapeKind::Rectangle, [corner, size]) => {
                DocumentationShapeGeometry::Rectangle {
                    first: *corner,
                    opposite: opposite(*corner, *size),
                }
            }
            (DocumentationShapeKind::Callout, [tip, corner, size]) => {
                // The leader meets the box at the corner nearest its tip.
                let far = opposite(*corner, *size);
                let x = if tip.x.abs_diff(corner.x) <= tip.x.abs_diff(far.x) {
                    (corner.x, far.x)
                } else {
                    (far.x, corner.x)
                };
                let y = if tip.y.abs_diff(corner.y) <= tip.y.abs_diff(far.y) {
                    (corner.y, far.y)
                } else {
                    (far.y, corner.y)
                };
                DocumentationShapeGeometry::Callout {
                    tip: *tip,
                    elbow: Point::new(x.0, y.0),
                    box_corner: Point::new(x.1, y.1),
                }
            }
            (kind, points) => {
                geometry_from_points(kind, points).map_err(|error| error.to_string())?
            }
        };
        if Self::new(&DocumentationShape {
            geometry: candidate.clone(),
            ..self.original.clone()
        })
        .points
            == Self::new(&self.original).points
        {
            return Ok(self.original.geometry.clone());
        }
        candidate.validate().map_err(|error| error.to_string())?;
        Ok(candidate)
    }
}

/// The top-left corner of the box two opposite corners span, and its width
/// and height.
fn rect_fields(first: Point, opposite: Point) -> (Point, i32, i32) {
    let corner = Point::new(first.x.min(opposite.x), first.y.min(opposite.y));
    let extent = |a: i32, b: i32| i32::try_from(a.abs_diff(b)).unwrap_or(i32::MAX);
    (
        corner,
        extent(first.x, opposite.x),
        extent(first.y, opposite.y),
    )
}

fn shape_fields(kind: DocumentationShapeKind, count: usize) -> Vec<ShapeFields> {
    let point = |name: &str| ShapeFields::Point(name.to_owned());
    match kind {
        DocumentationShapeKind::Rectangle => vec![point("Position"), ShapeFields::Size],
        DocumentationShapeKind::Callout => {
            vec![point("Target"), point("Box position"), ShapeFields::Size]
        }
        DocumentationShapeKind::Line => vec![point("Start"), point("End")],
        DocumentationShapeKind::Arc => vec![point("Start"), point("Through"), point("End")],
        DocumentationShapeKind::Polygon => (1..=count)
            .map(|index| ShapeFields::Point(format!("Vertex {index}")))
            .collect(),
    }
}

#[derive(Debug, Clone)]
pub enum ObjectPropertiesDraft {
    Bus(BusObjectPropertiesDraft),
    BusTap(BusTapObjectPropertiesDraft),
    NetLabel(NetLabelObjectPropertiesDraft),
    NamedNet(NamedNetObjectPropertiesDraft),
    DesignNote(DesignNoteObjectPropertiesDraft),
    DocumentationShape(DocumentationShapeObjectPropertiesDraft),
}

impl ObjectPropertiesDraft {
    pub fn is_modified(&self) -> bool {
        match self {
            Self::Bus(draft) => {
                let text = draft.declaration.trim();
                let candidate = if text.is_empty() {
                    Some(None)
                } else {
                    BusDeclaration::parse(text).ok().map(Some)
                };
                candidate.map_or_else(
                    || {
                        draft.declaration
                            != draft
                                .original
                                .declaration
                                .as_ref()
                                .map_or_else(String::new, ToString::to_string)
                    },
                    |candidate| candidate != draft.original.declaration,
                )
            }
            Self::BusTap(draft) => {
                let selector_changed = BusSlice::parse(draft.slice.trim()).map_or_else(
                    |_| draft.slice != draft.original.slice.to_string(),
                    |slice| slice != draft.original.slice,
                );
                draft.source_bus_id != draft.original.bus_id
                    || selector_changed
                    || draft.orientation != draft.original.orientation
            }
            Self::NetLabel(draft) => {
                let candidate = draft
                    .x
                    .trim()
                    .parse::<i32>()
                    .ok()
                    .zip(draft.y.trim().parse::<i32>().ok())
                    .map(|(x, y)| {
                        NetLabel::new(draft.original.id, Point::new(x, y), draft.name.trim())
                    });
                candidate.map_or_else(
                    || {
                        draft.name != draft.original.name
                            || draft.x != draft.original.pos.x.to_string()
                            || draft.y != draft.original.pos.y.to_string()
                    },
                    |candidate| candidate != draft.original,
                )
            }
            Self::NamedNet(draft) => draft.name.trim() != draft.original.name,
            Self::DesignNote(draft) => {
                let mut candidate = draft.original.clone();
                candidate
                    .update(draft.kind, draft.text.clone())
                    .map_or_else(
                        |_| true,
                        |_| {
                            if let Some(review_state) = draft.review_state
                                && candidate.set_review_state(review_state).is_err()
                            {
                                return true;
                            }
                            candidate != draft.original
                        },
                    )
            }
            Self::DocumentationShape(draft) => {
                draft.style != draft.original.style
                    || draft
                        .geometry()
                        .map_or(true, |geometry| geometry != draft.original.geometry)
            }
        }
    }
}

/// What this form is editing, as its title. The kind is the title's job: no
/// row below repeats it, and no row carries the object's internal ID.
pub fn title(draft: &ObjectPropertiesDraft) -> String {
    match draft {
        ObjectPropertiesDraft::Bus(_) => "Bus properties".to_owned(),
        ObjectPropertiesDraft::BusTap(_) => "Bus tap properties".to_owned(),
        ObjectPropertiesDraft::NetLabel(_) => "Net label properties".to_owned(),
        ObjectPropertiesDraft::NamedNet(_) => "Net properties".to_owned(),
        ObjectPropertiesDraft::DesignNote(_) => "Note properties".to_owned(),
        ObjectPropertiesDraft::DocumentationShape(draft) => {
            format!("{} properties", draft.original.kind().label())
        }
    }
}

/// How far the edit in the field above reaches, for the two drafts that rename
/// more than the object they are opened on. Nothing to say when the draft
/// renames only itself: a form does not report that it is ordinary.
fn rename_scope(
    draft: &ObjectPropertiesDraft,
    bus_impact: Option<BusPropertyImpact>,
) -> Option<String> {
    match draft {
        ObjectPropertiesDraft::Bus(_) => {
            let impact = bus_impact?;
            if !impact.has_changes() {
                return None;
            }
            let buses = match impact.buses_changed {
                0 => None,
                1 => Some("1 bus".to_owned()),
                count => Some(format!("{count} connected buses")),
            };
            let taps = match impact.taps_changed {
                0 => None,
                1 => Some("1 tap".to_owned()),
                count => Some(format!("{count} taps")),
            };
            Some(format!("Renames {}.", clause(buses, taps)?))
        }
        ObjectPropertiesDraft::NamedNet(draft) => {
            let labels = match draft.original.labels.len() {
                0 => None,
                1 => Some("1 label".to_owned()),
                count => Some(format!("{count} labels")),
            };
            let ports = match draft.original.ports.len() {
                0 => None,
                1 => Some("1 port".to_owned()),
                count => Some(format!("{count} ports")),
            };
            Some(format!("Renames {}.", clause(labels, ports)?))
        }
        ObjectPropertiesDraft::BusTap(_)
        | ObjectPropertiesDraft::NetLabel(_)
        | ObjectPropertiesDraft::DesignNote(_)
        | ObjectPropertiesDraft::DocumentationShape(_) => None,
    }
}

/// Two optional phrases joined by "and", or whichever one there is.
pub fn clause(left: Option<String>, right: Option<String>) -> Option<String> {
    match (left, right) {
        (Some(left), Some(right)) => Some(format!("{left} and {right}")),
        (Some(only), None) | (None, Some(only)) => Some(only),
        (None, None) => None,
    }
}

fn field_id(stable: &str) -> egui::Id {
    egui::Id::new(("object-properties", stable))
}

/// The one-line slot under a field: the refusal that names it, else what the
/// draft above it reaches, else nothing.
fn line_under(
    rows: &mut FormRows<'_>,
    field: Option<egui::Id>,
    owns_refusal: bool,
    refusal: Option<&str>,
    derived: Option<&str>,
) {
    let line = match (owns_refusal.then_some(refusal).flatten(), derived) {
        (Some(refusal), _) => FieldLine::Error(refusal),
        (None, Some(derived)) => FieldLine::Derived(derived),
        (None, None) => FieldLine::Blank,
    };
    rows.field_line(field, line);
}

/// The rows of one object's properties. Returns the control that takes initial
/// focus and whether the reader changed anything this frame.
pub fn show(
    rows: &mut FormRows<'_>,
    draft: &mut ObjectPropertiesDraft,
    bus_impact: Option<BusPropertyImpact>,
    bus_choices: &[(u64, String)],
    invalid_field: Option<&'static str>,
    refusal: Option<&str>,
) -> (Option<egui::Id>, bool) {
    let scope = rename_scope(draft, bus_impact);
    let owns = |field: &'static str| invalid_field == Some(field);
    let mut focus = None;
    let mut edited = false;
    match draft {
        ObjectPropertiesDraft::Bus(draft) => {
            let id = field_id(BUS_DECLARATION_FIELD);
            let response = rows.text(
                "Bus declaration",
                id,
                &mut draft.declaration,
                "DATA[15:0] or DATA<15:0>",
            );
            focus = Some(id);
            edited |= response.changed();
            line_under(
                rows,
                Some(id),
                owns(BUS_DECLARATION_FIELD),
                refusal,
                scope.as_deref(),
            );
        }
        ObjectPropertiesDraft::BusTap(draft) => {
            let labels: Vec<String> = bus_choices.iter().map(|(_, label)| label.clone()).collect();
            let selected = bus_choices
                .iter()
                .find(|(id, _)| id == &draft.source_bus_id)
                .map_or("No bus", |(_, label)| label.as_str())
                .to_owned();
            if let Some(index) = rows.select(
                "Source bus",
                "object-properties-tap-source-bus",
                &selected,
                &labels,
            ) && let Some((id, _)) = bus_choices.get(index)
                && draft.source_bus_id != *id
            {
                draft.source_bus_id = *id;
                edited = true;
            }
            line_under(rows, None, owns(TAP_SOURCE_FIELD), refusal, None);
            let slice = field_id(TAP_SLICE_FIELD);
            let response = rows.text("Bits", slice, &mut draft.slice, "DATA[3] or DATA[7:0]");
            focus = Some(slice);
            edited |= response.changed();
            line_under(rows, Some(slice), owns(TAP_SLICE_FIELD), refusal, None);
            edited |= orientation_row(rows, &mut draft.orientation);
        }
        ObjectPropertiesDraft::NetLabel(draft) => {
            let id = field_id(LABEL_NAME_FIELD);
            let response = rows.text("Net name", id, &mut draft.name, "vout or DATA[7]");
            focus = Some(id);
            edited |= response.changed();
            line_under(rows, Some(id), owns(LABEL_NAME_FIELD), refusal, None);
        }
        ObjectPropertiesDraft::NamedNet(draft) => {
            let id = field_id(NAMED_NET_NAME_FIELD);
            let response = rows.text("Net name", id, &mut draft.name, "vout or DATA[7]");
            focus = Some(id);
            edited |= response.changed();
            line_under(
                rows,
                Some(id),
                owns(NAMED_NET_NAME_FIELD),
                refusal,
                scope.as_deref(),
            );
        }
        ObjectPropertiesDraft::DesignNote(draft) => {
            let options = DesignNoteKind::ALL.map(|kind| kind.label().to_owned());
            if let Some(index) = rows.select("Type", NOTE_KIND_FIELD, draft.kind.label(), &options)
            {
                let next = DesignNoteKind::ALL[index];
                if next != draft.kind {
                    draft.kind = next;
                    draft.review_state =
                        (next == DesignNoteKind::ReviewNote).then_some(DesignReviewState::Open);
                    edited = true;
                }
            }
            let text = field_id(NOTE_TEXT_FIELD);
            let response = rows.text_multiline("Text", text, &mut draft.text, "", 3);
            focus = Some(text);
            edited |= response.changed();
            line_under(rows, Some(text), owns(NOTE_TEXT_FIELD), refusal, None);
            if draft.kind == DesignNoteKind::ReviewNote {
                let current = draft.review_state.unwrap_or(DesignReviewState::Open);
                let options = DesignReviewState::ALL.map(|state| state.label().to_owned());
                if let Some(index) = rows.select(
                    "Review state",
                    "design-note-review-state",
                    current.label(),
                    &options,
                ) {
                    let next = DesignReviewState::ALL[index];
                    if draft.review_state != Some(next) {
                        draft.review_state = Some(next);
                        edited = true;
                    }
                }
            }
        }
        ObjectPropertiesDraft::DocumentationShape(draft) => {
            let kind = draft.original.kind();
            let fields = shape_fields(kind, draft.points.len());
            for (index, ((x, y), fields)) in draft.points.iter_mut().zip(fields).enumerate() {
                let id = egui::Id::new(("documentation-shape-point", index));
                match fields {
                    ShapeFields::Point(name) => {
                        let (x, y) = rows.point(&name, id, x, y);
                        focus.get_or_insert(x.id);
                        edited |= x.changed() || y.changed();
                    }
                    ShapeFields::Size => {
                        let width = rows.text("Width", id.with("width"), x, "");
                        let height = rows.text("Height", id.with("height"), y, "");
                        edited |= width.changed() || height.changed();
                    }
                }
            }
            edited |= shape_style_rows(rows, kind, &mut draft.style);
        }
    }
    (focus, edited)
}

fn orientation_row(rows: &mut FormRows<'_>, orientation: &mut BusTapOrientation) -> bool {
    const ORIENTATIONS: [BusTapOrientation; 5] = [
        BusTapOrientation::Automatic,
        BusTapOrientation::Left,
        BusTapOrientation::Right,
        BusTapOrientation::Up,
        BusTapOrientation::Down,
    ];
    let options = ["Automatic", "Left", "Right", "Up", "Down"].map(str::to_owned);
    let selected = options[ORIENTATIONS
        .iter()
        .position(|candidate| candidate == orientation)
        .unwrap_or(0)]
    .clone();
    rows.select(
        "Orientation",
        "object-properties-tap-orientation",
        &selected,
        &options,
    )
    .and_then(|index| ORIENTATIONS.get(index))
    .is_some_and(|next| {
        if orientation == next {
            false
        } else {
            *orientation = *next;
            true
        }
    })
}

/// A shape's style as the symbol editor names it: the outline's stroke and
/// dash, and a closed shape's fill. Returns whether a choice changed.
fn shape_style_rows(
    rows: &mut FormRows<'_>,
    kind: DocumentationShapeKind,
    style: &mut SymbolStyle,
) -> bool {
    fn choice<T: Copy + PartialEq>(
        rows: &mut FormRows<'_>,
        label: &str,
        id: &str,
        all: &[T],
        name: fn(T) -> &'static str,
        value: &mut T,
    ) -> bool {
        let options = all
            .iter()
            .map(|item| name(*item).to_owned())
            .collect::<Vec<_>>();
        match rows
            .select(label, id, name(*value), &options)
            .and_then(|index| all.get(index))
        {
            Some(next) if next != value => {
                *value = *next;
                true
            }
            _ => false,
        }
    }
    let mut edited = choice(
        rows,
        "Stroke",
        "documentation-shape-stroke",
        &SymbolStroke::ALL,
        SymbolStroke::label,
        &mut style.stroke,
    );
    edited |= choice(
        rows,
        "Dash",
        "documentation-shape-dash",
        &SymbolDash::ALL,
        SymbolDash::label,
        &mut style.dash,
    );
    if kind.is_closed() {
        edited |= choice(
            rows,
            "Fill",
            "documentation-shape-fill",
            &SymbolFill::ALL,
            SymbolFill::label,
            &mut style.fill,
        );
    }
    edited
}
