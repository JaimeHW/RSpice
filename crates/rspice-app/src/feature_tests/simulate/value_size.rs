//! Every value a Studio page's fields hold reads at one size.

use egui::{Rect, vec2};

use crate::workbench::RSpiceApp;
use rspice_app_state::state::SimulationPage;

/// A text as painted: what it says, its font and its centre.
type Painted = (String, egui::FontId, egui::Pos2);

/// One Studio page painted at a desktop size: every text it paints, and its
/// accessibility nodes.
fn paint(
    page: SimulationPage,
) -> (
    Vec<Painted>,
    std::collections::HashMap<egui::accesskit::NodeId, egui::accesskit::Node>,
) {
    fn walk(shape: &egui::Shape, into: &mut Vec<Painted>) {
        match shape {
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| walk(shape, into)),
            egui::Shape::Text(text) => into.push((
                text.galley.text().to_owned(),
                text.galley.job.sections[0].format.font_id.clone(),
                text.pos + text.galley.rect.center().to_vec2(),
            )),
            _ => {}
        }
    }
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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
                        super::stage::show_on_stage(ui, &mut app);
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
    (texts, nodes)
}

/// The Analyses card's facts read at the field size, each in its face: the
/// plan statement is deck text, so mono; the prior datasets fact is words
/// ("No prior datasets", "Run 3 finished"), so the body face.
#[test]
fn the_analyses_cards_facts_read_in_their_faces_at_the_field_size() {
    let (texts, _) = paint(SimulationPage::Analyses);
    let font_of = |wanted: &str| {
        texts
            .iter()
            .find(|(text, _, _)| text == wanted)
            .map(|(_, font, _)| font.clone())
            .unwrap_or_else(|| panic!("{wanted:?} is painted"))
    };
    assert_eq!(
        font_of("No prior datasets"),
        rspice_app_workbench::ui::theme::field_sans()
    );
    let statement = texts
        .iter()
        .find(|(text, _, _)| text.starts_with(".tran"))
        .map(|(_, font, _)| font.clone())
        .expect("the plan statement is painted");
    assert_eq!(statement, rspice_app_workbench::ui::theme::field_mono());
}
