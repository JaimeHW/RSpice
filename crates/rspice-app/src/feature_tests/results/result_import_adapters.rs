//! A result import carried through a project reload by a whole application.

use crate::state::ResultImportFormat;
use crate::workbench::app_state::AppState;
use crate::workbench::documents::result_document::ResultsParts;
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

fn generated_fst() -> Vec<u8> {
    use fst_writer::{
        FstFileType, FstInfo, FstScopeType, FstSignalType, FstVarDirection, FstVarType,
    };
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "rspice-result-import-{}-{nonce}.fst",
        std::process::id()
    ));
    let info = FstInfo {
        start_time: 0,
        timescale_exponent: -9,
        version: "RSpice import fixture".to_owned(),
        date: "2026-08-31".to_owned(),
        file_type: FstFileType::Verilog,
    };
    let mut header = fst_writer::open_fst(&path, &info).expect("open FST fixture");
    header
        .scope("top", "top", FstScopeType::Module)
        .expect("FST scope");
    let clock = header
        .var(
            "clock",
            FstSignalType::bit_vec(1),
            FstVarType::Wire,
            FstVarDirection::Implicit,
            None,
        )
        .expect("FST signal");
    header.up_scope().expect("FST upscope");
    let mut body = header.finish().expect("FST header");
    for tick in 0..96_u64 {
        body.time_change(tick).expect("FST time change");
        body.signal_change(clock, if tick & 1 == 0 { b"0" } else { b"1" })
            .expect("FST signal change");
    }
    body.finish().expect("FST finish");
    let bytes = std::fs::read(&path).expect("read FST fixture");
    let _ = std::fs::remove_file(&path);
    bytes
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
