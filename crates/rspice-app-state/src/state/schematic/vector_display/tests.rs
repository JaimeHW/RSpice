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
    sheet
        .document_mut_for_test()
        .buses
        .push(drawn_bus(1, declaration));
    sheet
}

/// The lane's shared fixture: a design that declares `data[7:0]` and nothing
/// else. Every owned surface is checked against bit 3 of it, read inside
/// instance `x1`.
fn fixture_notations() -> BusNotations {
    BusNotations::of_sheets(std::iter::once(bus_sheet("data[7:0]").document()))
}

#[test]
fn a_declared_bit_renders_in_the_notation_its_bus_was_drawn_in() {
    for (declaration, bit) in [("DATA[7:0]", "DATA[3]"), ("DATA<7:0>", "DATA<3>")] {
        let notations = BusNotations::of_sheets(std::iter::once(bus_sheet(declaration).document()));

        assert_eq!(notations.display("DATA#3"), bit);
        // The bus's name reads as drawn, whatever case the engine answered in.
        assert_eq!(notations.display("data#3"), bit);
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
    sheet
        .document_mut_for_test()
        .buses
        .push(drawn_bus(2, "data[7:0]"));
    let notations = BusNotations::of_sheets(std::iter::once(sheet.document()));

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
    let notations = BusNotations::of_sheets([
        bus_sheet("DATA[7:0]").document(),
        bus_sheet("DATA<7:0>").document(),
    ]);

    assert_eq!(notations.display("DATA#3"), "DATA#3");
}

/// A vector interface pin declares exactly as a drawn bus does, and mints the
/// same deck bits, so it answers for them too.
#[test]
fn a_vector_interface_pin_declares_its_own_bits() {
    let mut sheet = SchematicState::default();
    let id = sheet.add_component(crate::state::ComponentType::Port, Point::new(0, 0));
    sheet
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed port")
        .value = "BUS<3:0>".to_owned();
    let notations = BusNotations::of_sheets(std::iter::once(sheet.document()));

    assert_eq!(notations.display("V(x1.bus#2)"), "V(x1.BUS<2>)");
}

/// One sheet with a net label for each of `names`, a supply rail `vdd`, and
/// an interface port `fb`.
fn labelled_sheet(names: &[&str]) -> SchematicState {
    use crate::state::{NetLabel, NetLabelKind};
    use rspice_design::schematic::net_label::SupplyGlyph;
    let mut sheet = SchematicState::default();
    let id = sheet.add_component(crate::state::ComponentType::Port, Point::new(0, 0));
    sheet
        .document_mut_for_test()
        .components
        .iter_mut()
        .find(|component| component.id == id)
        .expect("the placed port")
        .value = "fb".to_owned();
    let labels = &mut sheet.document_mut_for_test().net_labels;
    for (index, name) in names.iter().enumerate() {
        let id = u64::try_from(index).expect("a test identifier") + 1;
        labels.push(NetLabel::new(
            id,
            Point::new(0, 40 * id as i32),
            (*name).to_owned(),
        ));
    }
    let mut rail = NetLabel::new(99, Point::new(200, 0), "vdd".to_owned());
    rail.kind = NetLabelKind::Supply {
        glyph: SupplyGlyph::Bar,
    };
    labels.push(rail);
    sheet
}

/// The engine folds every node name to upper case; a net reads as the
/// drawing spells it, in its instance's scope, under any accessor that reads
/// a net, and a global rail as declared.
#[test]
fn a_drawn_net_reads_as_drawn() {
    let notations =
        BusNotations::of_sheets(std::iter::once(labelled_sheet(&["b0", "Vin"]).document()));

    assert_eq!(notations.display("B0"), "b0");
    assert_eq!(notations.display("V(B0)"), "V(b0)");
    assert_eq!(notations.display("D(B0)"), "D(b0)");
    assert_eq!(notations.display("phase(v(VIN))"), "phase(v(Vin))");
    assert_eq!(notations.display("xldo.FB"), "xldo.fb");
    assert_eq!(notations.display("V(x1.x2.B0)"), "V(x1.x2.b0)");
    assert_eq!(notations.display("VDD!"), "vdd!");
    // Already as drawn, or not drawn at all: the engine's spelling.
    for untouched in ["b0", "B1", "x1.MP.__gint", "V(B0)-V(VIN)", "B0#branch"] {
        assert_eq!(notations.display(untouched), untouched);
    }
}

/// A current or a power names a device, which no net label spells, even when
/// a net shares its name.
#[test]
fn a_device_named_like_a_net_keeps_its_name() {
    let notations = BusNotations::of_sheets(std::iter::once(labelled_sheet(&["vin"]).document()));

    assert_eq!(notations.display("I(VIN)"), "I(VIN)");
    assert_eq!(notations.display("P(VIN)"), "P(VIN)");
    assert_eq!(notations.display("V(VIN)"), "V(vin)");
}

/// Two drawings that spell one name two ways leave it as the engine spells
/// it: quoting either would misquote the other.
#[test]
fn a_net_spelled_two_ways_keeps_its_engine_name() {
    let notations = BusNotations::of_sheets([
        labelled_sheet(&["Out"]).document(),
        labelled_sheet(&["out"]).document(),
    ]);

    assert_eq!(notations.display("OUT"), "OUT");
    assert_eq!(notations.display("FB"), "fb");
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

/// A design that declares nothing changes nothing, which is what a surface
/// with no design behind it must show.
#[test]
fn an_empty_design_leaves_every_name_alone() {
    let notations = BusNotations::default();

    assert_eq!(notations.display("DATA#3"), "DATA#3");
    assert_eq!(notations.display("V(out)"), "V(out)");
}
