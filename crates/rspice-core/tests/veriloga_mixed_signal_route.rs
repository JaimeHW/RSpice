//! What a deck gets when it instantiates a mixed Verilog-AMS module.
//!
//! `MixedSignalHost` executes a module that has both analog equations and
//! digital processes, but nothing elaborates one into `CircuitData`: the
//! `.VERILOGA` route builds a `VerilogADevice` from a compiled analog model and
//! has no digital half to give anyone. The boundary between "runs" and "does
//! not exist yet" is therefore the deck route, and this file pins which side of
//! it a mixed module lands on.
//!
//! The property under test is that it lands on the *refusing* side. A route
//! that compiled the analog equations, dropped the processes, and produced a
//! waveform would be worse than no route at all: the answer would be a
//! plausible curve for a circuit the deck did not describe, with nothing in the
//! result saying so.
//!
//! # Where the door is
//!
//! `engine::builder`'s `.va` cache compiles with the default
//! [`CompilerOptions`](rspice_veriloga::CompilerOptions), whose `enable_ams` is
//! off, and code generation refuses the first digital declaration it meets. So
//! the refusal happens before a node is allocated or a device is built —
//! upstream of everything a half-built route could get wrong. When the
//! elaboration that gives a mixed module its `CircuitData` instance lands,
//! *this* is the test that has to change, and it should change into a run.
#![cfg(feature = "veriloga")]

use rspice_core::{Engine, Netlist};
use std::io::Write;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

fn write_model(name: &str, source: &str) -> PathBuf {
    let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let mut path = std::env::temp_dir();
    path.push(format!(
        "rspice_mixed_route_{name}_{}_{sequence}.va",
        std::process::id()
    ));
    let mut file = std::fs::File::create(&path).expect("create model file");
    file.write_all(source.as_bytes()).expect("write model");
    path
}

fn deck_path(path: &std::path::Path) -> String {
    path.display().to_string().replace('\\', "/")
}

/// Both halves: a continuous equation and a process that toggles a register.
/// Executing only the first is a different circuit.
const MIXED: &str = r#"
`include "disciplines.vams"
module mixed_deck_route(p, n, clk, q);
    inout p, n;
    electrical p, n;
    input clk;
    output q;
    wire clk;
    reg q;
    initial q = 1'b0;
    always @(posedge clk) q <= ~q;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn a_mixed_module_instantiated_from_a_deck_is_refused_rather_than_half_run() {
    let path = write_model("mixed", MIXED);
    let deck = format!(
        "* mixed module deck route\n\
         v1 in 0 dc 1\n\
         x1 in 0 clk q mixed_deck_route\n\
         .va \"{}\" mixed_deck_route\n\
         .tran 1n 5n\n\
         .end\n",
        deck_path(&path)
    );

    let outcome = Netlist::parse(&deck)
        .map_err(|error| error.to_string())
        .and_then(|netlist| {
            Engine::new(Default::default())
                .run_tran(&netlist, 5.0e-9, 1.0e-9)
                .map(|_| ())
                .map_err(|error| error.to_string())
        });

    let error = outcome.expect_err(
        "a mixed module has no deck route, so instantiating one must fail rather than run \
         the analog half alone",
    );
    let _ = std::fs::remove_file(&path);

    // The refusal has to be legible enough that a deck author knows the module
    // was understood and declined, not that the file was unreadable.
    // The refusal has to be legible enough that a deck author knows the module
    // was understood and declined. It arrives from code generation, before any
    // node is allocated or any device is built, which is what makes this a
    // closed door rather than a half-built route: the builder compiles a
    // `.va` module with `enable_ams` off, so the first digital declaration is
    // refused by name.
    let lowered = error.to_lowercase();
    assert!(
        lowered.contains("digital"),
        "the refusal must say it is the digital half it cannot run: {error}"
    );
    assert!(
        lowered.contains("mixed_deck_route"),
        "the refusal must name the module: {error}"
    );
    assert!(
        !lowered.contains("has not been defined"),
        "an unknown-subcircuit message would mean the module was never even read, which is a \
         different failure and would hide this one: {error}"
    );
}

const ANALOG_ONLY: &str = r#"
`include "disciplines.vams"
module analog_deck_route(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn the_same_deck_shape_runs_when_the_module_is_analog_only() {
    // The control. Whatever the mixed module's refusal says, it has to be
    // about the module being mixed and not about the route being broken.
    let path = write_model("analog", ANALOG_ONLY);
    let deck = format!(
        "* analog module deck route\n\
         v1 in 0 dc 1\n\
         x1 in 0 analog_deck_route\n\
         .va \"{}\" analog_deck_route\n\
         .tran 1n 5n\n\
         .end\n",
        deck_path(&path)
    );
    let netlist = Netlist::parse(&deck).expect("the deck parses");
    let result = Engine::new(Default::default())
        .run_tran(&netlist, 5.0e-9, 1.0e-9)
        .expect("an analog-only module instantiated this way runs");
    let _ = std::fs::remove_file(&path);
    assert!(!result.time.is_empty());
}
