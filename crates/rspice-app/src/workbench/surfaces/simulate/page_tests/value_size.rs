//! Every value a Studio page's fields hold reads at one size.

use egui::accesskit::Role;
use egui::{Rect, vec2};

use super::super::pages;
use crate::state::SimulationPage;
use crate::workbench::RSpiceApp;

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
                        // The Analyses route is drawn by the surface itself and
                        // never reaches `pages::show`, so its page is the
                        // whole surface.
                        if page == SimulationPage::Analyses {
                            super::super::show(ui, &mut app);
                        } else {
                            egui::ScrollArea::vertical()
                                .show(ui, |ui| pages::show(ui, &mut app, page));
                        }
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

/// Each field's value (or, while it is empty, its hint) as painted inside the
/// field, with the size it is painted at.
fn painted_field_values(page: SimulationPage) -> Vec<(String, f32)> {
    let (texts, nodes) = paint(page);
    let mut values = Vec::new();
    for node in nodes.values() {
        if !matches!(
            node.role(),
            Role::TextInput | Role::MultilineTextInput | Role::ComboBox | Role::SpinButton
        ) {
            continue;
        }
        let Some(bounds) = node.bounds() else {
            continue;
        };
        let bounds = Rect::from_min_max(
            egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
            egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
        );
        // The value, or the hint an empty field shows in its place: either is
        // what the field reads as.
        values.extend(
            texts
                .iter()
                .filter(|(text, _, at)| !text.trim().is_empty() && bounds.contains(*at))
                .map(|(text, font, _)| (text.clone(), font.size)),
        );
    }
    values
}

/// A field's value reads at the field size on every Studio page: the
/// Analyses page's integration wells (empty, so their hints) beside its
/// transient fields, the Solver page's temperature, the run set's typed
/// values. A well that set a size of its own read a size under the field
/// beside it.
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
    assert_eq!(font_of("No prior datasets"), crate::ui::theme::field_sans());
    let statement = texts
        .iter()
        .find(|(text, _, _)| text.starts_with(".tran"))
        .map(|(_, font, _)| font.clone())
        .expect("the plan statement is painted");
    assert_eq!(statement, crate::ui::theme::field_mono());
}
