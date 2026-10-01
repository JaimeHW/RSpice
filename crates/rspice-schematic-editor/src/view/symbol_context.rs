//! Resolved symbol geometry and selection queries over a borrowed design view.

use super::geometry::{point_in_rect, rects_intersect, segment_intersects_rect};
use super::resolved_symbol_render::resolved_symbol_world_bounds;
use super::{design_notes, documentation_shapes, net_labels};
use rspice_design::{
    resolved_symbol::{ResolvedCellSymbol, ResolvedSymbolSource},
    schematic::{
        component::{Component, LibraryCellInstance},
        component_type::ComponentType,
        document::SchematicDocument,
        selection::Selection,
    },
    symbol_resolver::SymbolResolver,
};
use rspice_design_model::Point;
use std::collections::HashMap;

#[derive(Default)]
pub struct SchematicSymbolContext {
    resolved_by_component_id: HashMap<u64, ResolvedCellSymbol>,
    resolved_by_binding: Vec<(LibraryCellInstance, ResolvedCellSymbol)>,
    pending_library_symbol: Option<ResolvedCellSymbol>,
    revision: u64,
}

impl SchematicSymbolContext {
    /// Resolve authored instance and placement symbols against one design view.
    /// `revision` is the caller's cache revision for these library/buffer inputs.
    pub fn new<S: AsRef<SchematicDocument>>(
        document: &SchematicDocument,
        pending_library_cell: Option<&LibraryCellInstance>,
        resolver: &SymbolResolver<'_, S>,
        revision: u64,
    ) -> Self {
        let mut resolved_by_component_id = HashMap::new();
        let mut resolved_by_binding = Vec::new();
        for component in document
            .components
            .iter()
            .filter(|component| component.kind == ComponentType::CellInstance)
        {
            let Some(binding) = component.library_cell.as_ref() else {
                continue;
            };
            let Some(resolved) = resolver
                .resolve_binding(binding)
                .filter(|symbol| symbol.source() == ResolvedSymbolSource::Authored)
            else {
                continue;
            };
            resolved_by_component_id.insert(component.id, resolved.clone());
            if !resolved_by_binding
                .iter()
                .any(|(candidate, _)| candidate == binding)
            {
                resolved_by_binding.push((binding.clone(), resolved));
            }
        }
        let pending_library_symbol = pending_library_cell
            .and_then(|binding| resolver.resolve_binding(binding))
            .filter(|symbol| symbol.source() == ResolvedSymbolSource::Authored);

        Self {
            resolved_by_component_id,
            resolved_by_binding,
            pending_library_symbol,
            revision,
        }
    }

    pub fn resolved_symbol(&self, component: &Component) -> Option<&ResolvedCellSymbol> {
        self.resolved_by_component_id
            .get(&component.id)
            .or_else(|| {
                let binding = component.library_cell.as_ref()?;
                self.resolved_by_binding
                    .iter()
                    .find_map(|(candidate, symbol)| (candidate == binding).then_some(symbol))
            })
    }

