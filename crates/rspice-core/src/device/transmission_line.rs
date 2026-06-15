//! Transmission Line Models
//!
//! Implements lossless and lossy transmission lines for high-frequency simulation.
//!
//! # SPICE Syntax
//! ```text
//! T<name> n1+ n1- n2+ n2- Z0=<impedance> TD=<delay>
//! T1 1 0 2 0 Z0=50 TD=1ns
//! ```
//!
//! # Theory
//! A lossless transmission line is characterized by:
//! - Z0: Characteristic impedance (Ω)
//! - TD: Propagation delay (s)
//!
//! The telegrapher's equations relate voltage and current at both ends:
//! ```text
//! V1(t) + Z0*I1(t) = V2(t-TD) + Z0*I2(t-TD)
//! V2(t) + Z0*I2(t) = V1(t-TD) + Z0*I1(t-TD)
//! ```
//!
//! # Implementation
//! Uses delay buffers to store past values and interpolates for accurate delays.
//! The transmission line is modeled as dependent sources with delay.

#![allow(clippy::too_many_arguments)]
use crate::{Value, circuit::NodeId};
use std::cell::Cell;
use std::collections::VecDeque;

/// DC fallback resistance for ideal/lossless lines when no explicit series
/// resistance is available from model parameters.
const TLINE_DC_SHORT_RESISTANCE: Value = 1e-3;
/// Relative tolerance used to truncate negligible distributed-RLC kernel terms.
///
/// Ngspice leaves the LTRA impulse-response chop tolerance at zero unless an
/// internal/user option sets it, so checked-in LTRA regression decks expect the
/// full history convolution rather than a lossy tail truncation.
const DISTRIBUTED_RLC_CHOP_RELTOL: Value = 0.0;
/// Default relative tolerance for ngspice-style LTRA straight-line compaction.
const DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT: Value = 1e-3;
/// Default absolute tolerance for ngspice-style LTRA straight-line compaction.
const DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT: Value = 1e-12;

mod delay;
mod distributed;
mod line;
mod lossy;
mod response;
mod txl;

use delay::DelayBuffer;
use distributed::{
    DistributedRlcKernel, distributed_rlc_coefficients, distributed_rlc_max_safe_step,
};
pub use line::TransmissionLine;
pub use lossy::LossyTransmissionLine;
pub(crate) use response::TlineTransientResponse;
pub(crate) use txl::TxlTransientStamp;

#[cfg(test)]
mod ltra_oracle_replay {
    use super::*;

