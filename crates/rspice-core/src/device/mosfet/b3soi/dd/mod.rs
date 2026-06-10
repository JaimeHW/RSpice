//! B3SOIDD (BSIMSOI dynamic-depletion, MOS level 56) device.
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim3soi_dd/`. This module ties
//! the model card ([`params`]), the size/temperature setup ([`temp`]), and the
//! DC load equations ([`eval`]) into a [`NonlinearDevice`] that RSpice can stamp.
//!
//! # Node topology (b3soiddset.c:975-1037)
//!
//! External terminals are drain `d`, gate `g`, source `s`, back-gate/substrate
//! `e` (substrate under the buried oxide), and an optional body contact `p`.
//! The device additionally owns an internal body node `B` and (in the full
//! model) optional drain/source primes and a self-heating temperature node.
//!
//! For the supported decks (`rbody = rbsh = 0`, `shmod = 0`, no series R):
//! - **Floating body** (`d g s e`): ngspice creates an internal `Body` node,
//!   sets `bodyMod = 0`, `float = 1`. RSpice allocates one internal node.
//! - **Ideal body tie** (`d g s e p`): ngspice sets `bodyMod = 2` and the
//!   external `p` node *is* the body node; no internal node is created.
//!
//! The drain/source primes coincide with the external drain/source (no series
//! resistance in the supported decks), and the temperature node does not exist
//! (`shmod = 0`).
//!
//! # Status
//!
//! The DC current path ([`eval::eval_dc`]) is a faithful transcription of the
//! `B3SOIDDload` DC block. The **charge model (CAPMOD=3)** and the resulting
//! transient capacitor companion stamping are **not yet implemented**; until
//! they are, this device is intentionally *not wired into the builder dispatch*
//! (see `engine/builder.rs`) so non-SOI paths remain byte-identical and the
//! existing regression suite cannot regress. `eval_dc` and the DC stamping are
//! exercised directly by the unit tests in this module.

pub use super::common;
pub use params::B3SoiDdModel;

pub mod eval;
pub mod params;
pub mod temp;

use crate::device::traits::{
    MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice,
};
use crate::{Value, circuit::NodeId};
use eval::{B3SoiDdBias, B3SoiDdOp, ModelConsts};
use std::sync::Arc;
use temp::{B3SoiDdGeometry, B3SoiDdSized};

/// Body-node configuration for one instance (b3soiddset.c node creation).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BodyMode {
    /// Floating body: `B` is an internal node, `float = 1` (bodyMod 0).
    Floating,
    /// Ideal body tie: the external contact node is the body node (bodyMod 2).
    TiedIdeal,
}

/// B3SOIDD device instance.
#[derive(Debug, Clone)]
pub struct B3SoiDd {
    pub name: String,
    /// +1 NMOS / -1 PMOS (model `mtype`).
    pub mtype: Value,

    // External / internal nodes (already resolved to NodeId by the builder).
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    /// Back-gate / substrate-under-BOX node `E`.
    pub node_e: NodeId,
    /// Body node: internal (floating) or the external contact (tied).
    pub node_body: NodeId,
    /// Body-contact node `P` (== `node_body` when tied; unused when floating).
    pub node_p: NodeId,

    pub body_mode: BodyMode,

    /// Shared model card (one per `.model`).
    pub model: Arc<B3SoiDdModel>,
    /// Size/temperature-resolved parameters (one per instance geometry).
    pub sized: Arc<B3SoiDdSized>,
    /// Model scalars needed inside the load.
    consts: ModelConsts,

    // Operating point (current iteration).
    op: B3SoiDdOp,
    // Branch voltages used at the last `update`, device polarity, pre-swap.
    bias: B3SoiDdBias,
    converged_ref: B3SoiDdBias,
    has_history: bool,
}

impl B3SoiDd {
    /// Build an instance from a model card and instance geometry.
    pub fn new(
        name: String,
        node_drain: NodeId,
        node_gate: NodeId,
        node_source: NodeId,
        node_e: NodeId,
        node_body: NodeId,
        node_p: NodeId,
        body_mode: BodyMode,
        model: Arc<B3SoiDdModel>,
        geom: B3SoiDdGeometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        // Self-heating is not yet implemented; per spec it must be a hard error
        // when enabled rather than silently ignored (SHMOD=0 in supported decks).
        if model.sh_mod == 1 && geom.rth0 != 0.0 {
            return Err(format!(
                "B3SOIDD '{name}': self-heating (SHMOD=1 with RTH0!=0) is not yet implemented"
            ));
        }
        let sized = Arc::new(B3SoiDdSized::new(&model, &geom, temp_k)?);
        let consts = ModelConsts {
            cox: model.cox,
            cbox: model.cbox,
            csi: model.csi,
            csieff: model.csieff,
            qsi: model.qsi,
            qsieff: model.qsieff,
            adice: model.adice,
            tox: model.tox,
            tsi: model.tsi,
            xj: model.xj,
            charge_q: super::common::CHARGE_Q,
            mob_mod: model.mob_mod,
        };
        Ok(Self {
            name,
            mtype: model.mtype,
            node_drain,
            node_gate,
            node_source,
            node_e,
            node_body,
            node_p,
            body_mode,
            model,
            sized,
            consts,
            op: B3SoiDdOp::default(),
            bias: B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
            },
            converged_ref: B3SoiDdBias {
                vbs: 0.0,
                vgs: 0.0,
                vds: 0.0,
                ves: 0.0,
                vps: 0.0,
            },
            has_history: false,
        })
    }

    /// Internal body node (used by the harness `@m1[vbs]` probe resolution).
    pub fn body_node(&self) -> NodeId {
        self.node_body
    }

    /// Extract device-polarity branch voltages from the solution vector
    /// (b3soiddld.c:483-498). Source-referenced, `mtype` folded in.
    fn branch_voltages(&self, v: &[Value]) -> B3SoiDdBias {
        let node = |n: NodeId| if n == 0 { 0.0 } else { v[n] };
        let vd = node(self.node_drain);
        let vg = node(self.node_gate);
        let vs = node(self.node_source);
        let ve = node(self.node_e);
        let vb = node(self.node_body);
        let vp = node(self.node_p);
        B3SoiDdBias {
            vbs: self.mtype * (vb - vs),
            vgs: self.mtype * (vg - vs),
            vds: self.mtype * (vd - vs),
            ves: self.mtype * (ve - vs),
            vps: self.mtype * (vp - vs),
        }
    }
}

impl NonlinearDevice for B3SoiDd {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_ref = self.bias;
        let bias = self.branch_voltages(voltages);
        self.bias = bias;
        self.op = eval::eval_dc(&self.sized, &self.consts, bias, self.mtype);
        self.has_history = true;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        rhs: &mut [Value],
    ) {
        let bias = self.branch_voltages(voltages);
        let op = if biases_match(bias, self.bias) {
            self.op.clone()
        } else {
            eval::eval_dc(&self.sized, &self.consts, bias, self.mtype)
        };
        self.stamp_op(&op, matrix, rhs);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.has_history {
            return false;
        }
        let reltol = criteria.relative_tolerance();
        let vtol = criteria.voltage_tolerance();
        let cmp = |new: Value, old: Value| {
            (new - old).abs() < reltol * new.abs().max(old.abs()) + vtol
        };
        cmp(self.bias.vbs, self.converged_ref.vbs)
            && cmp(self.bias.vgs, self.converged_ref.vgs)
            && cmp(self.bias.vds, self.converged_ref.vds)
            && cmp(self.bias.ves, self.converged_ref.ves)
            && cmp(self.bias.vps, self.converged_ref.vps)
    }
}

impl B3SoiDd {
    /// Stamp the linearized DC operating point.
    ///
    /// Faithful transcription of the DC portion of the B3SOIDD matrix/RHS load
    /// (b3soiddld.c:3886-4150) for `bodyMod` 0/2, no temp node, no series R, and
    /// `ChargeComputationNeeded == 0` (so all `gc*`/`ceqq*` charge terms vanish).
    /// `m` (multiplier) is 1. CKTgmin terms are omitted: the RSpice solver adds
    /// its own diagonal gmin, so replicating ngspice's per-device gmin would
    /// double-count it.
    ///
    /// In DC `dNodePrime == dNode`, `sNodePrime == sNode`, `bNode` is the body
    /// node (internal floating or external tie), and `pNode`/`tempNode` are
    /// absent.
    fn stamp_op(&self, op: &B3SoiDdOp, matrix: &mut impl MatrixStamper, rhs: &mut [Value]) {
        let (dp, g, sp, e, b) = (
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_e,
            self.node_body,
        );
        let mt = self.mtype;

        // The branch voltages used to form cdreq, in the *un-swapped* (raw)
        // frame, exactly as ngspice (vds/vgs/vbs/ves are pre-swap, device
        // polarity). These come from the last branch evaluation.
        let bias = self.bias;

        // ----- conductance groups (b3soiddld.c:3888-3944) -----
        let (gm, gmbs, gme, fwd_sum, rev_sum, cdreq, ceqbs, ceqbd);
        let (gbbg, gbbdp, gbbb, gbbe, gbbsp);
        let (gddpg, gddpdp, gddpb, gddpe, gddpsp);
        let (gsspg, gsspdp, gsspb, gsspsp);
        if op.mode >= 0 {
            gm = op.gm;
            gmbs = op.gmbs;
            gme = op.gme;
            fwd_sum = gm + gmbs + gme;
            rev_sum = 0.0;
            cdreq = mt
                * (op.cdrain
                    - op.gds * bias.vds
                    - gm * bias.vgs
                    - gmbs * bias.vbs
                    - gme * bias.ves);
            ceqbs = op.cjs;
            ceqbd = op.cjd;

            gbbg = -op.gbgs;
            gbbdp = -op.gbds;
            gbbb = -op.gbbs;
            gbbe = -op.gbes;
            let gbbp = -op.gbps;
            gbbsp = -(gbbg + gbbdp + gbbb + gbbe + gbbp);

            gddpg = -op.gjdg;
            gddpdp = -op.gjdd;
            gddpb = -op.gjdb;
            gddpe = -op.gjde;
            gddpsp = -(gddpg + gddpdp + gddpb + gddpe);

            gsspg = -op.gjsg;
            gsspdp = -op.gjsd;
            gsspb = -op.gjsb;
            let gsspe = 0.0;
            gsspsp = -(gsspg + gsspdp + gsspb + gsspe);
        } else {
            gm = -op.gm;
            gmbs = -op.gmbs;
            gme = -op.gme;
            fwd_sum = 0.0;
            rev_sum = -(gm + gmbs + gme);
            let vgd = bias.vgs - bias.vds;
            let vbd = bias.vbs - bias.vds;
            cdreq = -mt
                * (op.cdrain
                    + op.gds * bias.vds
                    + gm * vgd
                    + gmbs * vbd
                    + gme * (bias.ves - bias.vds));
            ceqbs = op.cjd;
            ceqbd = op.cjs;

            gbbg = -op.gbgs;
            gbbb = -op.gbbs;
            gbbe = -op.gbes;
            let gbbp = -op.gbps;
            gbbsp = -op.gbds;
            gbbdp = -(gbbg + gbbsp + gbbb + gbbe + gbbp);

            gddpg = -op.gjsg;
            gddpsp = -op.gjsd;
            gddpb = -op.gjsb;
            gddpe = 0.0;
            gddpdp = -(gddpg + gddpsp + gddpb + gddpe);

            gsspg = -op.gjdg;
            gsspdp = -op.gjdd;
            gsspb = -op.gjdb;
            let gsspe = -op.gjde;
            gsspsp = -(gsspg + gsspdp + gsspb + gsspe);
        }

        // type<0: flip junction/body equivalent currents (b3soiddld.c:3997)
        let ceqbody = -op.cbody;
        let (ceqbs, ceqbd, ceqbody) = if mt < 0.0 {
            (-ceqbs, -ceqbd, -ceqbody)
        } else {
            (ceqbs, ceqbd, ceqbody)
        };

        // ----- RHS (b3soiddld.c:4011-4017) -----
        rhs[b] -= ceqbody;
        rhs[dp] += ceqbd - cdreq;
        rhs[sp] += cdreq + ceqbs;

        // ----- matrix (b3soiddld.c:4090-4128, DC: gc*=0) -----
        // E row: only EePtr (gceeb==0) -> nothing in DC.
        // DP/SP body columns (b3soiddld.c:4092-4093):
        stamp(matrix, dp, b, -(-gddpb - gmbs));
        stamp(matrix, sp, b, -(-gsspb + gmbs));
        // Body row (b3soiddld.c:4094-4098):
        stamp(matrix, b, e, gbbe);
        stamp(matrix, b, g, gbbg);
        stamp(matrix, b, dp, gbbdp);
        stamp(matrix, b, sp, gbbsp);
        stamp(matrix, b, b, gbbb);

        // Gate row: gc* all zero in DC -> nothing.

        // Drain-prime row (b3soiddld.c:4106-4112):
        stamp(matrix, dp, g, gm + gddpg);
        stamp(matrix, dp, dp, op.gds + gddpdp + rev_sum);
        stamp(matrix, dp, sp, -(-gddpsp + op.gds + fwd_sum));

        // Source-prime row (b3soiddld.c:4114-4119):
        stamp(matrix, sp, g, -gm + gsspg);
        stamp(matrix, sp, dp, -(op.gds - gsspdp + rev_sum));
        stamp(matrix, sp, sp, op.gds + gsspsp + fwd_sum);
    }
}

#[inline]
fn stamp(matrix: &mut impl MatrixStamper, row: NodeId, col: NodeId, value: Value) {
    if row != 0 && col != 0 && value != 0.0 {
        matrix.stamp(row, col, value);
    }
}

#[inline]
fn biases_match(a: B3SoiDdBias, b: B3SoiDdBias) -> bool {
    a.vbs == b.vbs && a.vgs == b.vgs && a.vds == b.vds && a.ves == b.ves && a.vps == b.vps
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// The `N1` NMOS card from `tests/bsim3soidd/nmosdd.mod` (BSIMDD2.0).
    fn n1_params() -> HashMap<String, Value> {
        let pairs: &[(&str, Value)] = &[
            ("LEVEL", 56.0),
            ("TNOM", 27.0),
            ("TOX", 4.5e-9),
            ("TSI", 5e-8),
            ("TBOX", 8e-8),
            ("MOBMOD", 0.0),
            ("CAPMOD", 3.0),
            ("SHMOD", 0.0),
            ("PARAMCHK", 0.0),
            ("WINT", 0.0),
            ("LINT", -2e-8),
            ("VTH0", 0.52),
            ("K1", 0.39),
            ("K2", 0.1),
            ("K3", 0.0),
            ("KB1", 0.95),
            ("K3B", 2.2),
            ("NLX", 7.2e-8),
            ("DVT0", 0.55),
            ("DVT1", 0.28),
            ("DVT2", -1.4),
            ("DVT0W", 0.0),
            ("DVT1W", 0.0),
            ("DVT2W", 0.0),
            ("NCH", 3.3e17),
            ("NSUB", 1e15),
            ("NGATE", 1e20),
            ("DVBD0", 60.0),
            ("DVBD1", 1.1),
            ("VBSA", 0.0),
            ("KB3", 2.2),
            ("DELP", 0.02),
            ("ABP", 0.9),
            ("MXC", 0.9),
            ("ADICE0", 0.93),
            ("KBJT1", 1e-8),
            ("EDL", 5e-7),
            ("NDIODE", 1.13),
            ("NTUN", 14.0),
            ("ISBJT", 2e-6),
            ("ISDIF", 1e-6),
            ("ISTUN", 0.0),
            ("ISREC", 1e-5),
            ("XBJT", 0.01),
            ("XDIF", 0.01),
            ("XREC", 0.01),
            ("XTUN", 0.001),
            ("U0", 352.0),
            ("UA", 1.3e-11),
            ("UB", 1.7e-18),
            ("UC", -4e-10),
            ("W0", 1.16e-6),
            ("AGS", 0.25),
            ("A1", 0.0),
            ("A2", 1.0),
            ("B0", 0.01),
            ("B1", 10.0),
            ("RDSW", 700.0),
            ("PRWG", 0.0),
            ("PRWB", -0.2),
            ("WR", 1.0),
            ("RBODY", 0.0),
            ("RBSH", 0.0),
            ("A0", 1.4),
            ("KETA", -0.67),
            ("VSAT", 135000.0),
            ("DWG", 0.0),
            ("DWB", 0.0),
            ("ALPHA0", 0.0),
            ("ALPHA1", 1.5),
            ("BETA0", 20.5),
            ("AII", 1.2),
            ("BII", 0.1e-7),
            ("CII", 0.8),
            ("DII", 0.6),
            ("VOFF", -0.14),
            ("NFACTOR", 0.7),
            ("CDSC", 2e-5),
            ("CDSCB", 0.0),
            ("CDSCD", 0.0),
            ("CIT", 0.0),
            ("PCLM", 2.9),
            ("PVAG", 12.0),
            ("PDIBLC1", 0.18),
            ("PDIBLC2", 0.004),
            ("PDIBLCB", -0.234),
            ("DROUT", 0.2),
            ("DELTA", 0.01),
            ("ETA0", 0.01),
            ("ETAB", 0.0),
            ("DSUB", 0.3),
            ("RTH0", 0.006),
            ("CLC", 1e-7),
            ("CLE", 0.6),
            ("CF", 1e-20),
            ("CKAPPA", 0.6),
            ("CGDL", 1e-20),
            ("CGSL", 1e-20),
            ("KT1", -0.3),
            ("KT1L", 0.0),
            ("KT2", 0.022),
            ("UTE", -1.5),
            ("UA1", 4.31e-9),
            ("UB1", -7.61e-18),
            ("UC1", -5.6e-11),
            ("PRT", 760.0),
            ("AT", 22400.0),
            ("CGSO", 1e-10),
            ("CGDO", 1e-10),
            ("CJSWG", 5e-10),
            ("TT", 3e-10),
            ("ASD", 0.3),
            ("CSDESW", 1e-12),
        ];
        pairs.iter().map(|(k, v)| (k.to_string(), *v)).collect()
    }

    fn geom() -> B3SoiDdGeometry {
        // w=10u l=0.25u, no area/perimeter given on the card.
        B3SoiDdGeometry {
            l: 0.25e-6,
            w: 10e-6,
            drain_area: 0.0,
            source_area: 0.0,
            drain_squares: 0.0,
            source_squares: 0.0,
            drain_perimeter: 0.0,
            source_perimeter: 0.0,
            body_squares: 0.0,
            rth0: 0.006,
            cth0: 0.0,
        }
    }

    fn model_consts(m: &B3SoiDdModel) -> ModelConsts {
        ModelConsts {
            cox: m.cox,
            cbox: m.cbox,
            csi: m.csi,
            csieff: m.csieff,
            qsi: m.qsi,
            qsieff: m.qsieff,
            adice: m.adice,
            tox: m.tox,
            tsi: m.tsi,
            xj: m.xj,
            charge_q: super::super::common::CHARGE_Q,
            mob_mod: m.mob_mod,
        }
    }

    #[test]
    fn temp_setup_is_finite_and_physical() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        assert!(sized.phi > 0.5 && sized.phi < 1.2, "phi={}", sized.phi);
        assert!(sized.vtm > 0.02 && sized.vtm < 0.03, "vtm={}", sized.vtm);
        assert!(sized.u0temp > 0.0 && sized.u0temp < 1.0);
        assert!(sized.vth0.is_finite());
        assert!(sized.rds0 >= 0.0);
        // jbjt etc. must be positive saturation densities.
        assert!(sized.jbjt > 0.0 && sized.jrec > 0.0 && sized.jdif > 0.0);
    }

    #[test]
    fn eval_dc_strong_inversion_is_sane() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);

        // Tied body (vbs=0), Vg=3, Vd=0.05 (linear), like t4 first point.
        let bias = B3SoiDdBias {
            vbs: 0.0,
            vgs: 3.0,
            vds: 0.05,
            ves: 0.0,
            vps: 0.0,
        };
        let op = eval::eval_dc(&sized, &mc, bias, 1.0);
        assert!(op.ids.is_finite() && op.ids > 0.0, "ids={}", op.ids);
        assert!(op.gds.is_finite() && op.gds > 0.0, "gds={}", op.gds);
        assert!(op.gm.is_finite() && op.gm >= 0.0, "gm={}", op.gm);
        assert!(op.von.is_finite());
        // Linear-region drain current for W/L=40, this device is ~ mA range.
        assert!(op.ids < 1.0, "ids unreasonably large: {}", op.ids);
    }

    #[test]
    fn eval_dc_monotonic_in_vg() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        let mk = |vg: Value| {
            eval::eval_dc(
                &sized,
                &mc,
                B3SoiDdBias {
                    vbs: 0.0,
                    vgs: vg,
                    vds: 0.05,
                    ves: 0.0,
                    vps: 0.0,
                },
                1.0,
            )
            .ids
        };
        let i_low = mk(0.3); // below threshold
        let i_mid = mk(0.8);
        let i_high = mk(1.5);
        assert!(i_low < i_mid, "{} !< {}", i_low, i_mid);
        assert!(i_mid < i_high, "{} !< {}", i_mid, i_high);
        assert!(i_low >= 0.0);
    }

    #[test]
    fn eval_dc_no_nan_across_sweep() {
        let model = B3SoiDdModel::from_params(&n1_params(), false, 300.15);
        let sized = B3SoiDdSized::new(&model, &geom(), 300.15).expect("sized");
        let mc = model_consts(&model);
        for vg_i in 0..=15 {
            for vd_i in 0..=30 {
                for vbs_i in 0..=4 {
                    let bias = B3SoiDdBias {
                        vbs: 0.1 * vbs_i as Value,
                        vgs: 0.1 * vg_i as Value,
                        vds: 0.1 * vd_i as Value,
                        ves: 0.0,
                        vps: 0.0,
                    };
                    let op = eval::eval_dc(&sized, &mc, bias, 1.0);
                    assert!(op.ids.is_finite(), "ids NaN at {bias:?}");
                    assert!(op.cbody.is_finite(), "cbody NaN at {bias:?}");
                    assert!(op.gds.is_finite() && op.gm.is_finite());
                    assert!(op.cjd.is_finite() && op.cjs.is_finite());
                }
            }
        }
    }
}
