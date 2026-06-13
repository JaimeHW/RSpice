//! Engine-facing BSIM4 v4.8 device (MOS levels 14/54).
//!
//! Wraps the model math of this module ([`Bsim4v8`]) in a
//! [`NonlinearDevice`] the engine can iterate and stamp, transcribing the
//! matrix/RHS load of ngspice-46 `b4ld.c:4750-5350` for the canonical mode
//! set (`rdsMod = rgateMod = rbodyMod = trnqsMod = 0`, `igcMod = igbMod =
//! 0`):
//!
//! - the per-iteration limiting sequence (`DEVfetlim`/`DEVlimvds`/
//!   `DEVpnjlim`, b4ld.c:605-689) anchored at the previous accepted iterate
//!   and `von`, with the `Check` flag participating in nonconvergence
//!   (b4ld.c:4070-4072);
//! - the DC conductance/current stamp with the mode swap (`FwdSum`/
//!   `RevSum`, the `gbdp*`/`gbsp*` substrate-current groups and the
//!   `gbbdp`/`gbbsp` bulk rows), the GIDL/GISL injections of the dedicated
//!   stamp blocks (b4ld.c:5330-5348), and the `type`-folded equivalent
//!   currents `ceqdrn`/`ceqbd`/`ceqbs`/`ceqjd`/`ceqjs`;
//! - the mode-dependent charge-companion assembly (b4ld.c:4186-4566 for
//!   `trnqsMod = 0`): intrinsic `c***` plus the smoothed `cgdo`/`cgso`
//!   overlaps and the bias-independent `cgbo`, plus `capbd`/`capbs`, scaled
//!   by the caller's integration gain (`ag0` in transient, `omega` on the
//!   imaginary axis in AC — b4acld.c stamps the identical positions).
//!
//! The series drain/source conductances (`RSH * NRD/NRS`) are *not* stamped
//! here: the builder lowers them to ordinary linear resistors at internal
//! prime nodes, so `node_drain`/`node_source` are already the primes and
//! the `Dd`/`DPd`/`Ss`/`SPs` rows of the C vanish. `CKTgmin` enters through
//! [`Bsim4v8Device::set_eval_gmin`] and is consumed inside the module's
//! junction-diode equations — no second per-device gmin shunt may be
//! applied on top (see `eval.rs` scope notes).

use super::eval::{Bsim4v8Bias, Bsim4v8Charge, Bsim4v8Op};
use super::Bsim4v8;
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};

/// Mode-assembled charge-companion conductance matrix: the `gc**` of
/// b4ld.c:4216-4260 (mode > 0) / 4408-4456 (mode < 0) *before* the `ag0`
/// scaling, i.e. pure capacitances. AC multiplies by `omega` instead
/// (b4acld.c forms the same expressions as `xc***`). The bulk columns
/// (`gcgbb`/`gcdbb`/`gcsbb`/`gcbbb`) are the negative row sums of the C and
/// are reconstructed at stamp time.
#[derive(Debug, Clone, Copy, Default)]
pub struct Bsim4v8ChargeMatrix {
    pub gcggb: Value,
    pub gcgdb: Value,
    pub gcgsb: Value,
    pub gcdgb: Value,
    pub gcddb: Value,
    pub gcdsb: Value,
    pub gcsgb: Value,
    pub gcsdb: Value,
    pub gcssb: Value,
    pub gcbgb: Value,
    pub gcbdb: Value,
    pub gcbsb: Value,
}

/// One BSIM4 v4.8 instance wired into the engine.
#[derive(Debug, Clone)]
pub struct Bsim4v8Device {
    pub name: String,
    /// Drain prime (the internal node when an RSH*NRD resistor exists).
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    /// Source prime (the internal node when an RSH*NRS resistor exists).
    pub node_source: NodeId,
    pub node_bulk: NodeId,
    /// Parallel multiplier `M` (ngspice `here->BSIM4m`, applied at stamp
    /// time; `NF` is folded into the eval by the module).
    pub multiplier: Value,

