//! Size- and temperature-dependent parameter evaluation for B3SOIFD.
//!
//! Faithful port of ngspice-46 `b3soifdtemp.c` (`B3SOIFDtemp`). One
//! [`B3SoiFdSized`] is computed per (W, L, rth0, cth0) combination; RSpice
//! computes one per device instance and shares it behind an `Arc`.

use super::super::common::temp::{
    SoiAmbient, SoiSizedCore, soi_ambient, soi_bin, soi_effective_sizes, soi_junctions,
    soi_mobility_temperature, soi_threshold, soi_unbinned,
};
use super::super::common::{B3SoiDialect, EG300, EPSOX, EPSSI, KB_OVER_Q};
use super::params::B3SoiFdModel;
use crate::Value;

/// pParam + per-instance derived quantities (ngspice `b3soifdSizeDependParam`
/// plus the instance-level fields filled in by `B3SOIFDtemp`).
#[derive(Debug, Clone, Default)]
#[allow(clippy::too_many_arguments)]
pub struct B3SoiFdSized {
    core: SoiSizedCore,
}

super::super::common::deref_core!(B3SoiFdSized => SoiSizedCore);

/// Instance geometry inputs needed by the sizing pass.
#[derive(Debug, Clone, Copy)]
pub struct B3SoiFdGeometry {
    pub l: Value,
    pub w: Value,
    pub drain_area: Value,
    pub source_area: Value,
    pub drain_squares: Value,
    pub source_squares: Value,
    pub drain_perimeter: Value,
    pub source_perimeter: Value,
    pub body_squares: Value,
    pub rth0: Value,
    pub cth0: Value,
}

impl B3SoiFdSized {
    /// Port of `B3SOIFDtemp` (b3soifdtemp.c lines 38-813).
    ///
    /// `temp` is the device operating temperature in Kelvin (ngspice
    /// `CKTtemp`).
    pub fn new(model: &B3SoiFdModel, geom: &B3SoiFdGeometry, temp: Value) -> Result<Self, String> {
        let mut p = Self::default();
        let m = model;

        let SoiAmbient {
            tratio, eg0, ni, ..
        } = soi_ambient(&mut p, m, temp, geom.body_squares);

        // --- Geometry scaling (lines 119-167) ---
        let ldrn = geom.l;
        let wdrn = geom.w;
        let t0 = ldrn.powf(m.lln);
        let t1 = wdrn.powf(m.lwn);
        let tmp1 = m.ll / t0 + m.lw / t1 + m.lwl / (t0 * t1);
        let dl = m.lint + tmp1;
        let dlc = m.dlc + tmp1;
        let t2 = ldrn.powf(m.wln);
        let t3 = wdrn.powf(m.wwn);
        let tmp2 = m.wl / t2 + m.ww / t3 + m.wwl / (t2 * t3);
        let dw = m.wint + tmp2;
        let dwc = m.dwc + tmp2;

        soi_effective_sizes(&mut p, geom.l, geom.w, dl, dw, dlc, dwc)
            .map_err(|reason| format!("B3SOIFD: {reason}"))?;

        soi_unbinned(&mut p, m);
        p.abulk_cv_factor = match m.dialect {
            B3SoiDialect::Ngspice => (1.0 + p.clc / p.leff).powf(p.cle),
            B3SoiDialect::Xyce => 1.0 + (p.clc / p.leff).powf(p.cle),
        };

        soi_bin(&mut p, m);

        // --- Temperature adjustment (lines 530-552) ---
        let t0 = tratio - 1.0;
        p.uatemp = p.ua; // saved pre-temperature ua/ub/uc for self-heating in eval
        p.ubtemp = p.ub;
        p.uctemp = p.uc;
        p.rds0denom = (p.weff * 1e6).powf(p.wr);
        p.rth = geom.rth0 * (m.tbox / m.tsi).sqrt() / p.weff;
        p.cth = geom.cth0 * m.tsi;
        p.rbody = m.rbody * p.weff / p.leff;
        soi_mobility_temperature(&mut p, m, tratio, t0);

        // --- Diode/BJT saturation current temperature scaling (574-584) ---
        let t0 = tratio.powf(m.xbjt / p.ndiode);
        let t1 = tratio.powf(m.xdif / p.ndiode);
        let t2 = tratio.powf(m.xrec / p.ndiode / 2.0);
        let t4 = -eg0 / p.ndiode / p.vtm * (1.0 - tratio);
        let t5 = t4.exp();
        let t6 = t5.sqrt();
        p.jbjt = p.isbjt * t0 * t5;
        p.jdif = p.isdif * t1 * t5;
        p.jrec = p.isrec * t2 * t6;
        let t0 = tratio.powf(m.xtun / p.ntun);
        p.jtun = p.istun * t0;

        let (k1_defaulted, k2_defaulted) =
            soi_threshold(&mut p, m, ni, geom.source_perimeter, geom.drain_perimeter);
        if k1_defaulted {
            log::warn!("B3SOIFD: k1 should be specified with k2; using 0.53");
        }
        if k2_defaulted {
            log::warn!("B3SOIFD: k2 should be specified with k1; using -0.0186");
        }

        if m.vth0_given {
            p.vfb = m.mtype * p.vth0 - p.phi - p.k1 * p.sqrt_phi;
        } else {
            p.vfb = -1.0;
            p.vth0 = m.mtype * (p.vfb + p.phi + p.k1 * p.sqrt_phi);
        }
        let t1 = (EPSSI / EPSOX * m.tox * p.xdep0).sqrt();
        let t0 = (-0.5 * p.dsub * p.leff / t1).exp();
        p.theta0vb0 = t0 + 2.0 * t0 * t0;
        let t0 = (-0.5 * p.drout * p.leff / t1).exp();
        let t2 = t0 + 2.0 * t0 * t0;
        p.theta_rout = p.pdibl1 * t2 + p.pdibl2;

        p.min_isub = 5.0e-2 * p.weff * m.tsi * p.isdif.max(p.isrec);

        soi_junctions(
            &mut p,
            m,
            geom.source_area,
            geom.drain_area,
            geom.source_squares,
            geom.drain_squares,
        );

        Ok(p)
    }

