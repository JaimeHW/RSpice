//! A result import carried through a project reload by a whole application.

use crate::state::ResultImportFormat;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::test_fixtures::generated_fst;
use crate::workbench::workflows::result_import_workflow::{
    commit_result_import_draft, stage_imported_result_dataset,
};

/// Stage one imported dataset and commit it, as the import review's Import
/// does.
pub(super) fn apply_imported_result_dataset(
    state: &mut ResultsParts<'_>,
    source_name: &str,
    bytes: &[u8],
) -> Result<(), String> {
    stage_imported_result_dataset(state, source_name, bytes)?;
    commit_result_import_draft(state)
}

#[test]
fn event_source_fst_import_keeps_detected_format_through_project_reload() {
    let mut state = AppState::default();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    crate::workbench::app_state::services_access::accept_loaded_project(&mut state, project, None);
    apply_imported_result_dataset(&mut state.results_parts(), "capture.fst", &generated_fst())
        .unwrap();
    let project = crate::workbench::lifecycle::project_lifecycle::snapshot(&state).unwrap();
    let text = crate::io::project_io::serialize_project_file(&project).unwrap();
    let restored = crate::io::project_io::load_project_text(&text, None).unwrap();
    assert!(restored.file.simulation_results_warning.is_none());
    let restored =
        crate::io::simulation_state_from_results(restored.file.simulation_results).unwrap();
    let analysis = restored.active_analysis().unwrap();
    assert_eq!(
        analysis.import_source.as_ref().unwrap().format,
        ResultImportFormat::Fst
    );
    assert_eq!(
        analysis.result_payload,
        state
            .model
            .simulation
            .active_analysis()
            .unwrap()
            .result_payload
    );
    assert_eq!(
        analysis.result_data_digest(),
        state
            .model
            .simulation
            .active_analysis()
            .unwrap()
            .result_data_digest()
    );
}
