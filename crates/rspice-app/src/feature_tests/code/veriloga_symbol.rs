//! The symbol a Verilog-A module's compile generates for its cell: pins
//! placed by role, a hand-drawn symbol kept through a recompile and Update
//! symbol, and Open symbol taking the module to the symbol editor.

use super::veriloga_interface::{add_module_cell, compile_and_publish, view};
use crate::state::{CellViewRef, Point, PortDirection, SymbolDocument, SymbolElectricalType};
use crate::workbench::RSpiceApp;
use crate::workbench::app::actions::veriloga_symbol::update_veriloga_symbol;
use crate::workbench::app::veriloga_design_tests::{
    DESKTOP, frame_with, open_in_design, open_project_app, themed_context,
};
use crate::workbench::app::veriloga_interface_tests::edit_source;
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::documents::code_workspace::{
    VerilogACellSymbol, VerilogASymbolDrift, symbol_standing,
};
use rspice_design::symbol::SymbolShape;
use rspice_design::symbol::generated::{GENERATED_SYMBOL_VIEW, is_generated_symbol};
use rspice_design_model::symbol_pin::SymbolPinSide;

const LIBRARY: &str = "behavioral";

/// A transconductance amplifier spelled as op-amps are: two inputs, an
/// output and two supplies, in an order that puts the supplies last.
fn amplifier(extra: &str) -> String {
    format!(
        "module ota(inp, inn, out, vdd, vss{extra});\n  input inp, inn;\n  output out;\n  \
         inout vdd, vss{extra};\n  electrical inp, inn, out, vdd, vss{extra};\n  \
         analog I(out, vss) <+ 1m * V(inp, inn);\nendmodule\n"
    )
}

fn symbol(app: &RSpiceApp, reference: &CellViewRef) -> SymbolDocument {
    SymbolDocument::load_from_view(
        view(app, reference, GENERATED_SYMBOL_VIEW).expect("the cell has a symbol"),
    )
    .expect("the symbol loads")
}

fn terminal(document: &SymbolDocument, name: &str) -> Point {
    document
        .pin(name)
        .and_then(|pin| pin.position)
        .unwrap_or_else(|| panic!("pin {name} is placed"))
}

fn compiled_amplifier(app: &mut RSpiceApp) -> CellViewRef {
    let reference = add_module_cell(&mut app.state, "ota", "ota", &amplifier(""));
    open_in_design(&mut app.state, &reference);
    compile_and_publish(app);
    reference
}

/// Draw on the module's symbol, so it is its author's from then on.
fn hand_draw(app: &mut RSpiceApp, reference: &CellViewRef) -> (SymbolDocument, SymbolShape) {
    let mut drawn = symbol(app, reference);
    let mark = SymbolShape::Polyline {
        points: vec![Point::new(-10, -5), Point::new(10, 5)],
        closed: false,
    };
    drawn.body.push(mark.clone());
    app.state
        .model
        .library_manager
        .edit_library(LIBRARY)
        .and_then(|mut library| library.store_symbol_document("ota", GENERATED_SYMBOL_VIEW, &drawn))
        .expect("the symbol view")
        .expect("the symbol stores");
    (drawn, mark)
}

/// The roles the module's names and directions give reach the generated
/// symbol: the amplifier triangle with + on top, its supplies top and bottom.
#[test]
fn a_compiled_amplifier_draws_the_triangle_with_its_supplies_top_and_bottom() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    let drawn = symbol(&app, &reference);
    assert_eq!(terminal(&drawn, "inp"), Point::new(-40, -20), "+ on top");
    assert_eq!(terminal(&drawn, "inn"), Point::new(-40, 20));
    assert_eq!(terminal(&drawn, "out"), Point::new(40, 0));
    assert_eq!(terminal(&drawn, "vdd"), Point::new(0, -40));
    assert_eq!(terminal(&drawn, "vss"), Point::new(0, 40));
    assert_eq!(
        drawn.pin("vdd").and_then(|pin| pin.side),
        Some(SymbolPinSide::Top)
    );
}

/// A hand-drawn symbol is its author's: a recompile that adds a port leaves
/// the drawing as it is, and Update symbol adds the pin without touching the
/// artwork the author drew.
#[test]
fn a_hand_drawn_symbol_keeps_its_artwork_through_a_recompile_and_update() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    let (drawn, mark) = hand_draw(&mut app, &reference);

    edit_source(&mut app, &amplifier(", en"));
    compile_and_publish(&mut app);
    assert_eq!(symbol(&app, &reference), drawn, "a compile leaves it alone");

    update_veriloga_symbol(&mut app.state.model, &reference).expect("the update applies");
    let updated = symbol(&app, &reference);
    assert!(updated.body.contains(&mark), "the artwork survives");
    assert!(updated.pin("en").and_then(|pin| pin.position).is_some());
    assert_eq!(terminal(&updated, "inp"), terminal(&drawn, "inp"));
    assert!(!is_generated_symbol(
        view(&app, &reference, GENERATED_SYMBOL_VIEW).expect("the symbol")
    ));
}

/// Open symbol takes a compiled module to its symbol in the symbol editor.
#[test]
fn open_symbol_takes_a_compiled_module_to_its_symbol() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    assert!(app.command_key_enabled(CodeCommand::VerilogAOpenSymbol.key()));

    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());

    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new(&reference.library, &reference.cell, GENERATED_SYMBOL_VIEW)
    );
}

