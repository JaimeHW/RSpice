//! What a run would execute, named so evidence about it can expire.
//!
//! Preflight retains an immutable report and later queues it without
//! recomputing. That is only sound if the report can say *which* design and
//! *which* plan it was taken against, and notice when either has moved. These
//! two derivations produce that identity.
//!
//! The closure matters as much as the root: a hierarchical design changes when
//! any schematic it instantiates changes, so the topology revision is a set of
//! per-reference versions rather than one number. The active editor tab
//! contributes only when it *is* the configured root — another open schematic
//! can never expire or validate evidence for the design being run.
//!
//! This is derived state, not presentation, so it lives with the session data
//! rather than with the dialog that displays a report. Four modules ask the
//! question; none of them should have to name the dialog to get an answer.

use crate::state::ComponentType;
use crate::workbench::app_state::AppState;

impl AppState {
    /// Plan identity and revision of the active analysis plan, if one is
    /// configured. `None` means there is nothing to expire against.
    pub(crate) fn active_plan_revision(
        &self,
    ) -> Option<(
        crate::product::SimulationPlanId,
        crate::product::ObjectRevision,
    )> {
        self.sim_setup
            .analysis_plan
            .as_ref()
            .map(|plan| (plan.id(), plan.revision()))
    }

    /// Stable identity and revision of the configured execution root, with the
    /// per-reference topology versions of everything it instantiates.
    pub(crate) fn configured_topology_revision(&self) -> (String, u64, Vec<(String, u64)>) {
        let root = self.workspace.simulation_root_reference();
        let projection = self.workspace.configuration_execution_projection(
            &self.library_manager,
            &self.workspace.active_view,
            &self.schematic,
        );
        let mut closure = std::collections::BTreeMap::new();
        if let Ok(projection) = &projection {
            // Two independent things put a cell view in this closure, and
            // neither implies the other. The design *references* it, which is
            // what the walk below follows — a reference the resolution could
            // not bind still names content a user edits, and dropping it lets
            // a stale report read as current. A resolution *bound* it, which
            // the seeds carry — a configuration can select a view no placed
            // instance names, and that view's content expires the report too.
            let mut references = std::collections::BTreeSet::new();
            let mut pending = vec![root.key().to_ascii_lowercase()];
            pending.extend(
                projection
                    .plan()
                    .into_iter()
                    .flat_map(|plan| plan.bindings())
                    .map(|binding| binding.resolved_reference().key().to_ascii_lowercase()),
            );
            while let Some(reference) = pending.pop() {
                if !references.insert(reference.clone()) {
                    continue;
                }
                let Some((_, schematic)) = projection
                    .schematic_buffers()
                    .iter()
                    .find(|(key, _)| key.eq_ignore_ascii_case(&reference))
                else {
                    continue;
                };
                pending.extend(schematic.components.iter().filter_map(|component| {
                    (component.kind == ComponentType::CellInstance)
                        .then_some(component.library_cell.as_ref())
                        .flatten()
                        .filter(|binding| {
                            binding.source_path.is_none() && !binding.is_executable_builtin()
                        })
                        .map(|binding| {
                            format!(
                                "{}/{}/schematic",
                                binding.library.to_ascii_lowercase(),
                                binding.cell.to_ascii_lowercase()
                            )
                        })
                }));
            }
            for reference in references {
                if let Some((_, schematic)) = projection
                    .schematic_buffers()
                    .iter()
                    .find(|(key, _)| key.eq_ignore_ascii_case(&reference))
                {
                    closure.insert(reference, schematic.topology_version());
                }
            }
        } else {
            closure.extend(
                self.workspace
                    .schematic_buffers
                    .iter()
                    .map(|(key, schematic)| {
                        (key.to_ascii_lowercase(), schematic.topology_version())
                    }),
            );
            closure.insert(
                self.workspace.active_view.key().to_ascii_lowercase(),
                self.schematic.topology_version(),
            );
        }
        let root_key = root.key();
        let revision = closure
            .get(&root_key.to_ascii_lowercase())
            .copied()
            .or_else(|| {
                self.workspace
                    .simulation_root_schematic(&self.workspace.active_view, &self.schematic)
                    .map(crate::state::SchematicState::topology_version)
            })
            .unwrap_or(0);
        (root_key, revision, closure.into_iter().collect())
    }
}
