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
        self.design.hierarchy_source(
            &self.session.selection.components,
            self.session.selection.count(),
        )
    }

    pub fn plan_hierarchy_extraction(
        &self,
        terminals: &[HierarchyExtractionTerminal],
        connectivity: &HierarchyNetConnectivity,
        component_bounds: &HashMap<u64, (i32, i32, i32, i32)>,
    ) -> Result<HierarchyExtractionPlan, super::super::hierarchy::HierarchyExtractionError> {
        if self.session.read_only {
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
        if self.session.read_only {
            return Err(super::super::hierarchy::HierarchyExtractionError::ReadOnly);
        }
        let candidate = self.hierarchy_source().materialize_hierarchy_extraction(
            plan,
            library,
            cell,
            view,
            self.session.preview_rotation,
            self.session.preview_mirror_h,
        )?;
        let mut parent =
            self.clone_with_design(self.design.clone_with_hierarchy_document(candidate.parent));
        parent.session.is_dirty = true;
        parent
            .session
            .selection
            .select_only_component(candidate.instance_id);
        parent.repair_clipboard_after_load();
        let mut child = Self::default();
        child.design =
            rspice_design::schematic::owned::Schematic::from_hierarchy_document(candidate.child);
        child.session.is_dirty = true;
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
            session: self.session.clone(),
        }
    }
}
