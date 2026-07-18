//! BSIM3v3.3.0 model card: full parameter set with ngspice defaults.
//!
//! Parameter list transcribed from ngspice-46 `b3.c` (`BSIM3mPTable`),
//! defaults from `b3set.c` (`BSIM3setup`, lines 54-920). TNOM is converted
//! from Celsius to Kelvin on entry exactly as `b3mpar.c` does
//! (`BSIM3_MOD_TNOM: value + CONSTCtoK`).

use super::common::{EPSOX, PI};
use crate::Value;
use std::collections::HashMap;

/// Canonical BSIM3 bulk-MOS equation family selected by the simulator front.
///
/// Xyce 7.10's `MOSFET_B3` device is the BSIM3 3.2.2 implementation for both
/// levels 9 and 49.  Ngspice's `bsim3` device is BSIM3 3.3.0.  A model-card
/// `VERSION` value is metadata in Xyce and must therefore not silently select
/// a different set of equations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Bsim3v3EquationSet {
    /// Xyce 7.10 `MOSFET_B3`, BSIM3 3.2.2 equations.
    XyceV322,
    /// Ngspice 46 `bsim3`, BSIM3 3.3.0 equations.
    NgspiceV330,
}

impl Bsim3v3EquationSet {
    #[inline]
    pub(crate) fn default_version(self) -> &'static str {
        match self {
            Self::XyceV322 => "3.2.2",
            Self::NgspiceV330 => "3.3.0",
        }
    }
}

/// Binned model parameter: nominal value plus L/W/cross-term dependence.
///
/// `b3temp.c` evaluates every binned family as
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

/// Full BSIM3v3.3 `.model` card (one per model statement, shared by instances).
///
/// Field order follows the `sBSIM3model` struct in `bsim3def.h`. The SOA
/// voltage limits (`vgsMax` family) are accepted but not enforced here; the
/// noise-model card (`noia`/`noib`/`noic`/`em`/`ef`/`af`/`kf`/`lintnoi`) is
/// consumed by the engine's `.NOISE` source collector.
#[derive(Debug, Clone)]
pub struct Bsim3v3Model {
    /// Equations implemented by this model instance. This is simulator-front
    /// semantics, not user-editable model-card metadata.
    pub equation_set: Bsim3v3EquationSet,

    /// +1.0 for NMOS, -1.0 for PMOS (ngspice `BSIM3type`).
    pub mtype: Value,

    pub mob_mod: i32,
    pub cap_mod: i32,
    pub acm_mod: i32,
    pub acm_mod_value: Value,
    pub calcacm: i32,
    pub noi_mod: i32,
    pub nqs_mod: i32,
    pub acnqs_mod: i32,
    pub bin_unit: i32,
    pub param_chk: i32,
    pub version: String,
    pub tox: Value,
    pub toxm: Value,

    // Binned families (defaults per b3set.c; b3temp.c order).
    pub cdsc: Binned,
    pub cdscb: Binned,
    pub cdscd: Binned,
    pub cit: Binned,
    pub nfactor: Binned,
    pub xj: Binned,
    pub vsat: Binned,
    pub at: Binned,
    pub a0: Binned,
    pub ags: Binned,
    pub a1: Binned,
    pub a2: Binned,
    pub keta: Binned,
    pub nsub: Binned,
    pub npeak: Binned,
    pub npeak_given: bool,
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
    pub nlx: Binned,
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
    pub u0: Binned,
    pub ute: Binned,
    pub voff: Binned,
    pub delta: Binned,
    pub rdsw: Binned,
    pub prwg: Binned,
    pub prwb: Binned,
    pub prt: Binned,
    pub eta0: Binned,
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

    // C-V model binned families.
    pub elm: Binned,
    pub cgsl: Binned,
    pub cgdl: Binned,
    pub ckappa: Binned,
    pub cf: Binned,
    pub clc: Binned,
    pub cle: Binned,
    pub vfbcv: Binned,
    pub noff: Binned,
    pub voffcv: Binned,
    pub acde: Binned,
    pub moin: Binned,

