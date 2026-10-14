//! A Verilog-A source directive resolves against the file that names it.
//!
//! `.include "x.cir"` has always been resolved relative to the including
//! file's directory, with the top-level deck directory, the execution
//! directory, the configured search paths and the conventional library
//! directories behind it. `.va "d.va"` was handed to the builder exactly as
//! written and opened relative to the process working directory, so the two
//! directives in one deck disagreed about what a relative path means: a deck
//! in a subdirectory ran from its own directory and failed from anywhere else.
//!
//! These cases pin the agreement. Every fixture lives under the system temp
//! directory, so nothing here can pass because a model happens to sit in the
//! working directory.

#![cfg(feature = "veriloga")]

#[path = "common/veriloga_source_fixture.rs"]
mod fixture;

use fixture::{
    DIVIDER_VALUES, Fixture, HALF_DIVIDER_OHMS, divider_deck, divider_out, resistor_module,
};
use rspice_core::{Engine, Netlist};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};

/// Build a deck from its own path and read `out`.
fn out_voltage(deck: &Path) -> f64 {
    let netlist = Netlist::parse_file(deck)
        .unwrap_or_else(|error| panic!("{} must parse: {error}", deck.display()));
    let result = Engine::default()
        .run_dc_op(&netlist)
        .unwrap_or_else(|error| panic!("{} must solve: {error}", deck.display()));
    result
        .try_voltage_named("out")
        .unwrap_or_else(|| panic!("{} has no node 'out'", deck.display()))
}

/// A deck directory holding `deck.cir`, `vals.cir` and one `d.va` of `ohms`.
fn deck_beside_its_model(fixture: &Fixture, directory: &str, ohms: f64) -> PathBuf {
    fixture.write(&format!("{directory}/d.va"), &resistor_module(ohms));
    fixture.write(&format!("{directory}/vals.cir"), DIVIDER_VALUES);
    fixture.write(
        &format!("{directory}/deck.cir"),
        &divider_deck(".va \"d.va\" dres"),
    )
}

/// The working directory is process-wide, so the one case that moves it holds
/// this lock while it does.
static WORKING_DIRECTORY: Mutex<()> = Mutex::new(());

/// Runs the deck with the process working directory set elsewhere, and puts it
/// back whatever happens.
fn out_voltage_from(working_directory: &Path, deck: &Path) -> f64 {
    struct Restore(PathBuf);
    impl Drop for Restore {
        fn drop(&mut self) {
            let _ = std::env::set_current_dir(&self.0);
        }
    }

    let _held = WORKING_DIRECTORY
        .lock()
        .unwrap_or_else(PoisonError::into_inner);
    let restore = Restore(std::env::current_dir().expect("a readable working directory"));
    std::env::set_current_dir(working_directory).expect("an enterable working directory");
    let out = out_voltage(deck);
    drop(restore);
    out
}

/// The control: `.include` already resolves beside the deck, from anywhere.
#[test]
fn an_include_beside_the_deck_resolves_from_the_decks_own_directory() {
    let fixture = Fixture::new("include_control");
    fixture.write("sub/vals.cir", DIVIDER_VALUES);
    let deck = fixture.write(
        "sub/deck.cir",
        "* only the include half of the divider\n\
         V1 in 0 1\n\
         .include \"vals.cir\"\n\
         R2 out 0 1k\n\
         .end\n",
    );

    let out = out_voltage(&deck);
    assert!(
        (out - 0.5).abs() < 1e-9,
        "the included resistor must form the divider, got {out}"
    );
}

/// The directive under repair, in the same position as the control.
#[test]
fn a_verilog_a_source_beside_the_deck_resolves_like_an_include() {
    let fixture = Fixture::new("va_beside_deck");
    let deck = deck_beside_its_model(&fixture, "sub", HALF_DIVIDER_OHMS);

    let out = out_voltage(&deck);
    let expected = divider_out(HALF_DIVIDER_OHMS);
    assert!(
        (out - expected).abs() < 1e-9,
        "the Verilog-A resistor beside the deck must form the divider, got {out}"
    );
}

#[test]
fn the_same_deck_answers_the_same_from_two_working_directories() {
    let fixture = Fixture::new("two_working_dirs");
    let deck = deck_beside_its_model(&fixture, "sub", HALF_DIVIDER_OHMS);
    let expected = divider_out(HALF_DIVIDER_OHMS);

    let from_crate = out_voltage_from(Path::new(env!("CARGO_MANIFEST_DIR")), &deck);
    let from_temp = out_voltage_from(&std::env::temp_dir(), &deck);

    assert_eq!(
        from_crate.to_bits(),
        from_temp.to_bits(),
        "the deck answered {from_crate} from the crate directory and {from_temp} from the temp \
         directory"
    );
    assert!(
        (from_crate - expected).abs() < 1e-9,
        "both runs must solve the divider, got {from_crate}"
    );
}

