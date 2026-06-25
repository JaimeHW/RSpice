//! BSIM4 v4.8 bulk MOSFET model (SPICE levels 14/54).
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim4/`. The model math —
//! parameters ([`params`]), size/temperature preconditioning ([`temp`]),
//! and the load equations ([`eval`]) — is self-contained; the engine wiring
//! lives in [`device`] ([`Bsim4v8Device`]), which the builder registers for
//! LEVEL=14/54 cards. The module layout mirrors the in-tree BSIM3v3.3 port
//! (`device::mosfet::bsim3v3`) and its LEVEL=8/49 integration.
//!
//! # What is ported
//!
//! - Full model card with L/W/P binning and the b4set.c defaults
//!   ([`Bsim4v8Model`]); TNOM enters in Celsius exactly as b4mpar.c converts.
//! - `BSIM4temp`: the per-model temperature block, the (W, L, NF)-keyed
//!   `bsim4SizeDependParam` knots, and the per-instance tail
//!   (stress layout correction, well proximity, `delvto`/`mulu0`,
//!   `vfbzb`/`vtfbphi*`/`vbsc`, `BSIM4PAeffGeo` diffusion variants,
//!   series conductances from explicit `NRD`/`NRS` or `RGEOMOD=1..8`
//!   implicit resistance geometry, the junction-diode limiting anchors, the
//!   reverse TAT saturation currents) — see [`temp`], including the
//!   always-on fatal checks of b4check.c. All four `tempMod` variants
//!   (0/1/2/3) are covered.
//! - `BSIM4load` DC path for the canonical mode set plus `rdsMod=1`
//!   external source/drain resistance, `rbodyMod=1/2` distributed substrate
//!   resistance, builder-lowered `rgateMod=1` constant gate-electrode
//!   resistance, native `rgateMod=2` bias-dependent gate resistance, and
//!   native `rgateMod=3` middle-gate resistance:
//!   junction diodes
//!   (`dioMod=0/1/2`, explicit `gmin`), reverse-bias TAT
//!   current, Vth chain (DVT/DSUB, `K1ox`/`K2ox`, DITS incl. v4.7
//!   `DITS_SFT2`), poly depletion, `Vgsteff`, internal `Rds(V)` or external
//!   nonlinear `Rd(V)`/`Rs(V)`, `Abulk`,
//!   MOBMOD 0 through 6, `Vdsat` (with `lambda` velocity overshoot and `vtl`
//!   source-end velocity limit), `Vdseff`, the Early stack
//!   (`Vasat`/`VACLM`/`VADIBL`/`VADITS`/`VASCBE`), `Ids` with analytic
//!   `gm`/`gds`/`gmbs`, the substrate current, and GIDL/GISL for both
//!   `gidlMod` 0 and 1 — see [`eval`].
//! - CAPMOD=0/1/2 intrinsic charges with integer CVCHARGEMOD=0/1/2 where applicable,
//!   the full capacitance matrix, junction depletion charges, CAPMOD=0
//!   linear overlap charges, CAPMOD=1/2 smoothed overlap charges, and the
//!   mode-dependent node-charge assembly.
//!
//! # What is intentionally not ported (typed errors, not silent fallbacks)
//!
//! Rejected at construction ([`Bsim4v8::new`]):
//!
//! - invalid `rgateMod` and `rbodyMod` selector values. Transient NQS is
//!   native for the supported source/drain, body, and gate-resistance
//!   topologies, and it may coexist with AC NQS on the same model card.
//!
//! AC charge-deficit NQS (`acnqsMod=1`) is native for `rbodyMod = 0/1/2`,
//! `rdsMod = 0/1`, and `rgateMod = 0/1/2/3`.
//!
//! Rejected at charge-request time (DC is unaffected): unknown `cvchargeMod`
//! selectors beyond `0/1/2`; selectors `1` and `2` share ngspice's nonzero
//! branch. The `capMod=0/1/2` charge paths are ported; after those selectors are validated,
//! `xpart < 0` suppresses intrinsic channel charge as ngspice does.
//!
//! Noise selectors (`fnoiMod`/`tnoiMod`) and the SOA limits are accepted
//! and stored — they do not affect the DC/charge load. ngspice-46's BSIM4
//! parses an instance `dtemp` but never uses it; this port does the same
//! (the device evaluates at the temperature the temp pass ran at).
//! In noise analysis, `tnoiMod=0/1/2`, `fnoiMod=0/1`, gate-shot sources, and
//! the `tnoiMod=1` source/drain series-noise conductance adjustment are
//! emitted natively for the supported `rdsMod=0/1` topologies.
//!
//! # Integration seams
//!
//! The [`Bsim4v8`] core owns `Arc<Bsim4v8Model>` + `Arc<Bsim4v8SizeDep>`
//! + [`Bsim4v8InstTemp`] and exposes [`Bsim4v8::eval_polarity`] (raw node
//! voltages in, `mtype`-folded internally) plus the ngspice limiting
//! sequence [`Bsim4v8::limit_voltages`]. The op struct carries every
//! `here->BSIM4*` quantity the b4ld.c stamp consumes; the multiplier `m`,
//! the mode swap of the matrix load, and the gmin policy live in the
//! engine-facing wrapper ([`Bsim4v8Device`]).

pub mod common;
pub mod device;
pub mod eval;
pub mod params;
pub mod temp;

pub use device::{Bsim4v8ChargeMatrix, Bsim4v8Device};
pub use eval::{Bsim4v8Bias, Bsim4v8Charge, Bsim4v8JunctionBias, Bsim4v8Op};
pub use params::{Binned, Bsim4v8Model};
pub use temp::{Bsim4v8Geometry, Bsim4v8InstTemp, Bsim4v8ModelTemp, Bsim4v8SizeDep, SizeDepCache};

use crate::Value;
use crate::device::mosfet::Mosfet;
use std::sync::Arc;

/// One BSIM4 v4.8 instance: shared model card + size knot + instance tail.
#[derive(Debug, Clone)]
pub struct Bsim4v8 {
    pub name: String,
    /// +1 NMOS / -1 PMOS (model `mtype`).
    pub mtype: Value,
    pub model: Arc<Bsim4v8Model>,
    pub model_temp: Arc<Bsim4v8ModelTemp>,
    pub size: Arc<Bsim4v8SizeDep>,
    pub inst: Bsim4v8InstTemp,
    pub geom: Bsim4v8Geometry,
}

impl Bsim4v8 {
    /// Build an instance from a model card and instance geometry at the
    /// given device temperature (Kelvin). Mirrors `BSIM4setup` + `BSIM4temp`
    /// for a single instance; unsupported model options are rejected here
    /// rather than silently ignored (see the module docs for the list).
    pub fn new(
        name: String,
        model: Arc<Bsim4v8Model>,
        geom: Bsim4v8Geometry,
        temp_k: Value,
    ) -> Result<Self, String> {
        Self::validate_model(&name, &model, &geom)?;
        let model_temp = Arc::new(Bsim4v8ModelTemp::new(&model, temp_k));
        let size = Arc::new(Bsim4v8SizeDep::new(
            &model,
            &model_temp,
            geom.l,
            geom.w,
            geom.nf,
        )?);
        let inst = Bsim4v8InstTemp::new(&model, &model_temp, &size, &geom);
        Ok(Self {
            name,
            mtype: model.mtype,
            model,
            model_temp,
            size,
            inst,
            geom,
        })
    }

    /// Build an instance against a shared model-temperature block, memoizing
    /// the (W, L, NF) size knot in `cache` — the engine builder path,
    /// mirroring ngspice's `pSizeDependParamKnot` reuse across same-geometry
    /// instances.
    pub fn new_shared(
        name: String,
        model: Arc<Bsim4v8Model>,
        model_temp: Arc<Bsim4v8ModelTemp>,
        cache: &mut SizeDepCache,
        geom: Bsim4v8Geometry,
    ) -> Result<Self, String> {
        Self::validate_model(&name, &model, &geom)?;
        let size = cache.get(&model, &model_temp, geom.l, geom.w, geom.nf)?;
        let inst = Bsim4v8InstTemp::new(&model, &model_temp, &size, &geom);
        Ok(Self {
            name,
            mtype: model.mtype,
            model,
            model_temp,
            size,
            inst,
            geom,
        })
    }

    /// The unsupported-option rejections of `BSIM4setup`/`b4check.c`:
    /// unimplemented model options are typed errors, never silent fallbacks
    /// (see the module docs for the list).
    fn validate_model(
        name: &str,
        model: &Bsim4v8Model,
        geom: &Bsim4v8Geometry,
    ) -> Result<(), String> {
        if model.rgate_mod > 3 {
            return Err(format!(
                "BSIM4 '{name}': RGATEMOD={} is not implemented (only RGATEMOD=0, 1, 2, or 3)",
                model.rgate_mod
            ));
        }
        if model.rbody_mod > 2 {
            return Err(format!(
                "BSIM4 '{name}': RBODYMOD={} is not implemented (only RBODYMOD=0, 1, or 2)",
                model.rbody_mod
            ));
        }
        if model.trnqs_mod != 0 {
            if !matches!(model.rgate_mod, 0 | 1 | 2 | 3) {
                return Err(format!(
                    "BSIM4 '{name}': TRNQSMOD=1 with RGATEMOD={} is not implemented (only RGATEMOD=0, 1, 2, or 3)",
                    model.rgate_mod
                ));
            }
        }
        if model.acnqs_mod != 0 {
            if !matches!(model.rgate_mod, 0 | 1 | 2 | 3) {
                return Err(format!(
                    "BSIM4 '{name}': ACNQSMOD=1 with RGATEMOD={} is not implemented (only RGATEMOD=0, 1, 2, or 3)",
                    model.rgate_mod
                ));
            }
        }
        if !(0..=6).contains(&model.mob_mod) {
            return Err(format!(
                "BSIM4 '{name}': MOBMOD={} is unsupported (supported selectors: 0 through 6)",
                model.mob_mod
            ));
        }
        // Stress model activation test of b4temp.c:1656.
        let stress_active =
            geom.sa > 0.0 && geom.sb > 0.0 && (geom.nf == 1.0 || (geom.nf > 1.0 && geom.sd > 0.0));
        if stress_active {
            if model.saref <= 0.0 {
                return Err(format!(
                    "BSIM4 '{name}': SAREF={} is not positive for active stress effect",
                    model.saref
                ));
            }
            if model.sbref <= 0.0 {
                return Err(format!(
                    "BSIM4 '{name}': SBREF={} is not positive for active stress effect",
                    model.sbref
                ));
            }
        }
        let geo_mod = if geom.geo_mod_given {
            geom.geo_mod
        } else {
            model.geo_mod
        };
        if !(0..=10).contains(&geo_mod) {
            return Err(format!(
                "BSIM4 '{name}': GEOMOD={} is unsupported (supported selectors: 0 through 10)",
                geo_mod
            ));
        }
        let rgeo_mod = if geom.rgeo_mod_given || geom.rgeo_mod != 0 {
            geom.rgeo_mod
        } else {
            model.rgeo_mod
        };
        if !(0..=8).contains(&rgeo_mod) {
            return Err(format!(
                "BSIM4 '{name}': RGEOMOD={} is unsupported (supported selectors: 0 through 8)",
                rgeo_mod
            ));
        }
        Ok(())
    }

    /// Evaluate at device-polarity branch voltages (already limited).
    pub fn eval(
        &self,
        bias: Bsim4v8Bias,
        gmin: Value,
        compute_charges: bool,
    ) -> Result<Bsim4v8Op, String> {
        self.eval_with_junction_bias(bias, None, gmin, compute_charges)
    }

    pub fn eval_with_junction_bias(
        &self,
        bias: Bsim4v8Bias,
        junction_bias: Option<Bsim4v8JunctionBias>,
        gmin: Value,
        compute_charges: bool,
    ) -> Result<Bsim4v8Op, String> {
        self.eval_with_junction_and_gate_mid_bias(bias, junction_bias, None, gmin, compute_charges)
    }

    pub fn eval_with_junction_and_gate_mid_bias(
        &self,
        bias: Bsim4v8Bias,
        junction_bias: Option<Bsim4v8JunctionBias>,
        gate_mid_vgs: Option<Value>,
        gmin: Value,
        compute_charges: bool,
    ) -> Result<Bsim4v8Op, String> {
        eval::eval(
            &self.model,
            &self.model_temp,
            &self.size,
            &self.inst,
            bias,
            junction_bias,
            gate_mid_vgs,
            gmin,
            compute_charges,
        )
    }

    /// Evaluate the operating point at raw node-space voltages: `mtype` is
    /// folded into the branch voltages here, exactly as b4ld.c forms
    /// `vbs`/`vgs`/`vds` from the node vector (b4ld.c:380-391).
    pub fn eval_polarity(
        &self,
        vds_node: Value,
        vgs_node: Value,
        vbs_node: Value,
        gmin: Value,
        compute_charges: bool,
    ) -> Result<Bsim4v8Op, String> {
        self.eval(
            Bsim4v8Bias {
                vds: self.mtype * vds_node,
                vgs: self.mtype * vgs_node,
                vbs: self.mtype * vbs_node,
            },
            gmin,
            compute_charges,
        )
    }

    /// Per-iteration Newton limiting, the exact b4ld.c sequence (lines
    /// 605-689 for `rgateMod = rbodyMod = rdsMod = 0`): `DEVfetlim` on vgs
    /// (or vgd in inverse mode) around the previous `von`, `DEVlimvds` on
    /// vds, then `DEVpnjlim` on the forward-biased body junction with
    /// `CONSTvt0` and the model `vcrit`.
    ///
    /// `new`/`old` are device-polarity branch voltages (the `old` triple is
    /// the previous iterate's accepted state, ngspice `CKTstate0`);
    /// `von_prev` is the previous iterate's threshold (`here->BSIM4von`,
    /// 0 before the first evaluation). Returns the limited bias and the
    /// `Check` flag from `DEVpnjlim` (ngspice bumps `CKTnoncon` when set).
    pub fn limit_voltages(
        &self,
        new: Bsim4v8Bias,
        old: Bsim4v8Bias,
        von_prev: Value,
    ) -> (Bsim4v8Bias, bool) {
        let mut vgs = new.vgs;
        let mut vds = new.vds;
        let mut vbs = new.vbs;
        let mut vgd = vgs - vds;
        let vgdo = old.vgs - old.vds;

        if old.vds >= 0.0 {
            vgs = Mosfet::dev_fetlim(vgs, old.vgs, von_prev);
            vds = vgs - vgd;
            vds = Mosfet::dev_limvds(vds, old.vds);
            vgd = vgs - vds;
            let _ = vgd;
        } else {
            vgd = Mosfet::dev_fetlim(vgd, vgdo, von_prev);
            vds = vgs - vgd;
            vds = -Mosfet::dev_limvds(-vds, -old.vds);
            vgs = vgd + vds;
        }

        let mut check = false;
        if vds >= 0.0 {
            vbs = pnjlim(
                vbs,
                old.vbs,
                common::CONST_VT0,
                self.model_temp.vcrit,
                &mut check,
            );
        } else {
            let vbd_old = old.vbs - old.vds;
            let vbd = pnjlim(
                vbs - vds,
                vbd_old,
                common::CONST_VT0,
                self.model_temp.vcrit,
                &mut check,
            );
            vbs = vbd + vds;
        }

        (Bsim4v8Bias { vds, vgs, vbs }, check)
    }
}

/// `DEVpnjlim` (ngspice devsup.c:49-84) including the `*icheck` out-flag
/// that b4ld.c feeds into the convergence counter. The flag-less variant
/// lives on [`Mosfet`] (`dev_pnjlim`); the BSIM4 load needs the flag, so
/// the few lines are mirrored here with the check semantics intact.
pub(super) fn pnjlim(
    vnew: Value,
    vold: Value,
    vt: Value,
    vcrit: Value,
    icheck: &mut bool,
) -> Value {
    if vnew > vcrit && (vnew - vold).abs() > vt + vt {
        *icheck = true;
        if vold > 0.0 {
            let arg = (vnew - vold) / vt;
            if arg > 0.0 {
                vold + vt * (2.0 + (arg - 2.0).ln())
            } else {
                vold - vt * (2.0 + (2.0 - arg).ln())
            }
        } else {
            vt * (vnew / vt).ln()
        }
    } else if vnew < 0.0 {
        let arg = if vold > 0.0 {
            -vold - 1.0
        } else {
            2.0 * vold - 1.0
        };
        if vnew < arg {
            *icheck = true;
            arg
        } else {
            *icheck = false;
            vnew
        }
    } else {
        *icheck = false;
        vnew
    }
}

#[cfg(test)]
mod tests;
