//! Engine-facing BSIM4 v4.8 device (MOS levels 14/54).
//!
//! Wraps the model math of this module ([`Bsim4v8`]) in a
//! [`NonlinearDevice`] the engine can iterate and stamp, transcribing the
//! matrix/RHS load of ngspice-46 `b4ld.c:4750-5350` for the canonical mode
//! set plus `rdsMod=1` and the `rbodyMod=1/2` DC substrate-resistance networks;
//! the builder lowers `rgateMod=1` to an ordinary external gate resistor and
//! points this device at the resulting gate-prime node (`trnqsMod = 0`):
//!
//! - the per-iteration limiting sequence (`DEVfetlim`/`DEVlimvds`/
//!   `DEVpnjlim`, b4ld.c:605-689) anchored at the previous accepted iterate
//!   and `von`, with the `Check` flag participating in nonconvergence
//!   (b4ld.c:4070-4072);
//! - the DC conductance/current stamp with the mode swap (`FwdSum`/
//!   `RevSum`, the `gbdp*`/`gbsp*` substrate-current groups and the
//!   `gbbdp`/`gbbsp` bulk rows), gate-tunneling current rows, the GIDL/GISL
//!   injections of the dedicated stamp blocks (b4ld.c:5330-5348), and the
//!   `type`-folded equivalent currents;
//! - the mode-dependent charge-companion assembly (b4ld.c:4186-4566 for
//!   `trnqsMod = 0`): intrinsic `c***` plus the smoothed `cgdo`/`cgso`
//!   overlaps and the bias-independent `cgbo`, plus `capbd`/`capbs`, scaled
//!   by the caller's integration gain (`ag0` in transient, `omega` on the
//!   imaginary axis in AC). For `rbodyMod=1/2`, the AC load follows b4acld.c
//!   and routes the junction caps to the drain/source body nodes.
//!
//! Fixed `RSH * NRD/NRS` conductances are lowered by the builder for
//! `rdsMod=0`; the `rdsMod=1` external `gstot`/`gdtot` branches are stamped
//! here. `CKTgmin` enters through [`Bsim4v8Device::set_eval_gmin`] and is
//! consumed inside the module's junction-diode equations; no second
//! per-device gmin shunt may be applied on top (see `eval.rs` scope notes).

use super::eval::{Bsim4v8Bias, Bsim4v8Charge, Bsim4v8JunctionBias, Bsim4v8Op};
use super::{Bsim4v8, common, pnjlim};
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Complex64, Value, circuit::NodeId};

const TRNQS_SCALING: Value = 1.0e-9;

/// Mode-assembled charge-companion conductance matrix: the `gc**` of
/// b4ld.c:4216-4260 (mode > 0) / 4408-4456 (mode < 0) *before* the `ag0`
/// scaling, i.e. pure capacitances. AC multiplies by `omega` instead
/// (b4acld.c forms the same expressions as `xc***`). For `rgateMod=3`, the
/// `gc*gmb` entries route the overlap capacitances through the middle-gate
/// electrode. The bulk columns (`gcgbb`/`gcdbb`/`gcsbb`/`gcbbb`) are the
/// negative row sums of the C and are reconstructed at stamp time.
#[derive(Debug, Clone, Copy, Default)]
pub struct Bsim4v8ChargeMatrix {
    pub gcggb: Value,
    pub gcgdb: Value,
    pub gcgsb: Value,
    pub gcgmgmb: Value,
    pub gcgmdb: Value,
    pub gcgmsb: Value,
    pub gcgmbb: Value,
    pub gcdgb: Value,
    pub gcdgmb: Value,
    pub gcddb: Value,
    pub gcdsb: Value,
    pub gcsgb: Value,
    pub gcsgmb: Value,
    pub gcsdb: Value,
    pub gcssb: Value,
    pub gcbgb: Value,
    pub gcbgmb: Value,
    pub gcbdb: Value,
    pub gcbsb: Value,
}

#[derive(Debug, Clone, Copy, Default)]
struct Bsim4v8RdsBranch {
    gstot: Value,
    gstotd: Value,
    gstotg: Value,
    gstots: Value,
    gstotb: Value,
    gdtot: Value,
    gdtotd: Value,
    gdtotg: Value,
    gdtots: Value,
    gdtotb: Value,
}

