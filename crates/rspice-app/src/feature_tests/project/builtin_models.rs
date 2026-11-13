//! The built-in models band drawn through the whole application.

use egui::Ui;

use crate::workbench::app_state::AppState;
use crate::workbench::surfaces::project::library::builtin_models::{
    BuiltinModelsIntent, browse, detail, selected_model,
};

/// The band's text for `state` on a page `width` wide.
fn band_text(state: &AppState, width: f32) -> Vec<String> {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(width, 120.0),
        )),
        ..Default::default()
    };
    let pass = |ctx: &mut Ui| {
        egui::CentralPanel::default().show(ctx, |ui| {
            let _ = detail(
                ui,
                &state.project.builtin_models,
                &state.model.library_manager,
            );
        });
    };
    // Two passes: the first builds the fonts the band is measured in.
    let _ = ctx.run_ui(input(), pass);
    ctx.run_ui(input(), pass)
        .shapes
        .into_iter()
        .filter_map(|clipped| match clipped.shape {
            egui::Shape::Text(text) => Some(text.galley.text().to_owned()),
            _ => None,
        })
        .collect()
}

#[test]
fn the_band_states_the_summary_and_clips_it_before_the_buttons() {
    let mut state = AppState::default();
    browse(
        &mut state.project.builtin_models,
        BuiltinModelsIntent::ShowLibrary,
    );
    let model = selected_model(&state.project.builtin_models).expect("a selected model");

    let wide = band_text(&state, 1440.0);
    assert!(wide.iter().any(|text| text == model.summary), "{wide:?}");
    assert!(wide.iter().any(|text| text == "Open read-only"), "{wide:?}");
    let version = format!("version {}", model.version);
    for gone in [model.class.label(), version.as_str(), "read only"] {
        assert!(
            !wide.iter().any(|text| text == gone),
            "{gone:?} in {wide:?}"
        );
    }

    let narrow = band_text(&state, 280.0);
    let clipped = narrow
        .iter()
        .filter(|text| *text != "Copy cell\u{2026}")
        .find_map(|text| text.strip_suffix('\u{2026}'))
        .unwrap_or_else(|| panic!("no clipped summary in {narrow:?}"));
    assert!(model.summary.starts_with(clipped), "{clipped:?}");
    assert!(
        narrow.iter().any(|text| text == "Open read-only"),
        "{narrow:?}"
    );
}
