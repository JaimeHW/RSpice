//! BSIM4 v4.8 bulk MOSFET model (SPICE levels 14/54).
//!
//! Ported from ngspice-46 `src/spicelib/devices/bsim4/`. This is a
//! self-contained model module — parameters ([`params`]), size/temperature
//! preconditioning ([`temp`]), and the load equations ([`eval`]) — with **no
//! engine integration yet**: nothing here stamps a matrix or registers with
//! the builder. The module layout mirrors the in-tree BSIM3v3.3 port
//! (`device::mosfet::bsim3v3`) so the eventual LEVEL=14/54 wiring can follow
//! the LEVEL=8/49 pattern.
//!
//! # What is ported
//!
//! - Full model card with L/W/P binning and the b4set.c defaults
//!   ([`Bsim4v8Model`]); TNOM enters in Celsius exactly as b4mpar.c converts.
//! - `BSIM4temp`: the per-model temperature block, the (W, L, NF)-keyed
//!   `bsim4SizeDependParam` knots, and the per-instance tail
//!   (`delvto`/`mulu0`, `vfbzb`/`vtfbphi*`/`vbsc`, `BSIM4PAeffGeo` for
//!   geoMod=0, series conductances, the `dioMod=1` forward anchors, the
//!   reverse TAT saturation currents) — see [`temp`], including the
//!   always-on fatal checks of b4check.c. All four `tempMod` variants
//!   (0/1/2/3) are covered.
//! - `BSIM4load` DC path for the canonical mode set: junction diodes
//!   (`dioMod=1` ijth linearization, explicit `gmin`), reverse-bias TAT
//!   current, Vth chain (DVT/DSUB, `K1ox`/`K2ox`, DITS incl. v4.7
//!   `DITS_SFT2`), poly depletion, `Vgsteff`, internal `Rds(V)`, `Abulk`,
//!   MOBMOD 0/1/2, `Vdsat` (with `lambda` velocity overshoot and `vtl`
//!   source-end velocity limit), `Vdseff`, the Early stack
//!   (`Vasat`/`VACLM`/`VADIBL`/`VADITS`/`VASCBE`), `Ids` with analytic
//!   `gm`/`gds`/`gmbs`, the substrate current, and GIDL/GISL for both
//!   `gidlMod` 0 and 1 — see [`eval`].
//! - CAPMOD=2 (charge-thickness) intrinsic charges with the full
//!   capacitance matrix, junction depletion charges, and the smoothed
//!   overlap charges, plus the mode-dependent node-charge assembly.
//!
//! # What is intentionally not ported (typed errors, not silent fallbacks)
//!
//! Rejected at construction ([`Bsim4v8::new`]):
//!
//! - `rdsMod=1` (external S/D resistance nodes), `rgateMod=1/2/3` (gate
//!   resistance network), `rbodyMod=1/2` (substrate network),
//!   `trnqsMod`/`acnqsMod=1` (charge-deficit NQS).
//! - `mobMod=3/4/5/6` (high-k / Synopsys variants).
//! - `dioMod=0/2` (resistance-free / breakdown diode).
//! - `igcMod`/`igbMod != 0` (gate tunneling currents).
//! - `capMod=0/1` charge models (the default `capMod=2` is the ported one).
//! - `mtrlMod=1` (non-silicon substrate) and `wpemod=1` (well proximity).
//! - The stress model: instance `sa>0 && sb>0` (with `nf=1`, or `nf>1` and
//!   `sd>0`, the exact b4temp.c activation test).
//! - `geoMod != 0` unless all of AD/AS/PD/PS are given explicitly (only
//!   the geoMod=0 variant of `BSIM4PAeffGeo` is ported).
//!
//! Rejected at charge-request time (DC is unaffected): `cvchargeMod=1`,
//!   `xpart < 0` (intrinsic-charge suppression) is honored.
//!
//! Noise selectors (`fnoiMod`/`tnoiMod`) and the SOA limits are accepted
//! and stored — they do not affect the DC/charge load. ngspice-46's BSIM4
//! parses an instance `dtemp` but never uses it; this port does the same
//! (the device evaluates at the temperature the temp pass ran at).
//!
//! # Integration seams (for the future LEVEL=14/54 wiring)
//!
//! The [`Bsim4v8`] device owns `Arc<Bsim4v8Model>` + `Arc<Bsim4v8SizeDep>`
//! + [`Bsim4v8InstTemp`] and exposes [`Bsim4v8::eval_polarity`] (raw node
//! voltages in, `mtype`-folded internally) plus the ngspice limiting
//! sequence [`Bsim4v8::limit_voltages`]. The op struct carries every
//! `here->BSIM4*` quantity the b4ld.c stamp consumes; the multiplier `m`,
//! the mode swap of the matrix load, and the gmin policy are the engine's
//! responsibility.

