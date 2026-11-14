//! The configuration dialogs as a reader meets them.

use super::*;

#[test]
fn starting_from_default_writes_every_view_default_stops_at() {
    let (views, stops) = StartFrom::Default.lists();
    assert_eq!(views[..3], ["schematic", "extracted", "spice"]);
    let default_stops = DEFAULT_STOP_VIEW_TYPES
        .iter()
        .map(ViewType::display_name)
        .collect::<Vec<_>>();
    assert_eq!(stops, default_stops);
    for stop in &stops {
        assert!(
            views.contains(stop),
            "stop {stop:?} must be in the view list {views:?}"
        );
    }
}

#[test]
fn default_offers_only_a_new_configuration() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut offered = Vec::new();
    for selection in [
        ConfigurationSelection::Default,
        ConfigurationSelection::Configuration(ConfigurationSetId::new()),
    ] {
        let output = ctx.run_ui(Default::default(), |ui| {
            egui::CentralPanel::default().show(ui, |ui| {
                configuration_actions_menu(ui, selection, true);
            });
        });
        // The accessibility tree is unordered; the set is what is pinned.
        let mut labels = output
            .platform_output
            .accesskit_update
            .unwrap()
            .nodes
            .iter()
            .filter(|(_, node)| node.role() == egui::accesskit::Role::Button)
            .filter_map(|(_, node)| node.label().map(str::to_owned))
            .collect::<Vec<_>>();
        labels.sort();
        offered.push(labels);
    }
    assert_eq!(offered[0], ["New configuration\u{2026}"]);
    assert_eq!(
        offered[1],
        [
            "Clone\u{2026}",
            "Delete",
            "New configuration\u{2026}",
            "Rename\u{2026}"
        ]
    );
}