    // Non-binned scalars.
    pub ijth: Value,
    pub tcj: Value,
    pub tcjsw: Value,
    pub tcjswg: Value,
    pub tpb: Value,
    pub tpbsw: Value,
    pub tpbswg: Value,

    // ACM model scalars (ACM=1 is native; higher ACM families stay fail-closed).
    pub xl: Value,
    pub xw: Value,
    pub hdif: Value,
    pub ldif: Value,
    pub ld: Value,
    pub rd: Value,
    pub rs: Value,
    pub rdc: Value,
    pub rsc: Value,
    pub wmlt: Value,

    /// TNOM in Kelvin (card value is Celsius, transformed on entry).
    pub tnom: Value,
    pub cgso: Value,
    pub cgdo: Value,
    pub cgbo: Value,
    pub xpart: Value,

    pub sheet_resistance: Value,
    pub jct_sat_cur_density: Value,
    pub jct_sidewall_sat_cur_density: Value,
    pub bulk_jct_potential: Value,
    pub bulk_jct_bot_grading_coeff: Value,
    pub bulk_jct_side_grading_coeff: Value,
    pub bulk_jct_gate_side_grading_coeff: Value,
    pub sidewall_jct_potential: Value,
    pub gate_sidewall_jct_potential: Value,
    pub unit_area_jct_cap: Value,
    pub unit_length_sidewall_jct_cap: Value,
    pub unit_length_gate_sidewall_jct_cap: Value,
    pub jct_emission_coeff: Value,
    pub jct_temp_exponent: Value,

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

    // Flicker/thermal noise card (stored; the noise analyses are out of scope
    // for the load port).
    pub oxide_trap_density_a: Value,
    pub oxide_trap_density_b: Value,
    pub oxide_trap_density_c: Value,
    pub em: Value,
    pub ef: Value,
    pub af: Value,
    pub kf: Value,
    pub lintnoi: Value,

    // Derived constant (b3set.c line 87).
    pub cox: Value,
}

#[inline]
fn get(map: &HashMap<String, Value>, key: &str) -> Option<Value> {
    map.get(key).copied().filter(|v| v.is_finite())
}

#[inline]
fn val(map: &HashMap<String, Value>, key: &str, default: Value) -> Value {
    get(map, key).unwrap_or(default)
}

/// Fetch a binned family: `name`, `l<name>`, `w<name>`, `p<name>`.
fn binned(map: &HashMap<String, Value>, name: &str, v_def: Value) -> Binned {
    Binned {
        v: val(map, name, v_def),
        l: val(map, &format!("L{name}"), 0.0),
        w: val(map, &format!("W{name}"), 0.0),
        p: val(map, &format!("P{name}"), 0.0),
    }
}