/// One BSIM4 v4.8 instance wired into the engine.
#[derive(Debug, Clone)]
pub struct Bsim4v8Device {
    pub name: String,
    /// External drain terminal.
    pub node_drain_external: NodeId,
    /// Drain prime (the internal node when an RSH*NRD resistor exists).
    pub node_drain: NodeId,
    /// External gate terminal.
    pub node_gate_external: NodeId,
    /// Middle gate electrode node for `rgateMod=3`; otherwise aliases the external gate.
    pub node_gate_mid: NodeId,
    /// Intrinsic gate-prime node.
    pub node_gate: NodeId,
    /// External source terminal.
    pub node_source_external: NodeId,
    /// Source prime (the internal node when an RSH*NRS resistor exists).
    pub node_source: NodeId,
    /// External substrate/body terminal.
    pub node_bulk_external: NodeId,
    /// Body prime: channel-body node for `rbodyMod=1/2`; otherwise the external body.
    pub node_bulk: NodeId,
    /// Drain-side body junction node for `rbodyMod=1/2`.
    pub node_drain_body: NodeId,
    /// Source-side body junction node for `rbodyMod=1/2`.
    pub node_source_body: NodeId,
    /// Hidden charge-deficit state node for `trnqsMod=1`.
    pub node_charge_deficit: NodeId,
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
    junction_bias: Bsim4v8JunctionBias,
    converged_junction_ref: Bsim4v8JunctionBias,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        node_drain_external: NodeId,
        node_drain: NodeId,
        node_gate_external: NodeId,
        node_gate_mid: NodeId,
        node_gate: NodeId,
        node_source_external: NodeId,
        node_source: NodeId,
        node_bulk_external: NodeId,
        node_bulk: NodeId,
        node_drain_body: NodeId,
        node_source_body: NodeId,
        node_charge_deficit: NodeId,
        multiplier: Value,
        core: Bsim4v8,
    ) -> Self {
        Self {
            name,
            node_drain_external,
            node_drain,
            node_gate_external,
            node_gate_mid,
            node_gate,
            node_source_external,
            node_source,
            node_bulk_external,
            node_bulk,
            node_drain_body,
            node_source_body,
            node_charge_deficit,
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
            junction_bias: Bsim4v8JunctionBias { vbs: 0.0, vbd: 0.0 },
            converged_junction_ref: Bsim4v8JunctionBias { vbs: 0.0, vbd: 0.0 },
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
        let vd = Self::node_voltage(v, self.node_drain);
        let vg = Self::node_voltage(v, self.node_gate);
        let vs = Self::node_voltage(v, self.node_source);
        let vb = Self::node_voltage(v, self.node_bulk);
        let mt = self.core.mtype;
        Bsim4v8Bias {
            vds: mt * (vd - vs),
            vgs: mt * (vg - vs),
            vbs: mt * (vb - vs),
        }
    }

    pub fn rbody_enabled(&self) -> bool {
        self.core.model.rbody_mod != 0
    }

    pub fn uses_ac_nqs(&self) -> bool {
        self.core.model.acnqs_mod != 0
    }

    pub fn uses_trnqs(&self) -> bool {
        self.core.model.trnqs_mod != 0
    }

    fn raw_junction_bias(&self, v: &[Value]) -> Option<Bsim4v8JunctionBias> {
        if !self.rbody_enabled() {
            return None;
        }
        let vd = Self::node_voltage(v, self.node_drain);
        let vs = Self::node_voltage(v, self.node_source);
        let vdb = Self::node_voltage(v, self.node_drain_body);
        let vsb = Self::node_voltage(v, self.node_source_body);
        let mt = self.core.mtype;
        Some(Bsim4v8JunctionBias {
            vbs: mt * (vsb - vs),
            vbd: mt * (vdb - vd),
        })
    }

    #[inline]
    fn node_voltage(v: &[Value], node: NodeId) -> Value {
        if node == 0 { 0.0 } else { v[node - 1] }
    }

    fn rds_branch_at(&self, voltages: &[Value], bias: Bsim4v8Bias) -> Bsim4v8RdsBranch {
        if self.core.model.rds_mod != 1 {
            return Bsim4v8RdsBranch::default();
        }

        let p = &self.core.size;
        let inst = &self.core.inst;
        let mt = self.core.mtype;
        let vd_ext = Self::node_voltage(voltages, self.node_drain_external);
        let vd_prime = Self::node_voltage(voltages, self.node_drain);
        let vs_ext = Self::node_voltage(voltages, self.node_source_external);
        let vs_prime = Self::node_voltage(voltages, self.node_source);
        let vses = mt * (vs_ext - vs_prime);
        let vdes_minus_vds = mt * (vd_ext - vd_prime);

        let source = external_rds_conductance(
            bias.vgs,
            bias.vbs,
            p.vfbsd,
            p.prwg,
            p.prwb,
            p.rs0,
            p.rswmin,
            inst.source_conductance,
        );
        let drain = external_rds_conductance(
            bias.vgs - bias.vds,
            bias.vbs - bias.vds,
            p.vfbsd,
            p.prwg,
            p.prwb,
            p.rd0,
            p.rdwmin,
            inst.drain_conductance,
        );

        Bsim4v8RdsBranch {
            gstot: source.gtot,
            gstotd: 0.0,
            gstotg: vses * source.dgtot_dvg,
            gstots: -vses * (source.dgtot_dvg + source.dgtot_dvb),
            gstotb: vses * source.dgtot_dvb,
            gdtot: drain.gtot,
            gdtotd: vdes_minus_vds * drain.dgtot_dvd,
            gdtotg: vdes_minus_vds * drain.dgtot_dvg,
            gdtots: 0.0,
            gdtotb: vdes_minus_vds * drain.dgtot_dvb,
        }
    }

    /// The b4ld.c limiting sequence against the previous accepted iterate;
    /// the first iterate of a phase passes through (ngspice seeds CKTstate0
    /// before the first NIiter, the engine seeds from the raw solution).
    fn limited_branch_voltages(
        &self,
        v: &[Value],
    ) -> (Bsim4v8Bias, Option<Bsim4v8JunctionBias>, bool) {
        let raw = self.raw_branch_voltages(v);
        let raw_junction = self.raw_junction_bias(v);
        if !self.limit_anchor_valid.get() {
            return (raw, raw_junction, false);
        }
        let (limited, mut check) = self.core.limit_voltages(raw, self.bias, self.von_prev);
        let Some(raw_junction) = raw_junction else {
            return (limited, None, check);
        };

        let mut check1 = false;
        let mut check2 = false;
        let junction = if limited.vds >= 0.0 {
            let vdbs = pnjlim(
                raw_junction.vbd + raw.vds,
                self.junction_bias.vbd + self.bias.vds,
                common::CONST_VT0,
                self.core.model_temp.vcrit,
                &mut check1,
            );
            let vsbs = pnjlim(
                raw_junction.vbs,
                self.junction_bias.vbs,
                common::CONST_VT0,
                self.core.model_temp.vcrit,
                &mut check2,
            );
            Bsim4v8JunctionBias {
                vbs: vsbs,
                vbd: vdbs - limited.vds,
            }
        } else {
            let vdbd = pnjlim(
                raw_junction.vbd,
                self.junction_bias.vbd,
                common::CONST_VT0,
                self.core.model_temp.vcrit,
                &mut check1,
            );
            let vsbd = pnjlim(
                raw_junction.vbs - raw.vds,
                self.junction_bias.vbs - self.bias.vds,
                common::CONST_VT0,
                self.core.model_temp.vcrit,
                &mut check2,
            );
            Bsim4v8JunctionBias {
                vbs: vsbd + limited.vds,
                vbd: vdbd,
            }
        };
        check = check1 || check2;
        (limited, Some(junction), check)
    }

    /// DC operating point at an already-limited bias. The only error paths
    /// of the module's `eval` are charge-model rejections, which the
    /// builder rules out at construction; the DC-only call cannot fail.
    fn eval_dc(&self, bias: Bsim4v8Bias, junction_bias: Option<Bsim4v8JunctionBias>) -> Bsim4v8Op {
        self.core
            .eval_with_junction_bias(bias, junction_bias, self.gmin, false)
            .expect("BSIM4 DC eval: no charge model requested")
    }

    /// Charge state at the limited bias (the same bias the conductance
    /// stamp uses — mixing raw voltages with limited-bias conductances
    /// would inject companion currents that do not cancel at convergence).
    /// Returns the charge together with the channel mode of that
    /// evaluation, which selects the companion-matrix assembly.
    pub fn charge_at(&self, v: &[Value]) -> (Bsim4v8Charge, i32) {
        let (bias, junction_bias, _) = self.limited_branch_voltages(v);
        let gate_mid_vgs = (self.core.model.rgate_mod == 3).then(|| {
            self.core.mtype
                * (Self::node_voltage(v, self.node_gate_mid)
                    - Self::node_voltage(v, self.node_source))
        });
        let op = self
            .core
            .eval_with_junction_and_gate_mid_bias(
                bias,
                junction_bias,
                gate_mid_vgs,
                self.gmin,
                true,
            )
            .expect("BSIM4 charge eval: charge model validated at build");
        let charge = op
            .charge
            .expect("compute_charges=true yields a charge state");
        (charge, op.mode)
    }

    pub fn trnqs_qdef(&self, voltages: &[Value]) -> Value {
        self.core.mtype * Self::node_voltage(voltages, self.node_charge_deficit)
    }

    pub fn trnqs_qcdump_state(&self, voltages: &[Value]) -> Value {
        self.trnqs_qdef(voltages) * TRNQS_SCALING
    }

    pub fn trnqs_state_charges(
        &self,
        charge: &Bsim4v8Charge,
        voltages: &[Value],
    ) -> (Value, Value, Value, Value, Value, Value) {
        let (bias, _, _) = self.limited_branch_voltages(voltages);
        let vgb = bias.vgs - bias.vbs;
        let qgb = charge.cgbo * vgb;
        let qgate = charge.qgdo + charge.qgso + qgb;
        let qbulk = -qgb;
        let qdrn = -charge.qgdo;
        let qgmid = 0.0;
        let qd = qdrn - charge.qbd;
        let qb = if self.rbody_enabled() {
            qbulk
        } else {
            qbulk + charge.qbd + charge.qbs
        };
        (qgate, qgmid, qb, qd, charge.qbs, charge.qbd)
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

    /// Operating-point snapshot consumed by the noise analysis. The wrapper
    /// owns the cached `b4ld.c` quantities, while the engine constructs the
    /// equivalent current-noise sources so the device math stays decoupled
    /// from the analysis layer.
    pub fn noise_operating_point(&self) -> (&Bsim4v8Op, Bsim4v8Bias) {
        (&self.op, self.bias)
    }

    /// Current external source/drain conductances for BSIM4 noise analysis.
    /// Returns `(gdtot, gstot)` in siemens before the instance multiplier.
    pub fn external_rds_conductances(&self, voltages: &[Value]) -> Option<(Value, Value)> {
        if self.core.model.rds_mod != 1 {
            return None;
        }
        let branch = self.rds_branch_at(voltages, self.bias);
        Some((branch.gdtot, branch.gstot))
    }

    /// Assemble the legacy mode-dependent charge-companion capacitance matrix
    /// with `rgateMod != 3`.
    pub fn charge_matrix(charge: &Bsim4v8Charge, mode: i32) -> Bsim4v8ChargeMatrix {
        Self::charge_matrix_for_rgate(charge, mode, 0)
    }

    /// Assemble the mode-dependent charge-companion capacitance matrix
    /// (b4ld.c:4216-4260 / 4408-4456 with `ag0 = 1`; `rbodyMod = 0` so
    /// `capbd`/`capbs` fold into the prime rows).
    fn charge_matrix_for_rgate(
        charge: &Bsim4v8Charge,
        mode: i32,
        rgate_mod: i32,
    ) -> Bsim4v8ChargeMatrix {
        let c = charge;
        if mode > 0 && rgate_mod == 3 {
            Bsim4v8ChargeMatrix {
                gcggb: c.cggb,
                gcgdb: c.cgdb,
                gcgsb: c.cgsb,
                gcgmgmb: c.cgdo + c.cgso + c.cgbo,
                gcgmdb: -c.cgdo,
                gcgmsb: -c.cgso,
                gcgmbb: -c.cgbo,
                gcdgb: c.cdgb,
                gcdgmb: -c.cgdo,
                gcddb: c.cddb + c.capbd + c.cgdo,
                gcdsb: c.cdsb,
                gcsgb: -(c.cggb + c.cbgb + c.cdgb),
                gcsgmb: -c.cgso,
                gcsdb: -(c.cgdb + c.cbdb + c.cddb),
                gcssb: c.capbs + c.cgso - (c.cgsb + c.cbsb + c.cdsb),
                gcbgb: c.cbgb,
                gcbgmb: -c.cgbo,
                gcbdb: c.cbdb - c.capbd,
                gcbsb: c.cbsb - c.capbs,
            }
        } else if mode > 0 {
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
                ..Default::default()
            }
        } else if rgate_mod == 3 {
            Bsim4v8ChargeMatrix {
                gcggb: c.cggb,
                gcgdb: c.cgsb,
                gcgsb: c.cgdb,
                gcgmgmb: c.cgdo + c.cgso + c.cgbo,
                gcgmdb: -c.cgdo,
                gcgmsb: -c.cgso,
                gcgmbb: -c.cgbo,
                gcdgb: -(c.cggb + c.cbgb + c.cdgb),
                gcdgmb: -c.cgdo,
                gcddb: c.capbd + c.cgdo - (c.cgsb + c.cbsb + c.cdsb),
                gcdsb: -(c.cgdb + c.cbdb + c.cddb),
                gcsgb: c.cdgb,
                gcsgmb: -c.cgso,
                gcsdb: c.cdsb,
                gcssb: c.cddb + c.capbs + c.cgso,
                gcbgb: c.cbgb,
                gcbgmb: -c.cgbo,
                gcbdb: c.cbsb - c.capbd,
                gcbsb: c.cbdb - c.capbs,
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
                ..Default::default()
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
        let gm = self.node_gate_mid;
        let f = factor * self.multiplier;
        if self.core.model.rgate_mod == 3 {
            stamp(matrix, gm, gm, f * gc.gcgmgmb);
            stamp(matrix, gm, dp, f * gc.gcgmdb);
            stamp(matrix, gm, sp, f * gc.gcgmsb);
            stamp(matrix, gm, b, f * gc.gcgmbb);
            stamp(matrix, dp, gm, f * gc.gcdgmb);
            stamp(matrix, sp, gm, f * gc.gcsgmb);
            stamp(matrix, b, gm, f * gc.gcbgmb);
        }
        stamp(matrix, g, g, f * gc.gcggb);
        stamp(matrix, g, b, -f * (gc.gcggb + gc.gcgdb + gc.gcgsb));
        stamp(matrix, g, dp, f * gc.gcgdb);
        stamp(matrix, g, sp, f * gc.gcgsb);
        stamp(matrix, b, g, f * gc.gcbgb);
        stamp(
            matrix,
            b,
            b,
            -f * (gc.gcbgb + gc.gcbgmb + gc.gcbdb + gc.gcbsb),
        );
        stamp(matrix, b, dp, f * gc.gcbdb);
        stamp(matrix, b, sp, f * gc.gcbsb);
        stamp(matrix, dp, g, f * gc.gcdgb);
        stamp(
            matrix,
            dp,
            b,
            -f * (gc.gcdgb + gc.gcdgmb + gc.gcddb + gc.gcdsb),
        );
        stamp(matrix, dp, dp, f * gc.gcddb);
        stamp(matrix, dp, sp, f * gc.gcdsb);
        stamp(matrix, sp, g, f * gc.gcsgb);
        stamp(
            matrix,
            sp,
            b,
            -f * (gc.gcsgb + gc.gcsgmb + gc.gcsdb + gc.gcssb),
        );
        stamp(matrix, sp, dp, f * gc.gcsdb);
        stamp(matrix, sp, sp, f * gc.gcssb);
    }

    /// Stamp the AC imaginary-axis charge block. The canonical companion
    /// matrix folds `capbd`/`capbs` into BP, matching `rbodyMod=0` and the
    /// transient state assembly. In b4acld.c, `rbodyMod=1/2` instead stamps
    /// those two junction capacitances between DP-DB and SP-SB.
    pub fn stamp_ac_charge_matrix(
        &self,
        charge: &Bsim4v8Charge,
        mode: i32,
        omega: Value,
        matrix: &mut impl MatrixStamper,
    ) {
        let gc = Self::charge_matrix_for_rgate(charge, mode, self.core.model.rgate_mod);
        self.stamp_charge_matrix(&gc, omega, matrix);
        if !self.rbody_enabled() {
            return;
        }

        let f = omega * self.multiplier;
        let (dp, sp, bp, db, sb) = (
            self.node_drain,
            self.node_source,
            self.node_bulk,
            self.node_drain_body,
            self.node_source_body,
        );

        let capbd = charge.capbd;
        if capbd != 0.0 {
            stamp(matrix, dp, bp, f * capbd);
            stamp(matrix, bp, dp, f * capbd);
            stamp(matrix, bp, bp, -f * capbd);
            stamp(matrix, dp, db, -f * capbd);
            stamp(matrix, db, dp, -f * capbd);
            stamp(matrix, db, db, f * capbd);
        }

        let capbs = charge.capbs;
        if capbs != 0.0 {
            stamp(matrix, sp, bp, f * capbs);
            stamp(matrix, bp, sp, f * capbs);
            stamp(matrix, bp, bp, -f * capbs);
            stamp(matrix, sp, sb, -f * capbs);
            stamp(matrix, sb, sp, -f * capbs);
            stamp(matrix, sb, sb, f * capbs);
        }
    }

    /// AC-only charge-deficit NQS correction of ngspice-46 `b4acld.c`.
    /// The AC engine has already stamped the QS real Jacobian and QS
    /// `j*omega*C`; this adds the delta that converts the intrinsic
    /// channel/charge rows to `ACNQSMOD=1` for the supported gate,
    /// source/drain-resistance, and body-network topologies.
    pub fn stamp_ac_nqs_correction(
        &self,
        charge: &Bsim4v8Charge,
        mode: i32,
        omega: Value,
        mut stamp: impl FnMut(NodeId, NodeId, Complex64),
    ) {
        if !self.uses_ac_nqs()
            || !matches!(self.core.model.rgate_mod, 0..=3)
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
        let rgate3 = self.core.model.rgate_mod == 3;

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
            let (xcggbr, xcgdbr, xcgsbr, xcdgbr, xcsgbr, xcbgb, xcdgmb, xcsgmb, xcbgmb) = if rgate3
            {
                (
                    cggr * omega,
                    cgdr * omega,
                    cgsr * omega,
                    cdgr * omega,
                    csgr * omega,
                    c.cbgb * omega,
                    -c.cgdo * omega,
                    -c.cgso * omega,
                    -c.cgbo * omega,
                )
            } else {
                (
                    (cggr + c.cgdo + c.cgso + c.cgbo) * omega,
                    (cgdr - c.cgdo) * omega,
                    (cgsr - c.cgso) * omega,
                    (cdgr - c.cgdo) * omega,
                    (csgr - c.cgso) * omega,
                    (c.cbgb - c.cgbo) * omega,
                    0.0,
                    0.0,
                    0.0,
                )
            };
            let xcgbbr = -(xcggbr + xcgdbr + xcgsbr);
            let xcddbr = (cddr + c.capbd + c.cgdo) * omega;
            let xcdsbr = cdsr * omega;
            let xcdbbr = -(xcdgbr + xcddbr + xcdsbr + xcdgmb);
            let xcsdbr = csdr * omega;
            let xcssbr = (c.capbs + c.cgso + cssr) * omega;
            let xcsbbr = -(xcsgbr + xcsdbr + xcssbr + xcsgmb);
            let xcbdb = (c.cbdb - c.capbd) * omega;
            let xcbsb = (c.cbsb - c.capbs) * omega;
            let xcbbb = -(xcbgb + xcbdb + xcbsb + xcbgmb);
            (
                xcggbr, xcgdbr, xcgsbr, xcgbbr, xcdgbr, xcddbr, xcdsbr, xcdbbr, xcsgbr, xcsdbr,
                xcssbr, xcsbbr, xcbgb, xcbdb, xcbsb, xcbbb, cggi, cgdi, cgsi, cgbi, cdgi, cddi,
                cdsi, cdbi, csgi, csdi, cssi, csbi,
            )
        } else {
            let (xcggbr, xcgdbr, xcgsbr, xcdgbr, xcsgbr, xcbgb, xcdgmb, xcsgmb, xcbgmb) = if rgate3
            {
                (
                    cggr * omega,
                    cgsr * omega,
                    cgdr * omega,
                    csgr * omega,
                    cdgr * omega,
                    c.cbgb * omega,
                    -c.cgdo * omega,
                    -c.cgso * omega,
                    -c.cgbo * omega,
                )
            } else {
                (
                    (cggr + c.cgdo + c.cgso + c.cgbo) * omega,
                    (cgsr - c.cgdo) * omega,
                    (cgdr - c.cgso) * omega,
                    (csgr - c.cgdo) * omega,
                    (cdgr - c.cgso) * omega,
                    (c.cbgb - c.cgbo) * omega,
                    0.0,
                    0.0,
                    0.0,
                )
            };
            let xcgbbr = -(xcggbr + xcgdbr + xcgsbr);
            let xcddbr = (c.capbd + c.cgdo + cssr) * omega;
            let xcdsbr = csdr * omega;
            let xcdbbr = -(xcdgbr + xcddbr + xcdsbr + xcdgmb);
            let xcsdbr = cdsr * omega;
            let xcssbr = (cddr + c.capbs + c.cgso) * omega;
            let xcsbbr = -(xcsgbr + xcsdbr + xcssbr + xcsgmb);
            let xcbdb = (c.cbsb - c.capbd) * omega;
            let xcbsb = (c.cbdb - c.capbs) * omega;
            let xcbbb = -(xcbgb + xcbdb + xcbsb + xcbgmb);
            (
                xcggbr, xcgdbr, xcgsbr, xcgbbr, xcdgbr, xcddbr, xcdsbr, xcdbbr, xcsgbr, xcsdbr,
                xcssbr, xcsbbr, xcbgb, xcbdb, xcbsb, xcbbb, cggi, cgsi, cgdi, cgbi, csgi, cssi,
                csdi, csbi, cdgi, cdsi, cddi, cdbi,
            )
        };

        let qs = Self::charge_matrix_for_rgate(charge, mode, self.core.model.rgate_mod);
        let qs_gp_bp = -(qs.gcggb + qs.gcgdb + qs.gcgsb) * omega;
        let qs_dp_bp = -(qs.gcdgb + qs.gcdgmb + qs.gcddb + qs.gcdsb) * omega;
        let qs_sp_bp = -(qs.gcsgb + qs.gcsgmb + qs.gcsdb + qs.gcssb) * omega;
        let qs_bp_bp = -(qs.gcbgb + qs.gcbgmb + qs.gcbdb + qs.gcbsb) * omega;
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

    /// AC-only isolation for the transient NQS charge-deficit node.
    ///
    /// ngspice-46 `b4acld.c` stamps a unit diagonal on the TRNQS q node and
    /// leaves all q-terminal AC couplings at zero: the node is not part of AC
    /// NQS physics, but it must not be singular when the model also enables
    /// `trnqsMod=1`. RSpice AC starts from the DC Jacobian, which already
    /// contributes the tiny DC q-node anchor, so stamp the delta to reach the
    /// ngspice unit diagonal.
    pub fn stamp_trnqs_ac_charge_node_anchor_delta(
        &self,
        mut stamp: impl FnMut(NodeId, NodeId, Complex64),
    ) {
        if !self.uses_trnqs() || self.node_charge_deficit == 0 {
            return;
        }

        let existing_dc_anchor = self.multiplier * self.gmin.max(1.0e-12);
        let target_anchor = self.multiplier;
        let delta = target_anchor - existing_dc_anchor;
        if delta != 0.0 && delta.is_finite() {
            stamp(
                self.node_charge_deficit,
                self.node_charge_deficit,
                Complex64::new(delta, 0.0),
            );
        }
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

    /// Stamp the transient charge companion: `gc** * ag0` plus equivalent
    /// charge currents `ceqq*` (b4ld.c:4696-4700, RHS rows of the canonical
    /// load). The `cq*` values are integrated currents for the composite
    /// `qg`/`qb`/`qd` states (junction charges folded in, b4ld.c:4582-4596).
    #[allow(clippy::too_many_arguments)]
    pub fn stamp_charge_companion(
        &self,
        charge: &Bsim4v8Charge,
        mode: i32,
        ag0: Value,
        cqg: Value,
        cqgmid: Value,
        cqb: Value,
        cqd: Value,
        cqbs: Value,
        cqbd: Value,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let (bias, junction_bias, _) = self.limited_branch_voltages(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;
        let vgmb = self.core.mtype
            * (Self::node_voltage(voltages, self.node_gate_mid)
                - Self::node_voltage(voltages, self.node_bulk));
        let (vbs_jct, vbd_jct) = junction_bias.map(|j| (j.vbs, j.vbd)).unwrap_or((vbs, vbd));

        let gc = Self::charge_matrix_for_rgate(charge, mode, self.core.model.rgate_mod);
        let g = |c: Value| c * ag0;

        // ceqq* linearization corrections (b4ld.c:4696-4700; gcdgmb/gcdbdb
        // are zero in the canonical mode), then the type<0 sign flip
        // (b4ld.c:4963-4966).
        let mut ceqqg = cqg - g(gc.gcggb) * vgb + g(gc.gcgdb) * vbd + g(gc.gcgsb) * vbs;
        let mut ceqqb = cqb - g(gc.gcbgb) * vgb + g(gc.gcbdb) * vbd + g(gc.gcbsb) * vbs;
        let mut ceqqd =
            cqd - g(gc.gcdgb) * vgb - g(gc.gcdgmb) * vgmb + g(gc.gcddb) * vbd + g(gc.gcdsb) * vbs;
        if self.core.model.rgate_mod == 3 {
            ceqqb -= g(gc.gcbgmb) * vgmb;
        }
        let mut ceqqgmid = if self.core.model.rgate_mod == 3 {
            cqgmid + g(gc.gcgmdb) * vbd + g(gc.gcgmsb) * vbs - g(gc.gcgmgmb) * vgmb
        } else {
            0.0
        };
        let (mut ceqqjs, mut ceqqjd) = (0.0, 0.0);
        if self.rbody_enabled() {
            ceqqd += g(charge.capbd) * (vbd_jct - vbd);
            ceqqb += g(charge.capbd) * vbd + g(charge.capbs) * vbs;
            ceqqjs = cqbs - g(charge.capbs) * vbs_jct;
            ceqqjd = cqbd - g(charge.capbd) * vbd_jct;
        }
        if self.core.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqb = -ceqqb;
            ceqqd = -ceqqd;
            ceqqgmid = -ceqqgmid;
            ceqqjs = -ceqqjs;
            ceqqjd = -ceqqjd;
        }

        // RHS charge rows (b4ld.c:5023-5040, the mult_q parts).
        let m = self.multiplier;
        stamp_rhs(matrix, self.node_gate, -m * ceqqg);
        if self.core.model.rgate_mod == 3 {
            stamp_rhs(matrix, self.node_gate_mid, -m * ceqqgmid);
        }
        stamp_rhs(matrix, self.node_drain, -m * ceqqd);
        if self.rbody_enabled() {
            stamp_rhs(matrix, self.node_drain_body, -m * ceqqjd);
            stamp_rhs(matrix, self.node_bulk, -m * ceqqb);
            stamp_rhs(matrix, self.node_source_body, -m * ceqqjs);
            stamp_rhs(
                matrix,
                self.node_source,
                m * (ceqqg + ceqqgmid + ceqqb + ceqqd + ceqqjd + ceqqjs),
            );
        } else {
            stamp_rhs(matrix, self.node_bulk, -m * ceqqb);
            stamp_rhs(
                matrix,
                self.node_source,
                m * (ceqqg + ceqqgmid + ceqqb + ceqqd),
            );
        }

        self.stamp_charge_matrix(&gc, ag0, matrix);
        if self.rbody_enabled() {
            self.stamp_rbody_junction_charge_adjustments(charge, ag0, matrix);
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn stamp_trnqs_charge_companion(
        &self,
        charge: &Bsim4v8Charge,
        mode: i32,
        ag0: Value,
        cqg: Value,
        cqb: Value,
        cqd: Value,
        cqbs: Value,
        cqbd: Value,
        cqcheq: Value,
        cqcdump: Value,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let cox_wl = charge.cox_wl;
        if !(cox_wl > 0.0 && cox_wl.is_finite()) {
            return;
        }
        let gtau = if charge.taunet > 0.0 && charge.taunet.is_finite() {
            TRNQS_SCALING / charge.taunet
        } else if charge.gcrg > 0.0 {
            charge.gcrg / cox_wl * TRNQS_SCALING
        } else {
            0.0
        };
        if !(gtau.is_finite() && gtau >= 0.0) {
            return;
        }

        let (bias, junction_bias, _) = self.limited_branch_voltages(voltages);
        let vgb = bias.vgs - bias.vbs;
        let vbd = bias.vbs - bias.vds;
        let vbs = bias.vbs;
        let (vbs_jct, vbd_jct) = junction_bias.map(|j| (j.vbs, j.vbd)).unwrap_or((vbs, vbd));
        let qdef = self.trnqs_qdef(voltages);
        let t0 = qdef * TRNQS_SCALING / cox_wl;

        let cqgb = -(charge.cggb + charge.cbgb);
        let cqdb = -(charge.cgdb + charge.cbdb);
        let cqsb = -(charge.cgsb + charge.cbsb);
        let cqbb = -(cqgb + cqdb + cqsb);

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
            let ggtg = t0 * charge.gcrgg;
            let ggtd = t0 * charge.gcrgd;
            let ggts = t0 * charge.gcrgs;
            let ggtb = t0 * charge.gcrgb;
            let gcqgb = cqgb * ag0;
            let gcqdb = cqdb * ag0;
            let gcqsb = cqsb * ag0;
            let gcqbb = cqbb * ag0;
            let (dxpart, ddxpart_dvd, ddxpart_dvg, ddxpart_dvs, ddxpart_dvb) =
                if charge.qchqs.abs() <= 1.0e-5 * cox_wl {
                    let dxpart = if self.core.model.xpart < 0.5 {
                        0.4
                    } else if self.core.model.xpart > 0.5 {
                        0.0
                    } else {
                        0.5
                    };
                    (dxpart, 0.0, 0.0, 0.0, 0.0)
                } else {
                    let dxpart = charge.qdrn / charge.qchqs;
                    let cdd = charge.cddb;
                    let csd = -(charge.cgdb + charge.cddb + charge.cbdb);
                    let ddxpart_dvd = (cdd - dxpart * (cdd + csd)) / charge.qchqs;
                    let cdg = charge.cdgb;
                    let csg = -(charge.cggb + charge.cdgb + charge.cbgb);
                    let ddxpart_dvg = (cdg - dxpart * (cdg + csg)) / charge.qchqs;
                    let cds = charge.cdsb;
                    let css = -(charge.cgsb + charge.cdsb + charge.cbsb);
                    let ddxpart_dvs = (cds - dxpart * (cds + css)) / charge.qchqs;
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
            let ggtg = t0 * charge.gcrgg;
            let ggts = t0 * charge.gcrgd;
            let ggtd = t0 * charge.gcrgs;
            let ggtb = t0 * charge.gcrgb;
            let gcqgb = cqgb * ag0;
            let gcqdb = cqsb * ag0;
            let gcqsb = cqdb * ag0;
            let gcqbb = cqbb * ag0;
            let (sxpart, dsxpart_dvd, dsxpart_dvg, dsxpart_dvs, dsxpart_dvb) =
                if charge.qchqs.abs() <= 1.0e-5 * cox_wl {
                    let sxpart = if self.core.model.xpart < 0.5 {
                        0.4
                    } else if self.core.model.xpart > 0.5 {
                        0.0
                    } else {
                        0.5
                    };
                    (sxpart, 0.0, 0.0, 0.0, 0.0)
                } else {
                    let sxpart = charge.qdrn / charge.qchqs;
                    let css = charge.cddb;
                    let cds = -(charge.cgdb + charge.cddb + charge.cbdb);
                    let dsxpart_dvs = (css - sxpart * (css + cds)) / charge.qchqs;
                    let csg = charge.cdgb;
                    let cdg = -(charge.cggb + charge.cdgb + charge.cbgb);
                    let dsxpart_dvg = (csg - sxpart * (csg + cdg)) / charge.qchqs;
                    let csd = charge.cdsb;
                    let cdd = -(charge.cgsb + charge.cdsb + charge.cbsb);
                    let dsxpart_dvd = (csd - sxpart * (csd + cdd)) / charge.qchqs;
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
        let gcgbb = -charge.cgbo * ag0;
        let gcdgb = gcgdb;
        let gcsgb = gcgsb;
        let gcbgb = gcgbb;
        let gcddb = (charge.capbd + charge.cgdo) * ag0;
        let gcdsb = 0.0;
        let gcsdb = 0.0;
        let gcssb = (charge.capbs + charge.cgso) * ag0;
        let gcdbb = -(gcdgb + gcddb);
        let gcsbb = -(gcsgb + gcssb);
        let gcbdb = -charge.capbd * ag0;
        let gcbsb = -charge.capbs * ag0;
        let gcbbb = -(gcbdb + gcbgb + gcbsb);

        let nqs_terminal = ggtg * vgb - ggtd * vbd - ggts * vbs;
        let t1 = qdef * gtau;
        let mut ceqqg = cqg - gcggb * vgb + gcgdb * vbd + gcgsb * vbs + nqs_terminal;
        let mut ceqqd = cqd - gcdgb * vgb + gcddb * vbd + gcdsb * vbs
            - dxpart * nqs_terminal
            - t1 * (ddxpart_dvg * vgb - ddxpart_dvd * vbd - ddxpart_dvs * vbs);
        let mut ceqqb = cqb - gcbgb * vgb + gcbdb * vbd + gcbsb * vbs;
        let (mut ceqqjs, mut ceqqjd) = (0.0, 0.0);
        if self.rbody_enabled() {
            ceqqd += charge.capbd * ag0 * (vbd_jct - vbd);
            ceqqb += charge.capbd * ag0 * vbd + charge.capbs * ag0 * vbs;
            ceqqjs = cqbs - charge.capbs * ag0 * vbs_jct;
            ceqqjd = cqbd - charge.capbd * ag0 * vbd_jct;
        }
        let gqdef = TRNQS_SCALING * ag0;
        let mut cqdef = cqcdump - gqdef * qdef;
        let mut cqcheq_eq = cqcheq - (gcqgb * vgb - gcqdb * vbd - gcqsb * vbs) + nqs_terminal;
        if self.core.mtype < 0.0 {
            ceqqg = -ceqqg;
            ceqqd = -ceqqd;
            ceqqb = -ceqqb;
            ceqqjs = -ceqqjs;
            ceqqjd = -ceqqjd;
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
        stamp_rhs(matrix, dp, -m * ceqqd);
        if self.rbody_enabled() {
            stamp_rhs(matrix, self.node_drain_body, -m * ceqqjd);
            stamp_rhs(matrix, b, -m * ceqqb);
            stamp_rhs(matrix, self.node_source_body, -m * ceqqjs);
            stamp_rhs(matrix, sp, m * (ceqqg + ceqqb + ceqqd + ceqqjd + ceqqjs));
        } else {
            stamp_rhs(matrix, b, -m * ceqqb);
            stamp_rhs(matrix, sp, m * (ceqqg + ceqqb + ceqqd));
        }
        stamp_rhs(matrix, q, m * (cqcheq_eq - cqdef));

        stamp(matrix, g, g, m * (gcggb - ggtg));
        stamp(matrix, g, dp, m * (gcgdb - ggtd));
        stamp(matrix, g, sp, m * (gcgsb - ggts));
        stamp(matrix, g, b, m * (gcgbb - ggtb));

        stamp(
            matrix,
            dp,
            dp,
            m * (t1 * ddxpart_dvd + gcddb + dxpart * ggtd),
        );
        stamp(
            matrix,
            dp,
            g,
            m * (dxpart * ggtg + t1 * ddxpart_dvg + gcdgb),
        );
        stamp(
            matrix,
            dp,
            sp,
            m * (dxpart * ggts + t1 * ddxpart_dvs + gcdsb),
        );
        stamp(
            matrix,
            dp,
            b,
            m * (dxpart * ggtb + gcdbb + t1 * ddxpart_dvb),
        );

        stamp(
            matrix,
            sp,
            dp,
            m * (t1 * dsxpart_dvd + sxpart * ggtd + gcsdb),
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
            sp,
            m * (sxpart * ggts + t1 * dsxpart_dvs + gcssb),
        );
        stamp(
            matrix,
            sp,
            b,
            m * (gcsbb + sxpart * ggtb + t1 * dsxpart_dvb),
        );

        stamp(matrix, b, dp, m * gcbdb);
        stamp(matrix, b, g, m * gcbgb);
        stamp(matrix, b, sp, m * gcbsb);
        stamp(matrix, b, b, m * gcbbb);

        stamp(matrix, q, q, m * (gqdef + gtau));
        stamp(matrix, q, g, m * (ggtg - gcqgb));
        stamp(matrix, q, dp, m * (ggtd - gcqdb));
        stamp(matrix, q, sp, m * (ggts - gcqsb));
        stamp(matrix, q, b, m * (ggtb - gcqbb));
        stamp(matrix, dp, q, m * (dxpart * gtau));
        stamp(matrix, sp, q, m * (sxpart * gtau));
        stamp(matrix, g, q, -m * gtau);
        if self.rbody_enabled() {
            self.stamp_rbody_junction_charge_adjustments(charge, ag0, matrix);
        }
    }

    fn stamp_rbody_junction_charge_adjustments(
        &self,
        charge: &Bsim4v8Charge,
        ag0: Value,
        matrix: &mut impl MatrixStamper,
    ) {
        let f = ag0 * self.multiplier;
        let (dp, sp, bp, db, sb) = (
            self.node_drain,
            self.node_source,
            self.node_bulk,
            self.node_drain_body,
            self.node_source_body,
        );

        let capbd = charge.capbd;
        if capbd != 0.0 {
            stamp(matrix, dp, bp, f * capbd);
            stamp(matrix, bp, dp, f * capbd);
            stamp(matrix, bp, bp, -f * capbd);
            stamp(matrix, dp, db, -f * capbd);
            stamp(matrix, db, dp, -f * capbd);
            stamp(matrix, db, db, f * capbd);
        }

        let capbs = charge.capbs;
        if capbs != 0.0 {
            stamp(matrix, sp, bp, f * capbs);
            stamp(matrix, bp, sp, f * capbs);
            stamp(matrix, bp, bp, -f * capbs);
            stamp(matrix, sp, sb, -f * capbs);
            stamp(matrix, sb, sp, -f * capbs);
            stamp(matrix, sb, sb, f * capbs);
        }
    }

    fn rgate_gcrg_terms(
        &self,
        branch_pos: NodeId,
        voltages: &[Value],
        bias: Bsim4v8Bias,
        op: &Bsim4v8Op,
    ) -> (Value, Value, Value, Value, Value, Value) {
        let vge = self.core.mtype
            * (Self::node_voltage(voltages, branch_pos)
                - Self::node_voltage(voltages, self.node_source));
        Self::rgate_gcrg_terms_from(self.core.mtype, op.mode, vge, bias, op)
    }

    /// Bias-dependent gate-resistance current-equivalent and derivative assembly from
    /// ngspice-46 `b4ld.c:4828-4836`, `4915-4927`, and the PMOS type flip at
    /// `4961-4968`.
    fn rgate_gcrg_terms_from(
        mtype: Value,
        mode: i32,
        vge: Value,
        bias: Bsim4v8Bias,
        op: &Bsim4v8Op,
    ) -> (Value, Value, Value, Value, Value, Value) {
        let delta = vge - bias.vgs;
        let (gcrgd, mut gcrgg, gcrgs, gcrgb, mut ceqgcrg) = if mode >= 0 {
            let gcrgd = op.gcrgd * delta;
            let gcrgg = op.gcrgg * delta;
            let gcrgs = op.gcrgs * delta;
            let gcrgb = op.gcrgb * delta;
            let ceqgcrg = -(gcrgd * bias.vds + gcrgg * bias.vgs + gcrgb * bias.vbs);
            (gcrgd, gcrgg, gcrgs, gcrgb, ceqgcrg)
        } else {
            let vgd = bias.vgs - bias.vds;
            let vbd = bias.vbs - bias.vds;
            let gcrgd = op.gcrgs * delta;
            let gcrgg = op.gcrgg * delta;
            let gcrgs = op.gcrgd * delta;
            let gcrgb = op.gcrgb * delta;
            let ceqgcrg = -(gcrgg * vgd - gcrgs * bias.vds + gcrgb * vbd);
            (gcrgd, gcrgg, gcrgs, gcrgb, ceqgcrg)
        };
        if mtype < 0.0 {
            ceqgcrg = -ceqgcrg;
        }
        gcrgg -= op.gcrg;
        (op.gcrg, gcrgg, gcrgd, gcrgs, gcrgb, ceqgcrg)
    }

    /// Stamp the linearized DC operating point: matrix/RHS load of
    /// b4ld.c:4750-5350 with all charge (`gc**`/`ceqq*`) and NQS (`ggt*`,
    /// `T1`) terms zero. `gdpr`/`gspr` are zero here; `rdsMod=1` contributes
    /// the external `gstot`/`gdtot` rows directly.
    fn stamp_op(
        &self,
        op: &Bsim4v8Op,
        bias: Bsim4v8Bias,
        junction_bias: Option<Bsim4v8JunctionBias>,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        let (d, dp, ge, g, s, sp, b_ext, b, db, sb) = (
            self.node_drain_external,
            self.node_drain,
            self.node_gate_external,
            self.node_gate,
            self.node_source_external,
            self.node_source,
            self.node_bulk_external,
            self.node_bulk,
            self.node_drain_body,
            self.node_source_body,
        );
        let mt = self.core.mtype;
        let m = self.multiplier;
        let rbody = self.rbody_enabled();

        let vgd = bias.vgs - bias.vds;
        let vbd = bias.vbs - bias.vds;
        let rds_branch = self.rds_branch_at(voltages, bias);
        let ceqgstot = mt
            * (rds_branch.gstotd * bias.vds
                + rds_branch.gstotg * bias.vgs
                + rds_branch.gstotb * bias.vbs);
        let gstot = rds_branch.gstot;
        let gstotd = rds_branch.gstotd;
        let gstotg = rds_branch.gstotg;
        let gstots = rds_branch.gstots - gstot;
        let gstotb = rds_branch.gstotb;
        let ceqgdtot = -mt
            * (rds_branch.gdtotd * bias.vds
                + rds_branch.gdtotg * bias.vgs
                + rds_branch.gdtotb * bias.vbs);
        let gdtot = rds_branch.gdtot;
        let gdtotd = rds_branch.gdtotd - gdtot;
        let gdtotg = rds_branch.gdtotg;
        let gdtots = rds_branch.gdtots;
        let gdtotb = rds_branch.gdtotb;

        let (gm, gmbs, fwd_sum, rev_sum, ceqdrn, ceqbd, ceqbs);
        let (gbbdp, gbbsp);
        let (gbdpg, gbdpdp, gbdpb, gbdpsp);
        let (gbspg, gbspdp, gbspb, gbspsp);
        let (gistotg, gistotd, gistots, gistotb, istoteq);
        let (gidtotg, gidtotd, gidtots, gidtotb, idtoteq);
        let (gibtotg, gibtotd, gibtots, gibtotb, ibtoteq);
        let (gigtotg, gigtotd, gigtots, gigtotb, igtoteq);
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
            ceqbs = mt * (op.igisl + op.ggisls * bias.vds - op.ggislg * vgd - op.ggislb * vbd);

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

            gistotg = op.gigsg + op.gigcsg;
            gistotd = op.gigcsd;
            gistots = op.gigss + op.gigcss;
            gistotb = op.gigcsb;
            istoteq = mt
                * (op.igs + op.igcs
                    - gistotg * bias.vgs
                    - op.gigcsd * bias.vds
                    - op.gigcsb * bias.vbs);

            gidtotg = op.gigdg + op.gigcdg;
            gidtotd = op.gigdd + op.gigcdd;
            gidtots = op.gigcds;
            gidtotb = op.gigcdb;
            idtoteq = mt
                * (op.igd + op.igcd
                    - op.gigdg * vgd
                    - op.gigcdg * bias.vgs
                    - op.gigcdd * bias.vds
                    - op.gigcdb * bias.vbs);

            gibtotg = op.gigbg;
            gibtotd = op.gigbd;
            gibtots = op.gigbs;
            gibtotb = op.gigbb;
            ibtoteq =
                mt * (op.igb - op.gigbg * bias.vgs - op.gigbd * bias.vds - op.gigbb * bias.vbs);
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
                * (op.igidl - op.ggidld * bias.vds - op.ggidlg * bias.vgs - op.ggidlb * bias.vbs);

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

            gistotg = op.gigsg + op.gigcdg;
            gistotd = op.gigcds;
            gistots = op.gigss + op.gigcdd;
            gistotb = op.gigcdb;
            istoteq = mt
                * (op.igs + op.igcd - op.gigsg * bias.vgs - op.gigcdg * vgd + op.gigcdd * bias.vds
                    - op.gigcdb * vbd);

            gidtotg = op.gigdg + op.gigcsg;
            gidtotd = op.gigdd + op.gigcss;
            gidtots = op.gigcsd;
            gidtotb = op.gigcsb;
            idtoteq = mt
                * (op.igd + op.igcs - (op.gigdg + op.gigcsg) * vgd + op.gigcsd * bias.vds
                    - op.gigcsb * vbd);

            gibtotg = op.gigbg;
            gibtotd = op.gigbs;
            gibtots = op.gigbd;
            gibtotb = op.gigbb;
            ibtoteq = mt * (op.igb - op.gigbg * vgd + op.gigbd * bias.vds - op.gigbb * vbd);
        }
        gigtotg = gistotg + gidtotg + gibtotg;
        gigtotd = gistotd + gidtotd + gibtotd;
        gigtots = gistots + gidtots + gibtots;
        gigtotb = gistotb + gidtotb + gibtotb;
        igtoteq = istoteq + idtoteq + ibtoteq;

        // Junction equivalent currents; type<0 flips them (b4ld.c:4957-4961;
        // rbodyMod = 0 makes vbs_jct/vbd_jct the channel-side biases).
        let (vbs_jct, vbd_jct) = junction_bias
            .map(|j| (j.vbs, j.vbd))
            .unwrap_or((bias.vbs, vbd));
        let (ceqjs, ceqjd) = if mt > 0.0 {
            (op.cbs - op.gbs * vbs_jct, op.cbd - op.gbd * vbd_jct)
        } else {
            (-(op.cbs - op.gbs * vbs_jct), -(op.cbd - op.gbd * vbd_jct))
        };
        let gjbd = if rbody { 0.0 } else { op.gbd };
        let gjbs = if rbody { 0.0 } else { op.gbs };

        // RHS (b4ld.c:5023-5040, ceqq*/ceqg*tot = 0).
        let rgate_branch_pos = match self.core.model.rgate_mod {
            2 => Some(ge),
            3 => Some(self.node_gate_mid),
            _ => None,
        };
        let rgate_terms =
            rgate_branch_pos.map(|node| self.rgate_gcrg_terms(node, voltages, bias, op));
        stamp_rhs(
            matrix,
            dp,
            m * (ceqjd - ceqbd + ceqgdtot - ceqdrn + idtoteq),
        );
        stamp_rhs(matrix, g, -m * igtoteq);
        if let Some((_, _, _, _, _, ceqgcrg)) = rgate_terms {
            let branch_pos = rgate_branch_pos.expect("rgate terms imply branch node");
            stamp_rhs(matrix, branch_pos, -m * ceqgcrg);
            stamp_rhs(matrix, g, m * ceqgcrg);
        }
        if rbody {
            stamp_rhs(matrix, db, -m * ceqjd);
            stamp_rhs(matrix, b, m * (ceqbd + ceqbs + ibtoteq));
            stamp_rhs(matrix, sb, -m * ceqjs);
        } else {
            stamp_rhs(matrix, b, m * (ceqbd + ceqbs - ceqjd - ceqjs + ibtoteq));
        }
        stamp_rhs(
            matrix,
            sp,
            m * (ceqdrn - ceqbs + ceqjs - ceqgstot + istoteq),
        );
        stamp_rhs(matrix, d, -m * ceqgdtot);
        stamp_rhs(matrix, s, m * ceqgstot);

        // GIDL/GISL conductance shorthands (b4ld.c:5322-5327).
        let ggidl_sum = op.ggidlg + op.ggidld + op.ggidlb;
        let ggisl_sum = op.ggislg + op.ggisls + op.ggislb;

        // Matrix (b4ld.c:5299-5348, gc**/ggt*/T1 = 0, series G
        // external, with the gidl/gisl stamp blocks folded in).
        stamp(matrix, g, g, m * gigtotg);
        stamp(matrix, g, dp, m * gigtotd);
        stamp(matrix, g, sp, m * gigtots);
        stamp(matrix, g, b, m * gigtotb);
        if let Some((gcrg, gcrgg, gcrgd, gcrgs, gcrgb, _ceqgcrg)) = rgate_terms {
            let branch_pos = rgate_branch_pos.expect("rgate terms imply branch node");
            stamp(matrix, branch_pos, branch_pos, m * gcrg);
            stamp(matrix, branch_pos, g, m * gcrgg);
            stamp(matrix, branch_pos, dp, m * gcrgd);
            stamp(matrix, branch_pos, sp, m * gcrgs);
            stamp(matrix, branch_pos, b, m * gcrgb);
            stamp(matrix, g, branch_pos, -m * gcrg);
            stamp(matrix, g, g, -m * gcrgg);
            stamp(matrix, g, dp, -m * gcrgd);
            stamp(matrix, g, sp, -m * gcrgs);
            stamp(matrix, g, b, -m * gcrgb);
        }
        stamp(matrix, d, g, m * gdtotg);
        stamp(matrix, d, sp, m * gdtots);
        stamp(matrix, d, b, m * gdtotb);
        stamp(matrix, d, dp, m * gdtotd);
        stamp(matrix, d, d, m * gdtot);
        stamp(matrix, s, dp, m * gstotd);
        stamp(matrix, s, g, m * gstotg);
        stamp(matrix, s, b, m * gstotb);
        stamp(matrix, s, sp, m * gstots);
        stamp(matrix, s, s, m * gstot);
        stamp(
            matrix,
            dp,
            dp,
            m * (op.gds + gjbd - gdtotd + rev_sum + gbdpdp - gidtotd + op.ggidld),
        );
        stamp(matrix, dp, d, -m * gdtot);
        stamp(
            matrix,
            dp,
            g,
            m * (gm - gdtotg + gbdpg - gidtotg + op.ggidlg),
        );
        stamp(
            matrix,
            dp,
            sp,
            -m * (op.gds + gdtots + fwd_sum - gbdpsp + gidtots + ggidl_sum),
        );
        stamp(
            matrix,
            dp,
            b,
            -m * (gjbd + gdtotb - gmbs - gbdpb + gidtotb - op.ggidlb),
        );
        stamp(
            matrix,
            sp,
            dp,
            -m * (op.gds + gstotd + rev_sum - gbspdp + gistotd + ggisl_sum),
        );
        stamp(
            matrix,
            sp,
            g,
            m * (gbspg - gm - gstotg - gistotg + op.ggislg),
        );
        stamp(
            matrix,
            sp,
            sp,
            m * (op.gds + gjbs + fwd_sum + gbspsp - gistots - gstots + op.ggisls),
        );
        stamp(matrix, sp, s, -m * gstot);
        stamp(
            matrix,
            sp,
            b,
            -m * (gjbs + gstotb + gmbs - gbspb + gistotb - op.ggislb),
        );
        stamp(
            matrix,
            b,
            dp,
            m * (gbbdp - gjbd - gibtotd - op.ggidld + ggisl_sum),
        );
        stamp(
            matrix,
            b,
            g,
            -m * (op.gbgs + gibtotg + op.ggidlg + op.ggislg),
        );
        stamp(
            matrix,
            b,
            sp,
            m * (gbbsp - gjbs - gibtots + ggidl_sum - op.ggisls),
        );
        stamp(
            matrix,
            b,
            b,
            m * (gjbd + gjbs - op.gbbs - gibtotb - op.ggidlb - op.ggislb),
        );
        if rbody {
            stamp(matrix, dp, db, -m * op.gbd);
            stamp(matrix, db, dp, -m * op.gbd);
            stamp(
                matrix,
                db,
                db,
                m * (op.gbd
                    + self.core.inst.body_prime_drain_conductance
                    + self.core.inst.body_drain_bulk_conductance),
            );
            stamp(
                matrix,
                db,
                b,
                -m * self.core.inst.body_prime_drain_conductance,
            );
            stamp(
                matrix,
                db,
                b_ext,
                -m * self.core.inst.body_drain_bulk_conductance,
            );

            stamp(
                matrix,
                b,
                db,
                -m * self.core.inst.body_prime_drain_conductance,
            );
            stamp(
                matrix,
                b,
                b_ext,
                -m * self.core.inst.body_prime_bulk_conductance,
            );
            stamp(
                matrix,
                b,
                sb,
                -m * self.core.inst.body_prime_source_conductance,
            );
            stamp(
                matrix,
                b,
                b,
                m * (self.core.inst.body_prime_drain_conductance
                    + self.core.inst.body_prime_source_conductance
                    + self.core.inst.body_prime_bulk_conductance),
            );

            stamp(matrix, sp, sb, -m * op.gbs);
            stamp(matrix, sb, sp, -m * op.gbs);
            stamp(
                matrix,
                sb,
                b,
                -m * self.core.inst.body_prime_source_conductance,
            );
            stamp(
                matrix,
                sb,
                b_ext,
                -m * self.core.inst.body_source_bulk_conductance,
            );
            stamp(
                matrix,
                sb,
                sb,
                m * (op.gbs
                    + self.core.inst.body_prime_source_conductance
                    + self.core.inst.body_source_bulk_conductance),
            );

            stamp(
                matrix,
                b_ext,
                db,
                -m * self.core.inst.body_drain_bulk_conductance,
            );
            stamp(
                matrix,
                b_ext,
                b,
                -m * self.core.inst.body_prime_bulk_conductance,
            );
            stamp(
                matrix,
                b_ext,
                sb,
                -m * self.core.inst.body_source_bulk_conductance,
            );
            stamp(
                matrix,
                b_ext,
                b_ext,
                m * (self.core.inst.body_source_bulk_conductance
                    + self.core.inst.body_drain_bulk_conductance
                    + self.core.inst.body_prime_bulk_conductance),
            );
        }
        if self.uses_trnqs() {
            stamp(
                matrix,
                self.node_charge_deficit,
                self.node_charge_deficit,
                m * self.gmin.max(1.0e-12),
            );
        }
    }
}

impl NonlinearDevice for Bsim4v8Device {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        self.converged_junction_ref = self.junction_bias;
        let (bias, junction_bias, check) = self.limited_branch_voltages(voltages);
        self.last_limited.set(check);
        self.bias = bias;
        if let Some(junction_bias) = junction_bias {
            self.junction_bias = junction_bias;
        }
        self.op = self.eval_dc(bias, junction_bias);
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
        let (bias, junction_bias, _) = self.limited_branch_voltages(voltages);
        if bias == self.bias && (!self.rbody_enabled() || junction_bias == Some(self.junction_bias))
        {
            self.stamp_op(&self.op, bias, junction_bias, voltages, matrix);
        } else {
            // A probe at a different solution than the last `update`:
            // re-linearize so the stamp and its equivalent currents agree.
            let op = self.eval_dc(bias, junction_bias);
            self.stamp_op(&op, bias, junction_bias, voltages, matrix);
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
        let cmp =
            |new: Value, old: Value| (new - old).abs() < reltol * new.abs().max(old.abs()) + vtol;
        cmp(self.bias.vds, self.converged_ref.vds)
            && cmp(self.bias.vgs, self.converged_ref.vgs)
            && cmp(self.bias.vbs, self.converged_ref.vbs)
            && (!self.rbody_enabled()
                || (cmp(self.junction_bias.vbs, self.converged_junction_ref.vbs)
                    && cmp(self.junction_bias.vbd, self.converged_junction_ref.vbd)))
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ExternalRdsConductance {
    gtot: Value,
    dgtot_dvd: Value,
    dgtot_dvg: Value,
    dgtot_dvb: Value,
}

#[allow(clippy::too_many_arguments)]
fn external_rds_conductance(
    vg: Value,
    vb: Value,
    vfbsd: Value,
    prwg: Value,
    prwb: Value,
    r0: Value,
    rwmin: Value,
    conductance: Value,
) -> ExternalRdsConductance {
    if conductance <= 0.0 {
        return ExternalRdsConductance::default();
    }

    let t0 = vg - vfbsd;
    let t1 = (t0 * t0 + 1.0e-4).sqrt();
    let vg_eff = 0.5 * (t0 + t1);
    let dvg_eff_dvg = vg_eff / t1;

    let t0 = 1.0 + prwg * vg_eff;
    let dt0_dvg = -prwg / t0 / t0 * dvg_eff_dvg;
    let t1 = -prwb * vb;
    let dt1_dvb = -prwb;

    let t2 = 1.0 / t0 + t1;
    let t3 = t2 + (t2 * t2 + 0.01).sqrt();
    let dt3 = t3 / (t3 - t2);
    let dt3_dvg = dt3 * dt0_dvg;
    let dt3_dvb = dt3 * dt1_dvb;

    let t4 = r0 * 0.5;
    let resistance = rwmin + t3 * t4;
    let dresistance_dvg = t4 * dt3_dvg;
    let dresistance_dvb = t4 * dt3_dvb;

    let denom = 1.0 + conductance * resistance;
    let gtot = conductance / denom;
    let dgtot = -gtot * gtot;
    let dgtot_dvg = dgtot * dresistance_dvg;
    let dgtot_dvb = dgtot * dresistance_dvb;
    let dgtot_dvd = -(dgtot_dvg + dgtot_dvb);
    ExternalRdsConductance {
        gtot,
        dgtot_dvd,
        dgtot_dvg,
        dgtot_dvb,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn rgate_op() -> Bsim4v8Op {
        Bsim4v8Op {
            mode: 1,
            gcrg: 11.0,
            gcrgg: 5.0,
            gcrgd: 3.0,
            gcrgs: -15.0,
            gcrgb: 7.0,
            ..Bsim4v8Op::default()
        }
    }

    #[test]
    fn rgate_mod2_terms_flip_ceqgcrg_for_pmos() {
        let bias = Bsim4v8Bias {
            vds: 0.4,
            vgs: 0.8,
            vbs: -0.1,
        };
        let op = rgate_op();

        let (_, gcrgg, gcrgd, gcrgs, gcrgb, ceqgcrg) =
            Bsim4v8Device::rgate_gcrg_terms_from(-1.0, 1, 1.0, bias, &op);

        assert!((gcrgd - 0.6).abs() < 1e-14);
        assert!((gcrgg + 10.0).abs() < 1e-14);
        assert!((gcrgs + 3.0).abs() < 1e-14);
        assert!((gcrgb - 1.4).abs() < 1e-14);
        assert!((ceqgcrg - 0.9).abs() < 1e-14);
    }

    #[test]
    fn rgate_mod2_terms_swap_drain_source_derivatives_in_reverse_mode() {
        let bias = Bsim4v8Bias {
            vds: -0.3,
            vgs: 0.7,
            vbs: -0.2,
        };
        let mut op = rgate_op();
        op.mode = -1;

        let (_, gcrgg, gcrgd, gcrgs, gcrgb, ceqgcrg) =
            Bsim4v8Device::rgate_gcrg_terms_from(1.0, -1, 1.1, bias, &op);

        assert!((gcrgd + 6.0).abs() < 1e-14);
        assert!((gcrgg + 9.0).abs() < 1e-14);
        assert!((gcrgs - 1.2).abs() < 1e-14);
        assert!((gcrgb - 2.8).abs() < 1e-14);
        assert!((ceqgcrg + 2.64).abs() < 1e-14);
    }
}