/// The including file's directory wins, not just the top-level deck's: the
/// root of this fixture holds no `d.va` at all.
#[test]
fn a_nested_include_names_its_verilog_a_source_beside_itself() {
    let fixture = Fixture::new("nested_include");
    fixture.write("sub/d.va", &resistor_module(HALF_DIVIDER_OHMS));
    fixture.write(
        "sub/inner.cir",
        "* the included file names a model beside itself\n\
         R1 in out 1k\n\
         X1 out 0 dres\n\
         .va \"d.va\" dres\n",
    );
    let deck = fixture.write(
        "top.cir",
        "* the root deck has no model file of its own\n\
         V1 in 0 1\n\
         .include \"sub/inner.cir\"\n\
         .end\n",
    );

    let out = out_voltage(&deck);
    let expected = divider_out(HALF_DIVIDER_OHMS);
    assert!(
        (out - expected).abs() < 1e-9,
        "the nested include's own directory must supply d.va, got {out}"
    );
}

/// Two decks, two directories, one relative spelling, two different files.
#[test]
fn two_decks_that_each_name_their_own_source_file_get_their_own_model() {
    let fixture = Fixture::new("no_key_collision");
    let first = deck_beside_its_model(&fixture, "first", HALF_DIVIDER_OHMS);
    let second = deck_beside_its_model(&fixture, "second", 3000.0);

    let first_out = out_voltage(&first);
    let second_out = out_voltage(&second);

    assert!(
        (first_out - divider_out(HALF_DIVIDER_OHMS)).abs() < 1e-9,
        "the first deck must read its own d.va, got {first_out}"
    );
    assert!(
        (second_out - divider_out(3000.0)).abs() < 1e-9,
        "the second deck must read its own d.va, not the first's, got {second_out}"
    );
}

/// One file, two decks, two spellings: the same compiled model.
#[test]
fn one_shared_source_named_from_two_decks_answers_the_same() {
    let fixture = Fixture::new("shared_source");
    fixture.write("shared/d.va", &resistor_module(HALF_DIVIDER_OHMS));
    fixture.write("one/vals.cir", DIVIDER_VALUES);
    let one = fixture.write("one/deck.cir", &divider_deck(".va \"../shared/d.va\" dres"));
    fixture.write("two/deeper/vals.cir", DIVIDER_VALUES);
    let two = fixture.write(
        "two/deeper/deck.cir",
        &divider_deck(".va \"../../shared/d.va\" dres"),
    );

    let one_out = out_voltage(&one);
    let two_out = out_voltage(&two);
    assert_eq!(
        one_out.to_bits(),
        two_out.to_bits(),
        "one source read through two spellings answered {one_out} and {two_out}"
    );
    assert!(
        (one_out - divider_out(HALF_DIVIDER_OHMS)).abs() < 1e-9,
        "the shared source must form the divider, got {one_out}"
    );
}

#[test]
fn a_missing_verilog_a_source_names_the_path_it_tried_and_the_deck_line() {
    let fixture = Fixture::new("missing_source");
    fixture.write("sub/vals.cir", DIVIDER_VALUES);
    let deck = fixture.write("sub/deck.cir", &divider_deck(".va \"absent.va\" dres"));

    let error = Netlist::parse_file(&deck).expect_err("a missing Verilog-A source must refuse");
    let text = error.to_string();
    let tried = fixture.root().join("sub").join("absent.va");
    assert!(
        text.contains(&tried.display().to_string()),
        "the refusal must name the path it tried ({}): {text}",
        tried.display()
    );
    assert!(
        text.contains("line 5"),
        "the refusal must name the deck line the directive sits on: {text}"
    );
}

// The HDL sources of one deck are one compilation unit, the library `work`,
// read in deck order (IEEE 1364-2005 clause 19): a module of one file may
// instantiate a module of another, macros and `timescale` carry from one file
// into the next (sections 19.3 and 19.8), and a module name is declared once.

/// The deck's `out` voltage over a short transient, one sample per entry.
fn transient_out(engine: &Engine, deck: &Path, tstop: f64, step: f64) -> (Vec<f64>, Vec<f64>) {
    let netlist = Netlist::parse_file(deck)
        .unwrap_or_else(|error| panic!("{} must parse: {error}", deck.display()));
    let result = engine
        .run_tran(&netlist, tstop, step)
        .unwrap_or_else(|error| panic!("{} must run: {error}", deck.display()));
    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("a node 'out'");
    (result.time.clone(), result.voltages[out].clone())
}

/// The level the built-in D/A bridge (20 ohms from `vcc`) holds on a 1 kohm
/// load when the module drives 1.
const DRIVEN_HIGH: f64 = 3.3 * 1000.0 / 1020.0;

