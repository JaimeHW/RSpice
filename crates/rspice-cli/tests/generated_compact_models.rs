//! The generated Verilog-A compact models in the shipped binary.
//!
//! A release build sets `generated-veriloga-catalog`, so a card that names
//! one of those models runs; a build without the feature refuses the card
//! with the simulation exit code and names the model's feature, rather than
//! running a different model in its place.

mod common;

use common::test_dir;

use std::process::Command;

/// ngspice's B4SOI card, LEVEL=10 at VERSION 4.6: only the generated
/// BSIM-SOI 4.6.1 model implements it.
const B4SOI_DECK: &str = "* ngspice B4SOI by LEVEL=10\n\
                          VD d 0 1\n\
                          VG g 0 1\n\
                          M1 d g 0 0 ns l=1u w=10u\n\
                          .model ns nmos level=10 version=4.6\n\
                          .op\n\
                          .end\n";

fn run_b4soi_deck(tag: &str) -> std::process::Output {
    let dir = test_dir(tag);
    let deck = dir.join("b4soi.sp");
    std::fs::write(&deck, B4SOI_DECK).expect("write deck");
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run", "--spice-dialect", "ngspice"])
        .arg(&deck)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE")
        .env_remove("RUST_LOG")
        .output()
        .expect("run rspice")
}

#[cfg(feature = "generated-veriloga-catalog")]
#[test]
fn a_b4soi_card_runs_on_the_generated_model() {
    let output = run_b4soi_deck("b4soi_generated");
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[cfg(not(feature = "generated-veriloga-catalog"))]
#[test]
fn a_b4soi_card_is_refused_naming_the_model_feature() {
    let output = run_b4soi_deck("b4soi_refused");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(80), "stderr: {stderr}");
    assert!(
        stderr.contains("veriloga-model-bsimsoi-va"),
        "the refusal names the feature that compiles the model in: {stderr}"
    );
}
