//! EKV 2.6 MOSFET native evaluator.
//!
//! This is the Xyce MOS LEVEL=260 current path with native intrinsic
//! quasi-static terminal charge and junction depletion storage. Noise remains
//! intentionally fail-closed outside this slice.

use super::mosfet::MosType;
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::{Value, circuit::NodeId};
use std::collections::HashMap;

const NODE_COUNT: usize = 4;
const EPS0: Value = 8.854_187_923_944_2e-12;
const SILICON_EPS: Value = 11.7 * EPS0;
const K_OVER_Q: Value = 8.617_333_262_145e-5;
const XYCE_BOLTZMANN: Value = 1.380_622_6e-23;
const CELSIUS0: Value = 273.15;
const EKV26_MODEL_PARAMS: &[&str] = &[
    "LEVEL",
    "TYPE",
    "NOISE",
    "TRISE",
    "TEMP",
    "TNOM",
    "L",
    "W",
    "M",
    "MULT",
    "NS",
    "AS",
    "AD",
    "PS",
    "PD",
    "COX",
    "XJ",
    "VTO",
    "TCV",
    "GAMMA",
    "PHI",
    "KP",
    "BEX",
    "THETA",
    "E0",
    "UCRIT",
    "UCEX",
    "LAMBDA",
    "DL",
    "DW",
    "WETA",
    "LETA",
    "Q0",
    "LK",
    "IBA",
    "IBB",
    "IBBT",
    "IBN",
    "RSH",
    "HDIF",
    "AVTO",
    "AKP",
    "AGAMMA",
    "AF",
    "KF",
    "XD_N",
    "XD_JS",
    "XD_JSW",
    "XD_JSWG",
    "XD_MJ",
    "XD_MJSW",
    "XD_MJSWG",
    "XD_PB",
    "XD_PBSW",
    "XD_PBSWG",
    "XD_CJ",
    "XD_CJSW",
    "XD_CJSWG",
    "XD_GMIN",
    "XD_XJBV",
    "XD_BV",
    "XD_NJTS",
    "XD_NJTSSW",
    "XD_NJTSSWG",
    "XD_VTS",
    "XD_VTSSW",
    "XD_VTSSWG",
    "TP_XTI",
    "TP_CJ",
    "TP_CJSW",
    "TP_CJSWG",
    "TP_PB",
    "TP_PBSW",
    "TP_PBSWG",
    "TP_NJTS",
    "TP_NJTSSW",
    "TP_NJTSSWG",
];
const EKV26_INSTANCE_PARAMS: &[&str] = &[
    "L", "LENGTH", "W", "WIDTH", "M", "MULT", "NS", "AS", "AD", "PS", "PD", "TEMP", "DTEMP",
];
const EKV26_ZERO_INERT_MODEL_PARAMS: &[&str] = &["FNOIMOD", "NOIA", "CGSO", "CGDO", "CGBO"];

#[derive(Debug, Clone, Copy)]
pub struct Ekv26Op {
    pub id: Value,
    pub vgs: Value,
    pub vds: Value,
    pub vbs: Value,
}

#[derive(Debug, Clone, Copy)]
struct Ekv26Currents {
    d: Value,
    g: Value,
    s: Value,
    b: Value,
}

impl Ekv26Currents {
    fn as_array(self) -> [Value; NODE_COUNT] {
        [self.d, self.g, self.s, self.b]
    }
}

#[derive(Debug, Clone)]
pub struct Ekv26Setup {
    type_sign: Value,
    trise: Value,
    temp_c: Option<Value>,
    tnom_c: Option<Value>,
    l: Value,
    w: Value,
    mult: Value,
    series_mult: Value,
    as_area: Value,
    ad_area: Value,
    ps_perim: Value,
    pd_perim: Value,
    cox: Value,
    xj: Value,
    vto: Value,
    tcv: Value,
    gamma: Value,
    phi: Value,
    kp: Value,
    bex: Value,
    theta: Value,
    e0: Value,
    ucrit: Value,
    ucex: Value,
    lambda: Value,
    dl: Value,
    dw: Value,
    weta: Value,
    leta: Value,
    q0: Value,
    lk: Value,
    iba: Value,
    ibb: Value,
    ibbt: Value,
    ibn: Value,
    rsh: Value,
    hdif: Value,
    avto: Value,
    akp: Value,
    agamma: Value,
    af: Value,
    kf: Value,
    noise_enabled: bool,
    xd_n: Value,
    xd_js: Value,
    xd_jsw: Value,
    xd_jswg: Value,
    xd_mj: Value,
    xd_mjsw: Value,
    xd_mjswg: Value,
    xd_pb: Value,
    xd_pbsw: Value,
    xd_pbswg: Value,
    xd_cj: Value,
    xd_cjsw: Value,
    xd_cjswg: Value,
    xd_gmin: Value,
    xd_xjbv: Value,
    xd_bv: Value,
    xd_njts: Value,
    xd_njtssw: Value,
    xd_njtsswg: Value,
    xd_vts: Value,
    xd_vtssw: Value,
    xd_vtsswg: Value,
    tp_xti: Value,
    tp_cj: Value,
    tp_cjsw: Value,
    tp_cjswg: Value,
    tp_pb: Value,
    tp_pbsw: Value,
    tp_pbswg: Value,
    tp_njts: Value,
    tp_njtssw: Value,
    tp_njtsswg: Value,
}

impl Default for Ekv26Setup {
    fn default() -> Self {
        Self {
            type_sign: 1.0,
            trise: 0.0,
            temp_c: None,
            tnom_c: None,
            l: 10.0e-6,
            w: 10.0e-6,
            mult: 1.0,
            series_mult: 1.0,
            as_area: 0.0,
            ad_area: 0.0,
            ps_perim: 0.0,
            pd_perim: 0.0,
            cox: 2.0e-3,
            xj: 300.0e-9,
            vto: 0.5,
            tcv: 1.0e-3,
            gamma: 0.7,
            phi: 0.5,
            kp: 150.0e-6,
            bex: -1.5,
            theta: 0.0,
            e0: 1.0e8,
            ucrit: 2.0e6,
            ucex: 0.8,
            lambda: 0.8,
            dl: -0.01e-6,
            dw: -0.01e-6,
            weta: 0.2,
            leta: 0.3,
            q0: 230.0e-6,
            lk: 0.4e-6,
            iba: 5.0e8,
            ibb: 4.0e8,
            ibbt: 9.0e-4,
            ibn: 1.0,
            rsh: 0.0,
            hdif: 0.5e-6,
            avto: 1.0e-6,
            akp: 1.0e-6,
            agamma: 1.0e-6,
            af: 1.0,
            kf: 0.0,
            noise_enabled: true,
            xd_n: 1.0,
            xd_js: 1.0e-9,
            xd_jsw: 1.0e-12,
            xd_jswg: 1.0e-12,
            xd_mj: 0.9,
            xd_mjsw: 0.7,
            xd_mjswg: 0.7,
            xd_pb: 0.8,
            xd_pbsw: 0.6,
            xd_pbswg: 0.6,
            xd_cj: 1.0e-9,
            xd_cjsw: 1.0e-12,
            xd_cjswg: 1.0e-12,
            xd_gmin: 0.0,
            xd_xjbv: 0.0,
            xd_bv: 10.0,
            xd_njts: 1.0,
            xd_njtssw: 1.0,
            xd_njtsswg: 1.0,
            xd_vts: 0.0,
            xd_vtssw: 0.0,
            xd_vtsswg: 0.0,
            tp_xti: 3.0,
            tp_cj: 0.0,
            tp_cjsw: 0.0,
            tp_cjswg: 0.0,
            tp_pb: 0.0,
            tp_pbsw: 0.0,
            tp_pbswg: 0.0,
            tp_njts: 0.0,
            tp_njtssw: 0.0,
            tp_njtsswg: 0.0,
        }
    }
}

