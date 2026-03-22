use super::*;

#[test]
fn test_touchstone_export_path_uses_schematic_file_directory_and_stem() {
    let mut state = AppState::default();
    state.schematic.current_file = Some(std::path::PathBuf::from("designs/opamp_top.rsch"));

    let path = SimulationController::touchstone_export_path(&state, 7, 3, 4);

    assert_eq!(
        path,
        std::path::PathBuf::from("designs/opamp_top_run0007_sp03.s4p")
    );
}

#[test]
fn test_touchstone_export_path_clamps_analysis_and_port_minimums() {
    let mut state = AppState::default();
    state.schematic.current_file = Some(std::path::PathBuf::from("rf/front_end.rsch"));

    let path = SimulationController::touchstone_export_path(&state, 1, 0, 1);

    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("front_end_run0001_sp01.s2p")
    );
}

#[test]
fn test_touchstone_export_path_defaults_to_cwd_for_unsaved_documents() {
    let state = AppState::default();

    let path = SimulationController::touchstone_export_path(&state, 12, 5, 6);

    assert_eq!(path.parent(), std::env::current_dir().ok().as_deref());
    assert_eq!(
        path.file_name().and_then(|name| name.to_str()),
        Some("untitled_run0012_sp05.s6p")
    );
}
