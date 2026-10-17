//! Component placement, committed orientation edits, and terminal geometry.

use super::{
    component::{Component, LibraryCellInstance},
    component_type::ComponentType,
    document::SchematicDocument,
    documentation_shape::DocumentationShapeGeometry,
    identity::SchematicIdentity,
    movement::orthogonal_route_for_corresponding_points,
    rotation::Rotation,
    selection::Selection,
};
use rspice_design_model::Point;

/// Placement geometry supplied by the caller's current tool or command.
#[derive(Debug, Clone, Copy)]
pub struct ComponentPlacement {
    pub position: Point,
    pub rotation: Rotation,
    pub mirror_h: bool,
}

/// Model card and optional symbol skin for a matched native-device placement.
#[derive(Debug, Clone, Copy)]
pub struct ComponentModelOverride<'a> {
    pub model: &'a str,
    pub symbol_variant: Option<&'a str>,
}

/// A turn or flip of the whole selection about one pivot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionTransform {
    RotateClockwise,
    RotateCounterClockwise,
    /// Flip left to right, about a vertical axis.
    MirrorHorizontal,
    /// Flip top to bottom, about a horizontal axis.
    MirrorVertical,
}

pub fn add_component(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    kind: ComponentType,
    placement: ComponentPlacement,
    model_override: Option<ComponentModelOverride<'_>>,
) -> u64 {
    let id = identity.allocate(document);
    let name = identity.generate_name(kind);
    let mut component = Component::new(id, kind, placement.position);
    component.name = name;
    component.rotation = placement.rotation;
    component.mirror_h = placement.mirror_h;

    // Set default values
    component.value = kind.default_value().to_string();

    // A port's value IS its interface name — every placement gets a
    // fresh one so two new ports never silently short their nets.
    if kind == ComponentType::Port {
        component.value = next_port_name(document);
    }

    if let Some(model) = model_override {
        component.value = model.model.to_owned();
        component.symbol_variant = model.symbol_variant.map(str::to_owned);
    }

    document.components.push(component);
    id
}

/// First unused `p<N>` port name in this schematic.
fn next_port_name(document: &SchematicDocument) -> String {
    let taken: std::collections::HashSet<String> = document
        .components
        .iter()
        .filter(|c| c.kind == ComponentType::Port)
        .map(|c| c.value.trim().to_ascii_lowercase())
        .collect();
    (1..)
        .map(|n| format!("p{n}"))
        .find(|candidate| !taken.contains(candidate))
        .expect("unbounded name space")
}

pub fn add_library_cell_component(
    document: &mut SchematicDocument,
    identity: &mut SchematicIdentity,
    placement: ComponentPlacement,
    library_cell: LibraryCellInstance,
) -> u64 {
    let id = identity.allocate(document);
    let preferred_prefix = library_cell.effective_reference_prefix();
    let name = if let Some(prefix) = preferred_prefix {
        next_library_reference_name(&document.components, prefix)
    } else {
        identity.generate_name(ComponentType::CellInstance)
    };
    let mut component = Component::new(id, ComponentType::CellInstance, placement.position);
    component.name = name;
    component.rotation = placement.rotation;
    component.mirror_h = placement.mirror_h;
    component.value = library_cell.cell.clone();
    component.library_cell = Some(library_cell);

    document.components.push(component);
    id
}

impl SelectionTransform {
    /// The undo-history name of this edit.
    pub const fn description(self) -> &'static str {
        match self {
            Self::RotateClockwise => "rotate selection",
            Self::RotateCounterClockwise => "rotate selection counter-clockwise",
            Self::MirrorHorizontal => "mirror horizontally",
            Self::MirrorVertical => "mirror vertically",
        }
    }

    /// Where `point` goes when the drawing turns or flips about `pivot`.
    /// Screen y grows downward, so clockwise takes +x to +y, as a part's
    /// `R90` does.
    pub fn apply(self, point: Point, pivot: Point) -> Point {
        let (dx, dy) = (point.x - pivot.x, point.y - pivot.y);
        let (x, y) = match self {
            Self::RotateClockwise => (-dy, dx),
            Self::RotateCounterClockwise => (dy, -dx),
            Self::MirrorHorizontal => (-dx, dy),
            Self::MirrorVertical => (dx, -dy),
        };
        Point::new(pivot.x + x, pivot.y + y)
    }

    /// Turn or flip a part's orientation the same way in world terms. A
    /// part's orientation is a rotation applied after its local mirror, and
    /// a world flip reverses the sense of that rotation.
    fn orient(self, component: &mut Component) {
        let negated = |rotation: Rotation| match rotation {
            Rotation::R90 => Rotation::R270,
            Rotation::R270 => Rotation::R90,
            other => other,
        };
        match self {
            Self::RotateClockwise => component.rotation = component.rotation.rotate_cw(),
            Self::RotateCounterClockwise => component.rotation = component.rotation.rotate_ccw(),
            Self::MirrorHorizontal => {
                component.rotation = negated(component.rotation);
                component.toggle_mirror_h();
            }
            Self::MirrorVertical => {
                component.rotation = negated(component.rotation);
                component.toggle_mirror_v();
            }
        }
    }
}