/// Update symbol stands by the finding it answers and Open symbol on the
/// toolbar. Neither is repeated anywhere else on the page.
#[test]
fn each_symbol_action_has_one_place_on_the_page() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    hand_draw(&mut app, &reference);
    edit_source(&mut app, &amplifier(", en"));
    compile_and_publish(&mut app);

    let ctx = themed_context();
    ctx.enable_accesskit();
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let output = frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let count = |name: &str| {
        output
            .platform_output
            .accesskit_update
            .iter()
            .flat_map(|update| update.nodes.iter())
            .filter(|(_, node)| node.label() == Some(name))
            .count()
    };
    assert_eq!(count("Update symbol"), 1);
    assert_eq!(count("Open symbol"), 1);
}

/// Update symbol is the whole task: the pin is added and its author is left
/// in the symbol editor with it selected, to put it where it belongs.
#[test]
fn update_symbol_lands_in_the_editor_with_the_new_pin_selected() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    hand_draw(&mut app, &reference);
    edit_source(&mut app, &amplifier(", en"));
    compile_and_publish(&mut app);

    app.state.update_active_veriloga_symbol();
    let ctx = themed_context();
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());

    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new(&reference.library, &reference.cell, GENERATED_SYMBOL_VIEW)
    );
    assert!(
        app.state
            .design
            .symbol
            .editor
            .effective_selection()
            .pins
            .contains("en")
    );
}

/// What a compile drew can be saved as it stands, and Update symbol leaves
/// the rails it drew as rails.
#[test]
fn a_compiled_symbol_saves_and_keeps_its_rails_through_an_update() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());
    assert_eq!(app.state.refuse_unsavable_active_symbol(), None);

    open_in_design(&mut app.state, &reference);
    hand_draw(&mut app, &reference);
    edit_source(&mut app, &amplifier(", en"));
    compile_and_publish(&mut app);
    update_veriloga_symbol(&mut app.state.model, &reference).expect("the update applies");

    let updated = symbol(&app, &reference);
    let rail = updated.pin("vdd").expect("the rail");
    assert_eq!(rail.direction, PortDirection::Supply);
    assert_eq!(rail.electrical_type(), SymbolElectricalType::Power);
    assert!(matches!(
        symbol_standing(&app.state.model.library_manager, &reference),
        Some(VerilogACellSymbol::InStep)
    ));
}

/// The editor's Generate draws what the compile drew.
#[test]
fn the_editor_generates_what_the_compile_drew() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    let drawn = symbol(&app, &reference);
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());

    crate::workbench::app::generate_active_symbol_document(
        &mut app.state.design.symbol,
        &mut app.state.model,
    )
    .expect("the symbol generates");

    assert_eq!(symbol(&app, &reference).pins, drawn.pins);
}

/// The first edit of a compiled symbol raises the unsaved notice. It must
/// not take the drag that made the edit with it, or leave the terminal
/// anywhere but where it was dropped.
#[test]
fn the_first_drag_on_a_compiled_symbol_lands_the_pin_where_it_is_dropped() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());
    let ctx = themed_context();
    ctx.enable_accesskit();
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let output = frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let canvas = output
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .filter(|(_, node)| {
            node.label()
                .is_some_and(|label| label.starts_with("Symbol editor canvas"))
        })
        .find_map(|(_, node)| node.bounds())
        .expect("the canvas is on screen");
    let editor = &app.state.design.symbol.editor;
    let viewport = rspice_schematic_editor::symbol_editor::SymbolViewport::new(
        egui::Rect::from_min_max(
            egui::pos2(canvas.x0 as f32, canvas.y0 as f32),
            egui::pos2(canvas.x1 as f32, canvas.y1 as f32),
        ),
        editor.zoom,
        egui::vec2(editor.pan.0, editor.pan.1),
    );
    let drop_point = Point::new(40, -20);
    let from = viewport.world_to_screen(terminal(&symbol(&app, &reference), "vdd"));
    let to = viewport.world_to_screen(drop_point);
    let button = |pos, pressed| egui::Event::PointerButton {
        pos,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    frame_with(
        &ctx,
        &mut app,
        DESKTOP,
        vec![egui::Event::PointerMoved(from)],
    );
    frame_with(&ctx, &mut app, DESKTOP, vec![button(from, true)]);
    for step in 1..=12 {
        let at = from.lerp(to, step as f32 / 12.0);
        frame_with(&ctx, &mut app, DESKTOP, vec![egui::Event::PointerMoved(at)]);
    }
    frame_with(&ctx, &mut app, DESKTOP, vec![button(to, false)]);

    assert_eq!(terminal(&symbol(&app, &reference), "vdd"), drop_point);
}

/// Pins no port declares are their author's to remove, so Update symbol is
/// not offered for them alone.
#[test]
fn update_symbol_is_offered_only_when_it_has_something_to_do() {
    let drift = |missing: &[&str], extra: &[&str]| VerilogASymbolDrift {
        missing: missing.iter().map(|name| (*name).to_owned()).collect(),
        extra: extra.iter().map(|name| (*name).to_owned()).collect(),
        turned: Vec::new(),
    };
    assert!(drift(&["en"], &[]).updatable());
    assert!(!drift(&[], &["old"]).updatable());
}
