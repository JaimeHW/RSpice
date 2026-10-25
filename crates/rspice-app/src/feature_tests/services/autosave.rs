//! Autosave's failure notice, through the application that shows it.

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::session::autosave::autosave_tick;

/// A checkpoint that could not be written is said when it happens: the
/// user is relying on it, and a bell record is found too late.
#[test]
fn a_checkpoint_that_cannot_be_written_is_reported() {
    // A file where the checkpoint's folder has to be.
    let blocker = std::env::temp_dir().join(format!(
        "rspice-autosave-blocker-{}-{}",
        std::process::id(),
        crate::time_compat::unix_epoch().as_nanos()
    ));
    std::fs::write(&blocker, b"not a folder").expect("write the blocking file");

    let mut app = RSpiceApp::test_instance();
    app.state.platform.autosave_minutes = 1;
    app.state.model.schematic.session.is_dirty = true;
    app.state.model.schematic.session.current_file = Some(blocker.join("design.rspice"));
    app.autosave_last = Some(std::time::Instant::now() - std::time::Duration::from_secs(2 * 60));
    let ctx = egui::Context::default();
    autosave_tick(
        &mut app.state.platform,
        &mut app.state.model,
        &mut app.autosave_last,
        &ctx,
    );
    app.state.drain_notices(&ctx);
    {
        let activity = app.state.workbench.toasts.activity();
        assert_eq!(activity.len(), 1);
        assert_eq!(activity[0].title(), "Autosave failed");
        assert_eq!(activity[0].kind(), rspice_ui_kit::widgets::ToastKind::Error);
    }

    // The failure persists through the next interval: it was said when
    // it started, and an error that stays until answered is not raised
    // again for every checkpoint it goes on preventing.
    app.autosave_last = Some(std::time::Instant::now() - std::time::Duration::from_secs(2 * 60));
    autosave_tick(
        &mut app.state.platform,
        &mut app.state.model,
        &mut app.autosave_last,
        &ctx,
    );
    app.state.drain_notices(&ctx);
    let _ = std::fs::remove_file(&blocker);
    assert_eq!(app.state.workbench.toasts.activity().len(), 1);
    assert!(app.state.platform.autosave_failing);
}
