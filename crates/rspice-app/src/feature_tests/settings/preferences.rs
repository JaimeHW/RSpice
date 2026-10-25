//! Preferences drawn as the frame's dialog step draws it, in the case that
//! reads another feature's state: a restore reaches the open waveform view.

use crate::workbench::RouteTransitionSource;
use crate::workbench::app::dialogs::preferences::preferences_shell::PreferenceCategory;
use crate::workbench::app::dialogs::preferences::tests::{preferences_route, test_app};
use crate::workbench::app::dialogs::preferences::*;
use egui::Context;

/// Restoring Results and precision starts the open waveform view from
/// the restored defaults in the frame the reader confirms it, as a new
/// view would start: wrapped, and without the minor grid.
#[test]
fn restored_waveform_defaults_reach_the_open_view_in_the_same_frame() {
    let ctx = Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut app = test_app();
    app.state
        .workbench
        .navigate(preferences_route(), RouteTransitionSource::User)
        .unwrap();
    app.state.settings.preferences_open = true;
    app.state.results.results.session.phase_continuous = true;
    app.state.results.results.session.show_minor_grid = true;
    set_pending_restore(&ctx, Some(PreferenceCategory::Results));

    let _ = ctx.run_ui(egui::RawInput::default(), |ctx| {
        preferences_dialogs(&mut app.state).render(ctx);
        app.after_contribution();
    });
    assert!(
        app.state.results.results.session.phase_continuous,
        "nothing is restored until the reader confirms"
    );

    let _ = ctx.run_ui(
        egui::RawInput {
            events: vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
            ..egui::RawInput::default()
        },
        |ctx| {
            preferences_dialogs(&mut app.state).render(ctx);
            app.after_contribution();
        },
    );
    assert!(pending_restore(&ctx).is_none());
    assert!(!app.state.results.results.session.phase_continuous);
    assert!(!app.state.results.results.session.show_minor_grid);
    assert!(
        app.state
            .workbench
            .outbox
            .peek::<crate::workbench::framework::requests::ApplyWaveformDefaults>()
            .is_none(),
        "the dialog takes the request it raised before it returns"
    );
}
