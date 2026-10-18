//! Placing and removing components.

use super::*;
#[cfg(test)]
use rspice_design::schematic::component_edit::legacy_terminal_points;
use rspice_design::schematic::component_edit::{
    self, ComponentModelOverride, ComponentPlacement, SelectionTransform,
};

impl SchematicState {
    // =========================================================================
    // Component Management
    // =========================================================================

    /// Add a component at the given position
    pub fn add_component(&mut self, kind: ComponentType, pos: Point) -> u64 {
        // A pending card applies only while its exact device tool is armed.
        let model_override = self
            .session
            .editor
            .pending_part_model
            .as_ref()
            .filter(|armed| armed.tool == Tool::Place(kind))
            .map(|armed| ComponentModelOverride {
                model: &armed.model,
                symbol_variant: armed.variant.as_deref(),
            });
        let id = self.design.add_component(
            kind,
            ComponentPlacement {
                position: pos,
                rotation: self.session.editor.preview_rotation,
                mirror_h: self.session.editor.preview_mirror_h,
            },
            model_override,
        );
        self.session.is_dirty = true;
        id
    }

    /// Add a generic library/cell/view instance at the given position.
    pub fn add_library_cell_component(
        &mut self,
        pos: Point,
        library_cell: LibraryCellInstance,
    ) -> u64 {
        let id = self.design.add_library_cell_component(
            ComponentPlacement {
                position: pos,
                rotation: self.session.editor.preview_rotation,
                mirror_h: self.session.editor.preview_mirror_h,
            },
            library_cell,
        );
        self.session.is_dirty = true;
        id
    }

    /// Turn or flip the whole selection about `pivot`. Unselected wires that
    /// end on what moved follow it with square corners, a pin turned off a
    /// pin it touched gets a square wire back to it, a pin that lands inside
    /// a wire splits it, and the junction policy tidies the result. The
    /// caller owns the undo step and opens it before the sheet scope, so the
    /// step's snapshot holds every sheet and says where the pins stood.
    /// Returns whether anything moved.
    pub fn transform_selection(
        &mut self,
        transform: SelectionTransform,
        pivot: Point,
        mut terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        scope: &ActiveSheetTopology,
    ) -> bool {
        if self.session.read_only {
            return false;
        }
        let selection = self.session.editor.selection.clone();
        if !self
            .design
            .transform_selection(&selection, pivot, &mut terminal_points_for, transform)
        {
            return false;
        }
        self.session.is_dirty = true;
        let automatic_junctions = self.session.editor.automatic_junctions;
        self.connect_moved_pins(&selection.components, &mut terminal_points_for, scope);
        self.cleanup_wire_topology_with_junction_policy(automatic_junctions, scope);
        true
    }

    /// Emitted loop-probe names in the current sheet.
    pub fn placed_loop_probe_names(&self) -> Vec<String> {
        component_edit::placed_loop_probe_names(self.design.document())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Cell, Library, LibraryManager, PortDirection, PortSpec, ResolvedCellSymbol, SymbolDocument,
        SymbolPin, SymbolResolver, View, ViewType, Wire,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    fn resolved_amp_symbol() -> ResolvedCellSymbol {
        let document = SymbolDocument {
            pins: vec![
                SymbolPin::new("OUT", PortDirection::Out, Some(Point::new(70, 20))),
                SymbolPin::new("IN", PortDirection::In, Some(Point::new(-40, -10))),
            ],
            ..SymbolDocument::default()
        };

        let mut libraries = LibraryManager::new();
        let mut library = Library::new("work");
        let mut cell = Cell::new("amp");
        let mut symbol_view = View::new("symbol", ViewType::Symbol);
        document
            .store_in_view(&mut symbol_view)
            .expect("symbol stores");
        cell.add_view(symbol_view);
        library.add_cell(cell);
        libraries.add_library(library);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        SymbolResolver::<rspice_design::schematic::owned::Schematic>::new(
            &libraries,
            &HashMap::new(),
        )
        .resolve_binding(&binding)
        .expect("symbol resolves")
    }

    fn resolved_terminal_points(
        component: &Component,
        resolved: &ResolvedCellSymbol,
    ) -> Vec<Point> {
        component
            .terminal_positions_resolved(Some(resolved))
            .into_iter()
            .map(|(_, pos)| pos)
            .collect()
    }

    fn select_ids_in_iteration_order(schematic: &mut SchematicState) -> (u64, u64) {
        for first in 1..200 {
            for second in 1..200 {
                if first == second {
                    continue;
                }
                schematic.session.editor.selection.components.clear();
                schematic.session.editor.selection.select_component(first);
                schematic.session.editor.selection.select_component(second);
                let order: Vec<u64> = schematic
                    .session
                    .editor
                    .selection
                    .components
                    .iter()
                    .copied()
                    .collect();
                if order == [first, second] {
                    return (first, second);
                }
            }
        }
        panic!("could not find deterministic selected-component order");
    }

    #[test]
    fn rotating_selected_cell_uses_resolved_symbol_terminals_for_wire_updates() {
        let resolved = resolved_amp_symbol();
        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);

        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().components.push(
            Component::new(1, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding),
        );
        schematic
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(2, Point::new(60, 40), Point::new(60, 0)));
        schematic.session.editor.selection.select_component(1);