    /// Model math: card + temperature/size knots + instance tail.
    pub core: Bsim4v8,

    /// Engine `CKTgmin` consumed by the junction diodes inside `eval`.
    gmin: Value,

    // Newton iteration state (ngspice CKTstate0 vds/vgs/vbs + BSIM4von).
    op: Bsim4v8Op,
    bias: Bsim4v8Bias,
    converged_ref: Bsim4v8Bias,
    von_prev: Value,
    has_history: bool,
    /// The previous iterate exists, so the b4ld.c limiting sequence applies
    /// (the first iterate of an analysis phase passes through unlimited).
    limit_anchor_valid: std::cell::Cell<bool>,
    /// `DEVpnjlim` flagged the body junction on the last update (ngspice
    /// bumps `CKTnoncon`, b4ld.c:4070-4072; here it vetoes device
    /// convergence for the iterate).
    last_limited: std::cell::Cell<bool>,
}

impl Bsim4v8Device {
    pub fn new(
        name: String,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_bulk: NodeId,
        multiplier: Value,
        core: Bsim4v8,
    ) -> Self {
        Self {
            name,
            node_drain,
            node_gate,
            node_source,
            node_bulk,
            multiplier: if multiplier > 0.0 { multiplier } else { 1.0 },
            core,
            gmin: 1e-12,
            op: Bsim4v8Op::default(),
            bias: Bsim4v8Bias {
                vds: 0.0,
                vgs: 0.0,
                vbs: 0.0,
            },
            converged_ref: Bsim4v8Bias {
                vds: 0.0,
                vgs: 0.0,
                vbs: 0.0,
            },
            von_prev: 0.0,
            has_history: false,
            limit_anchor_valid: std::cell::Cell::new(false),
            last_limited: std::cell::Cell::new(false),
        }
    }

    /// Set the engine's junction GMIN (ngspice `CKTgmin`). The module's
    /// diode equations include it exactly as b4ld.c does (`gbs = ... +
    /// gmin`), so this is the only gmin path for the device.
    pub fn set_eval_gmin(&mut self, gmin: Value) {
        self.gmin = if gmin.is_finite() && gmin > 0.0 {
            gmin
        } else {
            0.0
        };
    }

