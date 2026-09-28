//! Capture project-document fingerprints from the application workspace.

use crate::io::ProjectSnapshot;
use crate::product::ContentDigest;
use crate::state::CellViewRef;
use crate::workbench::state::Workspace;
#[cfg(test)]
pub(super) use rspice_project::registry::result_fingerprint::RESULT_FINGERPRINT_PASSES;
pub(super) use rspice_project::registry::{DocumentFingerprints, ResultFingerprintCache};
pub(crate) use rspice_project::registry::{DocumentRegistry, ProjectDocumentId};
use rspice_project::registry::{
    SchematicDocumentContent, ViewDocumentContent, digest, project_configuration_value,
    reference_from_key, result_fingerprint,
};
use std::collections::{HashMap, HashSet};

#[cfg(test)]
thread_local! {
    /// Full document fingerprint passes, including retained sample scans.
    pub(super) static FINGERPRINT_PASSES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub(super) fn document_fingerprints(
    project: &ProjectSnapshot,
) -> Result<DocumentFingerprints, String> {
    Ok(DocumentFingerprints::from_documents(document_digests(
        project,
    )?))
}

pub(super) fn document_fingerprints_with_results_cache(
    project: &ProjectSnapshot,
    cache: &ResultFingerprintCache,
) -> Result<DocumentFingerprints, String> {
    Ok(DocumentFingerprints::from_documents(
        document_digests_with_results_cache(project, Some(cache))?,
    ))
}

pub(crate) fn active_document(
    workspace: Workspace,
    active_view: &CellViewRef,
) -> ProjectDocumentId {
    match workspace {
        Workspace::Project => ProjectDocumentId::ProjectConfiguration,
        Workspace::Design => ProjectDocumentId::CellView(active_view.clone()),
        Workspace::Simulate => ProjectDocumentId::SimulationPlan,
        Workspace::Stimulus => ProjectDocumentId::StimulusLibrary,
        Workspace::Results => ProjectDocumentId::ResultHistory,
        // Specifications are owned by the active named simulation plan so a
        // clone can copy or omit them truthfully as one atomic payload.
        Workspace::Verify => ProjectDocumentId::SimulationPlan,
        Workspace::Models => ProjectDocumentId::ModelCatalog,
        Workspace::Netlist => ProjectDocumentId::NetlistSource,
    }
}

pub(crate) fn content_digest(project: &ProjectSnapshot) -> Result<ContentDigest, String> {
    document_fingerprints(project).map(|fingerprints| fingerprints.content_digest())
}

fn document_digests(
    project: &ProjectSnapshot,
) -> Result<HashMap<ProjectDocumentId, ContentDigest>, String> {
    document_digests_with_results_cache(project, None)
}

fn document_digests_with_results_cache(
    project: &ProjectSnapshot,
    results_cache: Option<&ResultFingerprintCache>,
) -> Result<HashMap<ProjectDocumentId, ContentDigest>, String> {
    #[cfg(test)]
    FINGERPRINT_PASSES.with(|passes| passes.set(passes.get() + 1));
    let mut documents = HashMap::new();
    let mut plan_payloads = project
        .file
        .workspace
        .simulation_plan_payloads
        .iter()
        .map(|record| (record.plan_id, &record.payload))
        .collect::<Vec<_>>();
    plan_payloads
        .sort_by(|(left, _), (right, _)| left.as_uuid().as_bytes().cmp(right.as_uuid().as_bytes()));

    documents.insert(
        ProjectDocumentId::ProjectConfiguration,
        digest(&project_configuration_value(
            &project.file.workspace.project,
            &project.file.libraries,
            &project.file.workspace.configuration_sets,
            &project.file.workspace.design_management,
            project.file.workspace.pdk_callback_receipts(),
        )?)?,
    );
    documents.insert(
        ProjectDocumentId::SimulationPlan,
        digest(&(
            project
                .file
                .execution_context
                .as_ref()
                .map(|context| &context.simulation_plan),
            plan_payloads,
        ))?,
    );
    documents.insert(
        ProjectDocumentId::ModelCatalog,
        digest(&project.file.execution_context.as_ref().map(|context| {
            (
                &context.model_libraries,
                &context.model_resolution_records,
                &context.model_validation_receipt,
            )
        }))?,
    );
    documents.insert(
        ProjectDocumentId::ResultHistory,
        match results_cache {
            Some(cache) => cache.digest(
                &project.file.simulation_results,
                &project.file.workspace.report_documents,
                &project.file.workspace.visualization_documents,
                &project.file.result_presentation,
            )?,
            None => result_fingerprint::digest(
                &project.file.simulation_results,
                &project.file.workspace.report_documents,
                &project.file.workspace.visualization_documents,
                &project.file.result_presentation,
            )?,
        },
    );
    // The stimulus definitions ride the project document rather than a
    // sidecar, so without an identity here an edited library would move the
    // saved file while every document in this registry still read clean —
    // "no unsaved changes" over a library that had just been republished.
    documents.insert(
        ProjectDocumentId::StimulusLibrary,
        digest(&project.file.workspace.stimulus_library)?,
    );
    documents.insert(
        ProjectDocumentId::VerificationSpecifications,
        // Retained as a stable registry identity for older callers. Current
        // specifications participate in the simulation-plan payload digest.
        digest(&())?,
    );
    let code_workspace_sources = project
        .file
        .workspace
        .project_sources
        .iter_bundles()
        .filter(|bundle| {
            matches!(
                bundle.owner(),
                crate::state::ProjectSourceOwner::CodeWorkspace { .. }
            )
        })
        .collect::<Vec<_>>();
    documents.insert(
        ProjectDocumentId::NetlistSource,
        digest(&(
            &project.file.workspace.netlist_source,
            &project.file.workspace.netlist_source_path,
            &project.file.workspace.netlist_document,
            &project.file.workspace.netlist_descriptor,
            &project.file.workspace.retained_netlist_decks,
            code_workspace_sources,
        ))?,
    );

    let mut references = HashSet::new();
    for key in project.file.workspace.schematic_buffers.keys() {
        if let Some(reference) = reference_from_key(key) {
            references.insert(reference);
        }
    }
    for key in project.file.workspace.physical_layout_documents().keys() {
        if let Some(reference) = reference_from_key(key) {
            references.insert(reference);
        }
    }
    for (library_key, library) in project.file.libraries.libraries_by_key() {
        for (cell_key, cell) in &library.cells {
            for view_key in cell.views.keys() {
                references.insert(CellViewRef::new(library_key, cell_key, view_key));
            }
        }
    }
    for reference in references {
        let schematic = project
            .file
            .workspace
            .schematic_buffers
            .get(&reference.key())
            .map(|schematic| SchematicDocumentContent::from(schematic.document()));
        let physical_layout = project.file.workspace.physical_layout_document(&reference);
        let view = project
            .file
            .libraries
            .get_library(&reference.library)
            .and_then(|library| library.get_cell(&reference.cell))
            .and_then(|cell| cell.get_view(&reference.view))
            .map(ViewDocumentContent::from);
        let project_source = project.file.workspace.project_sources.bundle_for_owner(
            &crate::state::ProjectSourceOwner::cell_view(reference.clone()),
        );
        documents.insert(
            ProjectDocumentId::CellView(reference),
            digest(&(schematic, physical_layout, view, project_source))?,
        );
    }

    Ok(documents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        BusDeclaration, BusSlice, BusTapOrientation, ComponentType, DesignNote, DesignNoteKind,
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableScope,
        DesignVariableSweepEligibility, DocumentationShape, DocumentationShapeGeometry, Point,
        SimulationPlanPayloadRecord,
    };
    use crate::workbench::app_state::AppState;

    #[test]
    fn presentation_and_interaction_state_never_marks_engineering_documents_dirty() {
        let mut state = AppState::default();
        let component = state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(4, 6));
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");
        let active = state.workspace.content.active_view.clone();

        state
            .schematic
            .session
            .selection
            .select_component(component);
        state.schematic.copy_selection();
        state.schematic.session.pan = (125.0, -40.0);
        state.schematic.session.zoom = 2.25;
        state.schematic.session.current_file = Some(std::path::PathBuf::from("presentation.rsch"));
        state.workspace.content.open_views[0].dirty = true;
        state.library_manager.filter_text = "presentation filter".to_owned();
        state.library_manager.show_read_only = !state.library_manager.show_read_only;
        state
            .library_manager
            .select_view(&active.library, &active.cell, &active.view);
        let view = state
            .library_manager
            .get_library_mut(&active.library)
            .and_then(|library| library.get_cell_mut(&active.cell))
            .and_then(|cell| cell.get_view_mut(&active.view))
            .expect("active view metadata");
        view.is_open = true;
        view.modified = true;
        view.file_path = Some(std::path::PathBuf::from("presentation-only.sch"));
        view.modified_time = Some(42);

        let current = super::super::snapshot(&state).expect("current snapshot");
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&current).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );
        assert!(
            registry.records().iter().all(|record| !record.dirty),
            "selection, clipboard, viewport, open-state, and browser presentation are not engineering edits"
        );

        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(12, 9));
        let edited = super::super::snapshot(&state).expect("edited snapshot");
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&edited).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active)));
        assert!(!registry.is_dirty(&ProjectDocumentId::ProjectConfiguration));
    }

    #[test]
    fn typed_bus_and_tap_edits_mark_the_schematic_document_dirty() {
        let mut state = AppState::default();
        let active = state.workspace.content.active_view.clone();
        let empty = super::super::snapshot(&state).expect("empty baseline");
        let declaration = BusDeclaration::parse("DATA[15:0]").expect("valid declaration");
        let bus_id = state
            .schematic
            .add_bus(vec![Point::new(0, 0), Point::new(10, 0)], Some(declaration))
            .expect("add bus");

        let with_bus = super::super::snapshot(&state).expect("bus snapshot");
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&with_bus).unwrap(),
            Some(&document_fingerprints(&empty).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active.clone())));

        state
            .schematic
            .place_bus_tap(
                bus_id,
                Point::new(5, 0),
                Point::new(5, 4),
                BusSlice::parse("DATA[7]").expect("valid scalar tap"),
                BusTapOrientation::Automatic,
            )
            .expect("place tap");
        let with_tap = super::super::snapshot(&state).expect("tap snapshot");
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&with_tap).unwrap(),
            Some(&document_fingerprints(&with_bus).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active)));
    }

    #[test]
    fn design_note_edits_participate_in_the_schematic_document_digest() {
        let mut state = AppState::default();
        let active = state.workspace.content.active_view.clone();
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");
        state.schematic.document_mut_for_test().design_notes.push(
            DesignNote::new(
                71,
                Point::new(4, 6),
                DesignNoteKind::PlainText,
                "Bias network",
            )
            .unwrap(),
        );

        let current = super::super::snapshot(&state).expect("design-note snapshot");
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&current).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active.clone())));

        let mut edited_state = state;
        edited_state.schematic.document_mut_for_test().design_notes[0]
            .update(DesignNoteKind::PlainText, "Updated bias network")
            .unwrap();
        let edited = super::super::snapshot(&edited_state).expect("edited snapshot");
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&edited).unwrap(),
            Some(&document_fingerprints(&current).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active)));
    }

    #[test]
    fn documentation_shape_edits_participate_in_the_schematic_document_digest() {
        let mut state = AppState::default();
        let active = state.workspace.content.active_view.clone();
        state
            .schematic
            .document_mut_for_test()
            .documentation_shapes
            .push(
                DocumentationShape::new(
                    72,
                    DocumentationShapeGeometry::Rectangle {
                        first: Point::new(4, 6),
                        opposite: Point::new(14, 12),
                    },
                )
                .unwrap(),
            );
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");

        state.schematic.document_mut_for_test().documentation_shapes[0]
            .translate(Point::new(3, -2));
        let edited = super::super::snapshot(&state).expect("edited shape snapshot");
        let baseline_digest = document_digests(&baseline)
            .unwrap()
            .remove(&ProjectDocumentId::CellView(active.clone()))
            .unwrap();
        let edited_digest = document_digests(&edited)
            .unwrap()
            .remove(&ProjectDocumentId::CellView(active.clone()))
            .unwrap();

        assert_ne!(baseline_digest, edited_digest);
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&edited).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active)));
    }

    #[test]
    fn simulation_plan_payload_digest_is_owner_order_independent() {
        let state = AppState::default();
        let mut first = super::super::snapshot(&state).expect("first snapshot");
        let mut second = first.clone();
        first
            .file
            .workspace
            .simulation_plan_payloads
            .push(SimulationPlanPayloadRecord {
                plan_id: crate::product::SimulationPlanId::new(),
                payload: Default::default(),
            });
        second.file.workspace.simulation_plan_payloads = first
            .file
            .workspace
            .simulation_plan_payloads
            .iter()
            .cloned()
            .rev()
            .collect();

        let first_digest = document_digests(&first)
            .unwrap()
            .remove(&ProjectDocumentId::SimulationPlan)
            .unwrap();
        let second_digest = document_digests(&second)
            .unwrap()
            .remove(&ProjectDocumentId::SimulationPlan)
            .unwrap();
        assert_eq!(first_digest, second_digest);
    }

    #[test]
    fn report_documents_participate_in_the_result_history_digest() {
        let state = AppState::default();
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");
        let mut edited = baseline.clone();
        let mut report =
            crate::results::report_document::ReportDocument::new("Verification report")
                .expect("report document");
        report
            .transact(
                report.revision(),
                vec![crate::results::report_document::ReportEdit::AddPage {
                    title: "Executive summary".to_owned(),
                }],
                1,
            )
            .expect("report page transaction");
        edited.file.workspace.report_documents.push(report);

        let baseline_digest = document_digests(&baseline)
            .unwrap()
            .remove(&ProjectDocumentId::ResultHistory)
            .unwrap();
        let edited_digest = document_digests(&edited)
            .unwrap()
            .remove(&ProjectDocumentId::ResultHistory)
            .unwrap();
        assert_ne!(baseline_digest, edited_digest);
    }

    #[test]
    fn result_owner_preserves_existing_fingerprint_encoding() {
        let state = AppState::default();
        let mut project = super::super::snapshot(&state).unwrap();
        for populated in [false, true] {
            if populated {
                let key = serde_json::json!({
                    "dataset_id": "b3c6b2be-c997-4f5d-a06e-714071283df5",
                    "source": {"Legacy": 7}
                });
                project.file.result_presentation = serde_json::from_value(serde_json::json!({
                    "result_markers": [{
                        "id": 3, "analysis": key,
                        "anchor": {"analysis": key, "trace": {
                            "source_name": "V(out)", "kind": 0, "family_group": 0
                        }},
                        "trace_name": "V(out)", "x": 0.125, "kind": "Note", "note": "Test"
                    }],
                    "result_log_y_panes": [{"analysis": key, "unit": "V"}],
                    "result_expression_groups": [{"analysis": key, "traces": [{"text": "V(out)*2"}]}]
                })).unwrap();
            }
            // This is the pre-migration six-element encoding, including three
            // empty arrays when annotations are absent. Changing it would also
            // change generated-netlist authority for otherwise unchanged input.
            let legacy = digest(&(
                &project.file.simulation_results,
                &project.file.workspace.report_documents,
                &project.file.workspace.visualization_documents,
                &project.file.result_presentation.markers,
                &project.file.result_presentation.log_y_panes,
                &project.file.result_presentation.expression_groups,
            ))
            .unwrap();
            assert_eq!(
                document_digests(&project).unwrap()[&ProjectDocumentId::ResultHistory],
                legacy
            );
            let retained = project
                .file
                .result_presentation
                .markers
                .iter()
                .map(|marker| marker.id)
                .max()
                .unwrap_or(0);
            project.file.result_presentation.marker_id_high_water = Some(retained);
            assert_eq!(
                document_digests(&project).unwrap()[&ProjectDocumentId::ResultHistory],
                legacy,
                "an explicit, implied allocation limit preserves the legacy digest"
            );
            project.file.result_presentation.marker_id_high_water = Some(retained + 1);
            assert_ne!(
                document_digests(&project).unwrap()[&ProjectDocumentId::ResultHistory],
                legacy,
                "deleted or abandoned marker identities remain document content"
            );
        }
    }

    #[test]
    fn design_variable_edit_marks_the_simulation_plan_document_dirty() {
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(4, 6));
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");
        let plan_id = state.sim_setup.stable_analysis_plan().unwrap().id();
        let variable = DesignVariable::new(
            "RLOAD",
            "10 kohm",
            DesignVariableQuantity::Resistance,
            DesignVariableScope::Testbench,
            "Load resistance",
            None,
            DesignVariableSweepEligibility::NestedSweepAndOptimization,
            DesignVariableOverridePolicy::ExplicitTestLocalOverride,
        )
        .unwrap();
        state
            .workspace
            .content
            .ensure_active_plan_data(plan_id)
            .design_variables
            .push(variable);

        let current = super::super::snapshot(&state).expect("current snapshot");
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&current).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::SimulationPlan));
        assert!(!registry.is_dirty(&ProjectDocumentId::VerificationSpecifications));
    }

    #[test]
    fn code_sources_participate_in_the_code_document_digest() {
        let mut state = AppState::default();
        if state
            .workspace
            .content
            .project_sources
            .get(crate::state::ProjectSourceLanguage::VerilogA)
            .is_none()
        {
            state
                .workspace
                .content
                .project_sources
                .insert(
                    crate::state::ProjectSourceDocument::try_new(
                        "sensor_bridge.va",
                        crate::state::ProjectSourceLanguage::VerilogA,
                        "module sensor_bridge; endmodule",
                    )
                    .unwrap(),
                )
                .unwrap();
        }
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");

        state
            .workspace
            .content
            .replace_project_source(
                crate::state::ProjectSourceLanguage::VerilogA,
                "module sensor_bridge; analog begin end endmodule".to_owned(),
            )
            .unwrap();
        let current = super::super::snapshot(&state).expect("edited snapshot");
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&current).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );

        assert!(registry.is_dirty(&ProjectDocumentId::NetlistSource));
        assert!(!registry.is_dirty(&ProjectDocumentId::ProjectConfiguration));
    }

    #[test]
    fn cell_view_existence_and_owned_source_are_not_project_configuration_content() {
        let mut state = AppState::default();
        let baseline = super::super::snapshot(&state).expect("baseline snapshot");
        let reference = CellViewRef::new(
            state.workspace.content.active_view.library.clone(),
            state.workspace.content.active_view.cell.clone(),
            "behavior",
        );
        state
            .library_manager
            .get_library_mut(&reference.library)
            .and_then(|library| library.get_cell_mut(&reference.cell))
            .expect("active cell")
            .add_view(crate::state::View::new(
                reference.view.as_str(),
                crate::state::ViewType::VerilogA,
            ));
        state
            .workspace
            .content
            .project_sources
            .insert_bundle(
                crate::state::ProjectSourceBundle::try_new(
                    crate::state::ProjectSourceOwner::cell_view(reference.clone()),
                    crate::state::ProjectSourceLanguage::VerilogA,
                    "behavior.va",
                    "module behavior(p, n); inout p, n; endmodule",
                    std::iter::empty(),
                    std::iter::empty(),
                )
                .expect("valid bundle"),
            )
            .expect("unique owner");

        let current = super::super::snapshot(&state).expect("current snapshot");
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&current).unwrap(),
            Some(&document_fingerprints(&baseline).unwrap()),
        );

        assert!(registry.is_dirty(&ProjectDocumentId::CellView(reference)));
        assert!(!registry.is_dirty(&ProjectDocumentId::ProjectConfiguration));
        assert!(!registry.is_dirty(&ProjectDocumentId::NetlistSource));
    }
}
