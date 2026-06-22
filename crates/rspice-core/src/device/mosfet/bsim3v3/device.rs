//! Engine-facing BSIM3v3.3 device (MOS levels 8/9/49).
//!
//! Wraps the model math of this module ([`Bsim3v3`]) in a
//! [`NonlinearDevice`] the engine can iterate and stamp, transcribing the
//! matrix/RHS load of ngspice-46 `b3ld.c:2920-3120` for `nqsMod = 0`:
//!
//! - the per-iteration limiting sequence (`DEVfetlim`/`DEVlimvds`/
//!   `DEVpnjlim`) anchored at the previous accepted iterate and `von`,
//!   with the `Check` flag participating in nonconvergence;
//! - the DC conductance/current stamp with the mode swap (`FwdSum`/
//!   `RevSum`, the `gbdp*`/`gbsp*` substrate-current groups) and the
//!   `type`-folded equivalent currents `cdreq`/`ceqbd`/`ceqbs`;
//! - the mode-dependent charge-companion assembly (b3ld.c:2560-2596 /
//!   2676-2711): intrinsic `c***` plus `cgdo`/`cgso`/`cgbo` overlaps plus
//!   `capbd`/`capbs`, scaled by the caller's integration gain (`ag0` in
//!   transient, `omega` on the imaginary axis in AC — b3acld.c stamps the
//!   identical matrix positions).
//!
//! The series drain/source conductances (`RSH * NRD/NRS`) are *not* stamped
//! here: the builder lowers them to ordinary linear resistors at internal
//! prime nodes, so `node_drain`/`node_source` are already the primes.
//! `CKTgmin` enters through [`Bsim3v3Device::set_eval_gmin`] and is consumed
//! inside the module's junction-diode equations — no second per-device gmin
//! shunt may be applied on top (see `eval.rs` scope notes).

use super::eval::{Bsim3v3Bias, Bsim3v3Charge, Bsim3v3Op};
use super::{Bsim3v3, eval};
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Complex64, Value, circuit::NodeId};

const TRNQS_SCALING: Value = 1.0e-9;

/// Mode-assembled charge-companion conductance matrix: the `gc**` of
/// b3ld.c:2560-2596 (mode > 0) / 2676-2711 (mode < 0) *before* the `ag0`
/// scaling, i.e. pure capacitances. AC multiplies by `omega` instead
/// (b3acld.c:356-369 forms the same expressions as `xc***`).
#[derive(Debug, Clone, Copy, Default)]
pub struct Bsim3v3ChargeMatrix {
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

/// One BSIM3v3.3 instance wired into the engine.
#[derive(Debug, Clone)]
pub struct Bsim3v3Device {
    pub name: String,
    /// Drain prime (the internal node when an RSH*NRD resistor exists).
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    /// Source prime (the internal node when an RSH*NRS resistor exists).
    pub node_source: NodeId,
    pub node_bulk: NodeId,
    /// Hidden charge-deficit state node for `NQSMOD=1`.
    pub node_charge_deficit: NodeId,
    /// Parallel multiplier `M` (ngspice `here->BSIM3m`, applied at stamp time).
    pub multiplier: Value,

    /// Model math: card + temperature/size knots + instance tail.
    pub core: Bsim3v3,

    /// Engine `CKTgmin` consumed by the junction diodes inside `eval`.
    gmin: Value,

