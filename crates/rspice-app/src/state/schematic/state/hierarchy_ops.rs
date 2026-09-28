//! Editor state around headless hierarchy candidates.
use super::super::{
    hierarchy::{
        HierarchyExtractionCandidate, HierarchyExtractionPlan, HierarchyExtractionTerminal,
        HierarchyNetConnectivity,
    },
    hierarchy_edit::HierarchySource,
};
use super::*;
use std::collections::HashMap;

impl SchematicState {
    fn hierarchy_source(&self) -> HierarchySource<'_> {
        self.design
            .hierarchy_source(&self.selection.components, self.selection.count())
    }

    pub fn plan_hierarchy_extraction(
        &self,
        terminals: &[HierarchyExtractionTerminal],
        connectivity: &HierarchyNetConnectivity,
        component_bounds: &HashMap<u64, (i32, i32, i32, i32)>,
    ) -> Result<HierarchyExtractionPlan, super::super::hierarchy::HierarchyExtractionError> {
        if self.read_only {
            return Err(super::super::hierarchy::HierarchyExtractionError::ReadOnly);
        }
        self.hierarchy_source()
            .plan_hierarchy_extraction(terminals, connectivity, component_bounds)
    }

    pub fn materialize_hierarchy_extraction(
        &self,
        plan: &HierarchyExtractionPlan,
        library: &str,
        cell: &str,
        view: &str,
    ) -> Result<HierarchyExtractionCandidate, super::super::hierarchy::HierarchyExtractionError>
    {
        if self.read_only {
            return Err(super::super::hierarchy::HierarchyExtractionError::ReadOnly);
        }
        let candidate = self.hierarchy_source().materialize_hierarchy_extraction(
            plan,
            library,
            cell,
            view,
            self.preview_rotation,
            self.preview_mirror_h,
        )?;
        let mut parent =
            self.clone_with_design(self.design.clone_with_hierarchy_document(candidate.parent));
        parent.is_dirty = true;
        parent
            .selection
            .select_only_component(candidate.instance_id);
        parent.repair_clipboard_after_load();
        let mut child = Self::default();
        child.design =
            rspice_design::schematic::owned::Schematic::from_hierarchy_document(candidate.child);
        child.is_dirty = true;
        Ok(HierarchyExtractionCandidate {
            parent,
            child,
            instance_id: candidate.instance_id,
            instance_name: candidate.instance_name,
            binding: candidate.binding,
        })
    }

    // The document was already cloned by its owner. Clone only editor/session
    // fields here so candidate materialization does not copy that document twice.
    pub(crate) fn clone_with_design(
        &self,
        design: rspice_design::schematic::owned::Schematic,
    ) -> Self {
        Self {
            design,
            selection: self.selection.clone(),
            tool: self.tool,
            wire_drawing: self.wire_drawing.clone(),
            bus_drawing: self.bus_drawing.clone(),
            zoom: self.zoom,
            pan: self.pan,
            current_file: self.current_file.clone(),
            clipboard: self.clipboard.clone(),
            preview_rotation: self.preview_rotation,
            preview_mirror_h: self.preview_mirror_h,
            pending_library_cell: self.pending_library_cell.clone(),
            pending_part_model: self.pending_part_model.clone(),
            pending_stimulus: self.pending_stimulus.clone(),
            pending_bus_tap: self.pending_bus_tap.clone(),
            pending_port_sequence: self.pending_port_sequence.clone(),
            pending_design_note: self.pending_design_note.clone(),
            pending_documentation_shape: self.pending_documentation_shape.clone(),
            documentation_shape_drawing: self.documentation_shape_drawing.clone(),
            is_dirty: self.is_dirty,
            needs_fit: self.needs_fit,
            needs_drawing_sheet_fit: self.needs_drawing_sheet_fit,
            center_request: self.center_request,
            read_only: self.read_only,
            needs_history_reset: self.needs_history_reset,
            snap_engine: self.snap_engine.clone(),
            selection_rect: self.selection_rect,
            net_highlight: self.net_highlight.clone(),
            operation_cancel: self.operation_cancel.clone(),
            canvas_cache: self.canvas_cache.clone(),
        }
    }
}
