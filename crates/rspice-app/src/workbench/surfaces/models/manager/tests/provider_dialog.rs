//! Failed provider publication retains the rendered transaction for retry.

use super::*;

#[test]
fn resolved_model_and_subcircuit_rows_reopen_and_clear_their_provider_decisions() {
    for (scope, source) in [
        (
            ModelConsumerScope::PrimitiveModel,
            ".model shared NMOS (LEVEL=1 KP=1e-3)\n",
        ),
        (
            ModelConsumerScope::Subcircuit,
            ".subckt shared p n\nR1 p n 1k\n.ends shared\n",
        ),
    ] {
        let mut app = RSpiceApp::test_instance();
        app.state.model_library_manager.clear();
        for name in ["alpha", "beta"] {
            app.state
                .model_library_manager
                .load_library_bytes(&format!("{name}.lib"), source.as_bytes().to_vec(), None)
                .unwrap();
        }
        app.state
            .model_library_manager
            .resolve_definition_provider(scope, "shared", "beta", "Accepted provider")
            .unwrap();
        app.state.workbench.models_page = ModelsPage::Include;
        app.state.workbench.models_view.include_definition_query = "shared".to_owned();
        let mut studio = ModelsStudio::open(app);
        let revision = studio.app.state.workspace.project.revision();
        let row = format!("shared · {} ·", scope.label());
        studio.click(|label| label.starts_with(&row));
        let Some(ModelsWorkbenchDialog::DefinitionConflict {
            selected_provider,
            error,
            ..
        }) = &studio.app.state.workbench.models_view.dialog
        else {
            panic!("the resolved {} row must open its decision", scope.label());
        };
        assert_eq!(selected_provider, "beta");
        assert!(error.is_none());
        studio.click(|label| label == "Clear provider decision");
        assert!(studio.app.state.workbench.models_view.dialog.is_none());
        assert!(
            studio
                .app
                .state
                .model_library_manager
                .model_resolution_record(scope, "shared")
                .is_none()
        );
        assert_eq!(
            studio.app.state.workspace.project.revision().get(),
            revision.get() + 1
        );
    }
}

#[test]
fn failed_provider_publication_preserves_the_dialog_draft_and_accepted_decision() {
    let mut app = RSpiceApp::test_instance();
    app.state.model_library_manager.clear();
    for name in ["alpha", "beta"] {
        let mut library = ModelLibrary::new(name);
        library.add_model(DeviceModel::new("nch", ModelType::Nmos));
        app.state.model_library_manager.add_library(library);
    }
    let scope = ModelConsumerScope::PrimitiveModel;
    let original = app
        .state
        .model_library_manager
        .resolve_definition_provider(scope, "nch", "alpha", "Previously accepted provider")
        .unwrap();
    let draft = "Use the reviewed characterization source.\nRetain this audit reason.";
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::DefinitionConflict {
        definition: "nch".to_owned(),
        scope,
        providers: vec!["alpha".to_owned(), "beta".to_owned()],
        selected_provider: "beta".to_owned(),
        reason: draft.to_owned(),
        error: None,
    });
    let mut studio = ModelsStudio::open(app);
    let revision = studio.app.state.workspace.project.revision();
    let execution_epoch = studio.app.state.design_execution_epoch;

    for epoch in [
        Err("system clock is unavailable"),
        Ok(std::time::Duration::ZERO),
        Ok(std::time::Duration::MAX),
    ] {
        crate::time_compat::with_unix_epoch(epoch, || {
            studio.click(|label| label == "Publish provider decision");
        });
        let Some(ModelsWorkbenchDialog::DefinitionConflict {
            definition,
            scope: retained_scope,
            selected_provider,
            reason,
            error,
            ..
        }) = &studio.app.state.workbench.models_view.dialog
        else {
            panic!("a failed publication must retain its dialog");
        };
        assert_eq!(definition, "nch");
        assert_eq!(*retained_scope, scope);
        assert_eq!(selected_provider, "beta");
        assert_eq!(reason, draft);
        let error = error
            .as_deref()
            .expect("the retained dialog explains the failure");
        assert!(error.contains("system clock cannot timestamp provider decision"));
        assert!(studio.painted.iter().any(|(text, _)| text == error));
        assert_eq!(
            studio
                .app
                .state
                .model_library_manager
                .model_resolution_record(scope, "nch"),
            Some(&original)
        );
        assert_eq!(studio.app.state.workspace.project.revision(), revision);
        assert_eq!(studio.app.state.design_execution_epoch, execution_epoch);
    }

    studio.click(|label| label == "Publish provider decision");
    assert!(
        studio.app.state.workbench.models_view.dialog.is_none(),
        "healthy retry did not publish: {:?}",
        studio.app.state.workbench.models_view.dialog
    );
    let accepted = studio
        .app
        .state
        .model_library_manager
        .model_resolution_record(scope, "nch")
        .unwrap();
    assert_eq!(accepted.provider_library, "beta");
    assert_eq!(accepted.audit_reason, draft);
    assert_eq!(
        studio.app.state.workspace.project.revision().get(),
        revision.get() + 1
    );
    assert_eq!(
        studio.app.state.design_execution_epoch,
        execution_epoch.wrapping_add(1)
    );
}
