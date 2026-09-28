//! Document-scoped save candidates over canonical project content.

use crate::registry::ProjectDocumentId;
use crate::{ProjectExecutionContext, ProjectFile};
use rspice_design_model::cell_view::CellViewRef;
use rspice_simulation_contract::plan_model::SimulationPlan;

impl ProjectFile {
    /// Overlay one document and validate the resulting save candidate.
    /// An error may leave this candidate partially updated; discard it.
    pub fn overlay_document(
        &mut self,
        working: &Self,
        id: &ProjectDocumentId,
    ) -> Result<(), String> {
        match id {
            ProjectDocumentId::ProjectConfiguration => {
                self.workspace.project = working.workspace.project.clone();
                self.workspace.configuration_sets = working.workspace.configuration_sets.clone();
                self.workspace.design_management = working.workspace.design_management.clone();
                self.workspace.replace_pdk_callback_receipts_for_lifecycle(
                    working.workspace.pdk_callback_receipts().to_vec(),
                )?;
                self.libraries = working
                    .libraries
                    .with_document_content_from(&self.libraries);
            }
            ProjectDocumentId::CellView(reference) => overlay_cell_view(self, working, reference)?,
            ProjectDocumentId::SimulationPlan => {
                ensure_execution_context(self, working)?.simulation_plan = working
                    .execution_context
                    .as_ref()
                    .ok_or_else(|| "working project has no simulation plan".to_owned())?
                    .simulation_plan
                    .clone();
                self.workspace.simulation_plan_payloads =
                    working.workspace.simulation_plan_payloads.clone();
                if let Some(plan_id) = self
                    .execution_context
                    .as_ref()
                    .and_then(|context| context.simulation_plan.analysis_plan.as_ref())
                    .map(SimulationPlan::id)
                {
                    self.workspace.sync_legacy_specs_projection(plan_id);
                }
            }
            ProjectDocumentId::ModelCatalog => {
                let source = working
                    .execution_context
                    .as_ref()
                    .ok_or_else(|| "working project has no model catalog".to_owned())?;
                let destination = ensure_execution_context(self, working)?;
                destination
                    .model_libraries
                    .clone_from(&source.model_libraries);
                destination
                    .model_resolution_records
                    .clone_from(&source.model_resolution_records);
                destination
                    .model_validation_receipt
                    .clone_from(&source.model_validation_receipt);
            }
            ProjectDocumentId::ResultHistory => {
                self.simulation_results = working.simulation_results.clone();
                self.result_presentation = working.result_presentation.clone();
                self.workspace.report_documents = working.workspace.report_documents.clone();
                self.workspace.visualization_documents =
                    working.workspace.visualization_documents.clone();
            }
            ProjectDocumentId::VerificationSpecifications => {
                self.workspace.specs = working.workspace.specs.clone();
            }
            ProjectDocumentId::StimulusLibrary => {
                self.workspace.stimulus_library = working.workspace.stimulus_library.clone();
            }
            ProjectDocumentId::NetlistSource => {
                self.workspace.netlist_source = working.workspace.netlist_source.clone();
                self.workspace.netlist_source_path = working.workspace.netlist_source_path.clone();
                self.workspace.netlist_document = working.workspace.netlist_document.clone();
                self.workspace.netlist_descriptor = working.workspace.netlist_descriptor.clone();
                self.workspace.retained_netlist_decks =
                    working.workspace.retained_netlist_decks.clone();
                self.workspace
                    .project_sources
                    .synchronize_code_workspace_bundles_from(&working.workspace.project_sources)
                    .map_err(|error| error.to_string())?;
            }
        }
        self.validate().map_err(|error| error.to_string())
    }
}

fn ensure_execution_context<'a>(
    target: &'a mut ProjectFile,
    working: &ProjectFile,
) -> Result<&'a mut ProjectExecutionContext, String> {
    if target.execution_context.is_none() {
        target.execution_context = working.execution_context.clone();
    }
    target
        .execution_context
        .as_mut()
        .ok_or_else(|| "project has no execution context".to_owned())
}

fn overlay_cell_view(
    target: &mut ProjectFile,
    working: &ProjectFile,
    reference: &CellViewRef,
) -> Result<(), String> {
    target.libraries.overlay_cell_view_document_from_snapshot(
        &working.libraries,
        &reference.library,
        &reference.cell,
        &reference.view,
    )?;

    let key = reference.key();
    match working.workspace.schematic_buffers.get(&key).cloned() {
        Some(design) => {
            target.workspace.schematic_buffers.insert(key, design);
        }
        None => {
            target.workspace.schematic_buffers.remove(&key);
        }
    }
    target
        .workspace
        .synchronize_physical_layout_document_from(reference, &working.workspace)
        .map_err(|error| error.to_string())?;
    // Sheets are part of the drawing, not of project setup: a cell view that
    // is saved on its own carries its own sheet catalog, and leaves every
    // other cell view's sheets unpublished.
    target
        .workspace
        .overlay_sheet_catalog_from(reference, &working.workspace)
        .map_err(|error| error.to_string())?;
    target
        .workspace
        .project_sources
        .synchronize_cell_view_bundle_from(reference, &working.workspace.project_sources)
        .map_err(|error| error.to_string())?;
    Ok(())
}
