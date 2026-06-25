//! BSIM4 v4.8 model card: full parameter set with ngspice defaults.
//!
//! Parameter list transcribed from ngspice-46 `b4.c` (`BSIM4mPTable`),
//! defaults from `b4set.c` (`BSIM4setup`). TNOM is converted from Celsius to
//! Kelvin on entry exactly as `b4mpar.c` does (`BSIM4_MOD_TNOM:
//! value + CONSTCtoK`).
//!
//! # Mode-selector policy (what errors instead of computing wrong physics)
//!
//! ngspice rounds BSIM4 selector literals with `floor(value + 0.5)` before
//! validation, and silently *resets* some out-of-range selectors to their
//! defaults. This port follows reset-to-default only where the default path is
//! native and oracle-backed (`CAPMOD` and `DIOMOD`); other explicit selectors
//! must round to the native supported set or fail closed. Honoring the card
//! while ignoring or rewriting unvalidated selectors would silently change
//! results, so:
//!
//! - `rdsMod=0/1` internal and external bias-dependent S/D resistance are ported.
//! - `rbodyMod=1/2` distributed substrate networks are ported; `rbodyMod=2`
//!   follows the ngspice geometry-scaled bodymode logic.
//! - `rgateMod=1` constant gate-electrode resistance is ported through a
//!   builder-lowered external gate resistor. `rgateMod=2` is ported as the
//!   native bias-dependent gate-resistance branch. `rgateMod=3` is ported as
//!   the native middle-gate resistance network.
//! - `acnqsMod=1` is ported for AC with `rbodyMod = 0/1/2`,
//!   `rdsMod = 0/1`, and `rgateMod = 0/1/2/3`. `trnqsMod=1`
//!   transient charge-deficit NQS is native for the supported body,
//!   source/drain, and gate-resistance topologies, and may coexist with
//!   `acnqsMod=1`.
//! - `mobMod=0..6` mobility selectors are ported, including the high-k /
//!   Synopsys variants.
//! - `capMod=0/1/2` charge models are ported; `capMod=2` remains the
//!   default charge-thickness model, with integer `cvchargeMod=0/1/2/3`
//!   supported.
//!   DC is charge-model independent.
//! - `dioMod=0/1/2` source/drain junction diode selectors are ported.
//! - `wpemod=1` well-proximity is ported for `SC` and explicit
//!   `SCA`/`SCB`/`SCC` instance inputs.
//! - The BSIM4 stress layout correction is ported for active `SA`/`SB`
//!   layouts, including the multi-finger `SD` path.
//! - `mtrlMod=1` material constants are ported for both compatibility modes;
//!   `mtrlCompatMod=0` computes instance-local EOT-derived TOXP/COXP.
//! - `tempMod=0/1/2/3` are all ported (they only redirect a handful of
//!   temperature formulas).
//! - `geoMod=0..10` implicit `BSIM4PAeffGeo` diffusion geometries are
//!   ported; `rgeoMod=1..8` S/D resistance geometry is ported for omitted
//!   `NRD`/`NRS`, including Xyce-style model-card defaults.
//!
//! Noise selectors (`fnoiMod`/`tnoiMod`) and the SOA limits (`vgsMax`
//! family) are accepted and stored — they do not affect the DC/charge load.
//! `tnoiMod=1` adjusts source/drain series-resistance noise during noise
//! analysis; the load stores the selector and operating point, while the
//! engine applies that noise-only adjustment.
//! The engine currently emits native `tnoiMod=0/1/2`, `fnoiMod=0/1`, and
//! gate shot-noise sources for the supported `rdsMod=0/1` topologies.

use super::common::{EPS0, PI};
use crate::Value;
use std::collections::HashMap;

/// Binned model parameter: nominal value plus L/W/cross-term dependence.
///
/// `b4temp.c` evaluates every binned family as
/// `v + l*Inv_L + w*Inv_W + p*Inv_LW`.
#[derive(Debug, Clone, Copy)]
pub struct Binned {
    pub v: Value,
    pub l: Value,
    pub w: Value,
    pub p: Value,
}

impl Binned {
    #[inline]
    pub fn eval(&self, inv_l: Value, inv_w: Value, inv_lw: Value) -> Value {
        self.v + self.l * inv_l + self.w * inv_w + self.p * inv_lw
    }
}

/// Full BSIM4 v4.8 `.model` card (one per model statement, shared by
/// instances). Field order loosely follows `sBSIM4model` in `bsim4def.h`.
#[derive(Debug, Clone)]
pub struct Bsim4v8Model {
    /// +1.0 for NMOS, -1.0 for PMOS (ngspice `BSIM4type`).
    pub mtype: Value,

    // Selectors (omitted values use b4set.c defaults; explicit values are
    // validated before construction).
    pub cvcharge_mod: i32,
    pub cvcharge_mod_value: Value,
    pub cap_mod: i32,
    pub dio_mod: i32,
    pub rds_mod: i32,
    pub trnqs_mod: i32,
    pub acnqs_mod: i32,
    pub mob_mod: i32,
    pub rbody_mod: i32,
    pub rgate_mod: i32,
    pub per_mod: i32,
    pub geo_mod: i32,
    pub rgeo_mod: i32,
    pub fnoi_mod: i32,
    pub tnoi_mod: i32,
    pub mtrl_mod: i32,
    pub mtrl_compat_mod: i32,
    pub igc_mod: i32,
    pub igb_mod: i32,
    pub temp_mod: i32,
    pub gidl_mod: i32,
    pub param_chk: i32,
    pub bin_unit: i32,
    pub wpemod: i32,
    /// Version string; every "4.8x" spelling resolves to the 4.8.3 physics
    /// in ngspice-46 (`BSIM4v48intVersion` is V483 for any prefix "4.8"), so
    /// no behavior here branches on it beyond that resolution.
    pub version: String,

    // Oxide / gate stack.
    pub toxe: Value,
    pub toxe_given: bool,
    pub toxp: Value,
    pub toxp_given: bool,
    pub toxm: Value,
    pub toxm_given: bool,
    pub toxref: Value,
    pub dtox: Value,
    pub dtox_given: bool,
    pub epsrox: Value,
    pub eot: Value,
    pub eot_given: bool,
    pub vddeot: Value,
    pub tempeot: Value,
    pub leffeot: Value,
    pub weffeot: Value,
    pub ados: Value,
    pub bdos: Value,

    // Material card (mtrlMod=1).
    pub phig: Value,
    pub phig_given: bool,
    pub epsrgate: Value,
    pub easub: Value,
    pub epsrsub: Value,
    pub ni0sub: Value,
    pub bg0sub: Value,
    pub tbgasub: Value,
    pub tbgbsub: Value,