        schematic.with_single_sheet_topology(|schematic, scope| {
            schematic.transform_selection(
                SelectionTransform::RotateClockwise,
                Point::new(100, 50),
                |component| resolved_terminal_points(component, &resolved),
                scope,
            )
        });

        assert_eq!(
            schematic.design.document().components[0].rotation,
            Rotation::R90
        );
        // The wire follows the pin with square corners (audit W-14).
        let points = &schematic.design.document().wires[0].points;
        assert_eq!(points.first(), Some(&Point::new(110, 10)));
        assert_eq!(points.last(), Some(&Point::new(60, 0)));
        assert!(
            points
                .windows(2)
                .all(|pair| pair[0].x == pair[1].x || pair[0].y == pair[1].y)
        );
    }

    #[test]
    fn rotating_multiple_selected_cells_applies_wire_remap_once() {
        let resolved = resolved_amp_symbol();
        let mut schematic = SchematicState::default();
        let (first_id, second_id) = select_ids_in_iteration_order(&mut schematic);

        let mut binding = LibraryCellInstance::new("work", "amp", "schematic");
        binding.bind_interface(&[
            port("IN", PortDirection::In),
            port("OUT", PortDirection::Out),
        ]);
        schematic.design.document_mut_for_test().components.push(
            Component::new(first_id, ComponentType::CellInstance, Point::new(100, 50))
                .with_library_cell(binding.clone()),
        );
        schematic.design.document_mut_for_test().components.push(
            Component::new(second_id, ComponentType::CellInstance, Point::new(150, 20))
                .with_library_cell(binding),
        );
        schematic
            .design
            .document_mut_for_test()
            .wires
            .push(Wire::segment(9, Point::new(60, 40), Point::new(60, 0)));

        schematic.with_single_sheet_topology(|schematic, scope| {
            schematic.transform_selection(
                SelectionTransform::RotateClockwise,
                Point::new(100, 50),
                |component| resolved_terminal_points(component, &resolved),
                scope,
            )
        });

        assert_eq!(
            schematic.design.document().wires[0].points[0],
            Point::new(110, 10),
            "wire endpoint should follow the first component's pin once, not then match and follow \
             the second component's old pin"
        );
    }

    #[test]
    fn rotating_stale_component_selection_is_noop() {
        let mut schematic = SchematicState::default();
        schematic.session.editor.selection.select_component(404);
        let topology_version = schematic.topology_version();

        assert!(!schematic.with_single_sheet_topology(|schematic, scope| {
            schematic.transform_selection(
                SelectionTransform::RotateClockwise,
                Point::origin(),
                legacy_terminal_points,
                scope,
            )
        }));

        assert!(!schematic.session.is_dirty);
        assert_eq!(schematic.topology_version(), topology_version);
        assert!(schematic.design.document().components.is_empty());
    }

    #[test]
    fn model_bound_reference_prefix_is_unique_and_invalid_prefix_fails_closed() {
        let mut schematic = SchematicState::default();
        let mut model = LibraryCellInstance::new("models", "nmos_18", "spice");
        model.reference_prefix = Some("m".to_owned());

        let first = schematic.add_library_cell_component(Point::origin(), model.clone());
        let second = schematic.add_library_cell_component(Point::new(40, 0), model);
        let mut invalid = LibraryCellInstance::new("models", "unsafe", "spice");
        invalid.reference_prefix = Some("M;drop".to_owned());
        let third = schematic.add_library_cell_component(Point::new(80, 0), invalid);

        let name = |id| {
            schematic
                .design
                .document()
                .components
                .iter()
                .find(|component| component.id == id)
                .map(|component| component.name.as_str())
                .expect("placed component")
        };
        assert_eq!(name(first), "M1");
        assert_eq!(name(second), "M2");
        assert_eq!(name(third), "X1");
    }
}
