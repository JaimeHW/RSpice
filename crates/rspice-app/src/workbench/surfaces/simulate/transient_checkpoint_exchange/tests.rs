//! One accepted checkpoint through every file action. Export writes the packed
//! bytes unchanged; an import that lands after the design changed is dropped;
//! the byte limit is enforced before decoding; a repeated import is stored once
//! and survives a project round trip; a stale inspection closes.

use super::*;

#[test]
#[cfg(not(target_arch = "wasm32"))]
fn transient_checkpoint_file_actions_preserve_binary_identity_owner_and_project() {
    use rspice_core::engine::{TransientCheckpointEncoding, TransientStartupMode};
    let deck = rspice_core::Netlist::parse(
        "Exchange\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1p\n.tran 1n 20n\n.end\n",
    )
    .unwrap();
    let (_, images) = rspice_core::Engine::default()
        .run_tran_checkpoint_schedule_with_startup_mode(
            &deck,
            20e-9,
            1e-9,
            TransientStartupMode::OperatingPoint,
            &[7e-9],
        )
        .unwrap();
    let checkpoint = TransientCheckpointEvidence::from_bytes(
        images[0]
            .checkpoint
            .to_bytes(TransientCheckpointEncoding::Packed)
            .unwrap()
            .into(),
    )
    .unwrap();
    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();
    let directory =
        std::env::temp_dir().join(format!("rspice-tran-exchange-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    let path = directory.join("accepted.rspice-tran");
    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path.clone()));
    apply(&ctx, &mut app, Action::Export(checkpoint.clone()));
    poll(&ctx, &mut app);
    assert_eq!(std::fs::read(&path).unwrap(), checkpoint.bytes());

    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path.clone()));
    apply(&ctx, &mut app, Action::Import);
    app.state.design_execution_epoch += 1;
    poll(&ctx, &mut app);
    assert!(
        app.state
            .simulation
            .retained
            .imported_transient_checkpoints
            .is_empty()
    );
    assert!(file_exchange::take_opened_binary(&ctx, import_id()).is_none());

    // Size is checked before decoding, while binary payloads never visit UTF-8.
    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path.clone()));
    file_exchange::open_binary_file(&ctx, import_id(), KIND, checkpoint.bytes().len() - 1).unwrap();
    assert!(
        file_exchange::take_opened_binary(&ctx, import_id())
            .unwrap()
            .unwrap_err()
            .contains("byte limit")
    );

    let before = crate::workbench::lifecycle::project_lifecycle::snapshot(&app.state).unwrap();
    for _ in 0..2 {
        file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path.clone()));
        apply(&ctx, &mut app, Action::Import);
        poll(&ctx, &mut app);
    }
    let after = crate::workbench::lifecycle::project_lifecycle::snapshot(&app.state).unwrap();
    assert_ne!(
        before.file.simulation_results,
        after.file.simulation_results
    );
    assert_eq!(
        app.state
            .simulation
            .retained
            .imported_transient_checkpoints
            .iter()
            .count(),
        1
    );
    let json = crate::io::project_io::serialize_project_file(&after).unwrap();
    let restored = crate::io::project_io::load_project_text(&json, None).unwrap();
    assert!(restored.file.simulation_results_warning.is_none());
    assert_eq!(
        restored
            .file
            .simulation_results
            .imported_transient_checkpoints
            .get(checkpoint.digest()),
        Some(&checkpoint)
    );

    apply(
        &ctx,
        &mut app,
        Action::Inspect("accepted".into(), checkpoint.clone()),
    );
    assert_eq!(
        ctx.data(|data| data.get_temp::<Inspection>(inspection_id()))
            .unwrap()
            .checkpoint
            .accepted_time(),
        checkpoint.accepted_time()
    );
    app.state.design_execution_epoch += 1;
    poll(&ctx, &mut app);
    assert!(
        ctx.data(|data| data.get_temp::<Inspection>(inspection_id()))
            .is_none()
    );
    apply(&ctx, &mut app, Action::Remove(checkpoint.digest()));
    assert!(
        app.state
            .simulation
            .retained
            .imported_transient_checkpoints
            .is_empty()
    );

    std::fs::write(&path, b"wrong checkpoint kind").unwrap();
    file_exchange::script_next_choice(file_exchange::ScriptedChoice::Chose(path));
    apply(&ctx, &mut app, Action::Import);
    poll(&ctx, &mut app);
    assert!(
        app.state
            .simulation
            .retained
            .imported_transient_checkpoints
            .is_empty()
    );
    assert_eq!(file_exchange::scripted_choices_remaining(), 0);
    for entry in std::fs::read_dir(&directory).unwrap() {
        let entry = entry.unwrap();
        assert!(entry.file_type().unwrap().is_file());
        std::fs::remove_file(entry.path()).unwrap();
    }
    std::fs::remove_dir(directory).unwrap();
}
