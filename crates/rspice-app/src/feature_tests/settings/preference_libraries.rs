//! The Libraries page's folder lists over a whole application.

use crate::feature_tests::settings::preferences_dialogs;
use crate::workbench::app::dialogs::preferences::preference_pages::libraries::*;
use crate::workbench::app_state::AppState;
use egui::vec2;

type Nodes = Vec<(egui::accesskit::NodeId, egui::accesskit::Node)>;

fn frame(ctx: &egui::Context, state: &mut AppState, events: Vec<egui::Event>) -> Nodes {
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                vec2(1_000.0, 700.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| section(ui, &mut preferences_dialogs(state)));
        },
    );
    output
        .platform_output
        .accesskit_update
        .map(|update| update.nodes)
        .unwrap_or_default()
}

fn labels(nodes: &Nodes) -> Vec<String> {
    nodes
        .iter()
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect()
}

/// Press and release the control `label` names, as a pointer would.
fn click(ctx: &egui::Context, state: &mut AppState, label: &str) {
    let nodes = frame(ctx, state, Vec::new());
    let bounds = nodes
        .iter()
        .find(|(_, node)| node.label() == Some(label))
        .and_then(|(_, node)| node.bounds())
        .unwrap_or_else(|| panic!("no control named {label:?} in {:?}", labels(&nodes)));
    let position = egui::pos2(
        ((bounds.x0 + bounds.x1) * 0.5) as f32,
        ((bounds.y0 + bounds.y1) * 0.5) as f32,
    );
    for pressed in [true, false] {
        frame(
            ctx,
            state,
            vec![
                egui::Event::PointerMoved(position),
                egui::Event::PointerButton {
                    pos: position,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
    }
}

fn context() -> egui::Context {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    ctx
}

#[test]
fn the_list_reorders_and_removes_its_folders() {
    let ctx = context();
    let mut state = AppState::default();
    let first = std::env::temp_dir().join("rspice-library-first");
    let second = std::env::temp_dir().join("rspice-library-second");
    state
        .model
        .preferences
        .set_folders(KEY, vec![first.clone(), second.clone()])
        .expect("two full paths");

    click(&ctx, &mut state, &format!("2. {}", second.display()));
    click(&ctx, &mut state, "Move up");
    assert_eq!(
        state.model.preferences.folders(KEY),
        [second.clone(), first.clone()]
    );

    click(&ctx, &mut state, &format!("2. {}", first.display()));
    click(&ctx, &mut state, "Remove");
    assert_eq!(state.model.preferences.folders(KEY), [second]);
}

/// With nothing listed, adding is the one thing the row offers.
#[test]
fn an_empty_list_offers_only_to_add_a_folder() {
    let ctx = context();
    let mut state = AppState::default();
    frame(&ctx, &mut state, Vec::new());
    let shown = labels(&frame(&ctx, &mut state, Vec::new()));

    assert!(shown.iter().any(|label| label == "Add folder\u{2026}"));
    for absent in ["Move up", "Move down", "Remove"] {
        assert!(
            !shown.iter().any(|label| label == absent),
            "{absent} with no folder to act on"
        );
    }
}

/// A folder this machine does not have is named as missing, in place.
#[test]
fn a_folder_this_machine_lacks_is_marked_missing() {
    let ctx = context();
    let mut state = AppState::default();
    let gone = std::env::temp_dir().join(format!("rspice-library-gone-{}", uuid::Uuid::new_v4()));
    state
        .model
        .preferences
        .set_folders(KEY, vec![std::env::temp_dir(), gone])
        .expect("full paths");
    frame(&ctx, &mut state, Vec::new());
    let output = ctx.run_ui(egui::RawInput::default(), |ctx| {
        egui::CentralPanel::default()
            .show(ctx, |ui| section(ui, &mut preferences_dialogs(&mut state)));
    });
    let missing = output
        .shapes
        .iter()
        .filter(|clipped| {
            matches!(&clipped.shape, egui::Shape::Text(text) if text.galley.text() == "missing")
        })
        .count();
    assert_eq!(missing, 1, "only the folder that is not there is marked");
}
