//! The symbol a Verilog-A module's compile generates for its cell: pins
//! placed by role, a hand-drawn symbol kept through a recompile and Update
//! symbol, and Open symbol taking the module to the symbol editor.

use std::collections::BTreeSet;

use super::veriloga_interface::{add_module_cell, compile_and_publish, press_strip_button, view};
use crate::state::{CellViewRef, Point, PortDirection, SymbolDocument, SymbolElectricalType};
use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{
    DESKTOP, frame_with, open_in_design, open_project_app, themed_context,
};
use crate::workbench::app::veriloga_interface_tests::edit_source;
use crate::workbench::documents::code_workspace::{
    VerilogACellSymbol, VerilogASymbolDrift, symbol_standing,
};
use crate::workbench::features;
use crate::workbench::state::Workspace;
use rspice_app_code::workbench::app::actions::veriloga_symbol::update_veriloga_symbol;
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_code::workbench::surfaces::veriloga;
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
        style: Default::default(),
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
        drawn.pin("vdd").map(|pin| pin.side),
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

    veriloga::update_symbol(&mut features::cx(&mut app));
    app.after_contribution();
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

/// Update symbol, pressed on the strip, is carried out in the frame it is
/// pressed in: the symbol editor is open on the updated symbol with exactly
/// the pins the update added selected.
#[test]
fn pressing_update_symbol_opens_the_symbol_with_exactly_its_new_pins_selected() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    hand_draw(&mut app, &reference);
    edit_source(&mut app, &amplifier(", en, sel"));
    compile_and_publish(&mut app);
    let ctx = themed_context();

    press_strip_button(&ctx, &mut app, "Update symbol");

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new(&reference.library, &reference.cell, GENERATED_SYMBOL_VIEW)
    );
    let updated = symbol(&app, &reference);
    assert!(updated.pin("en").is_some() && updated.pin("sel").is_some());
    assert_eq!(
        app.state.design.symbol.editor.effective_selection().pins,
        BTreeSet::from(["en".to_owned(), "sel".to_owned()])
    );
}

/// What a compile drew can be saved as it stands, and Update symbol leaves
/// the rails it drew as rails.
#[test]
fn a_compiled_symbol_saves_and_keeps_its_rails_through_an_update() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());
    app.state.run_active_symbol_pin_checks();
    assert_eq!(app.state.active_symbol_pin_finding_count(), Some(0));

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

/// The AccessKit labels and values (a plain label's text is its value) of
/// the last of two whole-application frames.
fn frame_labels(ctx: &egui::Context, app: &mut RSpiceApp) -> Vec<String> {
    frame_with(ctx, app, DESKTOP, Vec::new());
    frame_with(ctx, app, DESKTOP, Vec::new())
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .flat_map(|(_, node)| [node.label(), node.value()])
        .flatten()
        .map(str::to_owned)
        .collect()
}

/// In the symbol editor the module's ports are the symbol's interface: a
/// port the module gained is listed with no pin and counted on the canvas
/// against the module, and Update pins from module adds it.
#[test]
fn the_symbol_editor_answers_a_compiled_symbol_to_its_module() {
    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    hand_draw(&mut app, &reference);
    edit_source(&mut app, &amplifier(", en"));
    compile_and_publish(&mut app);
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());
    let ctx = themed_context();

    let labels = frame_labels(&ctx, &mut app);
    assert!(
        labels.iter().any(|label| label == "en, no pin, Warning"),
        "{labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| label == "1 module pin has no symbol pin"),
        "{labels:?}"
    );

    press_strip_button(&ctx, &mut app, "Update pins from module");

    assert!(
        symbol(&app, &reference)
            .pin("en")
            .and_then(|pin| pin.position)
            .is_some()
    );
    assert!(matches!(
        symbol_standing(&app.state.model.library_manager, &reference),
        Some(VerilogACellSymbol::InStep)
    ));
}

