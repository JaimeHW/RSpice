//! Checkpoint exchange tests preserve project ownership and stored file identity.

use super::*;

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn monte_carlo_checkpoint_file_actions_preserve_owner_and_project_storage() {
    let (_, bytes, _) =
        crate::simulation::runner::monte_carlo_checkpoint::tests::completed_checkpoint_fixture();
    let checkpoint = MonteCarloCheckpointEvidence::from_bytes(bytes).unwrap();
    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();
    let directory =
        std::env::temp_dir().join(format!("rspice-mc-exchange-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    let path = directory.join("trials.rspice-mc");
    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path.clone()));
    begin_export(&ctx, &mut app.state, &checkpoint);
    poll(&ctx, &mut app);
    let exported = std::fs::read_to_string(&path).unwrap();
    assert_eq!(
        MonteCarloCheckpointEvidence::from_portable_file(&exported).unwrap(),
        checkpoint
    );

    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path.clone()));
    begin_import(&ctx, &mut app.state);
    app.state.design_execution_epoch += 1;
    poll(&ctx, &mut app);
    assert!(
        app.state
            .simulation
            .imported_monte_carlo_checkpoints
            .is_empty(),
        "stale picker cannot import into a replacement design"
    );
    assert!(file_exchange::take_opened(&ctx, import_id()).is_none());

    let before = crate::workbench::lifecycle::project_lifecycle::snapshot(&app.state).unwrap();
    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path));
    begin_import(&ctx, &mut app.state);
    poll(&ctx, &mut app);
    let after = crate::workbench::lifecycle::project_lifecycle::snapshot(&app.state).unwrap();
    assert_ne!(
        before.simulation_results, after.simulation_results,
        "import invalidates cached project results"
    );
    assert!(
        before
            .simulation_results
            .imported_monte_carlo_checkpoints
            .is_empty()
    );
    assert_eq!(
        after
            .simulation_results
            .imported_monte_carlo_checkpoints
            .get(checkpoint.digest()),
        Some(&checkpoint)
    );
    let json = crate::io::project_io::serialize_project_file(&after).unwrap();
    let restored = crate::io::project_io::load_project_text(&json, None).unwrap();
    assert!(restored.simulation_results_warning.is_none());
    assert_eq!(
        restored
            .simulation_results
            .imported_monte_carlo_checkpoints
            .get(checkpoint.digest()),
        Some(&checkpoint)
    );
    // The session uses the same results projection and keeps imported inputs.
    let session = serde_json::to_string(&app.state).unwrap();
    let restored: AppState = serde_json::from_str(&session).unwrap();
    assert_eq!(
        restored
            .simulation
            .imported_monte_carlo_checkpoints
            .get(checkpoint.digest()),
        Some(&checkpoint)
    );
    assert_eq!(file_exchange::scripted_choices_remaining(), 0);

    let journal = crate::simulation::runner::study::monte_carlo::checkpoint::StudyMonteCarloCheckpoint::from_bytes_with_limits(checkpoint.bytes(), rspice_core::ResourceLimits::default(), &rspice_core::NoAbort).unwrap();
    let inspection = journal.inspection();
    assert_eq!(inspection.successful_trials, 1);
    assert_eq!(inspection.failed_trials, 0);
    assert_eq!(inspection.ranges, ["0"]);
    assert_eq!(inspection.measurements, ["scalar:V(out)"]);
    // Durable publication also leaves its per-destination lease file. This
    // unique test directory has no other writers; remove only its plain files.
    for entry in std::fs::read_dir(&directory).unwrap() {
        let entry = entry.unwrap();
        assert!(entry.file_type().unwrap().is_file());
        std::fs::remove_file(entry.path()).unwrap();
    }
    std::fs::remove_dir(directory).unwrap();
}