    /// Device-polarity branch voltages from the solution vector
    /// (b4ld.c:380-391: `type` folded, source-referenced at the primes).
    /// The solution is 0-indexed (node 1 -> `v[0]`); ground reads as 0.
    fn raw_branch_voltages(&self, v: &[Value]) -> Bsim4v8Bias {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n - 1] };
        let vd = node(self.node_drain);
        let vg = node(self.node_gate);
        let vs = node(self.node_source);
        let vb = node(self.node_bulk);
        let mt = self.core.mtype;
        Bsim4v8Bias {
            vds: mt * (vd - vs),
            vgs: mt * (vg - vs),
            vbs: mt * (vb - vs),
        }
    }

    /// The b4ld.c limiting sequence against the previous accepted iterate;
    /// the first iterate of a phase passes through (ngspice seeds CKTstate0
    /// before the first NIiter, the engine seeds from the raw solution).
    fn limited_branch_voltages(&self, v: &[Value]) -> (Bsim4v8Bias, bool) {
        let raw = self.raw_branch_voltages(v);
        if !self.limit_anchor_valid.get() {
            return (raw, false);
        }
        self.core.limit_voltages(raw, self.bias, self.von_prev)
    }

    /// DC operating point at an already-limited bias. The only error paths
    /// of the module's `eval` are charge-model rejections, which the
    /// builder rules out at construction; the DC-only call cannot fail.
    fn eval_dc(&self, bias: Bsim4v8Bias) -> Bsim4v8Op {
        self.core
            .eval(bias, self.gmin, false)
            .expect("BSIM4 DC eval: no charge model requested")
    }

    /// Charge state at the limited bias (the same bias the conductance
    /// stamp uses — mixing raw voltages with limited-bias conductances
    /// would inject companion currents that do not cancel at convergence).
    /// Returns the charge together with the channel mode of that
    /// evaluation, which selects the companion-matrix assembly.
    pub fn charge_at(&self, v: &[Value]) -> (Bsim4v8Charge, i32) {
        let (bias, _) = self.limited_branch_voltages(v);
        let op = self
            .core
            .eval(bias, self.gmin, true)
            .expect("BSIM4 charge eval: charge model validated at build");
        let charge = op
            .charge
            .expect("compute_charges=true yields a charge state");
        (charge, op.mode)
    }

    /// Operating-point snapshot for the OP report: `(id, vgs, vds, vbs, vth,
    /// vdsat, gm, gds, gmbs, region)` — currents/derivatives in device
    /// polarity exactly as ngspice's `@m[id]`/`@m[gm]` report them.
    pub fn op_values(
        &self,
    ) -> (
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        Value,
        &'static str,
    ) {
        let op = &self.op;
        let bias = self.bias;
        // Mode-frame gate overdrive: vgs in normal mode, vgd in inverse.
        let (vgs_mode, vds_mode) = if op.mode >= 0 {
            (bias.vgs, bias.vds)
        } else {
            (bias.vgs - bias.vds, -bias.vds)
        };
        let region = if vgs_mode < op.von {
            "subthreshold"
        } else if vds_mode > op.vdsat {
            "saturation"
        } else {
            "linear"
        };
        (
            op.cd * self.multiplier,
            bias.vgs,
            bias.vds,
            bias.vbs,
            op.von,
            op.vdsat,
            op.gm * self.multiplier,
            op.gds * self.multiplier,
            op.gmbs * self.multiplier,
            region,
        )
    }

    /// Assemble the mode-dependent charge-companion capacitance matrix
    /// (b4ld.c:4216-4260 / 4408-4456 with `ag0 = 1`, `rgateMod = 0` so the
    /// `gc*gmb` legs are zero and `rbodyMod = 0` so `capbd`/`capbs` fold
    /// into the prime rows).
    pub fn charge_matrix(charge: &Bsim4v8Charge, mode: i32) -> Bsim4v8ChargeMatrix {
        let c = charge;
        if mode > 0 {
            Bsim4v8ChargeMatrix {
                gcggb: c.cggb + c.cgdo + c.cgso + c.cgbo,
                gcgdb: c.cgdb - c.cgdo,
                gcgsb: c.cgsb - c.cgso,
                gcdgb: c.cdgb - c.cgdo,
                gcddb: c.cddb + c.capbd + c.cgdo,
                gcdsb: c.cdsb,
                gcsgb: -(c.cggb + c.cbgb + c.cdgb + c.cgso),
                gcsdb: -(c.cgdb + c.cbdb + c.cddb),
                gcssb: c.capbs + c.cgso - (c.cgsb + c.cbsb + c.cdsb),
                gcbgb: c.cbgb - c.cgbo,
                gcbdb: c.cbdb - c.capbd,
                gcbsb: c.cbsb - c.capbs,
            }
        } else {
            Bsim4v8ChargeMatrix {
                gcggb: c.cggb + c.cgdo + c.cgso + c.cgbo,
                gcgdb: c.cgsb - c.cgdo,
                gcgsb: c.cgdb - c.cgso,
                gcdgb: -(c.cggb + c.cbgb + c.cdgb + c.cgdo),
                gcddb: c.capbd + c.cgdo - (c.cgsb + c.cbsb + c.cdsb),
                gcdsb: -(c.cgdb + c.cbdb + c.cddb),
                gcsgb: c.cdgb - c.cgso,
                gcsdb: c.cdsb,
                gcssb: c.cddb + c.capbs + c.cgso,
                gcbgb: c.cbgb - c.cgbo,
                gcbdb: c.cbsb - c.capbd,
                gcbsb: c.cbdb - c.capbs,
            }
        }
    }

    /// Stamp the charge-companion matrix block (the `gc**` positions of the
    /// b4ld.c matrix load, with the bulk columns as the C's negative row
    /// sums `gcgbb`/`gcdbb`/`gcsbb`/`gcbbb`) scaled by `factor` (`ag0`, or
    /// `omega` for the AC imaginary part via a wrapping stamper). `m` is
    /// applied here.
    pub fn stamp_charge_matrix(
        &self,
        gc: &Bsim4v8ChargeMatrix,
        factor: Value,
        matrix: &mut impl MatrixStamper,
    ) {
        let (dp, g, sp, b) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        );
        let f = factor * self.multiplier;
        stamp(matrix, g, g, f * gc.gcggb);
        stamp(matrix, g, b, -f * (gc.gcggb + gc.gcgdb + gc.gcgsb));
        stamp(matrix, g, dp, f * gc.gcgdb);
        stamp(matrix, g, sp, f * gc.gcgsb);
        stamp(matrix, b, g, f * gc.gcbgb);
        stamp(matrix, b, b, -f * (gc.gcbgb + gc.gcbdb + gc.gcbsb));
        stamp(matrix, b, dp, f * gc.gcbdb);
        stamp(matrix, b, sp, f * gc.gcbsb);
        stamp(matrix, dp, g, f * gc.gcdgb);
        stamp(matrix, dp, b, -f * (gc.gcdgb + gc.gcddb + gc.gcdsb));
        stamp(matrix, dp, dp, f * gc.gcddb);
        stamp(matrix, dp, sp, f * gc.gcdsb);
        stamp(matrix, sp, g, f * gc.gcsgb);
        stamp(matrix, sp, b, -f * (gc.gcsgb + gc.gcsdb + gc.gcssb));
        stamp(matrix, sp, dp, f * gc.gcsdb);
        stamp(matrix, sp, sp, f * gc.gcssb);
    }

    /// Stamp the transient charge companion: `gc**·ag0` plus the equivalent
    /// charge currents `ceqq*` (b4ld.c:4696-4700, RHS rows of the canonical
    /// load). The `cq*` are the integrated charge currents of the composite
    /// states `qg`/`qb`/`qd` (junction charges folded in, b4ld.c:4582-4596).
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_charge_companion(
        &self,
        charge: &Bsim4v8Charge,
        mode: i32,
        ag0: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let (bias, _) = self.limited_branch_voltages(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;

        let gc = Self::charge_matrix(charge, mode);
        let g = |c: Value| c * ag0;

        // ceqq* linearization corrections (b4ld.c:4696-4700; gcdgmb/gcdbdb
        // are zero in the canonical mode), then the type<0 sign flip
        // (b4ld.c:4963-4966).
        let mut ceqqg = cqg - g(gc.gcggb) * vgb + g(gc.gcgdb) * vbd + g(gc.gcgsb) * vbs;
        let mut ceqqb = cqb - g(gc.gcbgb) * vgb + g(gc.gcbdb) * vbd + g(gc.gcbsb) * vbs;
        let mut ceqqd = cqd - g(gc.gcdgb) * vgb + g(gc.gcddb) * vbd + g(gc.gcdsb) * vbs;
        if self.core.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqb = -ceqqb;
            ceqqd = -ceqqd;
        }

        // RHS charge rows (b4ld.c:5023-5040, the mult_q parts).
        let m = self.multiplier;
        stamp_rhs(matrix, self.node_gate, -m * ceqqg);
        stamp_rhs(matrix, self.node_bulk, -m * ceqqb);
        stamp_rhs(matrix, self.node_drain, -m * ceqqd);
        stamp_rhs(matrix, self.node_source, m * (ceqqg + ceqqb + ceqqd));

        self.stamp_charge_matrix(&gc, ag0, matrix);
    }

    /// Stamp the linearized DC operating point: matrix/RHS load of
    /// b4ld.c:4750-5350 with all charge (`gc**`/`ceqq*`), NQS (`ggt*`,
    /// `T1`) and gate-current (`gI*tot`) terms zero and the series
    /// conductances lowered to external resistors (so `gdpr`/`gspr` and the
    /// `Dd`/`DPd`/`Ss`/`SPs` rows vanish here).
    fn stamp_op(&self, op: &Bsim4v8Op, bias: Bsim4v8Bias, matrix: &mut impl MatrixStamper) {
        let (dp, g, sp, b) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        );
        let mt = self.core.mtype;
        let m = self.multiplier;

        let vgd = bias.vgs - bias.vds;
        let vbd = bias.vbs - bias.vds;

        let (gm, gmbs, fwd_sum, rev_sum, ceqdrn, ceqbd, ceqbs);
        let (gbbdp, gbbsp);
        let (gbdpg, gbdpdp, gbdpb, gbdpsp);
        let (gbspg, gbspdp, gbspb, gbspsp);
        if op.mode >= 0 {
            gm = op.gm;
            gmbs = op.gmbs;
            fwd_sum = gm + gmbs;
            rev_sum = 0.0;

            ceqdrn = mt * (op.cd - op.gds * bias.vds - gm * bias.vgs - gmbs * bias.vbs);
            ceqbd = mt
                * (op.csub + op.igidl
                    - (op.gbds + op.ggidld) * bias.vds
                    - (op.gbgs + op.ggidlg) * bias.vgs
                    - (op.gbbs + op.ggidlb) * bias.vbs);
            ceqbs = mt
                * (op.igisl + op.ggisls * bias.vds - op.ggislg * vgd - op.ggislb * vbd);

            gbbdp = -op.gbds;
            gbbsp = op.gbds + op.gbgs + op.gbbs;

            gbdpg = op.gbgs;
            gbdpdp = op.gbds;
            gbdpb = op.gbbs;
            gbdpsp = -(gbdpg + gbdpdp + gbdpb);

            gbspg = 0.0;
            gbspdp = 0.0;
            gbspb = 0.0;
            gbspsp = 0.0;
        } else {
            gm = -op.gm;
            gmbs = -op.gmbs;
            fwd_sum = 0.0;
            rev_sum = -(gm + gmbs);

            ceqdrn = -mt * (op.cd + op.gds * bias.vds + gm * vgd + gmbs * vbd);
            ceqbs = mt
                * (op.csub + op.igisl + (op.gbds + op.ggisls) * bias.vds
                    - (op.gbgs + op.ggislg) * vgd
                    - (op.gbbs + op.ggislb) * vbd);
            ceqbd = mt
                * (op.igidl
                    - op.ggidld * bias.vds
                    - op.ggidlg * bias.vgs
                    - op.ggidlb * bias.vbs);

            gbbsp = -op.gbds;
            gbbdp = op.gbds + op.gbgs + op.gbbs;

            gbdpg = 0.0;
            gbdpsp = 0.0;
            gbdpb = 0.0;
            gbdpdp = 0.0;

            gbspg = op.gbgs;
            gbspsp = op.gbds;
            gbspb = op.gbbs;
            gbspdp = -(gbspg + gbspsp + gbspb);
        }

        // Junction equivalent currents; type<0 flips them (b4ld.c:4957-4961;
        // rbodyMod = 0 makes vbs_jct/vbd_jct the channel-side biases).
        let (ceqjs, ceqjd) = if mt > 0.0 {
            (op.cbs - op.gbs * bias.vbs, op.cbd - op.gbd * vbd)
        } else {
            (-(op.cbs - op.gbs * bias.vbs), -(op.cbd - op.gbd * vbd))
        };

        // RHS (b4ld.c:5023-5040, ceqq*/Igtoteq/ceqg*tot = 0).
        stamp_rhs(matrix, dp, m * (ceqjd - ceqbd - ceqdrn));
        stamp_rhs(matrix, b, m * (ceqbd + ceqbs - ceqjd - ceqjs));
        stamp_rhs(matrix, sp, m * (ceqdrn - ceqbs + ceqjs));

        // GIDL/GISL conductance shorthands (b4ld.c:5322-5327).
        let ggidl_sum = op.ggidlg + op.ggidld + op.ggidlb;
        let ggisl_sum = op.ggislg + op.ggisls + op.ggislb;

        // Matrix (b4ld.c:5299-5348, gc**/ggt*/T1/gI*tot = 0, series G
        // external, with the gidl/gisl stamp blocks folded in).
        stamp(matrix, dp, dp, m * (op.gds + op.gbd + rev_sum + gbdpdp + op.ggidld));
        stamp(matrix, dp, g, m * (gm + gbdpg + op.ggidlg));
        stamp(matrix, dp, sp, -m * (op.gds + fwd_sum - gbdpsp + ggidl_sum));
        stamp(matrix, dp, b, -m * (op.gbd - gmbs - gbdpb - op.ggidlb));
        stamp(matrix, sp, dp, -m * (op.gds + rev_sum - gbspdp + ggisl_sum));
        stamp(matrix, sp, g, m * (gbspg - gm + op.ggislg));
        stamp(matrix, sp, sp, m * (op.gds + op.gbs + fwd_sum + gbspsp + op.ggisls));
        stamp(matrix, sp, b, -m * (op.gbs + gmbs - gbspb - op.ggislb));
        stamp(matrix, b, dp, m * (gbbdp - op.gbd - op.ggidld + ggisl_sum));
        stamp(matrix, b, g, -m * (op.gbgs + op.ggidlg + op.ggislg));
        stamp(matrix, b, sp, m * (gbbsp - op.gbs + ggidl_sum - op.ggisls));
        stamp(matrix, b, b, m * (op.gbd + op.gbs - op.gbbs - op.ggidlb - op.ggislb));
    }
}

