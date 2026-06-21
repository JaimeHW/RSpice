//! Numerical oracle pins for an optional Verilog-A BSIM4.8 source.
//!
//! Reference currents come from ngspice-46's native C BSIM4
//! (`.model nch nmos level=14 version=4.8.1`, default parameters,
//! W=1u L=0.1u, 27C) for the same bias points. Externally supplied
//! Verilog-A sources can differ in defaults and simulator conditionals, so
//! tolerances are percent-level rather than exact.
//!
//! These pins exercise the entire stack end to end: preprocessing, the
//! full parser, guarded-dataflow lowering with snapshot guards, runtime
//! nf loops, analog-function inlining, shadow-based Jacobians,
//! branch-current unknowns for the collapse/impedance voltage
//! contributions, companion stamping, and Newton convergence.
#![cfg(feature = "veriloga")]

mod common;

use rspice_core::{Engine, Netlist};
use std::path::Path;

fn bsim4_path() -> Option<std::path::PathBuf> {
    common::optional_bsim4_va_path(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn bsim4_optional_source_prefers_configured_external_path() {
    let path = std::env::temp_dir().join(format!(
        "rspice-bsim4-env-path-{}-{}.va",
        std::process::id(),
        "core"
    ));
    std::fs::write(&path, "module bsim4va; endmodule\n").expect("write temporary VA source");

    let actual =
        common::optional_bsim4_va_path_from(Some(path.clone().into_os_string()), Path::new(""));
    let _ = std::fs::remove_file(&path);

    assert_eq!(actual, Some(path));
}

/// Solve the single-transistor bias circuit and return the drain current
fn drain_current(model: &Path, vgs: f64, vds: f64) -> f64 {
    drain_current_at(model, vgs, vds, None)
}

/// Bias-point drain current with an optional circuit temperature (C)
fn drain_current_at(model: &Path, vgs: f64, vds: f64, temp_c: Option<f64>) -> f64 {
    let temp_line = match temp_c {
        Some(t) => format!(".options temp={t}\n"),
        None => String::new(),
    };
    let deck = format!(
        "* bsim4 va bias point\n\
         {temp_line}\
         vg g 0 {vgs}\n\
         vd d 0 {vds}\n\
         XM1 d g 0 0 bsim4va l=1e-7 w=1e-6\n\
         .va \"{}\" bsim4va\n\
         .end\n",
        model.display().to_string().replace('\\', "/")
    );
    let netlist = Netlist::parse(&deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 2e-9, 1e-9)
        .expect("bias point must converge");
    let idx = result
        .branch_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("vd"))
        .expect("vd branch");
    -result.branch_currents[idx]
        .last()
        .copied()
        .expect("samples")
}

fn assert_within(label: &str, got: f64, oracle: f64, rel_tol: f64) {
    let rel = ((got - oracle) / oracle).abs();
    assert!(
        rel <= rel_tol,
        "{label}: got {got:.6e}, ngspice oracle {oracle:.6e} (rel err {:.2}% > {:.1}%)",
        rel * 100.0,
        rel_tol * 100.0
    );
}

#[test]
fn bsim4_idvg_tracks_ngspice_oracle() {
    let Some(model) = bsim4_path() else {
        eprintln!("bsim4.va not present; skipping oracle pins");
        return;
    };

    // ngspice-46: dc vg 0..1.2, vds=1.2
    // Deep subthreshold (pA): the variant gap is largest here
    assert_within(
        "Id(vgs=0.0, vds=1.2)",
        drain_current(&model, 0.0, 1.2),
        1.82638199e-10,
        0.08,
    );
    // Subthreshold decades
    assert_within(
        "Id(vgs=0.3, vds=1.2)",
        drain_current(&model, 0.3, 1.2),
        3.83881131e-6,
        0.06,
    );
    // Moderate inversion
    assert_within(
        "Id(vgs=0.6, vds=1.2)",
        drain_current(&model, 0.6, 1.2),
        2.02222818e-4,
        0.03,
    );
    // Strong inversion
    assert_within(
        "Id(vgs=1.2, vds=1.2)",
        drain_current(&model, 1.2, 1.2),
        7.12754641e-4,
        0.03,
    );
}

#[test]
fn bsim4_temperature_law_tracks_ngspice_oracle() {
    let Some(model) = bsim4_path() else {
        eprintln!("bsim4.va not present; skipping oracle pins");
        return;
    };

    // ngspice-46, .options tnom=25 (matching the VA's DEFAULT_TNOM),
    // vgs=1.0 vds=1.2: mobility-dominated negative tempco
    for (temp_c, oracle) in [
        (-40.0, 6.124726e-4),
        (27.0, 5.377553e-4),
        (125.0, 4.425617e-4),
    ] {
        assert_within(
            &format!("Id(vgs=1.0, vds=1.2, T={temp_c}C)"),
            drain_current_at(&model, 1.0, 1.2, Some(temp_c)),
            oracle,
            0.04,
        );
    }
}

#[test]
fn bsim4_idvd_tracks_ngspice_oracle() {
    let Some(model) = bsim4_path() else {
        eprintln!("bsim4.va not present; skipping oracle pins");
        return;
    };

    // ngspice-46: dc vd 0..1.2, vgs=1.2
    // Linear region
    assert_within(
        "Id(vgs=1.2, vds=0.1)",
        drain_current(&model, 1.2, 0.1),
        2.11832469e-4,
        0.04,
    );
    // Onset of saturation
    assert_within(
        "Id(vgs=1.2, vds=0.6)",
        drain_current(&model, 1.2, 0.6),
        5.71243013e-4,
        0.04,
    );
}
