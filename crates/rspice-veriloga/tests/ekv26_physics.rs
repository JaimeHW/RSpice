//! Physics-invariant pins for the EKV 2.6 Verilog-A model (the second
//! industrial compact model after BSIM4.8).
//!
//! No native reference simulator implementation is available locally, so
//! validation relies on the model's defining physical properties: EKV is
//! built source/drain symmetric, its current vanishes identically at
//! Vds = 0, and the drain current is monotonic in the gate drive. The
//! finite-difference check ties the analytic Jacobian (shadow-variable
//! chain rule) to the evaluated currents.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::device::VerilogADevice;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

fn ekv_path() -> Option<PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("ekv26_mod.va");
    path.exists().then_some(path)
}

fn compile_ekv() -> Option<rspice_veriloga::CompiledModel> {
    let path = ekv_path()?;
    Some(
        VerilogACompiler::default()
            .compile_file(&path)
            .expect("EKV 2.6 must compile"),
    )
}

/// Total drain-terminal current with terminals d g s b at the given
/// node voltages (terminal order matches the module: d g s b)
fn drain_current(model: &rspice_veriloga::CompiledModel, vd: f64, vg: f64, vs: f64) -> f64 {
    // d -> node1, g -> node2, s -> node3, b -> ground
    let mut device = VerilogADevice::new("M1", model.clone(), &[1, 2, 3, 0]);
    let voltages = [vd, vg, vs];

    // Sum the device's KCL contributions at the drain row via the stamp
    // RHS identity Ieq = I - G*x: evaluate() returns the program values,
    // and the drain current is the sum of values whose positive node is d.
    let mut rhs: HashMap<usize, f64> = HashMap::new();
    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    device.stamp(
        &voltages,
        |r, c, v| *matrix.entry((r, c)).or_insert(0.0) += v,
        |n, v| *rhs.entry(n).or_insert(0.0) += v,
    );

    // Reconstruct I(drain) = sum_col G[d][col]*x[col] - rhs[d]
    // (row 0 is the drain). The companion form gives
    // rhs[d] = -(I - sum G*x), so I = sum(G*x) - rhs[d].
    let mut current = -rhs.get(&0).copied().unwrap_or(0.0);
    for ((r, c), g) in &matrix {
        if *r == 0 {
            let x = voltages.get(*c).copied().unwrap_or(0.0);
            current += g * x;
        }
    }
    current
}

#[test]
fn ekv26_compiles_with_expected_structure() {
    let Some(model) = compile_ekv() else {
        eprintln!("ekv26_mod.va not present; skipping");
        return;
    };
    assert_eq!(model.num_terminals, 4, "d g s b");
    assert!(
        model.parameters.len() >= 70,
        "EKV 2.6 carries ~78 parameters, got {}",
        model.parameters.len()
    );
    assert!(!model.stamp_programs.is_empty());
}

#[test]
fn ekv26_current_vanishes_at_zero_vds() {
    let Some(model) = compile_ekv() else {
        eprintln!("ekv26_mod.va not present; skipping");
        return;
    };
    for vg in [0.5, 1.0, 1.5, 2.5] {
        let id = drain_current(&model, 0.0, vg, 0.0);
        assert!(
            id.abs() < 1e-15,
            "Id(Vds=0, Vg={vg}) must vanish, got {id:.3e}"
        );
    }
}

#[test]
fn ekv26_source_drain_symmetry() {
    let Some(model) = compile_ekv() else {
        eprintln!("ekv26_mod.va not present; skipping");
        return;
    };
    // EKV is symmetric: swapping source and drain biases reverses the
    // current exactly (default instance has symmetric junctions)
    for (vh, vg) in [(0.6, 1.2), (1.0, 1.8), (0.3, 0.8)] {
        let forward = drain_current(&model, vh, vg, 0.0);
        let reversed = drain_current(&model, 0.0, vg, vh);
        assert!(
            (forward + reversed).abs() <= 1e-9 * forward.abs().max(1e-15),
            "S/D symmetry at vds={vh}, vg={vg}: {forward:.6e} vs {reversed:.6e}"
        );
    }
}

#[test]
fn ekv26_gate_drive_is_monotonic() {
    let Some(model) = compile_ekv() else {
        eprintln!("ekv26_mod.va not present; skipping");
        return;
    };
    let mut last = 0.0;
    for step in 0..=10 {
        let vg = 0.25 * step as f64;
        let id = drain_current(&model, 1.2, vg, 0.0);
        assert!(
            id >= last - 1e-18,
            "Id must be monotonic in Vg: Id({vg}) = {id:.3e} < {last:.3e}"
        );
        assert!(id.is_finite());
        last = id;
    }
    assert!(last > 1e-6, "strong-inversion current is substantial");
}

#[test]
fn ekv26_jacobian_matches_finite_difference() {
    let Some(model) = compile_ekv() else {
        eprintln!("ekv26_mod.va not present; skipping");
        return;
    };

    let (vd, vg) = (0.9, 1.4);
    let delta = 1e-7;

    // Analytic conductance dId/dVd from the stamp matrix
    let mut device = VerilogADevice::new("M1", model.clone(), &[1, 2, 3, 0]);
    let mut matrix: HashMap<(usize, usize), f64> = HashMap::new();
    device.stamp(
        &[vd, vg, 0.0],
        |r, c, v| *matrix.entry((r, c)).or_insert(0.0) += v,
        |_n, _v| {},
    );
    let gds_analytic = matrix.get(&(0, 0)).copied().unwrap_or(0.0);
    let gm_analytic = matrix.get(&(0, 1)).copied().unwrap_or(0.0);

    // Finite differences on the evaluated current
    let i0 = drain_current(&model, vd, vg, 0.0);
    let gds_fd = (drain_current(&model, vd + delta, vg, 0.0) - i0) / delta;
    let gm_fd = (drain_current(&model, vd, vg + delta, 0.0) - i0) / delta;

    let rel = |a: f64, b: f64| ((a - b) / b.abs().max(1e-30)).abs();
    assert!(
        rel(gds_analytic, gds_fd) < 1e-3,
        "gds: analytic {gds_analytic:.6e} vs FD {gds_fd:.6e}"
    );
    assert!(
        rel(gm_analytic, gm_fd) < 1e-3,
        "gm: analytic {gm_analytic:.6e} vs FD {gm_fd:.6e}"
    );
}