    /// Replay ngspice's committed LTRA history for o1 of the gate-isolated
    /// ltra2 deck through our distributed-RLC response and compare the branch
    /// RHS inputs point by point.
    ///
    /// The fixtures were extracted with gdb from the vendored ngspice debug
    /// build running the ltra2_2_line deck with the second-stage MOS gates
    /// buffered behind an ideal E source: committed history samples at
    /// ltraacct.c:127 (time, v1, i1, v2, i2 from CKTrhsOld) and per-load
    /// branch RHS stores at ltraload.c:853 (candidate time, LTRAinput1,
    /// LTRAinput2, plus the per-candidate first coefficients). Driving the
    /// response with the oracle's own accepted history isolates the
    /// convolution and delayed-interpolation algebra from solver grid
    /// differences.
    #[test]
    fn replay_oracle_ltra2_o1() {
        let acc_text = include_str!("transmission_line/testdata/ltra2_o1_acc.dat");
        let inp_text = include_str!("transmission_line/testdata/ltra2_o1_inp.dat");

        struct Acc {
            t: Value,
            v1: Value,
            i1: Value,
            v2: Value,
            i2: Value,
        }
        let acc: Vec<Acc> = acc_text
            .lines()
            .map(|line| {
                let f: Vec<&str> = line.split_whitespace().collect();
                Acc {
                    t: f[0].parse().unwrap(),
                    v1: f[2].parse().unwrap(),
                    i1: f[3].parse().unwrap(),
                    v2: f[4].parse().unwrap(),
                    i2: f[5].parse().unwrap(),
                }
            })
            .collect();

        // INP: time timeIndex auxIndex input1 input2 h1dashFirst h2First h3dashFirst
        let mut inp: std::collections::HashMap<u64, (Value, Value, Value, Value, Value)> =
            std::collections::HashMap::new();
        for line in inp_text.lines() {
            let f: Vec<&str> = line.split_whitespace().collect();
            let t: Value = f[0].parse().unwrap();
            let input1: Value = f[3].parse().unwrap();
            let input2: Value = f[4].parse().unwrap();
            let h1f: Value = f[5].parse().unwrap();
            let h2f: Value = f[6].parse().unwrap();
            let h3f: Value = f[7].parse().unwrap();
            inp.insert(t.to_bits(), (input1, input2, h1f, h2f, h3f));
        }

        let r = 12.45_f64;
        let l = 8.972e-9_f64;
        let c = 0.468e-12_f64;
        let len = 16.0_f64;
        let z0 = (l / c).sqrt();
        let td = (l * c).sqrt() * len;
        let admit = 1.0 / z0;

        let mut tl = TransmissionLine::new("o1".to_string(), 1, 0, 2, 0, z0, td);
        tl.set_distributed_rlgc(r, l, 0.0, c, len);
        tl.update_history(acc[0].t, acc[0].v1, acc[0].i1, acc[0].v2, acc[0].i2);

        let mut compared = 0usize;
        let mut worst: (Value, Value) = (0.0, 0.0);
        let mut first_bad: Option<Value> = None;
        for a in &acc[1..] {
            if let Some(&(in1, in2, ng_h1f, _ng_h2f, _ng_h3f)) = inp.get(&a.t.to_bits()) {
                let resp = tl.transient_port_response(a.t);
                let ours1 = resp.i_eq_port1();
                let ours2 = resp.i_eq_port2();
                let scale = in1.abs().max(in2.abs()).max(1e-12);
                let err = (ours1 - in1).abs().max((ours2 - in2).abs()) / scale;
                compared += 1;
                if err > worst.0 {
                    worst = (err, a.t);
                }
                if err > 1.0e-6 && first_bad.is_none() {
                    first_bad = Some(a.t);
                    let our_h1f = resp.self_conductance() / admit - 1.0;
                    println!(
                        "first divergence at t={:.6e}: ours=({:.12e},{:.12e}) oracle=({:.12e},{:.12e}) h1dash_first ours={:.12e} ngspice={:.12e}",
                        a.t, ours1, ours2, in1, in2, our_h1f, ng_h1f
                    );
                }
            }
            tl.update_history(a.t, a.v1, a.i1, a.v2, a.i2);
        }

        assert!(
            compared > 480,
            "expected to replay the full accepted sequence, compared {compared}"
        );
        assert!(
            first_bad.is_none() && worst.0 < 1.0e-6,
            "LTRA input fidelity regressed: worst rel err {:.3e} at t={:.6e}, first>1e-6 at {:?}",
            worst.0,
            worst.1,
            first_bad
        );
    }

