//! Every value a Studio page's fields hold reads at one size.

use egui::accesskit::Role;
use egui::{Rect, vec2};

use super::super::pages;
use crate::state::SimulationPage;
use crate::workbench::RSpiceApp;

/// Each field's value as painted inside the field, with the size it is painted at.
fn painted_field_values(page: SimulationPage) -> Vec<(String, f32)> {
    fn walk(shape: &egui::Shape, into: &mut Vec<(String, f32, egui::Pos2)>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
            egui::Shape::Text(text) => into.push((
                text.galley.text().to_owned(),
                text.galley.job.sections[0].format.font_id.size,
                text.pos + text.galley.rect.center().to_vec2(),
            )),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
    app.state.simulate.simulation_page = page;
    let mut nodes = std::collections::HashMap::new();
    let mut shapes = Vec::new();
    for _ in 0..3 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(egui::Pos2::ZERO, vec2(1100.0, 2600.0))),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| {
                        egui::ScrollArea::vertical().show(ui, |ui| pages::show(ui, &mut app, page));
                    });
            },
        );
        if let Some(update) = output.platform_output.accesskit_update {
            nodes.extend(update.nodes);
        }
        shapes = output.shapes;
    }
    let mut texts = Vec::new();
    for clipped in &shapes {
        walk(&clipped.shape, &mut texts);
    }
    let mut values = Vec::new();
    for node in nodes.values() {
        if !matches!(
            node.role(),
            Role::TextInput | Role::MultilineTextInput | Role::ComboBox
        ) {
            continue;
        }
        let (Some(value), Some(bounds)) = (node.value(), node.bounds()) else {
            continue;
        };
        let bounds = Rect::from_min_max(
            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
        );
        for line in value.lines().filter(|line| !line.trim().is_empty()) {
            values.extend(
                texts
                    .iter()
                    .filter(|(text, _, at)| text == line && bounds.contains(*at))
                    .map(|(text, size, _)| (text.clone(), *size)),
            );
        }
    }
    values
}

/// A field's value reads at the field size on every Studio page: the
/// Analyses page's integration wells beside its transient fields, the
/// Solver page's temperature, the run set's typed values. A well that set
/// a size of its own read a size under the field beside it.
#[test]
fn every_value_a_studio_page_holds_is_painted_at_the_field_size() {
    let field = crate::ui::theme::field_mono().size;
    let mut checked = 0;
    for page in SimulationPage::NAVIGATION {
        for (value, size) in painted_field_values(page) {
            checked += 1;
            assert_eq!(size, field, "{page:?}: {value:?} is painted at {size} px");
        }
    }
    assert!(
        checked >= 10,
        "only {checked} field values were found painted"
    );
}
