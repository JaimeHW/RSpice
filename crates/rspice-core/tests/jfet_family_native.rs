//! Native classic JFET-family integration tests.
//!
//! JFET2 has its own focused coverage in `jfet2_native.rs`; this file pins the
//! classic level-1 JFET path and related `J`/`Z` family behavior to external
//! simulator oracles.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn xyce_pjfet_switch_deck() -> &'static str {
    "\
2N5144 PJFET Switching Speed Characteristic
Vin 3 0 pulse(12 0 10n 5n 5n 1u 1m)
Vds 4 0 -15
Rout 3 2 50
Rterm 2 0 50
Rload 4 1 500
J1 1 2 0 2N5114
.MODEL 2N5114 PJF
+        VTO = -5.288
+       BETA = 2.1897M
+     LAMBDA = 9.946M
+         RD = 22.042
+         RS = 22.042
+        CGS = 14.6595P
+        CGD = 14.6595P
+         PB = 1.40863
+         IS = 39.24F
+         KF = 0
+         AF = 1
+         FC = 0.5
.TRAN 0.5n 1.07u 1u .1n
.PRINT TRAN V(1) V(2) V(3)
.END
"
}

fn node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("missing {want} node in {:?}", names));
    &voltages[idx]
}

fn interpolate(time: &[f64], values: &[f64], target: f64) -> f64 {
    assert_eq!(time.len(), values.len(), "time and value vectors align");
    if target <= time[0] {
        return values[0];
    }
    for index in 1..time.len() {
        if time[index] >= target {
            let t0 = time[index - 1];
            let t1 = time[index];
            let y0 = values[index - 1];
            let y1 = values[index];
            let frac = if t1 == t0 {
                0.0
            } else {
                (target - t0) / (t1 - t0)
            };
            return y0 + frac * (y1 - y0);
        }
    }
    *values.last().expect("non-empty value vector")
}

#[test]
fn xyce_pjfet_switch_transient_matches_xyce710() {
    let netlist = Netlist::parse(xyce_pjfet_switch_deck()).expect("deck parses");
    let result = engine()
        .run_tran(&netlist, 1.07e-6, 0.1e-9)
        .expect("classic PJFET switch transient runs");

    let v1 = node_series(&result.node_names, &result.voltages, "1");
    let v2 = node_series(&result.node_names, &result.voltages, "2");
    let v3 = node_series(&result.node_names, &result.voltages, "3");

    // Xyce 7.10 regression oracle:
    // `PJFET_SWITCH/pjfet_tran.cir.prn`, selected dynamic and settled rows.
    let reference = [
        (1.00000000e-6, -2.49656419, -6.43755897e-11, 0.0, 3.0e-2),
        (1.01622408e-6, -1.99684488, 1.05032316, 2.93778066, 5.0e-2),
        (1.02000000e-6, -6.22583251, 5.35491369, 12.0, 5.0e-2),
        (1.04006746e-6, -14.9992918, 5.99996130, 12.0, 3.0e-2),
        (1.07000000e-6, -15.0, 6.0, 12.0, 3.0e-2),
    ];

    for (time, v1_ref, v2_ref, v3_ref, tol) in reference {
        let got_v1 = interpolate(&result.time, v1, time);
        let got_v2 = interpolate(&result.time, v2, time);
        let got_v3 = interpolate(&result.time, v3, time);
        assert!(
            (got_v1 - v1_ref).abs() < tol,
            "V(1) at {time:.8e}s: rspice={got_v1:.9e} xyce={v1_ref:.9e}"
        );
        assert!(
            (got_v2 - v2_ref).abs() < tol,
            "V(2) at {time:.8e}s: rspice={got_v2:.9e} xyce={v2_ref:.9e}"
        );
        assert!(
            (got_v3 - v3_ref).abs() < tol,
            "V(3) at {time:.8e}s: rspice={got_v3:.9e} xyce={v3_ref:.9e}"
        );
    }
}
