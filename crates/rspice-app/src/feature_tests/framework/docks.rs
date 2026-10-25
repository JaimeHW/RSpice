use crate::workbench::docks::{SplitterKeyboardAction, apply_splitter_step, panel_cache_is_stale};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::WorkbenchSession;

#[test]
fn responsive_and_reset_layouts_discard_egui_panel_memory() {
    assert!(panel_cache_is_stale(Some(256.0), 230.4, false));
    assert!(panel_cache_is_stale(Some(256.0), 256.0, false));
}

#[test]
fn only_a_matching_explicit_resize_retains_egui_panel_memory() {
    assert!(!panel_cache_is_stale(Some(340.0), 340.0, true));
    assert!(panel_cache_is_stale(Some(340.0), 312.0, true));
    assert!(panel_cache_is_stale(None, 340.0, true));
}

#[test]
fn splitter_keyboard_steps_match_the_mockup_contract() {
    assert_eq!(
        apply_splitter_step(256.0, SplitterKeyboardAction::Increase, 12.0),
        Some(268.0)
    );
    assert_eq!(
        apply_splitter_step(312.0, SplitterKeyboardAction::Decrease, 12.0),
        Some(300.0)
    );
    assert_eq!(
        apply_splitter_step(145.0, SplitterKeyboardAction::Increase, 16.0),
        Some(161.0)
    );
    assert_eq!(
        apply_splitter_step(440.0, SplitterKeyboardAction::Reset, 12.0),
        None
    );
}

/// Every dock, dragged larger than the page in it draws, keeps the size it
/// was dragged to once the drag lets go and on every frame after. egui
/// stores a panel's size as the size its contents covered, so a short
/// Problems page or a narrow tree used to pull its dock back to the page
/// the moment the drag ended. The three docks are shown together, in the
/// order the workbench frame shows them, and each case drags one of them
/// away from the window edge it is attached to.
#[test]
fn every_dock_keeps_the_size_it_was_dragged_to() {
    use crate::workbench::RSpiceApp;
    use crate::workbench::docks::{
        CONSOLE_PANEL_ID, INSPECTOR_PANEL_ID, NAVIGATOR_PANEL_ID, show_console, show_inspector,
        show_navigator, synchronize_panel_memory,
    };
    use crate::workbench::state::ConsolePage;

    // Wide, so both side docks are docked and resizable.
    const SCREEN: egui::Vec2 = egui::vec2(1_600.0, 1_000.0);
    const DRAG: f32 = 120.0;
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };

    for (panel_id, grow) in [
        (NAVIGATOR_PANEL_ID, egui::vec2(1.0, 0.0)),
        (INSPECTOR_PANEL_ID, egui::vec2(-1.0, 0.0)),
        (CONSOLE_PANEL_ID, egui::vec2(0.0, -1.0)),
    ] {
        let mut app = RSpiceApp::test_instance();
        app.state.workbench.console_visible = true;
        app.state.workbench.console_page = ConsolePage::Problems;
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        // How far the dragged dock reaches from its window edge, read off
        // its splitter, which egui centres on the dock's free edge.
        let mut frame = |events: Vec<egui::Event>| {
            let layout = LayoutSpec::resolve(SCREEN.x, SCREEN.y, &app.state.workbench);
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, SCREEN)),
                    events,
                    ..Default::default()
                },
                |ui| {
                    synchronize_panel_memory(ui.ctx(), &app, layout);
                    show_navigator(ui, &mut app, layout);
                    show_inspector(ui, &mut app, layout);
                    show_console(ui, &mut app, layout);
                },
            );
            let edge = ctx
                .read_response(egui::Id::new(panel_id).with("__resize"))
                .expect("a resizable dock has a splitter")
                .rect
                .center();
            let size = match panel_id {
                NAVIGATOR_PANEL_ID => edge.x,
                INSPECTOR_PANEL_ID => SCREEN.x - edge.x,
                _ => SCREEN.y - edge.y,
            };
            (edge, size)
        };

        let (grab, opened) = frame(Vec::new());
        let dropped = grab + grow * DRAG;
        frame(vec![egui::Event::PointerMoved(grab)]);
        frame(vec![button(grab, true)]);
        frame(vec![egui::Event::PointerMoved(grab + grow * DRAG * 0.5)]);
        frame(vec![egui::Event::PointerMoved(dropped)]);
        frame(vec![button(dropped, false)]);
        for _ in 0..4 {
            let (_, held) = frame(Vec::new());
            assert!(
                (held - opened - DRAG).abs() < 1.0,
                "{panel_id} was dragged from {opened} to {} px and now stands {held} px",
                opened + DRAG,
            );
        }
        let stored = match panel_id {
            NAVIGATOR_PANEL_ID => app.state.workbench.navigator_width,
            INSPECTOR_PANEL_ID => app.state.workbench.inspector_width,
            _ => app.state.workbench.console_height,
        };
        assert!(
            (stored - opened - DRAG).abs() < 1.0,
            "{panel_id} stored {stored} px, not the {} px it was dragged to",
            opened + DRAG,
        );
    }
}

#[test]
fn splitter_keys_move_the_dock_the_width_class_actually_shows() {
    let mut state = WorkbenchSession::default();
    state.navigator_width = 440.0;
    state.navigator_width_custom = true;
    let layout = LayoutSpec::resolve(1_000.0, 900.0, &state);
    let step = |action| {
        apply_splitter_step(layout.navigator_width, action, 12.0)
            .map(|width| width.clamp(layout.navigator_min_width, layout.navigator_max_width))
    };

    assert_eq!(layout.navigator_width, 320.0);
    assert_eq!(step(SplitterKeyboardAction::Decrease), Some(308.0));
    assert_eq!(step(SplitterKeyboardAction::Increase), Some(320.0));
    assert_eq!(layout.navigator_reset_width, 228.0);
}
