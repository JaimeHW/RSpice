//! What `bus_notations` re-spells and — just as deliberately — what it leaves
//! alone: engine scopes stay prefixed, a dotted bus name beats its suffix,
//! and a `#` that is not a declared bus bit (a branch current, a device pin,
//! an undeclared stem) passes through in the deck's own spelling.

use super::*;
use crate::state::{Bus, BusDeclaration, Point, SchematicState};

/// One drawn bus, declared as `declaration` says.
fn drawn_bus(id: u64, declaration: &str) -> Bus {
    let y = i32::try_from(id).expect("a test identifier") * 40;
    Bus::segment(
        id,
        Point::new(0, y),
        Point::new(100, y),
        Some(BusDeclaration::parse(declaration).expect("a bus declaration")),
    )
    .expect("a drawn bus")
}

/// The lane's fixture: one eight-bit bus, drawn once.
fn bus_sheet(declaration: &str) -> SchematicState {
    let mut sheet = SchematicState::default();
    sheet.buses.push(drawn_bus(1, declaration));
    sheet
}

/// The lane's shared fixture: a design that declares `DATA[7:0]` and nothing
/// else. Every owned surface is checked against bit 3 of it, read inside
/// instance `x1`.
fn fixture_notations() -> BusNotations {
    BusNotations::of_sheets(std::iter::once(&bus_sheet("DATA[7:0]")))
}

#[test]
fn a_declared_bit_renders_in_the_notation_its_bus_was_drawn_in() {
    for (declaration, bit) in [("DATA[7:0]", "DATA[3]"), ("DATA<7:0>", "DATA<3>")] {
        let notations = BusNotations::of_sheets(std::iter::once(&bus_sheet(declaration)));

        assert_eq!(notations.display("DATA#3"), bit);
        assert_eq!(notations.display("data#3"), bit.to_ascii_lowercase());
        assert_eq!(notations.display("V(DATA#3)"), format!("V({bit})"));
    }
}

#[test]
fn the_engine_scope_a_bit_was_flattened_through_is_kept() {
    let notations = fixture_notations();

    assert_eq!(notations.display("x1.data#3"), "x1.data[3]");
    assert_eq!(notations.display("V(x1.data#3)"), "V(x1.data[3])");
    assert_eq!(notations.display("v(x1.x2.data#3)"), "v(x1.x2.data[3])");
    assert_eq!(
        notations.display("phase(v(x1.data#3))"),
        "phase(v(x1.data[3]))"
    );
}

/// A bus whose own name carries a dot must win over the suffix it ends with,
/// because dropping segments from the front finds the longer name first.
#[test]
fn a_dotted_bus_name_is_preferred_over_the_suffix_it_ends_with() {
    let mut sheet = bus_sheet("afe.data<7:0>");
    sheet.buses.push(drawn_bus(2, "data[7:0]"));
    let notations = BusNotations::of_sheets(std::iter::once(&sheet));

    assert_eq!(notations.display("x1.afe.data#3"), "x1.afe.data<3>");
    assert_eq!(notations.display("x1.data#3"), "x1.data[3]");
}

/// `BASE#DIGITS` is the deck's bit shape, not proof of a bus. The engine mints
/// the same shape for multiconductor-line conductors and for a periodic-noise
/// reference, and those must reach the reader exactly as the engine named
/// them.
#[test]
fn a_name_no_bus_declares_is_never_reshaped() {
    let notations = fixture_notations();

    for engine_name in [
        "out",
        "0",
        "T1#1",
        "R#0",
        "node#7",
        "v1#branch",
        "x1.r1#branch",
        "DATA#",
        "#3",
        "DATA#3x",
        "OTHER#3",
        "V(out)-V(in)",
        "|V(data#3)|",
    ] {
        assert_eq!(
            notations.display(engine_name),
            engine_name,
            "{engine_name} is not a declared bus bit"
        );
    }
}

/// Two sheets that spell one bus two ways leave it with no design spelling at
/// all: quoting either would name a declaration the other sheet does not have.
#[test]
fn a_bus_declared_in_two_notations_keeps_its_deck_name() {
    let notations = BusNotations::of_sheets([&bus_sheet("DATA[7:0]"), &bus_sheet("DATA<7:0>")]);

    assert_eq!(notations.display("DATA#3"), "DATA#3");
}

/// A vector interface pin declares exactly as a drawn bus does, and mints the
/// same deck bits, so it answers for them too.
#[test]
fn a_vector_interface_pin_declares_its_own_bits() {
    let mut sheet = SchematicState::default();
    let id = sheet.add_component(crate::state::ComponentType::Port, Point::new(0, 0));
    sheet
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed port")
        .value = "BUS<3:0>".to_owned();
    let notations = BusNotations::of_sheets(std::iter::once(&sheet));

    assert_eq!(notations.display("V(x1.bus#2)"), "V(x1.bus<2>)");
}

/// The lane's consistency guard: bit 3 of `DATA[7:0]` inside `x1` reads the
/// same on every surface that shows it, whichever shape that surface hands the
/// boundary — the vector's own name, the leaf of a wrapped one, or the whole
/// wrapper.
#[test]
fn every_results_surface_spells_one_bit_the_same_way() {
    let notations = fixture_notations();
    let bit = "data[3]";

    // The results navigator row, the waveform legend, the table's column
    // heading and the trace manager's row all label a trace by the vector's
    // own name, scope and all.
    assert_eq!(notations.display("x1.data#3"), format!("x1.{bit}"));
    // The operating-point inspector's NODE column shows the leaf alone, its
    // scope already split off into the group above it.
    assert_eq!(notations.display("data#3"), bit);
    // The canvas annotation and the inspector's spoken label carry the whole
    // accessor wrapper.
    assert_eq!(notations.display("V(x1.data#3)"), format!("V(x1.{bit})"));
    assert_eq!(notations.display("v(data#3)"), format!("v({bit})"));
}

/// Those surfaces sit in three different layers and none of their paint
/// functions can be called without a live `Ui`, so the one thing that keeps
/// them agreeing is checked over their source: a site that formats a result
/// name any other way is a second spelling, and a second spelling drifts.
#[test]
fn every_results_surface_paints_through_this_boundary() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    for (path, site) in [
        (
            "workbench/docks/navigator.rs",
            "notations.display(&signal.name)",
        ),
        (
            "workbench/documents/result_document/waves.rs",
            "notations.display(&trace.name)",
        ),
        (
            "workbench/documents/result_document/table.rs",
            "notations.display(name)",
        ),
        (
            "workbench/documents/result_document/op_inspector.rs",
            "notations.display(&leaf)",
        ),
        (
            "workbench/documents/visualization_studio/dock/traces.rs",
            "notations.display(label)",
        ),
        (
            "schematic/view/scene.rs",
            "notations.display(&voltage.name)",
        ),
    ] {
        let source = std::fs::read_to_string(root.join(path))
            .unwrap_or_else(|error| panic!("read {path}: {error}"));
        let production = crate::source_guard::production_source(&source);
        assert!(
            production.contains("bus_notations(") && production.contains(site),
            "{path} no longer renders a result name through `bus_notations`/`{site}`.\n\
             Every surface that shows a solved vector renders it through \
             `simulation::netlist_gen::vector_display`, so a bus bit reads the \
             same everywhere."
        );
    }
}

/// A design that declares nothing changes nothing, which is what a surface
/// with no design behind it must show.
#[test]
fn an_empty_design_leaves_every_name_alone() {
    let notations = BusNotations::default();

    assert_eq!(notations.display("DATA#3"), "DATA#3");
    assert_eq!(notations.display("V(out)"), "V(out)");
}
