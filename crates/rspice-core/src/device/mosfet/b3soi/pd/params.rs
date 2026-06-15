//! B3SOIPD model card: full parameter set with ngspice defaults.
//!
//! Parameter list transcribed from ngspice-46 `b3soipd.c` (`B3SOIPDmPTable`),
//! defaults from `b3soipdset.c` (`B3SOIPDsetup`, lines 52-873).

use super::super::common::{EPSOX, PI};
use crate::Value;
use std::collections::HashMap;

/// Binned model parameter: nominal value plus L/W/cross-term dependence.
///
/// `b3soipdtemp.c` evaluates every binned family as
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

/// Full B3SOIPD `.model` card (one per model statement, shared by instances).
#[derive(Debug, Clone)]
pub struct B3SoiPdModel {
    /// +1.0 for NMOS, -1.0 for PMOS (ngspice `B3SOIPDtype`).
    pub mtype: Value,

    pub mob_mod: i32,
    pub cap_mod: i32,
    pub sh_mod: i32,
    pub bin_unit: i32,
    pub param_chk: i32,
    pub version: Value,
    pub tox: Value,

    // Binned families (defaults per b3soipdset.c).
    pub npeak: Binned,
    pub npeak_given: bool,
    pub nsub: Binned,
    pub ngate: Binned,
    pub vth0: Binned,
    pub vth0_given: bool,
    pub k1: Binned,
    pub k1_given: bool,
    pub k2: Binned,
    pub k2_given: bool,
    pub k3: Binned,
    pub k3b: Binned,
    pub vbsa: Binned,
    pub delp: Binned,
    pub kb1: Binned,
    pub kb3: Binned,
    pub dvbd0: Binned,
    pub dvbd1: Binned,
    pub w0: Binned,
    pub nlx: Binned,
    pub dvt0: Binned,
    pub dvt1: Binned,
    pub dvt2: Binned,
    pub dvt0w: Binned,
    pub dvt1w: Binned,
    pub dvt2w: Binned,
    pub u0: Binned,
    pub ua: Binned,
    pub ub: Binned,
    pub uc: Binned,
    pub vsat: Binned,
    pub a0: Binned,
    pub ags: Binned,
    pub b0: Binned,
    pub b1: Binned,
    pub keta: Binned,
    /// PD bulk-charge body-bias coefficient for Abulk (`ketas`, b3soipdld.c:1207).
    pub ketas: Binned,
    pub abp: Binned,
    pub mxc: Binned,
    pub adice0: Binned,
    pub a1: Binned,
    pub a2: Binned,
    pub rdsw: Binned,
    pub prwb: Binned,
    pub prwg: Binned,
    pub wr: Binned,
    pub nfactor: Binned,
    pub dwg: Binned,
    pub dwb: Binned,
    pub voff: Binned,
    pub eta0: Binned,
    pub etab: Binned,
    pub dsub: Binned,
    pub cit: Binned,
    pub cdsc: Binned,
    pub cdscb: Binned,
    pub cdscd: Binned,
    pub pclm: Binned,
    pub pdibl1: Binned,
    pub pdibl2: Binned,
    pub pdiblb: Binned,
    pub drout: Binned,
    pub pvag: Binned,
    pub delta: Binned,
    pub aii: Binned,
    pub bii: Binned,
    pub cii: Binned,
    pub dii: Binned,
    pub alpha0: Binned,
    pub alpha1: Binned,
    pub beta0: Binned,
    // PD impact-ionization (b3soipdld.c:2534-2620) binned families.
    pub beta1: Binned,
    pub beta2: Binned,
    pub vdsatii0: Binned,
    pub esatii: Binned,
    pub sii0: Binned,
    pub sii1: Binned,
    pub sii2: Binned,
    pub siid: Binned,
    pub fbjtii: Binned,
    pub lii: Value,
    pub tii: Value,
    // PD body-diode / parasitic-BJT (b3soipdld.c:1862-2200) binned families.
    pub nrecf0: Binned,
    pub nrecr0: Binned,
    pub ahli: Binned,
    pub lbjt0: Binned,
    pub vabjt: Binned,
    pub aely: Binned,
    pub vrec0: Value,
    pub vtun0: Value,
    pub ntrecf: Value,
    pub ntrecr: Value,
    pub ln: Value,
    pub nbjt: Value,
    pub ndif: Value,
    pub ldif0: Value,
    pub agidl: Binned,
    pub bgidl: Binned,
    pub ngidl: Binned,
    pub ntun: Binned,
    pub ndiode: Binned,
    pub isbjt: Binned,
    pub isdif: Binned,
    pub isrec: Binned,
    pub istun: Binned,
    pub edl: Binned,
    pub kbjt1: Binned,
    pub vsdfb: Binned,
    pub vsdfb_given: bool,
    pub vsdth: Binned,
    pub vsdth_given: bool,

