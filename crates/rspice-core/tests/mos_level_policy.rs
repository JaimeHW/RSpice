//! Build-time policy for MOS model levels without a native implementation.
//!
//! BSIM-class cards (LEVEL=8/14/49/53/54, ...) must be rejected with a
//! remediation message instead of silently running the simplified
//! short-channel approximation, which honors only a handful of parameters
//! and produces plausible-looking but wrong currents. The
//! `.options allow_simplified_mos` opt-in downgrades the rejection to a
//! warning; LEVEL=3 stays runnable (with a warning) for compatibility with
//! the vendored ngspice MOS3 decks.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn op_deck(model_line: &str, options_line: &str) -> String {
    format!(
        "* mos level policy\n\
         vdd d 0 dc 1.8\n\
         vg g 0 dc 1.0\n\
         m1 d g 0 0 nmod w=1u l=0.1u\n\
         {model_line}\n\
         {options_line}\n\
         .op\n\
         .end\n"
    )
}

fn run(deck: &str) -> Result<(), String> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_dc_op_with_report(&netlist)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

#[test]
fn bsim4_level_without_native_model_is_rejected() {
    let deck = op_deck(".model nmod NMOS (LEVEL=54 VTH0=0.5)", "");
    let message = run(&deck).expect_err("LEVEL=54 must not silently run the approximation");
    assert!(
        message.contains("BSIM4"),
        "error names the model family: {message}"
    );
    assert!(
        message.contains("allow_simplified_mos"),
        "error names the opt-in: {message}"
    );
    assert!(
        message.contains("veriloga"),
        "error points at the Verilog-A BSIM4 path: {message}"
    );
}

#[test]
fn bsim3_level_without_native_model_is_rejected() {
    let deck = op_deck(".model nmod NMOS (LEVEL=49 VTH0=0.5)", "");
    let message = run(&deck).expect_err("LEVEL=49 must not silently run the approximation");
    assert!(
        message.contains("BSIM3v3"),
        "error names the model family: {message}"
    );
}

#[test]
fn simplified_mos_opt_in_accepts_the_deck() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=54 VTH0=0.5)",
        ".options allow_simplified_mos=1",
    );
    run(&deck).expect("explicit opt-in runs the simplified approximation");
}

#[test]
fn simplified_mos_opt_in_zero_still_rejects() {
    let deck = op_deck(
        ".model nmod NMOS (LEVEL=54 VTH0=0.5)",
        ".options allow_simplified_mos=0",
    );
    run(&deck).expect_err("allow_simplified_mos=0 keeps the rejection");
}

#[test]
fn level3_runs_with_warning_not_error() {
    let deck = op_deck(".model nmod NMOS (LEVEL=3 VTO=0.6 KP=100u)", "");
    run(&deck).expect("LEVEL=3 stays runnable for ngspice MOS3 deck compatibility");
}

#[test]
fn native_levels_unaffected() {
    for model_line in [
        ".model nmod NMOS (LEVEL=1 VTO=0.6 KP=100u)",
        ".model nmod NMOS (LEVEL=2 VTO=0.6 KP=100u)",
        ".model nmod NMOS (LEVEL=6 VTO=0.6 KC=100u)",
    ] {
        let deck = op_deck(model_line, "");
        run(&deck).unwrap_or_else(|err| panic!("{model_line} must build: {err}"));
    }
}
