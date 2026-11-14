//! The symbol definition import workflow driven through a whole application.

use crate::state::SymbolSourceContract;
use crate::workbench::app::dialogs::symbol_definition::import_workflow::*;
use crate::workbench::app::dialogs::symbol_definition::state::SymbolImportBindingChoice;

#[test]
fn review_only_svg_import_keeps_real_geometry_without_an_executable_contract() {
    let mut app = crate::workbench::app_state::AppState::default();
    app.open_project_session();
    app.design.symbol_import.source_name = "review.svg".to_owned();
    app.design.symbol_import.source_text =
        r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="0" y="0" width="40" height="20"/></svg>"#
            .to_owned();
    app.design.symbol_import.target_library =
        app.model.workspace.content.active_view.library.clone();
    app.design.symbol_import.target_name = "review_graphic".to_owned();
    app.design.symbol_import.binding_choice = SymbolImportBindingChoice::UnboundForReview;

    let imported =
        import_candidate(&app.design.symbol_import, &app.model).expect("review-only SVG import");

    assert!(
        imported
            .definition
            .source
            .is_explicitly_unbound_for_review()
    );
    assert!(imported.definition.netlist.template.is_empty());
    assert_eq!(imported.report.primitive_count, 1);
    assert_eq!(
        imported
            .definition
            .imported_graphic
            .as_ref()
            .expect("retained imported geometry")
            .shapes
            .len(),
        1
    );
}

#[test]
fn import_candidate_never_overwrites_an_existing_cell() {
    let mut app = crate::workbench::app_state::AppState::default();
    app.open_project_session();
    let library_name = app.model.workspace.content.active_view.library.clone();
    app.design.symbol_import.source_name = "review.svg".to_owned();
    app.design.symbol_import.source_text =
        r#"<svg><line x1="0" y1="0" x2="10" y2="10"/></svg>"#.to_owned();
    app.design.symbol_import.target_library = library_name.clone();
    app.design.symbol_import.target_name = "existing".to_owned();
    app.design.symbol_import.binding_choice = SymbolImportBindingChoice::UnboundForReview;
    app.model
        .library_manager
        .get_library_mut(&library_name)
        .expect("project library")
        .add_cell(crate::state::Cell::new("existing"));

    assert!(
        import_candidate(&app.design.symbol_import, &app.model)
            .expect_err("overwrite must fail closed")
            .contains("already exists")
    );
}

/// An LTspice symbol binds the subcircuit its `SpiceModel` names, through
/// its pins in `SpiceOrder`, once a library holding it is imported.
#[test]
fn an_ltspice_symbol_binds_the_subcircuit_it_names() {
    let dir = std::env::temp_dir().join(format!(
        "rspice-asy-binding-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).expect("fixture directory");
    let model_file = dir.join("testamp.lib");
    std::fs::write(
        &model_file,
        "* RSpice test amplifier\n\
         .subckt TESTAMP 1 2 99 50 45\n\
         E1 45 0 1 2 1e5\n\
         R1 99 50 1meg\n\
         .ends TESTAMP\n",
    )
    .expect("model file");
    let mut app = crate::workbench::app_state::AppState::default();
    app.open_project_session();
    app.model
        .model_library_manager
        .load_library_file(&model_file, None)
        .expect("model library imports");
    std::fs::remove_dir_all(&dir).ok();

    let mut asy = "Version 4\nSymbolType CELL\n\
                   LINE Normal -32 -32 32 0\nLINE Normal -32 32 32 0\n\
                   LINE Normal -32 -32 -32 32\n\
                   SYMATTR Prefix X\nSYMATTR SpiceModel TESTAMP\n\
                   SYMATTR ModelFile lib/sub/testamp.lib\n"
        .to_owned();
    for (order, (name, x, y)) in [
        ("In+", -48, 16),
        ("In-", -48, -16),
        ("V+", 0, -48),
        ("V-", 0, 48),
        ("OUT", 48, 0),
    ]
    .into_iter()
    .enumerate()
    {
        asy.push_str(&format!(
            "PIN {x} {y} NONE 0\nPINATTR PinName {name}\nPINATTR SpiceOrder {}\n",
            order + 1
        ));
    }
    app.design
        .symbol_import
        .accept_source("testamp.asy".to_owned(), asy.clone());
    assert_eq!(
        app.design.symbol_import.binding_choice,
        SymbolImportBindingChoice::SymbolAttributes
    );
    app.design.symbol_import.target_library =
        app.model.workspace.content.active_view.library.clone();
    app.design.symbol_import.target_name = "testamp".to_owned();

    let imported = import_candidate(&app.design.symbol_import, &app.model)
        .expect("the symbol binds the model it names");
    let SymbolSourceContract::Model { model, ports } = &imported.definition.source else {
        panic!("a model contract, not {:?}", imported.definition.source);
    };
    assert_eq!(model.model, "TESTAMP");
    assert_eq!(
        ports
            .iter()
            .map(|port| port.name.as_str())
            .collect::<Vec<_>>(),
        ["In_p", "In_n", "V_p", "V_n", "OUT"]
    );
    assert!(imported.report.pin_order_valid && imported.report.binding_valid);

    app.design
        .symbol_import
        .accept_source("missing.asy".to_owned(), asy.replace("TESTAMP", "MISSING"));
    let unresolved = import_candidate(&app.design.symbol_import, &app.model)
        .expect_err("an unimported model is named");
    assert!(
        unresolved.contains("MISSING") && unresolved.contains("testamp.lib"),
        "{unresolved}"
    );
}