    // Non-binned DC/temperature parameters.
    /// PD body-tie width correction coefficients for k1eff (b3soipdtemp.c:829).
    pub k1w1: Value,
    pub k1w2: Value,
    pub at: Value,
    pub gamma1: Value,
    pub gamma1_given: bool,
    pub gamma2: Value,
    pub gamma2_given: bool,
    pub vbx: Value,
    pub vbx_given: bool,
    pub vbm: Value,
    pub xt: Value,
    pub kt1: Value,
    pub kt1l: Value,
    pub kt2: Value,
    pub ua1: Value,
    pub ub1: Value,
    pub uc1: Value,
    pub ute: Value,
    pub prt: Value,

    // SOI-specific structure parameters.
    pub tbox: Value,
    pub tsi: Value,
    pub xj: Value,
    pub rbody: Value,
    pub rbsh: Value,
    pub rth0: Value,
    pub cth0: Value,
    pub xbjt: Value,
    pub xdif: Value,
    pub xrec: Value,
    pub xtun: Value,
    pub tt: Value,
    pub asd: Value,
    pub csdmin: Value,
    pub csdmin_given: bool,

    // C-V model.
    pub cgsl: Value,
    pub cgdl: Value,
    pub ckappa: Value,
    pub cf: Value,
    pub clc: Value,
    pub cle: Value,
    pub dwc: Value,
    pub dlc: Value,
    pub dlc_given: bool,
    /// Channel-length reduction for the body CV length `leffCVb`
    /// (b3soipdset.c:420-421, default 0).
    pub dlcb: Value,
    /// Back-gate CV length extension for `leffCVbg` (b3soipdset.c:436-437,
    /// default 0).
    pub dlbg: Value,
    /// Body-charge scaling factor (b3soipdset.c:422-423, default 1).
    pub fbody: Value,
    pub cgso: Value,
    pub cgdo: Value,
    pub cgeo: Value,
    pub xpart: Value,

    pub tnom: Value,
    pub sheet_resistance: Value,
    pub body_jct_gate_side_grading_coeff: Value,
    pub gate_sidewall_jct_potential: Value,
    pub unit_length_gate_sidewall_jct_cap: Value,
    pub csdesw: Value,

    // Geometry scaling.
    pub lint: Value,
    pub ll: Value,
    pub lln: Value,
    pub lw: Value,
    pub lwn: Value,
    pub lwl: Value,
    pub wint: Value,
    pub wl: Value,
    pub wln: Value,
    pub ww: Value,
    pub wwn: Value,
    pub wwl: Value,

    // Derived constants (computed in `finish_setup`, ngspice b3soipdset.c
    // lines 970-997).
    pub cox: Value,
    pub cbox: Value,
    pub csi: Value,
    pub csieff: Value,
    pub qsi: Value,
    pub qsieff: Value,
    pub csit: Value,
    pub cboxt: Value,
    pub nfb: Value,
    pub adice: Value,
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
/// `alias` allows the secondary spelling used by Berkeley (e.g. `NCH`).
fn binned(
    map: &HashMap<String, Value>,
    name: &str,
    v_def: Value,
    l_def: Value,
    w_def: Value,
    p_def: Value,
) -> Binned {
    Binned {
        v: val(map, name, v_def),
        l: val(map, &format!("L{name}"), l_def),
        w: val(map, &format!("W{name}"), w_def),
        p: val(map, &format!("P{name}"), p_def),
    }
}

impl B3SoiPdModel {
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
        let p = params;
        let mtype: Value = if is_pmos { -1.0 } else { 1.0 };

        let mob_mod = val(p, "MOBMOD", 1.0) as i32;
        let tox = val(p, "TOX", 100.0e-10);
        let cox = 3.453133e-11 / tox;

        let drout = binned(p, "DROUT", 0.56, 0.0, 0.0, 0.0);
        // dsub defaults to drout (the nominal value only; bins default to 0).
        let dsub = binned(p, "DSUB", drout.v, 0.0, 0.0, 0.0);

