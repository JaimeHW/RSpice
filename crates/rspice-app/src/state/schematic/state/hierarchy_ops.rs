//! Editor state around headless hierarchy candidates.
use super::super::{
    hierarchy::{
        HierarchyExtractionCandidate, HierarchyExtractionPlan, HierarchyExtractionTerminal,
        HierarchyNetConnectivity,
    },
    hierarchy_edit::{HierarchyDocument, HierarchySource},
};
use super::*;
use std::collections::HashMap;

impl SchematicState {
    fn hierarchy_source(&self) -> HierarchySource<'_> {
        HierarchySource {
            document: &self.document,
            identity: &self.identity,
            topology_version: self.topology_version(),
            selected_components: &self.selection.components,
            selected_count: self.selection.count(),
        }
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
        let mut parent = self.clone_with_hierarchy_document(candidate.parent);
        parent
            .selection
            .select_only_component(candidate.instance_id);
        parent.repair_clipboard_after_load();
        let mut child = Self::default();
        child.document = candidate.child.document;
        child.identity = candidate.child.identity;
        child.topology_version = candidate.child.topology_changes;
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
    fn clone_with_hierarchy_document(&self, candidate: HierarchyDocument) -> Self {
        Self {
            document: candidate.document,
            selection: self.selection.clone(),
            tool: self.tool,
            wire_drawing: self.wire_drawing.clone(),
            bus_drawing: self.bus_drawing.clone(),
            zoom: self.zoom,
            pan: self.pan,
            current_file: self.current_file.clone(),
            identity: candidate.identity,
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
            is_dirty: true,
            needs_fit: self.needs_fit,
            needs_drawing_sheet_fit: self.needs_drawing_sheet_fit,
            center_request: self.center_request,
            read_only: self.read_only,
            needs_history_reset: self.needs_history_reset,
            topology_version: self
                .topology_version
                .wrapping_add(candidate.topology_changes),
            content_version: self.content_version,
            snap_engine: self.snap_engine.clone(),
            selection_rect: self.selection_rect,
            net_highlight: self.net_highlight.clone(),
            undo_history: self.undo_history.clone(),
            canvas_cache: self.canvas_cache.clone(),
        }
    }
}