/// A pin the module no longer declares is named against the module: in the
/// pin list, on the canvas and in the finding the pin check publishes.
#[test]
fn a_pin_the_module_dropped_is_named_against_the_module() {
    use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;

    let mut app = open_project_app();
    let reference = add_module_cell(&mut app.state, "ota", "ota", &amplifier(", en"));
    open_in_design(&mut app.state, &reference);
    compile_and_publish(&mut app);
    hand_draw(&mut app, &reference);
    edit_source(&mut app, &amplifier(""));
    compile_and_publish(&mut app);
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());
    let ctx = themed_context();

    let labels = frame_labels(&ctx, &mut app);
    assert!(
        labels
            .iter()
            .any(|label| label == "6. en, not in module, Warning"),
        "{labels:?}"
    );
    assert!(
        labels
            .iter()
            .any(|label| label == "1 pin is not in the module"),
        "{labels:?}"
    );
    app.state.design.symbol.editor.select_pin("en");
    frame_with(&ctx, &mut app, DESKTOP, Vec::new());
    let interface = frame_with(&ctx, &mut app, DESKTOP, Vec::new())
        .platform_output
        .accesskit_update
        .iter()
        .flat_map(|update| update.nodes.iter())
        .find(|(_, node)| node.label() == Some("Interface"))
        .and_then(|(_, node)| node.value().map(str::to_owned));
    assert_eq!(
        interface.as_deref(),
        Some("not in module"),
        "the inspector names the module"
    );

    app.state.run_active_symbol_pin_checks();
    let subject = app.state.model.workspace.content.active_view.clone();
    let message = app
        .state
        .model
        .diagnostics
        .batches()
        .find(|batch| {
            batch.inputs
                == DiagnosticInputs::SymbolPins {
                    subject: subject.clone(),
                }
        })
        .and_then(|batch| {
            batch
                .records
                .iter()
                .find(|record| record.canonical.code.as_ref() == "SYMBOL-ORPHANED-PIN")
        })
        .map(|record| record.message.clone())
        .expect("the stray pin is a finding");
    assert!(
        message.contains("module") && !message.contains("schematic"),
        "{message}"
    );
}

/// A compiled module's symbol is edited, undone, saved and reverted as a
/// drawn symbol is. Its first edit makes it its author's, and undoing that
/// edit gives it back to the compile, which draws it again.
#[test]
fn a_compiled_symbol_undoes_saves_and_reverts_as_a_drawn_one() {
    use crate::workbench::commands::vocabulary::Command;
    use crate::workbench::lifecycle::project_lifecycle::{
        has_unsaved_changes, prepare_revert_active_document,
    };
    use crate::workbench::project::commands::ProjectCommand;

    let mut app = open_project_app();
    let reference = compiled_amplifier(&mut app);
    let path = std::env::temp_dir().join(format!(
        "rspice-veriloga-symbol-{}-{}.rspiceproj",
        std::process::id(),
        uuid::Uuid::new_v4()
    ));
    assert!(app.state.save_project_to_path(&path), "the project saves");
    app.execute_command_key(CodeCommand::VerilogAOpenSymbol.key());
    let symbol_view = || CellViewRef::new(LIBRARY, "ota", GENERATED_SYMBOL_VIEW);
    let compiled = symbol(&app, &reference);
    let revision = |app: &RSpiceApp| {
        app.state
            .model
            .load_symbol_editor_metadata(&symbol_view(), &symbol(app, &reference))
            .expect("the metadata loads")
            .revision
    };
    let saved_revision = revision(&app);
    let move_rail = |app: &mut RSpiceApp, by: Point| {
        let before = symbol(app, &reference);
        let metadata = app
            .state
            .model
            .load_symbol_editor_metadata(&symbol_view(), &before)
            .expect("the metadata loads");
        let mut moved = before.clone();
        let rail = moved
            .pins
            .iter_mut()
            .find(|pin| pin.name == "vdd")
            .expect("the rail");
        rail.position = rail.position.map(|at| at + by);
        crate::workbench::app::record_symbol_edit(
            &mut app.state.design.symbol,
            &app.state.model,
            &before,
            "Move pin",
        );
        app.state
            .store_active_symbol_editor_bundle(&moved, &metadata)
            .expect("the edit stores");
    };
    let generated = |app: &RSpiceApp| {
        is_generated_symbol(view(app, &reference, GENERATED_SYMBOL_VIEW).expect("the symbol"))
    };
    assert!(
        generated(&app),
        "saving left the compile's symbol the compile's"
    );

    move_rail(&mut app, Point::new(20, 0));
    assert!(!generated(&app), "the edit made the symbol its author's");
    app.execute_command_key(Command::Undo.key());
    assert_eq!(symbol(&app, &reference), compiled);
    assert!(generated(&app), "undo gave it back to the compile");
    app.execute_command_key(Command::Redo.key());
    assert_eq!(
        terminal(&symbol(&app, &reference), "vdd"),
        terminal(&compiled, "vdd") + Point::new(20, 0)
    );

    app.execute_command_key(ProjectCommand::Save.key());
    assert!(!has_unsaved_changes(&app.state));
    assert_eq!(
        revision(&app),
        saved_revision + 1,
        "the save recorded the edit"
    );
    let saved = symbol(&app, &reference);

    move_rail(&mut app, Point::new(20, 0));
    let token = prepare_revert_active_document(&app.state).expect("the revert is reviewed");
    assert!(app.state.confirm_revert_active_document(&token));
    assert_eq!(
        symbol(&app, &reference),
        saved,
        "the symbol is as last saved"
    );

    let _ = std::fs::remove_file(&path);
    let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
    let mut lock = path.as_os_str().to_os_string();
    lock.push(".rspice.lock");
    let _ = std::fs::remove_file(std::path::PathBuf::from(lock));
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