        let tsi = val(p, "TSI", 1e-7);
        let xj = val(p, "XJ", tsi);

        let cgsl = val(p, "CGSL", 0.0);
        let cgdl = val(p, "CGDL", 0.0);
        let lint = val(p, "LINT", 0.0);
        let wint = val(p, "WINT", 0.0);
        let dlc_given = get(p, "DLC").is_some();
        let dlc = val(p, "DLC", lint);
        let dwc = val(p, "DWC", wint);

        let cf = get(p, "CF").unwrap_or_else(|| 2.0 * EPSOX / PI * (1.0 + 0.4e-6 / tox).ln());
        let cgdo = get(p, "CGDO").unwrap_or(if dlc_given && dlc > 0.0 {
            dlc * cox - cgdl
        } else {
            0.6 * xj * cox
        });
        let cgso = get(p, "CGSO").unwrap_or(if dlc_given && dlc > 0.0 {
            dlc * cox - cgsl
        } else {
            0.6 * xj * cox
        });

        // NOTE (faithful to ngspice): the IF-parameter table maps "xdif" onto
        // the XBJT slot (`IOPR("xdif", B3SOIPD_MOD_XBJT, ...)` in b3soipd.c),
        // so a model card XDIF actually overwrites xbjt and the internal xdif
        // keeps its default of 2.0.
        let xbjt = get(p, "XDIF").or_else(|| get(p, "XBJT")).unwrap_or(2.0);

        // TNOM on the card is Celsius; CKTnomTemp is Kelvin.
        let tnom = get(p, "TNOM").map(|t| t + 273.15).unwrap_or(nominal_temp_k);

