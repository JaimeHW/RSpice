//! Array command coordination, selection and undo over document-owned edits.

#[cfg(test)]
use super::super::super::{BusSlice, SchematicArrayKind, SchematicArrayPlacement};
use super::super::super::{
    SchematicArrayCount, SchematicArrayError, SchematicArrayImpact, SchematicArrayNaming,
    SchematicArrayPlan, SchematicArrayPreview,
    array::ArrayObjectIds,
    array_edit::{ArraySelection, ArraySource},
};
use super::super::*;
use rspice_design::schematic::clipboard_edit::CopySelection;
#[cfg(test)]
use std::collections::HashSet;

fn array_selection_input(
    selection: &Selection,
) -> ArraySelection<'_, impl Iterator<Item = Point> + Clone> {
    ArraySelection {
        objects: CopySelection {
            components: &selection.components,
            wires: &selection.wires,
            buses: &selection.buses,
            bus_taps: &selection.bus_taps,
            net_labels: &selection.net_labels,
            design_notes: &selection.design_notes,
            documentation_shapes: &selection.documentation_shapes,
            probes: &selection.probes,
        },
        junctions: selection.junctions.iter().map(|junction| junction.pos),
        has_partial_objects: !selection.wire_segments.is_empty()
            || !selection.wire_vertices.is_empty(),
    }
}

fn array_result_selection(objects: ArrayObjectIds) -> Selection {
    Selection {
        components: objects.components,
        wires: objects.wires,
        buses: objects.buses,
        bus_taps: objects.bus_taps,
        net_labels: objects.net_labels,
        design_notes: objects.design_notes,
        documentation_shapes: objects.documentation_shapes,
        junctions: objects
            .junctions
            .into_iter()
            .map(super::super::super::JunctionSelection::new)
            .collect(),
        ..Selection::default()
    }
}

impl SchematicState {
    fn array_source(&self) -> ArraySource<'_, impl Iterator<Item = Point> + Clone> {
        ArraySource {
            document: &self.design.document(),
            identity_cursor: self.identity_cursor(),
            selection: array_selection_input(&self.selection),
        }
    }

    pub fn has_live_array_selection(&self) -> bool {
        self.array_source().has_live_array_selection()
    }
    pub fn validate_array_source_selection(&self) -> Result<(), SchematicArrayError> {
        self.array_source().validate_array_source_selection()
    }
    pub fn default_array_naming(
        &self,
        count: SchematicArrayCount,
    ) -> Result<SchematicArrayNaming, SchematicArrayError> {
        self.array_source().default_array_naming(count)
    }
    pub fn preview_array_selection(
        &self,
        plan: &SchematicArrayPlan,
    ) -> Result<SchematicArrayPreview, SchematicArrayError> {
        self.preview_array_selection_resolved(
            plan,
            |component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(|(name, point)| (name.to_owned(), point))
                    .collect()
            },
            Component::bounding_box,
        )
    }
    pub fn array_selection(
        &mut self,
        plan: &SchematicArrayPlan,
    ) -> Result<SchematicArrayImpact, SchematicArrayError> {
        self.array_selection_resolved(
            plan,
            |component| {
                component
                    .terminal_positions()
                    .into_iter()
                    .map(|(name, point)| (name.to_owned(), point))
                    .collect()
            },
            Component::bounding_box,
        )
    }

    pub fn preview_array_selection_resolved(
        &self,
        plan: &SchematicArrayPlan,
        terminal_points_for: impl FnMut(&Component) -> Vec<(String, Point)>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<SchematicArrayPreview, SchematicArrayError> {
        if self.read_only {
            return Err(SchematicArrayError::ReadOnly);
        }
        self.array_source().preview_array_selection_resolved(
            plan,
            terminal_points_for,
            component_bounds_for,
        )
    }

    pub fn array_selection_resolved(
        &mut self,
        plan: &SchematicArrayPlan,
        terminal_points_for: impl FnMut(&Component) -> Vec<(String, Point)>,
        component_bounds_for: impl FnMut(&Component) -> (i32, i32, i32, i32),
    ) -> Result<SchematicArrayImpact, SchematicArrayError> {
        if self.read_only {
            return Err(SchematicArrayError::ReadOnly);
        }
        let edit = self.design.array_selection_resolved(
            array_selection_input(&self.selection),
            plan,
            terminal_points_for,
            component_bounds_for,
        )?;
        let (impact, objects) = edit.value;
        self.selection = array_result_selection(objects);
        self.finish_document_edit(edit.committed);
        if !edit.committed {
            return Err(SchematicArrayError::CommitFailed);
        }
        Ok(impact)
    }
}

#[cfg(test)]
mod tests;
