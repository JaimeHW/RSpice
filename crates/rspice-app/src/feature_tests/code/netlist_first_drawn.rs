//! The netlist-first surface drawn through a whole application.

use super::netlist_first::netlist_first_app;
use crate::workbench::RSpiceApp;
use crate::workbench::documents::netlist_document::root_deck_source_index;
use rspice_app_code::workbench::code::netlist_first::deck_facts;

#[test]
fn imported_deck_with_only_the_pristine_bootstrap_buffer_is_netlist_first() {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(
        "V1 out 0 1\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));

    assert!(app.state.is_netlist_first_without_schematic());

    app.state.model.schematic.add_component(
        rspice_app_state::state::ComponentType::Resistor,
        rspice_app_state::state::Point::new(0, 0),
    );
    assert!(!app.state.is_netlist_first_without_schematic());
}

#[test]
fn deck_facts_read_the_live_document_rather_than_placeholders() {
    let mut app = netlist_first_app();
    let index = root_deck_source_index(&app.state.model, &mut app.state.code.netlist);
    let facts = deck_facts(&app.state.model, &app.state.code.netlist, &index);

    assert!(facts.name.contains("front_end"), "name: {}", facts.name);
    assert_eq!(facts.lines, 13);
    // V1, R1, C1, M1 at the top level and E1 inside the subcircuit.
    assert_eq!(facts.devices, 5);
    // .tran and .op.
    assert_eq!(facts.analyses, 2);
    assert_eq!(facts.models, 1);
    assert_eq!(facts.subcircuits, 1);
    assert_eq!(facts.dependencies, 0);
    assert!(
        facts.publication.is_some(),
        "an owned deck has a publication state"
    );
}
