//! What New Verilog-A device… refuses, and what it makes when it does not.

use super::*;
use crate::state::{Library, SymbolDocument};
use rspice_design::library::metadata::decode_source_view_ports;
use rspice_design::symbol::generated::is_generated_symbol;

const LIBRARY: &str = "behavioral";

/// An open project with a writable `behavioral` library holding `taken`, and
/// the form open on it with `name` and `ports` typed.
fn app_with_draft(name: &str, ports: &str) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    app.state.project_lifecycle.authority.open_session();
    let mut library = Library::new(LIBRARY);
    library.add_cell(Cell::new("taken"));
    app.state.library_manager.add_library(library);
    open_new_veriloga_device_form(&mut app.state);
    let form = &mut app.state.dialogs.new_veriloga_device;
    form.library = LIBRARY.to_owned();
    form.name = name.to_owned();
    form.ports = ports.to_owned();
    app
}

fn refusal(app: &RSpiceApp) -> String {
    match draft(&app.state) {
        Draft::Refused(message) => message,
        other => panic!("the draft is not refused: {other:?}"),
    }
}

fn view<'a>(app: &'a RSpiceApp, cell: &str, view: &str) -> Option<&'a View> {
    app.state
        .library_manager
        .get_library(LIBRARY)?
        .get_cell(cell)?
        .get_view(view)
}

fn root_source(app: &RSpiceApp, reference: &CellViewRef) -> String {
    app.state
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::cell_view(reference.clone()))
        .expect("the view owns its source")
        .root()
        .content()
        .to_owned()
}

#[test]
fn a_cell_name_the_library_already_holds_is_refused_in_the_form() {
    let mut app = app_with_draft("taken", "");
    assert_eq!(refusal(&app), "behavioral already has a cell named taken.");
    // The same cell under another case is the same canonical identity.
    app.state.dialogs.new_veriloga_device.name = "TAKEN".to_owned();
    assert_eq!(refusal(&app), "behavioral already has a cell named TAKEN.");
    let revision = app.state.workspace.content.project.revision();

    assert!(app.commit_new_veriloga_device().is_err());
    assert_eq!(app.state.workspace.content.project.revision(), revision);

    // A name the module could not be named after is refused too.
    app.state.dialogs.new_veriloga_device.name = "2stage".to_owned();
    assert!(refusal(&app).starts_with("A cell name starts with a letter"));
    app.state.dialogs.new_veriloga_device.name = String::new();
    assert_eq!(draft(&app.state), Draft::Empty);
}

#[test]
fn a_port_entry_the_codec_cannot_read_is_refused_in_the_form() {
    for (ports, expected) in [
        (
            "inp:in out:sideways",
            "out:sideways: sideways is not a direction; use in, out or inout.",
        ),
        (
            "inp:",
            "inp:: name a direction after the colon: in, out or inout.",
        ),
        (
            "vdd:supply",
            "vdd:supply: a Verilog-A port is in, out or inout.",
        ),
        (
            "9x:in",
            "9x:in: a port is a name such as inp, or a bus such as d[7:0].",
        ),
        (
            "a<3:0>",
            "a<3:0>: a port is a name such as inp, or a bus such as d[7:0].",
        ),
        ("out:out OUT:in", "OUT is listed twice."),
    ] {
        let app = app_with_draft("amp", ports);
        assert_eq!(refusal(&app), expected, "{ports}");
    }

    let app = app_with_draft("dac", "d[7:0]:in clk:input out:output vdd");
    assert_eq!(
        draft(&app.state),
        Draft::Ready {
            cell: "dac".to_owned(),
            ports: vec![
                PortSpec {
                    name: "d[7:0]".to_owned(),
                    direction: PortDirection::In,
                },
                PortSpec {
                    name: "clk".to_owned(),
                    direction: PortDirection::In,
                },
                PortSpec {
                    name: "out".to_owned(),
                    direction: PortDirection::Out,
                },
                PortSpec {
                    name: "vdd".to_owned(),
                    direction: PortDirection::InOut,
                },
            ],
        }
    );
}

