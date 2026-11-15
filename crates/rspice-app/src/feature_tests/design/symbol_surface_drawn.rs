//! The symbol canvas's surroundings drawn through a whole application: the
//! way to a first drawing, and the placed preview beside the drawing.

use egui::accesskit::Role;
use egui::{FullOutput, Rect, pos2};

use super::app_state_of;
use crate::feature_tests::replay::{Replay, only};
use crate::state::{Point, SymbolShape};
use crate::workbench::RSpiceApp;
use crate::workbench::app::actions::workspace::tests::state_with_amp_symbol_pin;
use crate::workbench::design::commands::DesignCommand;
use crate::workbench::state::Workspace;

/// The amp whose schematic declares IN; its symbol has the pin, unplaced,
/// and nothing drawn, or one pin placed and a circle drawn.
fn amp(drawn: bool) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let tip = drawn.then(|| Point::new(-40, 0));
    app.state = app_state_of(state_with_amp_symbol_pin("IN", tip));
    if drawn {
        draw_a_circle(&mut app);
    }
    app.state.workbench.activate(Workspace::Design);
    app
}

fn draw_a_circle(app: &mut RSpiceApp) {
    let mut document = app.state.model.load_active_symbol_document().unwrap();
    document.body.push(SymbolShape::Circle {
        center: Point::origin(),
        radius: 20,
        style: Default::default(),
    });
    app.state.store_active_symbol_document(&document).unwrap();
}

/// The symbol canvas's bounds as the frame announces them.
fn canvas(output: &FullOutput) -> Rect {
    let update = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("AccessKit is on");
    let bounds = update
        .nodes
        .iter()
        .find_map(|(_, node)| {
            (node.role() == Role::Canvas)
                .then(|| node.bounds())
                .flatten()
        })
        .expect("the symbol canvas is announced");
    Rect::from_min_max(
        pos2(bounds.x0 as f32, bounds.y0 as f32),
        pos2(bounds.x1 as f32, bounds.y1 as f32),
    )
}

/// A painted run whose text starts with `prefix`, and where it lies.
fn painted(output: &FullOutput, prefix: &str) -> Option<(String, Rect)> {
    fn find(shape: &egui::Shape, prefix: &str) -> Option<(String, Rect)> {
        match shape {
            egui::Shape::Text(text) if text.galley.text().starts_with(prefix) => Some((
                text.galley.text().to_owned(),
                Rect::from_min_size(text.pos, text.galley.size()),
            )),
            egui::Shape::Vec(shapes) => shapes.iter().find_map(|shape| find(shape, prefix)),
            _ => None,
        }
    }
    output
        .shapes
        .iter()
        .find_map(|clipped| find(&clipped.shape, prefix))
}

/// The card that offers to generate a first drawing lies on the canvas
/// and takes none of its room: drawing the first shape fades it where it
/// stands, and the canvas, its zoom and its pan stay exactly where they
/// were. It used to be a band above the canvas, so the canvas jumped up
/// by the band's height the moment the first shape was drawn.
#[test]
fn the_first_shape_fades_the_generate_card_and_the_canvas_stays_put() {
    let mut replay = Replay::of(amp(false));
    let controls = replay.frames(Vec::new());
    only(&controls, "Generate from schematic", Role::Button);
    let before = canvas(&replay.frame(Vec::new()));
    let editor = &replay.app.state.design.symbol.editor;
    let (zoom, pan) = (editor.zoom, editor.pan);

    draw_a_circle(&mut replay.app);
    let fading = replay.frame(Vec::new());
    assert_eq!(canvas(&fading), before, "the canvas stands where it stood");
    assert!(
        painted(&fading, "Generate from schematic").is_some(),
        "the card fades rather than vanishing"
    );
    for _ in 0..5 {
        replay.frame(Vec::new());
    }
    let settled = replay.frame(Vec::new());
    assert!(painted(&settled, "Generate from schematic").is_none());
    assert!(painted(&settled, "No symbol drawn yet").is_none());
    assert_eq!(canvas(&settled), before);
    let editor = &replay.app.state.design.symbol.editor;
    assert_eq!(
        (editor.zoom, editor.pan),
        (zoom, pan),
        "the view did not move"
    );
}

/// Shown from View, the placed preview keeps a column of its own beside
/// the canvas rather than floating over the drawing, and its header states
/// the scale it really draws at; hidden again, the canvas takes the column
/// back. It used to sit over the canvas's corner, under which the drawing
/// could pan, and to read 100% whatever it drew at.
#[test]
fn the_placed_preview_stands_beside_the_drawing_and_states_its_scale() {
    let mut replay = Replay::of(amp(true));
    replay.frames(Vec::new());
    let alone = canvas(&replay.frame(Vec::new()));
    assert!(painted(&replay.frame(Vec::new()), "AS PLACED").is_none());

    replay
        .app
        .execute_command_key(DesignCommand::SymbolPlacedPreview.key());
    assert!(replay.app.state.design.symbol.editor.preview_as_placed);
    replay.frames(Vec::new());
    let shown = replay.frame(Vec::new());
    let beside = canvas(&shown);
    let (header, at) = painted(&shown, "AS PLACED").expect("the preview is drawn");
    assert!(
        at.left() >= beside.right(),
        "the preview stands beside the canvas: {at:?} right of {beside:?}"
    );
    assert!(beside.width() < alone.width());
    let scale = header
        .strip_prefix("AS PLACED \u{b7} ")
        .and_then(|rest| rest.strip_suffix('%'))
        .and_then(|number| number.parse::<f32>().ok())
        .unwrap_or_else(|| panic!("the header states a scale: {header:?}"));
    assert!(scale > 0.0);

    replay
        .app
        .execute_command_key(DesignCommand::SymbolPlacedPreview.key());
    replay.frames(Vec::new());
    let hidden = replay.frame(Vec::new());
    assert!(painted(&hidden, "AS PLACED").is_none());
    assert_eq!(canvas(&hidden), alone, "the canvas takes the column back");
}
