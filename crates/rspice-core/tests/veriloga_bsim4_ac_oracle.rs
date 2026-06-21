//! AC small-signal oracle pins for an optional Verilog-A BSIM4.8 source.
//!
//! Same measurement in both simulators: vg carries AC 1 at the
//! (vgs=1.0, vds=1.2) operating point, .ac at 1 MHz, compare the complex
//! source currents against ngspice-46's native BSIM4 (level=14,
//! version=4.8.1, W=1u L=0.1u, 27C):
//!
//!   i(vg) = 0       - j 1.98375717e-8   (pure capacitive gate current)
//!   i(vd) = -8.52547011e-4 + j 8.91513950e-9   (gm and Cgd)
//!
//! This pins the resistive small-signal Jacobian (gm/gds through the
//! shadow chain rule) AND the reactive charge extraction of the full
//! BSIM4 charge model (capmod) in one shot.
#![cfg(feature = "veriloga")]

mod common;

use rspice_core::{Engine, Netlist};

fn bsim4_path() -> Option<std::path::PathBuf> {
    common::optional_bsim4_va_path(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn bsim4_ac_currents_track_ngspice_oracle() {
    let Some(model) = bsim4_path() else {
        eprintln!("bsim4.va not present; skipping optional AC oracle pins");
        return;
    };

    let deck = format!(
        "* bsim4 va ac bias point\n\
         vg g 0 DC 1.0 AC 1\n\
         vd d 0 DC 1.2\n\
         XM1 d g 0 0 bsim4va l=1e-7 w=1e-6\n\
         .va \"{}\" bsim4va\n\
         .end\n",
        model.display().to_string().replace('\\', "/")
    );

    let netlist = Netlist::parse(&deck).expect("parse");
    let results = Engine::default()
        .run_ac(&netlist, &[1e6])
        .expect("ac analysis");
    let result = &results[0];

    let branch = |name: &str| {
        let idx = result
            .branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{name} branch in {:?}", result.branch_names));
        result.currents[idx]
    };

    let i_vg = branch("vg");
    let i_vd = branch("vd");

    // Oracle values from ngspice-46 native BSIM4. Percent-level tolerances
    // are used on the dominant terms because externally supplied Verilog-A
    // sources can differ slightly in defaults and simulator conditionals.
    let rel = |got: f64, oracle: f64| ((got - oracle) / oracle).abs();

    // Capacitive gate current: Im{i(vg)} = -w * Cgg
    let cgg_oracle = -1.98375717e-8;
    assert!(
        rel(i_vg.im, cgg_oracle) < 0.05,
        "Im i(vg): got {:.6e}, oracle {cgg_oracle:.6e}",
        i_vg.im
    );
    // The real part is negligible against the capacitive term
    assert!(
        i_vg.re.abs() < 1e-2 * i_vg.im.abs(),
        "Re i(vg) must be negligible, got {:.3e} vs Im {:.3e}",
        i_vg.re,
        i_vg.im
    );

    // Transconductance: Re{i(vd)} = -gm
    let gm_oracle = -8.52547011e-4;
    assert!(
        rel(i_vd.re, gm_oracle) < 0.03,
        "Re i(vd): got {:.6e}, oracle {gm_oracle:.6e}",
        i_vd.re
    );

    // Drain-side capacitive coupling (Cgd path)
    let cgd_oracle = 8.91513950e-9;
    assert!(
        rel(i_vd.im, cgd_oracle) < 0.10,
        "Im i(vd): got {:.6e}, oracle {cgd_oracle:.6e}",
        i_vd.im
    );
}