impl Ekv26Setup {
    pub fn from_params(
        model_params: &HashMap<String, Value>,
        mos_type: MosType,
        instance_params: &[(String, Value)],
    ) -> Result<Self, String> {
        let mut setup = Self {
            type_sign: match mos_type {
                MosType::Nmos => 1.0,
                MosType::Pmos => -1.0,
            },
            ..Self::default()
        };

        setup.apply_model_params(model_params)?;
        setup.apply_instance_params(instance_params)?;
        setup.validate()?;
        Ok(setup)
    }

    fn apply_model_params(&mut self, params: &HashMap<String, Value>) -> Result<(), String> {
        reject_unsupported_map_params(params, EKV26_MODEL_PARAMS, "model")?;
        if let Some(value) = map_param(params, &["TYPE"]) {
            if !value.is_finite() || value == 0.0 {
                return Err(format!("EKV26 TYPE={value} must be finite and non-zero"));
            }
            self.type_sign = if value < 0.0 { -1.0 } else { 1.0 };
        }
        if let Some(value) = map_param(params, &["NOISE"]) {
            if !value.is_finite() || (value != 0.0 && value != 1.0) {
                return Err(format!("EKV26 NOISE={value} must be 0 or 1"));
            }
            self.noise_enabled = value != 0.0;
        }
        self.trise = map_param(params, &["TRISE"]).unwrap_or(self.trise);
        self.temp_c = map_param(params, &["TEMP"]);
        self.tnom_c = map_param(params, &["TNOM"]);
        self.l = map_param(params, &["L"]).unwrap_or(self.l);
        self.w = map_param(params, &["W"]).unwrap_or(self.w);
        self.mult = map_param(params, &["M", "MULT"]).unwrap_or(self.mult);
        self.series_mult = map_param(params, &["NS"]).unwrap_or(self.series_mult);
        self.as_area = map_param(params, &["AS"]).unwrap_or(self.as_area);
        self.ad_area = map_param(params, &["AD"]).unwrap_or(self.ad_area);
        self.ps_perim = map_param(params, &["PS"]).unwrap_or(self.ps_perim);
        self.pd_perim = map_param(params, &["PD"]).unwrap_or(self.pd_perim);
        self.cox = map_param(params, &["COX"]).unwrap_or(self.cox);
        self.xj = map_param(params, &["XJ"]).unwrap_or(self.xj);
        self.vto = map_param(params, &["VTO"]).unwrap_or(self.vto);
        self.tcv = map_param(params, &["TCV"]).unwrap_or(self.tcv);
        self.gamma = map_param(params, &["GAMMA"]).unwrap_or(self.gamma);
        self.phi = map_param(params, &["PHI"]).unwrap_or(self.phi);
        self.kp = map_param(params, &["KP"]).unwrap_or(self.kp);
        self.bex = map_param(params, &["BEX"]).unwrap_or(self.bex);
        self.theta = map_param(params, &["THETA"]).unwrap_or(self.theta);
        self.e0 = map_param(params, &["E0"]).unwrap_or(self.e0);
        self.ucrit = map_param(params, &["UCRIT"]).unwrap_or(self.ucrit);
        self.ucex = map_param(params, &["UCEX"]).unwrap_or(self.ucex);
        self.lambda = map_param(params, &["LAMBDA"]).unwrap_or(self.lambda);
        self.dl = map_param(params, &["DL"]).unwrap_or(self.dl);
        self.dw = map_param(params, &["DW"]).unwrap_or(self.dw);
        self.weta = map_param(params, &["WETA"]).unwrap_or(self.weta);
        self.leta = map_param(params, &["LETA"]).unwrap_or(self.leta);
        self.q0 = map_param(params, &["Q0"]).unwrap_or(self.q0);
        self.lk = map_param(params, &["LK"]).unwrap_or(self.lk);
        self.iba = map_param(params, &["IBA"]).unwrap_or(self.iba);
        self.ibb = map_param(params, &["IBB"]).unwrap_or(self.ibb);
        self.ibbt = map_param(params, &["IBBT"]).unwrap_or(self.ibbt);
        self.ibn = map_param(params, &["IBN"]).unwrap_or(self.ibn);
        self.rsh = map_param(params, &["RSH"]).unwrap_or(self.rsh);
        self.hdif = map_param(params, &["HDIF"]).unwrap_or(self.hdif);
        self.avto = map_param(params, &["AVTO"]).unwrap_or(self.avto);
        self.akp = map_param(params, &["AKP"]).unwrap_or(self.akp);
        self.agamma = map_param(params, &["AGAMMA"]).unwrap_or(self.agamma);
        self.af = map_param(params, &["AF"]).unwrap_or(self.af);
        self.kf = map_param(params, &["KF"]).unwrap_or(self.kf);
        self.xd_n = map_param(params, &["XD_N"]).unwrap_or(self.xd_n);
        self.xd_js = map_param(params, &["XD_JS"]).unwrap_or(self.xd_js);
        self.xd_jsw = map_param(params, &["XD_JSW"]).unwrap_or(self.xd_jsw);
        self.xd_jswg = map_param(params, &["XD_JSWG"]).unwrap_or(self.xd_jswg);
        self.xd_mj = map_param(params, &["XD_MJ"]).unwrap_or(self.xd_mj);
        self.xd_mjsw = map_param(params, &["XD_MJSW"]).unwrap_or(self.xd_mjsw);
        self.xd_mjswg = map_param(params, &["XD_MJSWG"]).unwrap_or(self.xd_mjswg);
        self.xd_pb = map_param(params, &["XD_PB"]).unwrap_or(self.xd_pb);
        self.xd_pbsw = map_param(params, &["XD_PBSW"]).unwrap_or(self.xd_pbsw);
        self.xd_pbswg = map_param(params, &["XD_PBSWG"]).unwrap_or(self.xd_pbswg);
        self.xd_cj = map_param(params, &["XD_CJ"]).unwrap_or(self.xd_cj);
        self.xd_cjsw = map_param(params, &["XD_CJSW"]).unwrap_or(self.xd_cjsw);
        self.xd_cjswg = map_param(params, &["XD_CJSWG"]).unwrap_or(self.xd_cjswg);
        self.xd_gmin = map_param(params, &["XD_GMIN"]).unwrap_or(self.xd_gmin);
        self.xd_xjbv = map_param(params, &["XD_XJBV"]).unwrap_or(self.xd_xjbv);
        self.xd_bv = map_param(params, &["XD_BV"]).unwrap_or(self.xd_bv);
        self.xd_njts = map_param(params, &["XD_NJTS"]).unwrap_or(self.xd_njts);
        self.xd_njtssw = map_param(params, &["XD_NJTSSW"]).unwrap_or(self.xd_njtssw);
        self.xd_njtsswg = map_param(params, &["XD_NJTSSWG"]).unwrap_or(self.xd_njtsswg);
        self.xd_vts = map_param(params, &["XD_VTS"]).unwrap_or(self.xd_vts);
        self.xd_vtssw = map_param(params, &["XD_VTSSW"]).unwrap_or(self.xd_vtssw);
        self.xd_vtsswg = map_param(params, &["XD_VTSSWG"]).unwrap_or(self.xd_vtsswg);
        self.tp_xti = map_param(params, &["TP_XTI"]).unwrap_or(self.tp_xti);
        self.tp_cj = map_param(params, &["TP_CJ"]).unwrap_or(self.tp_cj);
        self.tp_cjsw = map_param(params, &["TP_CJSW"]).unwrap_or(self.tp_cjsw);
        self.tp_cjswg = map_param(params, &["TP_CJSWG"]).unwrap_or(self.tp_cjswg);
        self.tp_pb = map_param(params, &["TP_PB"]).unwrap_or(self.tp_pb);
        self.tp_pbsw = map_param(params, &["TP_PBSW"]).unwrap_or(self.tp_pbsw);
        self.tp_pbswg = map_param(params, &["TP_PBSWG"]).unwrap_or(self.tp_pbswg);
        self.tp_njts = map_param(params, &["TP_NJTS"]).unwrap_or(self.tp_njts);
        self.tp_njtssw = map_param(params, &["TP_NJTSSW"]).unwrap_or(self.tp_njtssw);
        self.tp_njtsswg = map_param(params, &["TP_NJTSSWG"]).unwrap_or(self.tp_njtsswg);
        Ok(())
    }