/// Turn or flip everything `selection` holds about `pivot` as one rigid body:
/// parts (position and orientation), wires, buses, junctions, bus taps, net
/// labels, notes, shapes and probes. A wire left out of the selection that
/// ends on something that moved follows it, rerouted with square corners
/// where it was square; one that cannot be rerouted that way follows in a
/// straight line. Returns whether anything moved.
pub fn transform_selection(
    document: &mut SchematicDocument,
    selection: &Selection,
    pivot: Point,
    mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    transform: SelectionTransform,
) -> bool {
    if !selection_has_content(document, selection) {
        return false;
    }
    let map = |point: Point| transform.apply(point, pivot);
    // Old position to new, for everything an unselected wire can end on.
    let mut remaps: Vec<(Point, Point)> = Vec::new();

    for component in document
        .components
        .iter_mut()
        .filter(|component| selection.has_component(component.id))
    {
        let before = terminal_points_for(component);
        transform.orient(component);
        component.pos = map(component.pos);
        remaps.extend(before.into_iter().zip(terminal_points_for(component)));
    }
    let moved_wire_points: Vec<Point> = document
        .wires
        .iter()
        .filter(|wire| selection.has_wire(wire.id))
        .flat_map(|wire| wire.points.iter().copied())
        .collect();
    for junction in document.junctions.iter_mut().filter(|junction| {
        selection.has_junction(junction.pos) || moved_wire_points.contains(&junction.pos)
    }) {
        junction.pos = map(junction.pos);
    }
    for wire in document
        .wires
        .iter_mut()
        .filter(|wire| selection.has_wire(wire.id))
    {
        for point in &mut wire.points {
            remaps.push((*point, map(*point)));
            *point = map(*point);
        }
    }
    for bus in document
        .buses
        .iter_mut()
        .filter(|bus| selection.has_bus(bus.id))
    {
        for point in &mut bus.points {
            *point = map(*point);
        }
    }
    for tap in document
        .bus_taps
        .iter_mut()
        .filter(|tap| selection.has_bus_tap(tap.id))
    {
        tap.bus_point = map(tap.bus_point);
        tap.connection_point = map(tap.connection_point);
    }
    for label in document
        .net_labels
        .iter_mut()
        .filter(|label| selection.has_net_label(label.id))
    {
        label.pos = map(label.pos);
    }
    for note in document
        .design_notes
        .iter_mut()
        .filter(|note| selection.has_design_note(note.id))
    {
        let to = map(note.pos);
        note.translate(to - note.pos);
    }
    for shape in document
        .documentation_shapes
        .iter_mut()
        .filter(|shape| selection.has_documentation_shape(shape.id))
    {
        map_shape_points(&mut shape.geometry, map);
    }
    for probe in document
        .probes
        .iter_mut()
        .filter(|probe| selection.has_probe(probe.id))
    {
        probe.position = map(probe.position);
    }

    remaps.retain(|(before, after)| before != after);
    follow_with_unselected_wires(document, selection, &remaps);
    true
}

fn selection_has_content(document: &SchematicDocument, selection: &Selection) -> bool {
    document
        .components
        .iter()
        .any(|component| selection.has_component(component.id))
        || document
            .wires
            .iter()
            .any(|wire| selection.has_wire(wire.id))
        || document
            .bus_taps
            .iter()
            .any(|tap| selection.has_bus_tap(tap.id))
        || document
            .net_labels
            .iter()
            .any(|label| selection.has_net_label(label.id))
        || document
            .design_notes
            .iter()
            .any(|note| selection.has_design_note(note.id))
        || document
            .documentation_shapes
            .iter()
            .any(|shape| selection.has_documentation_shape(shape.id))
        || document
            .probes
            .iter()
            .any(|probe| selection.has_probe(probe.id))
        || document.buses.iter().any(|bus| selection.has_bus(bus.id))
}

fn map_shape_points(geometry: &mut DocumentationShapeGeometry, map: impl Fn(Point) -> Point) {
    let points: Vec<&mut Point> = match geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => vec![first, opposite],
        DocumentationShapeGeometry::Line { start, end } => vec![start, end],
        DocumentationShapeGeometry::Polygon { points } => points.iter_mut().collect(),
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => vec![start, through, end],
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => vec![tip, elbow, box_corner],
    };
    for point in points {
        *point = map(*point);
    }
}

