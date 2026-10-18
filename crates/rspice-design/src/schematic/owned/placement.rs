//! Typed placement and orientation changes on the live schematic owner.
use super::super::{
    component::{Component, LibraryCellInstance},
    component_edit::{self, ComponentModelOverride, ComponentPlacement, SelectionTransform},
    component_type::ComponentType,
    junction_edit,
    net_label::NetLabelKind,
    selection::Selection,
};
use super::Schematic;
use rspice_design_model::Point;

impl Schematic {
    pub fn add_component(
        &mut self,
        kind: ComponentType,
        placement: ComponentPlacement,
        model: Option<ComponentModelOverride<'_>>,
    ) -> u64 {
        let id = component_edit::add_component(
            &mut self.document,
            &mut self.identity,
            kind,
            placement,
            model,
        );
        self.invalidate_topology();
        id
    }
    pub fn add_library_cell_component(
        &mut self,
        placement: ComponentPlacement,
        binding: LibraryCellInstance,
    ) -> u64 {
        let id = component_edit::add_library_cell_component(
            &mut self.document,
            &mut self.identity,
            placement,
            binding,
        );
        self.invalidate_topology();
        id
    }
    pub fn transform_selection(
        &mut self,
        selection: &Selection,
        pivot: Point,
        terminal_points_for: impl FnMut(&Component) -> Vec<Point>,
        transform: SelectionTransform,
    ) -> bool {
        let changed = component_edit::transform_selection(
            &mut self.document,
            selection,
            pivot,
            terminal_points_for,
            transform,
        );
        if changed {
            self.invalidate_topology();
        }
        changed
    }
    pub fn add_junction(&mut self, pos: Point) -> (u64, bool) {
        let (id, inserted) =
            junction_edit::add_junction(&mut self.document, &mut self.identity, pos);
        if inserted {
            self.invalidate_topology();
        }
        (id, inserted)
    }
    pub fn remove_junction(&mut self, id: u64) -> bool {
        let removed = junction_edit::remove_junction(&mut self.document, id);
        if removed {
            self.invalidate_topology();
        }
        removed
    }
    pub fn add_net_label(&mut self, pos: Point, name: String) -> u64 {
        let id = junction_edit::add_net_label(&mut self.document, &mut self.identity, pos, name);
        self.invalidate_topology();
        id
    }
    pub fn add_net_label_with_kind(&mut self, pos: Point, name: String, kind: NetLabelKind) -> u64 {
        let id = self.add_net_label(pos, name);
        if let Some(label) = self
            .document
            .net_labels
            .iter_mut()
            .find(|label| label.id == id)
        {
            label.kind = kind;
        }
        id
    }
    pub fn auto_place_junctions(&mut self) -> bool {
        let changed = junction_edit::auto_place_junctions(&mut self.document, &mut self.identity);
        if changed {
            self.invalidate_topology();
        }
        changed
    }
    pub fn remove_orphan_junctions(&mut self) -> usize {
        let removed = junction_edit::remove_orphan_junctions(&mut self.document);
        if removed > 0 {
            self.invalidate_topology();
        }
        removed
    }
    /// [`Self::remove_orphan_junctions`] then [`Self::auto_place_junctions`],
    /// with the junction rule built once. Returns how many dots went and
    /// whether any were added.
    pub fn update_wire_junctions(&mut self) -> (usize, bool) {
        let (removed, placed) =
            junction_edit::update_junctions(&mut self.document, &mut self.identity);
        if removed > 0 {
            self.invalidate_topology();
        }
        if placed {
            self.invalidate_topology();
        }
        (removed, placed)
    }
}