    // Binned families (defaults per b4set.c; b4temp.c evaluation order).
    pub cdsc: Binned,
    pub cdscb: Binned,
    pub cdscd: Binned,
    pub cit: Binned,
    pub nfactor: Binned,
    pub tnfactor: Binned,
    pub xj: Binned,
    pub vsat: Binned,
    pub at: Binned,
    pub a0: Binned,
    pub ags: Binned,
    pub a1: Binned,
    pub a2: Binned,
    pub keta: Binned,
    pub ketac: Binned,
    pub nsub: Binned,
    pub nsub_given: bool,
    pub ndep: Binned,
    pub ndep_given: bool,
    pub nsd: Binned,
    pub phin: Binned,
    pub ngate: Binned,
    pub gamma1: Binned,
    pub gamma1_given: bool,
    pub gamma2: Binned,
    pub gamma2_given: bool,
    pub vbx: Binned,
    pub vbx_given: bool,
    pub vbm: Binned,
    pub xt: Binned,
    pub vfb: Binned,
    pub vfb_given: bool,
    pub k1: Binned,
    pub k1_given: bool,
    pub kt1: Binned,
    pub kt1l: Binned,
    pub k2: Binned,
    pub k2_given: bool,
    pub kt2: Binned,
    pub k3: Binned,
    pub k3b: Binned,
    pub w0: Binned,
    pub lpe0: Binned,
    pub lpeb: Binned,
    pub dvtp0: Binned,
    pub dvtp1: Binned,
    pub dvtp2: Binned,
    pub dvtp3: Binned,
    pub dvtp4: Binned,
    pub dvtp5: Binned,
    pub dvt0: Binned,
    pub dvt1: Binned,
    pub dvt2: Binned,
    pub dvt0w: Binned,
    pub dvt1w: Binned,
    pub dvt2w: Binned,
    pub drout: Binned,
    pub dsub: Binned,
    pub vth0: Binned,
    pub vth0_given: bool,
    pub ua: Binned,
    pub ua1: Binned,
    pub ub: Binned,
    pub ub1: Binned,
    pub uc: Binned,
    pub uc1: Binned,
    pub ud: Binned,
    pub ud1: Binned,
    pub up: Binned,
    pub lp: Binned,
    pub eu: Binned,
    pub u0: Binned,
    pub ute: Binned,
    pub ucs: Binned,
    pub ucste: Binned,
    pub voff: Binned,
    pub tvoff: Binned,
    pub minv: Binned,
    pub minvcv: Binned,
    pub fprout: Binned,
    pub pdits: Binned,
    pub pditsd: Binned,
    pub delta: Binned,
    pub rdsw: Binned,
    pub rdw: Binned,
    pub rsw: Binned,
    pub prwg: Binned,
    pub prwb: Binned,
    pub prt: Binned,
    pub eta0: Binned,
    pub teta0: Binned,
    pub tvoffcv: Binned,
    pub etab: Binned,
    pub pclm: Binned,
    pub pdibl1: Binned,
    pub pdibl2: Binned,
    pub pdiblb: Binned,
    pub pscbe1: Binned,
    pub pscbe2: Binned,
    pub pvag: Binned,
    pub wr: Binned,
    pub dwg: Binned,
    pub dwb: Binned,
    pub b0: Binned,
    pub b1: Binned,
    pub alpha0: Binned,
    pub alpha1: Binned,
    pub beta0: Binned,
    pub agidl: Binned,
    pub bgidl: Binned,
    pub cgidl: Binned,
    pub egidl: Binned,
    pub rgidl: Binned,
    pub kgidl: Binned,
    pub fgidl: Binned,
    pub agisl: Binned,
    pub bgisl: Binned,
    pub cgisl: Binned,
    pub egisl: Binned,
    pub rgisl: Binned,
    pub kgisl: Binned,
    pub fgisl: Binned,
    pub aigc: Binned,
    pub bigc: Binned,
    pub cigc: Binned,
    pub aigsd: Binned,
    pub bigsd: Binned,
    pub cigsd: Binned,
    pub aigs: Binned,
    pub bigs: Binned,
    pub cigs: Binned,
    pub aigd: Binned,
    pub bigd: Binned,
    pub cigd: Binned,
    pub aigbacc: Binned,
    pub bigbacc: Binned,
    pub cigbacc: Binned,
    pub aigbinv: Binned,
    pub bigbinv: Binned,
    pub cigbinv: Binned,
    pub nigc: Binned,
    pub nigbacc: Binned,
    pub nigbinv: Binned,
    pub ntox: Binned,
    pub eigbinv: Binned,
    pub pigcd: Binned,
    pub pigcd_given: bool,
    pub poxedge: Binned,
    pub xrcrg1: Binned,
    pub xrcrg2: Binned,
    pub lambda: Binned,
    pub lambda_given: bool,
    pub vtl: Binned,
    pub vtl_given: bool,
    pub xn: Binned,
    pub vfbsdoff: Binned,
    pub tvfbsdoff: Binned,

    // C-V binned families.
    pub cgsl: Binned,
    pub cgdl: Binned,
    pub ckappas: Binned,
    pub ckappad: Binned,
    pub cf: Binned,
    pub clc: Binned,
    pub cle: Binned,
    pub vfbcv: Binned,
    pub acde: Binned,
    pub moin: Binned,
    pub noff: Binned,
    pub voffcv: Binned,

    // Well-proximity binned families (wpemod=1).
    pub kvth0we: Binned,
    pub k2we: Binned,
    pub ku0we: Binned,

    // Non-binned scalars: Vgsteff / Rds / gate current.
    pub voffl: Value,
    pub voffcvl: Value,
    pub pditsl: Value,
    pub rdswmin: Value,
    pub rdwmin: Value,
    pub rswmin: Value,
    pub lc: Value,
    pub gidlclamp: Value,
    pub idovvdsc: Value,
    pub dlcig: Value,
    pub dlcigd: Value,

    // Diode card (S and D sides; D defaults to S).
    pub ijthsfwd: Value,
    pub ijthdfwd: Value,
    pub ijthsrev: Value,
    pub ijthdrev: Value,
    pub xjbvs: Value,
    pub xjbvd: Value,
    pub bvs: Value,
    pub bvd: Value,
    pub jss: Value,
    pub jsws: Value,
    pub jswgs: Value,
    pub pbs: Value,
    pub njs: Value,
    pub xtis: Value,
    pub mjs: Value,
    pub pbsws: Value,
    pub mjsws: Value,
    pub pbswgs: Value,
    pub mjswgs: Value,
    pub cjs: Value,
    pub cjsws: Value,
    pub cjswgs: Value,
    pub jsd: Value,
    pub jswd: Value,
    pub jswgd: Value,
    pub pbd: Value,
    pub njd: Value,
    pub xtid: Value,
    pub mjd: Value,
    pub pbswd: Value,
    pub mjswd: Value,
    pub pbswgd: Value,
    pub mjswgd: Value,
    pub cjd: Value,
    pub cjswd: Value,
    pub cjswgd: Value,

    // Trap-assisted tunneling card.
    pub jtss: Value,
    pub jtsd: Value,
    pub jtssws: Value,
    pub jtsswd: Value,
    pub jtsswgs: Value,
    pub jtsswgd: Value,
    pub jtweff: Value,
    pub njts: Value,
    pub njtssw: Value,
    pub njtsswg: Value,
    pub njtsd: Value,
    pub njtsswd: Value,
    pub njtsswgd: Value,
    pub xtss: Value,
    pub xtsd: Value,
    pub xtssws: Value,
    pub xtsswd: Value,
    pub xtsswgs: Value,
    pub xtsswgd: Value,
    pub tnjts: Value,
    pub tnjtssw: Value,
    pub tnjtsswg: Value,
    pub tnjtsd: Value,
    pub tnjtsswd: Value,
    pub tnjtsswgd: Value,
    pub vtss: Value,
    pub vtsd: Value,
    pub vtssws: Value,
    pub vtsswd: Value,
    pub vtsswgs: Value,
    pub vtsswgd: Value,

