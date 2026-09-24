//! Shared source fixture for the import, snapshot and worker boundaries.

use super::{
    PreparedVerilogARuntimeSet, append_project_veriloga_directive,
    compile_model_library_source_runtimes,
};

pub(crate) fn standalone_connection_authority()
-> crate::state::model_library::SealedModelLibraryVerilogAAuthority {
    let mut rules = rspice_veriloga::connect::library::BUILTIN_CONNECT_MODULES
        .iter()
        .map(|(_, source)| *source)
        .collect::<String>();
    rules.push_str("\nconnectrules Low; connect d2a #(.vsup(1.0)); endconnectrules\nconnectrules High; connect d2a #(.vsup(5.0)); endconnectrules\n");
    let mut manager = crate::state::model_library::ModelLibraryManager::new();
    manager.load_library_bundle(
        "standalone-ui-connections.lib",
        vec![
            ("root.lib".to_owned(), b".va \"driver.vams\" UI_DRIVER\n.va \"rules.vams\" UI_CONNECTIONS\n".to_vec()),
            ("driver.vams".to_owned(), b"module ui_driver(p,q); inout p; electrical p; output q; reg q; initial q=1; analog I(p)<+0; endmodule\n".to_vec()),
            ("rules.vams".to_owned(), rules.into_bytes()),
        ],
        None,
    ).expect("sealed library import retains both source kinds");
    manager
        .seal_execution_sources()
        .unwrap()
        .model_library_veriloga_authority()
        .unwrap()
        .unwrap()
}

pub(crate) fn standalone_connection_fixture() -> (PreparedVerilogARuntimeSet, String) {
    let sources =
        compile_model_library_source_runtimes(&standalone_connection_authority()).unwrap();
    let mut deck = "standalone UI connections\nV1 p 0 1\nX1 p q UI_DRIVER\n.options connectrules=Low connectrules_source=ui_connections\n.tran 0.2n 2n\n.end\n".to_owned();
    for source in sources.sources() {
        assert!(!std::path::Path::new(source.source_key()).exists());
        append_project_veriloga_directive(&mut deck, source.source_key(), source.netlist_alias());
    }
    (sources, deck)
}