    // Newton iteration state (ngspice CKTstate0 vds/vgs/vbs + BSIM3von).
    op: Bsim3v3Op,
    bias: Bsim3v3Bias,
    converged_ref: Bsim3v3Bias,
    von_prev: Value,
    has_history: bool,
    /// The previous iterate exists, so the b3ld.c limiting sequence applies
    /// (the first iterate of an analysis phase passes through unlimited).
    limit_anchor_valid: std::cell::Cell<bool>,
    /// `DEVpnjlim` flagged the body junction on the last update (ngspice
    /// bumps `CKTnoncon`; here it vetoes device convergence for the iterate).
    last_limited: std::cell::Cell<bool>,
}

impl Bsim3v3Device {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_bulk: NodeId,
        node_charge_deficit: NodeId,
        multiplier: Value,
        core: Bsim3v3,
    ) -> Self {
        Self {
            name,
            node_drain,
            node_gate,
            node_source,
            node_bulk,
            node_charge_deficit,
            multiplier: if multiplier > 0.0 { multiplier } else { 1.0 },
            core,
            gmin: 1e-12,
            op: Bsim3v3Op::default(),
            bias: Bsim3v3Bias {
                vds: 0.0,
                vgs: 0.0,
                vbs: 0.0,
            },
            converged_ref: Bsim3v3Bias {
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

    /// Set the engine's junction GMIN (ngspice `CKTgmin`). The module's diode
    /// equations include it exactly as b3ld.c does, so this is the only gmin
    /// path for the device.
    pub fn set_eval_gmin(&mut self, gmin: Value) {
        self.gmin = if gmin.is_finite() && gmin > 0.0 {
            gmin
        } else {
            0.0
        };
    }

    /// Device-polarity branch voltages from the solution vector
    /// (b3ld.c:248-256: `mtype` folded, source-referenced). The solution is
    /// 0-indexed (node 1 -> `v[0]`); ground reads as 0.
    fn raw_branch_voltages(&self, v: &[Value]) -> Bsim3v3Bias {
        let vd = Self::node_voltage(v, self.node_drain);
        let vg = Self::node_voltage(v, self.node_gate);
        let vs = Self::node_voltage(v, self.node_source);
        let vb = Self::node_voltage(v, self.node_bulk);
        let mt = self.core.mtype;
        Bsim3v3Bias {
            vds: mt * (vd - vs),
            vgs: mt * (vg - vs),
            vbs: mt * (vb - vs),
        }
    }

    #[inline]
    fn node_voltage(v: &[Value], n: NodeId) -> Value {
        if n == 0 { 0.0 } else { v[n - 1] }
    }

    /// The b3ld.c limiting sequence against the previous accepted iterate;
    /// the first iterate of a phase passes through (ngspice seeds CKTstate0
    /// before the first NIiter, the engine seeds from the raw solution).
    fn limited_branch_voltages(&self, v: &[Value]) -> (Bsim3v3Bias, bool) {
        let raw = self.raw_branch_voltages(v);
        if !self.limit_anchor_valid.get() {
            return (raw, false);
        }
        self.core.limit_voltages(raw, self.bias, self.von_prev)
    }

    /// Charge state at the limited bias (the same bias the conductance stamp
    /// uses — mixing raw voltages with limited-bias conductances would inject
    /// companion currents that do not cancel at convergence). Returns the
    /// charge together with the channel mode of that evaluation, which
    /// selects the companion-matrix assembly.
    pub fn charge_at(&self, v: &[Value]) -> (Bsim3v3Charge, i32) {
        let (bias, _) = self.limited_branch_voltages(v);
        let op = self
            .core
            .eval(bias, self.gmin, true)
            .expect("BSIM3 charge eval: CAPMOD validated at build");
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

    pub fn uses_ac_nqs(&self) -> bool {
        self.core.model.acnqs_mod != 0
    }

    pub fn uses_trnqs(&self) -> bool {
        self.core.model.nqs_mod != 0
    }

    pub fn trnqs_qdef(&self, voltages: &[Value]) -> Value {
        self.core.mtype * Self::node_voltage(voltages, self.node_charge_deficit)
    }

    pub fn trnqs_qcdump_state(&self, voltages: &[Value]) -> Value {
        self.trnqs_qdef(voltages) * TRNQS_SCALING
    }

    pub fn trnqs_state_charges(&self, charge: &Bsim3v3Charge) -> (Value, Value, Value) {
        (charge.qg_state(), charge.qb_state(), charge.qd_state())
    }

    /// Operating-point snapshot consumed by the noise analysis. The wrapper
    /// owns the cached `b3ld.c` quantities, while the engine constructs the
    /// equivalent current-noise sources so the device math stays decoupled
    /// from the analysis layer.
    pub fn noise_operating_point(&self) -> (&Bsim3v3Op, Bsim3v3Bias) {
        (&self.op, self.bias)
    }

    /// Operating-point snapshot with supported CAPMOD charge bookkeeping enabled.
    /// BSIM3 `noiMod=2/4` channel thermal noise consumes `BSIM3qinv`, which
    /// the DC-only path deliberately leaves at zero.
    pub fn noise_operating_point_with_charge(&self) -> (Bsim3v3Op, Bsim3v3Bias) {
        match self.core.eval(self.bias, self.gmin, true) {
            Ok(op) => (op, self.bias),
            Err(err) => {
                log::warn!(
                    "BSIM3 '{}': noise qinv evaluation failed ({err}); using cached DC snapshot",
                    self.name
                );
                (self.op.clone(), self.bias)
            }
        }
    }

    /// Assemble the mode-dependent charge-companion capacitance matrix
    /// (b3ld.c:2560-2596 / 2676-2711 with `ag0 = 1`).
    pub fn charge_matrix(charge: &Bsim3v3Charge, mode: i32) -> Bsim3v3ChargeMatrix {
        let c = charge;
        if mode > 0 {
            Bsim3v3ChargeMatrix {
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
            Bsim3v3ChargeMatrix {
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
    /// b3ld.c matrix load) scaled by `factor` (`ag0`, or `omega` for the AC
    /// imaginary part via a wrapping stamper). `m` is applied here.
    pub fn stamp_charge_matrix(
        &self,
        gc: &Bsim3v3ChargeMatrix,
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

    /// AC-only charge-deficit NQS correction of ngspice-46 `b3acld.c`.
    /// The AC engine has already stamped the QS real Jacobian and QS
    /// `j*omega*C`; this adds the delta that converts the intrinsic
    /// channel/charge rows to `ACNQSMOD=1`.
    pub fn stamp_ac_nqs_correction(
        &self,
        charge: &Bsim3v3Charge,
        mode: i32,
        omega: Value,
        mut stamp: impl FnMut(NodeId, NodeId, Complex64),
    ) {
        if !self.uses_ac_nqs()
            || omega == 0.0
            || !omega.is_finite()
            || charge.taunet <= 0.0
            || !charge.taunet.is_finite()
        {
            return;
        }

        let t0 = omega * charge.taunet;
        let t2 = 1.0 / (1.0 + t0 * t0);
        let t3 = t0 * t2;
        let nodes = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        );
        let m = self.multiplier;

        Self::stamp_ac_nqs_channel_delta(
            mode,
            nodes,
            self.op.gm * (t2 - 1.0),
            self.op.gmbs * (t2 - 1.0),
            self.op.gds * (t2 - 1.0),
            Complex64::new(m, 0.0),
            &mut stamp,
        );
        Self::stamp_ac_nqs_channel_delta(
            mode,
            nodes,
            -self.op.gm * t3,
            -self.op.gmbs * t3,
            -self.op.gds * t3,
            Complex64::new(0.0, m),
            &mut stamp,
        );

        let c = charge;
        let csd = -(c.cddb + c.cgdb + c.cbdb);
        let csg = -(c.cdgb + c.cggb + c.cbgb);
        let css = -(c.cdsb + c.cgsb + c.cbsb);

        let cddr = c.cddb * t2;
        let cdgr = c.cdgb * t2;
        let cdsr = c.cdsb * t2;
        let cddi = c.cddb * t3 * omega;
        let cdgi = c.cdgb * t3 * omega;
        let cdsi = c.cdsb * t3 * omega;
        let cdbi = -(cddi + cdgi + cdsi);

        let csdr = csd * t2;
        let csgr = csg * t2;
        let cssr = css * t2;
        let csdi = csd * t3 * omega;
        let csgi = csg * t3 * omega;
        let cssi = css * t3 * omega;
        let csbi = -(csdi + csgi + cssi);

        let cgdr = -(cddr + csdr + c.cbdb);
        let cggr = -(cdgr + csgr + c.cbgb);
        let cgsr = -(cdsr + cssr + c.cbsb);
        let cgdi = -(cddi + csdi);
        let cggi = -(cdgi + csgi);
        let cgsi = -(cdsi + cssi);
        let cgbi = -(cgdi + cggi + cgsi);

        let (
            xcggbr,
            xcgdbr,
            xcgsbr,
            xcgbbr,
            xcdgbr,
            xcddbr,
            xcdsbr,
            xcdbbr,
            xcsgbr,
            xcsdbr,
            xcssbr,
            xcsbbr,
            xcbgb,
            xcbdb,
            xcbsb,
            xcbbb,
            xcggbi,
            xcgdbi,
            xcgsbi,
            xcgbbi,
            xcdgbi,
            xcddbi,
            xcdsbi,
            xcdbbi,
            xcsgbi,
            xcsdbi,
            xcssbi,
            xcsbbi,
        ) = if mode >= 0 {
            let xcggbr = (cggr + c.cgdo + c.cgso + c.cgbo) * omega;
            let xcgdbr = (cgdr - c.cgdo) * omega;
            let xcgsbr = (cgsr - c.cgso) * omega;
            let xcgbbr = -(xcggbr + xcgdbr + xcgsbr);
            let xcdgbr = (cdgr - c.cgdo) * omega;
            let xcddbr = (cddr + c.capbd + c.cgdo) * omega;
            let xcdsbr = cdsr * omega;
            let xcdbbr = -(xcdgbr + xcddbr + xcdsbr);
            let xcsgbr = (csgr - c.cgso) * omega;
            let xcsdbr = csdr * omega;
            let xcssbr = (c.capbs + c.cgso + cssr) * omega;
            let xcsbbr = -(xcsgbr + xcsdbr + xcssbr);
            let xcbgb = (c.cbgb - c.cgbo) * omega;
            let xcbdb = (c.cbdb - c.capbd) * omega;
            let xcbsb = (c.cbsb - c.capbs) * omega;
            let xcbbb = -(xcbgb + xcbdb + xcbsb);
            (
                xcggbr, xcgdbr, xcgsbr, xcgbbr, xcdgbr, xcddbr, xcdsbr, xcdbbr, xcsgbr, xcsdbr,
                xcssbr, xcsbbr, xcbgb, xcbdb, xcbsb, xcbbb, cggi, cgdi, cgsi, cgbi, cdgi, cddi,
                cdsi, cdbi, csgi, csdi, cssi, csbi,
            )
        } else {
            let xcggbr = (cggr + c.cgdo + c.cgso + c.cgbo) * omega;
            let xcgdbr = (cgsr - c.cgdo) * omega;
            let xcgsbr = (cgdr - c.cgso) * omega;
            let xcgbbr = -(xcggbr + xcgdbr + xcgsbr);
            let xcdgbr = (csgr - c.cgdo) * omega;
            let xcddbr = (c.capbd + c.cgdo + cssr) * omega;
            let xcdsbr = csdr * omega;
            let xcdbbr = -(xcdgbr + xcddbr + xcdsbr);
            let xcsgbr = (cdgr - c.cgso) * omega;
            let xcsdbr = cdsr * omega;
            let xcssbr = (cddr + c.capbs + c.cgso) * omega;
            let xcsbbr = -(xcsgbr + xcsdbr + xcssbr);
            let xcbgb = (c.cbgb - c.cgbo) * omega;
            let xcbdb = (c.cbsb - c.capbd) * omega;
            let xcbsb = (c.cbdb - c.capbs) * omega;
            let xcbbb = -(xcbgb + xcbdb + xcbsb);
            (
                xcggbr, xcgdbr, xcgsbr, xcgbbr, xcdgbr, xcddbr, xcdsbr, xcdbbr, xcsgbr, xcsdbr,
                xcssbr, xcsbbr, xcbgb, xcbdb, xcbsb, xcbbb, cggi, cgsi, cgdi, cgbi, csgi, cssi,
                csdi, csbi, cdgi, cdsi, cddi, cdbi,
            )
        };

        let qs = Self::charge_matrix(charge, mode);
        let qs_gp_bp = -(qs.gcggb + qs.gcgdb + qs.gcgsb) * omega;
        let qs_dp_bp = -(qs.gcdgb + qs.gcddb + qs.gcdsb) * omega;
        let qs_sp_bp = -(qs.gcsgb + qs.gcsdb + qs.gcssb) * omega;
        let qs_bp_bp = -(qs.gcbgb + qs.gcbdb + qs.gcbsb) * omega;
        let mut add_cap_delta =
            |row: NodeId, col: NodeId, real: Value, nqs_imag: Value, qs_imag: Value| {
                let value = Complex64::new(m * real, m * (nqs_imag - qs_imag));
                if value.re != 0.0 || value.im != 0.0 {
                    stamp(row, col, value);
                }
            };
        let (dp, g, sp, b) = nodes;
        add_cap_delta(g, g, xcggbi, xcggbr, qs.gcggb * omega);
        add_cap_delta(g, dp, xcgdbi, xcgdbr, qs.gcgdb * omega);
        add_cap_delta(g, sp, xcgsbi, xcgsbr, qs.gcgsb * omega);
        add_cap_delta(g, b, xcgbbi, xcgbbr, qs_gp_bp);

        add_cap_delta(dp, g, xcdgbi, xcdgbr, qs.gcdgb * omega);
        add_cap_delta(dp, dp, xcddbi, xcddbr, qs.gcddb * omega);
        add_cap_delta(dp, sp, xcdsbi, xcdsbr, qs.gcdsb * omega);
        add_cap_delta(dp, b, xcdbbi, xcdbbr, qs_dp_bp);

        add_cap_delta(sp, g, xcsgbi, xcsgbr, qs.gcsgb * omega);
        add_cap_delta(sp, dp, xcsdbi, xcsdbr, qs.gcsdb * omega);
        add_cap_delta(sp, sp, xcssbi, xcssbr, qs.gcssb * omega);
        add_cap_delta(sp, b, xcsbbi, xcsbbr, qs_sp_bp);

        add_cap_delta(b, g, 0.0, xcbgb, qs.gcbgb * omega);
        add_cap_delta(b, dp, 0.0, xcbdb, qs.gcbdb * omega);
        add_cap_delta(b, sp, 0.0, xcbsb, qs.gcbsb * omega);
        add_cap_delta(b, b, 0.0, xcbbb, qs_bp_bp);
    }

    #[allow(clippy::too_many_arguments)]
    fn stamp_ac_nqs_channel_delta(
        mode: i32,
        nodes: (NodeId, NodeId, NodeId, NodeId),
        gm_in: Value,
        gmb_in: Value,
        gds_in: Value,
        scale: Complex64,
        stamp: &mut impl FnMut(NodeId, NodeId, Complex64),
    ) {
        let (dp, g, sp, b) = nodes;
        let (gm, gmb, fwd_sum, rev_sum) = if mode >= 0 {
            (gm_in, gmb_in, gm_in + gmb_in, 0.0)
        } else {
            let gm = -gm_in;
            let gmb = -gmb_in;
            (gm, gmb, 0.0, -(gm + gmb))
        };
        let mut add = |row: NodeId, col: NodeId, value: Value| {
            if value != 0.0 {
                stamp(row, col, Complex64::new(value * scale.re, value * scale.im));
            }
        };
        add(dp, dp, gds_in + rev_sum);
        add(dp, g, gm);
        add(dp, sp, -(gds_in + fwd_sum));
        add(dp, b, gmb);
        add(sp, dp, -(gds_in + rev_sum));
        add(sp, g, -gm);
        add(sp, sp, gds_in + fwd_sum);
        add(sp, b, -gmb);
    }

    /// Stamp the transient charge companion: `gc**·ag0` plus the equivalent
    /// charge currents `ceqq*` (b3ld.c:2860-2880, RHS rows of line900). The
    /// `cq*` are the integrated charge currents of the composite states
    /// `qg`/`qb`/`qd` (junction charges folded in, b3ld.c:2796-2801).
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_charge_companion(
        &self,
        charge: &Bsim3v3Charge,
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

        // ceqq* linearization corrections (b3ld.c:2862-2864), then the
        // type<0 sign flip (b3ld.c:3001-3004).
        let mut ceqqg = cqg - g(gc.gcggb) * vgb + g(gc.gcgdb) * vbd + g(gc.gcgsb) * vbs;
        let mut ceqqb = cqb - g(gc.gcbgb) * vgb + g(gc.gcbdb) * vbd + g(gc.gcbsb) * vbs;
        let mut ceqqd = cqd - g(gc.gcdgb) * vgb + g(gc.gcddb) * vbd + g(gc.gcdsb) * vbs;
        if self.core.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqb = -ceqqb;
            ceqqd = -ceqqd;
        }

        // RHS charge rows (b3ld.c:3010-3015).
        let m = self.multiplier;
        stamp_rhs(matrix, self.node_gate, -m * ceqqg);
        stamp_rhs(matrix, self.node_bulk, -m * ceqqb);
        stamp_rhs(matrix, self.node_drain, -m * ceqqd);
        stamp_rhs(matrix, self.node_source, m * (ceqqg + ceqqb + ceqqd));

        self.stamp_charge_matrix(&gc, ag0, matrix);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stamp_trnqs_charge_companion(
        &self,
        charge: &Bsim3v3Charge,
        mode: i32,
        ag0: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqcheq: Value,
        cqcdump: Value,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let cox_wl = charge.cox_wl;
        if !(cox_wl > 0.0 && cox_wl.is_finite()) {
            return;
        }
        let gtau = charge.gtau;
        if !(gtau.is_finite() && gtau >= 0.0) {
            return;
        }

        let (bias, _) = self.limited_branch_voltages(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;
        let qdef = self.trnqs_qdef(voltages);
        let t0 = if charge.qcheq > 0.0 {
            self.core.inst.tconst * qdef * TRNQS_SCALING
        } else {
            -self.core.inst.tconst * qdef * TRNQS_SCALING
        };

        let (
            ggtg,
            ggtd,
            ggts,
            ggtb,
            gcqgb,
            gcqdb,
            gcqsb,
            gcqbb,
            dxpart,
            sxpart,
            ddxpart_dvd,
            ddxpart_dvg,
            ddxpart_dvs,
            ddxpart_dvb,
            dsxpart_dvd,
            dsxpart_dvg,
            dsxpart_dvs,
            dsxpart_dvb,
        ) = if mode > 0 {
            let ggtg = t0 * charge.cqgb;
            let ggtd = t0 * charge.cqdb;
            let ggts = t0 * charge.cqsb;
            let ggtb = t0 * charge.cqbb;
            let gcqgb = charge.cqgb * ag0;
            let gcqdb = charge.cqdb * ag0;
            let gcqsb = charge.cqsb * ag0;
            let gcqbb = charge.cqbb * ag0;
            let (dxpart, ddxpart_dvd, ddxpart_dvg, ddxpart_dvs, ddxpart_dvb) =
                if charge.qcheq.abs() <= 1.0e-5 * cox_wl {
                    let dxpart = if self.core.model.xpart < 0.5 {
                        0.4
                    } else if self.core.model.xpart > 0.5 {
                        0.0
                    } else {
                        0.5
                    };
                    (dxpart, 0.0, 0.0, 0.0, 0.0)
                } else {
                    let dxpart = charge.qdrn_channel / charge.qcheq;
                    let cdd = charge.cddb;
                    let csd = -(charge.cgdb + charge.cddb + charge.cbdb);
                    let ddxpart_dvd = (cdd - dxpart * (cdd + csd)) / charge.qcheq;
                    let cdg = charge.cdgb;
                    let csg = -(charge.cggb + charge.cdgb + charge.cbgb);
                    let ddxpart_dvg = (cdg - dxpart * (cdg + csg)) / charge.qcheq;
                    let cds = charge.cdsb;
                    let css = -(charge.cgsb + charge.cdsb + charge.cbsb);
                    let ddxpart_dvs = (cds - dxpart * (cds + css)) / charge.qcheq;
                    let ddxpart_dvb = -(ddxpart_dvd + ddxpart_dvg + ddxpart_dvs);
                    (dxpart, ddxpart_dvd, ddxpart_dvg, ddxpart_dvs, ddxpart_dvb)
                };
            let sxpart = 1.0 - dxpart;
            let dsxpart_dvd = -ddxpart_dvd;
            let dsxpart_dvg = -ddxpart_dvg;
            let dsxpart_dvs = -ddxpart_dvs;
            let dsxpart_dvb = -(dsxpart_dvd + dsxpart_dvg + dsxpart_dvs);
            (
                ggtg,
                ggtd,
                ggts,
                ggtb,
                gcqgb,
                gcqdb,
                gcqsb,
                gcqbb,
                dxpart,
                sxpart,
                ddxpart_dvd,
                ddxpart_dvg,
                ddxpart_dvs,
                ddxpart_dvb,
                dsxpart_dvd,
                dsxpart_dvg,
                dsxpart_dvs,
                dsxpart_dvb,
            )
        } else {
            let ggtg = t0 * charge.cqgb;
            let ggts = t0 * charge.cqdb;
            let ggtd = t0 * charge.cqsb;
            let ggtb = t0 * charge.cqbb;
            let gcqgb = charge.cqgb * ag0;
            let gcqdb = charge.cqsb * ag0;
            let gcqsb = charge.cqdb * ag0;
            let gcqbb = charge.cqbb * ag0;
            let (sxpart, dsxpart_dvd, dsxpart_dvg, dsxpart_dvs, dsxpart_dvb) =
                if charge.qcheq.abs() <= 1.0e-5 * cox_wl {
                    let sxpart = if self.core.model.xpart < 0.5 {
                        0.4
                    } else if self.core.model.xpart > 0.5 {
                        0.0
                    } else {
                        0.5
                    };
                    (sxpart, 0.0, 0.0, 0.0, 0.0)
                } else {
                    let sxpart = charge.qdrn_channel / charge.qcheq;
                    let css = charge.cddb;
                    let cds = -(charge.cgdb + charge.cddb + charge.cbdb);
                    let dsxpart_dvs = (css - sxpart * (css + cds)) / charge.qcheq;
                    let csg = charge.cdgb;
                    let cdg = -(charge.cggb + charge.cdgb + charge.cbgb);
                    let dsxpart_dvg = (csg - sxpart * (csg + cdg)) / charge.qcheq;
                    let csd = charge.cdsb;
                    let cdd = -(charge.cgsb + charge.cdsb + charge.cbsb);
                    let dsxpart_dvd = (csd - sxpart * (csd + cdd)) / charge.qcheq;
                    let dsxpart_dvb = -(dsxpart_dvd + dsxpart_dvg + dsxpart_dvs);
                    (sxpart, dsxpart_dvd, dsxpart_dvg, dsxpart_dvs, dsxpart_dvb)
                };
            let dxpart = 1.0 - sxpart;
            let ddxpart_dvd = -dsxpart_dvd;
            let ddxpart_dvg = -dsxpart_dvg;
            let ddxpart_dvs = -dsxpart_dvs;
            let ddxpart_dvb = -(ddxpart_dvd + ddxpart_dvg + ddxpart_dvs);
            (
                ggtg,
                ggtd,
                ggts,
                ggtb,
                gcqgb,
                gcqdb,
                gcqsb,
                gcqbb,
                dxpart,
                sxpart,
                ddxpart_dvd,
                ddxpart_dvg,
                ddxpart_dvs,
                ddxpart_dvb,
                dsxpart_dvd,
                dsxpart_dvg,
                dsxpart_dvs,
                dsxpart_dvb,
            )
        };

        let gcggb = (charge.cgdo + charge.cgso + charge.cgbo) * ag0;
        let gcgdb = -charge.cgdo * ag0;
        let gcgsb = -charge.cgso * ag0;
        let gcgbb = -(gcggb + gcgdb + gcgsb);
        let gcdgb = gcgdb;
        let gcddb = (charge.capbd + charge.cgdo) * ag0;
        let gcdsb = 0.0;
        let gcdbb = -(gcdgb + gcddb + gcdsb);
        let gcsgb = gcgsb;
        let gcsdb = 0.0;
        let gcssb = (charge.capbs + charge.cgso) * ag0;
        let gcsbb = -(gcsgb + gcsdb + gcssb);
        let gcbgb = -charge.cgbo * ag0;
        let gcbdb = -charge.capbd * ag0;
        let gcbsb = -charge.capbs * ag0;
        let gcbbb = -(gcbgb + gcbdb + gcbsb);

        let nqs_terminal = ggtg * vgb - ggtd * vbd - ggts * vbs;
        let t1 = qdef * gtau;
        let mut ceqqg = cqg - gcggb * vgb + gcgdb * vbd + gcgsb * vbs + nqs_terminal;
        let mut ceqqd = cqd - gcdgb * vgb + gcddb * vbd + gcdsb * vbs
            - dxpart * nqs_terminal
            - t1 * (ddxpart_dvg * vgb - ddxpart_dvd * vbd - ddxpart_dvs * vbs);
        let mut ceqqb = cqb - gcbgb * vgb + gcbdb * vbd + gcbsb * vbs;
        let gqdef = TRNQS_SCALING * ag0;
        let mut cqdef = cqcdump - gqdef * qdef;
        let mut cqcheq_eq = cqcheq - (gcqgb * vgb - gcqdb * vbd - gcqsb * vbs) + nqs_terminal;
        if self.core.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqd = -ceqqd;
            ceqqb = -ceqqb;
            cqdef = -cqdef;
            cqcheq_eq = -cqcheq_eq;
        }

        let m = self.multiplier;
        let (dp, g, sp, b, q) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
            self.node_charge_deficit,
        );
        stamp_rhs(matrix, g, -m * ceqqg);
        stamp_rhs(matrix, b, -m * ceqqb);
        stamp_rhs(matrix, dp, -m * ceqqd);
        stamp_rhs(matrix, sp, m * (ceqqg + ceqqb + ceqqd));
        stamp_rhs(matrix, q, m * (cqcheq_eq - cqdef));

        stamp(matrix, g, g, m * (gcggb - ggtg));
        stamp(matrix, g, b, m * (gcgbb - ggtb));
        stamp(matrix, g, dp, m * (gcgdb - ggtd));
        stamp(matrix, g, sp, m * (gcgsb - ggts));

        stamp(matrix, b, g, m * gcbgb);
        stamp(matrix, b, b, m * gcbbb);
        stamp(matrix, b, dp, m * gcbdb);
        stamp(matrix, b, sp, m * gcbsb);

        stamp(
            matrix,
            dp,
            g,
            m * (gcdgb + dxpart * ggtg + t1 * ddxpart_dvg),
        );
        stamp(
            matrix,
            dp,
            b,
            m * (gcdbb + dxpart * ggtb + t1 * ddxpart_dvb),
        );
        stamp(
            matrix,
            dp,
            dp,
            m * (gcddb + dxpart * ggtd + t1 * ddxpart_dvd),
        );
        stamp(
            matrix,
            dp,
            sp,
            m * (gcdsb + dxpart * ggts + t1 * ddxpart_dvs),
        );

        stamp(
            matrix,
            sp,
            g,
            m * (gcsgb + sxpart * ggtg + t1 * dsxpart_dvg),
        );
        stamp(
            matrix,
            sp,
            b,
            m * (gcsbb + sxpart * ggtb + t1 * dsxpart_dvb),
        );
        stamp(
            matrix,
            sp,
            dp,
            m * (gcsdb + sxpart * ggtd + t1 * dsxpart_dvd),
        );
        stamp(
            matrix,
            sp,
            sp,
            m * (gcssb + sxpart * ggts + t1 * dsxpart_dvs),
        );

        stamp(matrix, q, q, m * (gqdef + gtau));
        stamp(matrix, q, g, m * (ggtg - gcqgb));
        stamp(matrix, q, dp, m * (ggtd - gcqdb));
        stamp(matrix, q, sp, m * (ggts - gcqsb));
        stamp(matrix, q, b, m * (ggtb - gcqbb));
        stamp(matrix, dp, q, m * (dxpart * gtau));
        stamp(matrix, sp, q, m * (sxpart * gtau));
        stamp(matrix, g, q, -m * gtau);
    }

    /// Stamp the linearized DC operating point: matrix/RHS load of
    /// b3ld.c:2920-3120 with all charge (`gc**`/`ceqq*`) and NQS (`ggt*`,
    /// `T1`) terms zero and the series conductances lowered to external
    /// resistors (so the `Dd`/`DPd`/`Ss`/`SPs` rows vanish here).
    fn stamp_op(&self, op: &Bsim3v3Op, bias: Bsim3v3Bias, matrix: &mut impl MatrixStamper) {
        let (dp, g, sp, b) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        );
        let mt = self.core.mtype;
        let m = self.multiplier;

        let (gm, gmbs, fwd_sum, rev_sum, cdreq, mut ceqbd, mut ceqbs);
        let (gbbdp, gbbsp);
        let (gbdpg, gbdpdp, gbdpb, gbdpsp);
        let (gbspg, gbspdp, gbspb, gbspsp);
        if op.mode >= 0 {
            gm = op.gm;
            gmbs = op.gmbs;
            fwd_sum = gm + gmbs;
            rev_sum = 0.0;
            cdreq = mt * (op.cd - op.gds * bias.vds - gm * bias.vgs - gmbs * bias.vbs);

            ceqbd = -mt * (op.csub - op.gbds * bias.vds - op.gbgs * bias.vgs - op.gbbs * bias.vbs);
            ceqbs = 0.0;

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
            let vgd = bias.vgs - bias.vds;
            let vbd = bias.vbs - bias.vds;
            cdreq = -mt * (op.cd + op.gds * bias.vds + gm * vgd + gmbs * vbd);

            ceqbs = -mt * (op.csub + op.gbds * bias.vds - op.gbgs * vgd - op.gbbs * vbd);
            ceqbd = 0.0;

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

        // Junction equivalent currents; type<0 flips them (b3ld.c:2975-2998).
        if mt > 0.0 {
            ceqbs += op.cbs - op.gbs * bias.vbs;
            ceqbd += op.cbd - op.gbd * (bias.vbs - bias.vds);
        } else {
            ceqbs -= op.cbs - op.gbs * bias.vbs;
            ceqbd -= op.cbd - op.gbd * (bias.vbs - bias.vds);
        }

        // RHS (b3ld.c:3010-3015, ceqq* = 0).
        stamp_rhs(matrix, b, -m * (ceqbs + ceqbd));
        stamp_rhs(matrix, dp, m * (ceqbd - cdreq));
        stamp_rhs(matrix, sp, m * (cdreq + ceqbs));

        // Matrix (b3ld.c:3050-3091, gc**/ggt*/T1 = 0, series G external).
        stamp(matrix, b, b, m * (op.gbd + op.gbs - op.gbbs));
        stamp(matrix, dp, dp, m * (op.gds + op.gbd + rev_sum + gbdpdp));
        stamp(matrix, sp, sp, m * (op.gds + op.gbs + fwd_sum + gbspsp));
        stamp(matrix, b, g, -m * op.gbgs);
        stamp(matrix, b, dp, m * (-op.gbd + gbbdp));
        stamp(matrix, b, sp, m * (-op.gbs + gbbsp));
        stamp(matrix, dp, g, m * (gm + gbdpg));
        stamp(matrix, dp, b, -m * (op.gbd - gmbs - gbdpb));
        stamp(matrix, dp, sp, -m * (op.gds + fwd_sum - gbdpsp));
        stamp(matrix, sp, g, m * (-gm + gbspg));
        stamp(matrix, sp, b, -m * (op.gbs + gmbs - gbspb));
        stamp(matrix, sp, dp, -m * (op.gds + rev_sum - gbspdp));
    }
}

impl NonlinearDevice for Bsim3v3Device {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        let (bias, check) = self.limited_branch_voltages(voltages);
        self.last_limited.set(check);
        self.bias = bias;
        self.op = eval::eval_dc(
            &self.core.model,
            &self.core.model_temp,
            &self.core.size,
            &self.core.inst,
            bias,
            self.gmin,
        );
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
            let op = eval::eval_dc(
                &self.core.model,
                &self.core.model_temp,
                &self.core.size,
                &self.core.inst,
                bias,
                self.gmin,
            );
            self.stamp_op(&op, bias, matrix);
        }
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.has_history || self.last_limited.get() {
            // ngspice bumps CKTnoncon whenever DEVpnjlim clips (b3ld.c:371).
            return false;
        }
        let reltol = criteria.relative_tolerance();
        let vtol = criteria.voltage_tolerance();
        let cmp =
            |new: Value, old: Value| (new - old).abs() < reltol * new.abs().max(old.abs()) + vtol;
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