#[test]
fn a_module_of_one_file_instantiates_a_module_of_another_in_either_order() {
    let fixture = Fixture::new("unit_cross_file");
    fixture.write(
        "top.v",
        "module unit_top(q); output q; wire q; unit_leaf u1(.y(q)); endmodule\n",
    );
    fixture.write(
        "leaf.v",
        "module unit_leaf(y); output y; reg y; initial y = 1'b1; endmodule\n",
    );
    for (first, second) in [("top.v", "leaf.v"), ("leaf.v", "top.v")] {
        let deck = fixture.write(
            &format!("deck_{first}.cir"),
            &format!(
                "* a module instantiating a module of another file\n.param vcc=3.3\n\
                 X1 out unit_top\nR1 out 0 1k\n.hdl \"{first}\"\n.hdl \"{second}\"\n.end\n"
            ),
        );
        let (_, out) = transient_out(&Engine::default(), &deck, 2e-9, 1e-10);
        assert!(!out.is_empty());
        for voltage in out {
            assert!(
                (voltage - DRIVEN_HIGH).abs() < 1e-9,
                "unit_top's child from the other file must drive out high ({first} first): \
                 {voltage}"
            );
        }
    }
}

#[test]
fn a_macro_and_a_timescale_carry_into_the_next_file_with_a_warning() {
    let fixture = Fixture::new("unit_directives");
    // A file of directives only: it declares no module and has no model.
    fixture.write("defs.v", "`timescale 10ns/1ns\n`define UNIT_LEVEL 1'b1\n");
    let edge = fixture.write(
        "edge.v",
        "module unit_edge(q); output q; reg q;\n\
         initial begin q = 1'b0; #10 q = `UNIT_LEVEL; end\n\
         endmodule\n",
    );
    let deck = fixture.write(
        "deck.cir",
        "* directives carried from one file into the next\n.param vcc=3.3\n\
         X1 out unit_edge\nR1 out 0 1k\n.hdl \"defs.v\"\n.hdl \"edge.v\"\n.end\n",
    );
    let engine = Engine::default();
    let (time, out) = transient_out(&engine, &deck, 200e-9, 1e-9);
    // `#10` is 100 ns in the inherited 10ns unit; the default 1ns would put
    // the edge at 10 ns.
    for (t, voltage) in time.iter().zip(&out) {
        if *t < 99e-9 {
            assert!(
                voltage.abs() < 1e-9,
                "out must be low before 100 ns: {voltage} at {t}"
            );
        } else if *t > 101e-9 {
            assert!(
                (voltage - DRIVEN_HIGH).abs() < 1e-9,
                "the macro's level must reach out after 100 ns: {voltage} at {t}"
            );
        }
    }
    let warnings: Vec<_> = engine
        .compiler_diagnostics()
        .into_iter()
        .filter(|diagnostic| diagnostic.code == "VA-UNIT-TIMESCALE-INHERITED")
        .collect();
    assert_eq!(
        warnings.len(),
        1,
        "one inheriting file, one warning: {warnings:?}"
    );
    let warning = &warnings[0];
    assert!(
        warning.message.contains("unit_edge") && warning.message.contains("10ns/1ns"),
        "the warning names the module and the inherited timescale: {}",
        warning.message
    );
    assert_eq!(
        warning
            .path
            .as_deref()
            .map(Path::new)
            .and_then(Path::file_name),
        edge.file_name(),
        "the warning points into the inheriting file: {warning}"
    );
}

#[test]
fn a_module_two_files_declare_is_refused_naming_both() {
    let fixture = Fixture::new("unit_duplicate");
    for name in ["first.v", "second.v"] {
        fixture.write(
            name,
            "module unit_twice(q); output q; reg q; initial q = 1'b1; endmodule\n",
        );
    }
    let deck = fixture.write(
        "deck.cir",
        "* one module name in two files\n.param vcc=3.3\nX1 out unit_twice\nR1 out 0 1k\n\
         .hdl \"first.v\"\n.hdl \"second.v\"\n.end\n",
    );
    let netlist = Netlist::parse_file(&deck).expect("the deck parses");
    let error = Engine::default()
        .run_tran(&netlist, 2e-9, 1e-10)
        .expect_err("a module declared twice must be refused");
    let rspice_core::SimulationError::Elaboration(refusal) = &error else {
        panic!("a duplicate module is a typed elaboration refusal: {error}");
    };
    assert_eq!(
        refusal.kind,
        rspice_core::ElaborationErrorKind::ModuleNotSelected,
        "{error}"
    );
    assert!(
        refusal.detail.contains("'unit_twice'")
            && refusal.detail.contains("first.v")
            && refusal.detail.contains("second.v"),
        "the refusal names the module and both files: {error}"
    );
    assert!(
        refusal
            .span
            .as_ref()
            .is_some_and(|span| span.to_string().contains("second.v")),
        "the refusal points at the second declaration's file: {error}"
    );
}