/// The cell, its module, the module's compiled interface and the symbol it
/// generates arrive as one library transaction, and the module opens in
/// Design.
#[test]
fn creating_a_device_makes_the_cell_its_module_and_its_generated_symbol_at_once() {
    let mut app = app_with_draft("opamp", "inp:in inn:in out:out vdd vss");
    let revision = app.state.workspace.content.project.revision().get();

    let reference = app
        .commit_new_veriloga_device()
        .expect("the device is created");

    assert_eq!(reference, CellViewRef::new(LIBRARY, "opamp", "veriloga"));
    assert_eq!(
        app.state.workspace.content.project.revision().get(),
        revision + 1,
        "one library transaction"
    );
    assert!(matches!(
        app.state
            .workspace
            .content
            .project
            .library_mutation_audit()
            .last()
            .map(|receipt| receipt.mutation()),
        Some(crate::state::ProjectLibraryMutation::CreateCell { library, cell })
            if library == LIBRARY && cell == "opamp"
    ));
    let source = root_source(&app, &reference);
    assert!(
        source.contains(
            "module rspice_opamp_va(inp, inn, out, vdd, vss);\n    input inp, inn;\n    output out;\n    inout vdd, vss;\n    electrical inp, inn, out, vdd, vss;\nendmodule\n"
        ),
        "{source}"
    );
    let veriloga = view(&app, "opamp", "veriloga").expect("the veriloga view");
    assert_eq!(
        veriloga.metadata.get("veriloga.module").map(String::as_str),
        Some("rspice_opamp_va")
    );
    let interface = decode_source_view_ports(
        veriloga
            .metadata
            .get("veriloga.ports")
            .expect("the compile states the interface"),
    )
    .expect("a structured interface");
    assert_eq!(
        interface
            .iter()
            .map(|port| (port.name.as_str(), port.direction))
            .collect::<Vec<_>>(),
        [
            ("inp", PortDirection::In),
            ("inn", PortDirection::In),
            ("out", PortDirection::Out),
            ("vdd", PortDirection::InOut),
            ("vss", PortDirection::InOut),
        ]
    );
    let symbol = view(&app, "opamp", "symbol").expect("the generated symbol view");
    assert!(is_generated_symbol(symbol));
    assert_eq!(
        SymbolDocument::load_from_view(symbol)
            .expect("the symbol loads")
            .pins
            .iter()
            .map(|pin| pin.name.as_str())
            .collect::<Vec<_>>(),
        ["inp", "inn", "out", "vdd", "vss"]
    );
    assert_eq!(app.state.workspace.content.active_view, reference);
    assert_eq!(
        app.state.workbench.workspace,
        crate::workbench::state::Workspace::Design
    );
    assert!(app.state.ui.code_workspace.veriloga.receipt.is_some());
    assert!(!app.state.dialogs.new_veriloga_device.open);
}

/// The second step, retaining the module's source, fails: the cell made in
/// the first step and the interface the third would write go with it.
#[test]
fn a_failure_after_the_cell_is_made_creates_nothing() {
    let mut app = app_with_draft("opamp", "inp:in out:out");
    let reference = CellViewRef::new(LIBRARY, "opamp", "veriloga");
    app.state
        .workspace
        .content
        .project_sources
        .insert_bundle(
            ProjectSourceBundle::try_new(
                ProjectSourceOwner::cell_view(reference.clone()),
                ProjectSourceLanguage::VerilogA,
                "stale.va",
                "module stale; endmodule\n",
                [],
                [],
            )
            .expect("a bundle"),
        )
        .expect("an orphaned owner the source registry still holds");
    let bundles = app
        .state
        .workspace
        .content
        .project_sources
        .iter_bundles()
        .count();
    let revision = app.state.workspace.content.project.revision();
    let catalog = app.state.library_manager.revision();

    let error = app
        .commit_new_veriloga_device()
        .expect_err("the source cannot be retained");

    assert!(error.contains("could not be retained"), "{error}");
    assert!(view(&app, "opamp", "veriloga").is_none());
    assert!(
        app.state
            .library_manager
            .get_library(LIBRARY)
            .is_some_and(|library| library.get_cell("opamp").is_none()),
        "the cell made before the failure is not published"
    );
    assert_eq!(
        app.state
            .workspace
            .content
            .project_sources
            .iter_bundles()
            .count(),
        bundles
    );
    assert_eq!(app.state.workspace.content.project.revision(), revision);
    assert_eq!(app.state.library_manager.revision(), catalog);
}

/// Ports are a starting point: with none typed, the module declares none and
/// the symbol has no pins until a compile gives it some.
#[test]
fn a_device_typed_without_ports_starts_a_module_that_declares_none() {
    let mut app = app_with_draft("blank", "  ");
    assert_eq!(
        ports_line("  ", &app.state.ui.messages()).as_deref(),
        Some("no ports yet · the module defines them when it compiles")
    );

    let reference = app
        .commit_new_veriloga_device()
        .expect("the device is created");

    let source = root_source(&app, &reference);
    assert!(
        source.contains("module rspice_blank_va;\nendmodule\n"),
        "{source}"
    );
    assert!(!source.contains("electrical"), "{source}");
    assert_eq!(
        view(&app, "blank", "veriloga")
            .and_then(|view| view.metadata.get("veriloga.ports"))
            .map(String::as_str),
        Some("[]")
    );
    let symbol = view(&app, "blank", "symbol").expect("the generated symbol view");
    assert!(is_generated_symbol(symbol));
    assert!(
        SymbolDocument::load_from_view(symbol)
            .expect("the symbol loads")
            .pins
            .is_empty()
    );
}

/// The form opens on the library holding the active design when it can be
/// written, and on the first writable one when it cannot.
#[test]
fn the_form_opens_on_the_active_designs_library() {
    let mut app = app_with_draft("", "");
    let active = app.state.workspace.content.active_view.library.clone();
    open_new_veriloga_device_form(&mut app.state);
    assert_eq!(app.state.dialogs.new_veriloga_device.library, active);

    app.state
        .library_manager
        .get_library_mut(&active)
        .expect("the active design's library")
        .read_only = true;
    open_new_veriloga_device_form(&mut app.state);
    assert_eq!(app.state.dialogs.new_veriloga_device.library, LIBRARY);
}