    // Junction capacitance temperature coefficients.
    pub tpb: Value,
    pub tcj: Value,
    pub tpbsw: Value,
    pub tcjsw: Value,
    pub tpbswg: Value,
    pub tcjswg: Value,

    // Layout / contact card (rgateMod / geoMod consumers; stored).
    pub dmcg: Value,
    pub dmci: Value,
    pub dmdg: Value,
    pub dmcgt: Value,
    pub xgw: Value,
    pub xgl: Value,
    pub rshg: Value,
    pub ngcon: Value,

    // Body-resistance network card.
    pub gbmin: Value,
    pub rbdb: Value,
    pub rbpb: Value,
    pub rbsb: Value,
    pub rbps: Value,
    pub rbpd: Value,
    pub rbps0: Value,
    pub rbps0_given: bool,
    pub rbpsl: Value,
    pub rbpsw: Value,
    pub rbpsnf: Value,
    pub rbpd0: Value,
    pub rbpd0_given: bool,
    pub rbpdl: Value,
    pub rbpdw: Value,
    pub rbpdnf: Value,
    pub rbpbx0: Value,
    pub rbpbxl: Value,
    pub rbpbxw: Value,
    pub rbpbxnf: Value,
    pub rbpby0: Value,
    pub rbpbyl: Value,
    pub rbpbyw: Value,
    pub rbpbynf: Value,
    pub rbsbx0: Value,
    pub rbsbx0_given: bool,
    pub rbsby0: Value,
    pub rbsby0_given: bool,
    pub rbdbx0: Value,
    pub rbdbx0_given: bool,
    pub rbdby0: Value,
    pub rbdby0_given: bool,
    pub rbsdbxl: Value,
    pub rbsdbxw: Value,
    pub rbsdbxnf: Value,
    pub rbsdbyl: Value,
    pub rbsdbyw: Value,
    pub rbsdbynf: Value,

    /// TNOM in Kelvin (card value is Celsius, transformed on entry).
    pub tnom: Value,
    pub cgso: Value,
    pub cgso_given: bool,
    pub cgdo: Value,
    pub cgdo_given: bool,
    pub cgbo: Value,
    pub cgbo_given: bool,
    pub cf_given: bool,
    pub xpart: Value,
    pub sheet_resistance: Value,

    // Geometry scaling.
    pub lint: Value,
    pub ll: Value,
    pub llc: Value,
    pub lln: Value,
    pub lw: Value,
    pub lwc: Value,
    pub lwn: Value,
    pub lwl: Value,
    pub lwlc: Value,
    pub lmin: Value,
    pub lmax: Value,
    pub wint: Value,
    pub wl: Value,
    pub wlc: Value,
    pub wln: Value,
    pub ww: Value,
    pub wwc: Value,
    pub wwn: Value,
    pub wwl: Value,
    pub wwlc: Value,
    pub wmin: Value,
    pub wmax: Value,
    pub dwc: Value,
    pub dlc: Value,
    pub dlc_given: bool,
    pub dwj: Value,
    pub xl: Value,
    pub xw: Value,

    // Stress-effect card (instance SA/SB/SD consumes these in temp.rs).
    pub saref: Value,
    pub sbref: Value,
    pub wlod: Value,
    pub ku0: Value,
    pub kvsat: Value,
    pub kvth0: Value,
    pub tku0: Value,
    pub llodku0: Value,
    pub wlodku0: Value,
    pub llodvth: Value,
    pub wlodvth: Value,
    pub lku0: Value,
    pub wku0: Value,
    pub pku0: Value,
    pub lkvth0: Value,
    pub wkvth0: Value,
    pub pkvth0: Value,
    pub stk2: Value,
    pub lodk2: Value,
    pub steta0: Value,
    pub lodeta0: Value,

    // Well-proximity scalar card (wpemod=1).
    pub web: Value,
    pub wec: Value,
    pub scref: Value,

    // Noise card (stored; noise analyses are out of scope for the load).
    pub tnoia: Value,
    pub tnoib: Value,
    pub tnoic: Value,
    pub rnoia: Value,
    pub rnoib: Value,
    pub rnoic: Value,
    pub ntnoi: Value,
    pub oxide_trap_density_a: Value,
    pub oxide_trap_density_b: Value,
    pub oxide_trap_density_c: Value,
    pub em: Value,
    pub ef: Value,
    pub af: Value,
    pub kf: Value,
    pub lintnoi: Value,

    // SOA voltage limits (stored; the SOA check is out of scope).
    pub vgs_max: Value,
    pub vgd_max: Value,
    pub vgb_max: Value,
    pub vds_max: Value,
    pub vbs_max: Value,
    pub vbd_max: Value,
    pub vgsr_max: Value,
    pub vgdr_max: Value,
    pub vgbr_max: Value,
    pub vbsr_max: Value,
    pub vbdr_max: Value,
}

#[inline]
fn get(map: &HashMap<String, Value>, key: &str) -> Option<Value> {
    map.get(key).copied().filter(|v| v.is_finite())
}

#[inline]
fn val(map: &HashMap<String, Value>, key: &str, default: Value) -> Value {
    get(map, key).unwrap_or(default)
}

/// Fetch a binned family: `name`, `l<name>`, `w<name>`, `p<name>`
/// (uppercase keys).
fn binned(map: &HashMap<String, Value>, name: &str, v_def: Value) -> Binned {
    Binned {
        v: val(map, name, v_def),
        l: val(map, &format!("L{name}"), 0.0),
        w: val(map, &format!("W{name}"), 0.0),
        p: val(map, &format!("P{name}"), 0.0),
    }
}

/// Binned family whose nominal *and* bins default to another family's
/// (`ketac` from `keta`, the GISL family from GIDL — b4set.c).
fn binned_from(map: &HashMap<String, Value>, name: &str, src: &Binned) -> Binned {
    Binned {
        v: val(map, name, src.v),
        l: val(map, &format!("L{name}"), src.l),
        w: val(map, &format!("W{name}"), src.w),
        p: val(map, &format!("P{name}"), src.p),
    }
}