    pub fn pending_library_symbol(&self) -> Option<&ResolvedCellSymbol> {
        self.pending_library_symbol.as_ref()
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn terminal_points(&self, component: &Component) -> Vec<Point> {
        component
            .terminal_positions_resolved(self.resolved_symbol(component))
            .into_iter()
            .map(|(_, position)| position)
            .collect()
    }

    pub fn named_terminal_points(&self, component: &Component) -> Vec<(String, Point)> {
        component
            .terminal_positions_resolved(self.resolved_symbol(component))
            .into_iter()
            .map(|(name, position)| (name.to_owned(), position))
            .collect()
    }

    /// Authoritative world bounds of the rendered instance, including custom
    /// symbol artwork. Geometry editors use this same extent so validation can
    /// never route through shapes that the user can see on the canvas.
    pub fn component_bounds_tuple(&self, component: &Component) -> (i32, i32, i32, i32) {
        let (min, max) = self.component_bounds(component);
        (min.x, min.y, max.x, max.y)
    }

    pub fn component_at_resolved_terminal(
        &self,
        components: &[Component],
        pos: Point,
    ) -> Option<u64> {
        components
            .iter()
            .find(|component| self.terminal_points(component).contains(&pos))
            .map(|component| component.id)
    }

    pub fn component_at_resolved_symbol(
        &self,
        components: &[Component],
        pos: Point,
    ) -> Option<u64> {
        self.component_at_resolved_terminal(components, pos)
            .or_else(|| {
                components
                    .iter()
                    .map(|component| (component.id, self.component_bounds(component)))
                    .find(|(_, (min, max))| {
                        pos.x >= min.x && pos.x <= max.x && pos.y >= min.y && pos.y <= max.y
                    })
                    .map(|(id, _)| id)
            })
    }

    pub fn component_bounds(&self, component: &Component) -> (Point, Point) {
        if let Some(symbol) = self.resolved_symbol(component)
            && let Some(bounds) = resolved_symbol_world_bounds(component, symbol)
        {
            return bounds;
        }
        let (min_x, min_y, max_x, max_y) = component.bounding_box();
        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }

    pub fn content_bounds(&self, document: &SchematicDocument) -> Option<(i32, i32, i32, i32)> {
        if document.components.is_empty()
            && document.wires.is_empty()
            && document.buses.is_empty()
            && document.bus_taps.is_empty()
            && document.junctions.is_empty()
            && document.net_labels.is_empty()
            && document.design_notes.is_empty()
            && document.documentation_shapes.is_empty()
            && document.probes.is_empty()
        {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut include = |min: Point, max: Point| {
            min_x = min_x.min(min.x);
            min_y = min_y.min(min.y);
            max_x = max_x.max(max.x);
            max_y = max_y.max(max.y);
        };

        for component in &document.components {
            let (min, max) = self.component_bounds(component);
            include(min, max);
        }

        for wire in &document.wires {
            for point in &wire.points {
                include(*point, *point);
            }
        }

        for bus in &document.buses {
            for point in &bus.points {
                include(*point, *point);
            }
        }

        for tap in &document.bus_taps {
            for point in crate::bus_geometry::bus_tap_route_points(tap) {
                include(point, point);
            }
        }

        for junction in &document.junctions {
            include(junction.pos, junction.pos);
        }

        for label in &document.net_labels {
            let (min, max) = net_labels::world_bounds(label);
            include(min, max);
        }

        for note in &document.design_notes {
            let (min, max) = design_notes::conservative_world_bounds(note);
            include(min, max);
        }

        for shape in &document.documentation_shapes {
            let (min, max) = documentation_shapes::world_bounds(shape);
            include(min, max);
        }

        for probe in &document.probes {
            let (min, max) = probe.world_bounds();
            include(min, max);
        }

        Some((min_x, min_y, max_x, max_y))
    }

    pub fn select_in_rect(
        &self,
        document: &SchematicDocument,
        selection: &mut Selection,
        window: SelectionWindow,
        add_to_selection: bool,
    ) -> usize {
        let SelectionWindow {
            min_x,
            min_y,
            max_x,
            max_y,
            enclosed_only,
        } = window;
        if !add_to_selection {
            selection.clear();
        }

        let mut count = 0;

        for component in &document.components {
            let (min, max) = self.component_bounds(component);
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !selection.has_component(component.id) {
                selection.select_component(component.id);
                count += 1;
            }
        }

        for wire in &document.wires {
            let wire_in_rect = if enclosed_only {
                wire.points
                    .iter()
                    .all(|point| point_in_rect(*point, min_x, min_y, max_x, max_y))
            } else {
                wire.points.windows(2).any(|points| {
                    segment_intersects_rect(points[0], points[1], min_x, min_y, max_x, max_y)
                })
            };
            if wire_in_rect && !selection.has_wire(wire.id) {
                selection.select_wire(wire.id);
                count += 1;
            }
        }

        for bus in &document.buses {
            let bus_in_rect = if enclosed_only {
                bus.points
                    .iter()
                    .all(|point| point_in_rect(*point, min_x, min_y, max_x, max_y))
            } else {
                bus.points.windows(2).any(|points| {
                    segment_intersects_rect(points[0], points[1], min_x, min_y, max_x, max_y)
                })
            };
            if bus_in_rect && !selection.has_bus(bus.id) {
                selection.select_bus(bus.id);
                count += 1;
            }
        }

        for tap in &document.bus_taps {
            let route = crate::bus_geometry::bus_tap_route_points(tap);
            let tap_in_rect = if enclosed_only {
                route
                    .iter()
                    .all(|point| point_in_rect(*point, min_x, min_y, max_x, max_y))
            } else {
                route.windows(2).any(|segment| {
                    segment_intersects_rect(segment[0], segment[1], min_x, min_y, max_x, max_y)
                })
            };
            if tap_in_rect && !selection.has_bus_tap(tap.id) {
                selection.select_bus_tap(tap.id);
                count += 1;
            }
        }

        for junction in &document.junctions {
            if point_in_rect(junction.pos, min_x, min_y, max_x, max_y)
                && !selection.has_junction(junction.pos)
            {
                selection.select_junction(junction.pos);
                count += 1;
            }
        }

        for label in &document.net_labels {
            let (min, max) = net_labels::world_bounds(label);
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !selection.has_net_label(label.id) {
                selection.net_labels.insert(label.id);
                count += 1;
            }
        }

        for note in &document.design_notes {
            let (min, max) = design_notes::conservative_world_bounds(note);
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !selection.has_design_note(note.id) {
                selection.select_design_note(note.id);
                count += 1;
            }
        }

        for shape in &document.documentation_shapes {
            let matches = documentation_shapes::shape_intersects_rect(
                shape,
                min_x,
                min_y,
                max_x,
                max_y,
                enclosed_only,
            );
            if matches && !selection.has_documentation_shape(shape.id) {
                selection.select_documentation_shape(shape.id);
                count += 1;
            }
        }

        for probe in &document.probes {
            let (min, max) = probe.world_bounds();
            let matches = if enclosed_only {
                rect_contains_rect(min, max, min_x, min_y, max_x, max_y)
            } else {
                rects_intersect(min, max, min_x, min_y, max_x, max_y)
            };
            if matches && !selection.has_probe(probe.id) {
                selection.select_probe(probe.id);
                count += 1;
            }
        }

        count
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionWindow {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
    enclosed_only: bool,
}

impl SelectionWindow {
    pub const fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32, enclosed_only: bool) -> Self {
        Self {
            min_x,
            min_y,
            max_x,
            max_y,
            enclosed_only,
        }
    }
}

fn rect_contains_rect(
    min: Point,
    max: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    min.x >= min_x && min.y >= min_y && max.x <= max_x && max.y <= max_y
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::{
        design_note::{DesignNote, DesignNoteKind},
        net_label::NetLabel,
        wire::Wire,
    };
    use rspice_design::symbol::{SymbolDocument, SymbolPin, SymbolShape};
    use rspice_design_model::port::{PortDirection, PortSpec};

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    #[test]
    fn component_at_resolved_symbol_hits_authored_body_outside_generic_bounds() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
        };

        assert_eq!(
            context.component_at_resolved_symbol(&[component], Point::new(200, 50)),
            Some(1)
        );
    }

    #[test]
    fn content_bounds_include_authored_symbol_body() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
        };
        let mut document = SchematicDocument::default();
        document.components.push(component);