    /// The values B3SOIFD's load re-evaluates at a self-heated temperature,
    /// in the load's own forms (b3soifdld.c:665-800): its junction scaling
    /// takes `Eg300` where the temperature pass takes `Eg0`. Everything else
    /// keeps its ambient value: the load takes `phi` from the instance, which
    /// the operating point resets to the ambient one (b3soifdld.c:361,
    /// 709-714), so `Xdep0` and all the temperature pass derives from `phi`,
    /// `vtm` or `ni` stay as computed at the ambient temperature.
    pub(crate) fn self_heated_at(&self, model: &B3SoiFdModel, temp: Value) -> Self {
        let m = model;
        let mut p = self.clone();
        let temp_ratio = temp / p.tnom;
        let t3 = temp_ratio - 1.0;

        p.temp = temp;
        p.vtm = KB_OVER_Q * temp;
        let eg = 1.16 - 7.02e-4 * (temp * temp) / (1108.0 + temp);
        let ni =
            1.45e10 * temp * temp.sqrt() * 1.9230584e-4 * (21.5565981 - eg / (2.0 * p.vtm)).exp();
        p.ni = ni;
        p.vbi = p.vtm * (1.0e20 * p.npeak / (ni * ni)).ln();
        p.vfbb = if p.nsub > 0.0 {
            -m.mtype * p.vtm * (p.npeak / p.nsub).ln()
        } else {
            -m.mtype * p.vtm * (-p.npeak * p.nsub / ni / ni).ln()
        };

        let t5 = (EG300 / p.ndiode / p.vtm * t3).exp();
        let t6 = t5.sqrt();
        p.jbjt = p.isbjt * temp_ratio.powf(m.xbjt / p.ndiode) * t5;
        p.jdif = p.isdif * temp_ratio.powf(m.xdif / p.ndiode) * t5;
        p.jrec = p.isrec * temp_ratio.powf(m.xrec / p.ndiode / 2.0) * t6;
        p.jtun = p.istun * temp_ratio.powf(m.xtun / p.ntun);

        p.u0temp = p.u0 * temp_ratio.powf(p.ute);
        p.vsattemp = p.vsat - p.at * t3;
        p.rds0 = (p.rdsw + p.prt * t3) / p.rds0denom;
        p.ua = p.uatemp + p.ua1 * t3;
        p.ub = p.ubtemp + p.ub1 * t3;
        p.uc = p.uctemp + p.uc1 * t3;
        p
    }
}
