//! Component placement, committed orientation edits, and terminal geometry.

use super::{
    component::{Component, LibraryCellInstance},
    component_type::ComponentType,
    document::SchematicDocument,
    identity::SchematicIdentity,
    rotation::Rotation,
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

/// Committed orientation changes supported by component editing.
#[derive(Debug, Clone, Copy)]
pub enum ComponentTransform {
    RotateClockwise,
    MirrorHorizontal,
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

/// Apply the requested edit and remap attached wire points once, in the supplied order.
pub fn transform_components_resolved(
    document: &mut SchematicDocument,
    ids: &[u64],
    terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    transform: ComponentTransform,
) {
    match transform {
        ComponentTransform::RotateClockwise => {
            transform_components_with(document, ids, terminal_points_for, |component| {
                component.rotation = component.rotation.rotate_cw()
            })
        }
        ComponentTransform::MirrorHorizontal => {
            transform_components_with(document, ids, terminal_points_for, |component| {
                component.toggle_mirror_h()
            })
        }
        ComponentTransform::MirrorVertical => {
            transform_components_with(document, ids, terminal_points_for, |component| {
                component.toggle_mirror_v()
            })
        }
    }
}

fn transform_components_with(
    document: &mut SchematicDocument,
    ids: &[u64],
    mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
    transform: impl Fn(&mut Component),
) {
    let before: Vec<(u64, Vec<Point>)> = ids
        .iter()
        .filter_map(|&id| {
            let component = document
                .components
                .iter()
                .find(|component| component.id == id)?;
            Some((id, terminal_points_for(component)))
        })
        .collect();

    for &id in ids {
        let Some(index) = document
            .components
            .iter()
            .position(|component| component.id == id)
        else {
            continue;
        };
        transform(&mut document.components[index]);
    }

    // Terminal order is positional and stable across transforms.
    // Build one old->new table for the whole selection, then apply it
    // once to the original wire state so selected components cannot
    // remap each other's freshly moved endpoints.
    let mut remaps: Vec<(Point, Point)> = Vec::new();
    for (id, before_points) in before {
        let Some(component) = document
            .components
            .iter()
            .find(|component| component.id == id)
        else {
            continue;
        };
        let after_points = terminal_points_for(component);
        for (old_pos, new_pos) in before_points.into_iter().zip(after_points) {
            if old_pos != new_pos {
                remaps.push((old_pos, new_pos));
            }
        }
    }

    let mut updates: Vec<(usize, usize, Point)> = Vec::new();
    for (wire_index, wire) in document.wires.iter().enumerate() {
        for (point_index, point) in wire.points.iter().enumerate() {
            if let Some((_, new_pos)) = remaps.iter().find(|(old_pos, _)| point == old_pos) {
                updates.push((wire_index, point_index, *new_pos));
            }
        }
    }

    for (wire_index, point_index, new_pos) in updates {
        document.wires[wire_index].points[point_index] = new_pos;
    }
}

/// The emitted names of every loop probe drawn on this sheet, in the
/// spelling the deck will carry.
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
        .map(Component::spice_instance_name)
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