        assert_eq!(context.content_bounds(&document), Some((80, 10, 220, 90)));
    }

    #[test]
    fn content_bounds_and_marquee_selection_include_net_label_text() {
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        let label = NetLabel::new(77, Point::new(100, 80), "afe_out");
        let (min, max) = net_labels::world_bounds(&label);
        document.net_labels.push(label);
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.content_bounds(&document),
            Some((min.x, min.y, max.x, max.y))
        );
        assert_eq!(
            context.select_in_rect(
                &document,
                &mut selection,
                SelectionWindow::new(min.x + 2, min.y + 2, max.x - 2, max.y - 2, false),
                false,
            ),
            1
        );
        assert_eq!(selection.single_net_label(), Some(77));
    }

    #[test]
    fn content_bounds_and_marquee_selection_include_design_note_text() {
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        let note = DesignNote::new(
            78,
            Point::new(100, 80),
            DesignNoteKind::PlainText,
            "Bias network\nKeep clear",
        )
        .unwrap();
        let (min, max) = design_notes::conservative_world_bounds(&note);
        document.design_notes.push(note);
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.content_bounds(&document),
            Some((min.x, min.y, max.x, max.y))
        );
        assert_eq!(
            context.select_in_rect(
                &document,
                &mut selection,
                SelectionWindow::new(min.x, min.y, max.x, max.y, false),
                false,
            ),
            1
        );
        assert_eq!(selection.single_design_note(), Some(78));
    }

    #[test]
    fn select_in_rect_commits_authored_body_intersections() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            resolved_by_binding: Vec::new(),
            pending_library_symbol: None,
            revision: 0,
        };
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        document.components.push(component);

        let selected = context.select_in_rect(
            &document,
            &mut selection,
            SelectionWindow::new(190, 40, 210, 60, false),
            false,
        );

        assert_eq!(selected, 1);
        assert!(selection.has_component(1));
    }

    #[test]
    fn enclosed_selection_rejects_partial_component_intersections() {
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        document.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(100, 50),
        ));
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.select_in_rect(
                &document,
                &mut selection,
                SelectionWindow::new(95, 45, 105, 55, true),
                false,
            ),
            0
        );
        assert!(!selection.has_component(1));
    }

    #[test]
    fn intersecting_selection_detects_wire_crossing_without_an_inside_vertex() {
        let mut document = SchematicDocument::default();
        let mut selection = Selection::default();
        document
            .wires
            .push(Wire::new(1, vec![Point::new(0, 50), Point::new(100, 50)]));
        let wire_id = document.wires[0].id;
        let context = SchematicSymbolContext::default();

        assert_eq!(
            context.select_in_rect(
                &document,
                &mut selection,
                SelectionWindow::new(40, 40, 60, 60, false),
                false,
            ),
            1
        );
        assert!(selection.has_wire(wire_id));
    }
}