impl NonlinearDevice for Bsim4v8Device {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        let (bias, check) = self.limited_branch_voltages(voltages);
        self.last_limited.set(check);
        self.bias = bias;
        self.op = self.eval_dc(bias);
        self.von_prev = self.op.von;
        self.has_history = true;
        self.limit_anchor_valid.set(true);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (bias, _) = self.limited_branch_voltages(voltages);
        if bias == self.bias {
            self.stamp_op(&self.op, bias, matrix);
        } else {
            // A probe at a different solution than the last `update`:
            // re-linearize so the stamp and its equivalent currents agree.
            let op = self.eval_dc(bias);
            self.stamp_op(&op, bias, matrix);
        }
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.has_history || self.last_limited.get() {
            // ngspice bumps CKTnoncon whenever DEVpnjlim clips
            // (b4ld.c:4070-4072 via the Check flag).
            return false;
        }
        let reltol = criteria.relative_tolerance();
        let vtol = criteria.voltage_tolerance();
        let cmp = |new: Value, old: Value| {
            (new - old).abs() < reltol * new.abs().max(old.abs()) + vtol
        };
        cmp(self.bias.vds, self.converged_ref.vds)
            && cmp(self.bias.vgs, self.converged_ref.vgs)
            && cmp(self.bias.vbs, self.converged_ref.vbs)
    }
}

#[inline]
fn stamp(matrix: &mut impl MatrixStamper, row: NodeId, col: NodeId, value: Value) {
    if row != 0 && col != 0 && value != 0.0 {
        matrix.stamp(row, col, value);
    }
}

#[inline]
fn stamp_rhs(matrix: &mut impl MatrixStamper, node: NodeId, value: Value) {
    if node != 0 && value != 0.0 {
        matrix.stamp_rhs(node, value);
    }
}