    /// Diagnostic probe: run the gate-isolated ltra2 deck on ngspice's exact
    /// accepted grid and diff our committed (v1, i1, v2) trajectory for o1
    /// against the gdb-extracted oracle history, printing where the engines
    /// drift. The LTRA response replay above proves the line algebra is
    /// faithful given identical history, so any drift printed here enters
    /// through the rest of the circuit (driver stage companions).
    #[test]
    #[ignore]
    fn locked_history_drift_ltra2_o1() {
        let acc_text = include_str!("transmission_line/testdata/ltra2_o1_acc.dat");
        let acc: Vec<(Value, Value, Value, Value, Value)> = acc_text
            .lines()
            .map(|line| {
                let f: Vec<&str> = line.split_whitespace().collect();
                (
                    f[0].parse().unwrap(),
                    f[2].parse().unwrap(),
                    f[3].parse().unwrap(),
                    f[4].parse().unwrap(),
                    f[5].parse().unwrap(),
                )
            })
            .collect();

        let deck = r#"MOSdriver -- 2 lossy lines LTRA model -- C load -- isolated gates
m5     0     168    2     0  mn0p9  w = 18.0u l=0.9u
m6     1     168    2     1  mp1p0  w = 36.0u l=1.0u
m1     0     ncopy    4     0  mn0p9  w = 18.0u l=0.9u
m2     1     ncopy    4     1  mp1p0  w = 36.0u l=1.0u
e1   ncopy  0  3 0 1.0
CN2  2   0  0.025398e-12
CN3  3   0  0.007398e-12
CN4  4   0  0.025398e-12
CN5  5   0  0.007398e-12
o1  2 0 3 0  lline
o2  4 0 5 0  lline
vdd    1    0   dc 	5.0
VS 168  0  PULSE (0 5 15.9NS 0.2NS 0.2NS 15.8NS 32NS )
.OPTION NOACCT
.TRAN 0.2N 47N 0 0.1N
.MODEL mn0p9 NMOS VTO=0.8 KP=48U GAMMA=0.30 PHI=0.55
+LAMBDA=0.00 CGSO=0 CGDO=0 CJ=0 CJSW=0 TOX=18000N LD=0.0U
.MODEL mp1p0 PMOS VTO=-0.8 KP=21U GAMMA=0.45 PHI=0.61
+LAMBDA=0.00 CGSO=0 CGDO=0 CJ=0 CJSW=0 TOX=18000N LD=0.0U
.model lline ltra rel=1 r=12.45 g=0 l=8.972e-9 c=0.468e-12
+len=16 steplimit compactrel=1.0e-3 compactabs=1.0e-14
.end
"#;
        let netlist = crate::Netlist::parse(deck).expect("deck parses");
        let mut config = crate::engine::SimulationConfig::default();
        config.integration_method = crate::analysis::IntegrationMethod::Trapezoidal;
        config.temperature = 300.15;
        config.min_timestep = 1e-12;
        config.locked_time_grid = Some(std::sync::Arc::new(
            acc.iter().map(|a| a.0).filter(|&t| t > 0.0).collect(),
        ));
        let engine = crate::engine::Engine::new(config);
        let result = engine
            .run_tran_with_abort(&netlist, 47e-9, 0.1e-9, &crate::abort_signal::NoAbort)
            .expect("locked transient runs");

        let v2_idx = result
            .node_names
            .iter()
            .position(|n| n == "2")
            .expect("node 2");
        let v3_idx = result
            .node_names
            .iter()
            .position(|n| n == "3")
            .expect("node 3");

        let mut printed = 0usize;
        let mut worst_v2: (Value, Value) = (0.0, 0.0);
        let mut worst_v3: (Value, Value) = (0.0, 0.0);
        for (k, &(t, v1_ng, _i1_ng, v2_ng, _i2_ng)) in acc.iter().enumerate() {
            let Some(pos) = result
                .time
                .iter()
                .position(|&rt| (rt - t).abs() <= 1e-22 + 5e-13 * t.abs())
            else {
                continue;
            };
            let v2_ours = result.voltages[v2_idx][pos];
            let v3_ours = result.voltages[v3_idx][pos];
            let d2 = (v2_ours - v1_ng).abs();
            let d3 = (v3_ours - v2_ng).abs();
            if d2 > worst_v2.0 {
                worst_v2 = (d2, t);
            }
            if d3 > worst_v3.0 {
                worst_v3 = (d3, t);
            }
            if (d2 > 2e-3 || d3 > 2e-3) && printed < 25 && k % 5 == 0 {
                println!(
                    "t={:.6e}  v(2): ours={:.9e} ng={:.9e} d={:.3e}   v(3): ours={:.9e} ng={:.9e} d={:.3e}",
                    t, v2_ours, v1_ng, d2, v3_ours, v2_ng, d3
                );
                printed += 1;
            }
        }
        println!(
            "worst |v2 drift|={:.3e} at t={:.6e}; worst |v3 drift|={:.3e} at t={:.6e}",
            worst_v2.0, worst_v2.1, worst_v3.0, worst_v3.1
        );
    }
}
