//! Temperature scaling entry points for native MEXTRAM 504.12.1.

use super::Mextram504Model;
use crate::Value;

#[allow(dead_code)]
const C2K: Value = 273.15;
#[allow(dead_code)]
const K_B_OVER_Q: Value = 8.617086918058125e-5;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub(super) struct Mextram504DcScale {
    pub trk: Value,
    pub tamb: Value,
    pub tk: Value,
    pub vt: Value,
    pub vtr: Value,
    pub vt_inv: Value,
    pub vtr_inv: Value,
    pub vdt_inv: Value,
    pub inv_mult: Value,
    pub cbeo_m: Value,
    pub cbco_m: Value,
    pub scrcv_m: Value,
    pub re_tm: Value,
    pub rbc_tm: Value,
    pub rbv_tm: Value,
    pub rccxx_tm: Value,
    pub rccex_tm: Value,
    pub rccin_tm: Value,
    pub rcv_tm: Value,
    pub gccxx_tm: Value,
    pub gccex_tm: Value,
    pub gccin_tm: Value,
}

#[allow(dead_code)]
pub(super) fn scale_dc(
    model: &Mextram504Model,
    simulator_temperature_kelvin: Value,
) -> Mextram504DcScale {
    let trk = model.tref + C2K;
    let tamb = simulator_temperature_kelvin + model.dta;
    let tk = tamb;
    let tn = tk / trk;
    let lntn = tn.ln();
    let vt = K_B_OVER_Q * tk;
    let vtr = K_B_OVER_Q * trk;
    let vt_inv = 1.0 / vt;
    let vtr_inv = 1.0 / vtr;
    let vdt_inv = vt_inv - vtr_inv;
    let inv_mult = 1.0 / model.mult;

    let re_t = model.re * (lntn * model.ae).exp();
    let rbv_t = model.rbv * (lntn * (model.ab - model.aqbo)).exp();
    let rbc_t = model.rbc * (lntn * model.aex).exp();
    let rccxx_t = model.rcc * (lntn * model.ac).exp();
    let rccex_t = model.rcblx * (lntn * model.acbl).exp();
    let rccin_t = model.rcbli * (lntn * model.acbl).exp();
    let rcv_t = model.rcv * (lntn * model.aepi).exp();

    let rccxx_tm = rccxx_t * inv_mult;
    let rccex_tm = rccex_t * inv_mult;
    let rccin_tm = rccin_t * inv_mult;

    Mextram504DcScale {
        trk,
        tamb,
        tk,
        vt,
        vtr,
        vt_inv,
        vtr_inv,
        vdt_inv,
        inv_mult,
        cbeo_m: model.cbeo * model.mult,
        cbco_m: model.cbco * model.mult,
        scrcv_m: model.scrcv * inv_mult,
        re_tm: re_t * inv_mult,
        rbc_tm: rbc_t * inv_mult,
        rbv_tm: rbv_t * inv_mult,
        rccxx_tm,
        rccex_tm,
        rccin_tm,
        rcv_tm: rcv_t * inv_mult,
        gccxx_tm: if model.rcc > 0.0 { 1.0 / rccxx_tm } else { 0.0 },
        gccex_tm: if model.rcblx > 0.0 {
            1.0 / rccex_tm
        } else {
            0.0
        },
        gccin_tm: if model.rcbli > 0.0 {
            1.0 / rccin_tm
        } else {
            0.0
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::semiconductor::mextram504::{Mextram504Model, Mextram504Polarity};
    use std::collections::HashMap;

    fn model_with(params: &[(&str, f64)]) -> Mextram504Model {
        let params = params
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect();
        Mextram504Model::from_params(&params, &HashMap::new(), Mextram504Polarity::Npn)
    }

    fn assert_close(got: f64, expected: f64, rel_tol: f64) {
        let rel = (got - expected).abs() / expected.abs().max(1.0e-30);
        assert!(
            rel <= rel_tol,
            "got {got:.17e}, expected {expected:.17e}, rel {rel:.3e} > {rel_tol:.3e}"
        );
    }

    #[test]
    fn dc_scale_uses_xyce_mult_for_resistances_and_conductances() {
        let model = model_with(&[("MULT", 1.5)]);

        let scale = scale_dc(&model, 298.15);

        assert_close(scale.inv_mult, 2.0 / 3.0, 1.0e-14);
        assert_close(scale.re_tm, 5.0 * 2.0 / 3.0, 1.0e-14);
        assert_close(scale.rbc_tm, 23.0 * 2.0 / 3.0, 1.0e-14);
        assert_close(scale.rbv_tm, 18.0 * 2.0 / 3.0, 1.0e-14);
        assert_close(scale.rccxx_tm, 12.0 * 2.0 / 3.0, 1.0e-14);
        assert_eq!(scale.rccex_tm, 0.0);
        assert_eq!(scale.rccin_tm, 0.0);
        assert_close(scale.rcv_tm, 150.0 * 2.0 / 3.0, 1.0e-14);
        assert_close(scale.scrcv_m, 1250.0 * 2.0 / 3.0, 1.0e-14);
        assert_eq!(scale.cbeo_m, 0.0);
        assert_eq!(scale.cbco_m, 0.0);
        assert_close(scale.gccxx_tm, 1.0 / 8.0, 1.0e-14);
        assert_eq!(scale.gccex_tm, 0.0);
        assert_eq!(scale.gccin_tm, 0.0);
    }

    #[test]
    fn dc_scale_keeps_simulator_temperature_in_kelvin_and_adds_dta() {
        let model = model_with(&[("DTA", 10.0)]);

        let scale = scale_dc(&model, 300.0);

        assert_close(scale.trk, 298.15, 1.0e-14);
        assert_close(scale.tamb, 310.0, 1.0e-14);
        assert_close(scale.tk, 310.0, 1.0e-14);
        assert_close(scale.vt, 8.617086918058125e-5 * 310.0, 1.0e-14);
        assert_close(scale.vtr, 8.617086918058125e-5 * 298.15, 1.0e-14);
        assert_close(scale.vt_inv, 1.0 / scale.vt, 1.0e-14);
        assert_close(scale.vtr_inv, 1.0 / scale.vtr, 1.0e-14);
        assert_close(scale.vdt_inv, scale.vt_inv - scale.vtr_inv, 1.0e-14);
    }
}
