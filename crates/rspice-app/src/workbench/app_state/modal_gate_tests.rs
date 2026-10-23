//! Which retained state makes the whole application modal.
//!
//! [`AppState::application_modal_open`] is a hand-written OR over the dialog
//! gate, the workbench's own gate, and five owners of their own. Every row
//! below sets one term on a default session and nothing else. The dialog
//! gate's terms each have their row beside `AppState::dialog_modal_open`;
//! here it is one term. The browser's file-operation term is set only by an
//! operation a browser is running, so no native test can open it.

use crate::workbench::app_state::AppState;
use crate::workbench::state::Workspace;
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

fn navigate(state: &mut AppState, surface: SurfaceId) {
    state
        .workbench
        .navigate(SurfaceRoute::surface(surface), RouteTransitionSource::User)
        .unwrap_or_else(|error| panic!("{surface:?} opens: {error}"));
}

/// Retained state by name, and the least that opens it on a default session.
type GateRow = (&'static str, fn(&mut AppState));

/// One row per term, in the order the two gates list them: the workbench's
/// first, then the session's own.
const APPLICATION_GATE: &[GateRow] = &[
    ("workbench.project_launcher_open", |state| {
        state.project.project_launcher_open = true;
    }),
    ("workbench.examples_selection", |state| {
        state.project.examples_selection = Some(0);
    }),
    ("workbench.notification_center_open", |state| {
        state.workbench.notification_center_open = true;
    }),
    ("workbench.model_correlation.dialog", |state| {
        state.models.model_correlation.dialog = Some(
            crate::workbench::documents::model_correlation::ModelCorrelationDialog::AddMetric(
                crate::workbench::documents::model_correlation::CorrelationMetricDraft::default(),
            ),
        );
    }),
    ("workbench.create_result_document", |state| {
        state.results.create_result_document.open = true;
    }),
    ("workbench.result_document_dialog", |state| {
        state.results.result_document_dialog = Some(
            rspice_results_ui::document_dialogs::ResultDocumentDialog::AddPane(
                rspice_results_ui::document_dialogs::add_pane::AddPaneDraft::default(),
            ),
        );
    }),
    ("workbench.result_import", |state| {
        state.results.result_import.open = true;
    }),
    ("workbench.models_view.dialog", |state| {
        state.models.models_view.dialog = Some(
            crate::workbench::state::ModelsWorkbenchDialog::SourcePreview {
                title: "nch".to_owned(),
                subtitle: "foundation.lib".to_owned(),
                source: ".model nch nmos".to_owned(),
                editable: false,
            },
        );
    }),
    ("workbench.simulation_workflow", |state| {
        state.simulate.simulation_workflow = Some(
            crate::workbench::state::SimulationWorkflowDialog::DesignVariable(
                crate::workbench::state::DesignVariableDraft::default(),
            ),
        );
    }),
    (
        "workbench.verification.regression_baseline_picker_open",
        |state| {
            state.simulate.verification.regression_baseline_picker_open = true;
        },
    ),
    ("workbench.verification.tuning_review_open", |state| {
        state.simulate.verification.tuning_review_open = true;
    }),
    ("route ProjectLauncher", |state| {
        navigate(state, SurfaceId::ProjectLauncher);
    }),
    ("route Preferences", |state| {
        navigate(state, SurfaceId::Preferences);
    }),
    ("route DesignManagement", |state| {
        navigate(state, SurfaceId::DesignManagement);
    }),
    ("route AccountOrganization", |state| {
        navigate(state, SurfaceId::AccountOrganization);
    }),
    ("route JobsManager", |state| {
        navigate(state, SurfaceId::JobsManager);
    }),
    ("route SpecialistToolBrowser", |state| {
        navigate(state, SurfaceId::SpecialistToolBrowser);
    }),
    ("route NotificationCenter", |state| {
        navigate(state, SurfaceId::NotificationCenter);
    }),
    ("dialogs", |state| state.shell.about = true),
    ("ui.netlist, in the Netlist workspace", |state| {
        state.workbench.activate(Workspace::Netlist);
        state.code.netlist.find.open = true;
    }),
    ("sim_setup.session.palette_open", |state| {
        state.model.sim_setup.session.palette_open = true;
    }),
    ("tabbed_property_dialog", |state| {
        state.design.tabbed_property_dialog.open = true;
    }),
    ("pdk_settings_dialog", |state| {
        state.models.pdk_settings_dialog.open = true;
    }),
    ("model_browser_state", |state| {
        state.models.model_browser_state.open = true;
    }),
];

/// Each term of the application gate makes the application modal on its
/// own, and a new session is not modal.
#[test]
fn each_term_of_the_application_gate_makes_it_modal_on_its_own() {
    assert!(
        !AppState::default().application_modal_open(),
        "a new session is not modal"
    );
    for (term, open) in APPLICATION_GATE {
        let mut state = AppState::default();
        open(&mut state);
        assert!(
            state.application_modal_open(),
            "`{term}` alone makes the application modal"
        );
    }
}

/// Drawn while open, and not a term of the gate today: a keystroke still
/// reaches the document beneath. This records the gate as it stands rather
/// than ruling on each row; changing one is a behaviour change.
const OUTSIDE_THE_APPLICATION_GATE: &[GateRow] = &[
    ("ui.netlist, outside the Netlist workspace", |state| {
        state.code.netlist.find.open = true;
    }),
    ("ui.code_workspace.source_document_dialog", |state| {
        state.code.code_workspace.source_document_dialog = true;
    }),
    ("ui.code_workspace.veriloga.import_form", |state| {
        state.code.code_workspace.veriloga.import_form = Some(
            crate::workbench::documents::code_workspace::VerilogAImportFormState {
                project_id: state.model.workspace.content.project.id(),
                file_name: "gain.va".to_owned(),
                contents: String::new(),
                library: "behavioral".to_owned(),
                modules: Ok(Vec::new()),
                error: None,
            },
        );
    }),
];

#[test]
fn application_modals_own_global_input_exclusively() {
    let mut state = AppState::default();
    assert!(!state.application_modal_open());

    state.project.project_launcher_open = true;
    assert!(state.application_modal_open());
    state.project.project_launcher_open = false;
    state.simulate.verification.regression_baseline_picker_open = true;
    assert!(state.application_modal_open());
    state.simulate.verification.regression_baseline_picker_open = false;
    state.simulate.verification.tuning_review_open = true;
    assert!(state.application_modal_open());
    state.simulate.verification.tuning_review_open = false;
    state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::Preferences),
            RouteTransitionSource::User,
        )
        .expect("Preferences has a registered executor");
    assert!(state.application_modal_open());
    state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::DesignManagement),
            RouteTransitionSource::User,
        )
        .expect("Design Management has a registered executor");
    assert_eq!(state.workbench.workspace, Workspace::Design);
    assert!(state.application_modal_open());
    state
        .workbench
        .navigate(
            SurfaceRoute::surface(SurfaceId::JobsManager),
            RouteTransitionSource::User,
        )
        .expect("jobs manager has a registered executor");
    assert!(state.application_modal_open());
}

#[test]
fn what_is_drawn_outside_the_application_gate_leaves_it_modeless() {
    for (term, open) in OUTSIDE_THE_APPLICATION_GATE {
        let mut state = AppState::default();
        open(&mut state);
        assert!(
            !state.application_modal_open(),
            "`{term}` is not a term of the gate today"
        );
    }
}
