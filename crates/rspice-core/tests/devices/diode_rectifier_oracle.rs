//! Oracle-pinned regression for the full-wave diode bridge rectifier.
//!
//! This deck is the canonical junction-charge stress case: at every mains
//! conduction knee (vin ≈ 0.2 V, each bridge diode at ~0.1 V) the floating
//! bridge island's common mode is anchored only by the junction depletion
//! capacitances — ngspice stamps the DIOcapCharge companion and walks the
//! knee in a handful of Newton iterations, while an engine without the
//! junction charge leaves the common mode on the 10 MEG bleeders and Newton
//! settles into a dt-independent two-cycle (max_dv ≈ 1.4·N·Vt) that no
//! timestep cut can break. The reference values below are ngspice-46
//! (`ngspice_con.exe -b` on this exact deck).

use rspice_core::{Engine, Netlist};

const RECTIFIER_DECK: &str = "\
* diode_rectifier -- full-wave diode bridge with RC smoothing load
VIN src 0 DC 0 SIN(0 10 50)
RSRC src ac 10
D1 ac plus DBRIDGE
D2 0 plus DBRIDGE
D3 minus ac DBRIDGE
D4 minus 0 DBRIDGE
RLOAD plus minus 1k
CLOAD plus minus 100u
RBLEEDP plus 0 10MEG
RBLEEDN minus 0 10MEG
.model DBRIDGE D(IS=1e-9 RS=0.2 N=1.8 CJO=100p)
.tran 10u 40m
.end
";

fn node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("node {want} not found in {names:?}"));
    &voltages[idx]
}

fn value_at(time: &[f64], series: &[f64], t_want: f64) -> f64 {
    let idx = time
        .iter()
        .position(|&t| t >= t_want)
        .unwrap_or(time.len() - 1);
    series[idx]
}

/// The bridge must ride through every conduction knee and land on the
/// ngspice-46 waveform: smoothing-capacitor ripple phase and amplitude both
/// pin the junction-charge transient model.
#[test]
fn bridge_rectifier_completes_and_matches_ngspice() {
    let netlist = Netlist::parse(RECTIFIER_DECK).expect("deck parses");
    let result = Engine::default()
        .run_tran(&netlist, 40e-3, 10e-6)
        .expect("rectifier transient must converge through the conduction knees");

    let v_plus = node_series(&result.node_names, &result.voltages, "plus");
    let v_minus = node_series(&result.node_names, &result.voltages, "minus");

    // ngspice-46 references on this deck (t, v(plus), v(minus)).
    let references = [
        (5.0e-3, 8.1368, 0.8767),
        (10.0e-3, 2.6518, -4.8390),
        (20.0e-3, 4.9695, -2.7547),
        (30.0e-3, 2.7686, -4.9868),
        (40.0e-3, 4.9917, -2.7677),
    ];
    // The ripple waveform swings ~5.5 V peak-to-peak; 2% of full scale
    // tolerates integrator phasing differences without letting a stuck or
    // restarted trace pass.
    let tolerance = 0.11;
    for (t_ref, plus_ref, minus_ref) in references {
        let plus = value_at(&result.time, v_plus, t_ref);
        let minus = value_at(&result.time, v_minus, t_ref);
        assert!(
            (plus - plus_ref).abs() < tolerance,
            "v(plus) at t={t_ref:.1e}: got {plus:.4}, ngspice {plus_ref:.4}"
        );
        assert!(
            (minus - minus_ref).abs() < tolerance,
            "v(minus) at t={t_ref:.1e}: got {minus:.4}, ngspice {minus_ref:.4}"
        );
    }
}
