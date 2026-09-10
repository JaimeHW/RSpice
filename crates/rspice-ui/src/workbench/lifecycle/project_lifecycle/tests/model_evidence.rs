//! Model evidence belongs to the same save/revert boundary as its catalog.

use super::*;
use crate::state::model_library::{
    ModelConsumerScope, ModelValidationFinding, ModelValidationFindingSeverity,
};

#[test]
fn model_validation_receipt_is_dirty_and_survives_save_reload_and_revert() {
    for scope in [SaveScope::ActiveDocument, SaveScope::AllDocuments] {
        let mut state = AppState::default();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Models);
        let path = unique_path("model-validation-evidence");
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        let solver_input = generated_netlist_input_digest(&state).unwrap();
        let plan = state
            .model_library_manager
            .seal_execution_sources()
            .unwrap()
            .reference_model_execution_plan(crate::product::ProcessCorner::TT)
            .unwrap();
        let receipt = state
            .model_library_manager
            .issue_model_validation_receipt(
                state.workspace.project.revision(),
                plan.digest(),
                None,
                crate::io::PROJECT_EXECUTION_CONTEXT_SCHEMA_VERSION,
                vec![ModelValidationFinding {
                    code: "SPICE_NAMESPACE_COMPILED".to_owned(),
                    severity: ModelValidationFindingSeverity::Information,
                    message: "The retained source namespace compiled.".to_owned(),
                }],
            )
            .unwrap();
        assert!(
            has_unsaved_changes(&state),
            "a new validation receipt must require saving"
        );
        assert!(active_document_is_dirty(&state));
        assert_eq!(dirty_document_count(&state), 1);
        assert_eq!(
            generated_netlist_input_digest(&state).unwrap(),
            solver_input,
            "validation evidence does not change the generated circuit"
        );
        insert_ac_analysis(&mut state);
        save_native(&mut state, scope, &path, DestinationAuthority::Canonical).unwrap();
        let reloaded = crate::io::load_project_file(&path)
            .unwrap()
            .execution_context
            .unwrap();
        assert_eq!(reloaded.model_validation_receipt, Some(receipt.clone()));
        assert_eq!(
            has_ac_analysis(&reloaded.simulation_plan),
            scope == SaveScope::AllDocuments
        );
        assert!(!active_document_is_dirty(&state));
        state
            .model_library_manager
            .restore_model_validation_receipt(None)
            .unwrap();
        assert!(active_document_is_dirty(&state));
        let review = prepare_revert_active_document(&state).unwrap();
        confirm_revert_active_document(&mut state, &review).unwrap();
        assert_eq!(
            state.model_library_manager.model_validation_receipt(),
            Some(&receipt)
        );
        assert!(!active_document_is_dirty(&state));
        assert!(
            has_ac_analysis(&state.sim_setup),
            "model revert must retain the plan draft"
        );
        for epoch in [
            Err("clock unavailable"),
            Ok(std::time::Duration::ZERO),
            Ok(std::time::Duration::MAX),
        ] {
            crate::time_compat::with_unix_epoch(epoch, || {
                assert!(
                    state
                        .model_library_manager
                        .issue_model_validation_receipt(
                            receipt.project_revision,
                            receipt.model_execution_plan_digest,
                            receipt.pdk_archive_digest,
                            receipt.execution_schema_version,
                            receipt.findings.clone(),
                        )
                        .is_err()
                );
            });
            assert_eq!(
                state.model_library_manager.model_validation_receipt(),
                Some(&receipt)
            );
            assert!(!active_document_is_dirty(&state));
        }
        remove_project_artifacts(&path);
    }
}

#[test]
fn provider_decision_is_dirty_and_survives_save_reload_and_revert() {
    for scope in [SaveScope::ActiveDocument, SaveScope::AllDocuments] {
        let mut state = AppState::default();
        state
            .workbench
            .activate(crate::workbench::state::Workspace::Models);
        let first = state
            .model_library_manager
            .load_library_bytes(
                "first.lib",
                b".model shared NMOS (LEVEL=1 KP=1e-3)\n".to_vec(),
                None,
            )
            .unwrap();
        let second = state
            .model_library_manager
            .load_library_bytes(
                "second.lib",
                b".model shared NMOS (LEVEL=1 KP=2e-3)\n".to_vec(),
                None,
            )
            .unwrap();
        state
            .model_library_manager
            .resolve_definition_provider(
                ModelConsumerScope::PrimitiveModel,
                "shared",
                &first,
                "Initial characterization source",
            )
            .unwrap();
        let path = unique_path("model-provider-evidence");
        save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .unwrap();
        let solver_input = generated_netlist_input_digest(&state).unwrap();
        let original = state
            .model_library_manager
            .model_resolution_record(ModelConsumerScope::PrimitiveModel, "shared")
            .cloned()
            .unwrap();
        for epoch in [
            Err("clock unavailable"),
            Ok(std::time::Duration::ZERO),
            Ok(std::time::Duration::MAX),
        ] {
            crate::time_compat::with_unix_epoch(epoch, || {
                assert!(
                    state
                        .model_library_manager
                        .resolve_definition_provider(
                            ModelConsumerScope::PrimitiveModel,
                            "shared",
                            &second,
                            "Reviewed replacement source",
                        )
                        .is_err()
                );
            });
            assert_eq!(
                state
                    .model_library_manager
                    .model_resolution_record(ModelConsumerScope::PrimitiveModel, "shared"),
                Some(&original)
            );
            assert!(!has_unsaved_changes(&state));
        }
        let decision = state
            .model_library_manager
            .resolve_definition_provider(
                ModelConsumerScope::PrimitiveModel,
                "shared",
                &second,
                "Reviewed replacement source",
            )
            .unwrap();
        assert!(
            has_unsaved_changes(&state),
            "a provider decision must require saving"
        );
        assert!(active_document_is_dirty(&state));
        assert_eq!(dirty_document_count(&state), 1);
        assert_ne!(
            generated_netlist_input_digest(&state).unwrap(),
            solver_input,
            "provider selection changes the engine-facing model source"
        );
        insert_ac_analysis(&mut state);
        save_native(&mut state, scope, &path, DestinationAuthority::Canonical).unwrap();
        let reloaded = crate::io::load_project_file(&path)
            .unwrap()
            .execution_context
            .unwrap();
        assert_eq!(reloaded.model_resolution_records, vec![decision.clone()]);
        assert_eq!(
            has_ac_analysis(&reloaded.simulation_plan),
            scope == SaveScope::AllDocuments
        );
        assert!(!active_document_is_dirty(&state));
        assert!(
            state
                .model_library_manager
                .clear_definition_provider(ModelConsumerScope::PrimitiveModel, "shared")
        );
        assert!(active_document_is_dirty(&state));
        let review = prepare_revert_active_document(&state).unwrap();
        confirm_revert_active_document(&mut state, &review).unwrap();
        assert_eq!(
            state
                .model_library_manager
                .model_resolution_record(ModelConsumerScope::PrimitiveModel, "shared"),
            Some(&decision)
        );
        assert!(!active_document_is_dirty(&state));
        assert!(has_ac_analysis(&state.sim_setup));
        remove_project_artifacts(&path);
    }
}
