//! The navigator's own header and search field: their spacing against the
//! mockup and the names they give assistive technology.

use crate::workbench::RSpiceApp;
use crate::workbench::state::Workspace;
use rspice_app_workbench::workbench::docks::navigator::{
    header, panel_search, panel_search_field_width,
};

#[test]
fn navigator_search_and_empty_hint_match_mockup_spacing() {
    assert_eq!(panel_search_field_width(260.0), 244.0);
}

#[cfg(not(target_arch = "wasm32"))]
#[test]
fn navigator_heading_and_search_expose_explicit_accesskit_names() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.activate(Workspace::Design);
    let mut query = String::new();
    let mut focus_pending = false;

    let nodes = ctx
        .run_ui(Default::default(), |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                ui.set_width(260.0);
                header(ui, &mut app);
                panel_search(
                    ui,
                    &mut query,
                    "navigator-accessibility-test",
                    "Find instance, net or port",
                    &mut focus_pending,
                );
            });
        })
        .platform_output
        .accesskit_update
        .expect("AccessKit tree update")
        .nodes;

    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::Heading
            && node.label() == Some("Design navigator")
            && node.level() == Some(2)
    }));
    assert!(nodes.iter().any(|(_, node)| {
        node.role() == egui::accesskit::Role::TextInput
            && node.label() == Some("Find instance, net or port")
    }));
}