fn selector_values(allowed: &[i32]) -> String {
    allowed
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn rounded_selector(key: &str, raw: Value, values: &str) -> Result<i32, String> {
    if !raw.is_finite() {
        return Err(format!(
            "BSIM4 selector {key} must be a finite numeric selector (supported values: {values}); got {key}={raw}"
        ));
    }
    let rounded = (raw + 0.5).floor();
    if !rounded.is_finite() || rounded < i32::MIN as Value || rounded > i32::MAX as Value {
        return Err(format!(
            "BSIM4 selector {key}={raw} rounds outside the finite integer range (supported values: {values})"
        ));
    }
    Ok(rounded as i32)
}

/// Fetch a native selector. Omitted selectors default like b4set.c; explicit
/// values are rounded with ngspice's `floor(value + 0.5)` rule, then fail
/// closed unless the effective selector is native.
fn selector(
    map: &HashMap<String, Value>,
    key: &str,
    default: i32,
    allowed: &[i32],
) -> Result<i32, String> {
    let Some(raw) = map.get(key).copied() else {
        return Ok(default);
    };
    let values = selector_values(allowed);
    let value = rounded_selector(key, raw, &values)?;
    if allowed.contains(&value) {
        Ok(value)
    } else {
        Err(format!(
            "BSIM4 selector {key} must round to a finite integer in the supported set ({values}); got {key}={raw} (effective {value})"
        ))
    }
}

fn selector_reset_to_default(
    map: &HashMap<String, Value>,
    key: &str,
    default: i32,
    allowed: &[i32],
) -> Result<i32, String> {
    let Some(raw) = map.get(key).copied() else {
        return Ok(default);
    };
    let values = selector_values(allowed);
    let value = rounded_selector(key, raw, &values)?;
    if allowed.contains(&value) {
        Ok(value)
    } else {
        Ok(default)
    }
}

impl Bsim4v8Model {
    pub fn cvcharge_mod_supported_for_charges(&self) -> bool {
        self.cvcharge_mod_value.trunc() == self.cvcharge_mod_value
            && (0..=3).contains(&self.cvcharge_mod)
    }

    /// Build a model card from an uppercase-keyed parameter map.
    ///
    /// `is_pmos` selects the device polarity, `nominal_temp_k` supplies the
    /// circuit nominal temperature in Kelvin (ngspice `CKTnomTemp`) used
    /// when `TNOM` is absent (TNOM on the card is in Celsius).
    pub fn from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Self {
        Self::try_from_params(params, is_pmos, nominal_temp_k)
            .expect("valid BSIM4 v4.8 model parameters")
    }

    /// Fallible model-card constructor used at user-facing build boundaries.
    pub fn try_from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Result<Self, String> {
        let p = params;
        let mtype: Value = if is_pmos { -1.0 } else { 1.0 };
        let nmos = !is_pmos;

        let mob_mod = selector(p, "MOBMOD", 0, &[0, 1, 2, 3, 4, 5, 6])?;
        let mtrl_mod = selector(p, "MTRLMOD", 0, &[0, 1])?;
        let mtrl_compat_mod = selector(p, "MTRLCOMPATMOD", 0, &[0, 1])?;

        // Oxide stack with the toxe/toxp/dtox or eot/toxp/dtox resolution
        // of b4temp.c:123-160. The mtrlCompatMod=0 TOXP iteration is
        // instance-local and stays out of the immutable card.
        let toxe_given = get(p, "TOXE").is_some();
        let toxp_given = get(p, "TOXP").is_some();
        let toxm_given = get(p, "TOXM").is_some();
        let dtox_given = get(p, "DTOX").is_some();
        let eot_given = get(p, "EOT").is_some();
        let mut toxe = val(p, "TOXE", 30.0e-10);
        let dtox = val(p, "DTOX", 0.0);
        let mut toxp = val(p, "TOXP", toxe);
        let mut toxm_default = toxe;
        let epsrox = val(p, "EPSROX", 3.9);
        let mut eot = val(p, "EOT", 15.0e-10);
        if mtrl_mod == 0 {
            let chktol = toxe.abs().max(toxp.abs()).max(dtox.abs()) * 1e-14;
            if toxe_given && toxp_given && dtox_given && (toxe - (toxp + dtox)).abs() > chktol {
                log::warn!(
                    "BSIM4: toxe, toxp and dtox all given and toxe != toxp + dtox; dtox ignored"
                );
            } else if toxe_given && !toxp_given {
                toxp = toxe - dtox;
            } else if !toxe_given && toxp_given {
                toxe = toxp + dtox;
                toxm_default = toxe;
            }
        } else if mtrl_compat_mod != 0 {
            let t0 = epsrox / 3.9;
            if eot_given && toxp_given && dtox_given && (eot * t0 - (toxp + dtox)).abs() > 1.0e-20 {
                log::warn!(
                    "BSIM4: eot, toxp and dtox all given and eot * EPSROX / 3.9 != toxp + dtox; dtox ignored"
                );
            } else if eot_given && !toxp_given {
                toxp = t0 * eot - dtox;
            } else if !eot_given && toxp_given {
                eot = (toxp + dtox) / t0;
                toxm_default = eot;
            }
        }
        let toxm = val(p, "TOXM", toxm_default);
        let material_epsrox = if mtrl_mod != 0 { 3.9 } else { epsrox };
        let material_toxe = if mtrl_mod != 0 { eot } else { toxe };
        let coxe = material_epsrox * EPS0 / material_toxe;

        // Binned families with cross-defaults.
        let keta = binned(p, "KETA", -0.047);
        let ketac = binned_from(p, "KETAC", &keta);
        let drout = binned(p, "DROUT", 0.56);
        let dsub = Binned {
            v: val(p, "DSUB", drout.v),
            l: val(p, "LDSUB", 0.0),
            w: val(p, "WDSUB", 0.0),
            p: val(p, "PDSUB", 0.0),
        };
        let agidl = binned(p, "AGIDL", 0.0);
        let bgidl = binned(p, "BGIDL", 2.3e9);
        let cgidl = binned(p, "CGIDL", 0.5);
        let egidl = binned(p, "EGIDL", 0.8);
        let rgidl = binned(p, "RGIDL", 1.0);
        let kgidl = binned(p, "KGIDL", 0.0);
        let fgidl = binned(p, "FGIDL", 1.0);

        // aigs/aigd vs aigsd (b4set.c:548-583 + the binned variants): when
        // AIGSD is given it overrides both sides, bins included.
        let ig_def = |n: Value, q: Value| if nmos { n } else { q };
        let aigsd_given = get(p, "AIGSD").is_some();
        let bigsd_given = get(p, "BIGSD").is_some();
        let cigsd_given = get(p, "CIGSD").is_some();
        let aigsd = binned(p, "AIGSD", ig_def(1.36e-2, 9.80e-3));
        let bigsd = binned(p, "BIGSD", ig_def(1.71e-3, 7.59e-4));
        let cigsd = binned(p, "CIGSD", ig_def(0.075, 0.03));
        let (aigs, aigd) = if aigsd_given {
            (aigsd, aigsd)
        } else {
            (
                binned(p, "AIGS", ig_def(1.36e-2, 9.80e-3)),
                binned(p, "AIGD", ig_def(1.36e-2, 9.80e-3)),
            )
        };
        let (bigs, bigd) = if bigsd_given {
            (bigsd, bigsd)
        } else {
            (
                binned(p, "BIGS", ig_def(1.71e-3, 7.59e-4)),
                binned(p, "BIGD", ig_def(1.71e-3, 7.59e-4)),
            )
        };
        let (cigs, cigd) = if cigsd_given {
            (cigsd, cigsd)
        } else {
            (
                binned(p, "CIGS", ig_def(0.075, 0.03)),
                binned(p, "CIGD", ig_def(0.075, 0.03)),
            )
        };

        let ckappas = binned(p, "CKAPPAS", 0.6);
        let ckappad = binned_from(p, "CKAPPAD", &ckappas);
        let xj = binned(p, "XJ", 0.15e-6);
        let cgsl = binned(p, "CGSL", 0.0);
        let cgdl = binned(p, "CGDL", 0.0);

        // Geometry-scaling chain.
        let lint = val(p, "LINT", 0.0);
        let ll = val(p, "LL", 0.0);
        let lw = val(p, "LW", 0.0);
        let lwl = val(p, "LWL", 0.0);
        let wint = val(p, "WINT", 0.0);
        let wl = val(p, "WL", 0.0);
        let ww = val(p, "WW", 0.0);
        let wwl = val(p, "WWL", 0.0);
        let dwc = val(p, "DWC", wint);
        let dlc_given = get(p, "DLC").is_some();
        let dlc = val(p, "DLC", lint);
        let dlcig = val(p, "DLCIG", lint);
        let dlcigd = get(p, "DLCIGD").unwrap_or(if get(p, "DLCIG").is_some() {
            dlcig
        } else {
            lint
        });

        // cf/cg defaults are computed in b4temp.c from effective toxe/coxe.
        let cf_given = get(p, "CF").is_some();
        let cf = Binned {
            v: get(p, "CF").unwrap_or_else(|| {
                2.0 * material_epsrox * EPS0 / PI * (1.0 + 0.4e-6 / material_toxe).ln()
            }),
            l: val(p, "LCF", 0.0),
            w: val(p, "WCF", 0.0),
            p: val(p, "PCF", 0.0),
        };
        // cgdo/cgso/cgbo defaults (b4temp.c:181-196).
        let cgdo_given = get(p, "CGDO").is_some();
        let cgdo = get(p, "CGDO").unwrap_or(if dlc_given && dlc > 0.0 {
            dlc * coxe - cgdl.v
        } else {
            0.6 * xj.v * coxe
        });
        let cgso_given = get(p, "CGSO").is_some();
        let cgso = get(p, "CGSO").unwrap_or(if dlc_given && dlc > 0.0 {
            dlc * coxe - cgsl.v
        } else {
            0.6 * xj.v * coxe
        });
        let cgbo_given = get(p, "CGBO").is_some();
        let cgbo = get(p, "CGBO").unwrap_or(2.0 * dwc * coxe);

        // Diode default chains (D side from S side).
        let ijthsfwd = val(p, "IJTHSFWD", 0.1);
        let ijthsrev = val(p, "IJTHSREV", 0.1);
        let xjbvs = val(p, "XJBVS", 1.0);
        let bvs = val(p, "BVS", 10.0);
        let jss = val(p, "JSS", 1.0e-4);
        let jsws = val(p, "JSWS", 0.0);
        let jswgs = val(p, "JSWGS", 0.0);
        let pbs = val(p, "PBS", 1.0);
        let njs = val(p, "NJS", 1.0);
        let xtis = val(p, "XTIS", 3.0);
        let mjs = val(p, "MJS", 0.5);
        let pbsws = val(p, "PBSWS", 1.0);
        let mjsws = val(p, "MJSWS", 0.33);
        let pbswgs = val(p, "PBSWGS", pbsws);
        let mjswgs = val(p, "MJSWGS", mjsws);
        let cjs = val(p, "CJS", 5.0e-4);
        let cjsws = val(p, "CJSWS", 5.0e-10);
        let cjswgs = val(p, "CJSWGS", cjsws);

        // Trap-assisted tunneling defaults (njtsd-family inherits the S side
        // when that was given; same value either way since the defaults
        // match — kept for given-ness fidelity).
        let jtss = val(p, "JTSS", 0.0);
        let jtssws = val(p, "JTSSWS", 0.0);
        let jtsswgs = val(p, "JTSSWGS", 0.0);
        let njts = val(p, "NJTS", 20.0);
        let njtssw = val(p, "NJTSSW", 20.0);
        let njtsswg = val(p, "NJTSSWG", 20.0);
        let xtss = val(p, "XTSS", 0.02);
        let xtssws = val(p, "XTSSWS", 0.02);
        let xtsswgs = val(p, "XTSSWGS", 0.02);
        let tnjts = val(p, "TNJTS", 0.0);
        let tnjtssw = val(p, "TNJTSSW", 0.0);
        let tnjtsswg = val(p, "TNJTSSWG", 0.0);
        let vtss = val(p, "VTSS", 10.0);
        let vtssws = val(p, "VTSSWS", 10.0);
        let vtsswgs = val(p, "VTSSWGS", 10.0);

        // TNOM on the card is Celsius; CKTnomTemp is Kelvin (b4mpar.c:3368).
        let tnom = get(p, "TNOM").map(|t| t + 273.15).unwrap_or(nominal_temp_k);

        let dmcg = val(p, "DMCG", 0.0);
        let cvcharge_mod_value = val(p, "CVCHARGEMOD", 0.0);

        Ok(Self {
            mtype,
            cvcharge_mod: cvcharge_mod_value as i32,
            cvcharge_mod_value,
            cap_mod: selector_reset_to_default(p, "CAPMOD", 2, &[0, 1, 2])?,
            dio_mod: selector_reset_to_default(p, "DIOMOD", 1, &[0, 1, 2])?,
            rds_mod: selector(p, "RDSMOD", 0, &[0, 1])?,
            trnqs_mod: selector(p, "TRNQSMOD", 0, &[0, 1])?,
            acnqs_mod: selector(p, "ACNQSMOD", 0, &[0, 1])?,
            mob_mod,
            rbody_mod: selector(p, "RBODYMOD", 0, &[0, 1, 2])?,
            rgate_mod: selector(p, "RGATEMOD", 0, &[0, 1, 2, 3])?,
            per_mod: selector(p, "PERMOD", 1, &[0, 1])?,
            geo_mod: selector(p, "GEOMOD", 0, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10])?,
            rgeo_mod: selector(p, "RGEOMOD", 0, &[0, 1, 2, 3, 4, 5, 6, 7, 8])?,
            fnoi_mod: selector(p, "FNOIMOD", 1, &[0, 1])?,
            tnoi_mod: selector(p, "TNOIMOD", 0, &[0, 1, 2])?,
            mtrl_mod,
            mtrl_compat_mod,
            igc_mod: selector(p, "IGCMOD", 0, &[0, 1, 2])?,
            igb_mod: selector(p, "IGBMOD", 0, &[0, 1])?,
            temp_mod: selector(p, "TEMPMOD", 0, &[0, 1, 2, 3])?,
            gidl_mod: selector(p, "GIDLMOD", 0, &[0, 1])?,
            param_chk: val(p, "PARAMCHK", 1.0) as i32,
            bin_unit: val(p, "BINUNIT", 1.0) as i32,
            wpemod: selector(p, "WPEMOD", 0, &[0, 1])?,
            version: get(p, "VERSION")
                .map(|v| format!("{v}"))
                .unwrap_or_else(|| "4.8.3".to_string()),

            toxe,
            toxe_given,
            toxp,
            toxp_given,
            toxm,
            toxm_given,
            toxref: val(p, "TOXREF", 30.0e-10),
            dtox,
            dtox_given,
            epsrox,
            eot,
            eot_given,
            vddeot: val(p, "VDDEOT", if nmos { 1.5 } else { -1.5 }),
            tempeot: val(p, "TEMPEOT", 300.15),
            leffeot: val(p, "LEFFEOT", 1.0),
            weffeot: val(p, "WEFFEOT", 10.0),
            ados: val(p, "ADOS", 1.0),
            bdos: val(p, "BDOS", 1.0),

            phig: val(p, "PHIG", 4.05),
            phig_given: get(p, "PHIG").is_some(),
            epsrgate: val(p, "EPSRGATE", 11.7),
            easub: val(p, "EASUB", 4.05),
            epsrsub: val(p, "EPSRSUB", 11.7),
            ni0sub: val(p, "NI0SUB", 1.45e10),
            bg0sub: val(p, "BG0SUB", 1.16),
            tbgasub: val(p, "TBGASUB", 7.02e-4),
            tbgbsub: val(p, "TBGBSUB", 1108.0),

            cdsc: binned(p, "CDSC", 2.4e-4),
            cdscb: binned(p, "CDSCB", 0.0),
            cdscd: binned(p, "CDSCD", 0.0),
            cit: binned(p, "CIT", 0.0),
            nfactor: binned(p, "NFACTOR", 1.0),
            tnfactor: binned(p, "TNFACTOR", 0.0),
            xj,
            vsat: binned(p, "VSAT", 8.0e4),
            at: binned(p, "AT", 3.3e4),
            a0: binned(p, "A0", 1.0),
            ags: binned(p, "AGS", 0.0),
            a1: binned(p, "A1", 0.0),
            a2: binned(p, "A2", 1.0),
            keta,
            ketac,
            nsub: binned(p, "NSUB", 6.0e16),
            nsub_given: get(p, "NSUB").is_some(),
            ndep: binned(p, "NDEP", 1.7e17),
            ndep_given: get(p, "NDEP").is_some(),
            nsd: binned(p, "NSD", 1.0e20),
            phin: binned(p, "PHIN", 0.0),
            ngate: binned(p, "NGATE", 0.0),
            gamma1: binned(p, "GAMMA1", 0.0),
            gamma1_given: get(p, "GAMMA1").is_some(),
            gamma2: binned(p, "GAMMA2", 0.0),
            gamma2_given: get(p, "GAMMA2").is_some(),
            vbx: binned(p, "VBX", 0.0),
            vbx_given: get(p, "VBX").is_some(),
            vbm: binned(p, "VBM", -3.0),
            xt: binned(p, "XT", 1.55e-7),
            vfb: binned(p, "VFB", -1.0),
            vfb_given: get(p, "VFB").is_some(),
            k1: binned(p, "K1", 0.0),
            k1_given: get(p, "K1").is_some(),
            kt1: binned(p, "KT1", -0.11),
            kt1l: binned(p, "KT1L", 0.0),
            k2: binned(p, "K2", 0.0),
            k2_given: get(p, "K2").is_some(),
            kt2: binned(p, "KT2", 0.022),
            k3: binned(p, "K3", 80.0),
            k3b: binned(p, "K3B", 0.0),
            w0: binned(p, "W0", 2.5e-6),
            lpe0: binned(p, "LPE0", 1.74e-7),
            lpeb: binned(p, "LPEB", 0.0),
            dvtp0: binned(p, "DVTP0", 0.0),
            dvtp1: binned(p, "DVTP1", 0.0),
            dvtp2: binned(p, "DVTP2", 0.0),
            dvtp3: binned(p, "DVTP3", 0.0),
            dvtp4: binned(p, "DVTP4", 0.0),
            dvtp5: binned(p, "DVTP5", 0.0),
            dvt0: binned(p, "DVT0", 2.2),
            dvt1: binned(p, "DVT1", 0.53),
            dvt2: binned(p, "DVT2", -0.032),
            dvt0w: binned(p, "DVT0W", 0.0),
            dvt1w: binned(p, "DVT1W", 5.3e6),
            dvt2w: binned(p, "DVT2W", -0.032),
            drout,
            dsub,
            vth0: Binned {
                v: get(p, "VTH0")
                    .or_else(|| get(p, "VTHO"))
                    .unwrap_or(if nmos { 0.7 } else { -0.7 }),
                l: get(p, "LVTH0").or_else(|| get(p, "LVTHO")).unwrap_or(0.0),
                w: get(p, "WVTH0").or_else(|| get(p, "WVTHO")).unwrap_or(0.0),
                p: get(p, "PVTH0").or_else(|| get(p, "PVTHO")).unwrap_or(0.0),
            },
            vth0_given: get(p, "VTH0").or_else(|| get(p, "VTHO")).is_some(),
            // 4.8.1+ defaults (ngspice-46 resolves every "4.8x" version
            // string to the 4.8.3 physics, so the <=4.8.0 default branch
            // of b4set.c is unreachable).
            ua: binned(
                p,
                "UA",
                if mob_mod == 2 || mob_mod == 6 {
                    1.0e-15
                } else {
                    1.0e-9
                },
            ),
            ua1: binned(p, "UA1", 1.0e-9),
            ub: binned(p, "UB", 1.0e-19),
            ub1: binned(p, "UB1", -1.0e-18),
            uc: binned(
                p,
                "UC",
                if mob_mod == 1 || mob_mod == 5 {
                    -0.0465
                } else {
                    -0.0465e-9
                },
            ),
            uc1: binned(
                p,
                "UC1",
                if mob_mod == 1 || mob_mod == 5 {
                    -0.056
                } else {
                    -0.056e-9
                },
            ),
            ud: binned(p, "UD", 0.0),
            ud1: binned(p, "UD1", 0.0),
            up: binned(p, "UP", 0.0),
            lp: binned(p, "LP", 1.0e-8),
            eu: binned(p, "EU", if nmos { 1.67 } else { 1.0 }),
            u0: binned(p, "U0", if nmos { 0.067 } else { 0.025 }),
            ute: binned(p, "UTE", -1.5),
            ucs: binned(p, "UCS", if nmos { 1.67 } else { 1.0 }),
            ucste: binned(p, "UCSTE", -4.775e-3),
            voff: binned(p, "VOFF", -0.08),
            tvoff: binned(p, "TVOFF", 0.0),
            minv: binned(p, "MINV", 0.0),
            minvcv: binned(p, "MINVCV", 0.0),
            fprout: binned(p, "FPROUT", 0.0),
            pdits: binned(p, "PDITS", 0.0),
            pditsd: binned(p, "PDITSD", 0.0),
            delta: binned(p, "DELTA", 0.01),
            rdsw: binned(p, "RDSW", 200.0),
            rdw: binned(p, "RDW", 100.0),
            rsw: binned(p, "RSW", 100.0),
            prwg: binned(p, "PRWG", 1.0),
            prwb: binned(p, "PRWB", 0.0),
            prt: binned(p, "PRT", 0.0),
            eta0: binned(p, "ETA0", 0.08),
            teta0: binned(p, "TETA0", 0.0),
            tvoffcv: binned(p, "TVOFFCV", 0.0),
            etab: binned(p, "ETAB", -0.07),
            pclm: binned(p, "PCLM", 1.3),
            pdibl1: Binned {
                v: val(p, "PDIBLC1", 0.39),
                l: val(p, "LPDIBLC1", 0.0),
                w: val(p, "WPDIBLC1", 0.0),
                p: val(p, "PPDIBLC1", 0.0),
            },
            pdibl2: Binned {
                v: val(p, "PDIBLC2", 0.0086),
                l: val(p, "LPDIBLC2", 0.0),
                w: val(p, "WPDIBLC2", 0.0),
                p: val(p, "PPDIBLC2", 0.0),
            },
            pdiblb: Binned {
                v: val(p, "PDIBLCB", 0.0),
                l: val(p, "LPDIBLCB", 0.0),
                w: val(p, "WPDIBLCB", 0.0),
                p: val(p, "PPDIBLCB", 0.0),
            },
            pscbe1: binned(p, "PSCBE1", 4.24e8),
            pscbe2: binned(p, "PSCBE2", 1.0e-5),
            pvag: binned(p, "PVAG", 0.0),
            wr: binned(p, "WR", 1.0),
            dwg: binned(p, "DWG", 0.0),
            dwb: binned(p, "DWB", 0.0),
            b0: binned(p, "B0", 0.0),
            b1: binned(p, "B1", 0.0),
            alpha0: binned(p, "ALPHA0", 0.0),
            alpha1: binned(p, "ALPHA1", 0.0),
            beta0: binned(p, "BETA0", 0.0),
            agisl: binned_from(p, "AGISL", &agidl),
            bgisl: binned_from(p, "BGISL", &bgidl),
            cgisl: binned_from(p, "CGISL", &cgidl),
            egisl: binned_from(p, "EGISL", &egidl),
            rgisl: binned_from(p, "RGISL", &rgidl),
            kgisl: binned_from(p, "KGISL", &kgidl),
            fgisl: binned_from(p, "FGISL", &fgidl),
            agidl,
            bgidl,
            cgidl,
            egidl,
            rgidl,
            kgidl,
            fgidl,
            aigc: binned(p, "AIGC", ig_def(1.36e-2, 9.80e-3)),
            bigc: binned(p, "BIGC", ig_def(1.71e-3, 7.59e-4)),
            cigc: binned(p, "CIGC", ig_def(0.075, 0.03)),
            aigsd,
            bigsd,
            cigsd,
            aigs,
            bigs,
            cigs,
            aigd,
            bigd,
            cigd,
            aigbacc: binned(p, "AIGBACC", 1.36e-2),
            bigbacc: binned(p, "BIGBACC", 1.71e-3),
            cigbacc: binned(p, "CIGBACC", 0.075),
            aigbinv: binned(p, "AIGBINV", 1.11e-2),
            bigbinv: binned(p, "BIGBINV", 9.49e-4),
            cigbinv: binned(p, "CIGBINV", 0.006),
            nigc: binned(p, "NIGC", 1.0),
            nigbacc: binned(p, "NIGBACC", 1.0),
            nigbinv: binned(p, "NIGBINV", 3.0),
            ntox: binned(p, "NTOX", 1.0),
            eigbinv: binned(p, "EIGBINV", 1.1),
            pigcd: binned(p, "PIGCD", 1.0),
            pigcd_given: get(p, "PIGCD").is_some(),
            poxedge: binned(p, "POXEDGE", 1.0),
            xrcrg1: binned(p, "XRCRG1", 12.0),
            xrcrg2: binned(p, "XRCRG2", 1.0),
            lambda: binned(p, "LAMBDA", 0.0),
            lambda_given: get(p, "LAMBDA").is_some(),
            vtl: binned(p, "VTL", 2.0e5),
            vtl_given: get(p, "VTL").is_some(),
            xn: binned(p, "XN", 3.0),
            vfbsdoff: binned(p, "VFBSDOFF", 0.0),
            tvfbsdoff: binned(p, "TVFBSDOFF", 0.0),

            cgsl,
            cgdl,
            ckappas,
            ckappad,
            cf,
            clc: binned(p, "CLC", 0.1e-6),
            cle: binned(p, "CLE", 0.6),
            vfbcv: binned(p, "VFBCV", -1.0),
            acde: binned(p, "ACDE", 1.0),
            moin: binned(p, "MOIN", 15.0),
            noff: binned(p, "NOFF", 1.0),
            voffcv: binned(p, "VOFFCV", 0.0),

            kvth0we: binned(p, "KVTH0WE", 0.0),
            k2we: binned(p, "K2WE", 0.0),
            ku0we: binned(p, "KU0WE", 0.0),

            voffl: val(p, "VOFFL", 0.0),
            voffcvl: val(p, "VOFFCVL", 0.0),
            pditsl: val(p, "PDITSL", 0.0),
            rdswmin: val(p, "RDSWMIN", 0.0),
            rdwmin: val(p, "RDWMIN", 0.0),
            rswmin: val(p, "RSWMIN", 0.0),
            lc: val(p, "LC", 5.0e-9),
            gidlclamp: val(p, "GIDLCLAMP", -1.0e-5),
            idovvdsc: val(p, "IDOVVDSC", 1.0e-9),
            dlcig,
            dlcigd,

            ijthsfwd,
            ijthdfwd: val(p, "IJTHDFWD", ijthsfwd),
            ijthsrev,
            ijthdrev: val(p, "IJTHDREV", ijthsrev),
            xjbvs,
            xjbvd: val(p, "XJBVD", xjbvs),
            bvs,
            bvd: val(p, "BVD", bvs),
            jss,
            jsws,
            jswgs,
            pbs,
            njs,
            xtis,
            mjs,
            pbsws,
            mjsws,
            pbswgs,
            mjswgs,
            cjs,
            cjsws,
            cjswgs,
            jsd: val(p, "JSD", jss),
            jswd: val(p, "JSWD", jsws),
            jswgd: val(p, "JSWGD", jswgs),
            pbd: val(p, "PBD", pbs),
            njd: val(p, "NJD", njs),
            xtid: val(p, "XTID", xtis),
            mjd: val(p, "MJD", mjs),
            pbswd: val(p, "PBSWD", pbsws),
            mjswd: val(p, "MJSWD", mjsws),
            pbswgd: val(p, "PBSWGD", pbswgs),
            mjswgd: val(p, "MJSWGD", mjswgs),
            cjd: val(p, "CJD", cjs),
            cjswd: val(p, "CJSWD", cjsws),
            cjswgd: val(p, "CJSWGD", cjswgs),

            jtss,
            jtsd: val(p, "JTSD", jtss),
            jtssws,
            jtsswd: val(p, "JTSSWD", jtssws),
            jtsswgs,
            jtsswgd: val(p, "JTSSWGD", jtsswgs),
            jtweff: val(p, "JTWEFF", 0.0),
            njts,
            njtssw,
            njtsswg,
            njtsd: val(p, "NJTSD", njts),
            njtsswd: val(p, "NJTSSWD", njtssw),
            njtsswgd: val(p, "NJTSSWGD", njtsswg),
            xtss,
            xtsd: val(p, "XTSD", xtss),
            xtssws,
            xtsswd: val(p, "XTSSWD", xtssws),
            xtsswgs,
            xtsswgd: val(p, "XTSSWGD", xtsswgs),
            tnjts,
            tnjtssw,
            tnjtsswg,
            tnjtsd: val(p, "TNJTSD", tnjts),
            tnjtsswd: val(p, "TNJTSSWD", tnjtssw),
            tnjtsswgd: val(p, "TNJTSSWGD", tnjtsswg),
            vtss,
            vtsd: val(p, "VTSD", vtss),
            vtssws,
            vtsswd: val(p, "VTSSWD", vtssws),
            vtsswgs,
            vtsswgd: val(p, "VTSSWGD", vtsswgs),

            tpb: val(p, "TPB", 0.0),
            tcj: val(p, "TCJ", 0.0),
            tpbsw: val(p, "TPBSW", 0.0),
            tcjsw: val(p, "TCJSW", 0.0),
            tpbswg: val(p, "TPBSWG", 0.0),
            tcjswg: val(p, "TCJSWG", 0.0),

            dmcg,
            dmci: val(p, "DMCI", dmcg),
            dmdg: val(p, "DMDG", 0.0),
            dmcgt: val(p, "DMCGT", 0.0),
            xgw: val(p, "XGW", 0.0),
            xgl: val(p, "XGL", 0.0),
            rshg: val(p, "RSHG", 0.1),
            ngcon: val(p, "NGCON", 1.0),

            gbmin: val(p, "GBMIN", 1.0e-12),
            rbdb: val(p, "RBDB", 50.0),
            rbpb: val(p, "RBPB", 50.0),
            rbsb: val(p, "RBSB", 50.0),
            rbps: val(p, "RBPS", 50.0),
            rbpd: val(p, "RBPD", 50.0),
            rbps0: val(p, "RBPS0", 50.0),
            rbps0_given: p.contains_key("RBPS0"),
            rbpsl: val(p, "RBPSL", 0.0),
            rbpsw: val(p, "RBPSW", 0.0),
            rbpsnf: val(p, "RBPSNF", 0.0),
            rbpd0: val(p, "RBPD0", 50.0),
            rbpd0_given: p.contains_key("RBPD0"),
            rbpdl: val(p, "RBPDL", 0.0),
            rbpdw: val(p, "RBPDW", 0.0),
            rbpdnf: val(p, "RBPDNF", 0.0),
            rbpbx0: val(p, "RBPBX0", 100.0),
            rbpbxl: val(p, "RBPBXL", 0.0),
            rbpbxw: val(p, "RBPBXW", 0.0),
            rbpbxnf: val(p, "RBPBXNF", 0.0),
            rbpby0: val(p, "RBPBY0", 100.0),
            rbpbyl: val(p, "RBPBYL", 0.0),
            rbpbyw: val(p, "RBPBYW", 0.0),
            rbpbynf: val(p, "RBPBYNF", 0.0),
            rbsbx0: val(p, "RBSBX0", 100.0),
            rbsbx0_given: p.contains_key("RBSBX0"),
            rbsby0: val(p, "RBSBY0", 100.0),
            rbsby0_given: p.contains_key("RBSBY0"),
            rbdbx0: val(p, "RBDBX0", 100.0),
            rbdbx0_given: p.contains_key("RBDBX0"),
            rbdby0: val(p, "RBDBY0", 100.0),
            rbdby0_given: p.contains_key("RBDBY0"),
            rbsdbxl: val(p, "RBSDBXL", 0.0),
            rbsdbxw: val(p, "RBSDBXW", 0.0),
            rbsdbxnf: val(p, "RBSDBXNF", 0.0),
            rbsdbyl: val(p, "RBSDBYL", 0.0),
            rbsdbyw: val(p, "RBSDBYW", 0.0),
            rbsdbynf: val(p, "RBSDBYNF", 0.0),

            tnom,
            cgso,
            cgso_given,
            cgdo,
            cgdo_given,
            cgbo,
            cgbo_given,
            cf_given,
            xpart: val(p, "XPART", 0.0),
            sheet_resistance: val(p, "RSH", 0.0),

            lint,
            ll,
            llc: val(p, "LLC", ll),
            lln: val(p, "LLN", 1.0),
            lw,
            lwc: val(p, "LWC", lw),
            lwn: val(p, "LWN", 1.0),
            lwl,
            lwlc: val(p, "LWLC", lwl),
            lmin: val(p, "LMIN", 0.0),
            lmax: val(p, "LMAX", 1.0),
            wint,
            wl,
            wlc: val(p, "WLC", wl),
            wln: val(p, "WLN", 1.0),
            ww,
            wwc: val(p, "WWC", ww),
            wwn: val(p, "WWN", 1.0),
            wwl,
            wwlc: val(p, "WWLC", wwl),
            wmin: val(p, "WMIN", 0.0),
            wmax: val(p, "WMAX", 1.0),
            dwc,
            dlc,
            dlc_given,
            dwj: val(p, "DWJ", dwc),
            xl: val(p, "XL", 0.0),
            xw: val(p, "XW", 0.0),

            saref: val(p, "SAREF", 1.0e-6),
            sbref: val(p, "SBREF", 1.0e-6),
            wlod: val(p, "WLOD", 0.0),
            ku0: val(p, "KU0", 0.0),
            kvsat: val(p, "KVSAT", 0.0),
            kvth0: val(p, "KVTH0", 0.0),
            tku0: val(p, "TKU0", 0.0),
            llodku0: val(p, "LLODKU0", 0.0),
            wlodku0: val(p, "WLODKU0", 0.0),
            llodvth: val(p, "LLODVTH", 0.0),
            wlodvth: val(p, "WLODVTH", 0.0),
            lku0: val(p, "LKU0", 0.0),
            wku0: val(p, "WKU0", 0.0),
            pku0: val(p, "PKU0", 0.0),
            lkvth0: val(p, "LKVTH0", 0.0),
            wkvth0: val(p, "WKVTH0", 0.0),
            pkvth0: val(p, "PKVTH0", 0.0),
            stk2: val(p, "STK2", 0.0),
            lodk2: val(p, "LODK2", 1.0),
            steta0: val(p, "STETA0", 0.0),
            lodeta0: val(p, "LODETA0", 1.0),

            web: val(p, "WEB", 0.0),
            wec: val(p, "WEC", 0.0),
            scref: val(p, "SCREF", 1.0e-6),

            tnoia: val(p, "TNOIA", 1.5),
            tnoib: val(p, "TNOIB", 3.5),
            tnoic: val(p, "TNOIC", 0.0),
            rnoia: val(p, "RNOIA", 0.577),
            rnoib: val(p, "RNOIB", 0.5164),
            rnoic: val(p, "RNOIC", 0.395),
            ntnoi: val(p, "NTNOI", 1.0),
            oxide_trap_density_a: val(p, "NOIA", if nmos { 6.25e41 } else { 6.188e40 }),
            oxide_trap_density_b: val(p, "NOIB", if nmos { 3.125e26 } else { 1.5e25 }),
            oxide_trap_density_c: val(p, "NOIC", 8.75e9),
            em: val(p, "EM", 4.1e7),
            ef: val(p, "EF", 1.0),
            af: val(p, "AF", 1.0),
            kf: val(p, "KF", 0.0),
            lintnoi: val(p, "LINTNOI", 0.0),

            vgs_max: val(p, "VGS_MAX", 1.0e99),
            vgd_max: val(p, "VGD_MAX", 1.0e99),
            vgb_max: val(p, "VGB_MAX", 1.0e99),
            vds_max: val(p, "VDS_MAX", 1.0e99),
            vbs_max: val(p, "VBS_MAX", 1.0e99),
            vbd_max: val(p, "VBD_MAX", 1.0e99),
            vgsr_max: val(p, "VGSR_MAX", 1.0e99),
            vgdr_max: val(p, "VGDR_MAX", 1.0e99),
            vgbr_max: val(p, "VGBR_MAX", 1.0e99),
            vbsr_max: val(p, "VBSR_MAX", 1.0e99),
            vbdr_max: val(p, "VBDR_MAX", 1.0e99),
        })
    }

    /// Effective oxide relative permittivity used by `BSIM4coxe`.
    #[inline]
    pub fn effective_epsrox(&self) -> Value {
        if self.mtrl_mod != 0 { 3.9 } else { self.epsrox }
    }

    /// Effective oxide thickness used by `BSIM4coxe`.
    #[inline]
    pub fn effective_toxe(&self) -> Value {
        if self.mtrl_mod != 0 {
            self.eot
        } else {
            self.toxe
        }
    }

    /// `epsrox * EPS0 / toxe` (`BSIM4coxe`, material-aware).
    #[inline]
    pub fn coxe(&self) -> Value {
        self.effective_epsrox() * EPS0 / self.effective_toxe()
    }

    /// `epsrox * EPS0 / toxp` (`BSIM4coxp`) when model-level TOXP is used.
    #[inline]
    pub fn coxp(&self) -> Value {
        if self.mtrl_mod == 0 || self.mtrl_compat_mod != 0 {
            self.epsrox * EPS0 / self.toxp
        } else {
            self.coxe()
        }
    }
}