        let mut model = Self {
            mtype,
            mob_mod,
            cap_mod: val(p, "CAPMOD", 2.0) as i32,
            sh_mod: val(p, "SHMOD", 0.0) as i32,
            bin_unit: val(p, "BINUNIT", 1.0) as i32,
            param_chk: val(p, "PARAMCHK", 0.0) as i32,
            version: val(p, "VERSION", 2.0),
            tox,

            npeak: binned(p, "NCH", 1.7e17, 0.0, 0.0, 0.0),
            npeak_given: get(p, "NCH").is_some(),
            nsub: binned(p, "NSUB", 6.0e16, 0.0, 0.0, 0.0),
            ngate: binned(p, "NGATE", 0.0, 0.0, 0.0, 0.0),
            vth0: Binned {
                v: get(p, "VTH0")
                    .or_else(|| get(p, "VTHO"))
                    .unwrap_or(if is_pmos { -0.7 } else { 0.7 }),
                l: val(p, "LVTH0", 0.0),
                w: val(p, "WVTH0", 0.0),
                p: val(p, "PVTH0", 0.0),
            },
            vth0_given: get(p, "VTH0").or_else(|| get(p, "VTHO")).is_some(),
            // k1/k2 nominal values default to 0.0 in ngspice (calloc'd model
            // struct); when neither is given they are recomputed from
            // gamma1/gamma2 in the temp setup.
            k1: binned(p, "K1", 0.0, 0.0, 0.0, 0.0),
            k1_given: get(p, "K1").is_some(),
            k2: binned(p, "K2", 0.0, 0.0, 0.0, 0.0),
            k2_given: get(p, "K2").is_some(),
            k3: binned(p, "K3", 0.0, 0.0, 0.0, 0.0),
            k3b: binned(p, "K3B", 0.0, 0.0, 0.0, 0.0),
            vbsa: binned(p, "VBSA", 0.0, 0.0, 0.0, 0.0),
            delp: binned(p, "DELP", 0.02, 0.0, 0.0, 0.0),
            kb1: binned(p, "KB1", 1.0, 0.0, 0.0, 0.0),
            // NOTE: lkb3/wkb3/pkb3 default to 1.0 in b3soipdset.c (lines
            // 351-352, 511-512, 671-672) — transcribed verbatim.
            kb3: binned(p, "KB3", 1.0, 1.0, 1.0, 1.0),
            // NOTE: ldvbd0/ldvbd1 (and w/p variants) default to 1.0 as well.
            dvbd0: binned(p, "DVBD0", 0.0, 1.0, 1.0, 1.0),
            dvbd1: binned(p, "DVBD1", 0.0, 1.0, 1.0, 1.0),
            w0: binned(p, "W0", 2.5e-6, 0.0, 0.0, 0.0),
            nlx: binned(p, "NLX", 1.74e-7, 0.0, 0.0, 0.0),
            dvt0: binned(p, "DVT0", 2.2, 0.0, 0.0, 0.0),
            dvt1: binned(p, "DVT1", 0.53, 0.0, 0.0, 0.0),
            dvt2: binned(p, "DVT2", -0.032, 0.0, 0.0, 0.0),
            dvt0w: binned(p, "DVT0W", 0.0, 0.0, 0.0, 0.0),
            dvt1w: binned(p, "DVT1W", 5.3e6, 0.0, 0.0, 0.0),
            dvt2w: binned(p, "DVT2W", -0.032, 0.0, 0.0, 0.0),
            u0: binned(p, "U0", if is_pmos { 0.025 } else { 0.067 }, 0.0, 0.0, 0.0),
            ua: binned(p, "UA", 2.25e-9, 0.0, 0.0, 0.0),
            ub: binned(p, "UB", 5.87e-19, 0.0, 0.0, 0.0),
            uc: binned(
                p,
                "UC",
                if mob_mod == 3 { -0.0465 } else { -0.0465e-9 },
                0.0,
                0.0,
                0.0,
            ),
            vsat: binned(p, "VSAT", 8.0e4, 0.0, 0.0, 0.0),
            a0: binned(p, "A0", 1.0, 0.0, 0.0, 0.0),
            ags: binned(p, "AGS", 0.0, 0.0, 0.0, 0.0),
            b0: binned(p, "B0", 0.0, 0.0, 0.0, 0.0),
            b1: binned(p, "B1", 0.0, 0.0, 0.0, 0.0),
            keta: binned(p, "KETA", -0.6, 0.0, 0.0, 0.0),
            ketas: binned(p, "KETAS", 0.0, 0.0, 0.0, 0.0),
            abp: binned(p, "ABP", 1.0, 0.0, 0.0, 0.0),
            mxc: binned(p, "MXC", -0.9, 0.0, 0.0, 0.0),
            adice0: binned(p, "ADICE0", 1.0, 0.0, 0.0, 0.0),
            a1: binned(p, "A1", 0.0, 0.0, 0.0, 0.0),
            a2: binned(p, "A2", 1.0, 0.0, 0.0, 0.0),
            rdsw: binned(p, "RDSW", 100.0, 0.0, 0.0, 0.0),
            prwb: binned(p, "PRWB", 0.0, 0.0, 0.0, 0.0),
            prwg: binned(p, "PRWG", 0.0, 0.0, 0.0, 0.0),
            wr: binned(p, "WR", 1.0, 0.0, 0.0, 0.0),
            nfactor: binned(p, "NFACTOR", 1.0, 0.0, 0.0, 0.0),
            dwg: binned(p, "DWG", 0.0, 0.0, 0.0, 0.0),
            dwb: binned(p, "DWB", 0.0, 0.0, 0.0, 0.0),
            voff: binned(p, "VOFF", -0.08, 0.0, 0.0, 0.0),
            eta0: binned(p, "ETA0", 0.08, 0.0, 0.0, 0.0),
            etab: binned(p, "ETAB", -0.07, 0.0, 0.0, 0.0),
            dsub,
            cit: binned(p, "CIT", 0.0, 0.0, 0.0, 0.0),
            cdsc: binned(p, "CDSC", 2.4e-4, 0.0, 0.0, 0.0),
            cdscb: binned(p, "CDSCB", 0.0, 0.0, 0.0, 0.0),
            cdscd: binned(p, "CDSCD", 0.0, 0.0, 0.0, 0.0),
            pclm: binned(p, "PCLM", 1.3, 0.0, 0.0, 0.0),
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
            drout,
            pvag: binned(p, "PVAG", 0.0, 0.0, 0.0, 0.0),
            delta: binned(p, "DELTA", 0.01, 0.0, 0.0, 0.0),
            aii: binned(p, "AII", 0.0, 0.0, 0.0, 0.0),
            bii: binned(p, "BII", 0.0, 0.0, 0.0, 0.0),
            cii: binned(p, "CII", 0.0, 0.0, 0.0, 0.0),
            dii: binned(p, "DII", -1.0, 0.0, 0.0, 0.0),
            alpha0: binned(p, "ALPHA0", 0.0, 0.0, 0.0, 0.0),
            alpha1: binned(p, "ALPHA1", 1.0, 0.0, 0.0, 0.0),
            // PD impact-ionization defaults (b3soipdset.c): beta0=0 (unlike DD's
            // 30), beta2=0.1, vdsatii0=0.9, sii0=0.5, sii1=0.1, esatii=1e7.
            beta0: binned(p, "BETA0", 0.0, 0.0, 0.0, 0.0),
            beta1: binned(p, "BETA1", 0.0, 0.0, 0.0, 0.0),
            beta2: binned(p, "BETA2", 0.1, 0.0, 0.0, 0.0),
            vdsatii0: binned(p, "VDSATII0", 0.9, 0.0, 0.0, 0.0),
            esatii: binned(p, "ESATII", 1e7, 0.0, 0.0, 0.0),
            sii0: binned(p, "SII0", 0.5, 0.0, 0.0, 0.0),
            sii1: binned(p, "SII1", 0.1, 0.0, 0.0, 0.0),
            sii2: binned(p, "SII2", 0.0, 0.0, 0.0, 0.0),
            siid: binned(p, "SIID", 0.0, 0.0, 0.0, 0.0),
            fbjtii: binned(p, "FBJTII", 0.0, 0.0, 0.0, 0.0),
            lii: val(p, "LII", 0.0),
            tii: val(p, "TII", 0.0),
            // PD body-diode / BJT defaults (b3soipdset.c).
            nrecf0: binned(p, "NRECF0", 2.0, 0.0, 0.0, 0.0),
            nrecr0: binned(p, "NRECR0", 10.0, 0.0, 0.0, 0.0),
            ahli: binned(p, "AHLI", 0.0, 0.0, 0.0, 0.0),
            lbjt0: binned(p, "LBJT0", 0.20e-6, 0.0, 0.0, 0.0),
            vabjt: binned(p, "VABJT", 10.0, 0.0, 0.0, 0.0),
            aely: binned(p, "AELY", 0.0, 0.0, 0.0, 0.0),
            vrec0: val(p, "VREC0", 0.0),
            vtun0: val(p, "VTUN0", 0.0),
            ntrecf: val(p, "NTRECF", 0.0),
            ntrecr: val(p, "NTRECR", 0.0),
            ln: val(p, "LN", 2e-6),
            nbjt: val(p, "NBJT", 1.0),
            ndif: val(p, "NDIF", -1.0),
            ldif0: val(p, "LDIF0", 1.0),
            agidl: binned(p, "AGIDL", 0.0, 0.0, 0.0, 0.0),
            bgidl: binned(p, "BGIDL", 0.0, 0.0, 0.0, 0.0),
            ngidl: binned(p, "NGIDL", 1.2, 0.0, 0.0, 0.0),
            ntun: binned(p, "NTUN", 10.0, 0.0, 0.0, 0.0),
            ndiode: binned(p, "NDIODE", 1.0, 0.0, 0.0, 0.0),
            isbjt: binned(p, "ISBJT", 1e-6, 0.0, 0.0, 0.0),
            isdif: binned(p, "ISDIF", 0.0, 0.0, 0.0, 0.0),
            isrec: binned(p, "ISREC", 1e-5, 0.0, 0.0, 0.0),
            istun: binned(p, "ISTUN", 0.0, 0.0, 0.0, 0.0),
            edl: binned(p, "EDL", 2e-6, 0.0, 0.0, 0.0),
            kbjt1: binned(p, "KBJT1", 0.0, 0.0, 0.0, 0.0),
            vsdfb: binned(p, "VSDFB", 0.0, 0.0, 0.0, 0.0),
            vsdfb_given: get(p, "VSDFB").is_some(),
            vsdth: binned(p, "VSDTH", 0.0, 0.0, 0.0, 0.0),
            vsdth_given: get(p, "VSDTH").is_some(),

            k1w1: val(p, "K1W1", 0.0),
            k1w2: val(p, "K1W2", 0.0),
            at: val(p, "AT", 3.3e4),
            gamma1: val(p, "GAMMA1", 0.0),
            gamma1_given: get(p, "GAMMA1").is_some(),
            gamma2: val(p, "GAMMA2", 0.0),
            gamma2_given: get(p, "GAMMA2").is_some(),
            vbx: val(p, "VBX", 0.0),
            vbx_given: get(p, "VBX").is_some(),
            vbm: val(p, "VBM", -3.0),
            xt: val(p, "XT", 1.55e-7),
            kt1: val(p, "KT1", -0.11),
            kt1l: val(p, "KT1L", 0.0),
            kt2: val(p, "KT2", 0.022),
            ua1: val(p, "UA1", 4.31e-9),
            ub1: val(p, "UB1", -7.61e-18),
            uc1: val(p, "UC1", if mob_mod == 3 { -0.056 } else { -0.056e-9 }),
            ute: val(p, "UTE", -1.5),
            prt: val(p, "PRT", 0.0),

            tbox: val(p, "TBOX", 3e-7),
            tsi,
            xj,
            rbody: val(p, "RBODY", 0.0),
            rbsh: val(p, "RBSH", 0.0),
            rth0: val(p, "RTH0", 0.0),
            cth0: val(p, "CTH0", 0.0),
            xbjt,
            xdif: 2.0,
            xrec: val(p, "XREC", 20.0),
            xtun: val(p, "XTUN", 0.0),
            tt: val(p, "TT", 1e-12),
            asd: val(p, "ASD", 0.3),
            csdmin: val(p, "CSDMIN", 0.0),
            csdmin_given: get(p, "CSDMIN").is_some(),

            cgsl,
            cgdl,
            ckappa: val(p, "CKAPPA", 0.6),
            cf,
            clc: val(p, "CLC", 0.1e-7),
            cle: val(p, "CLE", 0.0),
            dwc,
            dlc,
            dlc_given,
            dlcb: val(p, "DLCB", 0.0),
            dlbg: val(p, "DLBG", 0.0),
            fbody: val(p, "FBODY", 1.0),
            cgso,
            cgdo,
            cgeo: val(p, "CGEO", 0.0),
            xpart: val(p, "XPART", 0.0),

            tnom,
            sheet_resistance: val(p, "RSH", 0.0),
            body_jct_gate_side_grading_coeff: val(p, "MJSWG", 0.5),
            gate_sidewall_jct_potential: val(p, "PBSWG", 0.7),
            unit_length_gate_sidewall_jct_cap: val(p, "CJSWG", 1e-10),
            csdesw: val(p, "CSDESW", 0.0),

            lint,
            ll: val(p, "LL", 0.0),
            lln: val(p, "LLN", 1.0),
            lw: val(p, "LW", 0.0),
            lwn: val(p, "LWN", 1.0),
            lwl: val(p, "LWL", 0.0),
            wint,
            wl: val(p, "WL", 0.0),
            wln: val(p, "WLN", 1.0),
            ww: val(p, "WW", 0.0),
            wwn: val(p, "WWN", 1.0),
            wwl: val(p, "WWL", 0.0),

            cox,
            cbox: 0.0,
            csi: 0.0,
            csieff: 0.0,
            qsi: 0.0,
            qsieff: 0.0,
            csit: 0.0,
            cboxt: 0.0,
            nfb: 0.0,
            adice: 0.0,
        };

