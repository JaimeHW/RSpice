//! Cross-compiler oracle for PSP 103.6: ngspice-46 runs the same model
//! through OpenVAF/OSDI (psp103.osdi) with the CMC reference card
//! (psp_VA_and_CMC_ref_data, asym_nmos_t); RSpice compiles psp103.va
//! from the IHP SG13G2 PDK directly. Two independent Verilog-A compilers
//! and two engines must produce the same currents.
//!
//! Oracle: dc vg 0.3..1.5 at vds = 0.05 and 1.0, W=1u L=0.1u, 27C,
//! harvested with `pre_osdi psp103.osdi` + wrdata.
#![cfg(feature = "veriloga")]

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;
use std::path::{Path, PathBuf};

fn psp_path() -> Option<PathBuf> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
        .join("psp103")
        .join("psp103.va");
    path.exists().then_some(path)
}

/// Parse the vendored CMC card into instance-parameter assignments
/// (skipping the SPICE-only `level` selector)
fn card_parameters() -> String {
    let card = include_str!("testdata/psp103_nmos_qs.mod");
    let mut params = String::new();
    for line in card.lines() {
        let line = line.trim().trim_start_matches('+').trim();
        if line.is_empty() || line.starts_with('*') || line.starts_with(".model") {
            continue;
        }
        for assignment in line.split_whitespace() {
            let Some((name, value)) = assignment.split_once('=') else {
                continue;
            };
            if name.eq_ignore_ascii_case("level") {
                continue;
            }
            params.push_str(&format!(" {name}={value}"));
        }
    }
    params
}

fn drain_current(model: &Path, vg: f64, vd: f64) -> f64 {
    let deck = format!(
        "* psp oracle point\n\
         vd d 0 dc {vd}\n\
         vg g 0 dc {vg}\n\
         vs s 0 dc 0\n\
         vb b 0 dc 0\n\
         XM1 d g s b psp103va l=0.1u w=1u sa=0 sb=0 absource=1e-12 lssource=1e-6 \
         lgsource=1e-6 abdrain=1e-12 lsdrain=1e-6 lgdrain=1e-6 mult=1.0{}\n\
         .va \"{}\" psp103va\n\
         .end\n",
        card_parameters(),
        model.display().to_string().replace('\\', "/")
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("bias point converges");
    let idx = op
        .branch_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("vd"))
        .expect("vd branch");
    // i(vd) flows into the source's positive node; Id = -i(vd)
    -op.branch_currents[idx]
}

#[test]
fn psp103_idvg_tracks_the_osdi_oracle() {
    let Some(model) = psp_path() else {
        eprintln!("psp103.va not present; skipping oracle");
        return;
    };

    // ngspice-46 + psp103.osdi, CMC asym_nmos_t card: (vg, vds, Id)
    let oracle = [
        (0.3, 0.05, 5.53710408e-6),
        (0.5, 0.05, 3.78525072e-5),
        (0.9, 0.05, 9.38499605e-5),
        (1.3, 0.05, 1.17723730e-4),
        (0.3, 1.0, 2.33185945e-5),
        (0.5, 1.0, 1.46897026e-4),
        (0.9, 1.0, 5.29003046e-4),
        (1.3, 1.0, 8.84503968e-4),
    ];

    for (vg, vd, id_oracle) in oracle {
        let id = drain_current(&model, vg, vd);
        let rel = ((id - id_oracle) / id_oracle).abs();
        assert!(
            rel < 5e-3,
            "Id(vg={vg}, vd={vd}): RSpice {id:.6e} vs OSDI {id_oracle:.6e} \
             (rel err {:.3}%)",
            rel * 100.0
        );
    }
}
