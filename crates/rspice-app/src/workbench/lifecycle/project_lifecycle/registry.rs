//! Map the selected application surface to its canonical project document.

use crate::state::CellViewRef;
use crate::workbench::state::Workspace;
#[cfg(test)]
pub(super) use rspice_project::registry::FINGERPRINT_PASSES;
pub(super) use rspice_project::registry::content_digest;
#[cfg(test)]
pub(super) use rspice_project::registry::result_fingerprint::RESULT_FINGERPRINT_PASSES;
pub(crate) use rspice_project::registry::{DocumentRegistry, ProjectDocumentId};
#[cfg(test)]
use rspice_project::registry::{digest, document_digests, document_fingerprints};

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
            .editor
            .selection
            .select_component(component);
        state.schematic.copy_selection();
        state.schematic.session.editor.pan = (125.0, -40.0);
        state.schematic.session.editor.zoom = 2.25;
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
            &document_fingerprints(&current.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
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
            &document_fingerprints(&edited.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
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
            &document_fingerprints(&with_bus.file).unwrap(),
            Some(&document_fingerprints(&empty.file).unwrap()),
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
            &document_fingerprints(&with_tap.file).unwrap(),
            Some(&document_fingerprints(&with_bus.file).unwrap()),
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
            &document_fingerprints(&current.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
        );
        assert!(registry.is_dirty(&ProjectDocumentId::CellView(active.clone())));

        let mut edited_state = state;
        edited_state.schematic.document_mut_for_test().design_notes[0]
            .update(DesignNoteKind::PlainText, "Updated bias network")
            .unwrap();
        let edited = super::super::snapshot(&edited_state).expect("edited snapshot");
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&edited.file).unwrap(),
            Some(&document_fingerprints(&current.file).unwrap()),
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
        let baseline_digest = document_digests(&baseline.file)
            .unwrap()
            .remove(&ProjectDocumentId::CellView(active.clone()))
            .unwrap();
        let edited_digest = document_digests(&edited.file)
            .unwrap()
            .remove(&ProjectDocumentId::CellView(active.clone()))
            .unwrap();

        assert_ne!(baseline_digest, edited_digest);
        let mut registry = DocumentRegistry::default();
        registry.rebuild_from_fingerprints(
            &document_fingerprints(&edited.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
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

        let first_digest = document_digests(&first.file)
            .unwrap()
            .remove(&ProjectDocumentId::SimulationPlan)
            .unwrap();
        let second_digest = document_digests(&second.file)
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

        let baseline_digest = document_digests(&baseline.file)
            .unwrap()
            .remove(&ProjectDocumentId::ResultHistory)
            .unwrap();
        let edited_digest = document_digests(&edited.file)
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
                document_digests(&project.file).unwrap()[&ProjectDocumentId::ResultHistory],
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
                document_digests(&project.file).unwrap()[&ProjectDocumentId::ResultHistory],
                legacy,
                "an explicit, implied allocation limit preserves the legacy digest"
            );
            project.file.result_presentation.marker_id_high_water = Some(retained + 1);
            assert_ne!(
                document_digests(&project.file).unwrap()[&ProjectDocumentId::ResultHistory],
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
            &document_fingerprints(&current.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
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
            &document_fingerprints(&current.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
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
            &document_fingerprints(&current.file).unwrap(),
            Some(&document_fingerprints(&baseline.file).unwrap()),
        );

        assert!(registry.is_dirty(&ProjectDocumentId::CellView(reference)));
        assert!(!registry.is_dirty(&ProjectDocumentId::ProjectConfiguration));
        assert!(!registry.is_dirty(&ProjectDocumentId::NetlistSource));
    }
}