    fn apply_instance_params(&mut self, params: &[(String, Value)]) -> Result<(), String> {
        reject_unsupported_list_params(params, EKV26_INSTANCE_PARAMS, "instance")?;
        self.l = list_param(params, &["L", "LENGTH"]).unwrap_or(self.l);
        self.w = list_param(params, &["W", "WIDTH"]).unwrap_or(self.w);
        self.mult = list_param(params, &["M", "MULT"]).unwrap_or(self.mult);
        self.series_mult = list_param(params, &["NS"]).unwrap_or(self.series_mult);
        self.as_area = list_param(params, &["AS"]).unwrap_or(self.as_area);
        self.ad_area = list_param(params, &["AD"]).unwrap_or(self.ad_area);
        self.ps_perim = list_param(params, &["PS"]).unwrap_or(self.ps_perim);
        self.pd_perim = list_param(params, &["PD"]).unwrap_or(self.pd_perim);
        if let Some(temp) = list_param(params, &["TEMP"]) {
            self.temp_c = Some(if temp > 200.0 { temp - CELSIUS0 } else { temp });
        }
        if let Some(dtemp) = list_param(params, &["DTEMP"]) {
            self.trise += dtemp;
        }
        Ok(())
    }

    fn validate(&self) -> Result<(), String> {
        for (name, value) in [
            ("L", self.l),
            ("W", self.w),
            ("M", self.mult),
            ("NS", self.series_mult),
            ("COX", self.cox),
            ("XJ", self.xj),
            ("KP", self.kp),
            ("UCRIT", self.ucrit),
            ("LK", self.lk),
            ("IBB", self.ibb),
            ("XD_N", self.xd_n),
            ("XD_PB", self.xd_pb),
            ("XD_PBSW", self.xd_pbsw),
            ("XD_PBSWG", self.xd_pbswg),
        ] {
            if !value.is_finite() || value <= 0.0 {
                return Err(format!("EKV26 {name}={value} must be finite and > 0"));
            }
        }
        for (name, value) in [
            ("AS", self.as_area),
            ("AD", self.ad_area),
            ("PS", self.ps_perim),
            ("PD", self.pd_perim),
            ("GAMMA", self.gamma),
            ("PHI", self.phi),
            ("LAMBDA", self.lambda),
            ("WETA", self.weta),
            ("LETA", self.leta),
            ("Q0", self.q0),
            ("IBA", self.iba),
            ("IBN", self.ibn),
            ("RSH", self.rsh),
            ("HDIF", self.hdif),
            ("XD_JS", self.xd_js),
            ("XD_JSW", self.xd_jsw),
            ("XD_JSWG", self.xd_jswg),
            ("XD_CJ", self.xd_cj),
            ("XD_CJSW", self.xd_cjsw),
            ("XD_CJSWG", self.xd_cjswg),
            ("XD_GMIN", self.xd_gmin),
            ("XD_XJBV", self.xd_xjbv),
            ("XD_BV", self.xd_bv),
            ("XD_NJTS", self.xd_njts),
            ("XD_NJTSSW", self.xd_njtssw),
            ("XD_NJTSSWG", self.xd_njtsswg),
            ("XD_VTS", self.xd_vts),
            ("XD_VTSSW", self.xd_vtssw),
            ("XD_VTSSWG", self.xd_vtsswg),
            ("KF", self.kf),
            ("TP_NJTS", self.tp_njts),
            ("TP_NJTSSW", self.tp_njtssw),
            ("TP_NJTSSWG", self.tp_njtsswg),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(format!("EKV26 {name}={value} must be finite and >= 0"));
            }
        }
        if !self.af.is_finite() || self.af <= 0.0 {
            return Err(format!("EKV26 AF={} must be finite and > 0", self.af));
        }
        for (name, value) in [
            ("XD_MJ", self.xd_mj),
            ("XD_MJSW", self.xd_mjsw),
            ("XD_MJSWG", self.xd_mjswg),
        ] {
            if !value.is_finite() || !(0.0..=1.0).contains(&value) {
                return Err(format!("EKV26 {name}={value} must be finite and in [0, 1]"));
            }
        }
        for (name, value) in [
            ("TP_XTI", self.tp_xti),
            ("TP_CJ", self.tp_cj),
            ("TP_CJSW", self.tp_cjsw),
            ("TP_CJSWG", self.tp_cjswg),
            ("TP_PB", self.tp_pb),
            ("TP_PBSW", self.tp_pbsw),
            ("TP_PBSWG", self.tp_pbswg),
        ] {
            if !value.is_finite() {
                return Err(format!("EKV26 {name}={value} must be finite"));
            }
        }
        if self.effective_length() <= 0.0 || self.effective_width() <= 0.0 {
            return Err(format!(
                "EKV26 effective geometry must be positive (L+DL={}, W+DW={})",
                self.effective_length(),
                self.effective_width()
            ));
        }
        Ok(())
    }

    fn effective_length(&self) -> Value {
        self.l + self.dl
    }

    fn effective_width(&self) -> Value {
        self.w + self.dw
    }

    fn terminal_currents(
        &self,
        values: [Value; NODE_COUNT],
        circuit_temp_k: Value,
    ) -> Ekv26Currents {
        let [vd, vg, vs, vb] = values;
        let temp_k = self
            .temp_c
            .map(|temp_c| temp_c + CELSIUS0)
            .unwrap_or(circuit_temp_k + self.trise);
        let tnom_k = self.tnom_c.unwrap_or(25.0) + CELSIUS0;
        let bias = self.dc_bias_terms(vd, vg, vs, vb, temp_k, tnom_k);
        let mut channel = self.type_sign * bias.mode * bias.id * self.mult;

        if self.rsh > 0.0 && self.hdif > 0.0 {
            channel /= 1.0 + bias.gms * bias.rseff + bias.gds * bias.rdeff;
        }

        let (mut idb, mut isb) = self.junction_currents(vd, vs, vb, temp_k, tnom_k);
        if bias.mode > 0.0 {
            idb += self.type_sign * bias.isub * self.mult;
        } else {
            isb += self.type_sign * bias.isub * self.mult;
        }

        Ekv26Currents {
            d: channel + idb,
            g: 0.0,
            s: -channel + isb,
            b: -(idb + isb),
        }
    }

    fn terminal_charges(
        &self,
        values: [Value; NODE_COUNT],
        circuit_temp_k: Value,
    ) -> [Value; NODE_COUNT] {
        let [vd, vg, vs, vb] = values;
        let temp_k = self
            .temp_c
            .map(|temp_c| temp_c + CELSIUS0)
            .unwrap_or(circuit_temp_k + self.trise);
        let tnom_k = self.tnom_c.unwrap_or(25.0) + CELSIUS0;
        let bias = self.dc_bias_terms(vd, vg, vs, vb, temp_k, tnom_k);

        let w_l_cox = bias.weff * bias.leff * self.cox;
        let sif3 = bias.sif * bias.sif2;
        let sir3 = bias.sir * bias.sir2;
        let sqrt_phi_vp_2 = 2.0 * safe_sqrt(bias.vp + bias.phi_t + 1.0e-6);
        let sqrt_phi_vp2_2 = 2.0 * safe_sqrt(bias.phi_t + 0.5 * bias.vp);
        let n_vt_cox = (1.0 + bias.gammaprime / sqrt_phi_vp2_2) * bias.vt * w_l_cox;
        let qd = -n_vt_cox
            * (0.266_666_666
                * (3.0 * sir3
                    + 6.0 * bias.sir2 * bias.sif
                    + 4.0 * bias.sir * bias.sif2
                    + 2.0 * sif3)
                / bias.sif_sir_2
                - 0.5);
        let qs = -n_vt_cox
            * (0.266_666_666
                * (3.0 * sif3
                    + 6.0 * bias.sif2 * bias.sir
                    + 4.0 * bias.sif * bias.sir2
                    + 2.0 * sir3)
                / bias.sif_sir_2
                - 0.5);
        let qi = qs + qd;
        let qb = w_l_cox * (-0.5 * bias.gammaprime * sqrt_phi_vp_2 + bias.vgprime - bias.vgstar)
            - qi * bias.gammaprime / (bias.gammaprime + sqrt_phi_vp2_2);
        let qg = -qi - qb;

        let scale = self.type_sign * self.mult;
        let qd = scale * qd;
        let qs = scale * qs;
        let qg = scale * qg;
        let mut charges = [0.0; NODE_COUNT];
        if bias.mode > 0.0 {
            charges[0] += qd;
            charges[2] += qs;
        } else {
            charges[2] += qd;
            charges[0] += qs;
        }
        charges[1] += qg;
        charges[3] -= qd + qs + qg;
        let (qdb, qsb) = self.junction_charges(vd, vs, vb, temp_k, tnom_k);
        charges[0] += qdb;
        charges[2] += qsb;
        charges[3] -= qdb + qsb;
        charges
    }

    fn noise_psds(
        &self,
        values: [Value; NODE_COUNT],
        circuit_temp_k: Value,
    ) -> Option<(Value, Option<(Value, Value)>)> {
        if !self.noise_enabled {
            return None;
        }
        let [vd, vg, vs, vb] = values;
        let temp_k = self
            .temp_c
            .map(|temp_c| temp_c + CELSIUS0)
            .unwrap_or(circuit_temp_k + self.trise);
        let tnom_k = self.tnom_c.unwrap_or(25.0) + CELSIUS0;
        let bias = self.dc_bias_terms(vd, vg, vs, vb, temp_k, tnom_k);
        let thermal_psd = 4.0 * XYCE_BOLTZMANN * temp_k * bias.gn;
        let denominator = bias.weff * self.series_mult * bias.leff * self.cox;
        let flicker = if self.kf > 0.0 && denominator > 0.0 {
            let gm = self.channel_gm(values, temp_k, tnom_k);
            let psd = self.kf * gm * gm / denominator;
            if psd.is_finite() && psd > 0.0 {
                Some((psd, self.af))
            } else {
                None
            }
        } else {
            None
        };

        if thermal_psd.is_finite() && thermal_psd > 0.0 || flicker.is_some() {
            Some((thermal_psd.max(0.0), flicker))
        } else {
            None
        }
    }

    fn channel_gm(&self, values: [Value; NODE_COUNT], temp_k: Value, tnom_k: Value) -> Value {
        let step = 1.0e-6_f64.max(values[1].abs() * 1.0e-6);
        let mut plus = values;
        let mut minus = values;
        plus[1] += step;
        minus[1] -= step;
        let [vd_p, vg_p, vs_p, vb_p] = plus;
        let [vd_m, vg_m, vs_m, vb_m] = minus;
        let id_plus = self
            .dc_bias_terms(vd_p, vg_p, vs_p, vb_p, temp_k, tnom_k)
            .id;
        let id_minus = self
            .dc_bias_terms(vd_m, vg_m, vs_m, vb_m, temp_k, tnom_k)
            .id;
        (id_plus - id_minus) / (2.0 * step)
    }

    fn dc_bias_terms(
        &self,
        vd_ext: Value,
        vg_ext: Value,
        vs_ext: Value,
        vb_ext: Value,
        temp_k: Value,
        tnom_k: Value,
    ) -> Ekv26BiasTerms {
        let vt = K_OVER_Q * temp_k;
        let vt_01 = 0.1 * vt;
        let vt_2 = vt + vt;
        let vt_4 = vt_2 + vt_2;
        let vt2 = vt * vt;
        let vt2_2 = vt2 + vt2;
        let vt2_16 = 16.0 * vt2;
        let inv_vt = 1.0 / vt;
        let eps_cox = SILICON_EPS / self.cox;
        let lc = safe_sqrt(eps_cox * self.xj);
        let lc_lambda = lc * self.lambda;
        let eps_cox_w = 3.0 * eps_cox * self.weta;
        let eps_cox_l = eps_cox * self.leta;
        let t0 = if self.e0 == 0.0 {
            0.0
        } else {
            self.cox / (SILICON_EPS * self.e0)
        };
        let v0 = (self.q0 + self.q0) / self.cox;
        let eta_qi = if self.type_sign > 0.0 {
            0.5
        } else {
            0.333_333_333_333_3
        };
        let eg = bandgap(temp_k);
        let ref_eg = bandgap(tnom_k);
        let delta_t = temp_k - tnom_k;
        let ratio_t = temp_k / tnom_k;
        let vto_t = self.vto - self.tcv * delta_t;
        let kp_t = self.kp * ratio_t.powf(self.bex);
        let ucrit_t = self.ucrit * ratio_t.powf(self.ucex);
        let ibb_t = self.ibb * (1.0 + self.ibbt * delta_t);
        let mut phi_t = self.phi * ratio_t - 3.0 * vt * ratio_t.ln() - ref_eg * ratio_t + eg;
        let tmp = phi_t - 0.2;
        phi_t = 0.5 * (tmp + safe_sqrt(tmp * tmp + vt * vt)) + 0.2;
        let sqrt_phi = safe_sqrt(phi_t);
        let leff = self.effective_length();
        let weff = self.effective_width();
        let vc = ucrit_t * leff;
        let log_vc_vt = vt * ((0.5 * vc * inv_vt).ln() - 0.6);
        let awl = 1.0 / safe_sqrt(weff * leff);
        let vto_s = if self.type_sign > 0.0 {
            if self.avto != 1.0e-6 {
                awl * (self.avto - 1.0e-6) + vto_t
            } else {
                vto_t
            }
        } else if self.avto != 1.0e-6 {
            awl * (1.0e-6 - self.avto) - vto_t
        } else {
            -vto_t
        };
        let kp_weff = weff
            * if self.akp != 1.0e-6 {
                kp_t * (1.0 + (self.akp - 1.0e-6) * awl)
            } else {
                kp_t
            };
        let gamma_s = if self.agamma != 1.0e-6 {
            self.gamma + (self.agamma - 1.0e-6) * awl
        } else {
            self.gamma
        };
        let gamma_sqrt_phi = gamma_s * sqrt_phi;
        let delta_vfb = if v0 == 0.0 {
            0.0
        } else {
            let vl = 0.28 * (leff / (self.lk * self.series_mult) - 0.1);
            let sqv = 1.0 / (1.0 + 0.5 * (vl + safe_sqrt(vl * vl + 1.936e-3)));
            v0 * sqv * sqv
        };

        let vg = self.type_sign * (vg_ext - vb_ext);
        let mut vs = self.type_sign * (vs_ext - vb_ext);
        let mut vd = self.type_sign * (vd_ext - vb_ext);
        let mode = if vd - vs < 0.0 {
            std::mem::swap(&mut vs, &mut vd);
            -1.0
        } else {
            1.0
        };

        let vgstar = vg - vto_s - delta_vfb + phi_t + gamma_sqrt_phi;
        let sqrt_vgstar = safe_sqrt(vgstar * vgstar + 2.0 * vt2_16);
        let vgprime = 0.5 * (vgstar + sqrt_vgstar);
        let phi_vs = phi_t + vs;
        let sqrt_phi_vs_vt = safe_sqrt(phi_vs * phi_vs + vt2_16);
        let sqrt_phi_vs = safe_sqrt(0.5 * (phi_vs + sqrt_phi_vs_vt));
        let phi_vd = phi_t + vd;
        let sqrt_phi_vd_vt = safe_sqrt(phi_vd * phi_vd + vt2_16);
        let sqrt_phi_vd = safe_sqrt(0.5 * (phi_vd + sqrt_phi_vd_vt));
        let weta_w = eps_cox_w * self.mult / weff;
        let leta_l = eps_cox_l * self.series_mult / leff;
        let big_sqrt_vp0 = safe_sqrt(vgprime + 0.25 * gamma_s * gamma_s);
        let vp0 = vgprime - phi_t - gamma_s * (big_sqrt_vp0 - 0.5 * gamma_s);
        let sqrt_phi_vp0 = safe_sqrt(vp0 + phi_t + vt_01);
        let gammastar = gamma_s - leta_l * (sqrt_phi_vs + sqrt_phi_vd) + weta_w * sqrt_phi_vp0;
        let sqrt_gammastar = safe_sqrt(gammastar * gammastar + vt_01);
        let gammaprime = 0.5 * (gammastar + sqrt_gammastar);
        let big_sqrt_vp = safe_sqrt(vgprime + 0.25 * gammaprime * gammaprime);
        let vp = vgprime - phi_t - gammaprime * (big_sqrt_vp - 0.5 * gammaprime);

        let forward = ekv_norm((vp - vs) * inv_vt);
        let if_ = forward.current;
        let sqrt_if = forward.sqrt_current;
        let vt_vc = vt / vc;
        let vdss_sqrt = safe_sqrt(0.25 + sqrt_if * vt_vc);
        let vdss = vc * (vdss_sqrt - 0.5);
        let vds = 0.5 * (vd - vs);
        let delta_v2 = vt2_16 * (self.lambda * (sqrt_if - vdss * inv_vt) + 15.625e-3);
        let sqrt_vdss_delta_v = safe_sqrt(vdss * vdss + delta_v2);
        let sqrt_vds_vdss_delta_v = safe_sqrt((vds - vdss) * (vds - vdss) + delta_v2);
        let vip = sqrt_vdss_delta_v - sqrt_vds_vdss_delta_v;
        let vdssprime_sqrt = safe_sqrt(0.25 + (sqrt_if - 0.75 * if_.ln()) * vt_vc);
        let vdssprime = vc * (vdssprime_sqrt - 0.5) + log_vc_vt;
        let vdsprime = vds - vdssprime;
        let sqrt_vdssprime_delta_v = safe_sqrt(vdssprime * vdssprime + delta_v2);
        let sqrt_vds_vdssprime_delta_v = safe_sqrt(vdsprime * vdsprime + delta_v2);
        let irprime = ekv_norm(
            (vp - vds - vs - sqrt_vdssprime_delta_v + sqrt_vds_vdssprime_delta_v) * inv_vt,
        )
        .current;

        let lc_ucrit = lc * ucrit_t;
        let delta_l = lc_lambda * (1.0 + (vds - vip) / lc_ucrit).ln();
        let lprime = leff - delta_l + (vds + vip) / ucrit_t;
        let lmin = 0.1 * leff;
        let leq = 0.5 * (lprime + safe_sqrt(lprime * lprime + lmin * lmin));
        let reverse = ekv_norm((vp - vd) * inv_vt);
        let ir = reverse.current;
        let sif2 = 0.25 + if_;
        let sir2 = 0.25 + ir;
        let sif = safe_sqrt(sif2);
        let sir = safe_sqrt(sir2);
        let sif_sir_2 = (sif + sir) * (sif + sir);
        let vp_phi_eps = vp + phi_t + 1.0e-6;
        let sqrt_phi_vp_2 = 2.0 * safe_sqrt(vp_phi_eps);
        let n_1 = gamma_s / sqrt_phi_vp_2;
        let n_1_n = gamma_s / (sqrt_phi_vp_2 + gamma_s);
        let qi = -(1.0 + n_1)
            * vt
            * ((0.666_666_66 + 0.666_666_66) * (sir2 + sir * sif + sif2) / (sif + sir) - 1.0);
        let qb = -0.5 * gamma_s * sqrt_phi_vp_2 - n_1_n * qi;
        let beta = if self.e0 == 0.0 {
            let sqrt_vp_vt = safe_sqrt(vp * vp + vt2_2);
            let vpprime = 0.5 * (vp + sqrt_vp_vt);
            kp_weff / (leq * (1.0 + self.theta * vpprime))
        } else {
            let e0_q_1 = if qb + eta_qi * qi > 0.0 {
                1.0 + t0 * (qb + eta_qi * qi)
            } else {
                1.0 - t0 * (qb + eta_qi * qi)
            };
            let t0_gamma_1 = 1.0 + t0 * gamma_sqrt_phi;
            kp_weff * t0_gamma_1 / (leq * e0_q_1)
        };
        let sqrt_phi_vp = safe_sqrt(phi_t + vp + vt_4);
        let n = 1.0 + gamma_s / (2.0 * sqrt_phi_vp);
        let ispec = vt2_2 * n * beta;
        let id = ispec * (if_ - irprime);
        let gn = beta * qi.abs();

        let vib = vd - vs - (self.ibn + self.ibn) * vdss;
        let isub = if vib > 0.0 && self.iba > 0.0 {
            let exponent = (-lc * ibb_t / vib).max(-35.0);
            (self.iba / ibb_t) * vib * exponent.exp() * id
        } else {
            0.0
        };

        Ekv26BiasTerms {
            mode,
            id,
            isub,
            gms: 0.0,
            gds: 0.0,
            gn,
            rseff: (self.rsh * self.hdif) / (weff - self.dw),
            rdeff: (self.rsh * self.hdif) / (weff - self.dw),
            vt,
            phi_t,
            vp,
            vgprime,
            vgstar,
            gammaprime,
            leff,
            weff,
            sif,
            sir,
            sif2,
            sir2,
            sif_sir_2,
        }
    }

    fn junction_currents(
        &self,
        vd: Value,
        vs: Value,
        vb: Value,
        temp_k: Value,
        tnom_k: Value,
    ) -> (Value, Value) {
        let vt = K_OVER_Q * temp_k;
        let ratio_t = temp_k / tnom_k;
        let eg = bandgap(temp_k);
        let ref_eg = bandgap(tnom_k);
        let geometry = self.junction_geometry();
        let temp_arg = ((ref_eg / (K_OVER_Q * tnom_k) - eg / vt + self.tp_xti * ratio_t.ln())
            / self.xd_n)
            .exp();
        let js_t = self.xd_js * temp_arg;
        let jsw_t = self.xd_jsw * temp_arg;
        let jswg_t = self.xd_jswg * temp_arg;
        let njts_t = self.xd_njts * (1.0 + (ratio_t - 1.0) * self.tp_njts);
        let njtssw_t = self.xd_njtssw * (1.0 + (ratio_t - 1.0) * self.tp_njtssw);
        let njtsswg_t = self.xd_njtsswg * (1.0 + (ratio_t - 1.0) * self.tp_njtsswg);
        let v_db = self.type_sign * (vd - vb);
        let v_sb = self.type_sign * (vs - vb);
        let drain = self.junction_branch_current(
            v_db,
            geometry.ad_i,
            geometry.pd_i,
            geometry.width,
            js_t,
            jsw_t,
            jswg_t,
            njts_t,
            njtssw_t,
            njtsswg_t,
            ratio_t,
            vt,
        );
        let source = self.junction_branch_current(
            v_sb,
            geometry.as_i,
            geometry.ps_i,
            geometry.width,
            js_t,
            jsw_t,
            jswg_t,
            njts_t,
            njtssw_t,
            njtsswg_t,
            ratio_t,
            vt,
        );
        (
            drain * self.type_sign * self.mult,
            source * self.type_sign * self.mult,
        )
    }

    fn junction_geometry(&self) -> Ekv26JunctionGeometry {
        let width = self.effective_width();
        let as_i = if self.as_area == 0.0 && self.hdif > 0.0 {
            2.0 * self.hdif * width
        } else {
            self.as_area
        };
        let ad_i = if self.ad_area == 0.0 && self.hdif > 0.0 {
            2.0 * self.hdif * width
        } else {
            self.ad_area
        };
        let ps_i = if self.ps_perim == 0.0 && self.hdif > 0.0 {
            4.0 * self.hdif + width
        } else {
            self.ps_perim
        };
        let pd_i = if self.pd_perim == 0.0 && self.hdif > 0.0 {
            4.0 * self.hdif + width
        } else {
            self.pd_perim
        };
        Ekv26JunctionGeometry {
            as_i,
            ad_i,
            ps_i,
            pd_i,
            width,
        }
    }

    fn junction_charges(
        &self,
        vd: Value,
        vs: Value,
        vb: Value,
        temp_k: Value,
        tnom_k: Value,
    ) -> (Value, Value) {
        let delta_t = temp_k - tnom_k;
        let geometry = self.junction_geometry();
        let cj_t = self.xd_cj * (1.0 + self.tp_cj * delta_t);
        let cjsw_t = self.xd_cjsw * (1.0 + self.tp_cjsw * delta_t);
        let cjswg_t = self.xd_cjswg * (1.0 + self.tp_cjswg * delta_t);
        let pb_t = self.xd_pb - self.tp_pb * delta_t;
        let pbsw_t = self.xd_pbsw - self.tp_pbsw * delta_t;
        let pbswg_t = self.xd_pbswg - self.tp_pbswg * delta_t;
        let v_db = self.type_sign * (vd - vb);
        let v_sb = self.type_sign * (vs - vb);
        let drain = self.junction_branch_charge(
            v_db,
            geometry.ad_i,
            geometry.pd_i,
            geometry.width,
            cj_t,
            cjsw_t,
            cjswg_t,
            pb_t,
            pbsw_t,
            pbswg_t,
        );
        let source = self.junction_branch_charge(
            v_sb,
            geometry.as_i,
            geometry.ps_i,
            geometry.width,
            cj_t,
            cjsw_t,
            cjswg_t,
            pb_t,
            pbsw_t,
            pbswg_t,
        );
        (
            drain * self.type_sign * self.mult,
            source * self.type_sign * self.mult,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn junction_branch_charge(
        &self,
        voltage: Value,
        area: Value,
        perimeter: Value,
        width: Value,
        cj_t: Value,
        cjsw_t: Value,
        cjswg_t: Value,
        pb_t: Value,
        pbsw_t: Value,
        pbswg_t: Value,
    ) -> Value {
        let csb = junction_depletion_capacitance(voltage, cj_t * area, self.xd_mj, pb_t);
        let cssw =
            junction_depletion_capacitance(voltage, cjsw_t * perimeter, self.xd_mjsw, pbsw_t);
        let csswg =
            junction_depletion_capacitance(voltage, cjswg_t * width, self.xd_mjswg, pbswg_t);
        (csb + cssw + csswg) * voltage
    }

    #[allow(clippy::too_many_arguments)]
    fn junction_branch_current(
        &self,
        voltage: Value,
        area: Value,
        perimeter: Value,
        width: Value,
        js_t: Value,
        jsw_t: Value,
        jswg_t: Value,
        njts_t: Value,
        njtssw_t: Value,
        njtsswg_t: Value,
        ratio_t: Value,
        vt: Value,
    ) -> Value {
        let saturation = js_t * area + jsw_t * perimeter + jswg_t * width;
        let arg = (-voltage * ratio_t / (vt * self.xd_n)).max(-40.0);
        let breakdown_arg = (-voltage + self.xd_bv) * ratio_t / (vt * self.xd_n);
        let breakdown = if breakdown_arg > 70.0 || self.xd_xjbv == 0.0 {
            1.0
        } else {
            1.0 + self.xd_xjbv * (-breakdown_arg).exp()
        };
        let tat = -width
            * jswg_t
            * (lim_exp(
                voltage * ratio_t / (vt * njtsswg_t) * self.xd_vtsswg
                    / (self.xd_vtsswg + voltage).max(1.0e-3),
            ) - 1.0)
            - perimeter
                * jsw_t
                * (lim_exp(
                    voltage * ratio_t / (vt * njtssw_t) * self.xd_vtssw
                        / (self.xd_vtssw + voltage).max(1.0e-3),
                ) - 1.0)
            - area
                * js_t
                * (lim_exp(
                    voltage * ratio_t / (vt * njts_t) * self.xd_vts
                        / (self.xd_vts + voltage).max(1.0e-3),
                ) - 1.0);
        saturation * (1.0 - lim_exp(arg)) * breakdown + voltage * self.xd_gmin + tat
    }
}

#[derive(Debug, Clone, Copy)]
struct Ekv26BiasTerms {
    mode: Value,
    id: Value,
    isub: Value,
    gms: Value,
    gds: Value,
    gn: Value,
    rseff: Value,
    rdeff: Value,
    vt: Value,
    phi_t: Value,
    vp: Value,
    vgprime: Value,
    vgstar: Value,
    gammaprime: Value,
    leff: Value,
    weff: Value,
    sif: Value,
    sir: Value,
    sif2: Value,
    sir2: Value,
    sif_sir_2: Value,
}

#[derive(Debug, Clone, Copy)]
struct Ekv26JunctionGeometry {
    as_i: Value,
    ad_i: Value,
    ps_i: Value,
    pd_i: Value,
    width: Value,
}

#[derive(Debug, Clone, Copy)]
struct EkvNorm {
    current: Value,
    sqrt_current: Value,
}

#[derive(Debug, Clone)]
pub struct EkvMosfet {
    pub name: String,
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    pub node_bulk: NodeId,
    pub setup: Ekv26Setup,
    circuit_temp_k: Value,
    model_xd_gmin: Value,
    last_values: [Value; NODE_COUNT],
    converged_values: [Value; NODE_COUNT],
    has_history: bool,
}

impl EkvMosfet {
    pub fn new_nmos(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
    ) -> Self {
        Self::new(name, MosType::Nmos, drain, gate, source, bulk)
    }

    pub fn new_pmos(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
    ) -> Self {
        Self::new(name, MosType::Pmos, drain, gate, source, bulk)
    }

    fn new(
        name: String,
        mos_type: MosType,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
    ) -> Self {
        let setup = Ekv26Setup {
            type_sign: match mos_type {
                MosType::Nmos => 1.0,
                MosType::Pmos => -1.0,
            },
            ..Ekv26Setup::default()
        };
        Self::with_setup(name, drain, gate, source, bulk, setup, 300.15)
    }

    pub fn from_params(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
        mos_type: MosType,
        model_params: &HashMap<String, Value>,
        instance_params: &[(String, Value)],
        circuit_temp_k: Value,
    ) -> Result<Self, String> {
        let setup = Ekv26Setup::from_params(model_params, mos_type, instance_params)?;
        Ok(Self::with_setup(
            name,
            drain,
            gate,
            source,
            bulk,
            setup,
            circuit_temp_k,
        ))
    }

    fn with_setup(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
        setup: Ekv26Setup,
        circuit_temp_k: Value,
    ) -> Self {
        let model_xd_gmin = setup.xd_gmin;
        Self {
            name,
            node_drain: drain,
            node_gate: gate,
            node_source: source,
            node_bulk: bulk,
            setup,
            circuit_temp_k,
            model_xd_gmin,
            last_values: [0.0; NODE_COUNT],
            converged_values: [0.0; NODE_COUNT],
            has_history: false,
        }
    }

    pub fn with_geometry(mut self, w: Value, l: Value) -> Self {
        self.setup.w = w;
        self.setup.l = l;
        self
    }

    pub fn with_params(mut self, params: &HashMap<String, Value>) -> Result<Self, String> {
        self.setup.apply_model_params(params)?;
        self.setup.validate()?;
        self.model_xd_gmin = self.setup.xd_gmin;
        Ok(self)
    }

    pub fn nodes(&self) -> [NodeId; NODE_COUNT] {
        [
            self.node_drain,
            self.node_gate,
            self.node_source,
            self.node_bulk,
        ]
    }

    fn values(&self, voltages: &[Value]) -> [Value; NODE_COUNT] {
        self.nodes()
            .map(|node| if node == 0 { 0.0 } else { voltages[node - 1] })
    }

    fn terminal_currents_at(&self, values: [Value; NODE_COUNT]) -> Ekv26Currents {
        self.setup.terminal_currents(values, self.circuit_temp_k)
    }

    fn terminal_charges_at(&self, values: [Value; NODE_COUNT]) -> [Value; NODE_COUNT] {
        self.setup.terminal_charges(values, self.circuit_temp_k)
    }

    pub(crate) fn noise_psds_at_solution(
        &self,
        voltages: &[Value],
    ) -> Option<(Value, Option<(Value, Value)>)> {
        self.setup
            .noise_psds(self.values(voltages), self.circuit_temp_k)
    }

    fn terminal_charge_jacobian_at(
        &self,
        values: [Value; NODE_COUNT],
    ) -> [[Value; NODE_COUNT]; NODE_COUNT] {
        let nodes = self.nodes();
        let mut capacitance = [[0.0; NODE_COUNT]; NODE_COUNT];
        for col in 0..NODE_COUNT {
            if nodes[col] == 0 {
                continue;
            }
            let step = 1.0e-6_f64.max(values[col].abs() * 1.0e-6);
            let mut plus = values;
            let mut minus = values;
            plus[col] += step;
            minus[col] -= step;
            let charges_plus = self.terminal_charges_at(plus);
            let charges_minus = self.terminal_charges_at(minus);
            for row in 0..NODE_COUNT {
                capacitance[row][col] = (charges_plus[row] - charges_minus[row]) / (2.0 * step);
            }
        }
        capacitance
    }

    pub(crate) fn dynamic_charge_vector_at_solution(
        &self,
        voltages: &[Value],
    ) -> [Value; NODE_COUNT] {
        self.terminal_charges_at(self.values(voltages))
    }

    pub(crate) fn stamp_dynamic_companion(
        &self,
        voltages: &[Value],
        ag0: Value,
        history_currents: &[Value; NODE_COUNT],
        matrix: &mut impl MatrixStamper,
    ) {
        let nodes = self.nodes();
        let values = self.values(voltages);
        let charges = self.terminal_charges_at(values);
        let capacitance = self.terminal_charge_jacobian_at(values);
        for row in 0..NODE_COUNT {
            if nodes[row] == 0 {
                continue;
            }
            let mut rhs = -(ag0 * charges[row] + history_currents[row]);
            for col in 0..NODE_COUNT {
                let geq = ag0 * capacitance[row][col];
                rhs += geq * values[col];
                if nodes[col] != 0 && geq != 0.0 {
                    matrix.stamp(nodes[row], nodes[col], geq);
                }
            }
            if rhs != 0.0 {
                matrix.stamp_rhs(nodes[row], rhs);
            }
        }
    }

    pub(crate) fn stamp_ac_quasi_static_charge_matrix(
        &self,
        voltages: &[Value],
        omega: Value,
        matrix: &mut impl MatrixStamper,
    ) {
        if omega == 0.0 {
            return;
        }
        let nodes = self.nodes();
        let values = self.values(voltages);
        let capacitance = self.terminal_charge_jacobian_at(values);
        for row in 0..NODE_COUNT {
            if nodes[row] == 0 {
                continue;
            }
            for col in 0..NODE_COUNT {
                if nodes[col] != 0 && capacitance[row][col] != 0.0 {
                    matrix.stamp(nodes[row], nodes[col], omega * capacitance[row][col]);
                }
            }
        }
    }

    pub fn set_eval_gmin(&mut self, gmin: Value) {
        let circuit_gmin = if gmin.is_finite() && gmin > 0.0 {
            gmin
        } else {
            0.0
        };
        self.setup.xd_gmin = self.model_xd_gmin.max(circuit_gmin);
    }

    #[cfg(test)]
    pub(crate) fn eval_gmin(&self) -> Value {
        self.setup.xd_gmin
    }

    pub fn op_values(&self) -> Ekv26Op {
        let [vd, vg, vs, vb] = self.last_values;
        let currents = self.terminal_currents_at(self.last_values);
        Ekv26Op {
            id: currents.d,
            vgs: vg - vs,
            vds: vd - vs,
            vbs: vb - vs,
        }
    }

    fn stamp_linearized_at(&self, values: [Value; NODE_COUNT], matrix: &mut impl MatrixStamper) {
        let nodes = self.nodes();
        let currents = self.terminal_currents_at(values).as_array();
        let mut jacobian = [[0.0; NODE_COUNT]; NODE_COUNT];

        for col in 0..NODE_COUNT {
            if nodes[col] == 0 {
                continue;
            }
            let step = 1.0e-7_f64.max(values[col].abs() * 1.0e-6);
            let mut plus = values;
            plus[col] += step;
            let mut minus = values;
            minus[col] -= step;
            let plus_currents = self.terminal_currents_at(plus).as_array();
            let minus_currents = self.terminal_currents_at(minus).as_array();
            for row in 0..NODE_COUNT {
                jacobian[row][col] = (plus_currents[row] - minus_currents[row]) / (2.0 * step);
            }
        }

        for row in 0..NODE_COUNT {
            if nodes[row] == 0 {
                continue;
            }
            let mut rhs = -currents[row];
            for col in 0..NODE_COUNT {
                rhs += jacobian[row][col] * values[col];
                if nodes[col] != 0 && jacobian[row][col] != 0.0 {
                    matrix.stamp(nodes[row], nodes[col], jacobian[row][col]);
                }
            }
            if rhs != 0.0 {
                matrix.stamp_rhs(nodes[row], rhs);
            }
        }
    }
}

impl NonlinearDevice for EkvMosfet {
    fn update(&mut self, voltages: &[Value]) {
        self.converged_values = self.last_values;
        self.last_values = self.values(voltages);
        self.has_history = true;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        self.stamp_linearized_at(self.values(voltages), matrix);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.has_history {
            return false;
        }
        let tolerance = criteria.voltage_tolerance();
        self.last_values
            .iter()
            .zip(self.converged_values)
            .all(|(new, old)| (new - old).abs() <= tolerance)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_abs_close(label: &str, got: Value, expected: Value, abs_tol: Value) {
        let abs = (got - expected).abs();
        assert!(
            abs <= abs_tol,
            "{label}: got {got:.12e}, expected {expected:.12e}, abs {abs:.3e} > {abs_tol:.3e}"
        );
    }

    fn assert_charge_conserved(label: &str, charges: [Value; NODE_COUNT]) {
        let sum: Value = charges.iter().sum();
        assert_abs_close(label, sum, 0.0, 1.0e-24);
    }

    #[test]
    fn setup_from_params_rejects_unsupported_model_params() {
        for param in ["FNOIMOD", "NOIA"] {
            let params = HashMap::from([(param.to_string(), 1.0)]);
            let message = Ekv26Setup::from_params(&params, MosType::Nmos, &[])
                .expect_err("unsupported EKV26 model params must fail closed");

            assert!(
                message.contains(param) && message.contains("unsupported"),
                "error should identify unsupported {param}: {message}"
            );
        }
    }

    #[test]
    fn setup_from_params_accepts_native_junction_storage_params() {
        let params = HashMap::from([
            ("XD_MJ".to_string(), 0.45),
            ("XD_MJSW".to_string(), 0.35),
            ("XD_MJSWG".to_string(), 0.25),
            ("XD_PB".to_string(), 0.8),
            ("XD_PBSW".to_string(), 0.6),
            ("XD_PBSWG".to_string(), 0.55),
            ("XD_CJ".to_string(), 2.0e-3),
            ("XD_CJSW".to_string(), 3.0e-10),
            ("XD_CJSWG".to_string(), 4.0e-10),
            ("TP_CJ".to_string(), 1.0e-4),
            ("TP_CJSW".to_string(), 2.0e-4),
            ("TP_CJSWG".to_string(), 3.0e-4),
            ("TP_PB".to_string(), 1.0e-4),
            ("TP_PBSW".to_string(), 2.0e-4),
            ("TP_PBSWG".to_string(), 3.0e-4),
        ]);
        let setup = Ekv26Setup::from_params(&params, MosType::Nmos, &[])
            .expect("EKV26 junction storage params are native");

        assert_abs_close("XD_MJ parsed", setup.xd_mj, 0.45, 0.0);
        assert_abs_close("XD_MJSW parsed", setup.xd_mjsw, 0.35, 0.0);
        assert_abs_close("XD_MJSWG parsed", setup.xd_mjswg, 0.25, 0.0);
        assert_abs_close("XD_PB parsed", setup.xd_pb, 0.8, 0.0);
        assert_abs_close("XD_PBSW parsed", setup.xd_pbsw, 0.6, 0.0);
        assert_abs_close("XD_PBSWG parsed", setup.xd_pbswg, 0.55, 0.0);
        assert_abs_close("XD_CJ parsed", setup.xd_cj, 2.0e-3, 0.0);
        assert_abs_close("XD_CJSW parsed", setup.xd_cjsw, 3.0e-10, 0.0);
        assert_abs_close("XD_CJSWG parsed", setup.xd_cjswg, 4.0e-10, 0.0);
        assert_abs_close("TP_CJ parsed", setup.tp_cj, 1.0e-4, 0.0);
        assert_abs_close("TP_CJSW parsed", setup.tp_cjsw, 2.0e-4, 0.0);
        assert_abs_close("TP_CJSWG parsed", setup.tp_cjswg, 3.0e-4, 0.0);
        assert_abs_close("TP_PB parsed", setup.tp_pb, 1.0e-4, 0.0);
        assert_abs_close("TP_PBSW parsed", setup.tp_pbsw, 2.0e-4, 0.0);
        assert_abs_close("TP_PBSWG parsed", setup.tp_pbswg, 3.0e-4, 0.0);
    }

    #[test]
    fn setup_from_params_rejects_unsupported_instance_params() {
        let params = [("CGSO".to_string(), 1.0e-12)];
        let message = Ekv26Setup::from_params(&HashMap::new(), MosType::Nmos, &params)
            .expect_err("unsupported EKV26 instance params must fail closed");

        assert!(
            message.contains("CGSO") && message.contains("unsupported"),
            "error should identify unsupported instance parameter: {message}"
        );
    }

    #[test]
    fn with_params_rejects_unsupported_model_params() {
        let params = HashMap::from([("FNOIMOD".to_string(), 1.0)]);
        let message = EkvMosfet::new_nmos("m1".to_string(), 1, 2, 3, 4)
            .with_params(&params)
            .expect_err("unsupported EKV26 params must fail closed through EkvMosfet");

        assert!(
            message.contains("FNOIMOD") && message.contains("unsupported"),
            "error should identify unsupported with_params parameter: {message}"
        );
    }

    #[test]
    fn terminal_charges_conserve_and_swap_drain_source_in_reverse_mode() {
        let setup = Ekv26Setup::default();
        let forward = setup.terminal_charges([1.0, 0.8, 0.0, -1.0], 300.15);
        let reverse = setup.terminal_charges([0.0, 0.8, 1.0, -1.0], 300.15);

        assert!(forward[1].abs() > 1.0e-18, "gate charge should be active");
        assert_charge_conserved("forward EKV26 terminal charges", forward);
        assert_charge_conserved("reverse EKV26 terminal charges", reverse);
        assert_abs_close("reverse D maps forward S", reverse[0], forward[2], 1.0e-24);
        assert_abs_close("reverse S maps forward D", reverse[2], forward[0], 1.0e-24);
        assert_abs_close("reverse G stays gate", reverse[1], forward[1], 1.0e-24);
        assert_abs_close("reverse B stays bulk", reverse[3], forward[3], 1.0e-24);
    }

    #[test]
    fn terminal_charges_apply_pmos_type_sign() {
        let nmos = Ekv26Setup {
            vto: 0.5706,
            ..Ekv26Setup::default()
        };
        let pmos = Ekv26Setup {
            type_sign: -1.0,
            vto: -0.5706,
            tcv: -nmos.tcv,
            ..nmos.clone()
        };
        let nmos_charges = nmos.terminal_charges([1.0, 0.8, 0.0, -1.0], 300.15);
        let pmos_charges = pmos.terminal_charges([-1.0, -0.8, 0.0, 1.0], 300.15);

        assert_charge_conserved("NMOS EKV26 terminal charges", nmos_charges);
        assert_charge_conserved("PMOS EKV26 terminal charges", pmos_charges);
        for row in 0..NODE_COUNT {
            assert_abs_close(
                &format!("PMOS row {row} mirrors NMOS sign"),
                pmos_charges[row],
                -nmos_charges[row],
                1.0e-24,
            );
        }
    }

    #[test]
    fn set_eval_gmin_uses_larger_of_model_and_circuit_gmin() {
        let mut params = HashMap::new();
        params.insert("XD_GMIN".to_string(), 2.0e-9);
        let mut device = EkvMosfet::new_nmos("m1".to_string(), 1, 2, 0, 0)
            .with_params(&params)
            .expect("supported EKV26 params apply");

        device.set_eval_gmin(1.0e-8);
        assert_eq!(device.setup.xd_gmin, 1.0e-8);

        device.set_eval_gmin(1.0e-12);
        assert_eq!(device.setup.xd_gmin, 2.0e-9);

        device.set_eval_gmin(-1.0);
        assert_eq!(device.setup.xd_gmin, 2.0e-9);
    }
}

fn known_param(name: &str, supported: &[&str]) -> bool {
    supported
        .iter()
        .any(|supported_name| supported_name.eq_ignore_ascii_case(name))
}

fn reject_unsupported_map_params(
    params: &HashMap<String, Value>,
    supported: &[&str],
    context: &str,
) -> Result<(), String> {
    if let Some(name) = params.keys().find(|name| {
        !known_param(name, supported)
            && !zero_inert_model_param(name, params.get(*name).copied().unwrap_or(0.0))
    }) {
        return Err(format!(
            "EKV26 {context} parameter {name} is unsupported by the current native slice; \
             unsupported EKV parameters must not be silently ignored"
        ));
    }
    Ok(())
}

fn zero_inert_model_param(name: &str, value: Value) -> bool {
    value == 0.0 && known_param(name, EKV26_ZERO_INERT_MODEL_PARAMS)
}

fn reject_unsupported_list_params(
    params: &[(String, Value)],
    supported: &[&str],
    context: &str,
) -> Result<(), String> {
    if let Some((name, value)) = params
        .iter()
        .find(|(name, _)| !known_param(name, supported))
    {
        return Err(format!(
            "EKV26 {context} parameter {name}={value} is unsupported by the current native slice; \
             unsupported EKV parameters must not be silently ignored"
        ));
    }
    Ok(())
}

fn map_param(params: &HashMap<String, Value>, names: &[&str]) -> Option<Value> {
    params.iter().find_map(|(name, value)| {
        names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
            .then_some(*value)
    })
}

fn list_param(params: &[(String, Value)], names: &[&str]) -> Option<Value> {
    params.iter().find_map(|(name, value)| {
        names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
            .then_some(*value)
    })
}

fn bandgap(temp_k: Value) -> Value {
    1.16 - 7.02e-4 * temp_k * temp_k / (temp_k + 1108.0)
}

fn safe_sqrt(value: Value) -> Value {
    value.max(0.0).sqrt()
}

fn lim_exp(value: Value) -> Value {
    value.clamp(-80.0, 80.0).exp()
}

fn junction_depletion_capacitance(
    voltage: Value,
    zero_bias_capacitance: Value,
    grading: Value,
    potential: Value,
) -> Value {
    if voltage > 0.0 {
        zero_bias_capacitance * (-(grading) * (1.0 + voltage / potential).ln()).exp()
    } else {
        zero_bias_capacitance * (1.0 - grading * voltage / potential)
    }
}

fn ekv_norm(x: Value) -> EkvNorm {
    let y = if x > -0.35 {
        let z0 = 2.0 / (1.3 + x - (x + 1.6).ln());
        let zk = (2.0 + z0) / (1.0 + x + z0.ln());
        (1.0 + x + zk.ln()) / (2.0 + zk)
    } else if x > -15.0 {
        let z0 = 1.55 + (-x).exp();
        let zk = (2.0 + z0) / (1.0 + x + z0.ln());
        (1.0 + x + zk.ln()) / (2.0 + zk)
    } else if x > -23.0 {
        1.0 / (2.0 + (-x).exp())
    } else {
        x.exp() + 1.0e-64
    };
    let current = y * (1.0 + y);
    EkvNorm {
        current,
        sqrt_current: current.sqrt(),
    }
}
