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
use crate::{NodeId, Value};
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
pub(crate) const DISTRIBUTED_RLC_COMPACT_RELTOL_DEFAULT: Value = 1e-3;
/// Default absolute tolerance for ngspice-style LTRA straight-line compaction.
pub(crate) const DISTRIBUTED_RLC_COMPACT_ABSTOL_DEFAULT: Value = 1e-12;

mod delay;
mod distributed;
mod line;
mod lossy;
mod response;
mod txl;

use delay::DelayBuffer;
use distributed::{
    DistributedRcKernel, DistributedRlcKernel, distributed_rc_coefficients,
    distributed_rlc_coefficients, distributed_rlc_max_safe_step,
};
pub use line::TransmissionLine;
pub(crate) use line::{LtraRgTwoPort, TransmissionLineCheckpoint};
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
}