pub mod common;
pub mod eval;
pub mod params;
pub mod temp;

pub use eval::{Bsim4v8Bias, Bsim4v8Charge, Bsim4v8Op};
pub use params::{Binned, Bsim4v8Model};
pub use temp::{
    Bsim4v8Geometry, Bsim4v8InstTemp, Bsim4v8ModelTemp, Bsim4v8SizeDep, SizeDepCache,
};

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
        if model.rds_mod != 0 {
            return Err(format!(
                "BSIM4 '{name}': RDSMOD={} is not implemented (only RDSMOD=0)",
                model.rds_mod
            ));
        }
        if model.rgate_mod != 0 {
            return Err(format!(
                "BSIM4 '{name}': RGATEMOD={} is not implemented (only RGATEMOD=0)",
                model.rgate_mod
            ));
        }
        if model.rbody_mod != 0 {
            return Err(format!(
                "BSIM4 '{name}': RBODYMOD={} is not implemented (only RBODYMOD=0)",
                model.rbody_mod
            ));
        }
        if model.trnqs_mod != 0 || model.acnqs_mod != 0 {
            return Err(format!(
                "BSIM4 '{name}': TRNQSMOD/ACNQSMOD=1 is not implemented"
            ));
        }
        if !(0..=2).contains(&model.mob_mod) {
            return Err(format!(
                "BSIM4 '{name}': MOBMOD={} is not implemented (only 0, 1 or 2)",
                model.mob_mod
            ));
        }
        if model.dio_mod != 1 {
            return Err(format!(
                "BSIM4 '{name}': DIOMOD={} is not implemented (only DIOMOD=1)",
                model.dio_mod
            ));
        }
        if model.igc_mod != 0 || model.igb_mod != 0 {
            return Err(format!(
                "BSIM4 '{name}': gate current IGCMOD={}/IGBMOD={} is not implemented",
                model.igc_mod, model.igb_mod
            ));
        }
        if model.cap_mod != 2 {
            return Err(format!(
                "BSIM4 '{name}': CAPMOD={} is not implemented (only CAPMOD=2)",
                model.cap_mod
            ));
        }
        if model.mtrl_mod != 0 {
            return Err(format!(
                "BSIM4 '{name}': MTRLMOD=1 is not implemented (only MTRLMOD=0)"
            ));
        }
        if model.wpemod != 0 {
            return Err(format!(
                "BSIM4 '{name}': WPEMOD=1 (well proximity effect) is not implemented"
            ));
        }
        // Stress model activation test of b4temp.c:1593.
        if geom.sa > 0.0 && geom.sb > 0.0 && (geom.nf == 1.0 || (geom.nf > 1.0 && geom.sd > 0.0)) {
            return Err(format!(
                "BSIM4 '{name}': the stress effect model (SA/SB given) is not implemented"
            ));
        }
        if model.geo_mod != 0
            && !(geom.drain_area_given
                && geom.source_area_given
                && geom.drain_perimeter_given
                && geom.source_perimeter_given)
        {
            return Err(format!(
                "BSIM4 '{name}': GEOMOD={} without explicit AD/AS/PD/PS is not implemented \
                 (only the GEOMOD=0 diffusion geometry)",
                model.geo_mod
            ));
        }
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

    /// Evaluate at device-polarity branch voltages (already limited).
    pub fn eval(
        &self,
        bias: Bsim4v8Bias,
        gmin: Value,
        compute_charges: bool,
    ) -> Result<Bsim4v8Op, String> {
        eval::eval(
            &self.model,
            &self.model_temp,
            &self.size,
            &self.inst,
            bias,
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
fn pnjlim(vnew: Value, vold: Value, vt: Value, vcrit: Value, icheck: &mut bool) -> Value {
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