        model.finish_setup();
        model
    }

    /// Derived film/oxide constants (b3soipdset.c lines 970-997).
    fn finish_setup(&mut self) {
        use super::super::common::{CHARGE_Q, EPSSI};

        self.cbox = 3.453133e-11 / self.tbox;
        self.csi = 1.03594e-10 / self.tsi;
        let cboxt = self.cbox * self.csi / (self.cbox + self.csi);
        self.qsi = CHARGE_Q * self.npeak.v * 1e6 * self.tsi;

        // Tsieff: limit vbsa relative to the silicon film thickness.
        let tmp1 = 2.0 * EPSSI * self.vbsa.v / CHARGE_Q / (1e6 * self.npeak.v);
        let tmp2 = self.tsi * self.tsi;
        if tmp2 < tmp1 {
            log::warn!(
                "B3SOIPD: vbsa = {:.3} is too large for tsi = {:.3e}; treated as zero",
                self.vbsa.v,
                self.tsi
            );
            self.csieff = self.csi;
            self.qsieff = self.qsi;
        } else {
            let t = (self.tsi * self.tsi
                - 2.0 * EPSSI * self.vbsa.v / CHARGE_Q / (1e6 * self.npeak.v))
                .sqrt();
            self.csieff = 1.03594e-10 / t;
            self.qsieff = CHARGE_Q * self.npeak.v * 1e6 * t;
        }
        self.csit = 1.0 / (1.0 / self.cox + 1.0 / self.csieff);
        self.cboxt = 1.0 / (1.0 / self.cbox + 1.0 / self.csieff);
        let nfb0 = 1.0 / (1.0 + self.cbox / self.csit);
        self.nfb = self.kb3.v * nfb0;
        self.adice = self.adice0.v / (1.0 + cboxt / self.cox);
    }
}