fn selector_values(allowed: &[i32]) -> String {
    allowed
        .iter()
        .map(i32::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Fetch a native model-card selector. Omitted selectors default like b3set.c;
/// explicit values fail closed rather than being truncated or reset, except
/// for the handful of selectors with an oracle-backed ngspice reset rule.
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
    if !raw.is_finite() || raw.trunc() != raw || raw < i32::MIN as Value || raw > i32::MAX as Value
    {
        return Err(format!(
            "BSIM3 selector {key} must be a finite integer (supported values: {values}); got {key}={raw}"
        ));
    }
    let value = raw as i32;
    if allowed.contains(&value) {
        Ok(value)
    } else {
        Err(format!(
            "BSIM3 selector {key} must be a finite integer in the supported set ({values}); got {key}={value}"
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
    if !raw.is_finite() {
        return Err(format!(
            "BSIM3 selector {key} must be finite (supported values: {values}); got {key}={raw}"
        ));
    }
    let rounded = (raw + 0.5).floor();
    if rounded < i32::MIN as Value || rounded > i32::MAX as Value {
        return Err(format!(
            "BSIM3 selector {key} must be convertible to an integer (supported values: {values}); got {key}={raw}"
        ));
    }
    let value = rounded as i32;
    if allowed.contains(&value) {
        Ok(value)
    } else {
        Ok(default)
    }
}

impl Bsim3v3Model {
    /// Build a model card from an uppercase-keyed parameter map.
    ///
    /// `is_pmos` selects the device polarity, `nominal_temp_k` supplies the
    /// circuit nominal temperature in Kelvin (ngspice `CKTnomTemp`) used when
    /// `TNOM` is absent (TNOM on the card is in Celsius).
    pub fn from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Self {
        Self::try_from_params(params, is_pmos, nominal_temp_k)
            .expect("valid BSIM3 v3.3 model parameters")
    }

    /// Fallible model-card constructor used at user-facing build boundaries.
    pub fn try_from_params(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
    ) -> Result<Self, String> {
        Self::try_from_params_with_equation_set(
            params,
            is_pmos,
            nominal_temp_k,
            Bsim3v3EquationSet::NgspiceV330,
        )
    }

    /// Construct a model card for a specific canonical simulator equation
    /// family. Public programmatic callers retain ngspice-3.3 behavior through
    /// [`Self::try_from_params`]; compatibility fronts call this explicitly.
    pub fn try_from_params_with_equation_set(
        params: &HashMap<String, Value>,
        is_pmos: bool,
        nominal_temp_k: Value,
        equation_set: Bsim3v3EquationSet,
    ) -> Result<Self, String> {
        let p = params;
        let mtype: Value = if is_pmos { -1.0 } else { 1.0 };
        let geometry_exponent_default = match equation_set {
            Bsim3v3EquationSet::XyceV322 => 0.0,
            Bsim3v3EquationSet::NgspiceV330 => 1.0,
        };

        let mob_mod = selector(p, "MOBMOD", 1, &[1, 2, 3])?;
        let cap_mod = selector(p, "CAPMOD", 3, &[0, 1, 2, 3])?;
        let acm_mod = selector(p, "ACM", 0, &[0, 1])?;
        let calcacm = selector(p, "CALCACM", 0, &[0, 1])?;
        let noi_mod = selector(p, "NOIMOD", 1, &[1, 2, 3, 4, 5, 6])?;
        let nqs_mod = selector_reset_to_default(p, "NQSMOD", 0, &[0, 1])?;
        let acnqs_mod = selector_reset_to_default(p, "ACNQSMOD", 0, &[0, 1])?;
        let bin_unit = selector(p, "BINUNIT", 1, &[1, 2])?;
        let param_chk = selector(p, "PARAMCHK", 0, &[0, 1])?;
        let tox = val(p, "TOX", 150.0e-10);
        let cox = 3.453133e-11 / tox;

        // dsub defaults to drout (the nominal value only; bins default to 0).
        let drout = binned(p, "DROUT", 0.56);
        let dsub = binned(p, "DSUB", drout.v);

        let xj = binned(p, "XJ", 0.15e-6);
        let cgsl = binned(p, "CGSL", 0.0);
        let cgdl = binned(p, "CGDL", 0.0);

        let lint = val(p, "LINT", 0.0);
        let wint = val(p, "WINT", 0.0);
        let ll = val(p, "LL", 0.0);
        let lw = val(p, "LW", 0.0);
        let lwl = val(p, "LWL", 0.0);
        let wl = val(p, "WL", 0.0);
        let ww = val(p, "WW", 0.0);
        let wwl = val(p, "WWL", 0.0);
        let dwc = val(p, "DWC", wint);
        let dlc_given = get(p, "DLC").is_some();
        let dlc = val(p, "DLC", lint);

        // cf default depends on tox (b3set.c:818-820).
        let cf = Binned {
            v: get(p, "CF").unwrap_or_else(|| 2.0 * EPSOX / PI * (1.0 + 0.4e-6 / tox).ln()),
            l: val(p, "LCF", 0.0),
            w: val(p, "WCF", 0.0),
            p: val(p, "PCF", 0.0),
        };
        // cgdo/cgso defaults use the model-level dlc and cgdl/cgsl
        // (b3set.c:821-836).
        let cgdo = get(p, "CGDO").unwrap_or(if dlc_given && dlc > 0.0 {
            dlc * cox - cgdl.v
        } else {
            0.6 * xj.v * cox
        });
        let cgso = get(p, "CGSO").unwrap_or(if dlc_given && dlc > 0.0 {
            dlc * cox - cgsl.v
        } else {
            0.6 * xj.v * cox
        });
        let cgbo = get(p, "CGBO").unwrap_or(2.0 * dwc * cox);

        // Junction default chains (b3set.c:845-870).
        let unit_length_sidewall_jct_cap = val(p, "CJSW", 5.0e-10);
        let sidewall_jct_potential = val(p, "PBSW", 1.0);
        let bulk_jct_side_grading_coeff = val(p, "MJSW", 0.33);

        // TNOM on the card is Celsius; CKTnomTemp is Kelvin (b3mpar.c:1517).
        let tnom = get(p, "TNOM").map(|t| t + 273.15).unwrap_or(nominal_temp_k);

        Ok(Self {
            equation_set,
            mtype,
            mob_mod,
            cap_mod,
            acm_mod,
            acm_mod_value: val(p, "ACM", 0.0),
            calcacm,
            noi_mod,
            nqs_mod,
            acnqs_mod,
            bin_unit,
            param_chk,
            // The C stores version as a string; the numeric-card map can only
            // carry numbers, so a numeric VERSION is rendered back (the model
            // itself never branches on it — b3check.c only warns on != 3.3*).
            version: get(p, "VERSION")
                .map(|v| format!("{v}"))
                .unwrap_or_else(|| equation_set.default_version().to_string()),
            tox,
            toxm: val(p, "TOXM", tox),

            cdsc: binned(p, "CDSC", 2.4e-4),
            cdscb: binned(p, "CDSCB", 0.0),
            cdscd: binned(p, "CDSCD", 0.0),
            cit: binned(p, "CIT", 0.0),
            nfactor: binned(p, "NFACTOR", 1.0),
            xj,
            vsat: binned(p, "VSAT", 8.0e4),
            at: binned(p, "AT", 3.3e4),
            a0: binned(p, "A0", 1.0),
            ags: binned(p, "AGS", 0.0),
            a1: binned(p, "A1", 0.0),
            a2: binned(p, "A2", 1.0),
            keta: binned(p, "KETA", -0.047),
            nsub: binned(p, "NSUB", 6.0e16),
            npeak: {
                let mut b = binned(p, "NCH", 1.7e17);
                // The L/W/P bins of nch are spelled lnch/wnch/pnch in b3.c.
                b.l = val(p, "LNCH", 0.0);
                b.w = val(p, "WNCH", 0.0);
                b.p = val(p, "PNCH", 0.0);
                b
            },
            npeak_given: get(p, "NCH").is_some(),
            ngate: binned(p, "NGATE", 0.0),
            gamma1: binned(p, "GAMMA1", 0.0),
            gamma1_given: get(p, "GAMMA1").is_some(),
            gamma2: binned(p, "GAMMA2", 0.0),
            gamma2_given: get(p, "GAMMA2").is_some(),
            vbx: binned(p, "VBX", 0.0),
            vbx_given: get(p, "VBX").is_some(),
            vbm: binned(p, "VBM", -3.0),
            xt: binned(p, "XT", 1.55e-7),
            vfb: binned(p, "VFB", 0.0),
            vfb_given: get(p, "VFB").is_some(),
            // k1/k2 nominal values default to 0.0 in ngspice (calloc'd model
            // struct); when neither is given they are recomputed from
            // gamma1/gamma2 in the temp setup (b3temp.c:652-706).
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
            nlx: binned(p, "NLX", 1.74e-7),
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
                    .unwrap_or(if is_pmos { -0.7 } else { 0.7 }),
                l: get(p, "LVTH0").or_else(|| get(p, "LVTHO")).unwrap_or(0.0),
                w: get(p, "WVTH0").or_else(|| get(p, "WVTHO")).unwrap_or(0.0),
                p: get(p, "PVTH0").or_else(|| get(p, "PVTHO")).unwrap_or(0.0),
            },
            vth0_given: get(p, "VTH0").or_else(|| get(p, "VTHO")).is_some(),
            ua: binned(p, "UA", 2.25e-9),
            ua1: binned(p, "UA1", 4.31e-9),
            ub: binned(p, "UB", 5.87e-19),
            ub1: binned(p, "UB1", -7.61e-18),
            uc: binned(p, "UC", if mob_mod == 3 { -0.0465 } else { -0.0465e-9 }),
            uc1: binned(p, "UC1", if mob_mod == 3 { -0.056 } else { -0.056e-9 }),
            u0: binned(p, "U0", if is_pmos { 0.025 } else { 0.067 }),
            ute: binned(p, "UTE", -1.5),
            voff: binned(p, "VOFF", -0.08),
            delta: binned(p, "DELTA", 0.01),
            rdsw: binned(p, "RDSW", 0.0),
            prwg: binned(p, "PRWG", 0.0),
            prwb: binned(p, "PRWB", 0.0),
            prt: binned(p, "PRT", 0.0),
            eta0: binned(p, "ETA0", 0.08),
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
            beta0: binned(p, "BETA0", 30.0),

            elm: binned(p, "ELM", 5.0),
            cgsl,
            cgdl,
            ckappa: binned(p, "CKAPPA", 0.6),
            cf,
            clc: binned(p, "CLC", 0.1e-6),
            cle: binned(p, "CLE", 0.6),
            vfbcv: binned(p, "VFBCV", -1.0),
            noff: binned(p, "NOFF", 1.0),
            voffcv: binned(p, "VOFFCV", 0.0),
            acde: binned(p, "ACDE", 1.0),
            moin: binned(p, "MOIN", 15.0),

            ijth: val(p, "IJTH", 0.1),
            tcj: val(p, "TCJ", 0.0),
            tcjsw: val(p, "TCJSW", 0.0),
            tcjswg: val(p, "TCJSWG", 0.0),
            tpb: val(p, "TPB", 0.0),
            tpbsw: val(p, "TPBSW", 0.0),
            tpbswg: val(p, "TPBSWG", 0.0),

            xl: val(p, "XL", 0.0),
            xw: val(p, "XW", 0.0),
            hdif: val(p, "HDIF", 0.0),
            ldif: val(p, "LDIF", 0.0),
            ld: val(p, "LD", 0.0),
            rd: val(p, "RD", 0.0),
            rs: val(p, "RS", 0.0),
            rdc: val(p, "RDC", 0.0),
            rsc: val(p, "RSC", 0.0),
            wmlt: val(p, "WMLT", 1.0),

            tnom,
            cgso,
            cgdo,
            cgbo,
            xpart: val(p, "XPART", 0.0),

            sheet_resistance: val(p, "RSH", 0.0),
            jct_sat_cur_density: val(p, "JS", 1.0e-4),
            jct_sidewall_sat_cur_density: val(p, "JSW", 0.0),
            bulk_jct_potential: val(p, "PB", 1.0),
            bulk_jct_bot_grading_coeff: val(p, "MJ", 0.5),
            bulk_jct_side_grading_coeff,
            bulk_jct_gate_side_grading_coeff: val(p, "MJSWG", bulk_jct_side_grading_coeff),
            sidewall_jct_potential,
            gate_sidewall_jct_potential: val(p, "PBSWG", sidewall_jct_potential),
            unit_area_jct_cap: val(p, "CJ", 5.0e-4),
            unit_length_sidewall_jct_cap,
            unit_length_gate_sidewall_jct_cap: val(p, "CJSWG", unit_length_sidewall_jct_cap),
            jct_emission_coeff: val(p, "NJ", 1.0),
            jct_temp_exponent: val(p, "XTI", 3.0),

            lint,
            ll,
            llc: val(p, "LLC", ll),
            lln: val(p, "LLN", geometry_exponent_default),
            lw,
            lwc: val(p, "LWC", lw),
            lwn: val(p, "LWN", geometry_exponent_default),
            lwl,
            lwlc: val(p, "LWLC", lwl),
            lmin: val(p, "LMIN", 0.0),
            lmax: val(p, "LMAX", 1.0),
            wint,
            wl,
            wlc: val(p, "WLC", wl),
            wln: val(p, "WLN", geometry_exponent_default),
            ww,
            wwc: val(
                p,
                "WWC",
                match equation_set {
                    // Xyce MOSFET_B3 leaves the independently registered WWC
                    // parameter at its zero default when omitted.
                    Bsim3v3EquationSet::XyceV322 => 0.0,
                    // ngspice BSIM3setup aliases the omitted CV correction to
                    // the DC width-correction coefficient.
                    Bsim3v3EquationSet::NgspiceV330 => ww,
                },
            ),
            wwn: val(p, "WWN", geometry_exponent_default),
            wwl,
            wwlc: val(p, "WWLC", wwl),
            wmin: val(p, "WMIN", 0.0),
            wmax: val(p, "WMAX", 1.0),
            dwc,
            dlc,
            dlc_given,

            oxide_trap_density_a: val(p, "NOIA", if is_pmos { 9.9e18 } else { 1.0e20 }),
            oxide_trap_density_b: val(p, "NOIB", if is_pmos { 2.4e3 } else { 5.0e4 }),
            oxide_trap_density_c: val(p, "NOIC", if is_pmos { 1.4e-12 } else { -1.4e-12 }),
            em: val(p, "EM", 4.1e7),
            ef: val(p, "EF", 1.0),
            af: val(p, "AF", 1.0),
            kf: val(p, "KF", 0.0),
            lintnoi: val(p, "LINTNOI", 0.0),

            cox,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equation_families_apply_their_canonical_empty_card_defaults() {
        let params = HashMap::new();
        let xyce = Bsim3v3Model::try_from_params_with_equation_set(
            &params,
            false,
            300.15,
            Bsim3v3EquationSet::XyceV322,
        )
        .expect("Xyce BSIM3 3.2.2 defaults resolve");
        let ngspice = Bsim3v3Model::try_from_params_with_equation_set(
            &params,
            false,
            300.15,
            Bsim3v3EquationSet::NgspiceV330,
        )
        .expect("ngspice BSIM3 3.3 defaults resolve");

        assert_eq!(xyce.equation_set, Bsim3v3EquationSet::XyceV322);
        assert_eq!(xyce.version, "3.2.2");
        assert_eq!(
            (xyce.lln, xyce.lwn, xyce.wln, xyce.wwn),
            (0.0, 0.0, 0.0, 0.0)
        );
        assert_eq!(xyce.wwc, 0.0);

        assert_eq!(ngspice.equation_set, Bsim3v3EquationSet::NgspiceV330);
        assert_eq!(ngspice.version, "3.3.0");
        assert_eq!(
            (ngspice.lln, ngspice.lwn, ngspice.wln, ngspice.wwn),
            (1.0, 1.0, 1.0, 1.0)
        );
        assert_eq!(ngspice.wwc, 0.0);
    }

    #[test]
    fn equation_specific_wwc_fallback_does_not_cross_simulator_fronts() {
        let params = HashMap::from([("WW".to_string(), 2.5e-8)]);
        let xyce = Bsim3v3Model::try_from_params_with_equation_set(
            &params,
            false,
            300.15,
            Bsim3v3EquationSet::XyceV322,
        )
        .expect("Xyce card resolves");
        let ngspice = Bsim3v3Model::try_from_params_with_equation_set(
            &params,
            false,
            300.15,
            Bsim3v3EquationSet::NgspiceV330,
        )
        .expect("ngspice card resolves");

        assert_eq!(xyce.wwc, 0.0);
        assert_eq!(ngspice.wwc, 2.5e-8);
    }
}