/// Carry every unselected wire point that sat on a moved position to where
/// that position went. A square wire is rerouted with square corners, as
/// Move reroutes it, and the part connections that index its points follow.
fn follow_with_unselected_wires(
    document: &mut SchematicDocument,
    selection: &Selection,
    remaps: &[(Point, Point)],
) {
    let moved_to = |point: &Point| {
        remaps
            .iter()
            .find(|(before, _)| before == point)
            .map(|(_, after)| *after)
    };
    for index in 0..document.wires.len() {
        let wire = &document.wires[index];
        if selection.has_wire(wire.id) {
            continue;
        }
        let followed: Vec<Point> = wire
            .points
            .iter()
            .map(|point| moved_to(point).unwrap_or(*point))
            .collect();
        if followed == wire.points {
            continue;
        }
        let wire_id = wire.id;
        let rerouted = wire
            .is_orthogonal()
            .then(|| orthogonal_route_for_corresponding_points(wire_id, &wire.points, &followed));
        match rerouted {
            Some(Ok((route, point_indices))) => {
                for connection in document
                    .connections
                    .iter_mut()
                    .filter(|connection| connection.wire_id == wire_id)
                {
                    if let Some(&moved) = point_indices.get(connection.point_index) {
                        connection.point_index = moved;
                    }
                }
                document.wires[index].points = route;
            }
            Some(Err(_)) | None => document.wires[index].points = followed,
        }
    }
}

/// The names the deck writes every loop probe drawn on this sheet under, which
/// are the voltage sources `.STB` can name: a probe drawn `LOOP1` is `VLOOP1`.
///
/// This reads the drawing, not an elaborated circuit, so it answers for
/// the sheet the engineer is looking at. A probe placed inside a
/// hierarchical cell is emitted under its flattened path and is
/// deliberately absent here rather than offered under a name the deck
/// will not contain; naming that probe by hand is what the entered form
/// of the field is for.
pub fn placed_loop_probe_names(document: &SchematicDocument) -> Vec<String> {
    let mut names: Vec<String> = document
        .components
        .iter()
        .filter(|component| component.kind == ComponentType::LoopProbe)
        .map(Component::written_element_name)
        .filter(|name| !name.is_empty())
        .collect();
    names.sort_by(|left, right| {
        left.to_ascii_uppercase()
            .cmp(&right.to_ascii_uppercase())
            .then_with(|| left.cmp(right))
    });
    names.dedup_by(|left, right| left.eq_ignore_ascii_case(right));
    names
}

/// The placed loop probe, out of [`placed_loop_probe_names`], that a
/// stability plan's chosen reference names.
///
/// A plan saved while the picker offered drawn names holds `LOOP1` for the
/// probe the deck writes `VLOOP1`. A loop probe is written by the prefix rule
/// alone, so either spelling names one probe. `None` when no placed probe
/// answers to the reference.
pub fn placed_loop_probe<'a>(placed: &'a [String], reference: &str) -> Option<&'a str> {
    let reference = reference.trim();
    let written = rspice_app_types::hierarchy_path::written_instance_name(
        ComponentType::LoopProbe.spice_prefix(),
        reference,
    );
    placed
        .iter()
        .find(|name| name.eq_ignore_ascii_case(reference))
        .or_else(|| {
            placed
                .iter()
                .find(|name| name.eq_ignore_ascii_case(&written))
        })
        .map(String::as_str)
}

fn next_library_reference_name(components: &[Component], prefix: &str) -> String {
    let prefix = prefix.trim().to_ascii_uppercase();
    (1_u64..)
        .map(|ordinal| format!("{prefix}{ordinal}"))
        .find(|candidate| {
            components
                .iter()
                .all(|component| !component.name.eq_ignore_ascii_case(candidate))
        })
        .expect("library reference-designator namespace is unbounded")
}

/// Terminal coordinates without an externally resolved symbol.
pub fn legacy_terminal_points(component: &Component) -> Vec<Point> {
    component
        .terminal_positions()
        .into_iter()
        .map(|(_, pos)| pos)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every pin of a part lands where the turn or flip takes that same pin,
    /// whatever orientation the part had, so a wire on a pin stays on it and
    /// a drain stays a drain.
    #[test]
    fn a_part_turns_and_flips_with_the_drawing() {
        let pivot = Point::new(30, -20);
        let transforms = [
            SelectionTransform::RotateClockwise,
            SelectionTransform::RotateCounterClockwise,
            SelectionTransform::MirrorHorizontal,
            SelectionTransform::MirrorVertical,
        ];
        for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
            for (mirror_h, mirror_v) in [(false, false), (true, false), (false, true)] {
                for transform in transforms {
                    let mut part = Component::new(1, ComponentType::Nmos, Point::new(100, 40));
                    part.rotation = rotation;
                    part.mirror_h = mirror_h;
                    part.mirror_v = mirror_v;
                    let expected: Vec<Point> = legacy_terminal_points(&part)
                        .into_iter()
                        .map(|pin| transform.apply(pin, pivot))
                        .collect();

                    transform.orient(&mut part);
                    part.pos = transform.apply(part.pos, pivot);

                    assert_eq!(
                        legacy_terminal_points(&part),
                        expected,
                        "{transform:?} of a part at {rotation:?}, mirrored {mirror_h}/{mirror_v}"
                    );
                }
            }
        }
    }
}
