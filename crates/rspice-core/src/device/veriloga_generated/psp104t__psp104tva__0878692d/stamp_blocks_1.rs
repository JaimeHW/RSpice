#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_16(
        p: &Parameters,
        var_ax_i: f64,
        var_axac_i: f64,
        var_bgidl_i: f64,
        var_cfr_i: f64,
        var_cgidl_i: f64,
        var_cgov_i: f64,
        var_cgovaccg_i: f64,
        var_chib_i: f64,
        var_chnl_type: f64,
        var_cinr_i: f64,
        var_epsrox_i: f64,
        var_epssi: f64,
        var_facneffac_i: f64,
        var_fcgovacc_i: f64,
        var_feta_i: f64,
        var_gc2_i: f64,
        var_gc2ov_i: f64,
        var_gc3_i: f64,
        var_gc3ov_i: f64,
        var_guard150: f64,
        var_inv_phita: f64,
        var_neff_i: f64,
        var_nov_i: f64,
        var_novd_i: f64,
        var_rta: f64,
        var_stbgidl_i: f64,
        var_stig_i: f64,
        var_tox_i: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_vp_i: f64,
        var_ar_slot: &mut f64,
        var_arac_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_bch_slot: &mut f64,
        var_bgidld_i_slot: &mut f64,
        var_bov_slot: &mut f64,
        var_bov_d_slot: &mut f64,
        var_cfrd_i_slot: &mut f64,
        var_cgidld_i_slot: &mut f64,
        var_cgovd_i_slot: &mut f64,
        var_cinrd_i_slot: &mut f64,
        var_cox_over_q_slot: &mut f64,
        var_coxovprime_slot: &mut f64,
        var_coxovprime_d_slot: &mut f64,
        var_coxprime_slot: &mut f64,
        var_dxgb_ov_d_slot: &mut f64,
        var_dxgb_ov_s_slot: &mut f64,
        var_dxgb_ov_th_slot: &mut f64,
        var_e_eff0_slot: &mut f64,
        var_epsox_slot: &mut f64,
        var_eta_mu_slot: &mut f64,
        var_eta_mu1_slot: &mut f64,
        var_fcgovaccd_i_slot: &mut f64,
        var_gc2ovd_i_slot: &mut f64,
        var_gc3ovd_i_slot: &mut f64,
        var_gcq_slot: &mut f64,
        var_gcqov_slot: &mut f64,
        var_gcqovd_slot: &mut f64,
        var_gov2_d_slot: &mut f64,
        var_gov2_s_slot: &mut f64,
        var_gov_d_slot: &mut f64,
        var_gov_s_slot: &mut f64,
        var_guard151_slot: &mut f64,
        var_guard152_slot: &mut f64,
        var_guard153_slot: &mut f64,
        var_guard154_slot: &mut f64,
        var_guard155_slot: &mut f64,
        var_guard156_slot: &mut f64,
        var_guard157_slot: &mut f64,
        var_guard158_slot: &mut f64,
        var_guard159_slot: &mut f64,
        var_guard160_slot: &mut f64,
        var_guard161_slot: &mut f64,
        var_guard162_slot: &mut f64,
        var_iginv_i_slot: &mut f64,
        var_igov_i_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_inv_chib_slot: &mut f64,
        var_inv_gov_slot: &mut f64,
        var_inv_vp_slot: &mut f64,
        var_neffac_i_slot: &mut f64,
        var_qq_slot: &mut f64,
        var_sp_ov_a_d_slot: &mut f64,
        var_sp_ov_a_s_slot: &mut f64,
        var_sp_ov_delta_slot: &mut f64,
        var_sp_ov_delta1_d_slot: &mut f64,
        var_sp_ov_delta1_s_slot: &mut f64,
        var_sp_ov_eps_slot: &mut f64,
        var_sp_ov_eps2_d_slot: &mut f64,
        var_sp_ov_eps2_s_slot: &mut f64,
        var_stbgidld_i_slot: &mut f64,
        var_temp_slot: &mut f64,
        var_tf_ig_slot: &mut f64,
        var_tox_sq_slot: &mut f64,
    ) {
        let mut var_ar: f64 = *var_ar_slot;
        let mut var_arac: f64 = *var_arac_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_bch: f64 = *var_bch_slot;
        let mut var_bgidld_i: f64 = *var_bgidld_i_slot;
        let mut var_bov: f64 = *var_bov_slot;
        let mut var_bov_d: f64 = *var_bov_d_slot;
        let mut var_cfrd_i: f64 = *var_cfrd_i_slot;
        let mut var_cgidld_i: f64 = *var_cgidld_i_slot;
        let mut var_cgovd_i: f64 = *var_cgovd_i_slot;
        let mut var_cinrd_i: f64 = *var_cinrd_i_slot;
        let mut var_cox_over_q: f64 = *var_cox_over_q_slot;
        let mut var_coxovprime: f64 = *var_coxovprime_slot;
        let mut var_coxovprime_d: f64 = *var_coxovprime_d_slot;
        let mut var_coxprime: f64 = *var_coxprime_slot;
        let mut var_dxgb_ov_d: f64 = *var_dxgb_ov_d_slot;
        let mut var_dxgb_ov_s: f64 = *var_dxgb_ov_s_slot;
        let mut var_dxgb_ov_th: f64 = *var_dxgb_ov_th_slot;
        let mut var_e_eff0: f64 = *var_e_eff0_slot;
        let mut var_epsox: f64 = *var_epsox_slot;
        let mut var_eta_mu: f64 = *var_eta_mu_slot;
        let mut var_eta_mu1: f64 = *var_eta_mu1_slot;
        let mut var_fcgovaccd_i: f64 = *var_fcgovaccd_i_slot;
        let mut var_gc2ovd_i: f64 = *var_gc2ovd_i_slot;
        let mut var_gc3ovd_i: f64 = *var_gc3ovd_i_slot;
        let mut var_gcq: f64 = *var_gcq_slot;
        let mut var_gcqov: f64 = *var_gcqov_slot;
        let mut var_gcqovd: f64 = *var_gcqovd_slot;
        let mut var_gov2_d: f64 = *var_gov2_d_slot;
        let mut var_gov2_s: f64 = *var_gov2_s_slot;
        let mut var_gov_d: f64 = *var_gov_d_slot;
        let mut var_gov_s: f64 = *var_gov_s_slot;
        let mut var_guard151: f64 = *var_guard151_slot;
        let mut var_guard152: f64 = *var_guard152_slot;
        let mut var_guard153: f64 = *var_guard153_slot;
        let mut var_guard154: f64 = *var_guard154_slot;
        let mut var_guard155: f64 = *var_guard155_slot;
        let mut var_guard156: f64 = *var_guard156_slot;
        let mut var_guard157: f64 = *var_guard157_slot;
        let mut var_guard158: f64 = *var_guard158_slot;
        let mut var_guard159: f64 = *var_guard159_slot;
        let mut var_guard160: f64 = *var_guard160_slot;
        let mut var_guard161: f64 = *var_guard161_slot;
        let mut var_guard162: f64 = *var_guard162_slot;
        let mut var_iginv_i: f64 = *var_iginv_i_slot;
        let mut var_igov_i: f64 = *var_igov_i_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_inv_chib: f64 = *var_inv_chib_slot;
        let mut var_inv_gov: f64 = *var_inv_gov_slot;
        let mut var_inv_vp: f64 = *var_inv_vp_slot;
        let mut var_neffac_i: f64 = *var_neffac_i_slot;
        let mut var_qq: f64 = *var_qq_slot;
        let mut var_sp_ov_a_d: f64 = *var_sp_ov_a_d_slot;
        let mut var_sp_ov_a_s: f64 = *var_sp_ov_a_s_slot;
        let mut var_sp_ov_delta: f64 = *var_sp_ov_delta_slot;
        let mut var_sp_ov_delta1_d: f64 = *var_sp_ov_delta1_d_slot;
        let mut var_sp_ov_delta1_s: f64 = *var_sp_ov_delta1_s_slot;
        let mut var_sp_ov_eps: f64 = *var_sp_ov_eps_slot;
        let mut var_sp_ov_eps2_d: f64 = *var_sp_ov_eps2_d_slot;
        let mut var_sp_ov_eps2_s: f64 = *var_sp_ov_eps2_s_slot;
        let mut var_stbgidld_i: f64 = *var_stbgidld_i_slot;
        let mut var_temp: f64 = *var_temp_slot;
        let mut var_tf_ig: f64 = *var_tf_ig_slot;
        let mut var_tox_sq: f64 = *var_tox_sq_slot;

        let (assign11000_e10278,) = {
    if (var_guard150 != 0.0) {
        (var_bgidl_i,)
    } else {
        (var_bgidld_i,)
    }
};
        var_bgidld_i = assign11000_e10278;

        let (assign11010_e10282,) = {
    if (var_guard150 != 0.0) {
        (var_stbgidl_i,)
    } else {
        (var_stbgidld_i,)
    }
};
        var_stbgidld_i = assign11010_e10282;

        let (assign11020_e10286,) = {
    if (var_guard150 != 0.0) {
        (var_cgidl_i,)
    } else {
        (var_cgidld_i,)
    }
};
        var_cgidld_i = assign11020_e10286;

        let (assign11030_e10290,) = {
    if (var_guard150 != 0.0) {
        (var_igov_i,)
    } else {
        (var_igovd_i,)
    }
};
        var_igovd_i = assign11030_e10290;

        let (assign11040_e10294,) = {
    if (var_guard150 != 0.0) {
        (var_gc2ov_i,)
    } else {
        (var_gc2ovd_i,)
    }
};
        var_gc2ovd_i = assign11040_e10294;

        let (assign11050_e10298,) = {
    if (var_guard150 != 0.0) {
        (var_gc3ov_i,)
    } else {
        (var_gc3ovd_i,)
    }
};
        var_gc3ovd_i = assign11050_e10298;

        let (assign11060_e10302,) = {
    if (var_guard150 != 0.0) {
        (var_cgov_i,)
    } else {
        (var_cgovd_i,)
    }
};
        var_cgovd_i = assign11060_e10302;

        let (assign11070_e10306,) = {
    if (var_guard150 != 0.0) {
        (var_fcgovacc_i,)
    } else {
        (var_fcgovaccd_i,)
    }
};
        var_fcgovaccd_i = assign11070_e10306;

        let (assign11080_e10310,) = {
    if (var_guard150 != 0.0) {
        (var_cinr_i,)
    } else {
        (var_cinrd_i,)
    }
};
        var_cinrd_i = assign11080_e10310;

        let (assign11090_e10314,) = {
    if (var_guard150 != 0.0) {
        (var_cfr_i,)
    } else {
        (var_cfrd_i,)
    }
};
        var_cfrd_i = assign11090_e10314;

        let assign11100_e10317: f64 = (8.8541878176e-12 * var_epsrox_i);
        var_epsox = assign11100_e10317;

        let assign11110_e10320: f64 = (var_epsox / var_tox_i);
        var_coxprime = assign11110_e10320;

        let assign11120_e10323: f64 = (var_tox_i * var_tox_i);
        var_tox_sq = assign11120_e10323;

        let assign11130_e10326: f64 = (var_coxprime / 1.6021918e-19);
        var_cox_over_q = assign11130_e10326;

        let assign11140_e10329: f64 = (var_facneffac_i * var_neff_i);
        var_neffac_i = assign11140_e10329;

        let (assign11150_e10340,) = {
    if (var_neffac_i > 1e20) {
        let (assign11150_e10338,) = {
            if (var_neffac_i < 1e26) {
                (var_neffac_i,)
            } else {
                (1e26,)
            }
        };
        (assign11150_e10338,)
    } else {
        (1e20,)
    }
};
        var_neffac_i = assign11150_e10340;

        var_qq = 0.0;

        let assign11170_e10344: f64 = if p.p51 > 0.0 { 1.0 } else { 0.0 };
        var_guard151 = assign11170_e10344;

        let (assign11180_e10356,) = {
    if (var_guard151 != 0.0) {
        let assign11180_e10348: f64 = (0.4 * 5.951993);
        let assign11180_e10350: f64 = (assign11180_e10348 * p.p51);
        let assign11180_e10353: f64 = (var_coxprime).powf(0.6666666666666666);
        let assign11180_e10354: f64 = (assign11180_e10350 * assign11180_e10353);
        (assign11180_e10354,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11180_e10356;

        let assign11190_e10359: f64 = (-1.0);
        let assign11190_e10360: f64 = if var_chnl_type == assign11190_e10359 { 1.0 } else { 0.0 };
        var_guard152 = assign11190_e10360;

        let (assign11200_e10370,) = {
    if ((var_guard151 != 0.0) && (var_guard152 != 0.0)) {
        let assign11200_e10366: f64 = (7.448711 / 5.951993);
        let assign11200_e10368: f64 = (assign11200_e10366 * var_qq);
        (assign11200_e10368,)
    } else {
        (var_qq,)
    }
};
        var_qq = assign11200_e10370;

        let assign11210_e10373: f64 = (1e-8 * var_coxprime);
        let assign11210_e10375: f64 = (assign11210_e10373 / var_epssi);
        var_e_eff0 = assign11210_e10375;

        let assign11220_e10378: f64 = (0.5 * var_feta_i);
        var_eta_mu = assign11220_e10378;

        var_eta_mu1 = 0.5;

        let assign11240_e10382: f64 = (-1.0);
        let assign11240_e10383: f64 = if var_chnl_type == assign11240_e10382 { 1.0 } else { 0.0 };
        var_guard153 = assign11240_e10383;

        let (assign11250_e10389,) = {
    if (var_guard153 != 0.0) {
        let assign11250_e10387: f64 = (0.3333333333333333 * var_feta_i);
        (assign11250_e10387,)
    } else {
        (var_eta_mu,)
    }
};
        var_eta_mu = assign11250_e10389;

        let (assign11260_e10393,) = {
    if (var_guard153 != 0.0) {
        (0.3333333333333333,)
    } else {
        (var_eta_mu1,)
    }
};
        var_eta_mu1 = assign11260_e10393;

        let assign11270_e10396: f64 = (-2.0);
        let assign11270_e10398: f64 = (assign11270_e10396 / var_ax_i);
        let assign11270_e10400: f64 = (assign11270_e10398 + 1.0);
        let assign11270_e10401: f64 = (2.0_f64).powf(assign11270_e10400);
        let assign11270_e10403: f64 = (assign11270_e10401 - 1.0);
        var_temp = assign11270_e10403;

        let assign11280_e10406: f64 = (var_temp - 1.0);
        let assign11280_e10409: f64 = (var_temp - 1.0);
        let assign11280_e10410: f64 = (assign11280_e10406 * assign11280_e10409);
        let assign11280_e10413: f64 = (4.0 * var_temp);
        let (assign11280_e10420,) = {
    if (assign11280_e10413 > 0.0001) {
        let assign11280_e10418: f64 = (4.0 * var_temp);
        (assign11280_e10418,)
    } else {
        (0.0001,)
    }
};
        let assign11280_e10421: f64 = (assign11280_e10410 / assign11280_e10420);
        var_ar = assign11280_e10421;

        let assign11290_e10424: f64 = (-2.0);
        let assign11290_e10426: f64 = (assign11290_e10424 / var_axac_i);
        let assign11290_e10428: f64 = (assign11290_e10426 + 1.0);
        let assign11290_e10429: f64 = (2.0_f64).powf(assign11290_e10428);
        let assign11290_e10431: f64 = (assign11290_e10429 - 1.0);
        var_temp = assign11290_e10431;

        let assign11300_e10434: f64 = (var_temp - 1.0);
        let assign11300_e10437: f64 = (var_temp - 1.0);
        let assign11300_e10438: f64 = (assign11300_e10434 * assign11300_e10437);
        let assign11300_e10441: f64 = (4.0 * var_temp);
        let (assign11300_e10448,) = {
    if (assign11300_e10441 > 0.0001) {
        let assign11300_e10446: f64 = (4.0 * var_temp);
        (assign11300_e10446,)
    } else {
        (0.0001,)
    }
};
        let assign11300_e10449: f64 = (assign11300_e10438 / assign11300_e10448);
        var_arac = assign11300_e10449;

        let assign11310_e10452: f64 = (1.0 / var_vp_i);
        var_inv_vp = assign11310_e10452;

        let assign11320_e10455: f64 = (var_epsox / var_toxov_i);
        var_coxovprime = assign11320_e10455;

        let assign11330_e10458: f64 = (var_epsox / var_toxovd_i);
        var_coxovprime_d = assign11330_e10458;

        let assign11340_e10461: f64 = (2.0 * 1.6021918e-19);
        let assign11340_e10463: f64 = (assign11340_e10461 * var_nov_i);
        let assign11340_e10465: f64 = (assign11340_e10463 * var_epssi);
        let assign11340_e10467: f64 = (assign11340_e10465 * var_inv_phita);
        let assign11340_e10468: f64 = (assign11340_e10467).sqrt();
        let assign11340_e10470: f64 = (assign11340_e10468 / var_coxovprime);
        var_gov_s = assign11340_e10470;

        let assign11350_e10473: f64 = (2.0 * 1.6021918e-19);
        let assign11350_e10475: f64 = (assign11350_e10473 * var_novd_i);
        let assign11350_e10477: f64 = (assign11350_e10475 * var_epssi);
        let assign11350_e10479: f64 = (assign11350_e10477 * var_inv_phita);
        let assign11350_e10480: f64 = (assign11350_e10479).sqrt();
        let assign11350_e10482: f64 = (assign11350_e10480 / var_coxovprime_d);
        var_gov_d = assign11350_e10482;

        let assign11360_e10485: f64 = (var_gov_s * var_gov_s);
        var_gov2_s = assign11360_e10485;

        let assign11370_e10488: f64 = (var_gov_d * var_gov_d);
        var_gov2_d = assign11370_e10488;

        let assign11380_e10491: f64 = (var_cgovaccg_i * 0.005);
        let assign11380_e10493: f64 = (assign11380_e10491 * var_inv_phita);
        let assign11380_e10494: f64 = (assign11380_e10493).exp();
        let assign11380_e10496: f64 = (assign11380_e10494 - 1.0);
        let assign11380_e10497: f64 = (assign11380_e10496).ln();
        let assign11380_e10499: f64 = (assign11380_e10497 / var_cgovaccg_i);
        let assign11380_e10502: f64 = (0.005 * var_inv_phita);
        let assign11380_e10503: f64 = (assign11380_e10502).exp();
        let assign11380_e10505: f64 = (assign11380_e10503 - 1.0);
        let assign11380_e10506: f64 = (assign11380_e10505).ln();
        let assign11380_e10507: f64 = (assign11380_e10499 - assign11380_e10506);
        var_dxgb_ov_th = assign11380_e10507;

        let assign11390_e10510: f64 = (0.5 * var_gov_s);
        let assign11390_e10511: f64 = (assign11390_e10510).ln();
        let assign11390_e10513: f64 = (assign11390_e10511 + var_dxgb_ov_th);
        var_dxgb_ov_s = assign11390_e10513;

        let assign11400_e10516: f64 = (0.5 * var_gov_d);
        let assign11400_e10517: f64 = (assign11400_e10516).ln();
        let assign11400_e10519: f64 = (assign11400_e10517 + var_dxgb_ov_th);
        var_dxgb_ov_d = assign11400_e10519;

        let assign11410_e10522: f64 = (1.0 / var_gov_s);
        var_inv_gov = assign11410_e10522;

        let assign11420_e10525: f64 = (3.1 * var_gov_s);
        let assign11420_e10527: f64 = (assign11420_e10525 + 8.5);
        var_sp_ov_eps = assign11420_e10527;

        let assign11430_e10530: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_s = assign11430_e10530;

        let assign11440_e10533: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11440_e10533;

        let assign11450_e10536: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard154 = assign11450_e10536;

        let (assign11460_e10542,) = {
    if (var_guard154 != 0.0) {
        let assign11460_e10540: f64 = (64.0 * var_inv_gov);
        (assign11460_e10540,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11460_e10542;

        let assign11470_e10545: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard155 = assign11470_e10545;

        let (assign11480_e10556,) = {
    if ((var_guard154 == 0.0) && (var_guard155 != 0.0)) {
        let assign11480_e10552: f64 = (22.0 * var_inv_gov);
        let assign11480_e10554: f64 = (assign11480_e10552 + 3.0);
        (assign11480_e10554,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11480_e10556;

        let assign11490_e10559: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard156 = assign11490_e10559;

        let (assign11500_e10574,) = {
    if (((var_guard154 == 0.0) && (var_guard155 == 0.0)) && (var_guard156 != 0.0)) {
        let assign11500_e10568: f64 = (-7.2);
        let assign11500_e10570: f64 = (assign11500_e10568 * var_inv_gov);
        let assign11500_e10572: f64 = (assign11500_e10570 + 15.5);
        (assign11500_e10572,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11500_e10574;

        let (assign11510_e10585,) = {
    if (((var_guard154 == 0.0) && (var_guard155 == 0.0)) && (var_guard156 == 0.0)) {
        (var_gov_s,)
    } else {
        (var_sp_ov_a_s,)
    }
};
        var_sp_ov_a_s = assign11510_e10585;

        let assign11520_e10589: f64 = (var_gov2_s * 0.5);
        let assign11520_e10590: f64 = (var_sp_ov_delta + assign11520_e10589);
        let assign11520_e10595: f64 = (var_gov2_s * 0.25);
        let assign11520_e10596: f64 = (var_sp_ov_delta + assign11520_e10595);
        let assign11520_e10598: f64 = (assign11520_e10596 + var_sp_ov_a_s);
        let assign11520_e10599: f64 = (assign11520_e10598).sqrt();
        let assign11520_e10600: f64 = (var_gov_s * assign11520_e10599);
        let assign11520_e10601: f64 = (assign11520_e10590 - assign11520_e10600);
        var_sp_ov_delta1_s = assign11520_e10601;

        let assign11530_e10604: f64 = (1.0 / var_gov_d);
        var_inv_gov = assign11530_e10604;

        let assign11540_e10607: f64 = (3.1 * var_gov_d);
        let assign11540_e10609: f64 = (assign11540_e10607 + 8.5);
        var_sp_ov_eps = assign11540_e10609;

        let assign11550_e10612: f64 = (var_sp_ov_eps * var_sp_ov_eps);
        var_sp_ov_eps2_d = assign11550_e10612;

        let assign11560_e10615: f64 = (0.5 * var_sp_ov_eps);
        var_sp_ov_delta = assign11560_e10615;

        let assign11570_e10618: f64 = if var_inv_gov < 0.06 { 1.0 } else { 0.0 };
        var_guard157 = assign11570_e10618;

        let (assign11580_e10624,) = {
    if (var_guard157 != 0.0) {
        let assign11580_e10622: f64 = (64.0 * var_inv_gov);
        (assign11580_e10622,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11580_e10624;

        let assign11590_e10627: f64 = if var_inv_gov <= 0.45 { 1.0 } else { 0.0 };
        var_guard158 = assign11590_e10627;

        let (assign11600_e10638,) = {
    if ((var_guard157 == 0.0) && (var_guard158 != 0.0)) {
        let assign11600_e10634: f64 = (22.0 * var_inv_gov);
        let assign11600_e10636: f64 = (assign11600_e10634 + 3.0);
        (assign11600_e10636,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11600_e10638;

        let assign11610_e10641: f64 = if var_inv_gov <= 1.6 { 1.0 } else { 0.0 };
        var_guard159 = assign11610_e10641;

        let (assign11620_e10656,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 != 0.0)) {
        let assign11620_e10650: f64 = (-7.2);
        let assign11620_e10652: f64 = (assign11620_e10650 * var_inv_gov);
        let assign11620_e10654: f64 = (assign11620_e10652 + 15.5);
        (assign11620_e10654,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11620_e10656;

        let (assign11630_e10667,) = {
    if (((var_guard157 == 0.0) && (var_guard158 == 0.0)) && (var_guard159 == 0.0)) {
        (var_gov_d,)
    } else {
        (var_sp_ov_a_d,)
    }
};
        var_sp_ov_a_d = assign11630_e10667;

        let assign11640_e10671: f64 = (var_gov2_d * 0.5);
        let assign11640_e10672: f64 = (var_sp_ov_delta + assign11640_e10671);
        let assign11640_e10677: f64 = (var_gov2_d * 0.25);
        let assign11640_e10678: f64 = (var_sp_ov_delta + assign11640_e10677);
        let assign11640_e10680: f64 = (assign11640_e10678 + var_sp_ov_a_d);
        let assign11640_e10681: f64 = (assign11640_e10680).sqrt();
        let assign11640_e10682: f64 = (var_gov_d * assign11640_e10681);
        let assign11640_e10683: f64 = (assign11640_e10672 - assign11640_e10682);
        var_sp_ov_delta1_d = assign11640_e10683;

        let assign11650_e10686: f64 = (1.0 / var_chib_i);
        var_inv_chib = assign11650_e10686;

        let assign11660_e10689: f64 = (4.0 * 0.3333333333333333);
        let assign11660_e10692: f64 = (2.0 * 1.6021918e-19);
        let assign11660_e10694: f64 = (assign11660_e10692 * 9.1093826e-31);
        let assign11660_e10696: f64 = (assign11660_e10694 * var_chib_i);
        let assign11660_e10697: f64 = (assign11660_e10696).sqrt();
        let assign11660_e10698: f64 = (assign11660_e10689 * assign11660_e10697);
        let assign11660_e10700: f64 = (assign11660_e10698 / 1.05457168e-34);
        var_b_fact = assign11660_e10700;

        let assign11670_e10703: f64 = (var_b_fact * var_tox_i);
        var_bch = assign11670_e10703;

        let assign11680_e10706: f64 = (var_b_fact * var_toxov_i);
        var_bov = assign11680_e10706;

        let assign11690_e10709: f64 = (var_b_fact * var_toxovd_i);
        var_bov_d = assign11690_e10709;

        var_gcq = 0.0;

        let assign11710_e10713: f64 = if var_gc3_i < 0.0 { 1.0 } else { 0.0 };
        var_guard160 = assign11710_e10713;

        let (assign11720_e10722,) = {
    if (var_guard160 != 0.0) {
        let assign11720_e10716: f64 = (-0.495);
        let assign11720_e10718: f64 = (assign11720_e10716 * var_gc2_i);
        let assign11720_e10720: f64 = (assign11720_e10718 / var_gc3_i);
        (assign11720_e10720,)
    } else {
        (var_gcq,)
    }
};
        var_gcq = assign11720_e10722;

        var_gcqov = 0.0;

        let assign11740_e10726: f64 = if var_gc3ov_i < 0.0 { 1.0 } else { 0.0 };
        var_guard161 = assign11740_e10726;

        let (assign11750_e10735,) = {
    if (var_guard161 != 0.0) {
        let assign11750_e10729: f64 = (-0.495);
        let assign11750_e10731: f64 = (assign11750_e10729 * var_gc2ov_i);
        let assign11750_e10733: f64 = (assign11750_e10731 / var_gc3ov_i);
        (assign11750_e10733,)
    } else {
        (var_gcqov,)
    }
};
        var_gcqov = assign11750_e10735;

        let assign11760_e10738: f64 = if var_gc3ovd_i < 0.0 { 1.0 } else { 0.0 };
        var_guard162 = assign11760_e10738;

        let (assign11770_e10747,) = {
    if (var_guard162 != 0.0) {
        let assign11770_e10741: f64 = (-0.495);
        let assign11770_e10743: f64 = (assign11770_e10741 * var_gc2ovd_i);
        let assign11770_e10745: f64 = (assign11770_e10743 / var_gc3ovd_i);
        (assign11770_e10745,)
    } else {
        (var_gcqovd,)
    }
};
        var_gcqovd = assign11770_e10747;

        let assign11780_e10750: f64 = (var_rta).powf(var_stig_i);
        var_tf_ig = assign11780_e10750;

        let assign11790_e10753: f64 = (var_iginv_i * var_tf_ig);
        var_iginv_i = assign11790_e10753;

        let assign11800_e10756: f64 = (var_igov_i * var_tf_ig);
        var_igov_i = assign11800_e10756;

        *var_ar_slot = var_ar;
        *var_arac_slot = var_arac;
        *var_b_fact_slot = var_b_fact;
        *var_bch_slot = var_bch;
        *var_bgidld_i_slot = var_bgidld_i;
        *var_bov_slot = var_bov;
        *var_bov_d_slot = var_bov_d;
        *var_cfrd_i_slot = var_cfrd_i;
        *var_cgidld_i_slot = var_cgidld_i;
        *var_cgovd_i_slot = var_cgovd_i;
        *var_cinrd_i_slot = var_cinrd_i;
        *var_cox_over_q_slot = var_cox_over_q;
        *var_coxovprime_slot = var_coxovprime;
        *var_coxovprime_d_slot = var_coxovprime_d;
        *var_coxprime_slot = var_coxprime;
        *var_dxgb_ov_d_slot = var_dxgb_ov_d;
        *var_dxgb_ov_s_slot = var_dxgb_ov_s;
        *var_dxgb_ov_th_slot = var_dxgb_ov_th;
        *var_e_eff0_slot = var_e_eff0;
        *var_epsox_slot = var_epsox;
        *var_eta_mu_slot = var_eta_mu;
        *var_eta_mu1_slot = var_eta_mu1;
        *var_fcgovaccd_i_slot = var_fcgovaccd_i;
        *var_gc2ovd_i_slot = var_gc2ovd_i;
        *var_gc3ovd_i_slot = var_gc3ovd_i;
        *var_gcq_slot = var_gcq;
        *var_gcqov_slot = var_gcqov;
        *var_gcqovd_slot = var_gcqovd;
        *var_gov2_d_slot = var_gov2_d;
        *var_gov2_s_slot = var_gov2_s;
        *var_gov_d_slot = var_gov_d;
        *var_gov_s_slot = var_gov_s;
        *var_guard151_slot = var_guard151;
        *var_guard152_slot = var_guard152;
        *var_guard153_slot = var_guard153;
        *var_guard154_slot = var_guard154;
        *var_guard155_slot = var_guard155;
        *var_guard156_slot = var_guard156;
        *var_guard157_slot = var_guard157;
        *var_guard158_slot = var_guard158;
        *var_guard159_slot = var_guard159;
        *var_guard160_slot = var_guard160;
        *var_guard161_slot = var_guard161;
        *var_guard162_slot = var_guard162;
        *var_iginv_i_slot = var_iginv_i;
        *var_igov_i_slot = var_igov_i;
        *var_igovd_i_slot = var_igovd_i;
        *var_inv_chib_slot = var_inv_chib;
        *var_inv_gov_slot = var_inv_gov;
        *var_inv_vp_slot = var_inv_vp;
        *var_neffac_i_slot = var_neffac_i;
        *var_qq_slot = var_qq;
        *var_sp_ov_a_d_slot = var_sp_ov_a_d;
        *var_sp_ov_a_s_slot = var_sp_ov_a_s;
        *var_sp_ov_delta_slot = var_sp_ov_delta;
        *var_sp_ov_delta1_d_slot = var_sp_ov_delta1_d;
        *var_sp_ov_delta1_s_slot = var_sp_ov_delta1_s;
        *var_sp_ov_eps_slot = var_sp_ov_eps;
        *var_sp_ov_eps2_d_slot = var_sp_ov_eps2_d;
        *var_sp_ov_eps2_s_slot = var_sp_ov_eps2_s;
        *var_stbgidld_i_slot = var_stbgidld_i;
        *var_temp_slot = var_temp;
        *var_tf_ig_slot = var_tf_ig;
        *var_tox_sq_slot = var_tox_sq;
    }

    pub(super) fn stamp_transient_block_17(
        p: &Parameters,
        var_ad_i: f64,
        var_agidl_i: f64,
        var_agidld_i: f64,
        var_as_i: f64,
        var_axinr_i: f64,
        var_bgidl_i: f64,
        var_bgidld_i: f64,
        var_delta: f64,
        var_fcinracc_i: f64,
        var_fntexc_i: f64,
        var_invnf: f64,
        var_jw_i: f64,
        var_pd_i: f64,
        var_ps_i: f64,
        var_rbulk_i: f64,
        var_rde_i: f64,
        var_rg_i: f64,
        var_rjund_i: f64,
        var_rjuns_i: f64,
        var_rse_i: f64,
        var_rta: f64,
        var_rth_i: f64,
        var_rwell_i: f64,
        var_stbgidl_i: f64,
        var_stbgidld_i: f64,
        var_strth_i: f64,
        var_tf_ig: f64,
        var_toxov_i: f64,
        var_toxovd_i: f64,
        var_we: f64,
        var_abd_i_slot: &mut f64,
        var_abdrain_i_slot: &mut f64,
        var_abs_i_slot: &mut f64,
        var_absource_i_slot: &mut f64,
        var_agidlds_slot: &mut f64,
        var_agidls_slot: &mut f64,
        var_ainr_slot: &mut f64,
        var_b_fact_slot: &mut f64,
        var_bgidl_t_slot: &mut f64,
        var_bgidld_t_slot: &mut f64,
        var_bgidlds_slot: &mut f64,
        var_bgidls_slot: &mut f64,
        var_fac_exc_slot: &mut f64,
        var_gbulk_slot: &mut f64,
        var_gdrain_slot: &mut f64,
        var_ggate_slot: &mut f64,
        var_gjund_slot: &mut f64,
        var_gjuns_slot: &mut f64,
        var_gsource_slot: &mut f64,
        var_guard163_slot: &mut f64,
        var_guard164_slot: &mut f64,
        var_guard165_slot: &mut f64,
        var_guard166_slot: &mut f64,
        var_guard167_slot: &mut f64,
        var_guard168_slot: &mut f64,
        var_guard169_slot: &mut f64,
        var_guard170_slot: &mut f64,
        var_guard171_slot: &mut f64,
        var_guard172_slot: &mut f64,
        var_guard173_slot: &mut f64,
        var_guard174_slot: &mut f64,
        var_gwell_slot: &mut f64,
        var_igovd_i_slot: &mut f64,
        var_jwcorr_slot: &mut f64,
        var_jww_slot: &mut f64,
        var_lgd_i_slot: &mut f64,
        var_lgdrain_i_slot: &mut f64,
        var_lgs_i_slot: &mut f64,
        var_lgsource_i_slot: &mut f64,
        var_lsd_i_slot: &mut f64,
        var_lsdrain_i_slot: &mut f64,
        var_lss_i_slot: &mut f64,
        var_lssource_i_slot: &mut f64,
        var_rth_t_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vinr_max_slot: &mut f64,
    ) {
        let mut var_abd_i: f64 = *var_abd_i_slot;
        let mut var_abdrain_i: f64 = *var_abdrain_i_slot;
        let mut var_abs_i: f64 = *var_abs_i_slot;
        let mut var_absource_i: f64 = *var_absource_i_slot;
        let mut var_agidlds: f64 = *var_agidlds_slot;
        let mut var_agidls: f64 = *var_agidls_slot;
        let mut var_ainr: f64 = *var_ainr_slot;
        let mut var_b_fact: f64 = *var_b_fact_slot;
        let mut var_bgidl_t: f64 = *var_bgidl_t_slot;
        let mut var_bgidld_t: f64 = *var_bgidld_t_slot;
        let mut var_bgidlds: f64 = *var_bgidlds_slot;
        let mut var_bgidls: f64 = *var_bgidls_slot;
        let mut var_fac_exc: f64 = *var_fac_exc_slot;
        let mut var_gbulk: f64 = *var_gbulk_slot;
        let mut var_gdrain: f64 = *var_gdrain_slot;
        let mut var_ggate: f64 = *var_ggate_slot;
        let mut var_gjund: f64 = *var_gjund_slot;
        let mut var_gjuns: f64 = *var_gjuns_slot;
        let mut var_gsource: f64 = *var_gsource_slot;
        let mut var_guard163: f64 = *var_guard163_slot;
        let mut var_guard164: f64 = *var_guard164_slot;
        let mut var_guard165: f64 = *var_guard165_slot;
        let mut var_guard166: f64 = *var_guard166_slot;
        let mut var_guard167: f64 = *var_guard167_slot;
        let mut var_guard168: f64 = *var_guard168_slot;
        let mut var_guard169: f64 = *var_guard169_slot;
        let mut var_guard170: f64 = *var_guard170_slot;
        let mut var_guard171: f64 = *var_guard171_slot;
        let mut var_guard172: f64 = *var_guard172_slot;
        let mut var_guard173: f64 = *var_guard173_slot;
        let mut var_guard174: f64 = *var_guard174_slot;
        let mut var_gwell: f64 = *var_gwell_slot;
        let mut var_igovd_i: f64 = *var_igovd_i_slot;
        let mut var_jwcorr: f64 = *var_jwcorr_slot;
        let mut var_jww: f64 = *var_jww_slot;
        let mut var_lgd_i: f64 = *var_lgd_i_slot;
        let mut var_lgdrain_i: f64 = *var_lgdrain_i_slot;
        let mut var_lgs_i: f64 = *var_lgs_i_slot;
        let mut var_lgsource_i: f64 = *var_lgsource_i_slot;
        let mut var_lsd_i: f64 = *var_lsd_i_slot;
        let mut var_lsdrain_i: f64 = *var_lsdrain_i_slot;
        let mut var_lss_i: f64 = *var_lss_i_slot;
        let mut var_lssource_i: f64 = *var_lssource_i_slot;
        let mut var_rth_t: f64 = *var_rth_t_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vinr_max: f64 = *var_vinr_max_slot;

        let assign11810_e10759: f64 = (var_igovd_i * var_tf_ig);
        var_igovd_i = assign11810_e10759;

        let assign11820_e10762: f64 = (var_agidl_i * 4e-18);
        let assign11820_e10765: f64 = (var_toxov_i * var_toxov_i);
        let assign11820_e10766: f64 = (assign11820_e10762 / assign11820_e10765);
        var_agidls = assign11820_e10766;

        let assign11830_e10769: f64 = (var_agidld_i * 4e-18);
        let assign11830_e10772: f64 = (var_toxovd_i * var_toxovd_i);
        let assign11830_e10773: f64 = (assign11830_e10769 / assign11830_e10772);
        var_agidlds = assign11830_e10773;

        let assign11840_e10777: f64 = (var_stbgidl_i * var_delta);
        let assign11840_e10778: f64 = (1.0 + assign11840_e10777);
        let (assign11840_e10787,) = {
    if (assign11840_e10778 > 0.0) {
        let assign11840_e10784: f64 = (var_stbgidl_i * var_delta);
        let assign11840_e10785: f64 = (1.0 + assign11840_e10784);
        (assign11840_e10785,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign11840_e10787;

        let assign11850_e10790: f64 = (var_bgidl_i * var_b_fact);
        var_bgidl_t = assign11850_e10790;

        let assign11860_e10793: f64 = (var_bgidl_t * var_toxov_i);
        let assign11860_e10795: f64 = (assign11860_e10793 * 500000000.0);
        var_bgidls = assign11860_e10795;

        let assign11870_e10799: f64 = (var_stbgidld_i * var_delta);
        let assign11870_e10800: f64 = (1.0 + assign11870_e10799);
        let (assign11870_e10809,) = {
    if (assign11870_e10800 > 0.0) {
        let assign11870_e10806: f64 = (var_stbgidld_i * var_delta);
        let assign11870_e10807: f64 = (1.0 + assign11870_e10806);
        (assign11870_e10807,)
    } else {
        (0.0,)
    }
};
        var_b_fact = assign11870_e10809;

        let assign11880_e10812: f64 = (var_bgidld_i * var_b_fact);
        var_bgidld_t = assign11880_e10812;

        let assign11890_e10815: f64 = (var_bgidld_t * var_toxovd_i);
        let assign11890_e10817: f64 = (assign11890_e10815 * 500000000.0);
        var_bgidlds = assign11890_e10817;

        var_vinr_max = 0.0;

        let assign11910_e10821: f64 = if var_fcinracc_i > 1e-10 { 1.0 } else { 0.0 };
        var_guard163 = assign11910_e10821;

        let (assign11920_e10827,) = {
    if (var_guard163 != 0.0) {
        let assign11920_e10825: f64 = (0.75 / var_fcinracc_i);
        (assign11920_e10825,)
    } else {
        (var_vinr_max,)
    }
};
        var_vinr_max = assign11920_e10827;

        let assign11930_e10830: f64 = (var_axinr_i * var_axinr_i);
        var_ainr = assign11930_e10830;

        let assign11940_e10834: f64 = (var_rta).powf(var_strth_i);
        let assign11940_e10835: f64 = (var_rth_i * assign11940_e10834);
        var_rth_t = assign11940_e10835;

        let assign11950_e10838: f64 = (9.1093826e-31 * 1000000000.0);
        let assign11950_e10840: f64 = (assign11950_e10838 * var_fntexc_i);
        var_fac_exc = assign11950_e10840;

        let assign11960_e10843: f64 = if var_rg_i > 0.0 { 1.0 } else { 0.0 };
        var_guard164 = assign11960_e10843;

        let (assign11970_e10849,) = {
    if (var_guard164 != 0.0) {
        let assign11970_e10847: f64 = (1.0 / var_rg_i);
        (assign11970_e10847,)
    } else {
        (var_ggate,)
    }
};
        var_ggate = assign11970_e10849;

        let (assign11980_e10854,) = {
    if (var_guard164 == 0.0) {
        (0.0,)
    } else {
        (var_ggate,)
    }
};
        var_ggate = assign11980_e10854;

        let assign11990_e10857: f64 = if var_rse_i > 0.0 { 1.0 } else { 0.0 };
        var_guard165 = assign11990_e10857;

        let (assign12000_e10863,) = {
    if (var_guard165 != 0.0) {
        let assign12000_e10861: f64 = (1.0 / var_rse_i);
        (assign12000_e10861,)
    } else {
        (var_gsource,)
    }
};
        var_gsource = assign12000_e10863;

        let (assign12010_e10868,) = {
    if (var_guard165 == 0.0) {
        (0.0,)
    } else {
        (var_gsource,)
    }
};
        var_gsource = assign12010_e10868;

        let assign12020_e10871: f64 = if var_rde_i > 0.0 { 1.0 } else { 0.0 };
        var_guard166 = assign12020_e10871;

        let (assign12030_e10877,) = {
    if (var_guard166 != 0.0) {
        let assign12030_e10875: f64 = (1.0 / var_rde_i);
        (assign12030_e10875,)
    } else {
        (var_gdrain,)
    }
};
        var_gdrain = assign12030_e10877;

        let (assign12040_e10882,) = {
    if (var_guard166 == 0.0) {
        (0.0,)
    } else {
        (var_gdrain,)
    }
};
        var_gdrain = assign12040_e10882;

        let assign12050_e10885: f64 = if var_rbulk_i > 0.0 { 1.0 } else { 0.0 };
        var_guard167 = assign12050_e10885;

        let (assign12060_e10891,) = {
    if (var_guard167 != 0.0) {
        let assign12060_e10889: f64 = (1.0 / var_rbulk_i);
        (assign12060_e10889,)
    } else {
        (var_gbulk,)
    }
};
        var_gbulk = assign12060_e10891;

        let (assign12070_e10896,) = {
    if (var_guard167 == 0.0) {
        (0.0,)
    } else {
        (var_gbulk,)
    }
};
        var_gbulk = assign12070_e10896;

        let assign12080_e10899: f64 = if var_rjuns_i > 0.0 { 1.0 } else { 0.0 };
        var_guard168 = assign12080_e10899;

        let (assign12090_e10905,) = {
    if (var_guard168 != 0.0) {
        let assign12090_e10903: f64 = (1.0 / var_rjuns_i);
        (assign12090_e10903,)
    } else {
        (var_gjuns,)
    }
};
        var_gjuns = assign12090_e10905;

        let (assign12100_e10910,) = {
    if (var_guard168 == 0.0) {
        (0.0,)
    } else {
        (var_gjuns,)
    }
};
        var_gjuns = assign12100_e10910;

        let assign12110_e10913: f64 = if var_rjund_i > 0.0 { 1.0 } else { 0.0 };
        var_guard169 = assign12110_e10913;

        let (assign12120_e10919,) = {
    if (var_guard169 != 0.0) {
        let assign12120_e10917: f64 = (1.0 / var_rjund_i);
        (assign12120_e10917,)
    } else {
        (var_gjund,)
    }
};
        var_gjund = assign12120_e10919;

        let (assign12130_e10924,) = {
    if (var_guard169 == 0.0) {
        (0.0,)
    } else {
        (var_gjund,)
    }
};
        var_gjund = assign12130_e10924;

        let assign12140_e10927: f64 = if var_rwell_i > 0.0 { 1.0 } else { 0.0 };
        var_guard170 = assign12140_e10927;

        let (assign12150_e10933,) = {
    if (var_guard170 != 0.0) {
        let assign12150_e10931: f64 = (1.0 / var_rwell_i);
        (assign12150_e10931,)
    } else {
        (var_gwell,)
    }
};
        var_gwell = assign12150_e10933;

        let (assign12160_e10938,) = {
    if (var_guard170 == 0.0) {
        (0.0,)
    } else {
        (var_gwell,)
    }
};
        var_gwell = assign12160_e10938;

        let assign12170_e10941: f64 = (var_absource_i * var_invnf);
        var_abs_i = assign12170_e10941;

        let assign12180_e10944: f64 = (var_lssource_i * var_invnf);
        var_lss_i = assign12180_e10944;

        let assign12190_e10947: f64 = (var_lgsource_i * var_invnf);
        var_lgs_i = assign12190_e10947;

        let assign12200_e10950: f64 = (var_abdrain_i * var_invnf);
        var_abd_i = assign12200_e10950;

        let assign12210_e10953: f64 = (var_lsdrain_i * var_invnf);
        var_lsd_i = assign12210_e10953;

        let assign12220_e10956: f64 = (var_lgdrain_i * var_invnf);
        var_lgd_i = assign12220_e10956;

        var_jwcorr = 0.0;

        let assign12240_e10960: f64 = if p.p43 == 3.0 { 1.0 } else { 0.0 };
        var_guard171 = assign12240_e10960;

        let (assign12250_e10964,) = {
    if (var_guard171 != 0.0) {
        (1.0,)
    } else {
        (var_jwcorr,)
    }
};
        var_jwcorr = assign12250_e10964;

        var_jww = var_we;

        let assign12270_e10968: f64 = if p.p39 == 0.0 { 1.0 } else { 0.0 };
        var_guard172 = assign12270_e10968;

        let (assign12280_e10977,) = {
    if (var_guard172 != 0.0) {
        let (assign12280_e10975,) = {
            if (var_jw_i > 0.0) {
                (var_jw_i,)
            } else {
                (0.0,)
            }
        };
        (assign12280_e10975,)
    } else {
        (var_jww,)
    }
};
        var_jww = assign12280_e10977;

        let assign12290_e10984: f64 = if ((p.p43 == 2.0) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard173 = assign12290_e10984;

        let (assign12300_e10990,) = {
    if (var_guard173 != 0.0) {
        let assign12300_e10988: f64 = (var_as_i * var_invnf);
        (assign12300_e10988,)
    } else {
        (var_abs_i,)
    }
};
        var_abs_i = assign12300_e10990;

        let (assign12310_e11000,) = {
    if (var_guard173 != 0.0) {
        let assign12310_e10994: f64 = (var_ps_i * var_invnf);
        let assign12310_e10997: f64 = (var_jwcorr * var_jww);
        let assign12310_e10998: f64 = (assign12310_e10994 - assign12310_e10997);
        (assign12310_e10998,)
    } else {
        (var_lss_i,)
    }
};
        var_lss_i = assign12310_e11000;

        let (assign12320_e11004,) = {
    if (var_guard173 != 0.0) {
        (var_jww,)
    } else {
        (var_lgs_i,)
    }
};
        var_lgs_i = assign12320_e11004;

        let (assign12330_e11010,) = {
    if (var_guard173 != 0.0) {
        let assign12330_e11008: f64 = (var_ad_i * var_invnf);
        (assign12330_e11008,)
    } else {
        (var_abd_i,)
    }
};
        var_abd_i = assign12330_e11010;

        let (assign12340_e11020,) = {
    if (var_guard173 != 0.0) {
        let assign12340_e11014: f64 = (var_pd_i * var_invnf);
        let assign12340_e11017: f64 = (var_jwcorr * var_jww);
        let assign12340_e11018: f64 = (assign12340_e11014 - assign12340_e11017);
        (assign12340_e11018,)
    } else {
        (var_lsd_i,)
    }
};
        var_lsd_i = assign12340_e11020;

        let (assign12350_e11024,) = {
    if (var_guard173 != 0.0) {
        (var_jww,)
    } else {
        (var_lgd_i,)
    }
};
        var_lgd_i = assign12350_e11024;

        let assign12360_e11035: f64 = if (((p.p43 == 1.0) || (p.p43 == 2.0)) || (p.p43 == 3.0)) { 1.0 } else { 0.0 };
        var_guard174 = assign12360_e11035;

        let (assign12370_e11044,) = {
    if (var_guard174 != 0.0) {
        let (assign12370_e11042,) = {
            if (var_abs_i > 0.0) {
                (var_abs_i,)
            } else {
                (0.0,)
            }
        };
        (assign12370_e11042,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign12370_e11044;

        let (assign12380_e11053,) = {
    if (var_guard174 != 0.0) {
        let (assign12380_e11051,) = {
            if (var_lss_i > 0.0) {
                (var_lss_i,)
            } else {
                (0.0,)
            }
        };
        (assign12380_e11051,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign12380_e11053;

        let (assign12390_e11062,) = {
    if (var_guard174 != 0.0) {
        let (assign12390_e11060,) = {
            if (var_lgs_i > 0.0) {
                (var_lgs_i,)
            } else {
                (0.0,)
            }
        };
        (assign12390_e11060,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign12390_e11062;

        let (assign12400_e11071,) = {
    if (var_guard174 != 0.0) {
        let (assign12400_e11069,) = {
            if (var_abd_i > 0.0) {
                (var_abd_i,)
            } else {
                (0.0,)
            }
        };
        (assign12400_e11069,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign12400_e11071;

        let (assign12410_e11080,) = {
    if (var_guard174 != 0.0) {
        let (assign12410_e11078,) = {
            if (var_lsd_i > 0.0) {
                (var_lsd_i,)
            } else {
                (0.0,)
            }
        };
        (assign12410_e11078,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign12410_e11080;

        let (assign12420_e11089,) = {
    if (var_guard174 != 0.0) {
        let (assign12420_e11087,) = {
            if (var_lgd_i > 0.0) {
                (var_lgd_i,)
            } else {
                (0.0,)
            }
        };
        (assign12420_e11087,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign12420_e11089;

        let (assign12430_e11094,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_absource_i,)
    }
};
        var_absource_i = assign12430_e11094;

        let (assign12440_e11099,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lssource_i,)
    }
};
        var_lssource_i = assign12440_e11099;

        let (assign12450_e11104,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lgsource_i,)
    }
};
        var_lgsource_i = assign12450_e11104;

        let (assign12460_e11109,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_abdrain_i,)
    }
};
        var_abdrain_i = assign12460_e11109;

        let (assign12470_e11114,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lsdrain_i,)
    }
};
        var_lsdrain_i = assign12470_e11114;

        let (assign12480_e11119,) = {
    if (var_guard174 == 0.0) {
        (0.0,)
    } else {
        (var_lgdrain_i,)
    }
};
        var_lgdrain_i = assign12480_e11119;

        var_vbimin_s = 0.0;

        var_vbimin_d = 0.0;

        var_vfmin_s = 0.0;

        var_vfmin_d = 0.0;

        *var_abd_i_slot = var_abd_i;
        *var_abdrain_i_slot = var_abdrain_i;
        *var_abs_i_slot = var_abs_i;
        *var_absource_i_slot = var_absource_i;
        *var_agidlds_slot = var_agidlds;
        *var_agidls_slot = var_agidls;
        *var_ainr_slot = var_ainr;
        *var_b_fact_slot = var_b_fact;
        *var_bgidl_t_slot = var_bgidl_t;
        *var_bgidld_t_slot = var_bgidld_t;
        *var_bgidlds_slot = var_bgidlds;
        *var_bgidls_slot = var_bgidls;
        *var_fac_exc_slot = var_fac_exc;
        *var_gbulk_slot = var_gbulk;
        *var_gdrain_slot = var_gdrain;
        *var_ggate_slot = var_ggate;
        *var_gjund_slot = var_gjund;
        *var_gjuns_slot = var_gjuns;
        *var_gsource_slot = var_gsource;
        *var_guard163_slot = var_guard163;
        *var_guard164_slot = var_guard164;
        *var_guard165_slot = var_guard165;
        *var_guard166_slot = var_guard166;
        *var_guard167_slot = var_guard167;
        *var_guard168_slot = var_guard168;
        *var_guard169_slot = var_guard169;
        *var_guard170_slot = var_guard170;
        *var_guard171_slot = var_guard171;
        *var_guard172_slot = var_guard172;
        *var_guard173_slot = var_guard173;
        *var_guard174_slot = var_guard174;
        *var_gwell_slot = var_gwell;
        *var_igovd_i_slot = var_igovd_i;
        *var_jwcorr_slot = var_jwcorr;
        *var_jww_slot = var_jww;
        *var_lgd_i_slot = var_lgd_i;
        *var_lgdrain_i_slot = var_lgdrain_i;
        *var_lgs_i_slot = var_lgs_i;
        *var_lgsource_i_slot = var_lgsource_i;
        *var_lsd_i_slot = var_lsd_i;
        *var_lsdrain_i_slot = var_lsdrain_i;
        *var_lss_i_slot = var_lss_i;
        *var_lssource_i_slot = var_lssource_i;
        *var_rth_t_slot = var_rth_t;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vinr_max_slot = var_vinr_max;
    }

    pub(super) fn stamp_transient_block_18(
        p: &Parameters,
        var_absource_i: f64,
        var_idsatbot: f64,
        var_idsatgat: f64,
        var_idsatsti: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_vbibot: f64,
        var_vbigat: f64,
        var_vbisti: f64,
        var_alphaje_slot: &mut f64,
        var_alphaje_dn6_slot: &mut f64,
        var_alphaje_dn7_slot: &mut f64,
        var_alphaje_dn8_slot: &mut f64,
        var_alphaje_dn9_slot: &mut f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_exp_vmax_over_phitd_s_slot: &mut f64,
        var_expxhf1_d_slot: &mut f64,
        var_expxhf1_s_slot: &mut f64,
        var_expxhf2_d_slot: &mut f64,
        var_expxhf2_d_dn6_slot: &mut f64,
        var_expxhf2_d_dn7_slot: &mut f64,
        var_expxhf2_d_dn8_slot: &mut f64,
        var_expxhf2_d_dn9_slot: &mut f64,
        var_expxhf2_s_slot: &mut f64,
        var_expxhf2_s_dn6_slot: &mut f64,
        var_expxhf2_s_dn7_slot: &mut f64,
        var_expxhf2_s_dn8_slot: &mut f64,
        var_expxhf2_s_dn9_slot: &mut f64,
        var_expxhr_d_slot: &mut f64,
        var_expxhr_d_dn6_slot: &mut f64,
        var_expxhr_d_dn7_slot: &mut f64,
        var_expxhr_d_dn8_slot: &mut f64,
        var_expxhr_d_dn9_slot: &mut f64,
        var_expxhr_s_slot: &mut f64,
        var_expxhr_s_dn6_slot: &mut f64,
        var_expxhr_s_dn7_slot: &mut f64,
        var_expxhr_s_dn8_slot: &mut f64,
        var_expxhr_s_dn9_slot: &mut f64,
        var_guard175_slot: &mut f64,
        var_guard176_slot: &mut f64,
        var_guard177_slot: &mut f64,
        var_guard178_slot: &mut f64,
        var_guard179_slot: &mut f64,
        var_guard180_slot: &mut f64,
        var_guard181_slot: &mut f64,
        var_guard182_slot: &mut f64,
        var_i1_cor_slot: &mut f64,
        var_i1_cor_dn6_slot: &mut f64,
        var_i1_cor_dn7_slot: &mut f64,
        var_i1_cor_dn8_slot: &mut f64,
        var_i1_cor_dn9_slot: &mut f64,
        var_i2_cor_slot: &mut f64,
        var_i2_cor_dn6_slot: &mut f64,
        var_i2_cor_dn7_slot: &mut f64,
        var_i2_cor_dn8_slot: &mut f64,
        var_i2_cor_dn9_slot: &mut f64,
        var_i3_cor_slot: &mut f64,
        var_i3_cor_dn6_slot: &mut f64,
        var_i3_cor_dn7_slot: &mut f64,
        var_i3_cor_dn8_slot: &mut f64,
        var_i3_cor_dn9_slot: &mut f64,
        var_i4_cor_slot: &mut f64,
        var_i4_cor_dn6_slot: &mut f64,
        var_i4_cor_dn7_slot: &mut f64,
        var_i4_cor_dn8_slot: &mut f64,
        var_i4_cor_dn9_slot: &mut f64,
        var_i5_cor_slot: &mut f64,
        var_i5_cor_dn6_slot: &mut f64,
        var_i5_cor_dn7_slot: &mut f64,
        var_i5_cor_dn8_slot: &mut f64,
        var_i5_cor_dn9_slot: &mut f64,
        var_isatfor1_d_slot: &mut f64,
        var_isatfor1_s_slot: &mut f64,
        var_isatfor2_d_slot: &mut f64,
        var_isatfor2_d_dn6_slot: &mut f64,
        var_isatfor2_d_dn7_slot: &mut f64,
        var_isatfor2_d_dn8_slot: &mut f64,
        var_isatfor2_d_dn9_slot: &mut f64,
        var_isatfor2_s_slot: &mut f64,
        var_isatfor2_s_dn6_slot: &mut f64,
        var_isatfor2_s_dn7_slot: &mut f64,
        var_isatfor2_s_dn8_slot: &mut f64,
        var_isatfor2_s_dn9_slot: &mut f64,
        var_isatrev_d_slot: &mut f64,
        var_isatrev_d_dn6_slot: &mut f64,
        var_isatrev_d_dn7_slot: &mut f64,
        var_isatrev_d_dn8_slot: &mut f64,
        var_isatrev_d_dn9_slot: &mut f64,
        var_isatrev_s_slot: &mut f64,
        var_isatrev_s_dn6_slot: &mut f64,
        var_isatrev_s_dn7_slot: &mut f64,
        var_isatrev_s_dn8_slot: &mut f64,
        var_isatrev_s_dn9_slot: &mut f64,
        var_m0_rev_slot: &mut f64,
        var_m0_rev_dn6_slot: &mut f64,
        var_m0_rev_dn7_slot: &mut f64,
        var_m0_rev_dn8_slot: &mut f64,
        var_m0_rev_dn9_slot: &mut f64,
        var_m0flag_d_slot: &mut f64,
        var_m0flag_s_slot: &mut f64,
        var_mcor_rev_slot: &mut f64,
        var_mcor_rev_dn6_slot: &mut f64,
        var_mcor_rev_dn7_slot: &mut f64,
        var_mcor_rev_dn8_slot: &mut f64,
        var_mcor_rev_dn9_slot: &mut f64,
        var_mfor1_d_slot: &mut f64,
        var_mfor1_s_slot: &mut f64,
        var_mfor2_d_slot: &mut f64,
        var_mfor2_d_dn6_slot: &mut f64,
        var_mfor2_d_dn7_slot: &mut f64,
        var_mfor2_d_dn8_slot: &mut f64,
        var_mfor2_d_dn9_slot: &mut f64,
        var_mfor2_s_slot: &mut f64,
        var_mfor2_s_dn6_slot: &mut f64,
        var_mfor2_s_dn7_slot: &mut f64,
        var_mfor2_s_dn8_slot: &mut f64,
        var_mfor2_s_dn9_slot: &mut f64,
        var_mrev_d_slot: &mut f64,
        var_mrev_d_dn6_slot: &mut f64,
        var_mrev_d_dn7_slot: &mut f64,
        var_mrev_d_dn8_slot: &mut f64,
        var_mrev_d_dn9_slot: &mut f64,
        var_mrev_s_slot: &mut f64,
        var_mrev_s_dn6_slot: &mut f64,
        var_mrev_s_dn7_slot: &mut f64,
        var_mrev_s_dn8_slot: &mut f64,
        var_mrev_s_dn9_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_tt0_slot: &mut f64,
        var_tt1_slot: &mut f64,
        var_tt1_dn6_slot: &mut f64,
        var_tt1_dn7_slot: &mut f64,
        var_tt1_dn8_slot: &mut f64,
        var_tt1_dn9_slot: &mut f64,
        var_tt2_slot: &mut f64,
        var_tt2_dn6_slot: &mut f64,
        var_tt2_dn7_slot: &mut f64,
        var_tt2_dn8_slot: &mut f64,
        var_tt2_dn9_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmax_s_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
        var_xhighf1_d_slot: &mut f64,
        var_xhighf1_s_slot: &mut f64,
        var_xhighf2_d_slot: &mut f64,
        var_xhighf2_d_dn6_slot: &mut f64,
        var_xhighf2_d_dn7_slot: &mut f64,
        var_xhighf2_d_dn8_slot: &mut f64,
        var_xhighf2_d_dn9_slot: &mut f64,
        var_xhighf2_s_slot: &mut f64,
        var_xhighf2_s_dn6_slot: &mut f64,
        var_xhighf2_s_dn7_slot: &mut f64,
        var_xhighf2_s_dn8_slot: &mut f64,
        var_xhighf2_s_dn9_slot: &mut f64,
        var_xhighr_d_slot: &mut f64,
        var_xhighr_d_dn6_slot: &mut f64,
        var_xhighr_d_dn7_slot: &mut f64,
        var_xhighr_d_dn8_slot: &mut f64,
        var_xhighr_d_dn9_slot: &mut f64,
        var_xhighr_s_slot: &mut f64,
        var_xhighr_s_dn6_slot: &mut f64,
        var_xhighr_s_dn7_slot: &mut f64,
        var_xhighr_s_dn8_slot: &mut f64,
        var_xhighr_s_dn9_slot: &mut f64,
        var_zflagbot_d_slot: &mut f64,
        var_zflagbot_s_slot: &mut f64,
        var_zflaggat_d_slot: &mut f64,
        var_zflaggat_s_slot: &mut f64,
        var_zflagsti_d_slot: &mut f64,
        var_zflagsti_s_slot: &mut f64,
        var_zfrac_slot: &mut f64,
    ) {
        let mut var_alphaje: f64 = *var_alphaje_slot;
        let mut var_alphaje_dn6: f64 = *var_alphaje_dn6_slot;
        let mut var_alphaje_dn7: f64 = *var_alphaje_dn7_slot;
        let mut var_alphaje_dn8: f64 = *var_alphaje_dn8_slot;
        let mut var_alphaje_dn9: f64 = *var_alphaje_dn9_slot;
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_exp_vmax_over_phitd_s: f64 = *var_exp_vmax_over_phitd_s_slot;
        let mut var_expxhf1_d: f64 = *var_expxhf1_d_slot;
        let mut var_expxhf1_s: f64 = *var_expxhf1_s_slot;
        let mut var_expxhf2_d: f64 = *var_expxhf2_d_slot;
        let mut var_expxhf2_d_dn6: f64 = *var_expxhf2_d_dn6_slot;
        let mut var_expxhf2_d_dn7: f64 = *var_expxhf2_d_dn7_slot;
        let mut var_expxhf2_d_dn8: f64 = *var_expxhf2_d_dn8_slot;
        let mut var_expxhf2_d_dn9: f64 = *var_expxhf2_d_dn9_slot;
        let mut var_expxhf2_s: f64 = *var_expxhf2_s_slot;
        let mut var_expxhf2_s_dn6: f64 = *var_expxhf2_s_dn6_slot;
        let mut var_expxhf2_s_dn7: f64 = *var_expxhf2_s_dn7_slot;
        let mut var_expxhf2_s_dn8: f64 = *var_expxhf2_s_dn8_slot;
        let mut var_expxhf2_s_dn9: f64 = *var_expxhf2_s_dn9_slot;
        let mut var_expxhr_d: f64 = *var_expxhr_d_slot;
        let mut var_expxhr_d_dn6: f64 = *var_expxhr_d_dn6_slot;
        let mut var_expxhr_d_dn7: f64 = *var_expxhr_d_dn7_slot;
        let mut var_expxhr_d_dn8: f64 = *var_expxhr_d_dn8_slot;
        let mut var_expxhr_d_dn9: f64 = *var_expxhr_d_dn9_slot;
        let mut var_expxhr_s: f64 = *var_expxhr_s_slot;
        let mut var_expxhr_s_dn6: f64 = *var_expxhr_s_dn6_slot;
        let mut var_expxhr_s_dn7: f64 = *var_expxhr_s_dn7_slot;
        let mut var_expxhr_s_dn8: f64 = *var_expxhr_s_dn8_slot;
        let mut var_expxhr_s_dn9: f64 = *var_expxhr_s_dn9_slot;
        let mut var_guard175: f64 = *var_guard175_slot;
        let mut var_guard176: f64 = *var_guard176_slot;
        let mut var_guard177: f64 = *var_guard177_slot;
        let mut var_guard178: f64 = *var_guard178_slot;
        let mut var_guard179: f64 = *var_guard179_slot;
        let mut var_guard180: f64 = *var_guard180_slot;
        let mut var_guard181: f64 = *var_guard181_slot;
        let mut var_guard182: f64 = *var_guard182_slot;
        let mut var_i1_cor: f64 = *var_i1_cor_slot;
        let mut var_i1_cor_dn6: f64 = *var_i1_cor_dn6_slot;
        let mut var_i1_cor_dn7: f64 = *var_i1_cor_dn7_slot;
        let mut var_i1_cor_dn8: f64 = *var_i1_cor_dn8_slot;
        let mut var_i1_cor_dn9: f64 = *var_i1_cor_dn9_slot;
        let mut var_i2_cor: f64 = *var_i2_cor_slot;
        let mut var_i2_cor_dn6: f64 = *var_i2_cor_dn6_slot;
        let mut var_i2_cor_dn7: f64 = *var_i2_cor_dn7_slot;
        let mut var_i2_cor_dn8: f64 = *var_i2_cor_dn8_slot;
        let mut var_i2_cor_dn9: f64 = *var_i2_cor_dn9_slot;
        let mut var_i3_cor: f64 = *var_i3_cor_slot;
        let mut var_i3_cor_dn6: f64 = *var_i3_cor_dn6_slot;
        let mut var_i3_cor_dn7: f64 = *var_i3_cor_dn7_slot;
        let mut var_i3_cor_dn8: f64 = *var_i3_cor_dn8_slot;
        let mut var_i3_cor_dn9: f64 = *var_i3_cor_dn9_slot;
        let mut var_i4_cor: f64 = *var_i4_cor_slot;
        let mut var_i4_cor_dn6: f64 = *var_i4_cor_dn6_slot;
        let mut var_i4_cor_dn7: f64 = *var_i4_cor_dn7_slot;
        let mut var_i4_cor_dn8: f64 = *var_i4_cor_dn8_slot;
        let mut var_i4_cor_dn9: f64 = *var_i4_cor_dn9_slot;
        let mut var_i5_cor: f64 = *var_i5_cor_slot;
        let mut var_i5_cor_dn6: f64 = *var_i5_cor_dn6_slot;
        let mut var_i5_cor_dn7: f64 = *var_i5_cor_dn7_slot;
        let mut var_i5_cor_dn8: f64 = *var_i5_cor_dn8_slot;
        let mut var_i5_cor_dn9: f64 = *var_i5_cor_dn9_slot;
        let mut var_isatfor1_d: f64 = *var_isatfor1_d_slot;
        let mut var_isatfor1_s: f64 = *var_isatfor1_s_slot;
        let mut var_isatfor2_d: f64 = *var_isatfor2_d_slot;
        let mut var_isatfor2_d_dn6: f64 = *var_isatfor2_d_dn6_slot;
        let mut var_isatfor2_d_dn7: f64 = *var_isatfor2_d_dn7_slot;
        let mut var_isatfor2_d_dn8: f64 = *var_isatfor2_d_dn8_slot;
        let mut var_isatfor2_d_dn9: f64 = *var_isatfor2_d_dn9_slot;
        let mut var_isatfor2_s: f64 = *var_isatfor2_s_slot;
        let mut var_isatfor2_s_dn6: f64 = *var_isatfor2_s_dn6_slot;
        let mut var_isatfor2_s_dn7: f64 = *var_isatfor2_s_dn7_slot;
        let mut var_isatfor2_s_dn8: f64 = *var_isatfor2_s_dn8_slot;
        let mut var_isatfor2_s_dn9: f64 = *var_isatfor2_s_dn9_slot;
        let mut var_isatrev_d: f64 = *var_isatrev_d_slot;
        let mut var_isatrev_d_dn6: f64 = *var_isatrev_d_dn6_slot;
        let mut var_isatrev_d_dn7: f64 = *var_isatrev_d_dn7_slot;
        let mut var_isatrev_d_dn8: f64 = *var_isatrev_d_dn8_slot;
        let mut var_isatrev_d_dn9: f64 = *var_isatrev_d_dn9_slot;
        let mut var_isatrev_s: f64 = *var_isatrev_s_slot;
        let mut var_isatrev_s_dn6: f64 = *var_isatrev_s_dn6_slot;
        let mut var_isatrev_s_dn7: f64 = *var_isatrev_s_dn7_slot;
        let mut var_isatrev_s_dn8: f64 = *var_isatrev_s_dn8_slot;
        let mut var_isatrev_s_dn9: f64 = *var_isatrev_s_dn9_slot;
        let mut var_m0_rev: f64 = *var_m0_rev_slot;
        let mut var_m0_rev_dn6: f64 = *var_m0_rev_dn6_slot;
        let mut var_m0_rev_dn7: f64 = *var_m0_rev_dn7_slot;
        let mut var_m0_rev_dn8: f64 = *var_m0_rev_dn8_slot;
        let mut var_m0_rev_dn9: f64 = *var_m0_rev_dn9_slot;
        let mut var_m0flag_d: f64 = *var_m0flag_d_slot;
        let mut var_m0flag_s: f64 = *var_m0flag_s_slot;
        let mut var_mcor_rev: f64 = *var_mcor_rev_slot;
        let mut var_mcor_rev_dn6: f64 = *var_mcor_rev_dn6_slot;
        let mut var_mcor_rev_dn7: f64 = *var_mcor_rev_dn7_slot;
        let mut var_mcor_rev_dn8: f64 = *var_mcor_rev_dn8_slot;
        let mut var_mcor_rev_dn9: f64 = *var_mcor_rev_dn9_slot;
        let mut var_mfor1_d: f64 = *var_mfor1_d_slot;
        let mut var_mfor1_s: f64 = *var_mfor1_s_slot;
        let mut var_mfor2_d: f64 = *var_mfor2_d_slot;
        let mut var_mfor2_d_dn6: f64 = *var_mfor2_d_dn6_slot;
        let mut var_mfor2_d_dn7: f64 = *var_mfor2_d_dn7_slot;
        let mut var_mfor2_d_dn8: f64 = *var_mfor2_d_dn8_slot;
        let mut var_mfor2_d_dn9: f64 = *var_mfor2_d_dn9_slot;
        let mut var_mfor2_s: f64 = *var_mfor2_s_slot;
        let mut var_mfor2_s_dn6: f64 = *var_mfor2_s_dn6_slot;
        let mut var_mfor2_s_dn7: f64 = *var_mfor2_s_dn7_slot;
        let mut var_mfor2_s_dn8: f64 = *var_mfor2_s_dn8_slot;
        let mut var_mfor2_s_dn9: f64 = *var_mfor2_s_dn9_slot;
        let mut var_mrev_d: f64 = *var_mrev_d_slot;
        let mut var_mrev_d_dn6: f64 = *var_mrev_d_dn6_slot;
        let mut var_mrev_d_dn7: f64 = *var_mrev_d_dn7_slot;
        let mut var_mrev_d_dn8: f64 = *var_mrev_d_dn8_slot;
        let mut var_mrev_d_dn9: f64 = *var_mrev_d_dn9_slot;
        let mut var_mrev_s: f64 = *var_mrev_s_slot;
        let mut var_mrev_s_dn6: f64 = *var_mrev_s_dn6_slot;
        let mut var_mrev_s_dn7: f64 = *var_mrev_s_dn7_slot;
        let mut var_mrev_s_dn8: f64 = *var_mrev_s_dn8_slot;
        let mut var_mrev_s_dn9: f64 = *var_mrev_s_dn9_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_tt0: f64 = *var_tt0_slot;
        let mut var_tt1: f64 = *var_tt1_slot;
        let mut var_tt1_dn6: f64 = *var_tt1_dn6_slot;
        let mut var_tt1_dn7: f64 = *var_tt1_dn7_slot;
        let mut var_tt1_dn8: f64 = *var_tt1_dn8_slot;
        let mut var_tt1_dn9: f64 = *var_tt1_dn9_slot;
        let mut var_tt2: f64 = *var_tt2_slot;
        let mut var_tt2_dn6: f64 = *var_tt2_dn6_slot;
        let mut var_tt2_dn7: f64 = *var_tt2_dn7_slot;
        let mut var_tt2_dn8: f64 = *var_tt2_dn8_slot;
        let mut var_tt2_dn9: f64 = *var_tt2_dn9_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmax_s: f64 = *var_vmax_s_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;
        let mut var_xhighf1_d: f64 = *var_xhighf1_d_slot;
        let mut var_xhighf1_s: f64 = *var_xhighf1_s_slot;
        let mut var_xhighf2_d: f64 = *var_xhighf2_d_slot;
        let mut var_xhighf2_d_dn6: f64 = *var_xhighf2_d_dn6_slot;
        let mut var_xhighf2_d_dn7: f64 = *var_xhighf2_d_dn7_slot;
        let mut var_xhighf2_d_dn8: f64 = *var_xhighf2_d_dn8_slot;
        let mut var_xhighf2_d_dn9: f64 = *var_xhighf2_d_dn9_slot;
        let mut var_xhighf2_s: f64 = *var_xhighf2_s_slot;
        let mut var_xhighf2_s_dn6: f64 = *var_xhighf2_s_dn6_slot;
        let mut var_xhighf2_s_dn7: f64 = *var_xhighf2_s_dn7_slot;
        let mut var_xhighf2_s_dn8: f64 = *var_xhighf2_s_dn8_slot;
        let mut var_xhighf2_s_dn9: f64 = *var_xhighf2_s_dn9_slot;
        let mut var_xhighr_d: f64 = *var_xhighr_d_slot;
        let mut var_xhighr_d_dn6: f64 = *var_xhighr_d_dn6_slot;
        let mut var_xhighr_d_dn7: f64 = *var_xhighr_d_dn7_slot;
        let mut var_xhighr_d_dn8: f64 = *var_xhighr_d_dn8_slot;
        let mut var_xhighr_d_dn9: f64 = *var_xhighr_d_dn9_slot;
        let mut var_xhighr_s: f64 = *var_xhighr_s_slot;
        let mut var_xhighr_s_dn6: f64 = *var_xhighr_s_dn6_slot;
        let mut var_xhighr_s_dn7: f64 = *var_xhighr_s_dn7_slot;
        let mut var_xhighr_s_dn8: f64 = *var_xhighr_s_dn8_slot;
        let mut var_xhighr_s_dn9: f64 = *var_xhighr_s_dn9_slot;
        let mut var_zflagbot_d: f64 = *var_zflagbot_d_slot;
        let mut var_zflagbot_s: f64 = *var_zflagbot_s_slot;
        let mut var_zflaggat_d: f64 = *var_zflaggat_d_slot;
        let mut var_zflaggat_s: f64 = *var_zflaggat_s_slot;
        let mut var_zflagsti_d: f64 = *var_zflagsti_d_slot;
        let mut var_zflagsti_s: f64 = *var_zflagsti_s_slot;
        let mut var_zfrac: f64 = *var_zfrac_slot;

        var_vch_s = 0.0;

        var_vch_d = 0.0;

        var_vbbtlim_s = 0.0;

        var_vbbtlim_d = 0.0;

        var_vmax_s = 0.0;

        var_vmax_d = 0.0;

        var_exp_vmax_over_phitd_s = 0.0;

        var_exp_vmax_over_phitd_d = 0.0;

        var_isatfor1_s = 0.0;

        var_isatfor1_d = 0.0;

        var_mfor1_s = 1.0;

        var_mfor1_d = 1.0;

        var_isatfor2_s = 0.0;
        var_isatfor2_s_dn6 = 0.0;
        var_isatfor2_s_dn7 = 0.0;
        var_isatfor2_s_dn8 = 0.0;
        var_isatfor2_s_dn9 = 0.0;

        var_isatfor2_d = 0.0;
        var_isatfor2_d_dn6 = 0.0;
        var_isatfor2_d_dn7 = 0.0;
        var_isatfor2_d_dn8 = 0.0;
        var_isatfor2_d_dn9 = 0.0;

        var_mfor2_s = 1.0;
        var_mfor2_s_dn6 = 0.0;
        var_mfor2_s_dn7 = 0.0;
        var_mfor2_s_dn8 = 0.0;
        var_mfor2_s_dn9 = 0.0;

        var_mfor2_d = 1.0;
        var_mfor2_d_dn6 = 0.0;
        var_mfor2_d_dn7 = 0.0;
        var_mfor2_d_dn8 = 0.0;
        var_mfor2_d_dn9 = 0.0;

        var_isatrev_s = 0.0;
        var_isatrev_s_dn6 = 0.0;
        var_isatrev_s_dn7 = 0.0;
        var_isatrev_s_dn8 = 0.0;
        var_isatrev_s_dn9 = 0.0;

        var_isatrev_d = 0.0;
        var_isatrev_d_dn6 = 0.0;
        var_isatrev_d_dn7 = 0.0;
        var_isatrev_d_dn8 = 0.0;
        var_isatrev_d_dn9 = 0.0;

        var_mrev_s = 1.0;
        var_mrev_s_dn6 = 0.0;
        var_mrev_s_dn7 = 0.0;
        var_mrev_s_dn8 = 0.0;
        var_mrev_s_dn9 = 0.0;

        var_mrev_d = 1.0;
        var_mrev_d_dn6 = 0.0;
        var_mrev_d_dn7 = 0.0;
        var_mrev_d_dn8 = 0.0;
        var_mrev_d_dn9 = 0.0;

        var_m0flag_s = 0.0;

        var_m0flag_d = 0.0;

        var_xhighf1_s = 0.0;

        var_xhighf1_d = 0.0;

        var_expxhf1_s = 0.0;

        var_expxhf1_d = 0.0;

        var_xhighf2_s = 0.0;
        var_xhighf2_s_dn6 = 0.0;
        var_xhighf2_s_dn7 = 0.0;
        var_xhighf2_s_dn8 = 0.0;
        var_xhighf2_s_dn9 = 0.0;

        var_xhighf2_d = 0.0;
        var_xhighf2_d_dn6 = 0.0;
        var_xhighf2_d_dn7 = 0.0;
        var_xhighf2_d_dn8 = 0.0;
        var_xhighf2_d_dn9 = 0.0;

        var_expxhf2_s = 0.0;
        var_expxhf2_s_dn6 = 0.0;
        var_expxhf2_s_dn7 = 0.0;
        var_expxhf2_s_dn8 = 0.0;
        var_expxhf2_s_dn9 = 0.0;

        var_expxhf2_d = 0.0;
        var_expxhf2_d_dn6 = 0.0;
        var_expxhf2_d_dn7 = 0.0;
        var_expxhf2_d_dn8 = 0.0;
        var_expxhf2_d_dn9 = 0.0;

        var_xhighr_s = 0.0;
        var_xhighr_s_dn6 = 0.0;
        var_xhighr_s_dn7 = 0.0;
        var_xhighr_s_dn8 = 0.0;
        var_xhighr_s_dn9 = 0.0;

        var_xhighr_d = 0.0;
        var_xhighr_d_dn6 = 0.0;
        var_xhighr_d_dn7 = 0.0;
        var_xhighr_d_dn8 = 0.0;
        var_xhighr_d_dn9 = 0.0;

        var_expxhr_s = 0.0;
        var_expxhr_s_dn6 = 0.0;
        var_expxhr_s_dn7 = 0.0;
        var_expxhr_s_dn8 = 0.0;
        var_expxhr_s_dn9 = 0.0;

        var_expxhr_d = 0.0;
        var_expxhr_d_dn6 = 0.0;
        var_expxhr_d_dn7 = 0.0;
        var_expxhr_d_dn8 = 0.0;
        var_expxhr_d_dn9 = 0.0;

        var_zflagbot_s = 1.0;

        var_zflagbot_d = 1.0;

        var_zflagsti_s = 1.0;

        var_zflagsti_d = 1.0;

        var_zflaggat_s = 1.0;

        var_zflaggat_d = 1.0;

        var_m0_rev = 0.0;
        var_m0_rev_dn6 = 0.0;
        var_m0_rev_dn7 = 0.0;
        var_m0_rev_dn8 = 0.0;
        var_m0_rev_dn9 = 0.0;

        var_mcor_rev = 0.0;
        var_mcor_rev_dn6 = 0.0;
        var_mcor_rev_dn7 = 0.0;
        var_mcor_rev_dn8 = 0.0;
        var_mcor_rev_dn9 = 0.0;

        var_i1_cor = 0.0;
        var_i1_cor_dn6 = 0.0;
        var_i1_cor_dn7 = 0.0;
        var_i1_cor_dn8 = 0.0;
        var_i1_cor_dn9 = 0.0;

        var_i2_cor = 0.0;
        var_i2_cor_dn6 = 0.0;
        var_i2_cor_dn7 = 0.0;
        var_i2_cor_dn8 = 0.0;
        var_i2_cor_dn9 = 0.0;

        var_i3_cor = 0.0;
        var_i3_cor_dn6 = 0.0;
        var_i3_cor_dn7 = 0.0;
        var_i3_cor_dn8 = 0.0;
        var_i3_cor_dn9 = 0.0;

        var_i4_cor = 0.0;
        var_i4_cor_dn6 = 0.0;
        var_i4_cor_dn7 = 0.0;
        var_i4_cor_dn8 = 0.0;
        var_i4_cor_dn9 = 0.0;

        var_i5_cor = 0.0;
        var_i5_cor_dn6 = 0.0;
        var_i5_cor_dn7 = 0.0;
        var_i5_cor_dn8 = 0.0;
        var_i5_cor_dn9 = 0.0;

        var_tt0 = 0.0;

        var_tt1 = 0.0;
        var_tt1_dn6 = 0.0;
        var_tt1_dn7 = 0.0;
        var_tt1_dn8 = 0.0;
        var_tt1_dn9 = 0.0;

        var_tt2 = 0.0;
        var_tt2_dn6 = 0.0;
        var_tt2_dn7 = 0.0;
        var_tt2_dn8 = 0.0;
        var_tt2_dn9 = 0.0;

        var_zfrac = 0.0;

        var_alphaje = 0.0;
        var_alphaje_dn6 = 0.0;
        var_alphaje_dn7 = 0.0;
        var_alphaje_dn8 = 0.0;
        var_alphaje_dn9 = 0.0;

        let assign13050_e11178: f64 = if p.p43 > 0.0 { 1.0 } else { 0.0 };
        var_guard175 = assign13050_e11178;

        let assign13060_e11181: f64 = (var_idsatbot * var_absource_i);
        let assign13060_e11183: f64 = if assign13060_e11181 > 0.0 { 1.0 } else { 0.0 };
        var_guard176 = assign13060_e11183;

        let (assign13070_e11198,) = {
    if ((var_guard175 != 0.0) && (var_guard176 != 0.0)) {
        let assign13070_e11191: f64 = (var_idsatbot * var_absource_i);
        let assign13070_e11192: f64 = (p.p839 / assign13070_e11191);
        let assign13070_e11194: f64 = (assign13070_e11192 + 1.0);
        let assign13070_e11195: f64 = (assign13070_e11194).ln();
        let assign13070_e11196: f64 = (var_phitd * assign13070_e11195);
        (assign13070_e11196,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13070_e11198;

        let (assign13080_e11205,) = {
    if ((var_guard175 != 0.0) && (var_guard176 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13080_e11205;

        let assign13090_e11208: f64 = (var_idsatsti * var_lssource_i);
        let assign13090_e11210: f64 = if assign13090_e11208 > 0.0 { 1.0 } else { 0.0 };
        var_guard177 = assign13090_e11210;

        let (assign13100_e11225,) = {
    if ((var_guard175 != 0.0) && (var_guard177 != 0.0)) {
        let assign13100_e11218: f64 = (var_idsatsti * var_lssource_i);
        let assign13100_e11219: f64 = (p.p839 / assign13100_e11218);
        let assign13100_e11221: f64 = (assign13100_e11219 + 1.0);
        let assign13100_e11222: f64 = (assign13100_e11221).ln();
        let assign13100_e11223: f64 = (var_phitd * assign13100_e11222);
        (assign13100_e11223,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13100_e11225;

        let (assign13110_e11232,) = {
    if ((var_guard175 != 0.0) && (var_guard177 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13110_e11232;

        let assign13120_e11235: f64 = (var_idsatgat * var_lgsource_i);
        let assign13120_e11237: f64 = if assign13120_e11235 > 0.0 { 1.0 } else { 0.0 };
        var_guard178 = assign13120_e11237;

        let (assign13130_e11252,) = {
    if ((var_guard175 != 0.0) && (var_guard178 != 0.0)) {
        let assign13130_e11245: f64 = (var_idsatgat * var_lgsource_i);
        let assign13130_e11246: f64 = (p.p839 / assign13130_e11245);
        let assign13130_e11248: f64 = (assign13130_e11246 + 1.0);
        let assign13130_e11249: f64 = (assign13130_e11248).ln();
        let assign13130_e11250: f64 = (var_phitd * assign13130_e11249);
        (assign13130_e11250,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13130_e11252;

        let (assign13140_e11259,) = {
    if ((var_guard175 != 0.0) && (var_guard178 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13140_e11259;

        let (assign13150_e11267,) = {
    if (var_guard175 != 0.0) {
        let assign13150_e11263: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign13150_e11265: f64 = (assign13150_e11263).min(var_vmaxgat);
        (assign13150_e11265,)
    } else {
        (var_vmax_s,)
    }
};
        var_vmax_s = assign13150_e11267;

        let assign13160_e11270: f64 = (var_vmax_s * var_phitdinv);
        let assign13160_e11271: f64 = (assign13160_e11270).abs();
        let assign13160_e11273: f64 = if assign13160_e11271 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard179 = assign13160_e11273;

        let (assign13170_e11282,) = {
    if ((var_guard175 != 0.0) && (var_guard179 != 0.0)) {
        let assign13170_e11279: f64 = (var_vmax_s * var_phitdinv);
        let assign13170_e11280: f64 = (assign13170_e11279).exp();
        (assign13170_e11280,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign13170_e11282;

        let assign13180_e11285: f64 = (var_vmax_s * var_phitdinv);
        let assign13180_e11287: f64 = if assign13180_e11285 < 0.0 { 1.0 } else { 0.0 };
        var_guard180 = assign13180_e11287;

        let (assign13190_e11327,) = {
    if (((var_guard175 != 0.0) && (var_guard179 == 0.0)) && (var_guard180 != 0.0)) {
        let assign13190_e11297: f64 = (-230.25850929940458);
        let assign13190_e11300: f64 = (var_vmax_s * var_phitdinv);
        let assign13190_e11301: f64 = (assign13190_e11297 - assign13190_e11300);
        let assign13190_e11305: f64 = (-230.25850929940458);
        let assign13190_e11308: f64 = (var_vmax_s * var_phitdinv);
        let assign13190_e11309: f64 = (assign13190_e11305 - assign13190_e11308);
        let assign13190_e11312: f64 = (-230.25850929940458);
        let assign13190_e11315: f64 = (var_vmax_s * var_phitdinv);
        let assign13190_e11316: f64 = (assign13190_e11312 - assign13190_e11315);
        let assign13190_e11318: f64 = (assign13190_e11316 * 0.3333333333333333);
        let assign13190_e11319: f64 = (1.0 + assign13190_e11318);
        let assign13190_e11320: f64 = (assign13190_e11309 * assign13190_e11319);
        let assign13190_e11321: f64 = (0.5 * assign13190_e11320);
        let assign13190_e11322: f64 = (1.0 + assign13190_e11321);
        let assign13190_e11323: f64 = (assign13190_e11301 * assign13190_e11322);
        let assign13190_e11324: f64 = (1.0 + assign13190_e11323);
        let assign13190_e11325: f64 = (1e-100 / assign13190_e11324);
        (assign13190_e11325,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign13190_e11327;

        let (assign13200_e11365,) = {
    if (((var_guard175 != 0.0) && (var_guard179 == 0.0)) && (var_guard180 == 0.0)) {
        let assign13200_e11339: f64 = (var_vmax_s * var_phitdinv);
        let assign13200_e11341: f64 = (assign13200_e11339 - 230.25850929940458);
        let assign13200_e11346: f64 = (var_vmax_s * var_phitdinv);
        let assign13200_e11348: f64 = (assign13200_e11346 - 230.25850929940458);
        let assign13200_e11352: f64 = (var_vmax_s * var_phitdinv);
        let assign13200_e11354: f64 = (assign13200_e11352 - 230.25850929940458);
        let assign13200_e11356: f64 = (assign13200_e11354 * 0.3333333333333333);
        let assign13200_e11357: f64 = (1.0 + assign13200_e11356);
        let assign13200_e11358: f64 = (assign13200_e11348 * assign13200_e11357);
        let assign13200_e11359: f64 = (0.5 * assign13200_e11358);
        let assign13200_e11360: f64 = (1.0 + assign13200_e11359);
        let assign13200_e11361: f64 = (assign13200_e11341 * assign13200_e11360);
        let assign13200_e11362: f64 = (1.0 + assign13200_e11361);
        let assign13200_e11363: f64 = (1e100 * assign13200_e11362);
        (assign13200_e11363,)
    } else {
        (var_exp_vmax_over_phitd_s,)
    }
};
        var_exp_vmax_over_phitd_s = assign13200_e11365;

        let (assign13210_e11369,) = {
    if (var_guard175 != 0.0) {
        (var_vbibot,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13210_e11369;

        let (assign13220_e11373,) = {
    if (var_guard175 != 0.0) {
        (var_vbisti,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13220_e11373;

        let (assign13230_e11377,) = {
    if (var_guard175 != 0.0) {
        (var_vbigat,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13230_e11377;

        let (assign13240_e11381,) = {
    if (var_guard175 != 0.0) {
        (p.p848,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13240_e11381;

        let (assign13250_e11385,) = {
    if (var_guard175 != 0.0) {
        (p.p849,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13250_e11385;

        let (assign13260_e11389,) = {
    if (var_guard175 != 0.0) {
        (p.p850,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13260_e11389;

        let (assign13270_e11393,) = {
    if (var_guard175 != 0.0) {
        (p.p845,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13270_e11393;

        let (assign13280_e11397,) = {
    if (var_guard175 != 0.0) {
        (p.p846,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13280_e11397;

        let (assign13290_e11401,) = {
    if (var_guard175 != 0.0) {
        (p.p847,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13290_e11401;

        let assign13300_e11404: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard181 = assign13300_e11404;

        let (assign13310_e11412,) = {
    if ((var_guard175 != 0.0) && (var_guard181 != 0.0)) {
        let assign13310_e11410: f64 = (var_vbisti + var_vbigat);
        (assign13310_e11410,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13310_e11412;

        let (assign13320_e11422,) = {
    if ((var_guard175 != 0.0) && (var_guard181 != 0.0)) {
        let assign13320_e11419: f64 = (p.p849).min(p.p850);
        let assign13320_e11420: f64 = (0.9 * assign13320_e11419);
        (assign13320_e11420,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13320_e11422;

        let (assign13330_e11430,) = {
    if ((var_guard175 != 0.0) && (var_guard181 != 0.0)) {
        let assign13330_e11428: f64 = (p.p846 + p.p847);
        (assign13330_e11428,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13330_e11430;

        let assign13340_e11433: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard182 = assign13340_e11433;

        let (assign13350_e11441,) = {
    if ((var_guard175 != 0.0) && (var_guard182 != 0.0)) {
        let assign13350_e11439: f64 = (var_vbibot + var_vbigat);
        (assign13350_e11439,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13350_e11441;

        let (assign13360_e11451,) = {
    if ((var_guard175 != 0.0) && (var_guard182 != 0.0)) {
        let assign13360_e11448: f64 = (p.p848).min(p.p850);
        let assign13360_e11449: f64 = (0.9 * assign13360_e11448);
        (assign13360_e11449,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13360_e11451;

        *var_alphaje_slot = var_alphaje;
        *var_alphaje_dn6_slot = var_alphaje_dn6;
        *var_alphaje_dn7_slot = var_alphaje_dn7;
        *var_alphaje_dn8_slot = var_alphaje_dn8;
        *var_alphaje_dn9_slot = var_alphaje_dn9;
        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_exp_vmax_over_phitd_s_slot = var_exp_vmax_over_phitd_s;
        *var_expxhf1_d_slot = var_expxhf1_d;
        *var_expxhf1_s_slot = var_expxhf1_s;
        *var_expxhf2_d_slot = var_expxhf2_d;
        *var_expxhf2_d_dn6_slot = var_expxhf2_d_dn6;
        *var_expxhf2_d_dn7_slot = var_expxhf2_d_dn7;
        *var_expxhf2_d_dn8_slot = var_expxhf2_d_dn8;
        *var_expxhf2_d_dn9_slot = var_expxhf2_d_dn9;
        *var_expxhf2_s_slot = var_expxhf2_s;
        *var_expxhf2_s_dn6_slot = var_expxhf2_s_dn6;
        *var_expxhf2_s_dn7_slot = var_expxhf2_s_dn7;
        *var_expxhf2_s_dn8_slot = var_expxhf2_s_dn8;
        *var_expxhf2_s_dn9_slot = var_expxhf2_s_dn9;
        *var_expxhr_d_slot = var_expxhr_d;
        *var_expxhr_d_dn6_slot = var_expxhr_d_dn6;
        *var_expxhr_d_dn7_slot = var_expxhr_d_dn7;
        *var_expxhr_d_dn8_slot = var_expxhr_d_dn8;
        *var_expxhr_d_dn9_slot = var_expxhr_d_dn9;
        *var_expxhr_s_slot = var_expxhr_s;
        *var_expxhr_s_dn6_slot = var_expxhr_s_dn6;
        *var_expxhr_s_dn7_slot = var_expxhr_s_dn7;
        *var_expxhr_s_dn8_slot = var_expxhr_s_dn8;
        *var_expxhr_s_dn9_slot = var_expxhr_s_dn9;
        *var_guard175_slot = var_guard175;
        *var_guard176_slot = var_guard176;
        *var_guard177_slot = var_guard177;
        *var_guard178_slot = var_guard178;
        *var_guard179_slot = var_guard179;
        *var_guard180_slot = var_guard180;
        *var_guard181_slot = var_guard181;
        *var_guard182_slot = var_guard182;
        *var_i1_cor_slot = var_i1_cor;
        *var_i1_cor_dn6_slot = var_i1_cor_dn6;
        *var_i1_cor_dn7_slot = var_i1_cor_dn7;
        *var_i1_cor_dn8_slot = var_i1_cor_dn8;
        *var_i1_cor_dn9_slot = var_i1_cor_dn9;
        *var_i2_cor_slot = var_i2_cor;
        *var_i2_cor_dn6_slot = var_i2_cor_dn6;
        *var_i2_cor_dn7_slot = var_i2_cor_dn7;
        *var_i2_cor_dn8_slot = var_i2_cor_dn8;
        *var_i2_cor_dn9_slot = var_i2_cor_dn9;
        *var_i3_cor_slot = var_i3_cor;
        *var_i3_cor_dn6_slot = var_i3_cor_dn6;
        *var_i3_cor_dn7_slot = var_i3_cor_dn7;
        *var_i3_cor_dn8_slot = var_i3_cor_dn8;
        *var_i3_cor_dn9_slot = var_i3_cor_dn9;
        *var_i4_cor_slot = var_i4_cor;
        *var_i4_cor_dn6_slot = var_i4_cor_dn6;
        *var_i4_cor_dn7_slot = var_i4_cor_dn7;
        *var_i4_cor_dn8_slot = var_i4_cor_dn8;
        *var_i4_cor_dn9_slot = var_i4_cor_dn9;
        *var_i5_cor_slot = var_i5_cor;
        *var_i5_cor_dn6_slot = var_i5_cor_dn6;
        *var_i5_cor_dn7_slot = var_i5_cor_dn7;
        *var_i5_cor_dn8_slot = var_i5_cor_dn8;
        *var_i5_cor_dn9_slot = var_i5_cor_dn9;
        *var_isatfor1_d_slot = var_isatfor1_d;
        *var_isatfor1_s_slot = var_isatfor1_s;
        *var_isatfor2_d_slot = var_isatfor2_d;
        *var_isatfor2_d_dn6_slot = var_isatfor2_d_dn6;
        *var_isatfor2_d_dn7_slot = var_isatfor2_d_dn7;
        *var_isatfor2_d_dn8_slot = var_isatfor2_d_dn8;
        *var_isatfor2_d_dn9_slot = var_isatfor2_d_dn9;
        *var_isatfor2_s_slot = var_isatfor2_s;
        *var_isatfor2_s_dn6_slot = var_isatfor2_s_dn6;
        *var_isatfor2_s_dn7_slot = var_isatfor2_s_dn7;
        *var_isatfor2_s_dn8_slot = var_isatfor2_s_dn8;
        *var_isatfor2_s_dn9_slot = var_isatfor2_s_dn9;
        *var_isatrev_d_slot = var_isatrev_d;
        *var_isatrev_d_dn6_slot = var_isatrev_d_dn6;
        *var_isatrev_d_dn7_slot = var_isatrev_d_dn7;
        *var_isatrev_d_dn8_slot = var_isatrev_d_dn8;
        *var_isatrev_d_dn9_slot = var_isatrev_d_dn9;
        *var_isatrev_s_slot = var_isatrev_s;
        *var_isatrev_s_dn6_slot = var_isatrev_s_dn6;
        *var_isatrev_s_dn7_slot = var_isatrev_s_dn7;
        *var_isatrev_s_dn8_slot = var_isatrev_s_dn8;
        *var_isatrev_s_dn9_slot = var_isatrev_s_dn9;
        *var_m0_rev_slot = var_m0_rev;
        *var_m0_rev_dn6_slot = var_m0_rev_dn6;
        *var_m0_rev_dn7_slot = var_m0_rev_dn7;
        *var_m0_rev_dn8_slot = var_m0_rev_dn8;
        *var_m0_rev_dn9_slot = var_m0_rev_dn9;
        *var_m0flag_d_slot = var_m0flag_d;
        *var_m0flag_s_slot = var_m0flag_s;
        *var_mcor_rev_slot = var_mcor_rev;
        *var_mcor_rev_dn6_slot = var_mcor_rev_dn6;
        *var_mcor_rev_dn7_slot = var_mcor_rev_dn7;
        *var_mcor_rev_dn8_slot = var_mcor_rev_dn8;
        *var_mcor_rev_dn9_slot = var_mcor_rev_dn9;
        *var_mfor1_d_slot = var_mfor1_d;
        *var_mfor1_s_slot = var_mfor1_s;
        *var_mfor2_d_slot = var_mfor2_d;
        *var_mfor2_d_dn6_slot = var_mfor2_d_dn6;
        *var_mfor2_d_dn7_slot = var_mfor2_d_dn7;
        *var_mfor2_d_dn8_slot = var_mfor2_d_dn8;
        *var_mfor2_d_dn9_slot = var_mfor2_d_dn9;
        *var_mfor2_s_slot = var_mfor2_s;
        *var_mfor2_s_dn6_slot = var_mfor2_s_dn6;
        *var_mfor2_s_dn7_slot = var_mfor2_s_dn7;
        *var_mfor2_s_dn8_slot = var_mfor2_s_dn8;
        *var_mfor2_s_dn9_slot = var_mfor2_s_dn9;
        *var_mrev_d_slot = var_mrev_d;
        *var_mrev_d_dn6_slot = var_mrev_d_dn6;
        *var_mrev_d_dn7_slot = var_mrev_d_dn7;
        *var_mrev_d_dn8_slot = var_mrev_d_dn8;
        *var_mrev_d_dn9_slot = var_mrev_d_dn9;
        *var_mrev_s_slot = var_mrev_s;
        *var_mrev_s_dn6_slot = var_mrev_s_dn6;
        *var_mrev_s_dn7_slot = var_mrev_s_dn7;
        *var_mrev_s_dn8_slot = var_mrev_s_dn8;
        *var_mrev_s_dn9_slot = var_mrev_s_dn9;
        *var_pbot2_slot = var_pbot2;
        *var_pgat2_slot = var_pgat2;
        *var_psti2_slot = var_psti2;
        *var_tt0_slot = var_tt0;
        *var_tt1_slot = var_tt1;
        *var_tt1_dn6_slot = var_tt1_dn6;
        *var_tt1_dn7_slot = var_tt1_dn7;
        *var_tt1_dn8_slot = var_tt1_dn8;
        *var_tt1_dn9_slot = var_tt1_dn9;
        *var_tt2_slot = var_tt2;
        *var_tt2_dn6_slot = var_tt2_dn6;
        *var_tt2_dn7_slot = var_tt2_dn7;
        *var_tt2_dn8_slot = var_tt2_dn8;
        *var_tt2_dn9_slot = var_tt2_dn9;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vch_d_slot = var_vch_d;
        *var_vch_s_slot = var_vch_s;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmax_s_slot = var_vmax_s;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxsti_slot = var_vmaxsti;
        *var_xhighf1_d_slot = var_xhighf1_d;
        *var_xhighf1_s_slot = var_xhighf1_s;
        *var_xhighf2_d_slot = var_xhighf2_d;
        *var_xhighf2_d_dn6_slot = var_xhighf2_d_dn6;
        *var_xhighf2_d_dn7_slot = var_xhighf2_d_dn7;
        *var_xhighf2_d_dn8_slot = var_xhighf2_d_dn8;
        *var_xhighf2_d_dn9_slot = var_xhighf2_d_dn9;
        *var_xhighf2_s_slot = var_xhighf2_s;
        *var_xhighf2_s_dn6_slot = var_xhighf2_s_dn6;
        *var_xhighf2_s_dn7_slot = var_xhighf2_s_dn7;
        *var_xhighf2_s_dn8_slot = var_xhighf2_s_dn8;
        *var_xhighf2_s_dn9_slot = var_xhighf2_s_dn9;
        *var_xhighr_d_slot = var_xhighr_d;
        *var_xhighr_d_dn6_slot = var_xhighr_d_dn6;
        *var_xhighr_d_dn7_slot = var_xhighr_d_dn7;
        *var_xhighr_d_dn8_slot = var_xhighr_d_dn8;
        *var_xhighr_d_dn9_slot = var_xhighr_d_dn9;
        *var_xhighr_s_slot = var_xhighr_s;
        *var_xhighr_s_dn6_slot = var_xhighr_s_dn6;
        *var_xhighr_s_dn7_slot = var_xhighr_s_dn7;
        *var_xhighr_s_dn8_slot = var_xhighr_s_dn8;
        *var_xhighr_s_dn9_slot = var_xhighr_s_dn9;
        *var_zflagbot_d_slot = var_zflagbot_d;
        *var_zflagbot_s_slot = var_zflagbot_s;
        *var_zflaggat_d_slot = var_zflaggat_d;
        *var_zflaggat_s_slot = var_zflaggat_s;
        *var_zflagsti_d_slot = var_zflagsti_d;
        *var_zflagsti_s_slot = var_zflagsti_s;
        *var_zfrac_slot = var_zfrac;
    }

    pub(super) fn stamp_transient_block_19(
        p: &Parameters,
        var_abdrain_i: f64,
        var_guard175: f64,
        var_guard182: f64,
        var_idsatbot_d: f64,
        var_idsatgat_d: f64,
        var_idsatsti_d: f64,
        var_lgdrain_i: f64,
        var_lgsource_i: f64,
        var_lsdrain_i: f64,
        var_pbotd_i: f64,
        var_pgatd_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_pstid_i: f64,
        var_swjunexp_i: f64,
        var_vbibot: f64,
        var_vbibot_d: f64,
        var_vbigat_d: f64,
        var_vbirbotd_i: f64,
        var_vbirgatd_i: f64,
        var_vbirstid_i: f64,
        var_vbisti: f64,
        var_vbisti_d: f64,
        var_exp_vmax_over_phitd_d_slot: &mut f64,
        var_guard183_slot: &mut f64,
        var_guard184_slot: &mut f64,
        var_guard185_slot: &mut f64,
        var_guard186_slot: &mut f64,
        var_guard187_slot: &mut f64,
        var_guard188_slot: &mut f64,
        var_guard189_slot: &mut f64,
        var_guard190_slot: &mut f64,
        var_guard191_slot: &mut f64,
        var_guard192_slot: &mut f64,
        var_pbot2_slot: &mut f64,
        var_pgat2_slot: &mut f64,
        var_pmax_slot: &mut f64,
        var_psti2_slot: &mut f64,
        var_vbbtlim_d_slot: &mut f64,
        var_vbbtlim_s_slot: &mut f64,
        var_vbibot2_slot: &mut f64,
        var_vbibot2r_slot: &mut f64,
        var_vbigat2_slot: &mut f64,
        var_vbigat2r_slot: &mut f64,
        var_vbimin_d_slot: &mut f64,
        var_vbimin_s_slot: &mut f64,
        var_vbisti2_slot: &mut f64,
        var_vbisti2r_slot: &mut f64,
        var_vch_d_slot: &mut f64,
        var_vch_s_slot: &mut f64,
        var_vfmin_d_slot: &mut f64,
        var_vfmin_s_slot: &mut f64,
        var_vmax_d_slot: &mut f64,
        var_vmaxbot_slot: &mut f64,
        var_vmaxgat_slot: &mut f64,
        var_vmaxsti_slot: &mut f64,
    ) {
        let mut var_exp_vmax_over_phitd_d: f64 = *var_exp_vmax_over_phitd_d_slot;
        let mut var_guard183: f64 = *var_guard183_slot;
        let mut var_guard184: f64 = *var_guard184_slot;
        let mut var_guard185: f64 = *var_guard185_slot;
        let mut var_guard186: f64 = *var_guard186_slot;
        let mut var_guard187: f64 = *var_guard187_slot;
        let mut var_guard188: f64 = *var_guard188_slot;
        let mut var_guard189: f64 = *var_guard189_slot;
        let mut var_guard190: f64 = *var_guard190_slot;
        let mut var_guard191: f64 = *var_guard191_slot;
        let mut var_guard192: f64 = *var_guard192_slot;
        let mut var_pbot2: f64 = *var_pbot2_slot;
        let mut var_pgat2: f64 = *var_pgat2_slot;
        let mut var_pmax: f64 = *var_pmax_slot;
        let mut var_psti2: f64 = *var_psti2_slot;
        let mut var_vbbtlim_d: f64 = *var_vbbtlim_d_slot;
        let mut var_vbbtlim_s: f64 = *var_vbbtlim_s_slot;
        let mut var_vbibot2: f64 = *var_vbibot2_slot;
        let mut var_vbibot2r: f64 = *var_vbibot2r_slot;
        let mut var_vbigat2: f64 = *var_vbigat2_slot;
        let mut var_vbigat2r: f64 = *var_vbigat2r_slot;
        let mut var_vbimin_d: f64 = *var_vbimin_d_slot;
        let mut var_vbimin_s: f64 = *var_vbimin_s_slot;
        let mut var_vbisti2: f64 = *var_vbisti2_slot;
        let mut var_vbisti2r: f64 = *var_vbisti2r_slot;
        let mut var_vch_d: f64 = *var_vch_d_slot;
        let mut var_vch_s: f64 = *var_vch_s_slot;
        let mut var_vfmin_d: f64 = *var_vfmin_d_slot;
        let mut var_vfmin_s: f64 = *var_vfmin_s_slot;
        let mut var_vmax_d: f64 = *var_vmax_d_slot;
        let mut var_vmaxbot: f64 = *var_vmaxbot_slot;
        let mut var_vmaxgat: f64 = *var_vmaxgat_slot;
        let mut var_vmaxsti: f64 = *var_vmaxsti_slot;

        let (assign13370_e11459,) = {
    if ((var_guard175 != 0.0) && (var_guard182 != 0.0)) {
        let assign13370_e11457: f64 = (p.p845 + p.p847);
        (assign13370_e11457,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13370_e11459;

        let assign13380_e11462: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard183 = assign13380_e11462;

        let (assign13390_e11470,) = {
    if ((var_guard175 != 0.0) && (var_guard183 != 0.0)) {
        let assign13390_e11468: f64 = (var_vbibot + var_vbisti);
        (assign13390_e11468,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13390_e11470;

        let (assign13400_e11480,) = {
    if ((var_guard175 != 0.0) && (var_guard183 != 0.0)) {
        let assign13400_e11477: f64 = (p.p848).min(p.p849);
        let assign13400_e11478: f64 = (0.9 * assign13400_e11477);
        (assign13400_e11478,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13400_e11480;

        let (assign13410_e11488,) = {
    if ((var_guard175 != 0.0) && (var_guard183 != 0.0)) {
        let assign13410_e11486: f64 = (p.p845 + p.p846);
        (assign13410_e11486,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13410_e11488;

        let (assign13420_e11496,) = {
    if (var_guard175 != 0.0) {
        let assign13420_e11492: f64 = (var_vbibot2).min(var_vbisti2);
        let assign13420_e11494: f64 = (assign13420_e11492).min(var_vbigat2);
        (assign13420_e11494,)
    } else {
        (var_vbimin_s,)
    }
};
        var_vbimin_s = assign13420_e11496;

        let (assign13430_e11502,) = {
    if (var_guard175 != 0.0) {
        let assign13430_e11500: f64 = (var_vbimin_s * 0.1);
        (assign13430_e11500,)
    } else {
        (var_vch_s,)
    }
};
        var_vch_s = assign13430_e11502;

        let (assign13440_e11510,) = {
    if (var_guard175 != 0.0) {
        let assign13440_e11506: f64 = (var_pbot2).max(var_psti2);
        let assign13440_e11508: f64 = (assign13440_e11506).max(var_pgat2);
        (assign13440_e11508,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign13440_e11510;

        let (assign13450_e11523,) = {
    if (var_guard175 != 0.0) {
        let assign13450_e11516: f64 = (-1.0);
        let assign13450_e11518: f64 = (assign13450_e11516 / var_pmax);
        let assign13450_e11519: f64 = (2.0_f64).powf(assign13450_e11518);
        let assign13450_e11520: f64 = (1.0 - assign13450_e11519);
        let assign13450_e11521: f64 = (var_vbimin_s * assign13450_e11520);
        (assign13450_e11521,)
    } else {
        (var_vfmin_s,)
    }
};
        var_vfmin_s = assign13450_e11523;

        let (assign13460_e11533,) = {
    if (var_guard175 != 0.0) {
        let assign13460_e11527: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign13460_e11529: f64 = (assign13460_e11527).min(var_vbigat2r);
        let assign13460_e11531: f64 = (assign13460_e11529 - 0.05);
        (assign13460_e11531,)
    } else {
        (var_vbbtlim_s,)
    }
};
        var_vbbtlim_s = assign13460_e11533;

        let assign13470_e11536: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign13470_e11538: f64 = if assign13470_e11536 > 0.0 { 1.0 } else { 0.0 };
        var_guard184 = assign13470_e11538;

        let (assign13480_e11553,) = {
    if ((var_guard175 != 0.0) && (var_guard184 != 0.0)) {
        let assign13480_e11546: f64 = (var_idsatbot_d * var_abdrain_i);
        let assign13480_e11547: f64 = (p.p839 / assign13480_e11546);
        let assign13480_e11549: f64 = (assign13480_e11547 + 1.0);
        let assign13480_e11550: f64 = (assign13480_e11549).ln();
        let assign13480_e11551: f64 = (var_phitd * assign13480_e11550);
        (assign13480_e11551,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13480_e11553;

        let (assign13490_e11560,) = {
    if ((var_guard175 != 0.0) && (var_guard184 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxbot,)
    }
};
        var_vmaxbot = assign13490_e11560;

        let assign13500_e11563: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign13500_e11565: f64 = if assign13500_e11563 > 0.0 { 1.0 } else { 0.0 };
        var_guard185 = assign13500_e11565;

        let (assign13510_e11580,) = {
    if ((var_guard175 != 0.0) && (var_guard185 != 0.0)) {
        let assign13510_e11573: f64 = (var_idsatsti_d * var_lsdrain_i);
        let assign13510_e11574: f64 = (p.p839 / assign13510_e11573);
        let assign13510_e11576: f64 = (assign13510_e11574 + 1.0);
        let assign13510_e11577: f64 = (assign13510_e11576).ln();
        let assign13510_e11578: f64 = (var_phitd * assign13510_e11577);
        (assign13510_e11578,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13510_e11580;

        let (assign13520_e11587,) = {
    if ((var_guard175 != 0.0) && (var_guard185 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxsti,)
    }
};
        var_vmaxsti = assign13520_e11587;

        let assign13530_e11590: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign13530_e11592: f64 = if assign13530_e11590 > 0.0 { 1.0 } else { 0.0 };
        var_guard186 = assign13530_e11592;

        let (assign13540_e11607,) = {
    if ((var_guard175 != 0.0) && (var_guard186 != 0.0)) {
        let assign13540_e11600: f64 = (var_idsatgat_d * var_lgdrain_i);
        let assign13540_e11601: f64 = (p.p839 / assign13540_e11600);
        let assign13540_e11603: f64 = (assign13540_e11601 + 1.0);
        let assign13540_e11604: f64 = (assign13540_e11603).ln();
        let assign13540_e11605: f64 = (var_phitd * assign13540_e11604);
        (assign13540_e11605,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13540_e11607;

        let (assign13550_e11614,) = {
    if ((var_guard175 != 0.0) && (var_guard186 == 0.0)) {
        (100000000.0,)
    } else {
        (var_vmaxgat,)
    }
};
        var_vmaxgat = assign13550_e11614;

        let (assign13560_e11622,) = {
    if (var_guard175 != 0.0) {
        let assign13560_e11618: f64 = (var_vmaxbot).min(var_vmaxsti);
        let assign13560_e11620: f64 = (assign13560_e11618).min(var_vmaxgat);
        (assign13560_e11620,)
    } else {
        (var_vmax_d,)
    }
};
        var_vmax_d = assign13560_e11622;

        let assign13570_e11625: f64 = (var_vmax_d * var_phitdinv);
        let assign13570_e11626: f64 = (assign13570_e11625).abs();
        let assign13570_e11628: f64 = if assign13570_e11626 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard187 = assign13570_e11628;

        let (assign13580_e11637,) = {
    if ((var_guard175 != 0.0) && (var_guard187 != 0.0)) {
        let assign13580_e11634: f64 = (var_vmax_d * var_phitdinv);
        let assign13580_e11635: f64 = (assign13580_e11634).exp();
        (assign13580_e11635,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign13580_e11637;

        let assign13590_e11640: f64 = (var_vmax_d * var_phitdinv);
        let assign13590_e11642: f64 = if assign13590_e11640 < 0.0 { 1.0 } else { 0.0 };
        var_guard188 = assign13590_e11642;

        let (assign13600_e11682,) = {
    if (((var_guard175 != 0.0) && (var_guard187 == 0.0)) && (var_guard188 != 0.0)) {
        let assign13600_e11652: f64 = (-230.25850929940458);
        let assign13600_e11655: f64 = (var_vmax_d * var_phitdinv);
        let assign13600_e11656: f64 = (assign13600_e11652 - assign13600_e11655);
        let assign13600_e11660: f64 = (-230.25850929940458);
        let assign13600_e11663: f64 = (var_vmax_d * var_phitdinv);
        let assign13600_e11664: f64 = (assign13600_e11660 - assign13600_e11663);
        let assign13600_e11667: f64 = (-230.25850929940458);
        let assign13600_e11670: f64 = (var_vmax_d * var_phitdinv);
        let assign13600_e11671: f64 = (assign13600_e11667 - assign13600_e11670);
        let assign13600_e11673: f64 = (assign13600_e11671 * 0.3333333333333333);
        let assign13600_e11674: f64 = (1.0 + assign13600_e11673);
        let assign13600_e11675: f64 = (assign13600_e11664 * assign13600_e11674);
        let assign13600_e11676: f64 = (0.5 * assign13600_e11675);
        let assign13600_e11677: f64 = (1.0 + assign13600_e11676);
        let assign13600_e11678: f64 = (assign13600_e11656 * assign13600_e11677);
        let assign13600_e11679: f64 = (1.0 + assign13600_e11678);
        let assign13600_e11680: f64 = (1e-100 / assign13600_e11679);
        (assign13600_e11680,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign13600_e11682;

        let (assign13610_e11720,) = {
    if (((var_guard175 != 0.0) && (var_guard187 == 0.0)) && (var_guard188 == 0.0)) {
        let assign13610_e11694: f64 = (var_vmax_d * var_phitdinv);
        let assign13610_e11696: f64 = (assign13610_e11694 - 230.25850929940458);
        let assign13610_e11701: f64 = (var_vmax_d * var_phitdinv);
        let assign13610_e11703: f64 = (assign13610_e11701 - 230.25850929940458);
        let assign13610_e11707: f64 = (var_vmax_d * var_phitdinv);
        let assign13610_e11709: f64 = (assign13610_e11707 - 230.25850929940458);
        let assign13610_e11711: f64 = (assign13610_e11709 * 0.3333333333333333);
        let assign13610_e11712: f64 = (1.0 + assign13610_e11711);
        let assign13610_e11713: f64 = (assign13610_e11703 * assign13610_e11712);
        let assign13610_e11714: f64 = (0.5 * assign13610_e11713);
        let assign13610_e11715: f64 = (1.0 + assign13610_e11714);
        let assign13610_e11716: f64 = (assign13610_e11696 * assign13610_e11715);
        let assign13610_e11717: f64 = (1.0 + assign13610_e11716);
        let assign13610_e11718: f64 = (1e100 * assign13610_e11717);
        (assign13610_e11718,)
    } else {
        (var_exp_vmax_over_phitd_d,)
    }
};
        var_exp_vmax_over_phitd_d = assign13610_e11720;

        let (assign13620_e11724,) = {
    if (var_guard175 != 0.0) {
        (var_vbibot_d,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13620_e11724;

        let (assign13630_e11728,) = {
    if (var_guard175 != 0.0) {
        (var_vbisti_d,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13630_e11728;

        let (assign13640_e11732,) = {
    if (var_guard175 != 0.0) {
        (var_vbigat_d,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13640_e11732;

        let (assign13650_e11736,) = {
    if (var_guard175 != 0.0) {
        (var_pbotd_i,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13650_e11736;

        let (assign13660_e11740,) = {
    if (var_guard175 != 0.0) {
        (var_pstid_i,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13660_e11740;

        let (assign13670_e11744,) = {
    if (var_guard175 != 0.0) {
        (var_pgatd_i,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13670_e11744;

        let (assign13680_e11748,) = {
    if (var_guard175 != 0.0) {
        (var_vbirbotd_i,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13680_e11748;

        let (assign13690_e11752,) = {
    if (var_guard175 != 0.0) {
        (var_vbirstid_i,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13690_e11752;

        let (assign13700_e11756,) = {
    if (var_guard175 != 0.0) {
        (var_vbirgatd_i,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13700_e11756;

        let assign13710_e11759: f64 = if var_abdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard189 = assign13710_e11759;

        let (assign13720_e11767,) = {
    if ((var_guard175 != 0.0) && (var_guard189 != 0.0)) {
        let assign13720_e11765: f64 = (var_vbisti_d + var_vbigat_d);
        (assign13720_e11765,)
    } else {
        (var_vbibot2,)
    }
};
        var_vbibot2 = assign13720_e11767;

        let (assign13730_e11777,) = {
    if ((var_guard175 != 0.0) && (var_guard189 != 0.0)) {
        let assign13730_e11774: f64 = (var_pstid_i).min(var_pgatd_i);
        let assign13730_e11775: f64 = (0.9 * assign13730_e11774);
        (assign13730_e11775,)
    } else {
        (var_pbot2,)
    }
};
        var_pbot2 = assign13730_e11777;

        let (assign13740_e11785,) = {
    if ((var_guard175 != 0.0) && (var_guard189 != 0.0)) {
        let assign13740_e11783: f64 = (var_vbirstid_i + var_vbirgatd_i);
        (assign13740_e11783,)
    } else {
        (var_vbibot2r,)
    }
};
        var_vbibot2r = assign13740_e11785;

        let assign13750_e11788: f64 = if var_lsdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard190 = assign13750_e11788;

        let (assign13760_e11796,) = {
    if ((var_guard175 != 0.0) && (var_guard190 != 0.0)) {
        let assign13760_e11794: f64 = (var_vbibot_d + var_vbigat_d);
        (assign13760_e11794,)
    } else {
        (var_vbisti2,)
    }
};
        var_vbisti2 = assign13760_e11796;

        let (assign13770_e11806,) = {
    if ((var_guard175 != 0.0) && (var_guard190 != 0.0)) {
        let assign13770_e11803: f64 = (var_pbotd_i).min(var_pgatd_i);
        let assign13770_e11804: f64 = (0.9 * assign13770_e11803);
        (assign13770_e11804,)
    } else {
        (var_psti2,)
    }
};
        var_psti2 = assign13770_e11806;

        let (assign13780_e11814,) = {
    if ((var_guard175 != 0.0) && (var_guard190 != 0.0)) {
        let assign13780_e11812: f64 = (var_vbirbotd_i + var_vbirgatd_i);
        (assign13780_e11812,)
    } else {
        (var_vbisti2r,)
    }
};
        var_vbisti2r = assign13780_e11814;

        let assign13790_e11817: f64 = if var_lgdrain_i == 0.0 { 1.0 } else { 0.0 };
        var_guard191 = assign13790_e11817;

        let (assign13800_e11825,) = {
    if ((var_guard175 != 0.0) && (var_guard191 != 0.0)) {
        let assign13800_e11823: f64 = (var_vbibot_d + var_vbisti_d);
        (assign13800_e11823,)
    } else {
        (var_vbigat2,)
    }
};
        var_vbigat2 = assign13800_e11825;

        let (assign13810_e11835,) = {
    if ((var_guard175 != 0.0) && (var_guard191 != 0.0)) {
        let assign13810_e11832: f64 = (var_pbotd_i).min(var_pstid_i);
        let assign13810_e11833: f64 = (0.9 * assign13810_e11832);
        (assign13810_e11833,)
    } else {
        (var_pgat2,)
    }
};
        var_pgat2 = assign13810_e11835;

        let (assign13820_e11843,) = {
    if ((var_guard175 != 0.0) && (var_guard191 != 0.0)) {
        let assign13820_e11841: f64 = (var_vbirbotd_i + var_vbirstid_i);
        (assign13820_e11841,)
    } else {
        (var_vbigat2r,)
    }
};
        var_vbigat2r = assign13820_e11843;

        let (assign13830_e11851,) = {
    if (var_guard175 != 0.0) {
        let assign13830_e11847: f64 = (var_vbibot2).min(var_vbisti2);
        let assign13830_e11849: f64 = (assign13830_e11847).min(var_vbigat2);
        (assign13830_e11849,)
    } else {
        (var_vbimin_d,)
    }
};
        var_vbimin_d = assign13830_e11851;

        let (assign13840_e11857,) = {
    if (var_guard175 != 0.0) {
        let assign13840_e11855: f64 = (var_vbimin_d * 0.1);
        (assign13840_e11855,)
    } else {
        (var_vch_d,)
    }
};
        var_vch_d = assign13840_e11857;

        let (assign13850_e11865,) = {
    if (var_guard175 != 0.0) {
        let assign13850_e11861: f64 = (var_pbot2).max(var_psti2);
        let assign13850_e11863: f64 = (assign13850_e11861).max(var_pgat2);
        (assign13850_e11863,)
    } else {
        (var_pmax,)
    }
};
        var_pmax = assign13850_e11865;

        let (assign13860_e11878,) = {
    if (var_guard175 != 0.0) {
        let assign13860_e11871: f64 = (-1.0);
        let assign13860_e11873: f64 = (assign13860_e11871 / var_pmax);
        let assign13860_e11874: f64 = (2.0_f64).powf(assign13860_e11873);
        let assign13860_e11875: f64 = (1.0 - assign13860_e11874);
        let assign13860_e11876: f64 = (var_vbimin_d * assign13860_e11875);
        (assign13860_e11876,)
    } else {
        (var_vfmin_d,)
    }
};
        var_vfmin_d = assign13860_e11878;

        let (assign13870_e11888,) = {
    if (var_guard175 != 0.0) {
        let assign13870_e11882: f64 = (var_vbibot2r).min(var_vbisti2r);
        let assign13870_e11884: f64 = (assign13870_e11882).min(var_vbigat2r);
        let assign13870_e11886: f64 = (assign13870_e11884 - 0.05);
        (assign13870_e11886,)
    } else {
        (var_vbbtlim_d,)
    }
};
        var_vbbtlim_d = assign13870_e11888;

        let assign13880_e11891: f64 = if var_swjunexp_i == 1.0 { 1.0 } else { 0.0 };
        var_guard192 = assign13880_e11891;

        *var_exp_vmax_over_phitd_d_slot = var_exp_vmax_over_phitd_d;
        *var_guard183_slot = var_guard183;
        *var_guard184_slot = var_guard184;
        *var_guard185_slot = var_guard185;
        *var_guard186_slot = var_guard186;
        *var_guard187_slot = var_guard187;
        *var_guard188_slot = var_guard188;
        *var_guard189_slot = var_guard189;
        *var_guard190_slot = var_guard190;
        *var_guard191_slot = var_guard191;
        *var_guard192_slot = var_guard192;
        *var_pbot2_slot = var_pbot2;
        *var_pgat2_slot = var_pgat2;
        *var_pmax_slot = var_pmax;
        *var_psti2_slot = var_psti2;
        *var_vbbtlim_d_slot = var_vbbtlim_d;
        *var_vbbtlim_s_slot = var_vbbtlim_s;
        *var_vbibot2_slot = var_vbibot2;
        *var_vbibot2r_slot = var_vbibot2r;
        *var_vbigat2_slot = var_vbigat2;
        *var_vbigat2r_slot = var_vbigat2r;
        *var_vbimin_d_slot = var_vbimin_d;
        *var_vbimin_s_slot = var_vbimin_s;
        *var_vbisti2_slot = var_vbisti2;
        *var_vbisti2r_slot = var_vbisti2r;
        *var_vch_d_slot = var_vch_d;
        *var_vch_s_slot = var_vch_s;
        *var_vfmin_d_slot = var_vfmin_d;
        *var_vfmin_s_slot = var_vfmin_s;
        *var_vmax_d_slot = var_vmax_d;
        *var_vmaxbot_slot = var_vmaxbot;
        *var_vmaxgat_slot = var_vmaxgat;
        *var_vmaxsti_slot = var_vmaxsti;
    }

    pub(super) fn stamp_transient_block_20(
        p: &Parameters,
        var_guard175: f64,
        var_guard192: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_fraci_slot: &mut f64,
        var_fracna_slot: &mut f64,
        var_fracnb_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_v1_slot: &mut f64,
        var_v2_slot: &mut f64,
        var_v3_slot: &mut f64,
        var_v4_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_fraci: f64 = *var_fraci_slot;
        let mut var_fracna: f64 = *var_fracna_slot;
        let mut var_fracnb: f64 = *var_fracnb_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_v1: f64 = *var_v1_slot;
        let mut var_v2: f64 = *var_v2_slot;
        let mut var_v3: f64 = *var_v3_slot;
        let mut var_v4: f64 = *var_v4_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign13890_e11897, assign13890_e11897_d_n6, assign13890_e11897_d_n7, assign13890_e11897_d_n8, assign13890_e11897_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign13890_e11897;
        var_ysq_dn6 = assign13890_e11897_d_n6;
        var_ysq_dn7 = assign13890_e11897_d_n7;
        var_ysq_dn8 = assign13890_e11897_d_n8;
        var_ysq_dn9 = assign13890_e11897_d_n9;

        let (assign13900_e11903, assign13900_e11903_d_n6, assign13900_e11903_d_n7, assign13900_e11903_d_n8, assign13900_e11903_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign13900_e11903;
        var_terfc_dn6 = assign13900_e11903_d_n6;
        var_terfc_dn7 = assign13900_e11903_d_n7;
        var_terfc_dn8 = assign13900_e11903_d_n8;
        var_terfc_dn9 = assign13900_e11903_d_n9;

        let (assign13910_e11909, assign13910_e11909_d_n6, assign13910_e11909_d_n7, assign13910_e11909_d_n8, assign13910_e11909_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign13910_e11909;
        var_erfcpos_dn6 = assign13910_e11909_d_n6;
        var_erfcpos_dn7 = assign13910_e11909_d_n7;
        var_erfcpos_dn8 = assign13910_e11909_d_n8;
        var_erfcpos_dn9 = assign13910_e11909_d_n9;

        let (assign13980_e11951,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign13980_e11951;

        let (assign14000_e11963,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_z,)
    }
};
        var_z = assign14000_e11963;

        let (assign14010_e11969,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign14010_e11969;

        let (assign14020_e11975,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign14020_e11975;

        let (assign14030_e11981,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign14030_e11981;

        let (assign14040_e11987,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign14040_e11987;

        let (assign14050_e11993,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign14050_e11993;

        let (assign14060_e11999,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign14060_e11999;

        let (assign14070_e12005, assign14070_e12005_d_n6, assign14070_e12005_d_n7, assign14070_e12005_d_n8, assign14070_e12005_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign14070_e12005;
        var_tmp_dn6 = assign14070_e12005_d_n6;
        var_tmp_dn7 = assign14070_e12005_d_n7;
        var_tmp_dn8 = assign14070_e12005_d_n8;
        var_tmp_dn9 = assign14070_e12005_d_n9;

        let (assign14080_e12011,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign14080_e12011;

        let (assign14090_e12017, assign14090_e12017_d_n6, assign14090_e12017_d_n7, assign14090_e12017_d_n8, assign14090_e12017_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign14090_e12017;
        var_isrh_dn6 = assign14090_e12017_d_n6;
        var_isrh_dn7 = assign14090_e12017_d_n7;
        var_isrh_dn8 = assign14090_e12017_d_n8;
        var_isrh_dn9 = assign14090_e12017_d_n9;

        let (assign14100_e12023,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign14100_e12023;

        let (assign14110_e12029,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign14110_e12029;

        let (assign14120_e12035,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign14120_e12035;

        let (assign14130_e12041,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign14130_e12041;

        let (assign14140_e12047, assign14140_e12047_d_n6, assign14140_e12047_d_n7, assign14140_e12047_d_n8, assign14140_e12047_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign14140_e12047;
        var_wdep_dn6 = assign14140_e12047_d_n6;
        var_wdep_dn7 = assign14140_e12047_d_n7;
        var_wdep_dn8 = assign14140_e12047_d_n8;
        var_wdep_dn9 = assign14140_e12047_d_n9;

        let (assign14150_e12053, assign14150_e12053_d_n6, assign14150_e12053_d_n7, assign14150_e12053_d_n8, assign14150_e12053_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign14150_e12053;
        var_asrh_dn6 = assign14150_e12053_d_n6;
        var_asrh_dn7 = assign14150_e12053_d_n7;
        var_asrh_dn8 = assign14150_e12053_d_n8;
        var_asrh_dn9 = assign14150_e12053_d_n9;

        let (assign14160_e12059, assign14160_e12059_d_n6, assign14160_e12059_d_n7, assign14160_e12059_d_n8, assign14160_e12059_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign14160_e12059;
        var_itat_dn6 = assign14160_e12059_d_n6;
        var_itat_dn7 = assign14160_e12059_d_n7;
        var_itat_dn8 = assign14160_e12059_d_n8;
        var_itat_dn9 = assign14160_e12059_d_n9;

        let (assign14170_e12065, assign14170_e12065_d_n6, assign14170_e12065_d_n7, assign14170_e12065_d_n8, assign14170_e12065_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign14170_e12065;
        var_btat_dn6 = assign14170_e12065_d_n6;
        var_btat_dn7 = assign14170_e12065_d_n7;
        var_btat_dn8 = assign14170_e12065_d_n8;
        var_btat_dn9 = assign14170_e12065_d_n9;

        let (assign14180_e12071, assign14180_e12071_d_n6, assign14180_e12071_d_n7, assign14180_e12071_d_n8, assign14180_e12071_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign14180_e12071;
        var_twoatatoverthreebtat_dn6 = assign14180_e12071_d_n6;
        var_twoatatoverthreebtat_dn7 = assign14180_e12071_d_n7;
        var_twoatatoverthreebtat_dn8 = assign14180_e12071_d_n8;
        var_twoatatoverthreebtat_dn9 = assign14180_e12071_d_n9;

        let (assign14190_e12077, assign14190_e12077_d_n6, assign14190_e12077_d_n7, assign14190_e12077_d_n8, assign14190_e12077_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign14190_e12077;
        var_umaxbeforelimiting_dn6 = assign14190_e12077_d_n6;
        var_umaxbeforelimiting_dn7 = assign14190_e12077_d_n7;
        var_umaxbeforelimiting_dn8 = assign14190_e12077_d_n8;
        var_umaxbeforelimiting_dn9 = assign14190_e12077_d_n9;

        let (assign14200_e12083, assign14200_e12083_d_n6, assign14200_e12083_d_n7, assign14200_e12083_d_n8, assign14200_e12083_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign14200_e12083;
        var_umax_dn6 = assign14200_e12083_d_n6;
        var_umax_dn7 = assign14200_e12083_d_n7;
        var_umax_dn8 = assign14200_e12083_d_n8;
        var_umax_dn9 = assign14200_e12083_d_n9;

        let (assign14210_e12089, assign14210_e12089_d_n6, assign14210_e12089_d_n7, assign14210_e12089_d_n8, assign14210_e12089_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign14210_e12089;
        var_sqrtumax_dn6 = assign14210_e12089_d_n6;
        var_sqrtumax_dn7 = assign14210_e12089_d_n7;
        var_sqrtumax_dn8 = assign14210_e12089_d_n8;
        var_sqrtumax_dn9 = assign14210_e12089_d_n9;

        let (assign14220_e12095, assign14220_e12095_d_n6, assign14220_e12095_d_n7, assign14220_e12095_d_n8, assign14220_e12095_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign14220_e12095;
        var_umaxpoweronepointfive_dn6 = assign14220_e12095_d_n6;
        var_umaxpoweronepointfive_dn7 = assign14220_e12095_d_n7;
        var_umaxpoweronepointfive_dn8 = assign14220_e12095_d_n8;
        var_umaxpoweronepointfive_dn9 = assign14220_e12095_d_n9;

        let (assign14230_e12101, assign14230_e12101_d_n6, assign14230_e12101_d_n7, assign14230_e12101_d_n8, assign14230_e12101_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign14230_e12101;
        var_wgamma_dn6 = assign14230_e12101_d_n6;
        var_wgamma_dn7 = assign14230_e12101_d_n7;
        var_wgamma_dn8 = assign14230_e12101_d_n8;
        var_wgamma_dn9 = assign14230_e12101_d_n9;

        let (assign14240_e12107, assign14240_e12107_d_n6, assign14240_e12107_d_n7, assign14240_e12107_d_n8, assign14240_e12107_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign14240_e12107;
        var_wtat_dn6 = assign14240_e12107_d_n6;
        var_wtat_dn7 = assign14240_e12107_d_n7;
        var_wtat_dn8 = assign14240_e12107_d_n8;
        var_wtat_dn9 = assign14240_e12107_d_n9;

        let (assign14250_e12113, assign14250_e12113_d_n6, assign14250_e12113_d_n7, assign14250_e12113_d_n8, assign14250_e12113_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign14250_e12113;
        var_ktat_dn6 = assign14250_e12113_d_n6;
        var_ktat_dn7 = assign14250_e12113_d_n7;
        var_ktat_dn8 = assign14250_e12113_d_n8;
        var_ktat_dn9 = assign14250_e12113_d_n9;

        let (assign14260_e12119, assign14260_e12119_d_n6, assign14260_e12119_d_n7, assign14260_e12119_d_n8, assign14260_e12119_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign14260_e12119;
        var_ltat_dn6 = assign14260_e12119_d_n6;
        var_ltat_dn7 = assign14260_e12119_d_n7;
        var_ltat_dn8 = assign14260_e12119_d_n8;
        var_ltat_dn9 = assign14260_e12119_d_n9;

        let (assign14270_e12125, assign14270_e12125_d_n6, assign14270_e12125_d_n7, assign14270_e12125_d_n8, assign14270_e12125_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign14270_e12125;
        var_mtat_dn6 = assign14270_e12125_d_n6;
        var_mtat_dn7 = assign14270_e12125_d_n7;
        var_mtat_dn8 = assign14270_e12125_d_n8;
        var_mtat_dn9 = assign14270_e12125_d_n9;

        let (assign14280_e12131, assign14280_e12131_d_n6, assign14280_e12131_d_n7, assign14280_e12131_d_n8, assign14280_e12131_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign14280_e12131;
        var_xerfc_dn6 = assign14280_e12131_d_n6;
        var_xerfc_dn7 = assign14280_e12131_d_n7;
        var_xerfc_dn8 = assign14280_e12131_d_n8;
        var_xerfc_dn9 = assign14280_e12131_d_n9;

        let (assign14290_e12137, assign14290_e12137_d_n6, assign14290_e12137_d_n7, assign14290_e12137_d_n8, assign14290_e12137_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign14290_e12137;
        var_erfctimesexpmtat_dn6 = assign14290_e12137_d_n6;
        var_erfctimesexpmtat_dn7 = assign14290_e12137_d_n7;
        var_erfctimesexpmtat_dn8 = assign14290_e12137_d_n8;
        var_erfctimesexpmtat_dn9 = assign14290_e12137_d_n9;

        let (assign14300_e12143, assign14300_e12143_d_n6, assign14300_e12143_d_n7, assign14300_e12143_d_n8, assign14300_e12143_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign14300_e12143;
        var_gammamax_dn6 = assign14300_e12143_d_n6;
        var_gammamax_dn7 = assign14300_e12143_d_n7;
        var_gammamax_dn8 = assign14300_e12143_d_n8;
        var_gammamax_dn9 = assign14300_e12143_d_n9;

        let (assign14310_e12149, assign14310_e12149_d_n6, assign14310_e12149_d_n7, assign14310_e12149_d_n8, assign14310_e12149_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign14310_e12149;
        var_ibbt_dn6 = assign14310_e12149_d_n6;
        var_ibbt_dn7 = assign14310_e12149_d_n7;
        var_ibbt_dn8 = assign14310_e12149_d_n8;
        var_ibbt_dn9 = assign14310_e12149_d_n9;

        let (assign14320_e12155, assign14320_e12155_d_n6, assign14320_e12155_d_n7, assign14320_e12155_d_n8, assign14320_e12155_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign14320_e12155;
        var_fmaxr_dn6 = assign14320_e12155_d_n6;
        var_fmaxr_dn7 = assign14320_e12155_d_n7;
        var_fmaxr_dn8 = assign14320_e12155_d_n8;
        var_fmaxr_dn9 = assign14320_e12155_d_n9;

        let (assign14330_e12161, assign14330_e12161_d_n6, assign14330_e12161_d_n7, assign14330_e12161_d_n8, assign14330_e12161_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign14330_e12161;
        var_fbreakdown_dn6 = assign14330_e12161_d_n6;
        var_fbreakdown_dn7 = assign14330_e12161_d_n7;
        var_fbreakdown_dn8 = assign14330_e12161_d_n8;
        var_fbreakdown_dn9 = assign14330_e12161_d_n9;

        let (assign14340_e12167,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.4,)
    } else {
        (var_fracna,)
    }
};
        var_fracna = assign14340_e12167;

        let (assign14350_e12173,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.65,)
    } else {
        (var_fracnb,)
    }
};
        var_fracnb = assign14350_e12173;

        let (assign14360_e12179,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.8,)
    } else {
        (var_fraci,)
    }
};
        var_fraci = assign14360_e12179;

        let (assign14370_e12188,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign14370_e12184: f64 = (-var_fracna);
        let assign14370_e12186: f64 = (assign14370_e12184 * p.p945);
        (assign14370_e12186,)
    } else {
        (var_v1,)
    }
};
        var_v1 = assign14370_e12188;

        let (assign14380_e12197,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign14380_e12193: f64 = (-var_fracnb);
        let assign14380_e12195: f64 = (assign14380_e12193 * p.p945);
        (assign14380_e12195,)
    } else {
        (var_v2,)
    }
};
        var_v2 = assign14380_e12197;

        let (assign14390_e12206,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign14390_e12202: f64 = (-var_fraci);
        let assign14390_e12204: f64 = (assign14390_e12202 * p.p945);
        (assign14390_e12204,)
    } else {
        (var_v3,)
    }
};
        var_v3 = assign14390_e12206;

        let (assign14400_e12212,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.1,)
    } else {
        (var_v4,)
    }
};
        var_v4 = assign14400_e12212;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_fraci_slot = var_fraci;
        *var_fracna_slot = var_fracna;
        *var_fracnb_slot = var_fracnb;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_idmult_slot = var_idmult;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_v1_slot = var_v1;
        *var_v2_slot = var_v2;
        *var_v3_slot = var_v3;
        *var_v4_slot = var_v4;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_21(
        p: &Parameters,
        var_absource_i: f64,
        var_btatpartbot: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_ftdbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_idsatbot: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_one_minus_pbot: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_v1: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbimin_s: f64,
        var_vbirbotinv: f64,
        var_vmax_s: f64,
        var_wdepnulrbot: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_guard241_slot: &mut f64,
        var_guard242_slot: &mut f64,
        var_guard243_slot: &mut f64,
        var_guard244_slot: &mut f64,
        var_guard245_slot: &mut f64,
        var_guard246_slot: &mut f64,
        var_guard247_slot: &mut f64,
        var_guard248_slot: &mut f64,
        var_guard249_slot: &mut f64,
        var_guard250_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_v5_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_guard241: f64 = *var_guard241_slot;
        let mut var_guard242: f64 = *var_guard242_slot;
        let mut var_guard243: f64 = *var_guard243_slot;
        let mut var_guard244: f64 = *var_guard244_slot;
        let mut var_guard245: f64 = *var_guard245_slot;
        let mut var_guard246: f64 = *var_guard246_slot;
        let mut var_guard247: f64 = *var_guard247_slot;
        let mut var_guard248: f64 = *var_guard248_slot;
        let mut var_guard249: f64 = *var_guard249_slot;
        let mut var_guard250: f64 = *var_guard250_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_v5: f64 = *var_v5_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign14410_e12218,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.2,)
    } else {
        (var_v5,)
    }
};
        var_v5 = assign14410_e12218;

        let (assign14420_e12224,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign14420_e12224;

        let (assign14430_e12230,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign14430_e12230;

        let assign14440_e12242: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard241 = assign14440_e12242;

        let assign14520_e12328: f64 = if var_v1 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard242 = assign14520_e12328;

        let assign14530_e12330: f64 = (-0.5);
        let assign14530_e12333: f64 = (var_v1 * var_phitdinv);
        let assign14530_e12334: f64 = (assign14530_e12330 * assign14530_e12333);
        let assign14530_e12335: f64 = (assign14530_e12334).abs();
        let assign14530_e12337: f64 = if assign14530_e12335 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard243 = assign14530_e12337;

        let (assign14540_e12355,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 != 0.0)) && (var_guard243 != 0.0)) {
        let assign14540_e12348: f64 = (-0.5);
        let assign14540_e12351: f64 = (var_v1 * var_phitdinv);
        let assign14540_e12352: f64 = (assign14540_e12348 * assign14540_e12351);
        let assign14540_e12353: f64 = (assign14540_e12352).exp();
        (assign14540_e12353,)
    } else {
        (var_z,)
    }
};
        var_z = assign14540_e12355;

        let assign14550_e12357: f64 = (-0.5);
        let assign14550_e12360: f64 = (var_v1 * var_phitdinv);
        let assign14550_e12361: f64 = (assign14550_e12357 * assign14550_e12360);
        let assign14550_e12363: f64 = if assign14550_e12361 < 0.0 { 1.0 } else { 0.0 };
        var_guard244 = assign14550_e12363;

        let (assign14560_e12418,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 != 0.0)) && (var_guard243 == 0.0)) && (var_guard244 != 0.0)) {
        let assign14560_e12379: f64 = (-230.25850929940458);
        let assign14560_e12381: f64 = (-0.5);
        let assign14560_e12384: f64 = (var_v1 * var_phitdinv);
        let assign14560_e12385: f64 = (assign14560_e12381 * assign14560_e12384);
        let assign14560_e12386: f64 = (assign14560_e12379 - assign14560_e12385);
        let assign14560_e12390: f64 = (-230.25850929940458);
        let assign14560_e12392: f64 = (-0.5);
        let assign14560_e12395: f64 = (var_v1 * var_phitdinv);
        let assign14560_e12396: f64 = (assign14560_e12392 * assign14560_e12395);
        let assign14560_e12397: f64 = (assign14560_e12390 - assign14560_e12396);
        let assign14560_e12400: f64 = (-230.25850929940458);
        let assign14560_e12402: f64 = (-0.5);
        let assign14560_e12405: f64 = (var_v1 * var_phitdinv);
        let assign14560_e12406: f64 = (assign14560_e12402 * assign14560_e12405);
        let assign14560_e12407: f64 = (assign14560_e12400 - assign14560_e12406);
        let assign14560_e12409: f64 = (assign14560_e12407 * 0.3333333333333333);
        let assign14560_e12410: f64 = (1.0 + assign14560_e12409);
        let assign14560_e12411: f64 = (assign14560_e12397 * assign14560_e12410);
        let assign14560_e12412: f64 = (0.5 * assign14560_e12411);
        let assign14560_e12413: f64 = (1.0 + assign14560_e12412);
        let assign14560_e12414: f64 = (assign14560_e12386 * assign14560_e12413);
        let assign14560_e12415: f64 = (1.0 + assign14560_e12414);
        let assign14560_e12416: f64 = (1e-100 / assign14560_e12415);
        (assign14560_e12416,)
    } else {
        (var_z,)
    }
};
        var_z = assign14560_e12418;

        let (assign14570_e12471,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 != 0.0)) && (var_guard243 == 0.0)) && (var_guard244 == 0.0)) {
        let assign14570_e12435: f64 = (-0.5);
        let assign14570_e12438: f64 = (var_v1 * var_phitdinv);
        let assign14570_e12439: f64 = (assign14570_e12435 * assign14570_e12438);
        let assign14570_e12441: f64 = (assign14570_e12439 - 230.25850929940458);
        let assign14570_e12445: f64 = (-0.5);
        let assign14570_e12448: f64 = (var_v1 * var_phitdinv);
        let assign14570_e12449: f64 = (assign14570_e12445 * assign14570_e12448);
        let assign14570_e12451: f64 = (assign14570_e12449 - 230.25850929940458);
        let assign14570_e12454: f64 = (-0.5);
        let assign14570_e12457: f64 = (var_v1 * var_phitdinv);
        let assign14570_e12458: f64 = (assign14570_e12454 * assign14570_e12457);
        let assign14570_e12460: f64 = (assign14570_e12458 - 230.25850929940458);
        let assign14570_e12462: f64 = (assign14570_e12460 * 0.3333333333333333);
        let assign14570_e12463: f64 = (1.0 + assign14570_e12462);
        let assign14570_e12464: f64 = (assign14570_e12451 * assign14570_e12463);
        let assign14570_e12465: f64 = (0.5 * assign14570_e12464);
        let assign14570_e12466: f64 = (1.0 + assign14570_e12465);
        let assign14570_e12467: f64 = (assign14570_e12441 * assign14570_e12466);
        let assign14570_e12468: f64 = (1.0 + assign14570_e12467);
        let assign14570_e12469: f64 = (1e100 * assign14570_e12468);
        (assign14570_e12469,)
    } else {
        (var_z,)
    }
};
        var_z = assign14570_e12471;

        let (assign14580_e12483,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 != 0.0)) {
        let assign14580_e12481: f64 = (1.0 / var_z);
        (assign14580_e12481,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign14580_e12483;

        let (assign14590_e12495,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 != 0.0)) {
        let assign14590_e12493: f64 = (var_zinv * var_zinv);
        (assign14590_e12493,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign14590_e12495;

        let (assign14600_e12514,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 == 0.0)) {
        let assign14600_e12507: f64 = (var_v1 - var_vmax_s);
        let assign14600_e12509: f64 = (assign14600_e12507 * var_phitdinv);
        let assign14600_e12510: f64 = (1.0 + assign14600_e12509);
        let assign14600_e12512: f64 = (assign14600_e12510 * var_exp_vmax_over_phitd_s);
        (assign14600_e12512,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign14600_e12514;

        let (assign14610_e12526,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 == 0.0)) {
        let assign14610_e12524: f64 = (var_idmult).sqrt();
        (assign14610_e12524,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign14610_e12526;

        let (assign14620_e12539,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard242 == 0.0)) {
        let assign14620_e12537: f64 = (1.0 / var_zinv);
        (assign14620_e12537,)
    } else {
        (var_z,)
    }
};
        var_z = assign14620_e12539;

        let (assign14630_e12549,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) {
        let assign14630_e12547: f64 = (var_idmult - 1.0);
        (assign14630_e12547,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign14630_e12549;

        let assign14640_e12552: f64 = if var_v1 > 0.0 { 1.0 } else { 0.0 };
        var_guard245 = assign14640_e12552;

        let (assign14650_e12578,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard245 != 0.0)) {
        let assign14650_e12564: f64 = (2.0 + var_z);
        let assign14650_e12567: f64 = (var_z + 1.0);
        let assign14650_e12570: f64 = (var_z + 3.0);
        let assign14650_e12571: f64 = (assign14650_e12567 * assign14650_e12570);
        let assign14650_e12572: f64 = (assign14650_e12571).sqrt();
        let assign14650_e12573: f64 = (assign14650_e12564 + assign14650_e12572);
        let assign14650_e12574: f64 = (assign14650_e12573).ln();
        let assign14650_e12575: f64 = (var_phitd * assign14650_e12574);
        let assign14650_e12576: f64 = (2.0 * assign14650_e12575);
        (assign14650_e12576,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign14650_e12578;

        let (assign14660_e12612,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) && (var_guard245 == 0.0)) {
        let assign14660_e12588: f64 = (-var_v1);
        let assign14660_e12593: f64 = (2.0 * var_zinv);
        let assign14660_e12595: f64 = (assign14660_e12593 + 1.0);
        let assign14660_e12598: f64 = (1.0 + var_zinv);
        let assign14660_e12602: f64 = (3.0 * var_zinv);
        let assign14660_e12603: f64 = (1.0 + assign14660_e12602);
        let assign14660_e12604: f64 = (assign14660_e12598 * assign14660_e12603);
        let assign14660_e12605: f64 = (assign14660_e12604).sqrt();
        let assign14660_e12606: f64 = (assign14660_e12595 + assign14660_e12605);
        let assign14660_e12607: f64 = (assign14660_e12606).ln();
        let assign14660_e12608: f64 = (var_phitd * assign14660_e12607);
        let assign14660_e12609: f64 = (2.0 * assign14660_e12608);
        let assign14660_e12610: f64 = (assign14660_e12588 + assign14660_e12609);
        (assign14660_e12610,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign14660_e12612;

        let (assign14670_e12622,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) {
        let assign14670_e12620: f64 = (var_vbimin_s - var_two_psistar);
        (assign14670_e12620,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign14670_e12622;

        let (assign14680_e12649,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) {
        let assign14680_e12631: f64 = (var_v1 + var_vjlim);
        let assign14680_e12634: f64 = (var_v1 - var_vjlim);
        let assign14680_e12637: f64 = (var_v1 - var_vjlim);
        let assign14680_e12638: f64 = (assign14680_e12634 * assign14680_e12637);
        let assign14680_e12641: f64 = (4.0 * var_phitd);
        let assign14680_e12643: f64 = (assign14680_e12641 * var_phitd);
        let assign14680_e12644: f64 = (assign14680_e12638 + assign14680_e12643);
        let assign14680_e12645: f64 = (assign14680_e12644).sqrt();
        let assign14680_e12646: f64 = (assign14680_e12631 - assign14680_e12645);
        let assign14680_e12647: f64 = (0.5 * assign14680_e12646);
        (assign14680_e12647,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign14680_e12649;

        let (assign14690_e12676,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) {
        let assign14690_e12658: f64 = (var_v1 + var_vbbtlim_s);
        let assign14690_e12661: f64 = (var_v1 - var_vbbtlim_s);
        let assign14690_e12664: f64 = (var_v1 - var_vbbtlim_s);
        let assign14690_e12665: f64 = (assign14690_e12661 * assign14690_e12664);
        let assign14690_e12668: f64 = (4.0 * var_phitr);
        let assign14690_e12670: f64 = (assign14690_e12668 * var_phitr);
        let assign14690_e12671: f64 = (assign14690_e12665 + assign14690_e12670);
        let assign14690_e12672: f64 = (assign14690_e12671).sqrt();
        let assign14690_e12673: f64 = (assign14690_e12658 - assign14690_e12672);
        let assign14690_e12674: f64 = (0.5 * assign14690_e12673);
        (assign14690_e12674,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign14690_e12676;

        let (assign14700_e12703,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard241 != 0.0)) {
        let assign14700_e12685: f64 = var_v1;
        let assign14700_e12688: f64 = var_v1;
        let assign14700_e12691: f64 = var_v1;
        let assign14700_e12692: f64 = (assign14700_e12688 * assign14700_e12691);
        let assign14700_e12695: f64 = (4.0 * 1e-6);
        let assign14700_e12697: f64 = (assign14700_e12695 * 1e-6);
        let assign14700_e12698: f64 = (assign14700_e12692 + assign14700_e12697);
        let assign14700_e12699: f64 = (assign14700_e12698).sqrt();
        let assign14700_e12700: f64 = (assign14700_e12685 - assign14700_e12699);
        let assign14700_e12701: f64 = (0.5 * assign14700_e12700);
        (assign14700_e12701,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign14700_e12703;

        let assign14710_e12706: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard246 = assign14710_e12706;

        let (assign14720_e12714, assign14720_e12714_d_n6, assign14720_e12714_d_n7, assign14720_e12714_d_n8, assign14720_e12714_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign14720_e12714;
        var_ijunbot_dn6 = assign14720_e12714_d_n6;
        var_ijunbot_dn7 = assign14720_e12714_d_n7;
        var_ijunbot_dn8 = assign14720_e12714_d_n8;
        var_ijunbot_dn9 = assign14720_e12714_d_n9;

        let (assign14730_e12725,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) {
        let assign14730_e12723: f64 = (var_idsatbot * var_idmult);
        (assign14730_e12723,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign14730_e12725;

        let assign14740_e12732: f64 = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };
        var_guard247 = assign14740_e12732;

        let (assign14750_e12743, assign14750_e12743_d_n6, assign14750_e12743_d_n7, assign14750_e12743_d_n8, assign14750_e12743_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign14750_e12743;
        var_isrh_dn6 = assign14750_e12743_d_n6;
        var_isrh_dn7 = assign14750_e12743_d_n7;
        var_isrh_dn8 = assign14750_e12743_d_n8;
        var_isrh_dn9 = assign14750_e12743_d_n9;

        let (assign14760_e12757,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) {
        let assign14760_e12755: f64 = (var_vbibot - var_vjsrh);
        (assign14760_e12755,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign14760_e12757;

        let (assign14770_e12776,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) {
        let assign14770_e12771: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign14770_e12772: f64 = (1.0 - assign14770_e12771);
        let assign14770_e12773: f64 = (assign14770_e12772).sqrt();
        let assign14770_e12774: f64 = (1.0 - assign14770_e12773);
        (assign14770_e12774,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign14770_e12776;

        let assign14780_e12779: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard248 = assign14780_e12779;

        let (assign14790_e12793,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) && (var_guard248 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign14790_e12793;

        let (assign14800_e12825,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) && (var_guard248 == 0.0)) {
        let assign14800_e12808: f64 = (var_wsrhstep * var_wsrhstep);
        let assign14800_e12810: f64 = (var_wsrhstep).ln();
        let assign14800_e12811: f64 = (assign14800_e12808 * assign14800_e12810);
        let assign14800_e12814: f64 = (1.0 - var_wsrhstep);
        let assign14800_e12815: f64 = (assign14800_e12811 / assign14800_e12814);
        let assign14800_e12817: f64 = (assign14800_e12815 + var_wsrhstep);
        let assign14800_e12821: f64 = (2.0 * p.p848);
        let assign14800_e12822: f64 = (1.0 - assign14800_e12821);
        let assign14800_e12823: f64 = (assign14800_e12817 * assign14800_e12822);
        (assign14800_e12823,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign14800_e12825;

        let (assign14810_e12839,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) {
        let assign14810_e12837: f64 = (var_wsrhstep + var_dwsrh);
        (assign14810_e12837,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign14810_e12839;

        let assign14820_e12842: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard249 = assign14820_e12842;

        let (assign14830_e12859, assign14830_e12859_d_n6, assign14830_e12859_d_n7, assign14830_e12859_d_n8, assign14830_e12859_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) && (var_guard249 != 0.0)) {
        let assign14830_e12856: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign14830_e12857: f64 = (assign14830_e12856).sqrt();
        (assign14830_e12857, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign14830_e12859;
        var_tmp_dn6 = assign14830_e12859_d_n6;
        var_tmp_dn7 = assign14830_e12859_d_n7;
        var_tmp_dn8 = assign14830_e12859_d_n8;
        var_tmp_dn9 = assign14830_e12859_d_n9;

        let (assign14840_e12878, assign14840_e12878_d_n6, assign14840_e12878_d_n7, assign14840_e12878_d_n8, assign14840_e12878_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) && (var_guard249 == 0.0)) {
        let assign14840_e12874: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign14840_e12876: f64 = (assign14840_e12874).powf(p.p848);
        (assign14840_e12876, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign14840_e12878;
        var_tmp_dn6 = assign14840_e12878_d_n6;
        var_tmp_dn7 = assign14840_e12878_d_n7;
        var_tmp_dn8 = assign14840_e12878_d_n8;
        var_tmp_dn9 = assign14840_e12878_d_n9;

        let (assign14850_e12892, assign14850_e12892_d_n6, assign14850_e12892_d_n7, assign14850_e12892_d_n8, assign14850_e12892_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) {
        let assign14850_e12890: f64 = (var_wdepnulrbot * var_tmp);
        (assign14850_e12890, (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8), (var_wdepnulrbot * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign14850_e12892;
        var_wdep_dn6 = assign14850_e12892_d_n6;
        var_wdep_dn7 = assign14850_e12892_d_n7;
        var_wdep_dn8 = assign14850_e12892_d_n8;
        var_wdep_dn9 = assign14850_e12892_d_n9;

        let (assign14860_e12910, assign14860_e12910_d_n6, assign14860_e12910_d_n7, assign14860_e12910_d_n8, assign14860_e12910_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) {
        let assign14860_e12905: f64 = (var_zinv - 1.0);
        let assign14860_e12907: f64 = (assign14860_e12905 * var_wdep);
        let assign14860_e12908: f64 = (var_ftdbot * assign14860_e12907);
        (assign14860_e12908, (var_ftdbot * (assign14860_e12905 * var_wdep_dn6)), (var_ftdbot * (assign14860_e12905 * var_wdep_dn7)), (var_ftdbot * (assign14860_e12905 * var_wdep_dn8)), (var_ftdbot * (assign14860_e12905 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign14860_e12910;
        var_asrh_dn6 = assign14860_e12910_d_n6;
        var_asrh_dn7 = assign14860_e12910_d_n7;
        var_asrh_dn8 = assign14860_e12910_d_n8;
        var_asrh_dn9 = assign14860_e12910_d_n9;

        let (assign14870_e12926, assign14870_e12926_d_n6, assign14870_e12926_d_n7, assign14870_e12926_d_n8, assign14870_e12926_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard247 == 0.0)) {
        let assign14870_e12923: f64 = (var_asrh * var_wsrh);
        let assign14870_e12924: f64 = (p.p857 * assign14870_e12923);
        (assign14870_e12924, (p.p857 * (var_asrh_dn6 * var_wsrh)), (p.p857 * (var_asrh_dn7 * var_wsrh)), (p.p857 * (var_asrh_dn8 * var_wsrh)), (p.p857 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign14870_e12926;
        var_isrh_dn6 = assign14870_e12926_d_n6;
        var_isrh_dn7 = assign14870_e12926_d_n7;
        var_isrh_dn8 = assign14870_e12926_d_n8;
        var_isrh_dn9 = assign14870_e12926_d_n9;

        let assign14880_e12929: f64 = if p.p862 == 0.0 { 1.0 } else { 0.0 };
        var_guard250 = assign14880_e12929;

        let (assign14890_e12940, assign14890_e12940_d_n6, assign14890_e12940_d_n7, assign14890_e12940_d_n8, assign14890_e12940_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign14890_e12940;
        var_itat_dn6 = assign14890_e12940_d_n6;
        var_itat_dn7 = assign14890_e12940_d_n7;
        var_itat_dn8 = assign14890_e12940_d_n8;
        var_itat_dn9 = assign14890_e12940_d_n9;

        let (assign14900_e12958, assign14900_e12958_d_n6, assign14900_e12958_d_n7, assign14900_e12958_d_n8, assign14900_e12958_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14900_e12953: f64 = (var_wdep * var_one_minus_pbot);
        let assign14900_e12955: f64 = (assign14900_e12953 / var_vbi_minus_vjsrh);
        let assign14900_e12956: f64 = (var_btatpartbot * assign14900_e12955);
        (assign14900_e12956, (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn9 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign14900_e12958;
        var_btat_dn6 = assign14900_e12958_d_n6;
        var_btat_dn7 = assign14900_e12958_d_n7;
        var_btat_dn8 = assign14900_e12958_d_n8;
        var_btat_dn9 = assign14900_e12958_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_guard241_slot = var_guard241;
        *var_guard242_slot = var_guard242;
        *var_guard243_slot = var_guard243;
        *var_guard244_slot = var_guard244;
        *var_guard245_slot = var_guard245;
        *var_guard246_slot = var_guard246;
        *var_guard247_slot = var_guard247;
        *var_guard248_slot = var_guard248;
        *var_guard249_slot = var_guard249;
        *var_guard250_slot = var_guard250;
        *var_id__blk212_slot = var_id__blk212;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_v5_slot = var_v5;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_22(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatbot: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cerfc: f64,
        var_fbbtbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard246: f64,
        var_guard250: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_wdepnulrinvbot: f64,
        var_wsrh: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard251_slot: &mut f64,
        var_guard252_slot: &mut f64,
        var_guard253_slot: &mut f64,
        var_guard254_slot: &mut f64,
        var_guard255_slot: &mut f64,
        var_guard256_slot: &mut f64,
        var_guard257_slot: &mut f64,
        var_guard258_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard251: f64 = *var_guard251_slot;
        let mut var_guard252: f64 = *var_guard252_slot;
        let mut var_guard253: f64 = *var_guard253_slot;
        let mut var_guard254: f64 = *var_guard254_slot;
        let mut var_guard255: f64 = *var_guard255_slot;
        let mut var_guard256: f64 = *var_guard256_slot;
        let mut var_guard257: f64 = *var_guard257_slot;
        let mut var_guard258: f64 = *var_guard258_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign14910_e12974, assign14910_e12974_d_n6, assign14910_e12974_d_n7, assign14910_e12974_d_n8, assign14910_e12974_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14910_e12970: f64 = (0.666666666666667 * var_atatbot);
        let assign14910_e12972: f64 = (assign14910_e12970 / var_btat);
        (assign14910_e12972, (-((assign14910_e12970 * var_btat_dn6) / (var_btat * var_btat))), (-((assign14910_e12970 * var_btat_dn7) / (var_btat * var_btat))), (-((assign14910_e12970 * var_btat_dn8) / (var_btat * var_btat))), (-((assign14910_e12970 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign14910_e12974;
        var_twoatatoverthreebtat_dn6 = assign14910_e12974_d_n6;
        var_twoatatoverthreebtat_dn7 = assign14910_e12974_d_n7;
        var_twoatatoverthreebtat_dn8 = assign14910_e12974_d_n8;
        var_twoatatoverthreebtat_dn9 = assign14910_e12974_d_n9;

        let (assign14920_e12988, assign14920_e12988_d_n6, assign14920_e12988_d_n7, assign14920_e12988_d_n8, assign14920_e12988_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14920_e12986: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign14920_e12986, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign14920_e12988;
        var_umaxbeforelimiting_dn6 = assign14920_e12988_d_n6;
        var_umaxbeforelimiting_dn7 = assign14920_e12988_d_n7;
        var_umaxbeforelimiting_dn8 = assign14920_e12988_d_n8;
        var_umaxbeforelimiting_dn9 = assign14920_e12988_d_n9;

        let (assign14930_e13009, assign14930_e13009_d_n6, assign14930_e13009_d_n7, assign14930_e13009_d_n8, assign14930_e13009_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14930_e13000: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign14930_e13003: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign14930_e13005: f64 = (assign14930_e13003 + 1.0);
        let assign14930_e13006: f64 = (assign14930_e13000 / assign14930_e13005);
        let assign14930_e13007: f64 = (assign14930_e13006).sqrt();
        (assign14930_e13007, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign14930_e13005) - (assign14930_e13000 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign14930_e13005 * assign14930_e13005)) / (2.0 * assign14930_e13007)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign14930_e13005) - (assign14930_e13000 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign14930_e13005 * assign14930_e13005)) / (2.0 * assign14930_e13007)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign14930_e13005) - (assign14930_e13000 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign14930_e13005 * assign14930_e13005)) / (2.0 * assign14930_e13007)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign14930_e13005) - (assign14930_e13000 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign14930_e13005 * assign14930_e13005)) / (2.0 * assign14930_e13007)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign14930_e13009;
        var_umax_dn6 = assign14930_e13009_d_n6;
        var_umax_dn7 = assign14930_e13009_d_n7;
        var_umax_dn8 = assign14930_e13009_d_n8;
        var_umax_dn9 = assign14930_e13009_d_n9;

        let (assign14940_e13022, assign14940_e13022_d_n6, assign14940_e13022_d_n7, assign14940_e13022_d_n8, assign14940_e13022_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14940_e13020: f64 = (var_umax).sqrt();
        (assign14940_e13020, (var_umax_dn6 / (2.0 * assign14940_e13020)), (var_umax_dn7 / (2.0 * assign14940_e13020)), (var_umax_dn8 / (2.0 * assign14940_e13020)), (var_umax_dn9 / (2.0 * assign14940_e13020)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign14940_e13022;
        var_sqrtumax_dn6 = assign14940_e13022_d_n6;
        var_sqrtumax_dn7 = assign14940_e13022_d_n7;
        var_sqrtumax_dn8 = assign14940_e13022_d_n8;
        var_sqrtumax_dn9 = assign14940_e13022_d_n9;

        let (assign14950_e13036, assign14950_e13036_d_n6, assign14950_e13036_d_n7, assign14950_e13036_d_n8, assign14950_e13036_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14950_e13034: f64 = (var_umax * var_sqrtumax);
        (assign14950_e13034, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign14950_e13036;
        var_umaxpoweronepointfive_dn6 = assign14950_e13036_d_n6;
        var_umaxpoweronepointfive_dn7 = assign14950_e13036_d_n7;
        var_umaxpoweronepointfive_dn8 = assign14950_e13036_d_n8;
        var_umaxpoweronepointfive_dn9 = assign14950_e13036_d_n9;

        let assign14960_e13038: f64 = (-p.p848);
        let assign14960_e13040: f64 = (assign14960_e13038 * var_one_over_one_minus_pbot);
        let assign14960_e13042: f64 = (-1.0);
        let assign14960_e13043: f64 = if assign14960_e13040 == assign14960_e13042 { 1.0 } else { 0.0 };
        var_guard251 = assign14960_e13043;

        let (assign14970_e13063, assign14970_e13063_d_n6, assign14970_e13063_d_n7, assign14970_e13063_d_n8, assign14970_e13063_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard251 != 0.0)) {
        let assign14970_e13059: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign14970_e13060: f64 = (1.0 + assign14970_e13059);
        let assign14970_e13061: f64 = (1.0 / assign14970_e13060);
        (assign14970_e13061, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign14970_e13060 * assign14970_e13060))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign14970_e13060 * assign14970_e13060))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign14970_e13060 * assign14970_e13060))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign14970_e13060 * assign14970_e13060))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign14970_e13063;
        var_wgamma_dn6 = assign14970_e13063_d_n6;
        var_wgamma_dn7 = assign14970_e13063_d_n7;
        var_wgamma_dn8 = assign14970_e13063_d_n8;
        var_wgamma_dn9 = assign14970_e13063_d_n9;

        let (assign14980_e13087, assign14980_e13087_d_n6, assign14980_e13087_d_n7, assign14980_e13087_d_n8, assign14980_e13087_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard251 == 0.0)) {
        let assign14980_e13079: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign14980_e13080: f64 = (1.0 + assign14980_e13079);
        let assign14980_e13082: f64 = (-p.p848);
        let assign14980_e13084: f64 = (assign14980_e13082 * var_one_over_one_minus_pbot);
        let assign14980_e13085: f64 = (assign14980_e13080).powf(assign14980_e13084);
        (assign14980_e13085, if 0.0 == 0.0 && ((assign14980_e13084) as f64).is_finite() && ((assign14980_e13084) as f64).fract() == 0.0 { if assign14980_e13084 == 0.0 { 0.0 } else { (assign14980_e13084 * ((assign14980_e13080).powf(assign14980_e13084 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign14980_e13085 * (assign14980_e13084 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign14980_e13080))) }, if 0.0 == 0.0 && ((assign14980_e13084) as f64).is_finite() && ((assign14980_e13084) as f64).fract() == 0.0 { if assign14980_e13084 == 0.0 { 0.0 } else { (assign14980_e13084 * ((assign14980_e13080).powf(assign14980_e13084 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign14980_e13085 * (assign14980_e13084 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign14980_e13080))) }, if 0.0 == 0.0 && ((assign14980_e13084) as f64).is_finite() && ((assign14980_e13084) as f64).fract() == 0.0 { if assign14980_e13084 == 0.0 { 0.0 } else { (assign14980_e13084 * ((assign14980_e13080).powf(assign14980_e13084 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign14980_e13085 * (assign14980_e13084 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign14980_e13080))) }, if 0.0 == 0.0 && ((assign14980_e13084) as f64).is_finite() && ((assign14980_e13084) as f64).fract() == 0.0 { if assign14980_e13084 == 0.0 { 0.0 } else { (assign14980_e13084 * ((assign14980_e13080).powf(assign14980_e13084 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign14980_e13085 * (assign14980_e13084 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign14980_e13080))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign14980_e13087;
        var_wgamma_dn6 = assign14980_e13087_d_n6;
        var_wgamma_dn7 = assign14980_e13087_d_n7;
        var_wgamma_dn8 = assign14980_e13087_d_n8;
        var_wgamma_dn9 = assign14980_e13087_d_n9;

        let (assign14990_e13105, assign14990_e13105_d_n6, assign14990_e13105_d_n7, assign14990_e13105_d_n8, assign14990_e13105_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign14990_e13099: f64 = (var_wsrh * var_wgamma);
        let assign14990_e13102: f64 = (var_wsrh + var_wgamma);
        let assign14990_e13103: f64 = (assign14990_e13099 / assign14990_e13102);
        (assign14990_e13103, ((((var_wsrh * var_wgamma_dn6) * assign14990_e13102) - (assign14990_e13099 * var_wgamma_dn6)) / (assign14990_e13102 * assign14990_e13102)), ((((var_wsrh * var_wgamma_dn7) * assign14990_e13102) - (assign14990_e13099 * var_wgamma_dn7)) / (assign14990_e13102 * assign14990_e13102)), ((((var_wsrh * var_wgamma_dn8) * assign14990_e13102) - (assign14990_e13099 * var_wgamma_dn8)) / (assign14990_e13102 * assign14990_e13102)), ((((var_wsrh * var_wgamma_dn9) * assign14990_e13102) - (assign14990_e13099 * var_wgamma_dn9)) / (assign14990_e13102 * assign14990_e13102)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign14990_e13105;
        var_wtat_dn6 = assign14990_e13105_d_n6;
        var_wtat_dn7 = assign14990_e13105_d_n7;
        var_wtat_dn8 = assign14990_e13105_d_n8;
        var_wtat_dn9 = assign14990_e13105_d_n9;

        let (assign15000_e13122, assign15000_e13122_d_n6, assign15000_e13122_d_n7, assign15000_e13122_d_n8, assign15000_e13122_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15000_e13118: f64 = (var_btat / var_sqrtumax);
        let assign15000_e13119: f64 = (0.375 * assign15000_e13118);
        let assign15000_e13120: f64 = (assign15000_e13119).sqrt();
        (assign15000_e13120, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15000_e13120)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15000_e13120)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15000_e13120)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15000_e13120)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign15000_e13122;
        var_ktat_dn6 = assign15000_e13122_d_n6;
        var_ktat_dn7 = assign15000_e13122_d_n7;
        var_ktat_dn8 = assign15000_e13122_d_n8;
        var_ktat_dn9 = assign15000_e13122_d_n9;

        let (assign15010_e13140, assign15010_e13140_d_n6, assign15010_e13140_d_n7, assign15010_e13140_d_n8, assign15010_e13140_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15010_e13135: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign15010_e13136: f64 = (2.0 * assign15010_e13135);
        let assign15010_e13138: f64 = (assign15010_e13136 - var_umax);
        (assign15010_e13138, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign15010_e13140;
        var_ltat_dn6 = assign15010_e13140_d_n6;
        var_ltat_dn7 = assign15010_e13140_d_n7;
        var_ltat_dn8 = assign15010_e13140_d_n8;
        var_ltat_dn9 = assign15010_e13140_d_n9;

        let (assign15020_e13166, assign15020_e13166_d_n6, assign15020_e13166_d_n7, assign15020_e13166_d_n8, assign15020_e13166_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15020_e13152: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign15020_e13154: f64 = (assign15020_e13152 * var_sqrtumax);
        let assign15020_e13157: f64 = (var_atatbot * var_umax);
        let assign15020_e13158: f64 = (assign15020_e13154 - assign15020_e13157);
        let assign15020_e13162: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign15020_e13163: f64 = (0.5 * assign15020_e13162);
        let assign15020_e13164: f64 = (assign15020_e13158 + assign15020_e13163);
        (assign15020_e13164, (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign15020_e13152 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign15020_e13152 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign15020_e13152 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign15020_e13152 * var_sqrtumax_dn9)) - (var_atatbot * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign15020_e13166;
        var_mtat_dn6 = assign15020_e13166_d_n6;
        var_mtat_dn7 = assign15020_e13166_d_n7;
        var_mtat_dn8 = assign15020_e13166_d_n8;
        var_mtat_dn9 = assign15020_e13166_d_n9;

        let (assign15030_e13182, assign15030_e13182_d_n6, assign15030_e13182_d_n7, assign15030_e13182_d_n8, assign15030_e13182_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15030_e13178: f64 = (var_ltat - 1.0);
        let assign15030_e13180: f64 = (assign15030_e13178 * var_ktat);
        (assign15030_e13180, ((var_ltat_dn6 * var_ktat) + (assign15030_e13178 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign15030_e13178 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign15030_e13178 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign15030_e13178 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign15030_e13182;
        var_xerfc_dn6 = assign15030_e13182_d_n6;
        var_xerfc_dn7 = assign15030_e13182_d_n7;
        var_xerfc_dn8 = assign15030_e13182_d_n8;
        var_xerfc_dn9 = assign15030_e13182_d_n9;

        let (assign15040_e13196, assign15040_e13196_d_n6, assign15040_e13196_d_n7, assign15040_e13196_d_n8, assign15040_e13196_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15040_e13194: f64 = (var_xerfc * var_xerfc);
        (assign15040_e13194, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign15040_e13196;
        var_ysq_dn6 = assign15040_e13196_d_n6;
        var_ysq_dn7 = assign15040_e13196_d_n7;
        var_ysq_dn8 = assign15040_e13196_d_n8;
        var_ysq_dn9 = assign15040_e13196_d_n9;

        let assign15050_e13199: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard252 = assign15050_e13199;

        let (assign15060_e13219, assign15060_e13219_d_n6, assign15060_e13219_d_n7, assign15060_e13219_d_n8, assign15060_e13219_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard252 != 0.0)) {
        let assign15060_e13215: f64 = (var_perfc * var_xerfc);
        let assign15060_e13216: f64 = (1.0 + assign15060_e13215);
        let assign15060_e13217: f64 = (1.0 / assign15060_e13216);
        (assign15060_e13217, (-((var_perfc * var_xerfc_dn6) / (assign15060_e13216 * assign15060_e13216))), (-((var_perfc * var_xerfc_dn7) / (assign15060_e13216 * assign15060_e13216))), (-((var_perfc * var_xerfc_dn8) / (assign15060_e13216 * assign15060_e13216))), (-((var_perfc * var_xerfc_dn9) / (assign15060_e13216 * assign15060_e13216))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign15060_e13219;
        var_terfc_dn6 = assign15060_e13219_d_n6;
        var_terfc_dn7 = assign15060_e13219_d_n7;
        var_terfc_dn8 = assign15060_e13219_d_n8;
        var_terfc_dn9 = assign15060_e13219_d_n9;

        let (assign15070_e13240, assign15070_e13240_d_n6, assign15070_e13240_d_n7, assign15070_e13240_d_n8, assign15070_e13240_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard252 == 0.0)) {
        let assign15070_e13236: f64 = (var_perfc * var_xerfc);
        let assign15070_e13237: f64 = (1.0 - assign15070_e13236);
        let assign15070_e13238: f64 = (1.0 / assign15070_e13237);
        (assign15070_e13238, (-((-(var_perfc * var_xerfc_dn6)) / (assign15070_e13237 * assign15070_e13237))), (-((-(var_perfc * var_xerfc_dn7)) / (assign15070_e13237 * assign15070_e13237))), (-((-(var_perfc * var_xerfc_dn8)) / (assign15070_e13237 * assign15070_e13237))), (-((-(var_perfc * var_xerfc_dn9)) / (assign15070_e13237 * assign15070_e13237))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign15070_e13240;
        var_terfc_dn6 = assign15070_e13240_d_n6;
        var_terfc_dn7 = assign15070_e13240_d_n7;
        var_terfc_dn8 = assign15070_e13240_d_n8;
        var_terfc_dn9 = assign15070_e13240_d_n9;

        let assign15080_e13242: f64 = (-var_ysq);
        let assign15080_e13244: f64 = (assign15080_e13242 + var_mtat);
        let assign15080_e13246: f64 = (-230.25850929940458);
        let assign15080_e13247: f64 = if assign15080_e13244 > assign15080_e13246 { 1.0 } else { 0.0 };
        var_guard253 = assign15080_e13247;

        let (assign15090_e13265, assign15090_e13265_d_n6, assign15090_e13265_d_n7, assign15090_e13265_d_n8, assign15090_e13265_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard253 != 0.0)) {
        let assign15090_e13260: f64 = (-var_ysq);
        let assign15090_e13262: f64 = (assign15090_e13260 + var_mtat);
        let assign15090_e13263: f64 = (assign15090_e13262).exp();
        (assign15090_e13263, (assign15090_e13263 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign15090_e13263 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign15090_e13263 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign15090_e13263 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15090_e13265;
        var_tmp_dn6 = assign15090_e13265_d_n6;
        var_tmp_dn7 = assign15090_e13265_d_n7;
        var_tmp_dn8 = assign15090_e13265_d_n8;
        var_tmp_dn9 = assign15090_e13265_d_n9;

        let (assign15100_e13314, assign15100_e13314_d_n6, assign15100_e13314_d_n7, assign15100_e13314_d_n8, assign15100_e13314_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard253 == 0.0)) {
        let assign15100_e13281: f64 = (-230.25850929940458);
        let assign15100_e13283: f64 = (-var_ysq);
        let assign15100_e13285: f64 = (assign15100_e13283 + var_mtat);
        let assign15100_e13286: f64 = (assign15100_e13281 - assign15100_e13285);
        let assign15100_e13290: f64 = (-230.25850929940458);
        let assign15100_e13292: f64 = (-var_ysq);
        let assign15100_e13294: f64 = (assign15100_e13292 + var_mtat);
        let assign15100_e13295: f64 = (assign15100_e13290 - assign15100_e13294);
        let assign15100_e13298: f64 = (-230.25850929940458);
        let assign15100_e13300: f64 = (-var_ysq);
        let assign15100_e13302: f64 = (assign15100_e13300 + var_mtat);
        let assign15100_e13303: f64 = (assign15100_e13298 - assign15100_e13302);
        let assign15100_e13305: f64 = (assign15100_e13303 * 0.3333333333333333);
        let assign15100_e13306: f64 = (1.0 + assign15100_e13305);
        let assign15100_e13307: f64 = (assign15100_e13295 * assign15100_e13306);
        let assign15100_e13308: f64 = (0.5 * assign15100_e13307);
        let assign15100_e13309: f64 = (1.0 + assign15100_e13308);
        let assign15100_e13310: f64 = (assign15100_e13286 * assign15100_e13309);
        let assign15100_e13311: f64 = (1.0 + assign15100_e13310);
        let assign15100_e13312: f64 = (1e-100 / assign15100_e13311);
        (assign15100_e13312, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign15100_e13309) + (assign15100_e13286 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign15100_e13306) + (assign15100_e13295 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign15100_e13311 * assign15100_e13311))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign15100_e13309) + (assign15100_e13286 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign15100_e13306) + (assign15100_e13295 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign15100_e13311 * assign15100_e13311))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign15100_e13309) + (assign15100_e13286 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign15100_e13306) + (assign15100_e13295 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign15100_e13311 * assign15100_e13311))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign15100_e13309) + (assign15100_e13286 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign15100_e13306) + (assign15100_e13295 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign15100_e13311 * assign15100_e13311))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15100_e13314;
        var_tmp_dn6 = assign15100_e13314_d_n6;
        var_tmp_dn7 = assign15100_e13314_d_n7;
        var_tmp_dn8 = assign15100_e13314_d_n8;
        var_tmp_dn9 = assign15100_e13314_d_n9;

        let (assign15110_e13344, assign15110_e13344_d_n6, assign15110_e13344_d_n7, assign15110_e13344_d_n8, assign15110_e13344_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15110_e13326: f64 = (0.29214664 * var_terfc);
        let assign15110_e13330: f64 = (var_terfc * var_terfc);
        let assign15110_e13331: f64 = (var_berfc * assign15110_e13330);
        let assign15110_e13332: f64 = (assign15110_e13326 + assign15110_e13331);
        let assign15110_e13336: f64 = (var_terfc * var_terfc);
        let assign15110_e13338: f64 = (assign15110_e13336 * var_terfc);
        let assign15110_e13339: f64 = (var_cerfc * assign15110_e13338);
        let assign15110_e13340: f64 = (assign15110_e13332 + assign15110_e13339);
        let assign15110_e13342: f64 = (assign15110_e13340 * var_tmp);
        (assign15110_e13342, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign15110_e13336 * var_terfc_dn6)))) * var_tmp) + (assign15110_e13340 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign15110_e13336 * var_terfc_dn7)))) * var_tmp) + (assign15110_e13340 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign15110_e13336 * var_terfc_dn8)))) * var_tmp) + (assign15110_e13340 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign15110_e13336 * var_terfc_dn9)))) * var_tmp) + (assign15110_e13340 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign15110_e13344;
        var_erfcpos_dn6 = assign15110_e13344_d_n6;
        var_erfcpos_dn7 = assign15110_e13344_d_n7;
        var_erfcpos_dn8 = assign15110_e13344_d_n8;
        var_erfcpos_dn9 = assign15110_e13344_d_n9;

        let assign15120_e13347: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard254 = assign15120_e13347;

        let (assign15130_e13361, assign15130_e13361_d_n6, assign15130_e13361_d_n7, assign15130_e13361_d_n8, assign15130_e13361_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard254 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign15130_e13361;
        var_erfctimesexpmtat_dn6 = assign15130_e13361_d_n6;
        var_erfctimesexpmtat_dn7 = assign15130_e13361_d_n7;
        var_erfctimesexpmtat_dn8 = assign15130_e13361_d_n8;
        var_erfctimesexpmtat_dn9 = assign15130_e13361_d_n9;

        let assign15140_e13364: f64 = (-230.25850929940458);
        let assign15140_e13365: f64 = if var_mtat > assign15140_e13364 { 1.0 } else { 0.0 };
        var_guard255 = assign15140_e13365;

        let (assign15150_e13383, assign15150_e13383_d_n6, assign15150_e13383_d_n7, assign15150_e13383_d_n8, assign15150_e13383_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard254 == 0.0)) && (var_guard255 != 0.0)) {
        let assign15150_e13381: f64 = (var_mtat).exp();
        (assign15150_e13381, (assign15150_e13381 * var_mtat_dn6), (assign15150_e13381 * var_mtat_dn7), (assign15150_e13381 * var_mtat_dn8), (assign15150_e13381 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15150_e13383;
        var_tmp_dn6 = assign15150_e13383_d_n6;
        var_tmp_dn7 = assign15150_e13383_d_n7;
        var_tmp_dn8 = assign15150_e13383_d_n8;
        var_tmp_dn9 = assign15150_e13383_d_n9;

        let (assign15160_e13426, assign15160_e13426_d_n6, assign15160_e13426_d_n7, assign15160_e13426_d_n8, assign15160_e13426_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard254 == 0.0)) && (var_guard255 == 0.0)) {
        let assign15160_e13402: f64 = (-230.25850929940458);
        let assign15160_e13404: f64 = (assign15160_e13402 - var_mtat);
        let assign15160_e13408: f64 = (-230.25850929940458);
        let assign15160_e13410: f64 = (assign15160_e13408 - var_mtat);
        let assign15160_e13413: f64 = (-230.25850929940458);
        let assign15160_e13415: f64 = (assign15160_e13413 - var_mtat);
        let assign15160_e13417: f64 = (assign15160_e13415 * 0.3333333333333333);
        let assign15160_e13418: f64 = (1.0 + assign15160_e13417);
        let assign15160_e13419: f64 = (assign15160_e13410 * assign15160_e13418);
        let assign15160_e13420: f64 = (0.5 * assign15160_e13419);
        let assign15160_e13421: f64 = (1.0 + assign15160_e13420);
        let assign15160_e13422: f64 = (assign15160_e13404 * assign15160_e13421);
        let assign15160_e13423: f64 = (1.0 + assign15160_e13422);
        let assign15160_e13424: f64 = (1e-100 / assign15160_e13423);
        (assign15160_e13424, (-((1e-100 * (((-var_mtat_dn6) * assign15160_e13421) + (assign15160_e13404 * (0.5 * (((-var_mtat_dn6) * assign15160_e13418) + (assign15160_e13410 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign15160_e13423 * assign15160_e13423))), (-((1e-100 * (((-var_mtat_dn7) * assign15160_e13421) + (assign15160_e13404 * (0.5 * (((-var_mtat_dn7) * assign15160_e13418) + (assign15160_e13410 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign15160_e13423 * assign15160_e13423))), (-((1e-100 * (((-var_mtat_dn8) * assign15160_e13421) + (assign15160_e13404 * (0.5 * (((-var_mtat_dn8) * assign15160_e13418) + (assign15160_e13410 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign15160_e13423 * assign15160_e13423))), (-((1e-100 * (((-var_mtat_dn9) * assign15160_e13421) + (assign15160_e13404 * (0.5 * (((-var_mtat_dn9) * assign15160_e13418) + (assign15160_e13410 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign15160_e13423 * assign15160_e13423))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15160_e13426;
        var_tmp_dn6 = assign15160_e13426_d_n6;
        var_tmp_dn7 = assign15160_e13426_d_n7;
        var_tmp_dn8 = assign15160_e13426_d_n8;
        var_tmp_dn9 = assign15160_e13426_d_n9;

        let (assign15170_e13445, assign15170_e13445_d_n6, assign15170_e13445_d_n7, assign15170_e13445_d_n8, assign15170_e13445_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) && (var_guard254 == 0.0)) {
        let assign15170_e13441: f64 = (2.0 * var_tmp);
        let assign15170_e13443: f64 = (assign15170_e13441 - var_erfcpos);
        (assign15170_e13443, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign15170_e13445;
        var_erfctimesexpmtat_dn6 = assign15170_e13445_d_n6;
        var_erfctimesexpmtat_dn7 = assign15170_e13445_d_n7;
        var_erfctimesexpmtat_dn8 = assign15170_e13445_d_n8;
        var_erfctimesexpmtat_dn9 = assign15170_e13445_d_n9;

        let (assign15180_e13465, assign15180_e13465_d_n6, assign15180_e13465_d_n7, assign15180_e13465_d_n8, assign15180_e13465_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15180_e13457: f64 = (1.772453850905516 * 0.5);
        let assign15180_e13460: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign15180_e13462: f64 = (assign15180_e13460 / var_ktat);
        let assign15180_e13463: f64 = (assign15180_e13457 * assign15180_e13462);
        (assign15180_e13463, (assign15180_e13457 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign15180_e13460 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign15180_e13457 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign15180_e13460 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign15180_e13457 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign15180_e13460 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign15180_e13457 * ((((var_atatbot * var_erfctimesexpmtat_dn9) * var_ktat) - (assign15180_e13460 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign15180_e13465;
        var_gammamax_dn6 = assign15180_e13465_d_n6;
        var_gammamax_dn7 = assign15180_e13465_d_n7;
        var_gammamax_dn8 = assign15180_e13465_d_n8;
        var_gammamax_dn9 = assign15180_e13465_d_n9;

        let (assign15190_e13483, assign15190_e13483_d_n6, assign15190_e13483_d_n7, assign15190_e13483_d_n8, assign15190_e13483_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard250 == 0.0)) {
        let assign15190_e13478: f64 = (var_asrh * var_gammamax);
        let assign15190_e13480: f64 = (assign15190_e13478 * var_wtat);
        let assign15190_e13481: f64 = (p.p862 * assign15190_e13480);
        (assign15190_e13481, (p.p862 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign15190_e13478 * var_wtat_dn6))), (p.p862 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign15190_e13478 * var_wtat_dn7))), (p.p862 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign15190_e13478 * var_wtat_dn8))), (p.p862 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign15190_e13478 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign15190_e13483;
        var_itat_dn6 = assign15190_e13483_d_n6;
        var_itat_dn7 = assign15190_e13483_d_n7;
        var_itat_dn8 = assign15190_e13483_d_n8;
        var_itat_dn9 = assign15190_e13483_d_n9;

        let assign15200_e13486: f64 = if p.p868 == 0.0 { 1.0 } else { 0.0 };
        var_guard256 = assign15200_e13486;

        let (assign15210_e13497, assign15210_e13497_d_n6, assign15210_e13497_d_n7, assign15210_e13497_d_n8, assign15210_e13497_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign15210_e13497;
        var_ibbt_dn6 = assign15210_e13497_d_n6;
        var_ibbt_dn7 = assign15210_e13497_d_n7;
        var_ibbt_dn8 = assign15210_e13497_d_n8;
        var_ibbt_dn9 = assign15210_e13497_d_n9;

        let assign15220_e13500: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard257 = assign15220_e13500;

        let (assign15230_e13519, assign15230_e13519_d_n6, assign15230_e13519_d_n7, assign15230_e13519_d_n8, assign15230_e13519_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) && (var_guard257 != 0.0)) {
        let assign15230_e13514: f64 = (p.p845 - var_vbbt);
        let assign15230_e13516: f64 = (assign15230_e13514 * var_vbirbotinv);
        let assign15230_e13517: f64 = (assign15230_e13516).sqrt();
        (assign15230_e13517, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15230_e13519;
        var_tmp_dn6 = assign15230_e13519_d_n6;
        var_tmp_dn7 = assign15230_e13519_d_n7;
        var_tmp_dn8 = assign15230_e13519_d_n8;
        var_tmp_dn9 = assign15230_e13519_d_n9;

        let (assign15240_e13540, assign15240_e13540_d_n6, assign15240_e13540_d_n7, assign15240_e13540_d_n8, assign15240_e13540_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) && (var_guard257 == 0.0)) {
        let assign15240_e13534: f64 = (p.p845 - var_vbbt);
        let assign15240_e13536: f64 = (assign15240_e13534 * var_vbirbotinv);
        let assign15240_e13538: f64 = (assign15240_e13536).powf(p.p848);
        (assign15240_e13538, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15240_e13540;
        var_tmp_dn6 = assign15240_e13540_d_n6;
        var_tmp_dn7 = assign15240_e13540_d_n7;
        var_tmp_dn8 = assign15240_e13540_d_n8;
        var_tmp_dn9 = assign15240_e13540_d_n9;

        let (assign15250_e13560, assign15250_e13560_d_n6, assign15250_e13560_d_n7, assign15250_e13560_d_n8, assign15250_e13560_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) {
        let assign15250_e13553: f64 = (p.p845 - var_vbbt);
        let assign15250_e13555: f64 = (assign15250_e13553 * var_wdepnulrinvbot);
        let assign15250_e13557: f64 = (assign15250_e13555 / var_tmp);
        let assign15250_e13558: f64 = (var_one_over_one_minus_pbot * assign15250_e13557);
        (assign15250_e13558, (var_one_over_one_minus_pbot * (-((assign15250_e13555 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign15250_e13555 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign15250_e13555 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign15250_e13555 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign15250_e13560;
        var_fmaxr_dn6 = assign15250_e13560_d_n6;
        var_fmaxr_dn7 = assign15250_e13560_d_n7;
        var_fmaxr_dn8 = assign15250_e13560_d_n8;
        var_fmaxr_dn9 = assign15250_e13560_d_n9;

        let assign15260_e13562: f64 = (-var_fbbtbot);
        let assign15260_e13564: f64 = (assign15260_e13562 / var_fmaxr);
        let assign15260_e13565: f64 = (assign15260_e13564).abs();
        let assign15260_e13567: f64 = if assign15260_e13565 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard258 = assign15260_e13567;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard251_slot = var_guard251;
        *var_guard252_slot = var_guard252;
        *var_guard253_slot = var_guard253;
        *var_guard254_slot = var_guard254;
        *var_guard255_slot = var_guard255;
        *var_guard256_slot = var_guard256;
        *var_guard257_slot = var_guard257;
        *var_guard258_slot = var_guard258;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_23(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fbbtbot: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard246: f64,
        var_guard256: f64,
        var_guard258: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_psti: f64,
        var_slopebot: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard259_slot: &mut f64,
        var_guard260_slot: &mut f64,
        var_guard261_slot: &mut f64,
        var_guard262_slot: &mut f64,
        var_guard263_slot: &mut f64,
        var_guard264_slot: &mut f64,
        var_guard265_slot: &mut f64,
        var_guard266_slot: &mut f64,
        var_guard267_slot: &mut f64,
        var_guard268_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_guard259: f64 = *var_guard259_slot;
        let mut var_guard260: f64 = *var_guard260_slot;
        let mut var_guard261: f64 = *var_guard261_slot;
        let mut var_guard262: f64 = *var_guard262_slot;
        let mut var_guard263: f64 = *var_guard263_slot;
        let mut var_guard264: f64 = *var_guard264_slot;
        let mut var_guard265: f64 = *var_guard265_slot;
        let mut var_guard266: f64 = *var_guard266_slot;
        let mut var_guard267: f64 = *var_guard267_slot;
        let mut var_guard268: f64 = *var_guard268_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let (assign15270_e13585, assign15270_e13585_d_n6, assign15270_e13585_d_n7, assign15270_e13585_d_n8, assign15270_e13585_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) && (var_guard258 != 0.0)) {
        let assign15270_e13580: f64 = (-var_fbbtbot);
        let assign15270_e13582: f64 = (assign15270_e13580 / var_fmaxr);
        let assign15270_e13583: f64 = (assign15270_e13582).exp();
        (assign15270_e13583, (assign15270_e13583 * (-((assign15270_e13580 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign15270_e13583 * (-((assign15270_e13580 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign15270_e13583 * (-((assign15270_e13580 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign15270_e13583 * (-((assign15270_e13580 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15270_e13585;
        var_tmp_dn6 = assign15270_e13585_d_n6;
        var_tmp_dn7 = assign15270_e13585_d_n7;
        var_tmp_dn8 = assign15270_e13585_d_n8;
        var_tmp_dn9 = assign15270_e13585_d_n9;

        let assign15280_e13587: f64 = (-var_fbbtbot);
        let assign15280_e13589: f64 = (assign15280_e13587 / var_fmaxr);
        let assign15280_e13591: f64 = if assign15280_e13589 < 0.0 { 1.0 } else { 0.0 };
        var_guard259 = assign15280_e13591;

        let (assign15290_e13642, assign15290_e13642_d_n6, assign15290_e13642_d_n7, assign15290_e13642_d_n8, assign15290_e13642_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) && (var_guard258 == 0.0)) && (var_guard259 != 0.0)) {
        let assign15290_e13609: f64 = (-230.25850929940458);
        let assign15290_e13611: f64 = (-var_fbbtbot);
        let assign15290_e13613: f64 = (assign15290_e13611 / var_fmaxr);
        let assign15290_e13614: f64 = (assign15290_e13609 - assign15290_e13613);
        let assign15290_e13618: f64 = (-230.25850929940458);
        let assign15290_e13620: f64 = (-var_fbbtbot);
        let assign15290_e13622: f64 = (assign15290_e13620 / var_fmaxr);
        let assign15290_e13623: f64 = (assign15290_e13618 - assign15290_e13622);
        let assign15290_e13626: f64 = (-230.25850929940458);
        let assign15290_e13628: f64 = (-var_fbbtbot);
        let assign15290_e13630: f64 = (assign15290_e13628 / var_fmaxr);
        let assign15290_e13631: f64 = (assign15290_e13626 - assign15290_e13630);
        let assign15290_e13633: f64 = (assign15290_e13631 * 0.3333333333333333);
        let assign15290_e13634: f64 = (1.0 + assign15290_e13633);
        let assign15290_e13635: f64 = (assign15290_e13623 * assign15290_e13634);
        let assign15290_e13636: f64 = (0.5 * assign15290_e13635);
        let assign15290_e13637: f64 = (1.0 + assign15290_e13636);
        let assign15290_e13638: f64 = (assign15290_e13614 * assign15290_e13637);
        let assign15290_e13639: f64 = (1.0 + assign15290_e13638);
        let assign15290_e13640: f64 = (1e-100 / assign15290_e13639);
        (assign15290_e13640, (-((1e-100 * (((-(-((assign15290_e13611 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign15290_e13637) + (assign15290_e13614 * (0.5 * (((-(-((assign15290_e13620 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign15290_e13634) + (assign15290_e13623 * ((-(-((assign15290_e13628 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15290_e13639 * assign15290_e13639))), (-((1e-100 * (((-(-((assign15290_e13611 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign15290_e13637) + (assign15290_e13614 * (0.5 * (((-(-((assign15290_e13620 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign15290_e13634) + (assign15290_e13623 * ((-(-((assign15290_e13628 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15290_e13639 * assign15290_e13639))), (-((1e-100 * (((-(-((assign15290_e13611 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign15290_e13637) + (assign15290_e13614 * (0.5 * (((-(-((assign15290_e13620 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign15290_e13634) + (assign15290_e13623 * ((-(-((assign15290_e13628 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15290_e13639 * assign15290_e13639))), (-((1e-100 * (((-(-((assign15290_e13611 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign15290_e13637) + (assign15290_e13614 * (0.5 * (((-(-((assign15290_e13620 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign15290_e13634) + (assign15290_e13623 * ((-(-((assign15290_e13628 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15290_e13639 * assign15290_e13639))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15290_e13642;
        var_tmp_dn6 = assign15290_e13642_d_n6;
        var_tmp_dn7 = assign15290_e13642_d_n7;
        var_tmp_dn8 = assign15290_e13642_d_n8;
        var_tmp_dn9 = assign15290_e13642_d_n9;

        let (assign15300_e13691, assign15300_e13691_d_n6, assign15300_e13691_d_n7, assign15300_e13691_d_n8, assign15300_e13691_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) && (var_guard258 == 0.0)) && (var_guard259 == 0.0)) {
        let assign15300_e13661: f64 = (-var_fbbtbot);
        let assign15300_e13663: f64 = (assign15300_e13661 / var_fmaxr);
        let assign15300_e13665: f64 = (assign15300_e13663 - 230.25850929940458);
        let assign15300_e13669: f64 = (-var_fbbtbot);
        let assign15300_e13671: f64 = (assign15300_e13669 / var_fmaxr);
        let assign15300_e13673: f64 = (assign15300_e13671 - 230.25850929940458);
        let assign15300_e13676: f64 = (-var_fbbtbot);
        let assign15300_e13678: f64 = (assign15300_e13676 / var_fmaxr);
        let assign15300_e13680: f64 = (assign15300_e13678 - 230.25850929940458);
        let assign15300_e13682: f64 = (assign15300_e13680 * 0.3333333333333333);
        let assign15300_e13683: f64 = (1.0 + assign15300_e13682);
        let assign15300_e13684: f64 = (assign15300_e13673 * assign15300_e13683);
        let assign15300_e13685: f64 = (0.5 * assign15300_e13684);
        let assign15300_e13686: f64 = (1.0 + assign15300_e13685);
        let assign15300_e13687: f64 = (assign15300_e13665 * assign15300_e13686);
        let assign15300_e13688: f64 = (1.0 + assign15300_e13687);
        let assign15300_e13689: f64 = (1e100 * assign15300_e13688);
        (assign15300_e13689, (1e100 * (((-((assign15300_e13661 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign15300_e13686) + (assign15300_e13665 * (0.5 * (((-((assign15300_e13669 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign15300_e13683) + (assign15300_e13673 * ((-((assign15300_e13676 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign15300_e13661 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign15300_e13686) + (assign15300_e13665 * (0.5 * (((-((assign15300_e13669 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign15300_e13683) + (assign15300_e13673 * ((-((assign15300_e13676 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign15300_e13661 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign15300_e13686) + (assign15300_e13665 * (0.5 * (((-((assign15300_e13669 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign15300_e13683) + (assign15300_e13673 * ((-((assign15300_e13676 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign15300_e13661 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign15300_e13686) + (assign15300_e13665 * (0.5 * (((-((assign15300_e13669 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign15300_e13683) + (assign15300_e13673 * ((-((assign15300_e13676 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15300_e13691;
        var_tmp_dn6 = assign15300_e13691_d_n6;
        var_tmp_dn7 = assign15300_e13691_d_n7;
        var_tmp_dn8 = assign15300_e13691_d_n8;
        var_tmp_dn9 = assign15300_e13691_d_n9;

        let (assign15310_e13711, assign15310_e13711_d_n6, assign15310_e13711_d_n7, assign15310_e13711_d_n8, assign15310_e13711_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard256 == 0.0)) {
        let assign15310_e13704: f64 = (var_v1 * var_fmaxr);
        let assign15310_e13706: f64 = (assign15310_e13704 * var_fmaxr);
        let assign15310_e13708: f64 = (assign15310_e13706 * var_tmp);
        let assign15310_e13709: f64 = (p.p868 * assign15310_e13708);
        (assign15310_e13709, (p.p868 * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign15310_e13704 * var_fmaxr_dn6)) * var_tmp) + (assign15310_e13706 * var_tmp_dn6))), (p.p868 * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign15310_e13704 * var_fmaxr_dn7)) * var_tmp) + (assign15310_e13706 * var_tmp_dn7))), (p.p868 * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign15310_e13704 * var_fmaxr_dn8)) * var_tmp) + (assign15310_e13706 * var_tmp_dn8))), (p.p868 * (((((var_v1 * var_fmaxr_dn9) * var_fmaxr) + (assign15310_e13704 * var_fmaxr_dn9)) * var_tmp) + (assign15310_e13706 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign15310_e13711;
        var_ibbt_dn6 = assign15310_e13711_d_n6;
        var_ibbt_dn7 = assign15310_e13711_d_n7;
        var_ibbt_dn8 = assign15310_e13711_d_n8;
        var_ibbt_dn9 = assign15310_e13711_d_n9;

        let assign15320_e13714: f64 = if p.p877 > 1000.0 { 1.0 } else { 0.0 };
        var_guard260 = assign15320_e13714;

        let (assign15330_e13725, assign15330_e13725_d_n6, assign15330_e13725_d_n7, assign15330_e13725_d_n8, assign15330_e13725_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard260 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign15330_e13725;
        var_fbreakdown_dn6 = assign15330_e13725_d_n6;
        var_fbreakdown_dn7 = assign15330_e13725_d_n7;
        var_fbreakdown_dn8 = assign15330_e13725_d_n8;
        var_fbreakdown_dn9 = assign15330_e13725_d_n9;

        let assign15340_e13728: f64 = (-var_alphaav);
        let assign15340_e13730: f64 = (assign15340_e13728 * p.p877);
        let assign15340_e13731: f64 = if var_vav > assign15340_e13730 { 1.0 } else { 0.0 };
        var_guard261 = assign15340_e13731;

        let assign15350_e13734: f64 = if p.p880 == 4.0 { 1.0 } else { 0.0 };
        var_guard262 = assign15350_e13734;

        let (assign15360_e13764, assign15360_e13764_d_n6, assign15360_e13764_d_n7, assign15360_e13764_d_n8, assign15360_e13764_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard260 == 0.0)) && (var_guard261 != 0.0)) && (var_guard262 != 0.0)) {
        let assign15360_e13750: f64 = (var_vav * var_vbrinvbot);
        let assign15360_e13753: f64 = (var_vav * var_vbrinvbot);
        let assign15360_e13754: f64 = (assign15360_e13750 * assign15360_e13753);
        let assign15360_e13757: f64 = (var_vav * var_vbrinvbot);
        let assign15360_e13758: f64 = (assign15360_e13754 * assign15360_e13757);
        let assign15360_e13761: f64 = (var_vav * var_vbrinvbot);
        let assign15360_e13762: f64 = (assign15360_e13758 * assign15360_e13761);
        (assign15360_e13762, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15360_e13764;
        var_tmp_dn6 = assign15360_e13764_d_n6;
        var_tmp_dn7 = assign15360_e13764_d_n7;
        var_tmp_dn8 = assign15360_e13764_d_n8;
        var_tmp_dn9 = assign15360_e13764_d_n9;

        let (assign15370_e13786, assign15370_e13786_d_n6, assign15370_e13786_d_n7, assign15370_e13786_d_n8, assign15370_e13786_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard260 == 0.0)) && (var_guard261 != 0.0)) && (var_guard262 == 0.0)) {
        let assign15370_e13781: f64 = (var_vav * var_vbrinvbot);
        let assign15370_e13782: f64 = (assign15370_e13781).abs();
        let assign15370_e13784: f64 = (assign15370_e13782).powf(p.p880);
        (assign15370_e13784, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15370_e13786;
        var_tmp_dn6 = assign15370_e13786_d_n6;
        var_tmp_dn7 = assign15370_e13786_d_n7;
        var_tmp_dn8 = assign15370_e13786_d_n8;
        var_tmp_dn9 = assign15370_e13786_d_n9;

        let (assign15380_e13804, assign15380_e13804_d_n6, assign15380_e13804_d_n7, assign15380_e13804_d_n8, assign15380_e13804_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard260 == 0.0)) && (var_guard261 != 0.0)) {
        let assign15380_e13801: f64 = (1.0 - var_tmp);
        let assign15380_e13802: f64 = (1.0 / assign15380_e13801);
        (assign15380_e13802, (-((-var_tmp_dn6) / (assign15380_e13801 * assign15380_e13801))), (-((-var_tmp_dn7) / (assign15380_e13801 * assign15380_e13801))), (-((-var_tmp_dn8) / (assign15380_e13801 * assign15380_e13801))), (-((-var_tmp_dn9) / (assign15380_e13801 * assign15380_e13801))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign15380_e13804;
        var_fbreakdown_dn6 = assign15380_e13804_d_n6;
        var_fbreakdown_dn7 = assign15380_e13804_d_n7;
        var_fbreakdown_dn8 = assign15380_e13804_d_n8;
        var_fbreakdown_dn9 = assign15380_e13804_d_n9;

        let (assign15390_e13827, assign15390_e13827_d_n6, assign15390_e13827_d_n7, assign15390_e13827_d_n8, assign15390_e13827_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) && (var_guard260 == 0.0)) && (var_guard261 == 0.0)) {
        let assign15390_e13821: f64 = (var_alphaav * p.p877);
        let assign15390_e13822: f64 = (var_vav + assign15390_e13821);
        let assign15390_e13824: f64 = (assign15390_e13822 * var_slopebot);
        let assign15390_e13825: f64 = (var_fstopbot + assign15390_e13824);
        (assign15390_e13825, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign15390_e13827;
        var_fbreakdown_dn6 = assign15390_e13827_d_n6;
        var_fbreakdown_dn7 = assign15390_e13827_d_n7;
        var_fbreakdown_dn8 = assign15390_e13827_d_n8;
        var_fbreakdown_dn9 = assign15390_e13827_d_n9;

        let (assign15400_e13846, assign15400_e13846_d_n6, assign15400_e13846_d_n7, assign15400_e13846_d_n8, assign15400_e13846_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard246 == 0.0)) {
        let assign15400_e13837: f64 = (var_id__blk212 + var_isrh);
        let assign15400_e13839: f64 = (assign15400_e13837 + var_itat);
        let assign15400_e13841: f64 = (assign15400_e13839 + var_ibbt);
        let assign15400_e13842: f64 = (p.p29 * assign15400_e13841);
        let assign15400_e13844: f64 = (assign15400_e13842 * var_fbreakdown);
        (assign15400_e13844, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign15400_e13842 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign15400_e13842 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign15400_e13842 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign15400_e13842 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign15400_e13846;
        var_ijunbot_dn6 = assign15400_e13846_d_n6;
        var_ijunbot_dn7 = assign15400_e13846_d_n7;
        var_ijunbot_dn8 = assign15400_e13846_d_n8;
        var_ijunbot_dn9 = assign15400_e13846_d_n9;

        let assign15410_e13849: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard263 = assign15410_e13849;

        let (assign15420_e13857, assign15420_e13857_d_n6, assign15420_e13857_d_n7, assign15420_e13857_d_n8, assign15420_e13857_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign15420_e13857;
        var_ijunsti_dn6 = assign15420_e13857_d_n6;
        var_ijunsti_dn7 = assign15420_e13857_d_n7;
        var_ijunsti_dn8 = assign15420_e13857_d_n8;
        var_ijunsti_dn9 = assign15420_e13857_d_n9;

        let (assign15430_e13868,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) {
        let assign15430_e13866: f64 = (var_idsatsti * var_idmult);
        (assign15430_e13866,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign15430_e13868;

        let assign15440_e13875: f64 = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };
        var_guard264 = assign15440_e13875;

        let (assign15450_e13886, assign15450_e13886_d_n6, assign15450_e13886_d_n7, assign15450_e13886_d_n8, assign15450_e13886_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign15450_e13886;
        var_isrh_dn6 = assign15450_e13886_d_n6;
        var_isrh_dn7 = assign15450_e13886_d_n7;
        var_isrh_dn8 = assign15450_e13886_d_n8;
        var_isrh_dn9 = assign15450_e13886_d_n9;

        let (assign15460_e13900,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign15460_e13898: f64 = (var_vbisti - var_vjsrh);
        (assign15460_e13898,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign15460_e13900;

        let (assign15470_e13919,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign15470_e13914: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign15470_e13915: f64 = (1.0 - assign15470_e13914);
        let assign15470_e13916: f64 = (assign15470_e13915).sqrt();
        let assign15470_e13917: f64 = (1.0 - assign15470_e13916);
        (assign15470_e13917,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign15470_e13919;

        let assign15480_e13922: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard265 = assign15480_e13922;

        let (assign15490_e13936,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) && (var_guard265 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign15490_e13936;

        let (assign15500_e13968,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) && (var_guard265 == 0.0)) {
        let assign15500_e13951: f64 = (var_wsrhstep * var_wsrhstep);
        let assign15500_e13953: f64 = (var_wsrhstep).ln();
        let assign15500_e13954: f64 = (assign15500_e13951 * assign15500_e13953);
        let assign15500_e13957: f64 = (1.0 - var_wsrhstep);
        let assign15500_e13958: f64 = (assign15500_e13954 / assign15500_e13957);
        let assign15500_e13960: f64 = (assign15500_e13958 + var_wsrhstep);
        let assign15500_e13964: f64 = (2.0 * p.p849);
        let assign15500_e13965: f64 = (1.0 - assign15500_e13964);
        let assign15500_e13966: f64 = (assign15500_e13960 * assign15500_e13965);
        (assign15500_e13966,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign15500_e13968;

        let (assign15510_e13982,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign15510_e13980: f64 = (var_wsrhstep + var_dwsrh);
        (assign15510_e13980,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign15510_e13982;

        let assign15520_e13985: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard266 = assign15520_e13985;

        let (assign15530_e14002, assign15530_e14002_d_n6, assign15530_e14002_d_n7, assign15530_e14002_d_n8, assign15530_e14002_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) && (var_guard266 != 0.0)) {
        let assign15530_e13999: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign15530_e14000: f64 = (assign15530_e13999).sqrt();
        (assign15530_e14000, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15530_e14002;
        var_tmp_dn6 = assign15530_e14002_d_n6;
        var_tmp_dn7 = assign15530_e14002_d_n7;
        var_tmp_dn8 = assign15530_e14002_d_n8;
        var_tmp_dn9 = assign15530_e14002_d_n9;

        let (assign15540_e14021, assign15540_e14021_d_n6, assign15540_e14021_d_n7, assign15540_e14021_d_n8, assign15540_e14021_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) && (var_guard266 == 0.0)) {
        let assign15540_e14017: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign15540_e14019: f64 = (assign15540_e14017).powf(p.p849);
        (assign15540_e14019, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15540_e14021;
        var_tmp_dn6 = assign15540_e14021_d_n6;
        var_tmp_dn7 = assign15540_e14021_d_n7;
        var_tmp_dn8 = assign15540_e14021_d_n8;
        var_tmp_dn9 = assign15540_e14021_d_n9;

        let (assign15550_e14035, assign15550_e14035_d_n6, assign15550_e14035_d_n7, assign15550_e14035_d_n8, assign15550_e14035_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign15550_e14033: f64 = (var_wdepnulrsti * var_tmp);
        (assign15550_e14033, (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8), (var_wdepnulrsti * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign15550_e14035;
        var_wdep_dn6 = assign15550_e14035_d_n6;
        var_wdep_dn7 = assign15550_e14035_d_n7;
        var_wdep_dn8 = assign15550_e14035_d_n8;
        var_wdep_dn9 = assign15550_e14035_d_n9;

        let (assign15560_e14053, assign15560_e14053_d_n6, assign15560_e14053_d_n7, assign15560_e14053_d_n8, assign15560_e14053_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign15560_e14048: f64 = (var_zinv - 1.0);
        let assign15560_e14050: f64 = (assign15560_e14048 * var_wdep);
        let assign15560_e14051: f64 = (var_ftdsti * assign15560_e14050);
        (assign15560_e14051, (var_ftdsti * (assign15560_e14048 * var_wdep_dn6)), (var_ftdsti * (assign15560_e14048 * var_wdep_dn7)), (var_ftdsti * (assign15560_e14048 * var_wdep_dn8)), (var_ftdsti * (assign15560_e14048 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign15560_e14053;
        var_asrh_dn6 = assign15560_e14053_d_n6;
        var_asrh_dn7 = assign15560_e14053_d_n7;
        var_asrh_dn8 = assign15560_e14053_d_n8;
        var_asrh_dn9 = assign15560_e14053_d_n9;

        let (assign15570_e14069, assign15570_e14069_d_n6, assign15570_e14069_d_n7, assign15570_e14069_d_n8, assign15570_e14069_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard264 == 0.0)) {
        let assign15570_e14066: f64 = (var_asrh * var_wsrh);
        let assign15570_e14067: f64 = (p.p858 * assign15570_e14066);
        (assign15570_e14067, (p.p858 * (var_asrh_dn6 * var_wsrh)), (p.p858 * (var_asrh_dn7 * var_wsrh)), (p.p858 * (var_asrh_dn8 * var_wsrh)), (p.p858 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign15570_e14069;
        var_isrh_dn6 = assign15570_e14069_d_n6;
        var_isrh_dn7 = assign15570_e14069_d_n7;
        var_isrh_dn8 = assign15570_e14069_d_n8;
        var_isrh_dn9 = assign15570_e14069_d_n9;

        let assign15580_e14072: f64 = if p.p863 == 0.0 { 1.0 } else { 0.0 };
        var_guard267 = assign15580_e14072;

        let (assign15590_e14083, assign15590_e14083_d_n6, assign15590_e14083_d_n7, assign15590_e14083_d_n8, assign15590_e14083_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign15590_e14083;
        var_itat_dn6 = assign15590_e14083_d_n6;
        var_itat_dn7 = assign15590_e14083_d_n7;
        var_itat_dn8 = assign15590_e14083_d_n8;
        var_itat_dn9 = assign15590_e14083_d_n9;

        let (assign15600_e14101, assign15600_e14101_d_n6, assign15600_e14101_d_n7, assign15600_e14101_d_n8, assign15600_e14101_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15600_e14096: f64 = (var_wdep * var_one_minus_psti);
        let assign15600_e14098: f64 = (assign15600_e14096 / var_vbi_minus_vjsrh);
        let assign15600_e14099: f64 = (var_btatpartsti * assign15600_e14098);
        (assign15600_e14099, (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn9 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign15600_e14101;
        var_btat_dn6 = assign15600_e14101_d_n6;
        var_btat_dn7 = assign15600_e14101_d_n7;
        var_btat_dn8 = assign15600_e14101_d_n8;
        var_btat_dn9 = assign15600_e14101_d_n9;

        let (assign15610_e14117, assign15610_e14117_d_n6, assign15610_e14117_d_n7, assign15610_e14117_d_n8, assign15610_e14117_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15610_e14113: f64 = (0.666666666666667 * var_atatsti);
        let assign15610_e14115: f64 = (assign15610_e14113 / var_btat);
        (assign15610_e14115, (-((assign15610_e14113 * var_btat_dn6) / (var_btat * var_btat))), (-((assign15610_e14113 * var_btat_dn7) / (var_btat * var_btat))), (-((assign15610_e14113 * var_btat_dn8) / (var_btat * var_btat))), (-((assign15610_e14113 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign15610_e14117;
        var_twoatatoverthreebtat_dn6 = assign15610_e14117_d_n6;
        var_twoatatoverthreebtat_dn7 = assign15610_e14117_d_n7;
        var_twoatatoverthreebtat_dn8 = assign15610_e14117_d_n8;
        var_twoatatoverthreebtat_dn9 = assign15610_e14117_d_n9;

        let (assign15620_e14131, assign15620_e14131_d_n6, assign15620_e14131_d_n7, assign15620_e14131_d_n8, assign15620_e14131_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15620_e14129: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign15620_e14129, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign15620_e14131;
        var_umaxbeforelimiting_dn6 = assign15620_e14131_d_n6;
        var_umaxbeforelimiting_dn7 = assign15620_e14131_d_n7;
        var_umaxbeforelimiting_dn8 = assign15620_e14131_d_n8;
        var_umaxbeforelimiting_dn9 = assign15620_e14131_d_n9;

        let (assign15630_e14152, assign15630_e14152_d_n6, assign15630_e14152_d_n7, assign15630_e14152_d_n8, assign15630_e14152_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15630_e14143: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign15630_e14146: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign15630_e14148: f64 = (assign15630_e14146 + 1.0);
        let assign15630_e14149: f64 = (assign15630_e14143 / assign15630_e14148);
        let assign15630_e14150: f64 = (assign15630_e14149).sqrt();
        (assign15630_e14150, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign15630_e14148) - (assign15630_e14143 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign15630_e14148 * assign15630_e14148)) / (2.0 * assign15630_e14150)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign15630_e14148) - (assign15630_e14143 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign15630_e14148 * assign15630_e14148)) / (2.0 * assign15630_e14150)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign15630_e14148) - (assign15630_e14143 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign15630_e14148 * assign15630_e14148)) / (2.0 * assign15630_e14150)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign15630_e14148) - (assign15630_e14143 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign15630_e14148 * assign15630_e14148)) / (2.0 * assign15630_e14150)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign15630_e14152;
        var_umax_dn6 = assign15630_e14152_d_n6;
        var_umax_dn7 = assign15630_e14152_d_n7;
        var_umax_dn8 = assign15630_e14152_d_n8;
        var_umax_dn9 = assign15630_e14152_d_n9;

        let (assign15640_e14165, assign15640_e14165_d_n6, assign15640_e14165_d_n7, assign15640_e14165_d_n8, assign15640_e14165_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15640_e14163: f64 = (var_umax).sqrt();
        (assign15640_e14163, (var_umax_dn6 / (2.0 * assign15640_e14163)), (var_umax_dn7 / (2.0 * assign15640_e14163)), (var_umax_dn8 / (2.0 * assign15640_e14163)), (var_umax_dn9 / (2.0 * assign15640_e14163)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign15640_e14165;
        var_sqrtumax_dn6 = assign15640_e14165_d_n6;
        var_sqrtumax_dn7 = assign15640_e14165_d_n7;
        var_sqrtumax_dn8 = assign15640_e14165_d_n8;
        var_sqrtumax_dn9 = assign15640_e14165_d_n9;

        let (assign15650_e14179, assign15650_e14179_d_n6, assign15650_e14179_d_n7, assign15650_e14179_d_n8, assign15650_e14179_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15650_e14177: f64 = (var_umax * var_sqrtumax);
        (assign15650_e14177, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign15650_e14179;
        var_umaxpoweronepointfive_dn6 = assign15650_e14179_d_n6;
        var_umaxpoweronepointfive_dn7 = assign15650_e14179_d_n7;
        var_umaxpoweronepointfive_dn8 = assign15650_e14179_d_n8;
        var_umaxpoweronepointfive_dn9 = assign15650_e14179_d_n9;

        let assign15660_e14181: f64 = (-p.p849);
        let assign15660_e14183: f64 = (assign15660_e14181 * var_one_over_one_minus_psti);
        let assign15660_e14185: f64 = (-1.0);
        let assign15660_e14186: f64 = if assign15660_e14183 == assign15660_e14185 { 1.0 } else { 0.0 };
        var_guard268 = assign15660_e14186;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard259_slot = var_guard259;
        *var_guard260_slot = var_guard260;
        *var_guard261_slot = var_guard261;
        *var_guard262_slot = var_guard262;
        *var_guard263_slot = var_guard263;
        *var_guard264_slot = var_guard264;
        *var_guard265_slot = var_guard265;
        *var_guard266_slot = var_guard266;
        *var_guard267_slot = var_guard267;
        *var_guard268_slot = var_guard268;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_24(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard263: f64,
        var_guard267: f64,
        var_guard268: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_sqrtumax: f64,
        var_sqrtumax_dn6: f64,
        var_sqrtumax_dn7: f64,
        var_sqrtumax_dn8: f64,
        var_sqrtumax_dn9: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_twoatatoverthreebtat_dn9: f64,
        var_umax: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_umax_dn9: f64,
        var_umaxpoweronepointfive: f64,
        var_umaxpoweronepointfive_dn6: f64,
        var_umaxpoweronepointfive_dn7: f64,
        var_umaxpoweronepointfive_dn8: f64,
        var_umaxpoweronepointfive_dn9: f64,
        var_vbbt: f64,
        var_vbirstiinv: f64,
        var_wdepnulrinvsti: f64,
        var_wsrh: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard269_slot: &mut f64,
        var_guard270_slot: &mut f64,
        var_guard271_slot: &mut f64,
        var_guard272_slot: &mut f64,
        var_guard273_slot: &mut f64,
        var_guard274_slot: &mut f64,
        var_guard275_slot: &mut f64,
        var_guard276_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard269: f64 = *var_guard269_slot;
        let mut var_guard270: f64 = *var_guard270_slot;
        let mut var_guard271: f64 = *var_guard271_slot;
        let mut var_guard272: f64 = *var_guard272_slot;
        let mut var_guard273: f64 = *var_guard273_slot;
        let mut var_guard274: f64 = *var_guard274_slot;
        let mut var_guard275: f64 = *var_guard275_slot;
        let mut var_guard276: f64 = *var_guard276_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign15670_e14206, assign15670_e14206_d_n6, assign15670_e14206_d_n7, assign15670_e14206_d_n8, assign15670_e14206_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard268 != 0.0)) {
        let assign15670_e14202: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign15670_e14203: f64 = (1.0 + assign15670_e14202);
        let assign15670_e14204: f64 = (1.0 / assign15670_e14203);
        (assign15670_e14204, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign15670_e14203 * assign15670_e14203))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign15670_e14203 * assign15670_e14203))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign15670_e14203 * assign15670_e14203))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign15670_e14203 * assign15670_e14203))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign15670_e14206;
        var_wgamma_dn6 = assign15670_e14206_d_n6;
        var_wgamma_dn7 = assign15670_e14206_d_n7;
        var_wgamma_dn8 = assign15670_e14206_d_n8;
        var_wgamma_dn9 = assign15670_e14206_d_n9;

        let (assign15680_e14230, assign15680_e14230_d_n6, assign15680_e14230_d_n7, assign15680_e14230_d_n8, assign15680_e14230_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard268 == 0.0)) {
        let assign15680_e14222: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign15680_e14223: f64 = (1.0 + assign15680_e14222);
        let assign15680_e14225: f64 = (-p.p849);
        let assign15680_e14227: f64 = (assign15680_e14225 * var_one_over_one_minus_psti);
        let assign15680_e14228: f64 = (assign15680_e14223).powf(assign15680_e14227);
        (assign15680_e14228, if 0.0 == 0.0 && ((assign15680_e14227) as f64).is_finite() && ((assign15680_e14227) as f64).fract() == 0.0 { if assign15680_e14227 == 0.0 { 0.0 } else { (assign15680_e14227 * ((assign15680_e14223).powf(assign15680_e14227 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign15680_e14228 * (assign15680_e14227 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign15680_e14223))) }, if 0.0 == 0.0 && ((assign15680_e14227) as f64).is_finite() && ((assign15680_e14227) as f64).fract() == 0.0 { if assign15680_e14227 == 0.0 { 0.0 } else { (assign15680_e14227 * ((assign15680_e14223).powf(assign15680_e14227 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign15680_e14228 * (assign15680_e14227 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign15680_e14223))) }, if 0.0 == 0.0 && ((assign15680_e14227) as f64).is_finite() && ((assign15680_e14227) as f64).fract() == 0.0 { if assign15680_e14227 == 0.0 { 0.0 } else { (assign15680_e14227 * ((assign15680_e14223).powf(assign15680_e14227 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign15680_e14228 * (assign15680_e14227 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign15680_e14223))) }, if 0.0 == 0.0 && ((assign15680_e14227) as f64).is_finite() && ((assign15680_e14227) as f64).fract() == 0.0 { if assign15680_e14227 == 0.0 { 0.0 } else { (assign15680_e14227 * ((assign15680_e14223).powf(assign15680_e14227 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign15680_e14228 * (assign15680_e14227 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign15680_e14223))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign15680_e14230;
        var_wgamma_dn6 = assign15680_e14230_d_n6;
        var_wgamma_dn7 = assign15680_e14230_d_n7;
        var_wgamma_dn8 = assign15680_e14230_d_n8;
        var_wgamma_dn9 = assign15680_e14230_d_n9;

        let (assign15690_e14248, assign15690_e14248_d_n6, assign15690_e14248_d_n7, assign15690_e14248_d_n8, assign15690_e14248_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15690_e14242: f64 = (var_wsrh * var_wgamma);
        let assign15690_e14245: f64 = (var_wsrh + var_wgamma);
        let assign15690_e14246: f64 = (assign15690_e14242 / assign15690_e14245);
        (assign15690_e14246, ((((var_wsrh * var_wgamma_dn6) * assign15690_e14245) - (assign15690_e14242 * var_wgamma_dn6)) / (assign15690_e14245 * assign15690_e14245)), ((((var_wsrh * var_wgamma_dn7) * assign15690_e14245) - (assign15690_e14242 * var_wgamma_dn7)) / (assign15690_e14245 * assign15690_e14245)), ((((var_wsrh * var_wgamma_dn8) * assign15690_e14245) - (assign15690_e14242 * var_wgamma_dn8)) / (assign15690_e14245 * assign15690_e14245)), ((((var_wsrh * var_wgamma_dn9) * assign15690_e14245) - (assign15690_e14242 * var_wgamma_dn9)) / (assign15690_e14245 * assign15690_e14245)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign15690_e14248;
        var_wtat_dn6 = assign15690_e14248_d_n6;
        var_wtat_dn7 = assign15690_e14248_d_n7;
        var_wtat_dn8 = assign15690_e14248_d_n8;
        var_wtat_dn9 = assign15690_e14248_d_n9;

        let (assign15700_e14265, assign15700_e14265_d_n6, assign15700_e14265_d_n7, assign15700_e14265_d_n8, assign15700_e14265_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15700_e14261: f64 = (var_btat / var_sqrtumax);
        let assign15700_e14262: f64 = (0.375 * assign15700_e14261);
        let assign15700_e14263: f64 = (assign15700_e14262).sqrt();
        (assign15700_e14263, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15700_e14263)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15700_e14263)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15700_e14263)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign15700_e14263)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign15700_e14265;
        var_ktat_dn6 = assign15700_e14265_d_n6;
        var_ktat_dn7 = assign15700_e14265_d_n7;
        var_ktat_dn8 = assign15700_e14265_d_n8;
        var_ktat_dn9 = assign15700_e14265_d_n9;

        let (assign15710_e14283, assign15710_e14283_d_n6, assign15710_e14283_d_n7, assign15710_e14283_d_n8, assign15710_e14283_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15710_e14278: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign15710_e14279: f64 = (2.0 * assign15710_e14278);
        let assign15710_e14281: f64 = (assign15710_e14279 - var_umax);
        (assign15710_e14281, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign15710_e14283;
        var_ltat_dn6 = assign15710_e14283_d_n6;
        var_ltat_dn7 = assign15710_e14283_d_n7;
        var_ltat_dn8 = assign15710_e14283_d_n8;
        var_ltat_dn9 = assign15710_e14283_d_n9;

        let (assign15720_e14309, assign15720_e14309_d_n6, assign15720_e14309_d_n7, assign15720_e14309_d_n8, assign15720_e14309_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15720_e14295: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign15720_e14297: f64 = (assign15720_e14295 * var_sqrtumax);
        let assign15720_e14300: f64 = (var_atatsti * var_umax);
        let assign15720_e14301: f64 = (assign15720_e14297 - assign15720_e14300);
        let assign15720_e14305: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign15720_e14306: f64 = (0.5 * assign15720_e14305);
        let assign15720_e14307: f64 = (assign15720_e14301 + assign15720_e14306);
        (assign15720_e14307, (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign15720_e14295 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign15720_e14295 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign15720_e14295 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign15720_e14295 * var_sqrtumax_dn9)) - (var_atatsti * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign15720_e14309;
        var_mtat_dn6 = assign15720_e14309_d_n6;
        var_mtat_dn7 = assign15720_e14309_d_n7;
        var_mtat_dn8 = assign15720_e14309_d_n8;
        var_mtat_dn9 = assign15720_e14309_d_n9;

        let (assign15730_e14325, assign15730_e14325_d_n6, assign15730_e14325_d_n7, assign15730_e14325_d_n8, assign15730_e14325_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15730_e14321: f64 = (var_ltat - 1.0);
        let assign15730_e14323: f64 = (assign15730_e14321 * var_ktat);
        (assign15730_e14323, ((var_ltat_dn6 * var_ktat) + (assign15730_e14321 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign15730_e14321 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign15730_e14321 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign15730_e14321 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign15730_e14325;
        var_xerfc_dn6 = assign15730_e14325_d_n6;
        var_xerfc_dn7 = assign15730_e14325_d_n7;
        var_xerfc_dn8 = assign15730_e14325_d_n8;
        var_xerfc_dn9 = assign15730_e14325_d_n9;

        let (assign15740_e14339, assign15740_e14339_d_n6, assign15740_e14339_d_n7, assign15740_e14339_d_n8, assign15740_e14339_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15740_e14337: f64 = (var_xerfc * var_xerfc);
        (assign15740_e14337, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign15740_e14339;
        var_ysq_dn6 = assign15740_e14339_d_n6;
        var_ysq_dn7 = assign15740_e14339_d_n7;
        var_ysq_dn8 = assign15740_e14339_d_n8;
        var_ysq_dn9 = assign15740_e14339_d_n9;

        let assign15750_e14342: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard269 = assign15750_e14342;

        let (assign15760_e14362, assign15760_e14362_d_n6, assign15760_e14362_d_n7, assign15760_e14362_d_n8, assign15760_e14362_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard269 != 0.0)) {
        let assign15760_e14358: f64 = (var_perfc * var_xerfc);
        let assign15760_e14359: f64 = (1.0 + assign15760_e14358);
        let assign15760_e14360: f64 = (1.0 / assign15760_e14359);
        (assign15760_e14360, (-((var_perfc * var_xerfc_dn6) / (assign15760_e14359 * assign15760_e14359))), (-((var_perfc * var_xerfc_dn7) / (assign15760_e14359 * assign15760_e14359))), (-((var_perfc * var_xerfc_dn8) / (assign15760_e14359 * assign15760_e14359))), (-((var_perfc * var_xerfc_dn9) / (assign15760_e14359 * assign15760_e14359))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign15760_e14362;
        var_terfc_dn6 = assign15760_e14362_d_n6;
        var_terfc_dn7 = assign15760_e14362_d_n7;
        var_terfc_dn8 = assign15760_e14362_d_n8;
        var_terfc_dn9 = assign15760_e14362_d_n9;

        let (assign15770_e14383, assign15770_e14383_d_n6, assign15770_e14383_d_n7, assign15770_e14383_d_n8, assign15770_e14383_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard269 == 0.0)) {
        let assign15770_e14379: f64 = (var_perfc * var_xerfc);
        let assign15770_e14380: f64 = (1.0 - assign15770_e14379);
        let assign15770_e14381: f64 = (1.0 / assign15770_e14380);
        (assign15770_e14381, (-((-(var_perfc * var_xerfc_dn6)) / (assign15770_e14380 * assign15770_e14380))), (-((-(var_perfc * var_xerfc_dn7)) / (assign15770_e14380 * assign15770_e14380))), (-((-(var_perfc * var_xerfc_dn8)) / (assign15770_e14380 * assign15770_e14380))), (-((-(var_perfc * var_xerfc_dn9)) / (assign15770_e14380 * assign15770_e14380))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign15770_e14383;
        var_terfc_dn6 = assign15770_e14383_d_n6;
        var_terfc_dn7 = assign15770_e14383_d_n7;
        var_terfc_dn8 = assign15770_e14383_d_n8;
        var_terfc_dn9 = assign15770_e14383_d_n9;

        let assign15780_e14385: f64 = (-var_ysq);
        let assign15780_e14387: f64 = (assign15780_e14385 + var_mtat);
        let assign15780_e14389: f64 = (-230.25850929940458);
        let assign15780_e14390: f64 = if assign15780_e14387 > assign15780_e14389 { 1.0 } else { 0.0 };
        var_guard270 = assign15780_e14390;

        let (assign15790_e14408, assign15790_e14408_d_n6, assign15790_e14408_d_n7, assign15790_e14408_d_n8, assign15790_e14408_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard270 != 0.0)) {
        let assign15790_e14403: f64 = (-var_ysq);
        let assign15790_e14405: f64 = (assign15790_e14403 + var_mtat);
        let assign15790_e14406: f64 = (assign15790_e14405).exp();
        (assign15790_e14406, (assign15790_e14406 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign15790_e14406 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign15790_e14406 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign15790_e14406 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15790_e14408;
        var_tmp_dn6 = assign15790_e14408_d_n6;
        var_tmp_dn7 = assign15790_e14408_d_n7;
        var_tmp_dn8 = assign15790_e14408_d_n8;
        var_tmp_dn9 = assign15790_e14408_d_n9;

        let (assign15800_e14457, assign15800_e14457_d_n6, assign15800_e14457_d_n7, assign15800_e14457_d_n8, assign15800_e14457_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard270 == 0.0)) {
        let assign15800_e14424: f64 = (-230.25850929940458);
        let assign15800_e14426: f64 = (-var_ysq);
        let assign15800_e14428: f64 = (assign15800_e14426 + var_mtat);
        let assign15800_e14429: f64 = (assign15800_e14424 - assign15800_e14428);
        let assign15800_e14433: f64 = (-230.25850929940458);
        let assign15800_e14435: f64 = (-var_ysq);
        let assign15800_e14437: f64 = (assign15800_e14435 + var_mtat);
        let assign15800_e14438: f64 = (assign15800_e14433 - assign15800_e14437);
        let assign15800_e14441: f64 = (-230.25850929940458);
        let assign15800_e14443: f64 = (-var_ysq);
        let assign15800_e14445: f64 = (assign15800_e14443 + var_mtat);
        let assign15800_e14446: f64 = (assign15800_e14441 - assign15800_e14445);
        let assign15800_e14448: f64 = (assign15800_e14446 * 0.3333333333333333);
        let assign15800_e14449: f64 = (1.0 + assign15800_e14448);
        let assign15800_e14450: f64 = (assign15800_e14438 * assign15800_e14449);
        let assign15800_e14451: f64 = (0.5 * assign15800_e14450);
        let assign15800_e14452: f64 = (1.0 + assign15800_e14451);
        let assign15800_e14453: f64 = (assign15800_e14429 * assign15800_e14452);
        let assign15800_e14454: f64 = (1.0 + assign15800_e14453);
        let assign15800_e14455: f64 = (1e-100 / assign15800_e14454);
        (assign15800_e14455, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign15800_e14452) + (assign15800_e14429 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign15800_e14449) + (assign15800_e14438 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign15800_e14454 * assign15800_e14454))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign15800_e14452) + (assign15800_e14429 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign15800_e14449) + (assign15800_e14438 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign15800_e14454 * assign15800_e14454))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign15800_e14452) + (assign15800_e14429 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign15800_e14449) + (assign15800_e14438 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign15800_e14454 * assign15800_e14454))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign15800_e14452) + (assign15800_e14429 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign15800_e14449) + (assign15800_e14438 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign15800_e14454 * assign15800_e14454))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15800_e14457;
        var_tmp_dn6 = assign15800_e14457_d_n6;
        var_tmp_dn7 = assign15800_e14457_d_n7;
        var_tmp_dn8 = assign15800_e14457_d_n8;
        var_tmp_dn9 = assign15800_e14457_d_n9;

        let (assign15810_e14487, assign15810_e14487_d_n6, assign15810_e14487_d_n7, assign15810_e14487_d_n8, assign15810_e14487_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15810_e14469: f64 = (0.29214664 * var_terfc);
        let assign15810_e14473: f64 = (var_terfc * var_terfc);
        let assign15810_e14474: f64 = (var_berfc * assign15810_e14473);
        let assign15810_e14475: f64 = (assign15810_e14469 + assign15810_e14474);
        let assign15810_e14479: f64 = (var_terfc * var_terfc);
        let assign15810_e14481: f64 = (assign15810_e14479 * var_terfc);
        let assign15810_e14482: f64 = (var_cerfc * assign15810_e14481);
        let assign15810_e14483: f64 = (assign15810_e14475 + assign15810_e14482);
        let assign15810_e14485: f64 = (assign15810_e14483 * var_tmp);
        (assign15810_e14485, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign15810_e14479 * var_terfc_dn6)))) * var_tmp) + (assign15810_e14483 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign15810_e14479 * var_terfc_dn7)))) * var_tmp) + (assign15810_e14483 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign15810_e14479 * var_terfc_dn8)))) * var_tmp) + (assign15810_e14483 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign15810_e14479 * var_terfc_dn9)))) * var_tmp) + (assign15810_e14483 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign15810_e14487;
        var_erfcpos_dn6 = assign15810_e14487_d_n6;
        var_erfcpos_dn7 = assign15810_e14487_d_n7;
        var_erfcpos_dn8 = assign15810_e14487_d_n8;
        var_erfcpos_dn9 = assign15810_e14487_d_n9;

        let assign15820_e14490: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard271 = assign15820_e14490;

        let (assign15830_e14504, assign15830_e14504_d_n6, assign15830_e14504_d_n7, assign15830_e14504_d_n8, assign15830_e14504_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard271 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign15830_e14504;
        var_erfctimesexpmtat_dn6 = assign15830_e14504_d_n6;
        var_erfctimesexpmtat_dn7 = assign15830_e14504_d_n7;
        var_erfctimesexpmtat_dn8 = assign15830_e14504_d_n8;
        var_erfctimesexpmtat_dn9 = assign15830_e14504_d_n9;

        let assign15840_e14507: f64 = (-230.25850929940458);
        let assign15840_e14508: f64 = if var_mtat > assign15840_e14507 { 1.0 } else { 0.0 };
        var_guard272 = assign15840_e14508;

        let (assign15850_e14526, assign15850_e14526_d_n6, assign15850_e14526_d_n7, assign15850_e14526_d_n8, assign15850_e14526_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard271 == 0.0)) && (var_guard272 != 0.0)) {
        let assign15850_e14524: f64 = (var_mtat).exp();
        (assign15850_e14524, (assign15850_e14524 * var_mtat_dn6), (assign15850_e14524 * var_mtat_dn7), (assign15850_e14524 * var_mtat_dn8), (assign15850_e14524 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15850_e14526;
        var_tmp_dn6 = assign15850_e14526_d_n6;
        var_tmp_dn7 = assign15850_e14526_d_n7;
        var_tmp_dn8 = assign15850_e14526_d_n8;
        var_tmp_dn9 = assign15850_e14526_d_n9;

        let (assign15860_e14569, assign15860_e14569_d_n6, assign15860_e14569_d_n7, assign15860_e14569_d_n8, assign15860_e14569_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard271 == 0.0)) && (var_guard272 == 0.0)) {
        let assign15860_e14545: f64 = (-230.25850929940458);
        let assign15860_e14547: f64 = (assign15860_e14545 - var_mtat);
        let assign15860_e14551: f64 = (-230.25850929940458);
        let assign15860_e14553: f64 = (assign15860_e14551 - var_mtat);
        let assign15860_e14556: f64 = (-230.25850929940458);
        let assign15860_e14558: f64 = (assign15860_e14556 - var_mtat);
        let assign15860_e14560: f64 = (assign15860_e14558 * 0.3333333333333333);
        let assign15860_e14561: f64 = (1.0 + assign15860_e14560);
        let assign15860_e14562: f64 = (assign15860_e14553 * assign15860_e14561);
        let assign15860_e14563: f64 = (0.5 * assign15860_e14562);
        let assign15860_e14564: f64 = (1.0 + assign15860_e14563);
        let assign15860_e14565: f64 = (assign15860_e14547 * assign15860_e14564);
        let assign15860_e14566: f64 = (1.0 + assign15860_e14565);
        let assign15860_e14567: f64 = (1e-100 / assign15860_e14566);
        (assign15860_e14567, (-((1e-100 * (((-var_mtat_dn6) * assign15860_e14564) + (assign15860_e14547 * (0.5 * (((-var_mtat_dn6) * assign15860_e14561) + (assign15860_e14553 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign15860_e14566 * assign15860_e14566))), (-((1e-100 * (((-var_mtat_dn7) * assign15860_e14564) + (assign15860_e14547 * (0.5 * (((-var_mtat_dn7) * assign15860_e14561) + (assign15860_e14553 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign15860_e14566 * assign15860_e14566))), (-((1e-100 * (((-var_mtat_dn8) * assign15860_e14564) + (assign15860_e14547 * (0.5 * (((-var_mtat_dn8) * assign15860_e14561) + (assign15860_e14553 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign15860_e14566 * assign15860_e14566))), (-((1e-100 * (((-var_mtat_dn9) * assign15860_e14564) + (assign15860_e14547 * (0.5 * (((-var_mtat_dn9) * assign15860_e14561) + (assign15860_e14553 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign15860_e14566 * assign15860_e14566))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15860_e14569;
        var_tmp_dn6 = assign15860_e14569_d_n6;
        var_tmp_dn7 = assign15860_e14569_d_n7;
        var_tmp_dn8 = assign15860_e14569_d_n8;
        var_tmp_dn9 = assign15860_e14569_d_n9;

        let (assign15870_e14588, assign15870_e14588_d_n6, assign15870_e14588_d_n7, assign15870_e14588_d_n8, assign15870_e14588_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) && (var_guard271 == 0.0)) {
        let assign15870_e14584: f64 = (2.0 * var_tmp);
        let assign15870_e14586: f64 = (assign15870_e14584 - var_erfcpos);
        (assign15870_e14586, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign15870_e14588;
        var_erfctimesexpmtat_dn6 = assign15870_e14588_d_n6;
        var_erfctimesexpmtat_dn7 = assign15870_e14588_d_n7;
        var_erfctimesexpmtat_dn8 = assign15870_e14588_d_n8;
        var_erfctimesexpmtat_dn9 = assign15870_e14588_d_n9;

        let (assign15880_e14608, assign15880_e14608_d_n6, assign15880_e14608_d_n7, assign15880_e14608_d_n8, assign15880_e14608_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15880_e14600: f64 = (1.772453850905516 * 0.5);
        let assign15880_e14603: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign15880_e14605: f64 = (assign15880_e14603 / var_ktat);
        let assign15880_e14606: f64 = (assign15880_e14600 * assign15880_e14605);
        (assign15880_e14606, (assign15880_e14600 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign15880_e14603 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign15880_e14600 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign15880_e14603 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign15880_e14600 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign15880_e14603 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign15880_e14600 * ((((var_atatsti * var_erfctimesexpmtat_dn9) * var_ktat) - (assign15880_e14603 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign15880_e14608;
        var_gammamax_dn6 = assign15880_e14608_d_n6;
        var_gammamax_dn7 = assign15880_e14608_d_n7;
        var_gammamax_dn8 = assign15880_e14608_d_n8;
        var_gammamax_dn9 = assign15880_e14608_d_n9;

        let (assign15890_e14626, assign15890_e14626_d_n6, assign15890_e14626_d_n7, assign15890_e14626_d_n8, assign15890_e14626_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard267 == 0.0)) {
        let assign15890_e14621: f64 = (var_asrh * var_gammamax);
        let assign15890_e14623: f64 = (assign15890_e14621 * var_wtat);
        let assign15890_e14624: f64 = (p.p863 * assign15890_e14623);
        (assign15890_e14624, (p.p863 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign15890_e14621 * var_wtat_dn6))), (p.p863 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign15890_e14621 * var_wtat_dn7))), (p.p863 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign15890_e14621 * var_wtat_dn8))), (p.p863 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign15890_e14621 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign15890_e14626;
        var_itat_dn6 = assign15890_e14626_d_n6;
        var_itat_dn7 = assign15890_e14626_d_n7;
        var_itat_dn8 = assign15890_e14626_d_n8;
        var_itat_dn9 = assign15890_e14626_d_n9;

        let assign15900_e14629: f64 = if p.p869 == 0.0 { 1.0 } else { 0.0 };
        var_guard273 = assign15900_e14629;

        let (assign15910_e14640, assign15910_e14640_d_n6, assign15910_e14640_d_n7, assign15910_e14640_d_n8, assign15910_e14640_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign15910_e14640;
        var_ibbt_dn6 = assign15910_e14640_d_n6;
        var_ibbt_dn7 = assign15910_e14640_d_n7;
        var_ibbt_dn8 = assign15910_e14640_d_n8;
        var_ibbt_dn9 = assign15910_e14640_d_n9;

        let assign15920_e14643: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard274 = assign15920_e14643;

        let (assign15930_e14662, assign15930_e14662_d_n6, assign15930_e14662_d_n7, assign15930_e14662_d_n8, assign15930_e14662_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) && (var_guard274 != 0.0)) {
        let assign15930_e14657: f64 = (p.p846 - var_vbbt);
        let assign15930_e14659: f64 = (assign15930_e14657 * var_vbirstiinv);
        let assign15930_e14660: f64 = (assign15930_e14659).sqrt();
        (assign15930_e14660, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15930_e14662;
        var_tmp_dn6 = assign15930_e14662_d_n6;
        var_tmp_dn7 = assign15930_e14662_d_n7;
        var_tmp_dn8 = assign15930_e14662_d_n8;
        var_tmp_dn9 = assign15930_e14662_d_n9;

        let (assign15940_e14683, assign15940_e14683_d_n6, assign15940_e14683_d_n7, assign15940_e14683_d_n8, assign15940_e14683_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) && (var_guard274 == 0.0)) {
        let assign15940_e14677: f64 = (p.p846 - var_vbbt);
        let assign15940_e14679: f64 = (assign15940_e14677 * var_vbirstiinv);
        let assign15940_e14681: f64 = (assign15940_e14679).powf(p.p849);
        (assign15940_e14681, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15940_e14683;
        var_tmp_dn6 = assign15940_e14683_d_n6;
        var_tmp_dn7 = assign15940_e14683_d_n7;
        var_tmp_dn8 = assign15940_e14683_d_n8;
        var_tmp_dn9 = assign15940_e14683_d_n9;

        let (assign15950_e14703, assign15950_e14703_d_n6, assign15950_e14703_d_n7, assign15950_e14703_d_n8, assign15950_e14703_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) {
        let assign15950_e14696: f64 = (p.p846 - var_vbbt);
        let assign15950_e14698: f64 = (assign15950_e14696 * var_wdepnulrinvsti);
        let assign15950_e14700: f64 = (assign15950_e14698 / var_tmp);
        let assign15950_e14701: f64 = (var_one_over_one_minus_psti * assign15950_e14700);
        (assign15950_e14701, (var_one_over_one_minus_psti * (-((assign15950_e14698 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign15950_e14698 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign15950_e14698 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign15950_e14698 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign15950_e14703;
        var_fmaxr_dn6 = assign15950_e14703_d_n6;
        var_fmaxr_dn7 = assign15950_e14703_d_n7;
        var_fmaxr_dn8 = assign15950_e14703_d_n8;
        var_fmaxr_dn9 = assign15950_e14703_d_n9;

        let assign15960_e14705: f64 = (-var_fbbtsti);
        let assign15960_e14707: f64 = (assign15960_e14705 / var_fmaxr);
        let assign15960_e14708: f64 = (assign15960_e14707).abs();
        let assign15960_e14710: f64 = if assign15960_e14708 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard275 = assign15960_e14710;

        let (assign15970_e14728, assign15970_e14728_d_n6, assign15970_e14728_d_n7, assign15970_e14728_d_n8, assign15970_e14728_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) && (var_guard275 != 0.0)) {
        let assign15970_e14723: f64 = (-var_fbbtsti);
        let assign15970_e14725: f64 = (assign15970_e14723 / var_fmaxr);
        let assign15970_e14726: f64 = (assign15970_e14725).exp();
        (assign15970_e14726, (assign15970_e14726 * (-((assign15970_e14723 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign15970_e14726 * (-((assign15970_e14723 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign15970_e14726 * (-((assign15970_e14723 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign15970_e14726 * (-((assign15970_e14723 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15970_e14728;
        var_tmp_dn6 = assign15970_e14728_d_n6;
        var_tmp_dn7 = assign15970_e14728_d_n7;
        var_tmp_dn8 = assign15970_e14728_d_n8;
        var_tmp_dn9 = assign15970_e14728_d_n9;

        let assign15980_e14730: f64 = (-var_fbbtsti);
        let assign15980_e14732: f64 = (assign15980_e14730 / var_fmaxr);
        let assign15980_e14734: f64 = if assign15980_e14732 < 0.0 { 1.0 } else { 0.0 };
        var_guard276 = assign15980_e14734;

        let (assign15990_e14785, assign15990_e14785_d_n6, assign15990_e14785_d_n7, assign15990_e14785_d_n8, assign15990_e14785_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) && (var_guard275 == 0.0)) && (var_guard276 != 0.0)) {
        let assign15990_e14752: f64 = (-230.25850929940458);
        let assign15990_e14754: f64 = (-var_fbbtsti);
        let assign15990_e14756: f64 = (assign15990_e14754 / var_fmaxr);
        let assign15990_e14757: f64 = (assign15990_e14752 - assign15990_e14756);
        let assign15990_e14761: f64 = (-230.25850929940458);
        let assign15990_e14763: f64 = (-var_fbbtsti);
        let assign15990_e14765: f64 = (assign15990_e14763 / var_fmaxr);
        let assign15990_e14766: f64 = (assign15990_e14761 - assign15990_e14765);
        let assign15990_e14769: f64 = (-230.25850929940458);
        let assign15990_e14771: f64 = (-var_fbbtsti);
        let assign15990_e14773: f64 = (assign15990_e14771 / var_fmaxr);
        let assign15990_e14774: f64 = (assign15990_e14769 - assign15990_e14773);
        let assign15990_e14776: f64 = (assign15990_e14774 * 0.3333333333333333);
        let assign15990_e14777: f64 = (1.0 + assign15990_e14776);
        let assign15990_e14778: f64 = (assign15990_e14766 * assign15990_e14777);
        let assign15990_e14779: f64 = (0.5 * assign15990_e14778);
        let assign15990_e14780: f64 = (1.0 + assign15990_e14779);
        let assign15990_e14781: f64 = (assign15990_e14757 * assign15990_e14780);
        let assign15990_e14782: f64 = (1.0 + assign15990_e14781);
        let assign15990_e14783: f64 = (1e-100 / assign15990_e14782);
        (assign15990_e14783, (-((1e-100 * (((-(-((assign15990_e14754 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign15990_e14780) + (assign15990_e14757 * (0.5 * (((-(-((assign15990_e14763 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign15990_e14777) + (assign15990_e14766 * ((-(-((assign15990_e14771 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15990_e14782 * assign15990_e14782))), (-((1e-100 * (((-(-((assign15990_e14754 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign15990_e14780) + (assign15990_e14757 * (0.5 * (((-(-((assign15990_e14763 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign15990_e14777) + (assign15990_e14766 * ((-(-((assign15990_e14771 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15990_e14782 * assign15990_e14782))), (-((1e-100 * (((-(-((assign15990_e14754 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign15990_e14780) + (assign15990_e14757 * (0.5 * (((-(-((assign15990_e14763 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign15990_e14777) + (assign15990_e14766 * ((-(-((assign15990_e14771 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15990_e14782 * assign15990_e14782))), (-((1e-100 * (((-(-((assign15990_e14754 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign15990_e14780) + (assign15990_e14757 * (0.5 * (((-(-((assign15990_e14763 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign15990_e14777) + (assign15990_e14766 * ((-(-((assign15990_e14771 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign15990_e14782 * assign15990_e14782))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign15990_e14785;
        var_tmp_dn6 = assign15990_e14785_d_n6;
        var_tmp_dn7 = assign15990_e14785_d_n7;
        var_tmp_dn8 = assign15990_e14785_d_n8;
        var_tmp_dn9 = assign15990_e14785_d_n9;

        let (assign16000_e14834, assign16000_e14834_d_n6, assign16000_e14834_d_n7, assign16000_e14834_d_n8, assign16000_e14834_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) && (var_guard275 == 0.0)) && (var_guard276 == 0.0)) {
        let assign16000_e14804: f64 = (-var_fbbtsti);
        let assign16000_e14806: f64 = (assign16000_e14804 / var_fmaxr);
        let assign16000_e14808: f64 = (assign16000_e14806 - 230.25850929940458);
        let assign16000_e14812: f64 = (-var_fbbtsti);
        let assign16000_e14814: f64 = (assign16000_e14812 / var_fmaxr);
        let assign16000_e14816: f64 = (assign16000_e14814 - 230.25850929940458);
        let assign16000_e14819: f64 = (-var_fbbtsti);
        let assign16000_e14821: f64 = (assign16000_e14819 / var_fmaxr);
        let assign16000_e14823: f64 = (assign16000_e14821 - 230.25850929940458);
        let assign16000_e14825: f64 = (assign16000_e14823 * 0.3333333333333333);
        let assign16000_e14826: f64 = (1.0 + assign16000_e14825);
        let assign16000_e14827: f64 = (assign16000_e14816 * assign16000_e14826);
        let assign16000_e14828: f64 = (0.5 * assign16000_e14827);
        let assign16000_e14829: f64 = (1.0 + assign16000_e14828);
        let assign16000_e14830: f64 = (assign16000_e14808 * assign16000_e14829);
        let assign16000_e14831: f64 = (1.0 + assign16000_e14830);
        let assign16000_e14832: f64 = (1e100 * assign16000_e14831);
        (assign16000_e14832, (1e100 * (((-((assign16000_e14804 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign16000_e14829) + (assign16000_e14808 * (0.5 * (((-((assign16000_e14812 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign16000_e14826) + (assign16000_e14816 * ((-((assign16000_e14819 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16000_e14804 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign16000_e14829) + (assign16000_e14808 * (0.5 * (((-((assign16000_e14812 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign16000_e14826) + (assign16000_e14816 * ((-((assign16000_e14819 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16000_e14804 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign16000_e14829) + (assign16000_e14808 * (0.5 * (((-((assign16000_e14812 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign16000_e14826) + (assign16000_e14816 * ((-((assign16000_e14819 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign16000_e14804 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign16000_e14829) + (assign16000_e14808 * (0.5 * (((-((assign16000_e14812 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign16000_e14826) + (assign16000_e14816 * ((-((assign16000_e14819 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16000_e14834;
        var_tmp_dn6 = assign16000_e14834_d_n6;
        var_tmp_dn7 = assign16000_e14834_d_n7;
        var_tmp_dn8 = assign16000_e14834_d_n8;
        var_tmp_dn9 = assign16000_e14834_d_n9;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard269_slot = var_guard269;
        *var_guard270_slot = var_guard270;
        *var_guard271_slot = var_guard271;
        *var_guard272_slot = var_guard272;
        *var_guard273_slot = var_guard273;
        *var_guard274_slot = var_guard274;
        *var_guard275_slot = var_guard275;
        *var_guard276_slot = var_guard276;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_25(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard263: f64,
        var_guard273: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard277_slot: &mut f64,
        var_guard278_slot: &mut f64,
        var_guard279_slot: &mut f64,
        var_guard280_slot: &mut f64,
        var_guard281_slot: &mut f64,
        var_guard282_slot: &mut f64,
        var_guard283_slot: &mut f64,
        var_guard284_slot: &mut f64,
        var_guard285_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_guard277: f64 = *var_guard277_slot;
        let mut var_guard278: f64 = *var_guard278_slot;
        let mut var_guard279: f64 = *var_guard279_slot;
        let mut var_guard280: f64 = *var_guard280_slot;
        let mut var_guard281: f64 = *var_guard281_slot;
        let mut var_guard282: f64 = *var_guard282_slot;
        let mut var_guard283: f64 = *var_guard283_slot;
        let mut var_guard284: f64 = *var_guard284_slot;
        let mut var_guard285: f64 = *var_guard285_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;

        let (assign16010_e14854, assign16010_e14854_d_n6, assign16010_e14854_d_n7, assign16010_e14854_d_n8, assign16010_e14854_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard273 == 0.0)) {
        let assign16010_e14847: f64 = (var_v1 * var_fmaxr);
        let assign16010_e14849: f64 = (assign16010_e14847 * var_fmaxr);
        let assign16010_e14851: f64 = (assign16010_e14849 * var_tmp);
        let assign16010_e14852: f64 = (p.p869 * assign16010_e14851);
        (assign16010_e14852, (p.p869 * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign16010_e14847 * var_fmaxr_dn6)) * var_tmp) + (assign16010_e14849 * var_tmp_dn6))), (p.p869 * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign16010_e14847 * var_fmaxr_dn7)) * var_tmp) + (assign16010_e14849 * var_tmp_dn7))), (p.p869 * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign16010_e14847 * var_fmaxr_dn8)) * var_tmp) + (assign16010_e14849 * var_tmp_dn8))), (p.p869 * (((((var_v1 * var_fmaxr_dn9) * var_fmaxr) + (assign16010_e14847 * var_fmaxr_dn9)) * var_tmp) + (assign16010_e14849 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign16010_e14854;
        var_ibbt_dn6 = assign16010_e14854_d_n6;
        var_ibbt_dn7 = assign16010_e14854_d_n7;
        var_ibbt_dn8 = assign16010_e14854_d_n8;
        var_ibbt_dn9 = assign16010_e14854_d_n9;

        let assign16020_e14857: f64 = if p.p878 > 1000.0 { 1.0 } else { 0.0 };
        var_guard277 = assign16020_e14857;

        let (assign16030_e14868, assign16030_e14868_d_n6, assign16030_e14868_d_n7, assign16030_e14868_d_n8, assign16030_e14868_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard277 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign16030_e14868;
        var_fbreakdown_dn6 = assign16030_e14868_d_n6;
        var_fbreakdown_dn7 = assign16030_e14868_d_n7;
        var_fbreakdown_dn8 = assign16030_e14868_d_n8;
        var_fbreakdown_dn9 = assign16030_e14868_d_n9;

        let assign16040_e14871: f64 = (-var_alphaav);
        let assign16040_e14873: f64 = (assign16040_e14871 * p.p878);
        let assign16040_e14874: f64 = if var_vav > assign16040_e14873 { 1.0 } else { 0.0 };
        var_guard278 = assign16040_e14874;

        let assign16050_e14877: f64 = if p.p881 == 4.0 { 1.0 } else { 0.0 };
        var_guard279 = assign16050_e14877;

        let (assign16060_e14907, assign16060_e14907_d_n6, assign16060_e14907_d_n7, assign16060_e14907_d_n8, assign16060_e14907_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard277 == 0.0)) && (var_guard278 != 0.0)) && (var_guard279 != 0.0)) {
        let assign16060_e14893: f64 = (var_vav * var_vbrinvsti);
        let assign16060_e14896: f64 = (var_vav * var_vbrinvsti);
        let assign16060_e14897: f64 = (assign16060_e14893 * assign16060_e14896);
        let assign16060_e14900: f64 = (var_vav * var_vbrinvsti);
        let assign16060_e14901: f64 = (assign16060_e14897 * assign16060_e14900);
        let assign16060_e14904: f64 = (var_vav * var_vbrinvsti);
        let assign16060_e14905: f64 = (assign16060_e14901 * assign16060_e14904);
        (assign16060_e14905, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16060_e14907;
        var_tmp_dn6 = assign16060_e14907_d_n6;
        var_tmp_dn7 = assign16060_e14907_d_n7;
        var_tmp_dn8 = assign16060_e14907_d_n8;
        var_tmp_dn9 = assign16060_e14907_d_n9;

        let (assign16070_e14929, assign16070_e14929_d_n6, assign16070_e14929_d_n7, assign16070_e14929_d_n8, assign16070_e14929_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard277 == 0.0)) && (var_guard278 != 0.0)) && (var_guard279 == 0.0)) {
        let assign16070_e14924: f64 = (var_vav * var_vbrinvsti);
        let assign16070_e14925: f64 = (assign16070_e14924).abs();
        let assign16070_e14927: f64 = (assign16070_e14925).powf(p.p881);
        (assign16070_e14927, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16070_e14929;
        var_tmp_dn6 = assign16070_e14929_d_n6;
        var_tmp_dn7 = assign16070_e14929_d_n7;
        var_tmp_dn8 = assign16070_e14929_d_n8;
        var_tmp_dn9 = assign16070_e14929_d_n9;

        let (assign16080_e14947, assign16080_e14947_d_n6, assign16080_e14947_d_n7, assign16080_e14947_d_n8, assign16080_e14947_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard277 == 0.0)) && (var_guard278 != 0.0)) {
        let assign16080_e14944: f64 = (1.0 - var_tmp);
        let assign16080_e14945: f64 = (1.0 / assign16080_e14944);
        (assign16080_e14945, (-((-var_tmp_dn6) / (assign16080_e14944 * assign16080_e14944))), (-((-var_tmp_dn7) / (assign16080_e14944 * assign16080_e14944))), (-((-var_tmp_dn8) / (assign16080_e14944 * assign16080_e14944))), (-((-var_tmp_dn9) / (assign16080_e14944 * assign16080_e14944))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign16080_e14947;
        var_fbreakdown_dn6 = assign16080_e14947_d_n6;
        var_fbreakdown_dn7 = assign16080_e14947_d_n7;
        var_fbreakdown_dn8 = assign16080_e14947_d_n8;
        var_fbreakdown_dn9 = assign16080_e14947_d_n9;

        let (assign16090_e14970, assign16090_e14970_d_n6, assign16090_e14970_d_n7, assign16090_e14970_d_n8, assign16090_e14970_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) && (var_guard277 == 0.0)) && (var_guard278 == 0.0)) {
        let assign16090_e14964: f64 = (var_alphaav * p.p878);
        let assign16090_e14965: f64 = (var_vav + assign16090_e14964);
        let assign16090_e14967: f64 = (assign16090_e14965 * var_slopesti);
        let assign16090_e14968: f64 = (var_fstopsti + assign16090_e14967);
        (assign16090_e14968, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign16090_e14970;
        var_fbreakdown_dn6 = assign16090_e14970_d_n6;
        var_fbreakdown_dn7 = assign16090_e14970_d_n7;
        var_fbreakdown_dn8 = assign16090_e14970_d_n8;
        var_fbreakdown_dn9 = assign16090_e14970_d_n9;

        let (assign16100_e14989, assign16100_e14989_d_n6, assign16100_e14989_d_n7, assign16100_e14989_d_n8, assign16100_e14989_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard263 == 0.0)) {
        let assign16100_e14980: f64 = (var_id__blk212 + var_isrh);
        let assign16100_e14982: f64 = (assign16100_e14980 + var_itat);
        let assign16100_e14984: f64 = (assign16100_e14982 + var_ibbt);
        let assign16100_e14985: f64 = (p.p29 * assign16100_e14984);
        let assign16100_e14987: f64 = (assign16100_e14985 * var_fbreakdown);
        (assign16100_e14987, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign16100_e14985 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign16100_e14985 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign16100_e14985 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign16100_e14985 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign16100_e14989;
        var_ijunsti_dn6 = assign16100_e14989_d_n6;
        var_ijunsti_dn7 = assign16100_e14989_d_n7;
        var_ijunsti_dn8 = assign16100_e14989_d_n8;
        var_ijunsti_dn9 = assign16100_e14989_d_n9;

        let assign16110_e14992: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard280 = assign16110_e14992;

        let (assign16120_e15000, assign16120_e15000_d_n6, assign16120_e15000_d_n7, assign16120_e15000_d_n8, assign16120_e15000_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign16120_e15000;
        var_ijungat_dn6 = assign16120_e15000_d_n6;
        var_ijungat_dn7 = assign16120_e15000_d_n7;
        var_ijungat_dn8 = assign16120_e15000_d_n8;
        var_ijungat_dn9 = assign16120_e15000_d_n9;

        let (assign16130_e15011,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) {
        let assign16130_e15009: f64 = (var_idsatgat * var_idmult);
        (assign16130_e15009,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign16130_e15011;

        let assign16140_e15018: f64 = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };
        var_guard281 = assign16140_e15018;

        let (assign16150_e15029, assign16150_e15029_d_n6, assign16150_e15029_d_n7, assign16150_e15029_d_n8, assign16150_e15029_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign16150_e15029;
        var_isrh_dn6 = assign16150_e15029_d_n6;
        var_isrh_dn7 = assign16150_e15029_d_n7;
        var_isrh_dn8 = assign16150_e15029_d_n8;
        var_isrh_dn9 = assign16150_e15029_d_n9;

        let (assign16160_e15043,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign16160_e15041: f64 = (var_vbigat - var_vjsrh);
        (assign16160_e15041,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign16160_e15043;

        let (assign16170_e15062,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign16170_e15057: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign16170_e15058: f64 = (1.0 - assign16170_e15057);
        let assign16170_e15059: f64 = (assign16170_e15058).sqrt();
        let assign16170_e15060: f64 = (1.0 - assign16170_e15059);
        (assign16170_e15060,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign16170_e15062;

        let assign16180_e15065: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard282 = assign16180_e15065;

        let (assign16190_e15079,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) && (var_guard282 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign16190_e15079;

        let (assign16200_e15111,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) && (var_guard282 == 0.0)) {
        let assign16200_e15094: f64 = (var_wsrhstep * var_wsrhstep);
        let assign16200_e15096: f64 = (var_wsrhstep).ln();
        let assign16200_e15097: f64 = (assign16200_e15094 * assign16200_e15096);
        let assign16200_e15100: f64 = (1.0 - var_wsrhstep);
        let assign16200_e15101: f64 = (assign16200_e15097 / assign16200_e15100);
        let assign16200_e15103: f64 = (assign16200_e15101 + var_wsrhstep);
        let assign16200_e15107: f64 = (2.0 * p.p850);
        let assign16200_e15108: f64 = (1.0 - assign16200_e15107);
        let assign16200_e15109: f64 = (assign16200_e15103 * assign16200_e15108);
        (assign16200_e15109,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign16200_e15111;

        let (assign16210_e15125,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign16210_e15123: f64 = (var_wsrhstep + var_dwsrh);
        (assign16210_e15123,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign16210_e15125;

        let assign16220_e15128: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard283 = assign16220_e15128;

        let (assign16230_e15145, assign16230_e15145_d_n6, assign16230_e15145_d_n7, assign16230_e15145_d_n8, assign16230_e15145_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) && (var_guard283 != 0.0)) {
        let assign16230_e15142: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign16230_e15143: f64 = (assign16230_e15142).sqrt();
        (assign16230_e15143, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16230_e15145;
        var_tmp_dn6 = assign16230_e15145_d_n6;
        var_tmp_dn7 = assign16230_e15145_d_n7;
        var_tmp_dn8 = assign16230_e15145_d_n8;
        var_tmp_dn9 = assign16230_e15145_d_n9;

        let (assign16240_e15164, assign16240_e15164_d_n6, assign16240_e15164_d_n7, assign16240_e15164_d_n8, assign16240_e15164_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) && (var_guard283 == 0.0)) {
        let assign16240_e15160: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign16240_e15162: f64 = (assign16240_e15160).powf(p.p850);
        (assign16240_e15162, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16240_e15164;
        var_tmp_dn6 = assign16240_e15164_d_n6;
        var_tmp_dn7 = assign16240_e15164_d_n7;
        var_tmp_dn8 = assign16240_e15164_d_n8;
        var_tmp_dn9 = assign16240_e15164_d_n9;

        let (assign16250_e15178, assign16250_e15178_d_n6, assign16250_e15178_d_n7, assign16250_e15178_d_n8, assign16250_e15178_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign16250_e15176: f64 = (var_wdepnulrgat * var_tmp);
        (assign16250_e15176, (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8), (var_wdepnulrgat * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign16250_e15178;
        var_wdep_dn6 = assign16250_e15178_d_n6;
        var_wdep_dn7 = assign16250_e15178_d_n7;
        var_wdep_dn8 = assign16250_e15178_d_n8;
        var_wdep_dn9 = assign16250_e15178_d_n9;

        let (assign16260_e15196, assign16260_e15196_d_n6, assign16260_e15196_d_n7, assign16260_e15196_d_n8, assign16260_e15196_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign16260_e15191: f64 = (var_zinv - 1.0);
        let assign16260_e15193: f64 = (assign16260_e15191 * var_wdep);
        let assign16260_e15194: f64 = (var_ftdgat * assign16260_e15193);
        (assign16260_e15194, (var_ftdgat * (assign16260_e15191 * var_wdep_dn6)), (var_ftdgat * (assign16260_e15191 * var_wdep_dn7)), (var_ftdgat * (assign16260_e15191 * var_wdep_dn8)), (var_ftdgat * (assign16260_e15191 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign16260_e15196;
        var_asrh_dn6 = assign16260_e15196_d_n6;
        var_asrh_dn7 = assign16260_e15196_d_n7;
        var_asrh_dn8 = assign16260_e15196_d_n8;
        var_asrh_dn9 = assign16260_e15196_d_n9;

        let (assign16270_e15212, assign16270_e15212_d_n6, assign16270_e15212_d_n7, assign16270_e15212_d_n8, assign16270_e15212_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard281 == 0.0)) {
        let assign16270_e15209: f64 = (var_asrh * var_wsrh);
        let assign16270_e15210: f64 = (p.p859 * assign16270_e15209);
        (assign16270_e15210, (p.p859 * (var_asrh_dn6 * var_wsrh)), (p.p859 * (var_asrh_dn7 * var_wsrh)), (p.p859 * (var_asrh_dn8 * var_wsrh)), (p.p859 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign16270_e15212;
        var_isrh_dn6 = assign16270_e15212_d_n6;
        var_isrh_dn7 = assign16270_e15212_d_n7;
        var_isrh_dn8 = assign16270_e15212_d_n8;
        var_isrh_dn9 = assign16270_e15212_d_n9;

        let assign16280_e15215: f64 = if p.p864 == 0.0 { 1.0 } else { 0.0 };
        var_guard284 = assign16280_e15215;

        let (assign16290_e15226, assign16290_e15226_d_n6, assign16290_e15226_d_n7, assign16290_e15226_d_n8, assign16290_e15226_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign16290_e15226;
        var_itat_dn6 = assign16290_e15226_d_n6;
        var_itat_dn7 = assign16290_e15226_d_n7;
        var_itat_dn8 = assign16290_e15226_d_n8;
        var_itat_dn9 = assign16290_e15226_d_n9;

        let (assign16300_e15244, assign16300_e15244_d_n6, assign16300_e15244_d_n7, assign16300_e15244_d_n8, assign16300_e15244_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16300_e15239: f64 = (var_wdep * var_one_minus_pgat);
        let assign16300_e15241: f64 = (assign16300_e15239 / var_vbi_minus_vjsrh);
        let assign16300_e15242: f64 = (var_btatpartgat * assign16300_e15241);
        (assign16300_e15242, (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn9 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign16300_e15244;
        var_btat_dn6 = assign16300_e15244_d_n6;
        var_btat_dn7 = assign16300_e15244_d_n7;
        var_btat_dn8 = assign16300_e15244_d_n8;
        var_btat_dn9 = assign16300_e15244_d_n9;

        let (assign16310_e15260, assign16310_e15260_d_n6, assign16310_e15260_d_n7, assign16310_e15260_d_n8, assign16310_e15260_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16310_e15256: f64 = (0.666666666666667 * var_atatgat);
        let assign16310_e15258: f64 = (assign16310_e15256 / var_btat);
        (assign16310_e15258, (-((assign16310_e15256 * var_btat_dn6) / (var_btat * var_btat))), (-((assign16310_e15256 * var_btat_dn7) / (var_btat * var_btat))), (-((assign16310_e15256 * var_btat_dn8) / (var_btat * var_btat))), (-((assign16310_e15256 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign16310_e15260;
        var_twoatatoverthreebtat_dn6 = assign16310_e15260_d_n6;
        var_twoatatoverthreebtat_dn7 = assign16310_e15260_d_n7;
        var_twoatatoverthreebtat_dn8 = assign16310_e15260_d_n8;
        var_twoatatoverthreebtat_dn9 = assign16310_e15260_d_n9;

        let (assign16320_e15274, assign16320_e15274_d_n6, assign16320_e15274_d_n7, assign16320_e15274_d_n8, assign16320_e15274_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16320_e15272: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign16320_e15272, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign16320_e15274;
        var_umaxbeforelimiting_dn6 = assign16320_e15274_d_n6;
        var_umaxbeforelimiting_dn7 = assign16320_e15274_d_n7;
        var_umaxbeforelimiting_dn8 = assign16320_e15274_d_n8;
        var_umaxbeforelimiting_dn9 = assign16320_e15274_d_n9;

        let (assign16330_e15295, assign16330_e15295_d_n6, assign16330_e15295_d_n7, assign16330_e15295_d_n8, assign16330_e15295_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16330_e15286: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign16330_e15289: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign16330_e15291: f64 = (assign16330_e15289 + 1.0);
        let assign16330_e15292: f64 = (assign16330_e15286 / assign16330_e15291);
        let assign16330_e15293: f64 = (assign16330_e15292).sqrt();
        (assign16330_e15293, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign16330_e15291) - (assign16330_e15286 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign16330_e15291 * assign16330_e15291)) / (2.0 * assign16330_e15293)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign16330_e15291) - (assign16330_e15286 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign16330_e15291 * assign16330_e15291)) / (2.0 * assign16330_e15293)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign16330_e15291) - (assign16330_e15286 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign16330_e15291 * assign16330_e15291)) / (2.0 * assign16330_e15293)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign16330_e15291) - (assign16330_e15286 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign16330_e15291 * assign16330_e15291)) / (2.0 * assign16330_e15293)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign16330_e15295;
        var_umax_dn6 = assign16330_e15295_d_n6;
        var_umax_dn7 = assign16330_e15295_d_n7;
        var_umax_dn8 = assign16330_e15295_d_n8;
        var_umax_dn9 = assign16330_e15295_d_n9;

        let (assign16340_e15308, assign16340_e15308_d_n6, assign16340_e15308_d_n7, assign16340_e15308_d_n8, assign16340_e15308_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16340_e15306: f64 = (var_umax).sqrt();
        (assign16340_e15306, (var_umax_dn6 / (2.0 * assign16340_e15306)), (var_umax_dn7 / (2.0 * assign16340_e15306)), (var_umax_dn8 / (2.0 * assign16340_e15306)), (var_umax_dn9 / (2.0 * assign16340_e15306)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign16340_e15308;
        var_sqrtumax_dn6 = assign16340_e15308_d_n6;
        var_sqrtumax_dn7 = assign16340_e15308_d_n7;
        var_sqrtumax_dn8 = assign16340_e15308_d_n8;
        var_sqrtumax_dn9 = assign16340_e15308_d_n9;

        let (assign16350_e15322, assign16350_e15322_d_n6, assign16350_e15322_d_n7, assign16350_e15322_d_n8, assign16350_e15322_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16350_e15320: f64 = (var_umax * var_sqrtumax);
        (assign16350_e15320, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign16350_e15322;
        var_umaxpoweronepointfive_dn6 = assign16350_e15322_d_n6;
        var_umaxpoweronepointfive_dn7 = assign16350_e15322_d_n7;
        var_umaxpoweronepointfive_dn8 = assign16350_e15322_d_n8;
        var_umaxpoweronepointfive_dn9 = assign16350_e15322_d_n9;

        let assign16360_e15324: f64 = (-p.p850);
        let assign16360_e15326: f64 = (assign16360_e15324 * var_one_over_one_minus_pgat);
        let assign16360_e15328: f64 = (-1.0);
        let assign16360_e15329: f64 = if assign16360_e15326 == assign16360_e15328 { 1.0 } else { 0.0 };
        var_guard285 = assign16360_e15329;

        let (assign16370_e15349, assign16370_e15349_d_n6, assign16370_e15349_d_n7, assign16370_e15349_d_n8, assign16370_e15349_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard285 != 0.0)) {
        let assign16370_e15345: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16370_e15346: f64 = (1.0 + assign16370_e15345);
        let assign16370_e15347: f64 = (1.0 / assign16370_e15346);
        (assign16370_e15347, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign16370_e15346 * assign16370_e15346))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign16370_e15346 * assign16370_e15346))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign16370_e15346 * assign16370_e15346))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign16370_e15346 * assign16370_e15346))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign16370_e15349;
        var_wgamma_dn6 = assign16370_e15349_d_n6;
        var_wgamma_dn7 = assign16370_e15349_d_n7;
        var_wgamma_dn8 = assign16370_e15349_d_n8;
        var_wgamma_dn9 = assign16370_e15349_d_n9;

        let (assign16380_e15373, assign16380_e15373_d_n6, assign16380_e15373_d_n7, assign16380_e15373_d_n8, assign16380_e15373_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard285 == 0.0)) {
        let assign16380_e15365: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16380_e15366: f64 = (1.0 + assign16380_e15365);
        let assign16380_e15368: f64 = (-p.p850);
        let assign16380_e15370: f64 = (assign16380_e15368 * var_one_over_one_minus_pgat);
        let assign16380_e15371: f64 = (assign16380_e15366).powf(assign16380_e15370);
        (assign16380_e15371, if 0.0 == 0.0 && ((assign16380_e15370) as f64).is_finite() && ((assign16380_e15370) as f64).fract() == 0.0 { if assign16380_e15370 == 0.0 { 0.0 } else { (assign16380_e15370 * ((assign16380_e15366).powf(assign16380_e15370 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign16380_e15371 * (assign16380_e15370 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign16380_e15366))) }, if 0.0 == 0.0 && ((assign16380_e15370) as f64).is_finite() && ((assign16380_e15370) as f64).fract() == 0.0 { if assign16380_e15370 == 0.0 { 0.0 } else { (assign16380_e15370 * ((assign16380_e15366).powf(assign16380_e15370 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign16380_e15371 * (assign16380_e15370 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign16380_e15366))) }, if 0.0 == 0.0 && ((assign16380_e15370) as f64).is_finite() && ((assign16380_e15370) as f64).fract() == 0.0 { if assign16380_e15370 == 0.0 { 0.0 } else { (assign16380_e15370 * ((assign16380_e15366).powf(assign16380_e15370 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign16380_e15371 * (assign16380_e15370 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign16380_e15366))) }, if 0.0 == 0.0 && ((assign16380_e15370) as f64).is_finite() && ((assign16380_e15370) as f64).fract() == 0.0 { if assign16380_e15370 == 0.0 { 0.0 } else { (assign16380_e15370 * ((assign16380_e15366).powf(assign16380_e15370 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign16380_e15371 * (assign16380_e15370 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign16380_e15366))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign16380_e15373;
        var_wgamma_dn6 = assign16380_e15373_d_n6;
        var_wgamma_dn7 = assign16380_e15373_d_n7;
        var_wgamma_dn8 = assign16380_e15373_d_n8;
        var_wgamma_dn9 = assign16380_e15373_d_n9;

        let (assign16390_e15391, assign16390_e15391_d_n6, assign16390_e15391_d_n7, assign16390_e15391_d_n8, assign16390_e15391_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16390_e15385: f64 = (var_wsrh * var_wgamma);
        let assign16390_e15388: f64 = (var_wsrh + var_wgamma);
        let assign16390_e15389: f64 = (assign16390_e15385 / assign16390_e15388);
        (assign16390_e15389, ((((var_wsrh * var_wgamma_dn6) * assign16390_e15388) - (assign16390_e15385 * var_wgamma_dn6)) / (assign16390_e15388 * assign16390_e15388)), ((((var_wsrh * var_wgamma_dn7) * assign16390_e15388) - (assign16390_e15385 * var_wgamma_dn7)) / (assign16390_e15388 * assign16390_e15388)), ((((var_wsrh * var_wgamma_dn8) * assign16390_e15388) - (assign16390_e15385 * var_wgamma_dn8)) / (assign16390_e15388 * assign16390_e15388)), ((((var_wsrh * var_wgamma_dn9) * assign16390_e15388) - (assign16390_e15385 * var_wgamma_dn9)) / (assign16390_e15388 * assign16390_e15388)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign16390_e15391;
        var_wtat_dn6 = assign16390_e15391_d_n6;
        var_wtat_dn7 = assign16390_e15391_d_n7;
        var_wtat_dn8 = assign16390_e15391_d_n8;
        var_wtat_dn9 = assign16390_e15391_d_n9;

        let (assign16400_e15408, assign16400_e15408_d_n6, assign16400_e15408_d_n7, assign16400_e15408_d_n8, assign16400_e15408_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16400_e15404: f64 = (var_btat / var_sqrtumax);
        let assign16400_e15405: f64 = (0.375 * assign16400_e15404);
        let assign16400_e15406: f64 = (assign16400_e15405).sqrt();
        (assign16400_e15406, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16400_e15406)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16400_e15406)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16400_e15406)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign16400_e15406)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign16400_e15408;
        var_ktat_dn6 = assign16400_e15408_d_n6;
        var_ktat_dn7 = assign16400_e15408_d_n7;
        var_ktat_dn8 = assign16400_e15408_d_n8;
        var_ktat_dn9 = assign16400_e15408_d_n9;

        let (assign16410_e15426, assign16410_e15426_d_n6, assign16410_e15426_d_n7, assign16410_e15426_d_n8, assign16410_e15426_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16410_e15421: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign16410_e15422: f64 = (2.0 * assign16410_e15421);
        let assign16410_e15424: f64 = (assign16410_e15422 - var_umax);
        (assign16410_e15424, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign16410_e15426;
        var_ltat_dn6 = assign16410_e15426_d_n6;
        var_ltat_dn7 = assign16410_e15426_d_n7;
        var_ltat_dn8 = assign16410_e15426_d_n8;
        var_ltat_dn9 = assign16410_e15426_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard277_slot = var_guard277;
        *var_guard278_slot = var_guard278;
        *var_guard279_slot = var_guard279;
        *var_guard280_slot = var_guard280;
        *var_guard281_slot = var_guard281;
        *var_guard282_slot = var_guard282;
        *var_guard283_slot = var_guard283;
        *var_guard284_slot = var_guard284;
        *var_guard285_slot = var_guard285;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
    }

    pub(super) fn stamp_transient_block_26(
        p: &Parameters,
        var_alphaav: f64,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatgat: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cerfc: f64,
        var_fbbtgat: f64,
        var_fbbtgat_dn6: f64,
        var_fbbtgat_dn7: f64,
        var_fbbtgat_dn8: f64,
        var_fbbtgat_dn9: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard280: f64,
        var_guard284: f64,
        var_ktat: f64,
        var_ktat_dn6: f64,
        var_ktat_dn7: f64,
        var_ktat_dn8: f64,
        var_ktat_dn9: f64,
        var_ltat: f64,
        var_ltat_dn6: f64,
        var_ltat_dn7: f64,
        var_ltat_dn8: f64,
        var_ltat_dn9: f64,
        var_one_over_one_minus_pgat: f64,
        var_perfc: f64,
        var_sqrtumax: f64,
        var_sqrtumax_dn6: f64,
        var_sqrtumax_dn7: f64,
        var_sqrtumax_dn8: f64,
        var_sqrtumax_dn9: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_twoatatoverthreebtat_dn9: f64,
        var_umax: f64,
        var_umax_dn6: f64,
        var_umax_dn7: f64,
        var_umax_dn8: f64,
        var_umax_dn9: f64,
        var_umaxpoweronepointfive: f64,
        var_umaxpoweronepointfive_dn6: f64,
        var_umaxpoweronepointfive_dn7: f64,
        var_umaxpoweronepointfive_dn8: f64,
        var_umaxpoweronepointfive_dn9: f64,
        var_v1: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirgatinv: f64,
        var_vbrinvgat: f64,
        var_vbrinvgat_dn6: f64,
        var_vbrinvgat_dn7: f64,
        var_vbrinvgat_dn8: f64,
        var_vbrinvgat_dn9: f64,
        var_wdepnulrinvgat: f64,
        var_wtat: f64,
        var_wtat_dn6: f64,
        var_wtat_dn7: f64,
        var_wtat_dn8: f64,
        var_wtat_dn9: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard286_slot: &mut f64,
        var_guard287_slot: &mut f64,
        var_guard288_slot: &mut f64,
        var_guard289_slot: &mut f64,
        var_guard290_slot: &mut f64,
        var_guard291_slot: &mut f64,
        var_guard292_slot: &mut f64,
        var_guard293_slot: &mut f64,
        var_guard294_slot: &mut f64,
        var_guard295_slot: &mut f64,
        var_guard296_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard286: f64 = *var_guard286_slot;
        let mut var_guard287: f64 = *var_guard287_slot;
        let mut var_guard288: f64 = *var_guard288_slot;
        let mut var_guard289: f64 = *var_guard289_slot;
        let mut var_guard290: f64 = *var_guard290_slot;
        let mut var_guard291: f64 = *var_guard291_slot;
        let mut var_guard292: f64 = *var_guard292_slot;
        let mut var_guard293: f64 = *var_guard293_slot;
        let mut var_guard294: f64 = *var_guard294_slot;
        let mut var_guard295: f64 = *var_guard295_slot;
        let mut var_guard296: f64 = *var_guard296_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign16420_e15452, assign16420_e15452_d_n6, assign16420_e15452_d_n7, assign16420_e15452_d_n8, assign16420_e15452_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16420_e15438: f64 = (var_atatgat * var_twoatatoverthreebtat);
        let assign16420_e15440: f64 = (assign16420_e15438 * var_sqrtumax);
        let assign16420_e15443: f64 = (var_atatgat * var_umax);
        let assign16420_e15444: f64 = (assign16420_e15440 - assign16420_e15443);
        let assign16420_e15448: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign16420_e15449: f64 = (0.5 * assign16420_e15448);
        let assign16420_e15450: f64 = (assign16420_e15444 + assign16420_e15449);
        (assign16420_e15450, (((((var_atatgat * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign16420_e15438 * var_sqrtumax_dn6)) - (var_atatgat * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatgat * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign16420_e15438 * var_sqrtumax_dn7)) - (var_atatgat * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatgat * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign16420_e15438 * var_sqrtumax_dn8)) - (var_atatgat * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatgat * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign16420_e15438 * var_sqrtumax_dn9)) - (var_atatgat * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign16420_e15452;
        var_mtat_dn6 = assign16420_e15452_d_n6;
        var_mtat_dn7 = assign16420_e15452_d_n7;
        var_mtat_dn8 = assign16420_e15452_d_n8;
        var_mtat_dn9 = assign16420_e15452_d_n9;

        let (assign16430_e15468, assign16430_e15468_d_n6, assign16430_e15468_d_n7, assign16430_e15468_d_n8, assign16430_e15468_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16430_e15464: f64 = (var_ltat - 1.0);
        let assign16430_e15466: f64 = (assign16430_e15464 * var_ktat);
        (assign16430_e15466, ((var_ltat_dn6 * var_ktat) + (assign16430_e15464 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign16430_e15464 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign16430_e15464 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign16430_e15464 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign16430_e15468;
        var_xerfc_dn6 = assign16430_e15468_d_n6;
        var_xerfc_dn7 = assign16430_e15468_d_n7;
        var_xerfc_dn8 = assign16430_e15468_d_n8;
        var_xerfc_dn9 = assign16430_e15468_d_n9;

        let (assign16440_e15482, assign16440_e15482_d_n6, assign16440_e15482_d_n7, assign16440_e15482_d_n8, assign16440_e15482_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16440_e15480: f64 = (var_xerfc * var_xerfc);
        (assign16440_e15480, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign16440_e15482;
        var_ysq_dn6 = assign16440_e15482_d_n6;
        var_ysq_dn7 = assign16440_e15482_d_n7;
        var_ysq_dn8 = assign16440_e15482_d_n8;
        var_ysq_dn9 = assign16440_e15482_d_n9;

        let assign16450_e15485: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard286 = assign16450_e15485;

        let (assign16460_e15505, assign16460_e15505_d_n6, assign16460_e15505_d_n7, assign16460_e15505_d_n8, assign16460_e15505_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard286 != 0.0)) {
        let assign16460_e15501: f64 = (var_perfc * var_xerfc);
        let assign16460_e15502: f64 = (1.0 + assign16460_e15501);
        let assign16460_e15503: f64 = (1.0 / assign16460_e15502);
        (assign16460_e15503, (-((var_perfc * var_xerfc_dn6) / (assign16460_e15502 * assign16460_e15502))), (-((var_perfc * var_xerfc_dn7) / (assign16460_e15502 * assign16460_e15502))), (-((var_perfc * var_xerfc_dn8) / (assign16460_e15502 * assign16460_e15502))), (-((var_perfc * var_xerfc_dn9) / (assign16460_e15502 * assign16460_e15502))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign16460_e15505;
        var_terfc_dn6 = assign16460_e15505_d_n6;
        var_terfc_dn7 = assign16460_e15505_d_n7;
        var_terfc_dn8 = assign16460_e15505_d_n8;
        var_terfc_dn9 = assign16460_e15505_d_n9;

        let (assign16470_e15526, assign16470_e15526_d_n6, assign16470_e15526_d_n7, assign16470_e15526_d_n8, assign16470_e15526_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard286 == 0.0)) {
        let assign16470_e15522: f64 = (var_perfc * var_xerfc);
        let assign16470_e15523: f64 = (1.0 - assign16470_e15522);
        let assign16470_e15524: f64 = (1.0 / assign16470_e15523);
        (assign16470_e15524, (-((-(var_perfc * var_xerfc_dn6)) / (assign16470_e15523 * assign16470_e15523))), (-((-(var_perfc * var_xerfc_dn7)) / (assign16470_e15523 * assign16470_e15523))), (-((-(var_perfc * var_xerfc_dn8)) / (assign16470_e15523 * assign16470_e15523))), (-((-(var_perfc * var_xerfc_dn9)) / (assign16470_e15523 * assign16470_e15523))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign16470_e15526;
        var_terfc_dn6 = assign16470_e15526_d_n6;
        var_terfc_dn7 = assign16470_e15526_d_n7;
        var_terfc_dn8 = assign16470_e15526_d_n8;
        var_terfc_dn9 = assign16470_e15526_d_n9;

        let assign16480_e15528: f64 = (-var_ysq);
        let assign16480_e15530: f64 = (assign16480_e15528 + var_mtat);
        let assign16480_e15532: f64 = (-230.25850929940458);
        let assign16480_e15533: f64 = if assign16480_e15530 > assign16480_e15532 { 1.0 } else { 0.0 };
        var_guard287 = assign16480_e15533;

        let (assign16490_e15551, assign16490_e15551_d_n6, assign16490_e15551_d_n7, assign16490_e15551_d_n8, assign16490_e15551_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard287 != 0.0)) {
        let assign16490_e15546: f64 = (-var_ysq);
        let assign16490_e15548: f64 = (assign16490_e15546 + var_mtat);
        let assign16490_e15549: f64 = (assign16490_e15548).exp();
        (assign16490_e15549, (assign16490_e15549 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign16490_e15549 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign16490_e15549 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign16490_e15549 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16490_e15551;
        var_tmp_dn6 = assign16490_e15551_d_n6;
        var_tmp_dn7 = assign16490_e15551_d_n7;
        var_tmp_dn8 = assign16490_e15551_d_n8;
        var_tmp_dn9 = assign16490_e15551_d_n9;

        let (assign16500_e15600, assign16500_e15600_d_n6, assign16500_e15600_d_n7, assign16500_e15600_d_n8, assign16500_e15600_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard287 == 0.0)) {
        let assign16500_e15567: f64 = (-230.25850929940458);
        let assign16500_e15569: f64 = (-var_ysq);
        let assign16500_e15571: f64 = (assign16500_e15569 + var_mtat);
        let assign16500_e15572: f64 = (assign16500_e15567 - assign16500_e15571);
        let assign16500_e15576: f64 = (-230.25850929940458);
        let assign16500_e15578: f64 = (-var_ysq);
        let assign16500_e15580: f64 = (assign16500_e15578 + var_mtat);
        let assign16500_e15581: f64 = (assign16500_e15576 - assign16500_e15580);
        let assign16500_e15584: f64 = (-230.25850929940458);
        let assign16500_e15586: f64 = (-var_ysq);
        let assign16500_e15588: f64 = (assign16500_e15586 + var_mtat);
        let assign16500_e15589: f64 = (assign16500_e15584 - assign16500_e15588);
        let assign16500_e15591: f64 = (assign16500_e15589 * 0.3333333333333333);
        let assign16500_e15592: f64 = (1.0 + assign16500_e15591);
        let assign16500_e15593: f64 = (assign16500_e15581 * assign16500_e15592);
        let assign16500_e15594: f64 = (0.5 * assign16500_e15593);
        let assign16500_e15595: f64 = (1.0 + assign16500_e15594);
        let assign16500_e15596: f64 = (assign16500_e15572 * assign16500_e15595);
        let assign16500_e15597: f64 = (1.0 + assign16500_e15596);
        let assign16500_e15598: f64 = (1e-100 / assign16500_e15597);
        (assign16500_e15598, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign16500_e15595) + (assign16500_e15572 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign16500_e15592) + (assign16500_e15581 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign16500_e15597 * assign16500_e15597))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign16500_e15595) + (assign16500_e15572 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign16500_e15592) + (assign16500_e15581 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign16500_e15597 * assign16500_e15597))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign16500_e15595) + (assign16500_e15572 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign16500_e15592) + (assign16500_e15581 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign16500_e15597 * assign16500_e15597))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign16500_e15595) + (assign16500_e15572 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign16500_e15592) + (assign16500_e15581 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign16500_e15597 * assign16500_e15597))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16500_e15600;
        var_tmp_dn6 = assign16500_e15600_d_n6;
        var_tmp_dn7 = assign16500_e15600_d_n7;
        var_tmp_dn8 = assign16500_e15600_d_n8;
        var_tmp_dn9 = assign16500_e15600_d_n9;

        let (assign16510_e15630, assign16510_e15630_d_n6, assign16510_e15630_d_n7, assign16510_e15630_d_n8, assign16510_e15630_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16510_e15612: f64 = (0.29214664 * var_terfc);
        let assign16510_e15616: f64 = (var_terfc * var_terfc);
        let assign16510_e15617: f64 = (var_berfc * assign16510_e15616);
        let assign16510_e15618: f64 = (assign16510_e15612 + assign16510_e15617);
        let assign16510_e15622: f64 = (var_terfc * var_terfc);
        let assign16510_e15624: f64 = (assign16510_e15622 * var_terfc);
        let assign16510_e15625: f64 = (var_cerfc * assign16510_e15624);
        let assign16510_e15626: f64 = (assign16510_e15618 + assign16510_e15625);
        let assign16510_e15628: f64 = (assign16510_e15626 * var_tmp);
        (assign16510_e15628, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign16510_e15622 * var_terfc_dn6)))) * var_tmp) + (assign16510_e15626 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign16510_e15622 * var_terfc_dn7)))) * var_tmp) + (assign16510_e15626 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign16510_e15622 * var_terfc_dn8)))) * var_tmp) + (assign16510_e15626 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign16510_e15622 * var_terfc_dn9)))) * var_tmp) + (assign16510_e15626 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign16510_e15630;
        var_erfcpos_dn6 = assign16510_e15630_d_n6;
        var_erfcpos_dn7 = assign16510_e15630_d_n7;
        var_erfcpos_dn8 = assign16510_e15630_d_n8;
        var_erfcpos_dn9 = assign16510_e15630_d_n9;

        let assign16520_e15633: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard288 = assign16520_e15633;

        let (assign16530_e15647, assign16530_e15647_d_n6, assign16530_e15647_d_n7, assign16530_e15647_d_n8, assign16530_e15647_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard288 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign16530_e15647;
        var_erfctimesexpmtat_dn6 = assign16530_e15647_d_n6;
        var_erfctimesexpmtat_dn7 = assign16530_e15647_d_n7;
        var_erfctimesexpmtat_dn8 = assign16530_e15647_d_n8;
        var_erfctimesexpmtat_dn9 = assign16530_e15647_d_n9;

        let assign16540_e15650: f64 = (-230.25850929940458);
        let assign16540_e15651: f64 = if var_mtat > assign16540_e15650 { 1.0 } else { 0.0 };
        var_guard289 = assign16540_e15651;

        let (assign16550_e15669, assign16550_e15669_d_n6, assign16550_e15669_d_n7, assign16550_e15669_d_n8, assign16550_e15669_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard288 == 0.0)) && (var_guard289 != 0.0)) {
        let assign16550_e15667: f64 = (var_mtat).exp();
        (assign16550_e15667, (assign16550_e15667 * var_mtat_dn6), (assign16550_e15667 * var_mtat_dn7), (assign16550_e15667 * var_mtat_dn8), (assign16550_e15667 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16550_e15669;
        var_tmp_dn6 = assign16550_e15669_d_n6;
        var_tmp_dn7 = assign16550_e15669_d_n7;
        var_tmp_dn8 = assign16550_e15669_d_n8;
        var_tmp_dn9 = assign16550_e15669_d_n9;

        let (assign16560_e15712, assign16560_e15712_d_n6, assign16560_e15712_d_n7, assign16560_e15712_d_n8, assign16560_e15712_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard288 == 0.0)) && (var_guard289 == 0.0)) {
        let assign16560_e15688: f64 = (-230.25850929940458);
        let assign16560_e15690: f64 = (assign16560_e15688 - var_mtat);
        let assign16560_e15694: f64 = (-230.25850929940458);
        let assign16560_e15696: f64 = (assign16560_e15694 - var_mtat);
        let assign16560_e15699: f64 = (-230.25850929940458);
        let assign16560_e15701: f64 = (assign16560_e15699 - var_mtat);
        let assign16560_e15703: f64 = (assign16560_e15701 * 0.3333333333333333);
        let assign16560_e15704: f64 = (1.0 + assign16560_e15703);
        let assign16560_e15705: f64 = (assign16560_e15696 * assign16560_e15704);
        let assign16560_e15706: f64 = (0.5 * assign16560_e15705);
        let assign16560_e15707: f64 = (1.0 + assign16560_e15706);
        let assign16560_e15708: f64 = (assign16560_e15690 * assign16560_e15707);
        let assign16560_e15709: f64 = (1.0 + assign16560_e15708);
        let assign16560_e15710: f64 = (1e-100 / assign16560_e15709);
        (assign16560_e15710, (-((1e-100 * (((-var_mtat_dn6) * assign16560_e15707) + (assign16560_e15690 * (0.5 * (((-var_mtat_dn6) * assign16560_e15704) + (assign16560_e15696 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign16560_e15709 * assign16560_e15709))), (-((1e-100 * (((-var_mtat_dn7) * assign16560_e15707) + (assign16560_e15690 * (0.5 * (((-var_mtat_dn7) * assign16560_e15704) + (assign16560_e15696 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign16560_e15709 * assign16560_e15709))), (-((1e-100 * (((-var_mtat_dn8) * assign16560_e15707) + (assign16560_e15690 * (0.5 * (((-var_mtat_dn8) * assign16560_e15704) + (assign16560_e15696 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign16560_e15709 * assign16560_e15709))), (-((1e-100 * (((-var_mtat_dn9) * assign16560_e15707) + (assign16560_e15690 * (0.5 * (((-var_mtat_dn9) * assign16560_e15704) + (assign16560_e15696 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign16560_e15709 * assign16560_e15709))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16560_e15712;
        var_tmp_dn6 = assign16560_e15712_d_n6;
        var_tmp_dn7 = assign16560_e15712_d_n7;
        var_tmp_dn8 = assign16560_e15712_d_n8;
        var_tmp_dn9 = assign16560_e15712_d_n9;

        let (assign16570_e15731, assign16570_e15731_d_n6, assign16570_e15731_d_n7, assign16570_e15731_d_n8, assign16570_e15731_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) && (var_guard288 == 0.0)) {
        let assign16570_e15727: f64 = (2.0 * var_tmp);
        let assign16570_e15729: f64 = (assign16570_e15727 - var_erfcpos);
        (assign16570_e15729, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign16570_e15731;
        var_erfctimesexpmtat_dn6 = assign16570_e15731_d_n6;
        var_erfctimesexpmtat_dn7 = assign16570_e15731_d_n7;
        var_erfctimesexpmtat_dn8 = assign16570_e15731_d_n8;
        var_erfctimesexpmtat_dn9 = assign16570_e15731_d_n9;

        let (assign16580_e15751, assign16580_e15751_d_n6, assign16580_e15751_d_n7, assign16580_e15751_d_n8, assign16580_e15751_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16580_e15743: f64 = (1.772453850905516 * 0.5);
        let assign16580_e15746: f64 = (var_atatgat * var_erfctimesexpmtat);
        let assign16580_e15748: f64 = (assign16580_e15746 / var_ktat);
        let assign16580_e15749: f64 = (assign16580_e15743 * assign16580_e15748);
        (assign16580_e15749, (assign16580_e15743 * ((((var_atatgat * var_erfctimesexpmtat_dn6) * var_ktat) - (assign16580_e15746 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign16580_e15743 * ((((var_atatgat * var_erfctimesexpmtat_dn7) * var_ktat) - (assign16580_e15746 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign16580_e15743 * ((((var_atatgat * var_erfctimesexpmtat_dn8) * var_ktat) - (assign16580_e15746 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign16580_e15743 * ((((var_atatgat * var_erfctimesexpmtat_dn9) * var_ktat) - (assign16580_e15746 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign16580_e15751;
        var_gammamax_dn6 = assign16580_e15751_d_n6;
        var_gammamax_dn7 = assign16580_e15751_d_n7;
        var_gammamax_dn8 = assign16580_e15751_d_n8;
        var_gammamax_dn9 = assign16580_e15751_d_n9;

        let (assign16590_e15769, assign16590_e15769_d_n6, assign16590_e15769_d_n7, assign16590_e15769_d_n8, assign16590_e15769_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard284 == 0.0)) {
        let assign16590_e15764: f64 = (var_asrh * var_gammamax);
        let assign16590_e15766: f64 = (assign16590_e15764 * var_wtat);
        let assign16590_e15767: f64 = (p.p864 * assign16590_e15766);
        (assign16590_e15767, (p.p864 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign16590_e15764 * var_wtat_dn6))), (p.p864 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign16590_e15764 * var_wtat_dn7))), (p.p864 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign16590_e15764 * var_wtat_dn8))), (p.p864 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign16590_e15764 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign16590_e15769;
        var_itat_dn6 = assign16590_e15769_d_n6;
        var_itat_dn7 = assign16590_e15769_d_n7;
        var_itat_dn8 = assign16590_e15769_d_n8;
        var_itat_dn9 = assign16590_e15769_d_n9;

        let assign16600_e15772: f64 = if p.p870 == 0.0 { 1.0 } else { 0.0 };
        var_guard290 = assign16600_e15772;

        let (assign16610_e15783, assign16610_e15783_d_n6, assign16610_e15783_d_n7, assign16610_e15783_d_n8, assign16610_e15783_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign16610_e15783;
        var_ibbt_dn6 = assign16610_e15783_d_n6;
        var_ibbt_dn7 = assign16610_e15783_d_n7;
        var_ibbt_dn8 = assign16610_e15783_d_n8;
        var_ibbt_dn9 = assign16610_e15783_d_n9;

        let assign16620_e15786: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard291 = assign16620_e15786;

        let (assign16630_e15805, assign16630_e15805_d_n6, assign16630_e15805_d_n7, assign16630_e15805_d_n8, assign16630_e15805_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) && (var_guard291 != 0.0)) {
        let assign16630_e15800: f64 = (p.p847 - var_vbbt);
        let assign16630_e15802: f64 = (assign16630_e15800 * var_vbirgatinv);
        let assign16630_e15803: f64 = (assign16630_e15802).sqrt();
        (assign16630_e15803, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16630_e15805;
        var_tmp_dn6 = assign16630_e15805_d_n6;
        var_tmp_dn7 = assign16630_e15805_d_n7;
        var_tmp_dn8 = assign16630_e15805_d_n8;
        var_tmp_dn9 = assign16630_e15805_d_n9;

        let (assign16640_e15826, assign16640_e15826_d_n6, assign16640_e15826_d_n7, assign16640_e15826_d_n8, assign16640_e15826_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) && (var_guard291 == 0.0)) {
        let assign16640_e15820: f64 = (p.p847 - var_vbbt);
        let assign16640_e15822: f64 = (assign16640_e15820 * var_vbirgatinv);
        let assign16640_e15824: f64 = (assign16640_e15822).powf(p.p850);
        (assign16640_e15824, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16640_e15826;
        var_tmp_dn6 = assign16640_e15826_d_n6;
        var_tmp_dn7 = assign16640_e15826_d_n7;
        var_tmp_dn8 = assign16640_e15826_d_n8;
        var_tmp_dn9 = assign16640_e15826_d_n9;

        let (assign16650_e15846, assign16650_e15846_d_n6, assign16650_e15846_d_n7, assign16650_e15846_d_n8, assign16650_e15846_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) {
        let assign16650_e15839: f64 = (p.p847 - var_vbbt);
        let assign16650_e15841: f64 = (assign16650_e15839 * var_wdepnulrinvgat);
        let assign16650_e15843: f64 = (assign16650_e15841 / var_tmp);
        let assign16650_e15844: f64 = (var_one_over_one_minus_pgat * assign16650_e15843);
        (assign16650_e15844, (var_one_over_one_minus_pgat * (-((assign16650_e15841 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign16650_e15841 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign16650_e15841 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pgat * (-((assign16650_e15841 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign16650_e15846;
        var_fmaxr_dn6 = assign16650_e15846_d_n6;
        var_fmaxr_dn7 = assign16650_e15846_d_n7;
        var_fmaxr_dn8 = assign16650_e15846_d_n8;
        var_fmaxr_dn9 = assign16650_e15846_d_n9;

        let assign16660_e15848: f64 = (-var_fbbtgat);
        let assign16660_e15850: f64 = (assign16660_e15848 / var_fmaxr);
        let assign16660_e15851: f64 = (assign16660_e15850).abs();
        let assign16660_e15853: f64 = if assign16660_e15851 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard292 = assign16660_e15853;

        let (assign16670_e15871, assign16670_e15871_d_n6, assign16670_e15871_d_n7, assign16670_e15871_d_n8, assign16670_e15871_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) && (var_guard292 != 0.0)) {
        let assign16670_e15866: f64 = (-var_fbbtgat);
        let assign16670_e15868: f64 = (assign16670_e15866 / var_fmaxr);
        let assign16670_e15869: f64 = (assign16670_e15868).exp();
        (assign16670_e15869, (assign16670_e15869 * ((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16670_e15866 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))), (assign16670_e15869 * ((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16670_e15866 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))), (assign16670_e15869 * ((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16670_e15866 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))), (assign16670_e15869 * ((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16670_e15866 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16670_e15871;
        var_tmp_dn6 = assign16670_e15871_d_n6;
        var_tmp_dn7 = assign16670_e15871_d_n7;
        var_tmp_dn8 = assign16670_e15871_d_n8;
        var_tmp_dn9 = assign16670_e15871_d_n9;

        let assign16680_e15873: f64 = (-var_fbbtgat);
        let assign16680_e15875: f64 = (assign16680_e15873 / var_fmaxr);
        let assign16680_e15877: f64 = if assign16680_e15875 < 0.0 { 1.0 } else { 0.0 };
        var_guard293 = assign16680_e15877;

        let (assign16690_e15928, assign16690_e15928_d_n6, assign16690_e15928_d_n7, assign16690_e15928_d_n8, assign16690_e15928_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) && (var_guard292 == 0.0)) && (var_guard293 != 0.0)) {
        let assign16690_e15895: f64 = (-230.25850929940458);
        let assign16690_e15897: f64 = (-var_fbbtgat);
        let assign16690_e15899: f64 = (assign16690_e15897 / var_fmaxr);
        let assign16690_e15900: f64 = (assign16690_e15895 - assign16690_e15899);
        let assign16690_e15904: f64 = (-230.25850929940458);
        let assign16690_e15906: f64 = (-var_fbbtgat);
        let assign16690_e15908: f64 = (assign16690_e15906 / var_fmaxr);
        let assign16690_e15909: f64 = (assign16690_e15904 - assign16690_e15908);
        let assign16690_e15912: f64 = (-230.25850929940458);
        let assign16690_e15914: f64 = (-var_fbbtgat);
        let assign16690_e15916: f64 = (assign16690_e15914 / var_fmaxr);
        let assign16690_e15917: f64 = (assign16690_e15912 - assign16690_e15916);
        let assign16690_e15919: f64 = (assign16690_e15917 * 0.3333333333333333);
        let assign16690_e15920: f64 = (1.0 + assign16690_e15919);
        let assign16690_e15921: f64 = (assign16690_e15909 * assign16690_e15920);
        let assign16690_e15922: f64 = (0.5 * assign16690_e15921);
        let assign16690_e15923: f64 = (1.0 + assign16690_e15922);
        let assign16690_e15924: f64 = (assign16690_e15900 * assign16690_e15923);
        let assign16690_e15925: f64 = (1.0 + assign16690_e15924);
        let assign16690_e15926: f64 = (1e-100 / assign16690_e15925);
        (assign16690_e15926, (-((1e-100 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16690_e15897 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign16690_e15923) + (assign16690_e15900 * (0.5 * (((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16690_e15906 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * assign16690_e15920) + (assign16690_e15909 * ((-((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16690_e15914 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign16690_e15925 * assign16690_e15925))), (-((1e-100 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16690_e15897 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign16690_e15923) + (assign16690_e15900 * (0.5 * (((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16690_e15906 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * assign16690_e15920) + (assign16690_e15909 * ((-((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16690_e15914 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign16690_e15925 * assign16690_e15925))), (-((1e-100 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16690_e15897 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign16690_e15923) + (assign16690_e15900 * (0.5 * (((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16690_e15906 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * assign16690_e15920) + (assign16690_e15909 * ((-((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16690_e15914 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign16690_e15925 * assign16690_e15925))), (-((1e-100 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16690_e15897 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign16690_e15923) + (assign16690_e15900 * (0.5 * (((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16690_e15906 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * assign16690_e15920) + (assign16690_e15909 * ((-((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16690_e15914 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))) / (assign16690_e15925 * assign16690_e15925))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16690_e15928;
        var_tmp_dn6 = assign16690_e15928_d_n6;
        var_tmp_dn7 = assign16690_e15928_d_n7;
        var_tmp_dn8 = assign16690_e15928_d_n8;
        var_tmp_dn9 = assign16690_e15928_d_n9;

        let (assign16700_e15977, assign16700_e15977_d_n6, assign16700_e15977_d_n7, assign16700_e15977_d_n8, assign16700_e15977_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) && (var_guard292 == 0.0)) && (var_guard293 == 0.0)) {
        let assign16700_e15947: f64 = (-var_fbbtgat);
        let assign16700_e15949: f64 = (assign16700_e15947 / var_fmaxr);
        let assign16700_e15951: f64 = (assign16700_e15949 - 230.25850929940458);
        let assign16700_e15955: f64 = (-var_fbbtgat);
        let assign16700_e15957: f64 = (assign16700_e15955 / var_fmaxr);
        let assign16700_e15959: f64 = (assign16700_e15957 - 230.25850929940458);
        let assign16700_e15962: f64 = (-var_fbbtgat);
        let assign16700_e15964: f64 = (assign16700_e15962 / var_fmaxr);
        let assign16700_e15966: f64 = (assign16700_e15964 - 230.25850929940458);
        let assign16700_e15968: f64 = (assign16700_e15966 * 0.3333333333333333);
        let assign16700_e15969: f64 = (1.0 + assign16700_e15968);
        let assign16700_e15970: f64 = (assign16700_e15959 * assign16700_e15969);
        let assign16700_e15971: f64 = (0.5 * assign16700_e15970);
        let assign16700_e15972: f64 = (1.0 + assign16700_e15971);
        let assign16700_e15973: f64 = (assign16700_e15951 * assign16700_e15972);
        let assign16700_e15974: f64 = (1.0 + assign16700_e15973);
        let assign16700_e15975: f64 = (1e100 * assign16700_e15974);
        (assign16700_e15975, (1e100 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16700_e15947 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign16700_e15972) + (assign16700_e15951 * (0.5 * ((((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16700_e15955 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * assign16700_e15969) + (assign16700_e15959 * (((((-var_fbbtgat_dn6) * var_fmaxr) - (assign16700_e15962 * var_fmaxr_dn6)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16700_e15947 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign16700_e15972) + (assign16700_e15951 * (0.5 * ((((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16700_e15955 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * assign16700_e15969) + (assign16700_e15959 * (((((-var_fbbtgat_dn7) * var_fmaxr) - (assign16700_e15962 * var_fmaxr_dn7)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16700_e15947 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign16700_e15972) + (assign16700_e15951 * (0.5 * ((((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16700_e15955 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * assign16700_e15969) + (assign16700_e15959 * (((((-var_fbbtgat_dn8) * var_fmaxr) - (assign16700_e15962 * var_fmaxr_dn8)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))), (1e100 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16700_e15947 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign16700_e15972) + (assign16700_e15951 * (0.5 * ((((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16700_e15955 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * assign16700_e15969) + (assign16700_e15959 * (((((-var_fbbtgat_dn9) * var_fmaxr) - (assign16700_e15962 * var_fmaxr_dn9)) / (var_fmaxr * var_fmaxr)) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16700_e15977;
        var_tmp_dn6 = assign16700_e15977_d_n6;
        var_tmp_dn7 = assign16700_e15977_d_n7;
        var_tmp_dn8 = assign16700_e15977_d_n8;
        var_tmp_dn9 = assign16700_e15977_d_n9;

        let (assign16710_e15997, assign16710_e15997_d_n6, assign16710_e15997_d_n7, assign16710_e15997_d_n8, assign16710_e15997_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard290 == 0.0)) {
        let assign16710_e15990: f64 = (var_v1 * var_fmaxr);
        let assign16710_e15992: f64 = (assign16710_e15990 * var_fmaxr);
        let assign16710_e15994: f64 = (assign16710_e15992 * var_tmp);
        let assign16710_e15995: f64 = (p.p870 * assign16710_e15994);
        (assign16710_e15995, (p.p870 * (((((var_v1 * var_fmaxr_dn6) * var_fmaxr) + (assign16710_e15990 * var_fmaxr_dn6)) * var_tmp) + (assign16710_e15992 * var_tmp_dn6))), (p.p870 * (((((var_v1 * var_fmaxr_dn7) * var_fmaxr) + (assign16710_e15990 * var_fmaxr_dn7)) * var_tmp) + (assign16710_e15992 * var_tmp_dn7))), (p.p870 * (((((var_v1 * var_fmaxr_dn8) * var_fmaxr) + (assign16710_e15990 * var_fmaxr_dn8)) * var_tmp) + (assign16710_e15992 * var_tmp_dn8))), (p.p870 * (((((var_v1 * var_fmaxr_dn9) * var_fmaxr) + (assign16710_e15990 * var_fmaxr_dn9)) * var_tmp) + (assign16710_e15992 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign16710_e15997;
        var_ibbt_dn6 = assign16710_e15997_d_n6;
        var_ibbt_dn7 = assign16710_e15997_d_n7;
        var_ibbt_dn8 = assign16710_e15997_d_n8;
        var_ibbt_dn9 = assign16710_e15997_d_n9;

        let assign16720_e16000: f64 = if p.p879 > 1000.0 { 1.0 } else { 0.0 };
        var_guard294 = assign16720_e16000;

        let (assign16730_e16011, assign16730_e16011_d_n6, assign16730_e16011_d_n7, assign16730_e16011_d_n8, assign16730_e16011_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard294 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign16730_e16011;
        var_fbreakdown_dn6 = assign16730_e16011_d_n6;
        var_fbreakdown_dn7 = assign16730_e16011_d_n7;
        var_fbreakdown_dn8 = assign16730_e16011_d_n8;
        var_fbreakdown_dn9 = assign16730_e16011_d_n9;

        let assign16740_e16014: f64 = (-var_alphaav);
        let assign16740_e16016: f64 = (assign16740_e16014 * p.p879);
        let assign16740_e16017: f64 = if var_vav > assign16740_e16016 { 1.0 } else { 0.0 };
        var_guard295 = assign16740_e16017;

        let assign16750_e16020: f64 = if p.p882 == 4.0 { 1.0 } else { 0.0 };
        var_guard296 = assign16750_e16020;

        let (assign16760_e16050, assign16760_e16050_d_n6, assign16760_e16050_d_n7, assign16760_e16050_d_n8, assign16760_e16050_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard294 == 0.0)) && (var_guard295 != 0.0)) && (var_guard296 != 0.0)) {
        let assign16760_e16036: f64 = (var_vav * var_vbrinvgat);
        let assign16760_e16039: f64 = (var_vav * var_vbrinvgat);
        let assign16760_e16040: f64 = (assign16760_e16036 * assign16760_e16039);
        let assign16760_e16043: f64 = (var_vav * var_vbrinvgat);
        let assign16760_e16044: f64 = (assign16760_e16040 * assign16760_e16043);
        let assign16760_e16047: f64 = (var_vav * var_vbrinvgat);
        let assign16760_e16048: f64 = (assign16760_e16044 * assign16760_e16047);
        (assign16760_e16048, (((((((var_vav * var_vbrinvgat_dn6) * assign16760_e16039) + (assign16760_e16036 * (var_vav * var_vbrinvgat_dn6))) * assign16760_e16043) + (assign16760_e16040 * (var_vav * var_vbrinvgat_dn6))) * assign16760_e16047) + (assign16760_e16044 * (var_vav * var_vbrinvgat_dn6))), (((((((var_vav * var_vbrinvgat_dn7) * assign16760_e16039) + (assign16760_e16036 * (var_vav * var_vbrinvgat_dn7))) * assign16760_e16043) + (assign16760_e16040 * (var_vav * var_vbrinvgat_dn7))) * assign16760_e16047) + (assign16760_e16044 * (var_vav * var_vbrinvgat_dn7))), (((((((var_vav * var_vbrinvgat_dn8) * assign16760_e16039) + (assign16760_e16036 * (var_vav * var_vbrinvgat_dn8))) * assign16760_e16043) + (assign16760_e16040 * (var_vav * var_vbrinvgat_dn8))) * assign16760_e16047) + (assign16760_e16044 * (var_vav * var_vbrinvgat_dn8))), (((((((var_vav * var_vbrinvgat_dn9) * assign16760_e16039) + (assign16760_e16036 * (var_vav * var_vbrinvgat_dn9))) * assign16760_e16043) + (assign16760_e16040 * (var_vav * var_vbrinvgat_dn9))) * assign16760_e16047) + (assign16760_e16044 * (var_vav * var_vbrinvgat_dn9))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16760_e16050;
        var_tmp_dn6 = assign16760_e16050_d_n6;
        var_tmp_dn7 = assign16760_e16050_d_n7;
        var_tmp_dn8 = assign16760_e16050_d_n8;
        var_tmp_dn9 = assign16760_e16050_d_n9;

        let (assign16770_e16072, assign16770_e16072_d_n6, assign16770_e16072_d_n7, assign16770_e16072_d_n8, assign16770_e16072_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard294 == 0.0)) && (var_guard295 != 0.0)) && (var_guard296 == 0.0)) {
        let assign16770_e16067: f64 = (var_vav * var_vbrinvgat);
        let assign16770_e16068: f64 = (assign16770_e16067).abs();
        let assign16770_e16070: f64 = (assign16770_e16068).powf(p.p882);
        (assign16770_e16070, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign16770_e16068).powf(p.p882 - 1.0) * if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) })) } } else { (assign16770_e16070 * (p.p882 * (if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn6) } else { (-(var_vav * var_vbrinvgat_dn6)) } / assign16770_e16068))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign16770_e16068).powf(p.p882 - 1.0) * if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) })) } } else { (assign16770_e16070 * (p.p882 * (if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn7) } else { (-(var_vav * var_vbrinvgat_dn7)) } / assign16770_e16068))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign16770_e16068).powf(p.p882 - 1.0) * if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) })) } } else { (assign16770_e16070 * (p.p882 * (if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn8) } else { (-(var_vav * var_vbrinvgat_dn8)) } / assign16770_e16068))) }, if 0.0 == 0.0 && ((p.p882) as f64).is_finite() && ((p.p882) as f64).fract() == 0.0 { if p.p882 == 0.0 { 0.0 } else { (p.p882 * ((assign16770_e16068).powf(p.p882 - 1.0) * if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) })) } } else { (assign16770_e16070 * (p.p882 * (if assign16770_e16067 >= 0.0 { (var_vav * var_vbrinvgat_dn9) } else { (-(var_vav * var_vbrinvgat_dn9)) } / assign16770_e16068))) },)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign16770_e16072;
        var_tmp_dn6 = assign16770_e16072_d_n6;
        var_tmp_dn7 = assign16770_e16072_d_n7;
        var_tmp_dn8 = assign16770_e16072_d_n8;
        var_tmp_dn9 = assign16770_e16072_d_n9;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard286_slot = var_guard286;
        *var_guard287_slot = var_guard287;
        *var_guard288_slot = var_guard288;
        *var_guard289_slot = var_guard289;
        *var_guard290_slot = var_guard290;
        *var_guard291_slot = var_guard291;
        *var_guard292_slot = var_guard292;
        *var_guard293_slot = var_guard293;
        *var_guard294_slot = var_guard294;
        *var_guard295_slot = var_guard295;
        *var_guard296_slot = var_guard296;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_27(
        p: &Parameters,
        var_absource_i: f64,
        var_alphaav: f64,
        var_exp_vmax_over_phitd_s: f64,
        var_fstopgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard280: f64,
        var_guard294: f64,
        var_guard295: f64,
        var_ibbt: f64,
        var_ibbt_dn6: f64,
        var_ibbt_dn7: f64,
        var_ibbt_dn8: f64,
        var_ibbt_dn9: f64,
        var_idsatbot: f64,
        var_ijunsti: f64,
        var_ijunsti_dn6: f64,
        var_ijunsti_dn7: f64,
        var_ijunsti_dn8: f64,
        var_ijunsti_dn9: f64,
        var_itat: f64,
        var_itat_dn6: f64,
        var_itat_dn7: f64,
        var_itat_dn8: f64,
        var_itat_dn9: f64,
        var_lgsource_i: f64,
        var_lssource_i: f64,
        var_phitd: f64,
        var_phitdinv: f64,
        var_phitr: f64,
        var_slopegat: f64,
        var_slopegat_dn6: f64,
        var_slopegat_dn7: f64,
        var_slopegat_dn8: f64,
        var_slopegat_dn9: f64,
        var_v2: f64,
        var_vbbtlim_s: f64,
        var_vbibot: f64,
        var_vbimin_s: f64,
        var_vbirbotinv: f64,
        var_vmax_s: f64,
        var_wdepnulrbot: f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard297_slot: &mut f64,
        var_guard298_slot: &mut f64,
        var_guard299_slot: &mut f64,
        var_guard300_slot: &mut f64,
        var_guard301_slot: &mut f64,
        var_guard302_slot: &mut f64,
        var_guard303_slot: &mut f64,
        var_guard304_slot: &mut f64,
        var_guard305_slot: &mut f64,
        var_i1_slot: &mut f64,
        var_i1_dn6_slot: &mut f64,
        var_i1_dn7_slot: &mut f64,
        var_i1_dn8_slot: &mut f64,
        var_i1_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_idmult_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_two_psistar_slot: &mut f64,
        var_vav_slot: &mut f64,
        var_vbbt_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_vjlim_slot: &mut f64,
        var_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
        var_z_slot: &mut f64,
        var_zinv_slot: &mut f64,
    ) {
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_guard297: f64 = *var_guard297_slot;
        let mut var_guard298: f64 = *var_guard298_slot;
        let mut var_guard299: f64 = *var_guard299_slot;
        let mut var_guard300: f64 = *var_guard300_slot;
        let mut var_guard301: f64 = *var_guard301_slot;
        let mut var_guard302: f64 = *var_guard302_slot;
        let mut var_guard303: f64 = *var_guard303_slot;
        let mut var_guard304: f64 = *var_guard304_slot;
        let mut var_guard305: f64 = *var_guard305_slot;
        let mut var_i1: f64 = *var_i1_slot;
        let mut var_i1_dn6: f64 = *var_i1_dn6_slot;
        let mut var_i1_dn7: f64 = *var_i1_dn7_slot;
        let mut var_i1_dn8: f64 = *var_i1_dn8_slot;
        let mut var_i1_dn9: f64 = *var_i1_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_idmult: f64 = *var_idmult_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_two_psistar: f64 = *var_two_psistar_slot;
        let mut var_vav: f64 = *var_vav_slot;
        let mut var_vbbt: f64 = *var_vbbt_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_vjlim: f64 = *var_vjlim_slot;
        let mut var_vjsrh: f64 = *var_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;
        let mut var_z: f64 = *var_z_slot;
        let mut var_zinv: f64 = *var_zinv_slot;

        let (assign16780_e16090, assign16780_e16090_d_n6, assign16780_e16090_d_n7, assign16780_e16090_d_n8, assign16780_e16090_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard294 == 0.0)) && (var_guard295 != 0.0)) {
        let assign16780_e16087: f64 = (1.0 - var_tmp);
        let assign16780_e16088: f64 = (1.0 / assign16780_e16087);
        (assign16780_e16088, (-((-var_tmp_dn6) / (assign16780_e16087 * assign16780_e16087))), (-((-var_tmp_dn7) / (assign16780_e16087 * assign16780_e16087))), (-((-var_tmp_dn8) / (assign16780_e16087 * assign16780_e16087))), (-((-var_tmp_dn9) / (assign16780_e16087 * assign16780_e16087))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign16780_e16090;
        var_fbreakdown_dn6 = assign16780_e16090_d_n6;
        var_fbreakdown_dn7 = assign16780_e16090_d_n7;
        var_fbreakdown_dn8 = assign16780_e16090_d_n8;
        var_fbreakdown_dn9 = assign16780_e16090_d_n9;

        let (assign16790_e16113, assign16790_e16113_d_n6, assign16790_e16113_d_n7, assign16790_e16113_d_n8, assign16790_e16113_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) && (var_guard294 == 0.0)) && (var_guard295 == 0.0)) {
        let assign16790_e16107: f64 = (var_alphaav * p.p879);
        let assign16790_e16108: f64 = (var_vav + assign16790_e16107);
        let assign16790_e16110: f64 = (assign16790_e16108 * var_slopegat);
        let assign16790_e16111: f64 = (var_fstopgat + assign16790_e16110);
        (assign16790_e16111, (assign16790_e16108 * var_slopegat_dn6), (assign16790_e16108 * var_slopegat_dn7), (assign16790_e16108 * var_slopegat_dn8), (assign16790_e16108 * var_slopegat_dn9),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign16790_e16113;
        var_fbreakdown_dn6 = assign16790_e16113_d_n6;
        var_fbreakdown_dn7 = assign16790_e16113_d_n7;
        var_fbreakdown_dn8 = assign16790_e16113_d_n8;
        var_fbreakdown_dn9 = assign16790_e16113_d_n9;

        let (assign16800_e16132, assign16800_e16132_d_n6, assign16800_e16132_d_n7, assign16800_e16132_d_n8, assign16800_e16132_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard280 == 0.0)) {
        let assign16800_e16123: f64 = (var_id__blk212 + var_isrh);
        let assign16800_e16125: f64 = (assign16800_e16123 + var_itat);
        let assign16800_e16127: f64 = (assign16800_e16125 + var_ibbt);
        let assign16800_e16128: f64 = (p.p29 * assign16800_e16127);
        let assign16800_e16130: f64 = (assign16800_e16128 * var_fbreakdown);
        (assign16800_e16130, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign16800_e16128 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign16800_e16128 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign16800_e16128 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign16800_e16128 * var_fbreakdown_dn9)),)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign16800_e16132;
        var_ijungat_dn6 = assign16800_e16132_d_n6;
        var_ijungat_dn7 = assign16800_e16132_d_n7;
        var_ijungat_dn8 = assign16800_e16132_d_n8;
        var_ijungat_dn9 = assign16800_e16132_d_n9;

        let (assign16810_e16148, assign16810_e16148_d_n6, assign16810_e16148_d_n7, assign16810_e16148_d_n8, assign16810_e16148_d_n9,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        let assign16810_e16138: f64 = (var_absource_i * var_ijunbot);
        let assign16810_e16141: f64 = (var_lssource_i * var_ijunsti);
        let assign16810_e16142: f64 = (assign16810_e16138 + assign16810_e16141);
        let assign16810_e16145: f64 = (var_lgsource_i * var_ijungat);
        let assign16810_e16146: f64 = (assign16810_e16142 + assign16810_e16145);
        (assign16810_e16146, (((var_absource_i * var_ijunbot_dn6) + (var_lssource_i * var_ijunsti_dn6)) + (var_lgsource_i * var_ijungat_dn6)), (((var_absource_i * var_ijunbot_dn7) + (var_lssource_i * var_ijunsti_dn7)) + (var_lgsource_i * var_ijungat_dn7)), (((var_absource_i * var_ijunbot_dn8) + (var_lssource_i * var_ijunsti_dn8)) + (var_lgsource_i * var_ijungat_dn8)), (((var_absource_i * var_ijunbot_dn9) + (var_lssource_i * var_ijunsti_dn9)) + (var_lgsource_i * var_ijungat_dn9)),)
    } else {
        (var_i1, var_i1_dn6, var_i1_dn7, var_i1_dn8, var_i1_dn9,)
    }
};
        var_i1 = assign16810_e16148;
        var_i1_dn6 = assign16810_e16148_d_n6;
        var_i1_dn7 = assign16810_e16148_d_n7;
        var_i1_dn8 = assign16810_e16148_d_n8;
        var_i1_dn9 = assign16810_e16148_d_n9;

        let (assign16820_e16154,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign16820_e16154;

        let (assign16830_e16160,) = {
    if ((var_guard175 != 0.0) && (var_guard192 != 0.0)) {
        (0.0,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign16830_e16160;

        let assign16840_e16172: f64 = if (!(((var_absource_i == 0.0) && (var_lssource_i == 0.0)) && (var_lgsource_i == 0.0))) { 1.0 } else { 0.0 };
        var_guard297 = assign16840_e16172;

        let assign16920_e16258: f64 = if var_v2 < var_vmax_s { 1.0 } else { 0.0 };
        var_guard298 = assign16920_e16258;

        let assign16930_e16260: f64 = (-0.5);
        let assign16930_e16263: f64 = (var_v2 * var_phitdinv);
        let assign16930_e16264: f64 = (assign16930_e16260 * assign16930_e16263);
        let assign16930_e16265: f64 = (assign16930_e16264).abs();
        let assign16930_e16267: f64 = if assign16930_e16265 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard299 = assign16930_e16267;

        let (assign16940_e16285,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 != 0.0)) && (var_guard299 != 0.0)) {
        let assign16940_e16278: f64 = (-0.5);
        let assign16940_e16281: f64 = (var_v2 * var_phitdinv);
        let assign16940_e16282: f64 = (assign16940_e16278 * assign16940_e16281);
        let assign16940_e16283: f64 = (assign16940_e16282).exp();
        (assign16940_e16283,)
    } else {
        (var_z,)
    }
};
        var_z = assign16940_e16285;

        let assign16950_e16287: f64 = (-0.5);
        let assign16950_e16290: f64 = (var_v2 * var_phitdinv);
        let assign16950_e16291: f64 = (assign16950_e16287 * assign16950_e16290);
        let assign16950_e16293: f64 = if assign16950_e16291 < 0.0 { 1.0 } else { 0.0 };
        var_guard300 = assign16950_e16293;

        let (assign16960_e16348,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 != 0.0)) && (var_guard299 == 0.0)) && (var_guard300 != 0.0)) {
        let assign16960_e16309: f64 = (-230.25850929940458);
        let assign16960_e16311: f64 = (-0.5);
        let assign16960_e16314: f64 = (var_v2 * var_phitdinv);
        let assign16960_e16315: f64 = (assign16960_e16311 * assign16960_e16314);
        let assign16960_e16316: f64 = (assign16960_e16309 - assign16960_e16315);
        let assign16960_e16320: f64 = (-230.25850929940458);
        let assign16960_e16322: f64 = (-0.5);
        let assign16960_e16325: f64 = (var_v2 * var_phitdinv);
        let assign16960_e16326: f64 = (assign16960_e16322 * assign16960_e16325);
        let assign16960_e16327: f64 = (assign16960_e16320 - assign16960_e16326);
        let assign16960_e16330: f64 = (-230.25850929940458);
        let assign16960_e16332: f64 = (-0.5);
        let assign16960_e16335: f64 = (var_v2 * var_phitdinv);
        let assign16960_e16336: f64 = (assign16960_e16332 * assign16960_e16335);
        let assign16960_e16337: f64 = (assign16960_e16330 - assign16960_e16336);
        let assign16960_e16339: f64 = (assign16960_e16337 * 0.3333333333333333);
        let assign16960_e16340: f64 = (1.0 + assign16960_e16339);
        let assign16960_e16341: f64 = (assign16960_e16327 * assign16960_e16340);
        let assign16960_e16342: f64 = (0.5 * assign16960_e16341);
        let assign16960_e16343: f64 = (1.0 + assign16960_e16342);
        let assign16960_e16344: f64 = (assign16960_e16316 * assign16960_e16343);
        let assign16960_e16345: f64 = (1.0 + assign16960_e16344);
        let assign16960_e16346: f64 = (1e-100 / assign16960_e16345);
        (assign16960_e16346,)
    } else {
        (var_z,)
    }
};
        var_z = assign16960_e16348;

        let (assign16970_e16401,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 != 0.0)) && (var_guard299 == 0.0)) && (var_guard300 == 0.0)) {
        let assign16970_e16365: f64 = (-0.5);
        let assign16970_e16368: f64 = (var_v2 * var_phitdinv);
        let assign16970_e16369: f64 = (assign16970_e16365 * assign16970_e16368);
        let assign16970_e16371: f64 = (assign16970_e16369 - 230.25850929940458);
        let assign16970_e16375: f64 = (-0.5);
        let assign16970_e16378: f64 = (var_v2 * var_phitdinv);
        let assign16970_e16379: f64 = (assign16970_e16375 * assign16970_e16378);
        let assign16970_e16381: f64 = (assign16970_e16379 - 230.25850929940458);
        let assign16970_e16384: f64 = (-0.5);
        let assign16970_e16387: f64 = (var_v2 * var_phitdinv);
        let assign16970_e16388: f64 = (assign16970_e16384 * assign16970_e16387);
        let assign16970_e16390: f64 = (assign16970_e16388 - 230.25850929940458);
        let assign16970_e16392: f64 = (assign16970_e16390 * 0.3333333333333333);
        let assign16970_e16393: f64 = (1.0 + assign16970_e16392);
        let assign16970_e16394: f64 = (assign16970_e16381 * assign16970_e16393);
        let assign16970_e16395: f64 = (0.5 * assign16970_e16394);
        let assign16970_e16396: f64 = (1.0 + assign16970_e16395);
        let assign16970_e16397: f64 = (assign16970_e16371 * assign16970_e16396);
        let assign16970_e16398: f64 = (1.0 + assign16970_e16397);
        let assign16970_e16399: f64 = (1e100 * assign16970_e16398);
        (assign16970_e16399,)
    } else {
        (var_z,)
    }
};
        var_z = assign16970_e16401;

        let (assign16980_e16413,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 != 0.0)) {
        let assign16980_e16411: f64 = (1.0 / var_z);
        (assign16980_e16411,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign16980_e16413;

        let (assign16990_e16425,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 != 0.0)) {
        let assign16990_e16423: f64 = (var_zinv * var_zinv);
        (assign16990_e16423,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign16990_e16425;

        let (assign17000_e16444,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 == 0.0)) {
        let assign17000_e16437: f64 = (var_v2 - var_vmax_s);
        let assign17000_e16439: f64 = (assign17000_e16437 * var_phitdinv);
        let assign17000_e16440: f64 = (1.0 + assign17000_e16439);
        let assign17000_e16442: f64 = (assign17000_e16440 * var_exp_vmax_over_phitd_s);
        (assign17000_e16442,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign17000_e16444;

        let (assign17010_e16456,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 == 0.0)) {
        let assign17010_e16454: f64 = (var_idmult).sqrt();
        (assign17010_e16454,)
    } else {
        (var_zinv,)
    }
};
        var_zinv = assign17010_e16456;

        let (assign17020_e16469,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard298 == 0.0)) {
        let assign17020_e16467: f64 = (1.0 / var_zinv);
        (assign17020_e16467,)
    } else {
        (var_z,)
    }
};
        var_z = assign17020_e16469;

        let (assign17030_e16479,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) {
        let assign17030_e16477: f64 = (var_idmult - 1.0);
        (assign17030_e16477,)
    } else {
        (var_idmult,)
    }
};
        var_idmult = assign17030_e16479;

        let assign17040_e16482: f64 = if var_v2 > 0.0 { 1.0 } else { 0.0 };
        var_guard301 = assign17040_e16482;

        let (assign17050_e16508,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard301 != 0.0)) {
        let assign17050_e16494: f64 = (2.0 + var_z);
        let assign17050_e16497: f64 = (var_z + 1.0);
        let assign17050_e16500: f64 = (var_z + 3.0);
        let assign17050_e16501: f64 = (assign17050_e16497 * assign17050_e16500);
        let assign17050_e16502: f64 = (assign17050_e16501).sqrt();
        let assign17050_e16503: f64 = (assign17050_e16494 + assign17050_e16502);
        let assign17050_e16504: f64 = (assign17050_e16503).ln();
        let assign17050_e16505: f64 = (var_phitd * assign17050_e16504);
        let assign17050_e16506: f64 = (2.0 * assign17050_e16505);
        (assign17050_e16506,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign17050_e16508;

        let (assign17060_e16542,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) && (var_guard301 == 0.0)) {
        let assign17060_e16518: f64 = (-var_v2);
        let assign17060_e16523: f64 = (2.0 * var_zinv);
        let assign17060_e16525: f64 = (assign17060_e16523 + 1.0);
        let assign17060_e16528: f64 = (1.0 + var_zinv);
        let assign17060_e16532: f64 = (3.0 * var_zinv);
        let assign17060_e16533: f64 = (1.0 + assign17060_e16532);
        let assign17060_e16534: f64 = (assign17060_e16528 * assign17060_e16533);
        let assign17060_e16535: f64 = (assign17060_e16534).sqrt();
        let assign17060_e16536: f64 = (assign17060_e16525 + assign17060_e16535);
        let assign17060_e16537: f64 = (assign17060_e16536).ln();
        let assign17060_e16538: f64 = (var_phitd * assign17060_e16537);
        let assign17060_e16539: f64 = (2.0 * assign17060_e16538);
        let assign17060_e16540: f64 = (assign17060_e16518 + assign17060_e16539);
        (assign17060_e16540,)
    } else {
        (var_two_psistar,)
    }
};
        var_two_psistar = assign17060_e16542;

        let (assign17070_e16552,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) {
        let assign17070_e16550: f64 = (var_vbimin_s - var_two_psistar);
        (assign17070_e16550,)
    } else {
        (var_vjlim,)
    }
};
        var_vjlim = assign17070_e16552;

        let (assign17080_e16579,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) {
        let assign17080_e16561: f64 = (var_v2 + var_vjlim);
        let assign17080_e16564: f64 = (var_v2 - var_vjlim);
        let assign17080_e16567: f64 = (var_v2 - var_vjlim);
        let assign17080_e16568: f64 = (assign17080_e16564 * assign17080_e16567);
        let assign17080_e16571: f64 = (4.0 * var_phitd);
        let assign17080_e16573: f64 = (assign17080_e16571 * var_phitd);
        let assign17080_e16574: f64 = (assign17080_e16568 + assign17080_e16573);
        let assign17080_e16575: f64 = (assign17080_e16574).sqrt();
        let assign17080_e16576: f64 = (assign17080_e16561 - assign17080_e16575);
        let assign17080_e16577: f64 = (0.5 * assign17080_e16576);
        (assign17080_e16577,)
    } else {
        (var_vjsrh,)
    }
};
        var_vjsrh = assign17080_e16579;

        let (assign17090_e16606,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) {
        let assign17090_e16588: f64 = (var_v2 + var_vbbtlim_s);
        let assign17090_e16591: f64 = (var_v2 - var_vbbtlim_s);
        let assign17090_e16594: f64 = (var_v2 - var_vbbtlim_s);
        let assign17090_e16595: f64 = (assign17090_e16591 * assign17090_e16594);
        let assign17090_e16598: f64 = (4.0 * var_phitr);
        let assign17090_e16600: f64 = (assign17090_e16598 * var_phitr);
        let assign17090_e16601: f64 = (assign17090_e16595 + assign17090_e16600);
        let assign17090_e16602: f64 = (assign17090_e16601).sqrt();
        let assign17090_e16603: f64 = (assign17090_e16588 - assign17090_e16602);
        let assign17090_e16604: f64 = (0.5 * assign17090_e16603);
        (assign17090_e16604,)
    } else {
        (var_vbbt,)
    }
};
        var_vbbt = assign17090_e16606;

        let (assign17100_e16633,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard297 != 0.0)) {
        let assign17100_e16615: f64 = var_v2;
        let assign17100_e16618: f64 = var_v2;
        let assign17100_e16621: f64 = var_v2;
        let assign17100_e16622: f64 = (assign17100_e16618 * assign17100_e16621);
        let assign17100_e16625: f64 = (4.0 * 1e-6);
        let assign17100_e16627: f64 = (assign17100_e16625 * 1e-6);
        let assign17100_e16628: f64 = (assign17100_e16622 + assign17100_e16627);
        let assign17100_e16629: f64 = (assign17100_e16628).sqrt();
        let assign17100_e16630: f64 = (assign17100_e16615 - assign17100_e16629);
        let assign17100_e16631: f64 = (0.5 * assign17100_e16630);
        (assign17100_e16631,)
    } else {
        (var_vav,)
    }
};
        var_vav = assign17100_e16633;

        let assign17110_e16636: f64 = if var_absource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard302 = assign17110_e16636;

        let (assign17120_e16644, assign17120_e16644_d_n6, assign17120_e16644_d_n7, assign17120_e16644_d_n8, assign17120_e16644_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign17120_e16644;
        var_ijunbot_dn6 = assign17120_e16644_d_n6;
        var_ijunbot_dn7 = assign17120_e16644_d_n7;
        var_ijunbot_dn8 = assign17120_e16644_d_n8;
        var_ijunbot_dn9 = assign17120_e16644_d_n9;

        let (assign17130_e16655,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) {
        let assign17130_e16653: f64 = (var_idsatbot * var_idmult);
        (assign17130_e16653,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign17130_e16655;

        let assign17140_e16662: f64 = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };
        var_guard303 = assign17140_e16662;

        let (assign17150_e16673, assign17150_e16673_d_n6, assign17150_e16673_d_n7, assign17150_e16673_d_n8, assign17150_e16673_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign17150_e16673;
        var_isrh_dn6 = assign17150_e16673_d_n6;
        var_isrh_dn7 = assign17150_e16673_d_n7;
        var_isrh_dn8 = assign17150_e16673_d_n8;
        var_isrh_dn9 = assign17150_e16673_d_n9;

        let (assign17160_e16687,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) {
        let assign17160_e16685: f64 = (var_vbibot - var_vjsrh);
        (assign17160_e16685,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign17160_e16687;

        let (assign17170_e16706,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) {
        let assign17170_e16701: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign17170_e16702: f64 = (1.0 - assign17170_e16701);
        let assign17170_e16703: f64 = (assign17170_e16702).sqrt();
        let assign17170_e16704: f64 = (1.0 - assign17170_e16703);
        (assign17170_e16704,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign17170_e16706;

        let assign17180_e16709: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard304 = assign17180_e16709;

        let (assign17190_e16723,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) && (var_guard304 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign17190_e16723;

        let (assign17200_e16755,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) && (var_guard304 == 0.0)) {
        let assign17200_e16738: f64 = (var_wsrhstep * var_wsrhstep);
        let assign17200_e16740: f64 = (var_wsrhstep).ln();
        let assign17200_e16741: f64 = (assign17200_e16738 * assign17200_e16740);
        let assign17200_e16744: f64 = (1.0 - var_wsrhstep);
        let assign17200_e16745: f64 = (assign17200_e16741 / assign17200_e16744);
        let assign17200_e16747: f64 = (assign17200_e16745 + var_wsrhstep);
        let assign17200_e16751: f64 = (2.0 * p.p848);
        let assign17200_e16752: f64 = (1.0 - assign17200_e16751);
        let assign17200_e16753: f64 = (assign17200_e16747 * assign17200_e16752);
        (assign17200_e16753,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign17200_e16755;

        let (assign17210_e16769,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) {
        let assign17210_e16767: f64 = (var_wsrhstep + var_dwsrh);
        (assign17210_e16767,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign17210_e16769;

        let assign17220_e16772: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard305 = assign17220_e16772;

        let (assign17230_e16789, assign17230_e16789_d_n6, assign17230_e16789_d_n7, assign17230_e16789_d_n8, assign17230_e16789_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) && (var_guard305 != 0.0)) {
        let assign17230_e16786: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign17230_e16787: f64 = (assign17230_e16786).sqrt();
        (assign17230_e16787, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17230_e16789;
        var_tmp_dn6 = assign17230_e16789_d_n6;
        var_tmp_dn7 = assign17230_e16789_d_n7;
        var_tmp_dn8 = assign17230_e16789_d_n8;
        var_tmp_dn9 = assign17230_e16789_d_n9;

        let (assign17240_e16808, assign17240_e16808_d_n6, assign17240_e16808_d_n7, assign17240_e16808_d_n8, assign17240_e16808_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) && (var_guard305 == 0.0)) {
        let assign17240_e16804: f64 = (var_vbi_minus_vjsrh * var_vbirbotinv);
        let assign17240_e16806: f64 = (assign17240_e16804).powf(p.p848);
        (assign17240_e16806, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17240_e16808;
        var_tmp_dn6 = assign17240_e16808_d_n6;
        var_tmp_dn7 = assign17240_e16808_d_n7;
        var_tmp_dn8 = assign17240_e16808_d_n8;
        var_tmp_dn9 = assign17240_e16808_d_n9;

        let (assign17250_e16822, assign17250_e16822_d_n6, assign17250_e16822_d_n7, assign17250_e16822_d_n8, assign17250_e16822_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) {
        let assign17250_e16820: f64 = (var_wdepnulrbot * var_tmp);
        (assign17250_e16820, (var_wdepnulrbot * var_tmp_dn6), (var_wdepnulrbot * var_tmp_dn7), (var_wdepnulrbot * var_tmp_dn8), (var_wdepnulrbot * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign17250_e16822;
        var_wdep_dn6 = assign17250_e16822_d_n6;
        var_wdep_dn7 = assign17250_e16822_d_n7;
        var_wdep_dn8 = assign17250_e16822_d_n8;
        var_wdep_dn9 = assign17250_e16822_d_n9;

        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard297_slot = var_guard297;
        *var_guard298_slot = var_guard298;
        *var_guard299_slot = var_guard299;
        *var_guard300_slot = var_guard300;
        *var_guard301_slot = var_guard301;
        *var_guard302_slot = var_guard302;
        *var_guard303_slot = var_guard303;
        *var_guard304_slot = var_guard304;
        *var_guard305_slot = var_guard305;
        *var_i1_slot = var_i1;
        *var_i1_dn6_slot = var_i1_dn6;
        *var_i1_dn7_slot = var_i1_dn7;
        *var_i1_dn8_slot = var_i1_dn8;
        *var_i1_dn9_slot = var_i1_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_idmult_slot = var_idmult;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_two_psistar_slot = var_two_psistar;
        *var_vav_slot = var_vav;
        *var_vbbt_slot = var_vbbt;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_vjlim_slot = var_vjlim;
        *var_vjsrh_slot = var_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
        *var_z_slot = var_z;
        *var_zinv_slot = var_zinv;
    }

    pub(super) fn stamp_transient_block_28(
        p: &Parameters,
        var_atatbot: f64,
        var_berfc: f64,
        var_btatpartbot: f64,
        var_cerfc: f64,
        var_ftdbot: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard302: f64,
        var_guard303: f64,
        var_one_minus_pbot: f64,
        var_one_over_one_minus_pbot: f64,
        var_perfc: f64,
        var_vbi_minus_vjsrh: f64,
        var_wdep: f64,
        var_wdep_dn6: f64,
        var_wdep_dn7: f64,
        var_wdep_dn8: f64,
        var_wdep_dn9: f64,
        var_wsrh: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard306_slot: &mut f64,
        var_guard307_slot: &mut f64,
        var_guard308_slot: &mut f64,
        var_guard309_slot: &mut f64,
        var_guard310_slot: &mut f64,
        var_guard311_slot: &mut f64,
        var_guard312_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard306: f64 = *var_guard306_slot;
        let mut var_guard307: f64 = *var_guard307_slot;
        let mut var_guard308: f64 = *var_guard308_slot;
        let mut var_guard309: f64 = *var_guard309_slot;
        let mut var_guard310: f64 = *var_guard310_slot;
        let mut var_guard311: f64 = *var_guard311_slot;
        let mut var_guard312: f64 = *var_guard312_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign17260_e16840, assign17260_e16840_d_n6, assign17260_e16840_d_n7, assign17260_e16840_d_n8, assign17260_e16840_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) {
        let assign17260_e16835: f64 = (var_zinv - 1.0);
        let assign17260_e16837: f64 = (assign17260_e16835 * var_wdep);
        let assign17260_e16838: f64 = (var_ftdbot * assign17260_e16837);
        (assign17260_e16838, (var_ftdbot * (assign17260_e16835 * var_wdep_dn6)), (var_ftdbot * (assign17260_e16835 * var_wdep_dn7)), (var_ftdbot * (assign17260_e16835 * var_wdep_dn8)), (var_ftdbot * (assign17260_e16835 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign17260_e16840;
        var_asrh_dn6 = assign17260_e16840_d_n6;
        var_asrh_dn7 = assign17260_e16840_d_n7;
        var_asrh_dn8 = assign17260_e16840_d_n8;
        var_asrh_dn9 = assign17260_e16840_d_n9;

        let (assign17270_e16856, assign17270_e16856_d_n6, assign17270_e16856_d_n7, assign17270_e16856_d_n8, assign17270_e16856_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard303 == 0.0)) {
        let assign17270_e16853: f64 = (var_asrh * var_wsrh);
        let assign17270_e16854: f64 = (p.p857 * assign17270_e16853);
        (assign17270_e16854, (p.p857 * (var_asrh_dn6 * var_wsrh)), (p.p857 * (var_asrh_dn7 * var_wsrh)), (p.p857 * (var_asrh_dn8 * var_wsrh)), (p.p857 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign17270_e16856;
        var_isrh_dn6 = assign17270_e16856_d_n6;
        var_isrh_dn7 = assign17270_e16856_d_n7;
        var_isrh_dn8 = assign17270_e16856_d_n8;
        var_isrh_dn9 = assign17270_e16856_d_n9;

        let assign17280_e16859: f64 = if p.p862 == 0.0 { 1.0 } else { 0.0 };
        var_guard306 = assign17280_e16859;

        let (assign17290_e16870, assign17290_e16870_d_n6, assign17290_e16870_d_n7, assign17290_e16870_d_n8, assign17290_e16870_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign17290_e16870;
        var_itat_dn6 = assign17290_e16870_d_n6;
        var_itat_dn7 = assign17290_e16870_d_n7;
        var_itat_dn8 = assign17290_e16870_d_n8;
        var_itat_dn9 = assign17290_e16870_d_n9;

        let (assign17300_e16888, assign17300_e16888_d_n6, assign17300_e16888_d_n7, assign17300_e16888_d_n8, assign17300_e16888_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17300_e16883: f64 = (var_wdep * var_one_minus_pbot);
        let assign17300_e16885: f64 = (assign17300_e16883 / var_vbi_minus_vjsrh);
        let assign17300_e16886: f64 = (var_btatpartbot * assign17300_e16885);
        (assign17300_e16886, (var_btatpartbot * ((var_wdep_dn6 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn7 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn8 * var_one_minus_pbot) / var_vbi_minus_vjsrh)), (var_btatpartbot * ((var_wdep_dn9 * var_one_minus_pbot) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign17300_e16888;
        var_btat_dn6 = assign17300_e16888_d_n6;
        var_btat_dn7 = assign17300_e16888_d_n7;
        var_btat_dn8 = assign17300_e16888_d_n8;
        var_btat_dn9 = assign17300_e16888_d_n9;

        let (assign17310_e16904, assign17310_e16904_d_n6, assign17310_e16904_d_n7, assign17310_e16904_d_n8, assign17310_e16904_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17310_e16900: f64 = (0.666666666666667 * var_atatbot);
        let assign17310_e16902: f64 = (assign17310_e16900 / var_btat);
        (assign17310_e16902, (-((assign17310_e16900 * var_btat_dn6) / (var_btat * var_btat))), (-((assign17310_e16900 * var_btat_dn7) / (var_btat * var_btat))), (-((assign17310_e16900 * var_btat_dn8) / (var_btat * var_btat))), (-((assign17310_e16900 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign17310_e16904;
        var_twoatatoverthreebtat_dn6 = assign17310_e16904_d_n6;
        var_twoatatoverthreebtat_dn7 = assign17310_e16904_d_n7;
        var_twoatatoverthreebtat_dn8 = assign17310_e16904_d_n8;
        var_twoatatoverthreebtat_dn9 = assign17310_e16904_d_n9;

        let (assign17320_e16918, assign17320_e16918_d_n6, assign17320_e16918_d_n7, assign17320_e16918_d_n8, assign17320_e16918_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17320_e16916: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign17320_e16916, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign17320_e16918;
        var_umaxbeforelimiting_dn6 = assign17320_e16918_d_n6;
        var_umaxbeforelimiting_dn7 = assign17320_e16918_d_n7;
        var_umaxbeforelimiting_dn8 = assign17320_e16918_d_n8;
        var_umaxbeforelimiting_dn9 = assign17320_e16918_d_n9;

        let (assign17330_e16939, assign17330_e16939_d_n6, assign17330_e16939_d_n7, assign17330_e16939_d_n8, assign17330_e16939_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17330_e16930: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign17330_e16933: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign17330_e16935: f64 = (assign17330_e16933 + 1.0);
        let assign17330_e16936: f64 = (assign17330_e16930 / assign17330_e16935);
        let assign17330_e16937: f64 = (assign17330_e16936).sqrt();
        (assign17330_e16937, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign17330_e16935) - (assign17330_e16930 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign17330_e16935 * assign17330_e16935)) / (2.0 * assign17330_e16937)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign17330_e16935) - (assign17330_e16930 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign17330_e16935 * assign17330_e16935)) / (2.0 * assign17330_e16937)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign17330_e16935) - (assign17330_e16930 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign17330_e16935 * assign17330_e16935)) / (2.0 * assign17330_e16937)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign17330_e16935) - (assign17330_e16930 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign17330_e16935 * assign17330_e16935)) / (2.0 * assign17330_e16937)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign17330_e16939;
        var_umax_dn6 = assign17330_e16939_d_n6;
        var_umax_dn7 = assign17330_e16939_d_n7;
        var_umax_dn8 = assign17330_e16939_d_n8;
        var_umax_dn9 = assign17330_e16939_d_n9;

        let (assign17340_e16952, assign17340_e16952_d_n6, assign17340_e16952_d_n7, assign17340_e16952_d_n8, assign17340_e16952_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17340_e16950: f64 = (var_umax).sqrt();
        (assign17340_e16950, (var_umax_dn6 / (2.0 * assign17340_e16950)), (var_umax_dn7 / (2.0 * assign17340_e16950)), (var_umax_dn8 / (2.0 * assign17340_e16950)), (var_umax_dn9 / (2.0 * assign17340_e16950)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign17340_e16952;
        var_sqrtumax_dn6 = assign17340_e16952_d_n6;
        var_sqrtumax_dn7 = assign17340_e16952_d_n7;
        var_sqrtumax_dn8 = assign17340_e16952_d_n8;
        var_sqrtumax_dn9 = assign17340_e16952_d_n9;

        let (assign17350_e16966, assign17350_e16966_d_n6, assign17350_e16966_d_n7, assign17350_e16966_d_n8, assign17350_e16966_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17350_e16964: f64 = (var_umax * var_sqrtumax);
        (assign17350_e16964, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign17350_e16966;
        var_umaxpoweronepointfive_dn6 = assign17350_e16966_d_n6;
        var_umaxpoweronepointfive_dn7 = assign17350_e16966_d_n7;
        var_umaxpoweronepointfive_dn8 = assign17350_e16966_d_n8;
        var_umaxpoweronepointfive_dn9 = assign17350_e16966_d_n9;

        let assign17360_e16968: f64 = (-p.p848);
        let assign17360_e16970: f64 = (assign17360_e16968 * var_one_over_one_minus_pbot);
        let assign17360_e16972: f64 = (-1.0);
        let assign17360_e16973: f64 = if assign17360_e16970 == assign17360_e16972 { 1.0 } else { 0.0 };
        var_guard307 = assign17360_e16973;

        let (assign17370_e16993, assign17370_e16993_d_n6, assign17370_e16993_d_n7, assign17370_e16993_d_n8, assign17370_e16993_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard307 != 0.0)) {
        let assign17370_e16989: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign17370_e16990: f64 = (1.0 + assign17370_e16989);
        let assign17370_e16991: f64 = (1.0 / assign17370_e16990);
        (assign17370_e16991, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign17370_e16990 * assign17370_e16990))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign17370_e16990 * assign17370_e16990))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign17370_e16990 * assign17370_e16990))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign17370_e16990 * assign17370_e16990))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign17370_e16993;
        var_wgamma_dn6 = assign17370_e16993_d_n6;
        var_wgamma_dn7 = assign17370_e16993_d_n7;
        var_wgamma_dn8 = assign17370_e16993_d_n8;
        var_wgamma_dn9 = assign17370_e16993_d_n9;

        let (assign17380_e17017, assign17380_e17017_d_n6, assign17380_e17017_d_n7, assign17380_e17017_d_n8, assign17380_e17017_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard307 == 0.0)) {
        let assign17380_e17009: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign17380_e17010: f64 = (1.0 + assign17380_e17009);
        let assign17380_e17012: f64 = (-p.p848);
        let assign17380_e17014: f64 = (assign17380_e17012 * var_one_over_one_minus_pbot);
        let assign17380_e17015: f64 = (assign17380_e17010).powf(assign17380_e17014);
        (assign17380_e17015, if 0.0 == 0.0 && ((assign17380_e17014) as f64).is_finite() && ((assign17380_e17014) as f64).fract() == 0.0 { if assign17380_e17014 == 0.0 { 0.0 } else { (assign17380_e17014 * ((assign17380_e17010).powf(assign17380_e17014 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign17380_e17015 * (assign17380_e17014 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign17380_e17010))) }, if 0.0 == 0.0 && ((assign17380_e17014) as f64).is_finite() && ((assign17380_e17014) as f64).fract() == 0.0 { if assign17380_e17014 == 0.0 { 0.0 } else { (assign17380_e17014 * ((assign17380_e17010).powf(assign17380_e17014 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign17380_e17015 * (assign17380_e17014 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign17380_e17010))) }, if 0.0 == 0.0 && ((assign17380_e17014) as f64).is_finite() && ((assign17380_e17014) as f64).fract() == 0.0 { if assign17380_e17014 == 0.0 { 0.0 } else { (assign17380_e17014 * ((assign17380_e17010).powf(assign17380_e17014 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign17380_e17015 * (assign17380_e17014 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign17380_e17010))) }, if 0.0 == 0.0 && ((assign17380_e17014) as f64).is_finite() && ((assign17380_e17014) as f64).fract() == 0.0 { if assign17380_e17014 == 0.0 { 0.0 } else { (assign17380_e17014 * ((assign17380_e17010).powf(assign17380_e17014 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign17380_e17015 * (assign17380_e17014 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign17380_e17010))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign17380_e17017;
        var_wgamma_dn6 = assign17380_e17017_d_n6;
        var_wgamma_dn7 = assign17380_e17017_d_n7;
        var_wgamma_dn8 = assign17380_e17017_d_n8;
        var_wgamma_dn9 = assign17380_e17017_d_n9;

        let (assign17390_e17035, assign17390_e17035_d_n6, assign17390_e17035_d_n7, assign17390_e17035_d_n8, assign17390_e17035_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17390_e17029: f64 = (var_wsrh * var_wgamma);
        let assign17390_e17032: f64 = (var_wsrh + var_wgamma);
        let assign17390_e17033: f64 = (assign17390_e17029 / assign17390_e17032);
        (assign17390_e17033, ((((var_wsrh * var_wgamma_dn6) * assign17390_e17032) - (assign17390_e17029 * var_wgamma_dn6)) / (assign17390_e17032 * assign17390_e17032)), ((((var_wsrh * var_wgamma_dn7) * assign17390_e17032) - (assign17390_e17029 * var_wgamma_dn7)) / (assign17390_e17032 * assign17390_e17032)), ((((var_wsrh * var_wgamma_dn8) * assign17390_e17032) - (assign17390_e17029 * var_wgamma_dn8)) / (assign17390_e17032 * assign17390_e17032)), ((((var_wsrh * var_wgamma_dn9) * assign17390_e17032) - (assign17390_e17029 * var_wgamma_dn9)) / (assign17390_e17032 * assign17390_e17032)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign17390_e17035;
        var_wtat_dn6 = assign17390_e17035_d_n6;
        var_wtat_dn7 = assign17390_e17035_d_n7;
        var_wtat_dn8 = assign17390_e17035_d_n8;
        var_wtat_dn9 = assign17390_e17035_d_n9;

        let (assign17400_e17052, assign17400_e17052_d_n6, assign17400_e17052_d_n7, assign17400_e17052_d_n8, assign17400_e17052_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17400_e17048: f64 = (var_btat / var_sqrtumax);
        let assign17400_e17049: f64 = (0.375 * assign17400_e17048);
        let assign17400_e17050: f64 = (assign17400_e17049).sqrt();
        (assign17400_e17050, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17400_e17050)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17400_e17050)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17400_e17050)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign17400_e17050)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign17400_e17052;
        var_ktat_dn6 = assign17400_e17052_d_n6;
        var_ktat_dn7 = assign17400_e17052_d_n7;
        var_ktat_dn8 = assign17400_e17052_d_n8;
        var_ktat_dn9 = assign17400_e17052_d_n9;

        let (assign17410_e17070, assign17410_e17070_d_n6, assign17410_e17070_d_n7, assign17410_e17070_d_n8, assign17410_e17070_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17410_e17065: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign17410_e17066: f64 = (2.0 * assign17410_e17065);
        let assign17410_e17068: f64 = (assign17410_e17066 - var_umax);
        (assign17410_e17068, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign17410_e17070;
        var_ltat_dn6 = assign17410_e17070_d_n6;
        var_ltat_dn7 = assign17410_e17070_d_n7;
        var_ltat_dn8 = assign17410_e17070_d_n8;
        var_ltat_dn9 = assign17410_e17070_d_n9;

        let (assign17420_e17096, assign17420_e17096_d_n6, assign17420_e17096_d_n7, assign17420_e17096_d_n8, assign17420_e17096_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17420_e17082: f64 = (var_atatbot * var_twoatatoverthreebtat);
        let assign17420_e17084: f64 = (assign17420_e17082 * var_sqrtumax);
        let assign17420_e17087: f64 = (var_atatbot * var_umax);
        let assign17420_e17088: f64 = (assign17420_e17084 - assign17420_e17087);
        let assign17420_e17092: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign17420_e17093: f64 = (0.5 * assign17420_e17092);
        let assign17420_e17094: f64 = (assign17420_e17088 + assign17420_e17093);
        (assign17420_e17094, (((((var_atatbot * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign17420_e17082 * var_sqrtumax_dn6)) - (var_atatbot * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatbot * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign17420_e17082 * var_sqrtumax_dn7)) - (var_atatbot * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatbot * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign17420_e17082 * var_sqrtumax_dn8)) - (var_atatbot * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatbot * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign17420_e17082 * var_sqrtumax_dn9)) - (var_atatbot * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign17420_e17096;
        var_mtat_dn6 = assign17420_e17096_d_n6;
        var_mtat_dn7 = assign17420_e17096_d_n7;
        var_mtat_dn8 = assign17420_e17096_d_n8;
        var_mtat_dn9 = assign17420_e17096_d_n9;

        let (assign17430_e17112, assign17430_e17112_d_n6, assign17430_e17112_d_n7, assign17430_e17112_d_n8, assign17430_e17112_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17430_e17108: f64 = (var_ltat - 1.0);
        let assign17430_e17110: f64 = (assign17430_e17108 * var_ktat);
        (assign17430_e17110, ((var_ltat_dn6 * var_ktat) + (assign17430_e17108 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign17430_e17108 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign17430_e17108 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign17430_e17108 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign17430_e17112;
        var_xerfc_dn6 = assign17430_e17112_d_n6;
        var_xerfc_dn7 = assign17430_e17112_d_n7;
        var_xerfc_dn8 = assign17430_e17112_d_n8;
        var_xerfc_dn9 = assign17430_e17112_d_n9;

        let (assign17440_e17126, assign17440_e17126_d_n6, assign17440_e17126_d_n7, assign17440_e17126_d_n8, assign17440_e17126_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17440_e17124: f64 = (var_xerfc * var_xerfc);
        (assign17440_e17124, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign17440_e17126;
        var_ysq_dn6 = assign17440_e17126_d_n6;
        var_ysq_dn7 = assign17440_e17126_d_n7;
        var_ysq_dn8 = assign17440_e17126_d_n8;
        var_ysq_dn9 = assign17440_e17126_d_n9;

        let assign17450_e17129: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard308 = assign17450_e17129;

        let (assign17460_e17149, assign17460_e17149_d_n6, assign17460_e17149_d_n7, assign17460_e17149_d_n8, assign17460_e17149_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard308 != 0.0)) {
        let assign17460_e17145: f64 = (var_perfc * var_xerfc);
        let assign17460_e17146: f64 = (1.0 + assign17460_e17145);
        let assign17460_e17147: f64 = (1.0 / assign17460_e17146);
        (assign17460_e17147, (-((var_perfc * var_xerfc_dn6) / (assign17460_e17146 * assign17460_e17146))), (-((var_perfc * var_xerfc_dn7) / (assign17460_e17146 * assign17460_e17146))), (-((var_perfc * var_xerfc_dn8) / (assign17460_e17146 * assign17460_e17146))), (-((var_perfc * var_xerfc_dn9) / (assign17460_e17146 * assign17460_e17146))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign17460_e17149;
        var_terfc_dn6 = assign17460_e17149_d_n6;
        var_terfc_dn7 = assign17460_e17149_d_n7;
        var_terfc_dn8 = assign17460_e17149_d_n8;
        var_terfc_dn9 = assign17460_e17149_d_n9;

        let (assign17470_e17170, assign17470_e17170_d_n6, assign17470_e17170_d_n7, assign17470_e17170_d_n8, assign17470_e17170_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard308 == 0.0)) {
        let assign17470_e17166: f64 = (var_perfc * var_xerfc);
        let assign17470_e17167: f64 = (1.0 - assign17470_e17166);
        let assign17470_e17168: f64 = (1.0 / assign17470_e17167);
        (assign17470_e17168, (-((-(var_perfc * var_xerfc_dn6)) / (assign17470_e17167 * assign17470_e17167))), (-((-(var_perfc * var_xerfc_dn7)) / (assign17470_e17167 * assign17470_e17167))), (-((-(var_perfc * var_xerfc_dn8)) / (assign17470_e17167 * assign17470_e17167))), (-((-(var_perfc * var_xerfc_dn9)) / (assign17470_e17167 * assign17470_e17167))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign17470_e17170;
        var_terfc_dn6 = assign17470_e17170_d_n6;
        var_terfc_dn7 = assign17470_e17170_d_n7;
        var_terfc_dn8 = assign17470_e17170_d_n8;
        var_terfc_dn9 = assign17470_e17170_d_n9;

        let assign17480_e17172: f64 = (-var_ysq);
        let assign17480_e17174: f64 = (assign17480_e17172 + var_mtat);
        let assign17480_e17176: f64 = (-230.25850929940458);
        let assign17480_e17177: f64 = if assign17480_e17174 > assign17480_e17176 { 1.0 } else { 0.0 };
        var_guard309 = assign17480_e17177;

        let (assign17490_e17195, assign17490_e17195_d_n6, assign17490_e17195_d_n7, assign17490_e17195_d_n8, assign17490_e17195_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard309 != 0.0)) {
        let assign17490_e17190: f64 = (-var_ysq);
        let assign17490_e17192: f64 = (assign17490_e17190 + var_mtat);
        let assign17490_e17193: f64 = (assign17490_e17192).exp();
        (assign17490_e17193, (assign17490_e17193 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign17490_e17193 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign17490_e17193 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign17490_e17193 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17490_e17195;
        var_tmp_dn6 = assign17490_e17195_d_n6;
        var_tmp_dn7 = assign17490_e17195_d_n7;
        var_tmp_dn8 = assign17490_e17195_d_n8;
        var_tmp_dn9 = assign17490_e17195_d_n9;

        let (assign17500_e17244, assign17500_e17244_d_n6, assign17500_e17244_d_n7, assign17500_e17244_d_n8, assign17500_e17244_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard309 == 0.0)) {
        let assign17500_e17211: f64 = (-230.25850929940458);
        let assign17500_e17213: f64 = (-var_ysq);
        let assign17500_e17215: f64 = (assign17500_e17213 + var_mtat);
        let assign17500_e17216: f64 = (assign17500_e17211 - assign17500_e17215);
        let assign17500_e17220: f64 = (-230.25850929940458);
        let assign17500_e17222: f64 = (-var_ysq);
        let assign17500_e17224: f64 = (assign17500_e17222 + var_mtat);
        let assign17500_e17225: f64 = (assign17500_e17220 - assign17500_e17224);
        let assign17500_e17228: f64 = (-230.25850929940458);
        let assign17500_e17230: f64 = (-var_ysq);
        let assign17500_e17232: f64 = (assign17500_e17230 + var_mtat);
        let assign17500_e17233: f64 = (assign17500_e17228 - assign17500_e17232);
        let assign17500_e17235: f64 = (assign17500_e17233 * 0.3333333333333333);
        let assign17500_e17236: f64 = (1.0 + assign17500_e17235);
        let assign17500_e17237: f64 = (assign17500_e17225 * assign17500_e17236);
        let assign17500_e17238: f64 = (0.5 * assign17500_e17237);
        let assign17500_e17239: f64 = (1.0 + assign17500_e17238);
        let assign17500_e17240: f64 = (assign17500_e17216 * assign17500_e17239);
        let assign17500_e17241: f64 = (1.0 + assign17500_e17240);
        let assign17500_e17242: f64 = (1e-100 / assign17500_e17241);
        (assign17500_e17242, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign17500_e17239) + (assign17500_e17216 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign17500_e17236) + (assign17500_e17225 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign17500_e17241 * assign17500_e17241))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign17500_e17239) + (assign17500_e17216 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign17500_e17236) + (assign17500_e17225 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign17500_e17241 * assign17500_e17241))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign17500_e17239) + (assign17500_e17216 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign17500_e17236) + (assign17500_e17225 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign17500_e17241 * assign17500_e17241))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign17500_e17239) + (assign17500_e17216 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign17500_e17236) + (assign17500_e17225 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign17500_e17241 * assign17500_e17241))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17500_e17244;
        var_tmp_dn6 = assign17500_e17244_d_n6;
        var_tmp_dn7 = assign17500_e17244_d_n7;
        var_tmp_dn8 = assign17500_e17244_d_n8;
        var_tmp_dn9 = assign17500_e17244_d_n9;

        let (assign17510_e17274, assign17510_e17274_d_n6, assign17510_e17274_d_n7, assign17510_e17274_d_n8, assign17510_e17274_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17510_e17256: f64 = (0.29214664 * var_terfc);
        let assign17510_e17260: f64 = (var_terfc * var_terfc);
        let assign17510_e17261: f64 = (var_berfc * assign17510_e17260);
        let assign17510_e17262: f64 = (assign17510_e17256 + assign17510_e17261);
        let assign17510_e17266: f64 = (var_terfc * var_terfc);
        let assign17510_e17268: f64 = (assign17510_e17266 * var_terfc);
        let assign17510_e17269: f64 = (var_cerfc * assign17510_e17268);
        let assign17510_e17270: f64 = (assign17510_e17262 + assign17510_e17269);
        let assign17510_e17272: f64 = (assign17510_e17270 * var_tmp);
        (assign17510_e17272, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign17510_e17266 * var_terfc_dn6)))) * var_tmp) + (assign17510_e17270 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign17510_e17266 * var_terfc_dn7)))) * var_tmp) + (assign17510_e17270 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign17510_e17266 * var_terfc_dn8)))) * var_tmp) + (assign17510_e17270 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign17510_e17266 * var_terfc_dn9)))) * var_tmp) + (assign17510_e17270 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign17510_e17274;
        var_erfcpos_dn6 = assign17510_e17274_d_n6;
        var_erfcpos_dn7 = assign17510_e17274_d_n7;
        var_erfcpos_dn8 = assign17510_e17274_d_n8;
        var_erfcpos_dn9 = assign17510_e17274_d_n9;

        let assign17520_e17277: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard310 = assign17520_e17277;

        let (assign17530_e17291, assign17530_e17291_d_n6, assign17530_e17291_d_n7, assign17530_e17291_d_n8, assign17530_e17291_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard310 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign17530_e17291;
        var_erfctimesexpmtat_dn6 = assign17530_e17291_d_n6;
        var_erfctimesexpmtat_dn7 = assign17530_e17291_d_n7;
        var_erfctimesexpmtat_dn8 = assign17530_e17291_d_n8;
        var_erfctimesexpmtat_dn9 = assign17530_e17291_d_n9;

        let assign17540_e17294: f64 = (-230.25850929940458);
        let assign17540_e17295: f64 = if var_mtat > assign17540_e17294 { 1.0 } else { 0.0 };
        var_guard311 = assign17540_e17295;

        let (assign17550_e17313, assign17550_e17313_d_n6, assign17550_e17313_d_n7, assign17550_e17313_d_n8, assign17550_e17313_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard310 == 0.0)) && (var_guard311 != 0.0)) {
        let assign17550_e17311: f64 = (var_mtat).exp();
        (assign17550_e17311, (assign17550_e17311 * var_mtat_dn6), (assign17550_e17311 * var_mtat_dn7), (assign17550_e17311 * var_mtat_dn8), (assign17550_e17311 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17550_e17313;
        var_tmp_dn6 = assign17550_e17313_d_n6;
        var_tmp_dn7 = assign17550_e17313_d_n7;
        var_tmp_dn8 = assign17550_e17313_d_n8;
        var_tmp_dn9 = assign17550_e17313_d_n9;

        let (assign17560_e17356, assign17560_e17356_d_n6, assign17560_e17356_d_n7, assign17560_e17356_d_n8, assign17560_e17356_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard310 == 0.0)) && (var_guard311 == 0.0)) {
        let assign17560_e17332: f64 = (-230.25850929940458);
        let assign17560_e17334: f64 = (assign17560_e17332 - var_mtat);
        let assign17560_e17338: f64 = (-230.25850929940458);
        let assign17560_e17340: f64 = (assign17560_e17338 - var_mtat);
        let assign17560_e17343: f64 = (-230.25850929940458);
        let assign17560_e17345: f64 = (assign17560_e17343 - var_mtat);
        let assign17560_e17347: f64 = (assign17560_e17345 * 0.3333333333333333);
        let assign17560_e17348: f64 = (1.0 + assign17560_e17347);
        let assign17560_e17349: f64 = (assign17560_e17340 * assign17560_e17348);
        let assign17560_e17350: f64 = (0.5 * assign17560_e17349);
        let assign17560_e17351: f64 = (1.0 + assign17560_e17350);
        let assign17560_e17352: f64 = (assign17560_e17334 * assign17560_e17351);
        let assign17560_e17353: f64 = (1.0 + assign17560_e17352);
        let assign17560_e17354: f64 = (1e-100 / assign17560_e17353);
        (assign17560_e17354, (-((1e-100 * (((-var_mtat_dn6) * assign17560_e17351) + (assign17560_e17334 * (0.5 * (((-var_mtat_dn6) * assign17560_e17348) + (assign17560_e17340 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign17560_e17353 * assign17560_e17353))), (-((1e-100 * (((-var_mtat_dn7) * assign17560_e17351) + (assign17560_e17334 * (0.5 * (((-var_mtat_dn7) * assign17560_e17348) + (assign17560_e17340 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign17560_e17353 * assign17560_e17353))), (-((1e-100 * (((-var_mtat_dn8) * assign17560_e17351) + (assign17560_e17334 * (0.5 * (((-var_mtat_dn8) * assign17560_e17348) + (assign17560_e17340 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign17560_e17353 * assign17560_e17353))), (-((1e-100 * (((-var_mtat_dn9) * assign17560_e17351) + (assign17560_e17334 * (0.5 * (((-var_mtat_dn9) * assign17560_e17348) + (assign17560_e17340 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign17560_e17353 * assign17560_e17353))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17560_e17356;
        var_tmp_dn6 = assign17560_e17356_d_n6;
        var_tmp_dn7 = assign17560_e17356_d_n7;
        var_tmp_dn8 = assign17560_e17356_d_n8;
        var_tmp_dn9 = assign17560_e17356_d_n9;

        let (assign17570_e17375, assign17570_e17375_d_n6, assign17570_e17375_d_n7, assign17570_e17375_d_n8, assign17570_e17375_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) && (var_guard310 == 0.0)) {
        let assign17570_e17371: f64 = (2.0 * var_tmp);
        let assign17570_e17373: f64 = (assign17570_e17371 - var_erfcpos);
        (assign17570_e17373, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign17570_e17375;
        var_erfctimesexpmtat_dn6 = assign17570_e17375_d_n6;
        var_erfctimesexpmtat_dn7 = assign17570_e17375_d_n7;
        var_erfctimesexpmtat_dn8 = assign17570_e17375_d_n8;
        var_erfctimesexpmtat_dn9 = assign17570_e17375_d_n9;

        let (assign17580_e17395, assign17580_e17395_d_n6, assign17580_e17395_d_n7, assign17580_e17395_d_n8, assign17580_e17395_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17580_e17387: f64 = (1.772453850905516 * 0.5);
        let assign17580_e17390: f64 = (var_atatbot * var_erfctimesexpmtat);
        let assign17580_e17392: f64 = (assign17580_e17390 / var_ktat);
        let assign17580_e17393: f64 = (assign17580_e17387 * assign17580_e17392);
        (assign17580_e17393, (assign17580_e17387 * ((((var_atatbot * var_erfctimesexpmtat_dn6) * var_ktat) - (assign17580_e17390 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign17580_e17387 * ((((var_atatbot * var_erfctimesexpmtat_dn7) * var_ktat) - (assign17580_e17390 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign17580_e17387 * ((((var_atatbot * var_erfctimesexpmtat_dn8) * var_ktat) - (assign17580_e17390 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign17580_e17387 * ((((var_atatbot * var_erfctimesexpmtat_dn9) * var_ktat) - (assign17580_e17390 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign17580_e17395;
        var_gammamax_dn6 = assign17580_e17395_d_n6;
        var_gammamax_dn7 = assign17580_e17395_d_n7;
        var_gammamax_dn8 = assign17580_e17395_d_n8;
        var_gammamax_dn9 = assign17580_e17395_d_n9;

        let (assign17590_e17413, assign17590_e17413_d_n6, assign17590_e17413_d_n7, assign17590_e17413_d_n8, assign17590_e17413_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard306 == 0.0)) {
        let assign17590_e17408: f64 = (var_asrh * var_gammamax);
        let assign17590_e17410: f64 = (assign17590_e17408 * var_wtat);
        let assign17590_e17411: f64 = (p.p862 * assign17590_e17410);
        (assign17590_e17411, (p.p862 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign17590_e17408 * var_wtat_dn6))), (p.p862 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign17590_e17408 * var_wtat_dn7))), (p.p862 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign17590_e17408 * var_wtat_dn8))), (p.p862 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign17590_e17408 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign17590_e17413;
        var_itat_dn6 = assign17590_e17413_d_n6;
        var_itat_dn7 = assign17590_e17413_d_n7;
        var_itat_dn8 = assign17590_e17413_d_n8;
        var_itat_dn9 = assign17590_e17413_d_n9;

        let assign17600_e17416: f64 = if p.p868 == 0.0 { 1.0 } else { 0.0 };
        var_guard312 = assign17600_e17416;

        let (assign17610_e17427, assign17610_e17427_d_n6, assign17610_e17427_d_n7, assign17610_e17427_d_n8, assign17610_e17427_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign17610_e17427;
        var_ibbt_dn6 = assign17610_e17427_d_n6;
        var_ibbt_dn7 = assign17610_e17427_d_n7;
        var_ibbt_dn8 = assign17610_e17427_d_n8;
        var_ibbt_dn9 = assign17610_e17427_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard306_slot = var_guard306;
        *var_guard307_slot = var_guard307;
        *var_guard308_slot = var_guard308;
        *var_guard309_slot = var_guard309;
        *var_guard310_slot = var_guard310;
        *var_guard311_slot = var_guard311;
        *var_guard312_slot = var_guard312;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_29(
        p: &Parameters,
        var_alphaav: f64,
        var_atatsti: f64,
        var_btatpartsti: f64,
        var_fbbtbot: f64,
        var_fstopbot: f64,
        var_ftdsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard302: f64,
        var_guard312: f64,
        var_idmult: f64,
        var_idsatsti: f64,
        var_lssource_i: f64,
        var_one_minus_psti: f64,
        var_one_over_one_minus_pbot: f64,
        var_slopebot: f64,
        var_two_psistar: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbbt: f64,
        var_vbirbotinv: f64,
        var_vbirstiinv: f64,
        var_vbisti: f64,
        var_vbrinvbot: f64,
        var_vjsrh: f64,
        var_wdepnulrinvbot: f64,
        var_wdepnulrsti: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_guard313_slot: &mut f64,
        var_guard314_slot: &mut f64,
        var_guard315_slot: &mut f64,
        var_guard316_slot: &mut f64,
        var_guard317_slot: &mut f64,
        var_guard318_slot: &mut f64,
        var_guard319_slot: &mut f64,
        var_guard320_slot: &mut f64,
        var_guard321_slot: &mut f64,
        var_guard322_slot: &mut f64,
        var_guard323_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijunbot_slot: &mut f64,
        var_ijunbot_dn6_slot: &mut f64,
        var_ijunbot_dn7_slot: &mut f64,
        var_ijunbot_dn8_slot: &mut f64,
        var_ijunbot_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_guard313: f64 = *var_guard313_slot;
        let mut var_guard314: f64 = *var_guard314_slot;
        let mut var_guard315: f64 = *var_guard315_slot;
        let mut var_guard316: f64 = *var_guard316_slot;
        let mut var_guard317: f64 = *var_guard317_slot;
        let mut var_guard318: f64 = *var_guard318_slot;
        let mut var_guard319: f64 = *var_guard319_slot;
        let mut var_guard320: f64 = *var_guard320_slot;
        let mut var_guard321: f64 = *var_guard321_slot;
        let mut var_guard322: f64 = *var_guard322_slot;
        let mut var_guard323: f64 = *var_guard323_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijunbot: f64 = *var_ijunbot_slot;
        let mut var_ijunbot_dn6: f64 = *var_ijunbot_dn6_slot;
        let mut var_ijunbot_dn7: f64 = *var_ijunbot_dn7_slot;
        let mut var_ijunbot_dn8: f64 = *var_ijunbot_dn8_slot;
        let mut var_ijunbot_dn9: f64 = *var_ijunbot_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let assign17620_e17430: f64 = if p.p848 == 0.5 { 1.0 } else { 0.0 };
        var_guard313 = assign17620_e17430;

        let (assign17630_e17449, assign17630_e17449_d_n6, assign17630_e17449_d_n7, assign17630_e17449_d_n8, assign17630_e17449_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) && (var_guard313 != 0.0)) {
        let assign17630_e17444: f64 = (p.p845 - var_vbbt);
        let assign17630_e17446: f64 = (assign17630_e17444 * var_vbirbotinv);
        let assign17630_e17447: f64 = (assign17630_e17446).sqrt();
        (assign17630_e17447, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17630_e17449;
        var_tmp_dn6 = assign17630_e17449_d_n6;
        var_tmp_dn7 = assign17630_e17449_d_n7;
        var_tmp_dn8 = assign17630_e17449_d_n8;
        var_tmp_dn9 = assign17630_e17449_d_n9;

        let (assign17640_e17470, assign17640_e17470_d_n6, assign17640_e17470_d_n7, assign17640_e17470_d_n8, assign17640_e17470_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) && (var_guard313 == 0.0)) {
        let assign17640_e17464: f64 = (p.p845 - var_vbbt);
        let assign17640_e17466: f64 = (assign17640_e17464 * var_vbirbotinv);
        let assign17640_e17468: f64 = (assign17640_e17466).powf(p.p848);
        (assign17640_e17468, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17640_e17470;
        var_tmp_dn6 = assign17640_e17470_d_n6;
        var_tmp_dn7 = assign17640_e17470_d_n7;
        var_tmp_dn8 = assign17640_e17470_d_n8;
        var_tmp_dn9 = assign17640_e17470_d_n9;

        let (assign17650_e17490, assign17650_e17490_d_n6, assign17650_e17490_d_n7, assign17650_e17490_d_n8, assign17650_e17490_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) {
        let assign17650_e17483: f64 = (p.p845 - var_vbbt);
        let assign17650_e17485: f64 = (assign17650_e17483 * var_wdepnulrinvbot);
        let assign17650_e17487: f64 = (assign17650_e17485 / var_tmp);
        let assign17650_e17488: f64 = (var_one_over_one_minus_pbot * assign17650_e17487);
        (assign17650_e17488, (var_one_over_one_minus_pbot * (-((assign17650_e17485 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign17650_e17485 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign17650_e17485 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_pbot * (-((assign17650_e17485 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign17650_e17490;
        var_fmaxr_dn6 = assign17650_e17490_d_n6;
        var_fmaxr_dn7 = assign17650_e17490_d_n7;
        var_fmaxr_dn8 = assign17650_e17490_d_n8;
        var_fmaxr_dn9 = assign17650_e17490_d_n9;

        let assign17660_e17492: f64 = (-var_fbbtbot);
        let assign17660_e17494: f64 = (assign17660_e17492 / var_fmaxr);
        let assign17660_e17495: f64 = (assign17660_e17494).abs();
        let assign17660_e17497: f64 = if assign17660_e17495 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard314 = assign17660_e17497;

        let (assign17670_e17515, assign17670_e17515_d_n6, assign17670_e17515_d_n7, assign17670_e17515_d_n8, assign17670_e17515_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) && (var_guard314 != 0.0)) {
        let assign17670_e17510: f64 = (-var_fbbtbot);
        let assign17670_e17512: f64 = (assign17670_e17510 / var_fmaxr);
        let assign17670_e17513: f64 = (assign17670_e17512).exp();
        (assign17670_e17513, (assign17670_e17513 * (-((assign17670_e17510 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign17670_e17513 * (-((assign17670_e17510 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign17670_e17513 * (-((assign17670_e17510 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign17670_e17513 * (-((assign17670_e17510 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17670_e17515;
        var_tmp_dn6 = assign17670_e17515_d_n6;
        var_tmp_dn7 = assign17670_e17515_d_n7;
        var_tmp_dn8 = assign17670_e17515_d_n8;
        var_tmp_dn9 = assign17670_e17515_d_n9;

        let assign17680_e17517: f64 = (-var_fbbtbot);
        let assign17680_e17519: f64 = (assign17680_e17517 / var_fmaxr);
        let assign17680_e17521: f64 = if assign17680_e17519 < 0.0 { 1.0 } else { 0.0 };
        var_guard315 = assign17680_e17521;

        let (assign17690_e17572, assign17690_e17572_d_n6, assign17690_e17572_d_n7, assign17690_e17572_d_n8, assign17690_e17572_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) && (var_guard314 == 0.0)) && (var_guard315 != 0.0)) {
        let assign17690_e17539: f64 = (-230.25850929940458);
        let assign17690_e17541: f64 = (-var_fbbtbot);
        let assign17690_e17543: f64 = (assign17690_e17541 / var_fmaxr);
        let assign17690_e17544: f64 = (assign17690_e17539 - assign17690_e17543);
        let assign17690_e17548: f64 = (-230.25850929940458);
        let assign17690_e17550: f64 = (-var_fbbtbot);
        let assign17690_e17552: f64 = (assign17690_e17550 / var_fmaxr);
        let assign17690_e17553: f64 = (assign17690_e17548 - assign17690_e17552);
        let assign17690_e17556: f64 = (-230.25850929940458);
        let assign17690_e17558: f64 = (-var_fbbtbot);
        let assign17690_e17560: f64 = (assign17690_e17558 / var_fmaxr);
        let assign17690_e17561: f64 = (assign17690_e17556 - assign17690_e17560);
        let assign17690_e17563: f64 = (assign17690_e17561 * 0.3333333333333333);
        let assign17690_e17564: f64 = (1.0 + assign17690_e17563);
        let assign17690_e17565: f64 = (assign17690_e17553 * assign17690_e17564);
        let assign17690_e17566: f64 = (0.5 * assign17690_e17565);
        let assign17690_e17567: f64 = (1.0 + assign17690_e17566);
        let assign17690_e17568: f64 = (assign17690_e17544 * assign17690_e17567);
        let assign17690_e17569: f64 = (1.0 + assign17690_e17568);
        let assign17690_e17570: f64 = (1e-100 / assign17690_e17569);
        (assign17690_e17570, (-((1e-100 * (((-(-((assign17690_e17541 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign17690_e17567) + (assign17690_e17544 * (0.5 * (((-(-((assign17690_e17550 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign17690_e17564) + (assign17690_e17553 * ((-(-((assign17690_e17558 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17690_e17569 * assign17690_e17569))), (-((1e-100 * (((-(-((assign17690_e17541 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign17690_e17567) + (assign17690_e17544 * (0.5 * (((-(-((assign17690_e17550 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign17690_e17564) + (assign17690_e17553 * ((-(-((assign17690_e17558 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17690_e17569 * assign17690_e17569))), (-((1e-100 * (((-(-((assign17690_e17541 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign17690_e17567) + (assign17690_e17544 * (0.5 * (((-(-((assign17690_e17550 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign17690_e17564) + (assign17690_e17553 * ((-(-((assign17690_e17558 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17690_e17569 * assign17690_e17569))), (-((1e-100 * (((-(-((assign17690_e17541 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign17690_e17567) + (assign17690_e17544 * (0.5 * (((-(-((assign17690_e17550 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign17690_e17564) + (assign17690_e17553 * ((-(-((assign17690_e17558 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign17690_e17569 * assign17690_e17569))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17690_e17572;
        var_tmp_dn6 = assign17690_e17572_d_n6;
        var_tmp_dn7 = assign17690_e17572_d_n7;
        var_tmp_dn8 = assign17690_e17572_d_n8;
        var_tmp_dn9 = assign17690_e17572_d_n9;

        let (assign17700_e17621, assign17700_e17621_d_n6, assign17700_e17621_d_n7, assign17700_e17621_d_n8, assign17700_e17621_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) && (var_guard314 == 0.0)) && (var_guard315 == 0.0)) {
        let assign17700_e17591: f64 = (-var_fbbtbot);
        let assign17700_e17593: f64 = (assign17700_e17591 / var_fmaxr);
        let assign17700_e17595: f64 = (assign17700_e17593 - 230.25850929940458);
        let assign17700_e17599: f64 = (-var_fbbtbot);
        let assign17700_e17601: f64 = (assign17700_e17599 / var_fmaxr);
        let assign17700_e17603: f64 = (assign17700_e17601 - 230.25850929940458);
        let assign17700_e17606: f64 = (-var_fbbtbot);
        let assign17700_e17608: f64 = (assign17700_e17606 / var_fmaxr);
        let assign17700_e17610: f64 = (assign17700_e17608 - 230.25850929940458);
        let assign17700_e17612: f64 = (assign17700_e17610 * 0.3333333333333333);
        let assign17700_e17613: f64 = (1.0 + assign17700_e17612);
        let assign17700_e17614: f64 = (assign17700_e17603 * assign17700_e17613);
        let assign17700_e17615: f64 = (0.5 * assign17700_e17614);
        let assign17700_e17616: f64 = (1.0 + assign17700_e17615);
        let assign17700_e17617: f64 = (assign17700_e17595 * assign17700_e17616);
        let assign17700_e17618: f64 = (1.0 + assign17700_e17617);
        let assign17700_e17619: f64 = (1e100 * assign17700_e17618);
        (assign17700_e17619, (1e100 * (((-((assign17700_e17591 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign17700_e17616) + (assign17700_e17595 * (0.5 * (((-((assign17700_e17599 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign17700_e17613) + (assign17700_e17603 * ((-((assign17700_e17606 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17700_e17591 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign17700_e17616) + (assign17700_e17595 * (0.5 * (((-((assign17700_e17599 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign17700_e17613) + (assign17700_e17603 * ((-((assign17700_e17606 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17700_e17591 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign17700_e17616) + (assign17700_e17595 * (0.5 * (((-((assign17700_e17599 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign17700_e17613) + (assign17700_e17603 * ((-((assign17700_e17606 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign17700_e17591 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign17700_e17616) + (assign17700_e17595 * (0.5 * (((-((assign17700_e17599 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign17700_e17613) + (assign17700_e17603 * ((-((assign17700_e17606 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17700_e17621;
        var_tmp_dn6 = assign17700_e17621_d_n6;
        var_tmp_dn7 = assign17700_e17621_d_n7;
        var_tmp_dn8 = assign17700_e17621_d_n8;
        var_tmp_dn9 = assign17700_e17621_d_n9;

        let (assign17710_e17641, assign17710_e17641_d_n6, assign17710_e17641_d_n7, assign17710_e17641_d_n8, assign17710_e17641_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard312 == 0.0)) {
        let assign17710_e17634: f64 = (var_v2 * var_fmaxr);
        let assign17710_e17636: f64 = (assign17710_e17634 * var_fmaxr);
        let assign17710_e17638: f64 = (assign17710_e17636 * var_tmp);
        let assign17710_e17639: f64 = (p.p868 * assign17710_e17638);
        (assign17710_e17639, (p.p868 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign17710_e17634 * var_fmaxr_dn6)) * var_tmp) + (assign17710_e17636 * var_tmp_dn6))), (p.p868 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign17710_e17634 * var_fmaxr_dn7)) * var_tmp) + (assign17710_e17636 * var_tmp_dn7))), (p.p868 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign17710_e17634 * var_fmaxr_dn8)) * var_tmp) + (assign17710_e17636 * var_tmp_dn8))), (p.p868 * (((((var_v2 * var_fmaxr_dn9) * var_fmaxr) + (assign17710_e17634 * var_fmaxr_dn9)) * var_tmp) + (assign17710_e17636 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign17710_e17641;
        var_ibbt_dn6 = assign17710_e17641_d_n6;
        var_ibbt_dn7 = assign17710_e17641_d_n7;
        var_ibbt_dn8 = assign17710_e17641_d_n8;
        var_ibbt_dn9 = assign17710_e17641_d_n9;

        let assign17720_e17644: f64 = if p.p877 > 1000.0 { 1.0 } else { 0.0 };
        var_guard316 = assign17720_e17644;

        let (assign17730_e17655, assign17730_e17655_d_n6, assign17730_e17655_d_n7, assign17730_e17655_d_n8, assign17730_e17655_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard316 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign17730_e17655;
        var_fbreakdown_dn6 = assign17730_e17655_d_n6;
        var_fbreakdown_dn7 = assign17730_e17655_d_n7;
        var_fbreakdown_dn8 = assign17730_e17655_d_n8;
        var_fbreakdown_dn9 = assign17730_e17655_d_n9;

        let assign17740_e17658: f64 = (-var_alphaav);
        let assign17740_e17660: f64 = (assign17740_e17658 * p.p877);
        let assign17740_e17661: f64 = if var_vav > assign17740_e17660 { 1.0 } else { 0.0 };
        var_guard317 = assign17740_e17661;

        let assign17750_e17664: f64 = if p.p880 == 4.0 { 1.0 } else { 0.0 };
        var_guard318 = assign17750_e17664;

        let (assign17760_e17694, assign17760_e17694_d_n6, assign17760_e17694_d_n7, assign17760_e17694_d_n8, assign17760_e17694_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard316 == 0.0)) && (var_guard317 != 0.0)) && (var_guard318 != 0.0)) {
        let assign17760_e17680: f64 = (var_vav * var_vbrinvbot);
        let assign17760_e17683: f64 = (var_vav * var_vbrinvbot);
        let assign17760_e17684: f64 = (assign17760_e17680 * assign17760_e17683);
        let assign17760_e17687: f64 = (var_vav * var_vbrinvbot);
        let assign17760_e17688: f64 = (assign17760_e17684 * assign17760_e17687);
        let assign17760_e17691: f64 = (var_vav * var_vbrinvbot);
        let assign17760_e17692: f64 = (assign17760_e17688 * assign17760_e17691);
        (assign17760_e17692, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17760_e17694;
        var_tmp_dn6 = assign17760_e17694_d_n6;
        var_tmp_dn7 = assign17760_e17694_d_n7;
        var_tmp_dn8 = assign17760_e17694_d_n8;
        var_tmp_dn9 = assign17760_e17694_d_n9;

        let (assign17770_e17716, assign17770_e17716_d_n6, assign17770_e17716_d_n7, assign17770_e17716_d_n8, assign17770_e17716_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard316 == 0.0)) && (var_guard317 != 0.0)) && (var_guard318 == 0.0)) {
        let assign17770_e17711: f64 = (var_vav * var_vbrinvbot);
        let assign17770_e17712: f64 = (assign17770_e17711).abs();
        let assign17770_e17714: f64 = (assign17770_e17712).powf(p.p880);
        (assign17770_e17714, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17770_e17716;
        var_tmp_dn6 = assign17770_e17716_d_n6;
        var_tmp_dn7 = assign17770_e17716_d_n7;
        var_tmp_dn8 = assign17770_e17716_d_n8;
        var_tmp_dn9 = assign17770_e17716_d_n9;

        let (assign17780_e17734, assign17780_e17734_d_n6, assign17780_e17734_d_n7, assign17780_e17734_d_n8, assign17780_e17734_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard316 == 0.0)) && (var_guard317 != 0.0)) {
        let assign17780_e17731: f64 = (1.0 - var_tmp);
        let assign17780_e17732: f64 = (1.0 / assign17780_e17731);
        (assign17780_e17732, (-((-var_tmp_dn6) / (assign17780_e17731 * assign17780_e17731))), (-((-var_tmp_dn7) / (assign17780_e17731 * assign17780_e17731))), (-((-var_tmp_dn8) / (assign17780_e17731 * assign17780_e17731))), (-((-var_tmp_dn9) / (assign17780_e17731 * assign17780_e17731))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign17780_e17734;
        var_fbreakdown_dn6 = assign17780_e17734_d_n6;
        var_fbreakdown_dn7 = assign17780_e17734_d_n7;
        var_fbreakdown_dn8 = assign17780_e17734_d_n8;
        var_fbreakdown_dn9 = assign17780_e17734_d_n9;

        let (assign17790_e17757, assign17790_e17757_d_n6, assign17790_e17757_d_n7, assign17790_e17757_d_n8, assign17790_e17757_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) && (var_guard316 == 0.0)) && (var_guard317 == 0.0)) {
        let assign17790_e17751: f64 = (var_alphaav * p.p877);
        let assign17790_e17752: f64 = (var_vav + assign17790_e17751);
        let assign17790_e17754: f64 = (assign17790_e17752 * var_slopebot);
        let assign17790_e17755: f64 = (var_fstopbot + assign17790_e17754);
        (assign17790_e17755, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign17790_e17757;
        var_fbreakdown_dn6 = assign17790_e17757_d_n6;
        var_fbreakdown_dn7 = assign17790_e17757_d_n7;
        var_fbreakdown_dn8 = assign17790_e17757_d_n8;
        var_fbreakdown_dn9 = assign17790_e17757_d_n9;

        let (assign17800_e17776, assign17800_e17776_d_n6, assign17800_e17776_d_n7, assign17800_e17776_d_n8, assign17800_e17776_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard302 == 0.0)) {
        let assign17800_e17767: f64 = (var_id__blk212 + var_isrh);
        let assign17800_e17769: f64 = (assign17800_e17767 + var_itat);
        let assign17800_e17771: f64 = (assign17800_e17769 + var_ibbt);
        let assign17800_e17772: f64 = (p.p29 * assign17800_e17771);
        let assign17800_e17774: f64 = (assign17800_e17772 * var_fbreakdown);
        (assign17800_e17774, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign17800_e17772 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign17800_e17772 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign17800_e17772 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign17800_e17772 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunbot, var_ijunbot_dn6, var_ijunbot_dn7, var_ijunbot_dn8, var_ijunbot_dn9,)
    }
};
        var_ijunbot = assign17800_e17776;
        var_ijunbot_dn6 = assign17800_e17776_d_n6;
        var_ijunbot_dn7 = assign17800_e17776_d_n7;
        var_ijunbot_dn8 = assign17800_e17776_d_n8;
        var_ijunbot_dn9 = assign17800_e17776_d_n9;

        let assign17810_e17779: f64 = if var_lssource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard319 = assign17810_e17779;

        let (assign17820_e17787, assign17820_e17787_d_n6, assign17820_e17787_d_n7, assign17820_e17787_d_n8, assign17820_e17787_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign17820_e17787;
        var_ijunsti_dn6 = assign17820_e17787_d_n6;
        var_ijunsti_dn7 = assign17820_e17787_d_n7;
        var_ijunsti_dn8 = assign17820_e17787_d_n8;
        var_ijunsti_dn9 = assign17820_e17787_d_n9;

        let (assign17830_e17798,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) {
        let assign17830_e17796: f64 = (var_idsatsti * var_idmult);
        (assign17830_e17796,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign17830_e17798;

        let assign17840_e17805: f64 = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };
        var_guard320 = assign17840_e17805;

        let (assign17850_e17816, assign17850_e17816_d_n6, assign17850_e17816_d_n7, assign17850_e17816_d_n8, assign17850_e17816_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign17850_e17816;
        var_isrh_dn6 = assign17850_e17816_d_n6;
        var_isrh_dn7 = assign17850_e17816_d_n7;
        var_isrh_dn8 = assign17850_e17816_d_n8;
        var_isrh_dn9 = assign17850_e17816_d_n9;

        let (assign17860_e17830,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign17860_e17828: f64 = (var_vbisti - var_vjsrh);
        (assign17860_e17828,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign17860_e17830;

        let (assign17870_e17849,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign17870_e17844: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign17870_e17845: f64 = (1.0 - assign17870_e17844);
        let assign17870_e17846: f64 = (assign17870_e17845).sqrt();
        let assign17870_e17847: f64 = (1.0 - assign17870_e17846);
        (assign17870_e17847,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign17870_e17849;

        let assign17880_e17852: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard321 = assign17880_e17852;

        let (assign17890_e17866,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) && (var_guard321 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign17890_e17866;

        let (assign17900_e17898,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) && (var_guard321 == 0.0)) {
        let assign17900_e17881: f64 = (var_wsrhstep * var_wsrhstep);
        let assign17900_e17883: f64 = (var_wsrhstep).ln();
        let assign17900_e17884: f64 = (assign17900_e17881 * assign17900_e17883);
        let assign17900_e17887: f64 = (1.0 - var_wsrhstep);
        let assign17900_e17888: f64 = (assign17900_e17884 / assign17900_e17887);
        let assign17900_e17890: f64 = (assign17900_e17888 + var_wsrhstep);
        let assign17900_e17894: f64 = (2.0 * p.p849);
        let assign17900_e17895: f64 = (1.0 - assign17900_e17894);
        let assign17900_e17896: f64 = (assign17900_e17890 * assign17900_e17895);
        (assign17900_e17896,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign17900_e17898;

        let (assign17910_e17912,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign17910_e17910: f64 = (var_wsrhstep + var_dwsrh);
        (assign17910_e17910,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign17910_e17912;

        let assign17920_e17915: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard322 = assign17920_e17915;

        let (assign17930_e17932, assign17930_e17932_d_n6, assign17930_e17932_d_n7, assign17930_e17932_d_n8, assign17930_e17932_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) && (var_guard322 != 0.0)) {
        let assign17930_e17929: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign17930_e17930: f64 = (assign17930_e17929).sqrt();
        (assign17930_e17930, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17930_e17932;
        var_tmp_dn6 = assign17930_e17932_d_n6;
        var_tmp_dn7 = assign17930_e17932_d_n7;
        var_tmp_dn8 = assign17930_e17932_d_n8;
        var_tmp_dn9 = assign17930_e17932_d_n9;

        let (assign17940_e17951, assign17940_e17951_d_n6, assign17940_e17951_d_n7, assign17940_e17951_d_n8, assign17940_e17951_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) && (var_guard322 == 0.0)) {
        let assign17940_e17947: f64 = (var_vbi_minus_vjsrh * var_vbirstiinv);
        let assign17940_e17949: f64 = (assign17940_e17947).powf(p.p849);
        (assign17940_e17949, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign17940_e17951;
        var_tmp_dn6 = assign17940_e17951_d_n6;
        var_tmp_dn7 = assign17940_e17951_d_n7;
        var_tmp_dn8 = assign17940_e17951_d_n8;
        var_tmp_dn9 = assign17940_e17951_d_n9;

        let (assign17950_e17965, assign17950_e17965_d_n6, assign17950_e17965_d_n7, assign17950_e17965_d_n8, assign17950_e17965_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign17950_e17963: f64 = (var_wdepnulrsti * var_tmp);
        (assign17950_e17963, (var_wdepnulrsti * var_tmp_dn6), (var_wdepnulrsti * var_tmp_dn7), (var_wdepnulrsti * var_tmp_dn8), (var_wdepnulrsti * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign17950_e17965;
        var_wdep_dn6 = assign17950_e17965_d_n6;
        var_wdep_dn7 = assign17950_e17965_d_n7;
        var_wdep_dn8 = assign17950_e17965_d_n8;
        var_wdep_dn9 = assign17950_e17965_d_n9;

        let (assign17960_e17983, assign17960_e17983_d_n6, assign17960_e17983_d_n7, assign17960_e17983_d_n8, assign17960_e17983_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign17960_e17978: f64 = (var_zinv - 1.0);
        let assign17960_e17980: f64 = (assign17960_e17978 * var_wdep);
        let assign17960_e17981: f64 = (var_ftdsti * assign17960_e17980);
        (assign17960_e17981, (var_ftdsti * (assign17960_e17978 * var_wdep_dn6)), (var_ftdsti * (assign17960_e17978 * var_wdep_dn7)), (var_ftdsti * (assign17960_e17978 * var_wdep_dn8)), (var_ftdsti * (assign17960_e17978 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign17960_e17983;
        var_asrh_dn6 = assign17960_e17983_d_n6;
        var_asrh_dn7 = assign17960_e17983_d_n7;
        var_asrh_dn8 = assign17960_e17983_d_n8;
        var_asrh_dn9 = assign17960_e17983_d_n9;

        let (assign17970_e17999, assign17970_e17999_d_n6, assign17970_e17999_d_n7, assign17970_e17999_d_n8, assign17970_e17999_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard320 == 0.0)) {
        let assign17970_e17996: f64 = (var_asrh * var_wsrh);
        let assign17970_e17997: f64 = (p.p858 * assign17970_e17996);
        (assign17970_e17997, (p.p858 * (var_asrh_dn6 * var_wsrh)), (p.p858 * (var_asrh_dn7 * var_wsrh)), (p.p858 * (var_asrh_dn8 * var_wsrh)), (p.p858 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign17970_e17999;
        var_isrh_dn6 = assign17970_e17999_d_n6;
        var_isrh_dn7 = assign17970_e17999_d_n7;
        var_isrh_dn8 = assign17970_e17999_d_n8;
        var_isrh_dn9 = assign17970_e17999_d_n9;

        let assign17980_e18002: f64 = if p.p863 == 0.0 { 1.0 } else { 0.0 };
        var_guard323 = assign17980_e18002;

        let (assign17990_e18013, assign17990_e18013_d_n6, assign17990_e18013_d_n7, assign17990_e18013_d_n8, assign17990_e18013_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign17990_e18013;
        var_itat_dn6 = assign17990_e18013_d_n6;
        var_itat_dn7 = assign17990_e18013_d_n7;
        var_itat_dn8 = assign17990_e18013_d_n8;
        var_itat_dn9 = assign17990_e18013_d_n9;

        let (assign18000_e18031, assign18000_e18031_d_n6, assign18000_e18031_d_n7, assign18000_e18031_d_n8, assign18000_e18031_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18000_e18026: f64 = (var_wdep * var_one_minus_psti);
        let assign18000_e18028: f64 = (assign18000_e18026 / var_vbi_minus_vjsrh);
        let assign18000_e18029: f64 = (var_btatpartsti * assign18000_e18028);
        (assign18000_e18029, (var_btatpartsti * ((var_wdep_dn6 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn7 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn8 * var_one_minus_psti) / var_vbi_minus_vjsrh)), (var_btatpartsti * ((var_wdep_dn9 * var_one_minus_psti) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign18000_e18031;
        var_btat_dn6 = assign18000_e18031_d_n6;
        var_btat_dn7 = assign18000_e18031_d_n7;
        var_btat_dn8 = assign18000_e18031_d_n8;
        var_btat_dn9 = assign18000_e18031_d_n9;

        let (assign18010_e18047, assign18010_e18047_d_n6, assign18010_e18047_d_n7, assign18010_e18047_d_n8, assign18010_e18047_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18010_e18043: f64 = (0.666666666666667 * var_atatsti);
        let assign18010_e18045: f64 = (assign18010_e18043 / var_btat);
        (assign18010_e18045, (-((assign18010_e18043 * var_btat_dn6) / (var_btat * var_btat))), (-((assign18010_e18043 * var_btat_dn7) / (var_btat * var_btat))), (-((assign18010_e18043 * var_btat_dn8) / (var_btat * var_btat))), (-((assign18010_e18043 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign18010_e18047;
        var_twoatatoverthreebtat_dn6 = assign18010_e18047_d_n6;
        var_twoatatoverthreebtat_dn7 = assign18010_e18047_d_n7;
        var_twoatatoverthreebtat_dn8 = assign18010_e18047_d_n8;
        var_twoatatoverthreebtat_dn9 = assign18010_e18047_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_guard313_slot = var_guard313;
        *var_guard314_slot = var_guard314;
        *var_guard315_slot = var_guard315;
        *var_guard316_slot = var_guard316;
        *var_guard317_slot = var_guard317;
        *var_guard318_slot = var_guard318;
        *var_guard319_slot = var_guard319;
        *var_guard320_slot = var_guard320;
        *var_guard321_slot = var_guard321;
        *var_guard322_slot = var_guard322;
        *var_guard323_slot = var_guard323;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijunbot_slot = var_ijunbot;
        *var_ijunbot_dn6_slot = var_ijunbot_dn6;
        *var_ijunbot_dn7_slot = var_ijunbot_dn7;
        *var_ijunbot_dn8_slot = var_ijunbot_dn8;
        *var_ijunbot_dn9_slot = var_ijunbot_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }

    pub(super) fn stamp_transient_block_30(
        p: &Parameters,
        var_asrh: f64,
        var_asrh_dn6: f64,
        var_asrh_dn7: f64,
        var_asrh_dn8: f64,
        var_asrh_dn9: f64,
        var_atatsti: f64,
        var_berfc: f64,
        var_btat: f64,
        var_btat_dn6: f64,
        var_btat_dn7: f64,
        var_btat_dn8: f64,
        var_btat_dn9: f64,
        var_cerfc: f64,
        var_fbbtsti: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard319: f64,
        var_guard323: f64,
        var_one_over_one_minus_psti: f64,
        var_perfc: f64,
        var_twoatatoverthreebtat: f64,
        var_twoatatoverthreebtat_dn6: f64,
        var_twoatatoverthreebtat_dn7: f64,
        var_twoatatoverthreebtat_dn8: f64,
        var_twoatatoverthreebtat_dn9: f64,
        var_vbbt: f64,
        var_vbirstiinv: f64,
        var_wdepnulrinvsti: f64,
        var_wsrh: f64,
        var_erfcpos_slot: &mut f64,
        var_erfcpos_dn6_slot: &mut f64,
        var_erfcpos_dn7_slot: &mut f64,
        var_erfcpos_dn8_slot: &mut f64,
        var_erfcpos_dn9_slot: &mut f64,
        var_erfctimesexpmtat_slot: &mut f64,
        var_erfctimesexpmtat_dn6_slot: &mut f64,
        var_erfctimesexpmtat_dn7_slot: &mut f64,
        var_erfctimesexpmtat_dn8_slot: &mut f64,
        var_erfctimesexpmtat_dn9_slot: &mut f64,
        var_fmaxr_slot: &mut f64,
        var_fmaxr_dn6_slot: &mut f64,
        var_fmaxr_dn7_slot: &mut f64,
        var_fmaxr_dn8_slot: &mut f64,
        var_fmaxr_dn9_slot: &mut f64,
        var_gammamax_slot: &mut f64,
        var_gammamax_dn6_slot: &mut f64,
        var_gammamax_dn7_slot: &mut f64,
        var_gammamax_dn8_slot: &mut f64,
        var_gammamax_dn9_slot: &mut f64,
        var_guard324_slot: &mut f64,
        var_guard325_slot: &mut f64,
        var_guard326_slot: &mut f64,
        var_guard327_slot: &mut f64,
        var_guard328_slot: &mut f64,
        var_guard329_slot: &mut f64,
        var_guard330_slot: &mut f64,
        var_guard331_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_ktat_slot: &mut f64,
        var_ktat_dn6_slot: &mut f64,
        var_ktat_dn7_slot: &mut f64,
        var_ktat_dn8_slot: &mut f64,
        var_ktat_dn9_slot: &mut f64,
        var_ltat_slot: &mut f64,
        var_ltat_dn6_slot: &mut f64,
        var_ltat_dn7_slot: &mut f64,
        var_ltat_dn8_slot: &mut f64,
        var_ltat_dn9_slot: &mut f64,
        var_mtat_slot: &mut f64,
        var_mtat_dn6_slot: &mut f64,
        var_mtat_dn7_slot: &mut f64,
        var_mtat_dn8_slot: &mut f64,
        var_mtat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_terfc_slot: &mut f64,
        var_terfc_dn6_slot: &mut f64,
        var_terfc_dn7_slot: &mut f64,
        var_terfc_dn8_slot: &mut f64,
        var_terfc_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wtat_slot: &mut f64,
        var_wtat_dn6_slot: &mut f64,
        var_wtat_dn7_slot: &mut f64,
        var_wtat_dn8_slot: &mut f64,
        var_wtat_dn9_slot: &mut f64,
        var_xerfc_slot: &mut f64,
        var_xerfc_dn6_slot: &mut f64,
        var_xerfc_dn7_slot: &mut f64,
        var_xerfc_dn8_slot: &mut f64,
        var_xerfc_dn9_slot: &mut f64,
        var_ysq_slot: &mut f64,
        var_ysq_dn6_slot: &mut f64,
        var_ysq_dn7_slot: &mut f64,
        var_ysq_dn8_slot: &mut f64,
        var_ysq_dn9_slot: &mut f64,
    ) {
        let mut var_erfcpos: f64 = *var_erfcpos_slot;
        let mut var_erfcpos_dn6: f64 = *var_erfcpos_dn6_slot;
        let mut var_erfcpos_dn7: f64 = *var_erfcpos_dn7_slot;
        let mut var_erfcpos_dn8: f64 = *var_erfcpos_dn8_slot;
        let mut var_erfcpos_dn9: f64 = *var_erfcpos_dn9_slot;
        let mut var_erfctimesexpmtat: f64 = *var_erfctimesexpmtat_slot;
        let mut var_erfctimesexpmtat_dn6: f64 = *var_erfctimesexpmtat_dn6_slot;
        let mut var_erfctimesexpmtat_dn7: f64 = *var_erfctimesexpmtat_dn7_slot;
        let mut var_erfctimesexpmtat_dn8: f64 = *var_erfctimesexpmtat_dn8_slot;
        let mut var_erfctimesexpmtat_dn9: f64 = *var_erfctimesexpmtat_dn9_slot;
        let mut var_fmaxr: f64 = *var_fmaxr_slot;
        let mut var_fmaxr_dn6: f64 = *var_fmaxr_dn6_slot;
        let mut var_fmaxr_dn7: f64 = *var_fmaxr_dn7_slot;
        let mut var_fmaxr_dn8: f64 = *var_fmaxr_dn8_slot;
        let mut var_fmaxr_dn9: f64 = *var_fmaxr_dn9_slot;
        let mut var_gammamax: f64 = *var_gammamax_slot;
        let mut var_gammamax_dn6: f64 = *var_gammamax_dn6_slot;
        let mut var_gammamax_dn7: f64 = *var_gammamax_dn7_slot;
        let mut var_gammamax_dn8: f64 = *var_gammamax_dn8_slot;
        let mut var_gammamax_dn9: f64 = *var_gammamax_dn9_slot;
        let mut var_guard324: f64 = *var_guard324_slot;
        let mut var_guard325: f64 = *var_guard325_slot;
        let mut var_guard326: f64 = *var_guard326_slot;
        let mut var_guard327: f64 = *var_guard327_slot;
        let mut var_guard328: f64 = *var_guard328_slot;
        let mut var_guard329: f64 = *var_guard329_slot;
        let mut var_guard330: f64 = *var_guard330_slot;
        let mut var_guard331: f64 = *var_guard331_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_ktat: f64 = *var_ktat_slot;
        let mut var_ktat_dn6: f64 = *var_ktat_dn6_slot;
        let mut var_ktat_dn7: f64 = *var_ktat_dn7_slot;
        let mut var_ktat_dn8: f64 = *var_ktat_dn8_slot;
        let mut var_ktat_dn9: f64 = *var_ktat_dn9_slot;
        let mut var_ltat: f64 = *var_ltat_slot;
        let mut var_ltat_dn6: f64 = *var_ltat_dn6_slot;
        let mut var_ltat_dn7: f64 = *var_ltat_dn7_slot;
        let mut var_ltat_dn8: f64 = *var_ltat_dn8_slot;
        let mut var_ltat_dn9: f64 = *var_ltat_dn9_slot;
        let mut var_mtat: f64 = *var_mtat_slot;
        let mut var_mtat_dn6: f64 = *var_mtat_dn6_slot;
        let mut var_mtat_dn7: f64 = *var_mtat_dn7_slot;
        let mut var_mtat_dn8: f64 = *var_mtat_dn8_slot;
        let mut var_mtat_dn9: f64 = *var_mtat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_terfc: f64 = *var_terfc_slot;
        let mut var_terfc_dn6: f64 = *var_terfc_dn6_slot;
        let mut var_terfc_dn7: f64 = *var_terfc_dn7_slot;
        let mut var_terfc_dn8: f64 = *var_terfc_dn8_slot;
        let mut var_terfc_dn9: f64 = *var_terfc_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wtat: f64 = *var_wtat_slot;
        let mut var_wtat_dn6: f64 = *var_wtat_dn6_slot;
        let mut var_wtat_dn7: f64 = *var_wtat_dn7_slot;
        let mut var_wtat_dn8: f64 = *var_wtat_dn8_slot;
        let mut var_wtat_dn9: f64 = *var_wtat_dn9_slot;
        let mut var_xerfc: f64 = *var_xerfc_slot;
        let mut var_xerfc_dn6: f64 = *var_xerfc_dn6_slot;
        let mut var_xerfc_dn7: f64 = *var_xerfc_dn7_slot;
        let mut var_xerfc_dn8: f64 = *var_xerfc_dn8_slot;
        let mut var_xerfc_dn9: f64 = *var_xerfc_dn9_slot;
        let mut var_ysq: f64 = *var_ysq_slot;
        let mut var_ysq_dn6: f64 = *var_ysq_dn6_slot;
        let mut var_ysq_dn7: f64 = *var_ysq_dn7_slot;
        let mut var_ysq_dn8: f64 = *var_ysq_dn8_slot;
        let mut var_ysq_dn9: f64 = *var_ysq_dn9_slot;

        let (assign18020_e18061, assign18020_e18061_d_n6, assign18020_e18061_d_n7, assign18020_e18061_d_n8, assign18020_e18061_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18020_e18059: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign18020_e18059, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign18020_e18061;
        var_umaxbeforelimiting_dn6 = assign18020_e18061_d_n6;
        var_umaxbeforelimiting_dn7 = assign18020_e18061_d_n7;
        var_umaxbeforelimiting_dn8 = assign18020_e18061_d_n8;
        var_umaxbeforelimiting_dn9 = assign18020_e18061_d_n9;

        let (assign18030_e18082, assign18030_e18082_d_n6, assign18030_e18082_d_n7, assign18030_e18082_d_n8, assign18030_e18082_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18030_e18073: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign18030_e18076: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign18030_e18078: f64 = (assign18030_e18076 + 1.0);
        let assign18030_e18079: f64 = (assign18030_e18073 / assign18030_e18078);
        let assign18030_e18080: f64 = (assign18030_e18079).sqrt();
        (assign18030_e18080, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign18030_e18078) - (assign18030_e18073 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign18030_e18078 * assign18030_e18078)) / (2.0 * assign18030_e18080)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign18030_e18078) - (assign18030_e18073 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign18030_e18078 * assign18030_e18078)) / (2.0 * assign18030_e18080)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign18030_e18078) - (assign18030_e18073 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign18030_e18078 * assign18030_e18078)) / (2.0 * assign18030_e18080)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign18030_e18078) - (assign18030_e18073 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign18030_e18078 * assign18030_e18078)) / (2.0 * assign18030_e18080)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign18030_e18082;
        var_umax_dn6 = assign18030_e18082_d_n6;
        var_umax_dn7 = assign18030_e18082_d_n7;
        var_umax_dn8 = assign18030_e18082_d_n8;
        var_umax_dn9 = assign18030_e18082_d_n9;

        let (assign18040_e18095, assign18040_e18095_d_n6, assign18040_e18095_d_n7, assign18040_e18095_d_n8, assign18040_e18095_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18040_e18093: f64 = (var_umax).sqrt();
        (assign18040_e18093, (var_umax_dn6 / (2.0 * assign18040_e18093)), (var_umax_dn7 / (2.0 * assign18040_e18093)), (var_umax_dn8 / (2.0 * assign18040_e18093)), (var_umax_dn9 / (2.0 * assign18040_e18093)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign18040_e18095;
        var_sqrtumax_dn6 = assign18040_e18095_d_n6;
        var_sqrtumax_dn7 = assign18040_e18095_d_n7;
        var_sqrtumax_dn8 = assign18040_e18095_d_n8;
        var_sqrtumax_dn9 = assign18040_e18095_d_n9;

        let (assign18050_e18109, assign18050_e18109_d_n6, assign18050_e18109_d_n7, assign18050_e18109_d_n8, assign18050_e18109_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18050_e18107: f64 = (var_umax * var_sqrtumax);
        (assign18050_e18107, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign18050_e18109;
        var_umaxpoweronepointfive_dn6 = assign18050_e18109_d_n6;
        var_umaxpoweronepointfive_dn7 = assign18050_e18109_d_n7;
        var_umaxpoweronepointfive_dn8 = assign18050_e18109_d_n8;
        var_umaxpoweronepointfive_dn9 = assign18050_e18109_d_n9;

        let assign18060_e18111: f64 = (-p.p849);
        let assign18060_e18113: f64 = (assign18060_e18111 * var_one_over_one_minus_psti);
        let assign18060_e18115: f64 = (-1.0);
        let assign18060_e18116: f64 = if assign18060_e18113 == assign18060_e18115 { 1.0 } else { 0.0 };
        var_guard324 = assign18060_e18116;

        let (assign18070_e18136, assign18070_e18136_d_n6, assign18070_e18136_d_n7, assign18070_e18136_d_n8, assign18070_e18136_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard324 != 0.0)) {
        let assign18070_e18132: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18070_e18133: f64 = (1.0 + assign18070_e18132);
        let assign18070_e18134: f64 = (1.0 / assign18070_e18133);
        (assign18070_e18134, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign18070_e18133 * assign18070_e18133))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign18070_e18133 * assign18070_e18133))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign18070_e18133 * assign18070_e18133))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign18070_e18133 * assign18070_e18133))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign18070_e18136;
        var_wgamma_dn6 = assign18070_e18136_d_n6;
        var_wgamma_dn7 = assign18070_e18136_d_n7;
        var_wgamma_dn8 = assign18070_e18136_d_n8;
        var_wgamma_dn9 = assign18070_e18136_d_n9;

        let (assign18080_e18160, assign18080_e18160_d_n6, assign18080_e18160_d_n7, assign18080_e18160_d_n8, assign18080_e18160_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard324 == 0.0)) {
        let assign18080_e18152: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18080_e18153: f64 = (1.0 + assign18080_e18152);
        let assign18080_e18155: f64 = (-p.p849);
        let assign18080_e18157: f64 = (assign18080_e18155 * var_one_over_one_minus_psti);
        let assign18080_e18158: f64 = (assign18080_e18153).powf(assign18080_e18157);
        (assign18080_e18158, if 0.0 == 0.0 && ((assign18080_e18157) as f64).is_finite() && ((assign18080_e18157) as f64).fract() == 0.0 { if assign18080_e18157 == 0.0 { 0.0 } else { (assign18080_e18157 * ((assign18080_e18153).powf(assign18080_e18157 - 1.0) * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))) } } else { (assign18080_e18158 * (assign18080_e18157 * (((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / assign18080_e18153))) }, if 0.0 == 0.0 && ((assign18080_e18157) as f64).is_finite() && ((assign18080_e18157) as f64).fract() == 0.0 { if assign18080_e18157 == 0.0 { 0.0 } else { (assign18080_e18157 * ((assign18080_e18153).powf(assign18080_e18157 - 1.0) * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))) } } else { (assign18080_e18158 * (assign18080_e18157 * (((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / assign18080_e18153))) }, if 0.0 == 0.0 && ((assign18080_e18157) as f64).is_finite() && ((assign18080_e18157) as f64).fract() == 0.0 { if assign18080_e18157 == 0.0 { 0.0 } else { (assign18080_e18157 * ((assign18080_e18153).powf(assign18080_e18157 - 1.0) * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))) } } else { (assign18080_e18158 * (assign18080_e18157 * (((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / assign18080_e18153))) }, if 0.0 == 0.0 && ((assign18080_e18157) as f64).is_finite() && ((assign18080_e18157) as f64).fract() == 0.0 { if assign18080_e18157 == 0.0 { 0.0 } else { (assign18080_e18157 * ((assign18080_e18153).powf(assign18080_e18157 - 1.0) * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))) } } else { (assign18080_e18158 * (assign18080_e18157 * (((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / assign18080_e18153))) },)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign18080_e18160;
        var_wgamma_dn6 = assign18080_e18160_d_n6;
        var_wgamma_dn7 = assign18080_e18160_d_n7;
        var_wgamma_dn8 = assign18080_e18160_d_n8;
        var_wgamma_dn9 = assign18080_e18160_d_n9;

        let (assign18090_e18178, assign18090_e18178_d_n6, assign18090_e18178_d_n7, assign18090_e18178_d_n8, assign18090_e18178_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18090_e18172: f64 = (var_wsrh * var_wgamma);
        let assign18090_e18175: f64 = (var_wsrh + var_wgamma);
        let assign18090_e18176: f64 = (assign18090_e18172 / assign18090_e18175);
        (assign18090_e18176, ((((var_wsrh * var_wgamma_dn6) * assign18090_e18175) - (assign18090_e18172 * var_wgamma_dn6)) / (assign18090_e18175 * assign18090_e18175)), ((((var_wsrh * var_wgamma_dn7) * assign18090_e18175) - (assign18090_e18172 * var_wgamma_dn7)) / (assign18090_e18175 * assign18090_e18175)), ((((var_wsrh * var_wgamma_dn8) * assign18090_e18175) - (assign18090_e18172 * var_wgamma_dn8)) / (assign18090_e18175 * assign18090_e18175)), ((((var_wsrh * var_wgamma_dn9) * assign18090_e18175) - (assign18090_e18172 * var_wgamma_dn9)) / (assign18090_e18175 * assign18090_e18175)),)
    } else {
        (var_wtat, var_wtat_dn6, var_wtat_dn7, var_wtat_dn8, var_wtat_dn9,)
    }
};
        var_wtat = assign18090_e18178;
        var_wtat_dn6 = assign18090_e18178_d_n6;
        var_wtat_dn7 = assign18090_e18178_d_n7;
        var_wtat_dn8 = assign18090_e18178_d_n8;
        var_wtat_dn9 = assign18090_e18178_d_n9;

        let (assign18100_e18195, assign18100_e18195_d_n6, assign18100_e18195_d_n7, assign18100_e18195_d_n8, assign18100_e18195_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18100_e18191: f64 = (var_btat / var_sqrtumax);
        let assign18100_e18192: f64 = (0.375 * assign18100_e18191);
        let assign18100_e18193: f64 = (assign18100_e18192).sqrt();
        (assign18100_e18193, ((0.375 * (((var_btat_dn6 * var_sqrtumax) - (var_btat * var_sqrtumax_dn6)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18100_e18193)), ((0.375 * (((var_btat_dn7 * var_sqrtumax) - (var_btat * var_sqrtumax_dn7)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18100_e18193)), ((0.375 * (((var_btat_dn8 * var_sqrtumax) - (var_btat * var_sqrtumax_dn8)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18100_e18193)), ((0.375 * (((var_btat_dn9 * var_sqrtumax) - (var_btat * var_sqrtumax_dn9)) / (var_sqrtumax * var_sqrtumax))) / (2.0 * assign18100_e18193)),)
    } else {
        (var_ktat, var_ktat_dn6, var_ktat_dn7, var_ktat_dn8, var_ktat_dn9,)
    }
};
        var_ktat = assign18100_e18195;
        var_ktat_dn6 = assign18100_e18195_d_n6;
        var_ktat_dn7 = assign18100_e18195_d_n7;
        var_ktat_dn8 = assign18100_e18195_d_n8;
        var_ktat_dn9 = assign18100_e18195_d_n9;

        let (assign18110_e18213, assign18110_e18213_d_n6, assign18110_e18213_d_n7, assign18110_e18213_d_n8, assign18110_e18213_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18110_e18208: f64 = (var_twoatatoverthreebtat * var_sqrtumax);
        let assign18110_e18209: f64 = (2.0 * assign18110_e18208);
        let assign18110_e18211: f64 = (assign18110_e18209 - var_umax);
        (assign18110_e18211, ((2.0 * ((var_twoatatoverthreebtat_dn6 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn6))) - var_umax_dn6), ((2.0 * ((var_twoatatoverthreebtat_dn7 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn7))) - var_umax_dn7), ((2.0 * ((var_twoatatoverthreebtat_dn8 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn8))) - var_umax_dn8), ((2.0 * ((var_twoatatoverthreebtat_dn9 * var_sqrtumax) + (var_twoatatoverthreebtat * var_sqrtumax_dn9))) - var_umax_dn9),)
    } else {
        (var_ltat, var_ltat_dn6, var_ltat_dn7, var_ltat_dn8, var_ltat_dn9,)
    }
};
        var_ltat = assign18110_e18213;
        var_ltat_dn6 = assign18110_e18213_d_n6;
        var_ltat_dn7 = assign18110_e18213_d_n7;
        var_ltat_dn8 = assign18110_e18213_d_n8;
        var_ltat_dn9 = assign18110_e18213_d_n9;

        let (assign18120_e18239, assign18120_e18239_d_n6, assign18120_e18239_d_n7, assign18120_e18239_d_n8, assign18120_e18239_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18120_e18225: f64 = (var_atatsti * var_twoatatoverthreebtat);
        let assign18120_e18227: f64 = (assign18120_e18225 * var_sqrtumax);
        let assign18120_e18230: f64 = (var_atatsti * var_umax);
        let assign18120_e18231: f64 = (assign18120_e18227 - assign18120_e18230);
        let assign18120_e18235: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18120_e18236: f64 = (0.5 * assign18120_e18235);
        let assign18120_e18237: f64 = (assign18120_e18231 + assign18120_e18236);
        (assign18120_e18237, (((((var_atatsti * var_twoatatoverthreebtat_dn6) * var_sqrtumax) + (assign18120_e18225 * var_sqrtumax_dn6)) - (var_atatsti * var_umax_dn6)) + (0.5 * ((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)))), (((((var_atatsti * var_twoatatoverthreebtat_dn7) * var_sqrtumax) + (assign18120_e18225 * var_sqrtumax_dn7)) - (var_atatsti * var_umax_dn7)) + (0.5 * ((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)))), (((((var_atatsti * var_twoatatoverthreebtat_dn8) * var_sqrtumax) + (assign18120_e18225 * var_sqrtumax_dn8)) - (var_atatsti * var_umax_dn8)) + (0.5 * ((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)))), (((((var_atatsti * var_twoatatoverthreebtat_dn9) * var_sqrtumax) + (assign18120_e18225 * var_sqrtumax_dn9)) - (var_atatsti * var_umax_dn9)) + (0.5 * ((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)))),)
    } else {
        (var_mtat, var_mtat_dn6, var_mtat_dn7, var_mtat_dn8, var_mtat_dn9,)
    }
};
        var_mtat = assign18120_e18239;
        var_mtat_dn6 = assign18120_e18239_d_n6;
        var_mtat_dn7 = assign18120_e18239_d_n7;
        var_mtat_dn8 = assign18120_e18239_d_n8;
        var_mtat_dn9 = assign18120_e18239_d_n9;

        let (assign18130_e18255, assign18130_e18255_d_n6, assign18130_e18255_d_n7, assign18130_e18255_d_n8, assign18130_e18255_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18130_e18251: f64 = (var_ltat - 1.0);
        let assign18130_e18253: f64 = (assign18130_e18251 * var_ktat);
        (assign18130_e18253, ((var_ltat_dn6 * var_ktat) + (assign18130_e18251 * var_ktat_dn6)), ((var_ltat_dn7 * var_ktat) + (assign18130_e18251 * var_ktat_dn7)), ((var_ltat_dn8 * var_ktat) + (assign18130_e18251 * var_ktat_dn8)), ((var_ltat_dn9 * var_ktat) + (assign18130_e18251 * var_ktat_dn9)),)
    } else {
        (var_xerfc, var_xerfc_dn6, var_xerfc_dn7, var_xerfc_dn8, var_xerfc_dn9,)
    }
};
        var_xerfc = assign18130_e18255;
        var_xerfc_dn6 = assign18130_e18255_d_n6;
        var_xerfc_dn7 = assign18130_e18255_d_n7;
        var_xerfc_dn8 = assign18130_e18255_d_n8;
        var_xerfc_dn9 = assign18130_e18255_d_n9;

        let (assign18140_e18269, assign18140_e18269_d_n6, assign18140_e18269_d_n7, assign18140_e18269_d_n8, assign18140_e18269_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18140_e18267: f64 = (var_xerfc * var_xerfc);
        (assign18140_e18267, ((var_xerfc_dn6 * var_xerfc) + (var_xerfc * var_xerfc_dn6)), ((var_xerfc_dn7 * var_xerfc) + (var_xerfc * var_xerfc_dn7)), ((var_xerfc_dn8 * var_xerfc) + (var_xerfc * var_xerfc_dn8)), ((var_xerfc_dn9 * var_xerfc) + (var_xerfc * var_xerfc_dn9)),)
    } else {
        (var_ysq, var_ysq_dn6, var_ysq_dn7, var_ysq_dn8, var_ysq_dn9,)
    }
};
        var_ysq = assign18140_e18269;
        var_ysq_dn6 = assign18140_e18269_d_n6;
        var_ysq_dn7 = assign18140_e18269_d_n7;
        var_ysq_dn8 = assign18140_e18269_d_n8;
        var_ysq_dn9 = assign18140_e18269_d_n9;

        let assign18150_e18272: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard325 = assign18150_e18272;

        let (assign18160_e18292, assign18160_e18292_d_n6, assign18160_e18292_d_n7, assign18160_e18292_d_n8, assign18160_e18292_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard325 != 0.0)) {
        let assign18160_e18288: f64 = (var_perfc * var_xerfc);
        let assign18160_e18289: f64 = (1.0 + assign18160_e18288);
        let assign18160_e18290: f64 = (1.0 / assign18160_e18289);
        (assign18160_e18290, (-((var_perfc * var_xerfc_dn6) / (assign18160_e18289 * assign18160_e18289))), (-((var_perfc * var_xerfc_dn7) / (assign18160_e18289 * assign18160_e18289))), (-((var_perfc * var_xerfc_dn8) / (assign18160_e18289 * assign18160_e18289))), (-((var_perfc * var_xerfc_dn9) / (assign18160_e18289 * assign18160_e18289))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign18160_e18292;
        var_terfc_dn6 = assign18160_e18292_d_n6;
        var_terfc_dn7 = assign18160_e18292_d_n7;
        var_terfc_dn8 = assign18160_e18292_d_n8;
        var_terfc_dn9 = assign18160_e18292_d_n9;

        let (assign18170_e18313, assign18170_e18313_d_n6, assign18170_e18313_d_n7, assign18170_e18313_d_n8, assign18170_e18313_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard325 == 0.0)) {
        let assign18170_e18309: f64 = (var_perfc * var_xerfc);
        let assign18170_e18310: f64 = (1.0 - assign18170_e18309);
        let assign18170_e18311: f64 = (1.0 / assign18170_e18310);
        (assign18170_e18311, (-((-(var_perfc * var_xerfc_dn6)) / (assign18170_e18310 * assign18170_e18310))), (-((-(var_perfc * var_xerfc_dn7)) / (assign18170_e18310 * assign18170_e18310))), (-((-(var_perfc * var_xerfc_dn8)) / (assign18170_e18310 * assign18170_e18310))), (-((-(var_perfc * var_xerfc_dn9)) / (assign18170_e18310 * assign18170_e18310))),)
    } else {
        (var_terfc, var_terfc_dn6, var_terfc_dn7, var_terfc_dn8, var_terfc_dn9,)
    }
};
        var_terfc = assign18170_e18313;
        var_terfc_dn6 = assign18170_e18313_d_n6;
        var_terfc_dn7 = assign18170_e18313_d_n7;
        var_terfc_dn8 = assign18170_e18313_d_n8;
        var_terfc_dn9 = assign18170_e18313_d_n9;

        let assign18180_e18315: f64 = (-var_ysq);
        let assign18180_e18317: f64 = (assign18180_e18315 + var_mtat);
        let assign18180_e18319: f64 = (-230.25850929940458);
        let assign18180_e18320: f64 = if assign18180_e18317 > assign18180_e18319 { 1.0 } else { 0.0 };
        var_guard326 = assign18180_e18320;

        let (assign18190_e18338, assign18190_e18338_d_n6, assign18190_e18338_d_n7, assign18190_e18338_d_n8, assign18190_e18338_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard326 != 0.0)) {
        let assign18190_e18333: f64 = (-var_ysq);
        let assign18190_e18335: f64 = (assign18190_e18333 + var_mtat);
        let assign18190_e18336: f64 = (assign18190_e18335).exp();
        (assign18190_e18336, (assign18190_e18336 * ((-var_ysq_dn6) + var_mtat_dn6)), (assign18190_e18336 * ((-var_ysq_dn7) + var_mtat_dn7)), (assign18190_e18336 * ((-var_ysq_dn8) + var_mtat_dn8)), (assign18190_e18336 * ((-var_ysq_dn9) + var_mtat_dn9)),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18190_e18338;
        var_tmp_dn6 = assign18190_e18338_d_n6;
        var_tmp_dn7 = assign18190_e18338_d_n7;
        var_tmp_dn8 = assign18190_e18338_d_n8;
        var_tmp_dn9 = assign18190_e18338_d_n9;

        let (assign18200_e18387, assign18200_e18387_d_n6, assign18200_e18387_d_n7, assign18200_e18387_d_n8, assign18200_e18387_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard326 == 0.0)) {
        let assign18200_e18354: f64 = (-230.25850929940458);
        let assign18200_e18356: f64 = (-var_ysq);
        let assign18200_e18358: f64 = (assign18200_e18356 + var_mtat);
        let assign18200_e18359: f64 = (assign18200_e18354 - assign18200_e18358);
        let assign18200_e18363: f64 = (-230.25850929940458);
        let assign18200_e18365: f64 = (-var_ysq);
        let assign18200_e18367: f64 = (assign18200_e18365 + var_mtat);
        let assign18200_e18368: f64 = (assign18200_e18363 - assign18200_e18367);
        let assign18200_e18371: f64 = (-230.25850929940458);
        let assign18200_e18373: f64 = (-var_ysq);
        let assign18200_e18375: f64 = (assign18200_e18373 + var_mtat);
        let assign18200_e18376: f64 = (assign18200_e18371 - assign18200_e18375);
        let assign18200_e18378: f64 = (assign18200_e18376 * 0.3333333333333333);
        let assign18200_e18379: f64 = (1.0 + assign18200_e18378);
        let assign18200_e18380: f64 = (assign18200_e18368 * assign18200_e18379);
        let assign18200_e18381: f64 = (0.5 * assign18200_e18380);
        let assign18200_e18382: f64 = (1.0 + assign18200_e18381);
        let assign18200_e18383: f64 = (assign18200_e18359 * assign18200_e18382);
        let assign18200_e18384: f64 = (1.0 + assign18200_e18383);
        let assign18200_e18385: f64 = (1e-100 / assign18200_e18384);
        (assign18200_e18385, (-((1e-100 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign18200_e18382) + (assign18200_e18359 * (0.5 * (((-((-var_ysq_dn6) + var_mtat_dn6)) * assign18200_e18379) + (assign18200_e18368 * ((-((-var_ysq_dn6) + var_mtat_dn6)) * 0.3333333333333333))))))) / (assign18200_e18384 * assign18200_e18384))), (-((1e-100 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign18200_e18382) + (assign18200_e18359 * (0.5 * (((-((-var_ysq_dn7) + var_mtat_dn7)) * assign18200_e18379) + (assign18200_e18368 * ((-((-var_ysq_dn7) + var_mtat_dn7)) * 0.3333333333333333))))))) / (assign18200_e18384 * assign18200_e18384))), (-((1e-100 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign18200_e18382) + (assign18200_e18359 * (0.5 * (((-((-var_ysq_dn8) + var_mtat_dn8)) * assign18200_e18379) + (assign18200_e18368 * ((-((-var_ysq_dn8) + var_mtat_dn8)) * 0.3333333333333333))))))) / (assign18200_e18384 * assign18200_e18384))), (-((1e-100 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign18200_e18382) + (assign18200_e18359 * (0.5 * (((-((-var_ysq_dn9) + var_mtat_dn9)) * assign18200_e18379) + (assign18200_e18368 * ((-((-var_ysq_dn9) + var_mtat_dn9)) * 0.3333333333333333))))))) / (assign18200_e18384 * assign18200_e18384))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18200_e18387;
        var_tmp_dn6 = assign18200_e18387_d_n6;
        var_tmp_dn7 = assign18200_e18387_d_n7;
        var_tmp_dn8 = assign18200_e18387_d_n8;
        var_tmp_dn9 = assign18200_e18387_d_n9;

        let (assign18210_e18417, assign18210_e18417_d_n6, assign18210_e18417_d_n7, assign18210_e18417_d_n8, assign18210_e18417_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18210_e18399: f64 = (0.29214664 * var_terfc);
        let assign18210_e18403: f64 = (var_terfc * var_terfc);
        let assign18210_e18404: f64 = (var_berfc * assign18210_e18403);
        let assign18210_e18405: f64 = (assign18210_e18399 + assign18210_e18404);
        let assign18210_e18409: f64 = (var_terfc * var_terfc);
        let assign18210_e18411: f64 = (assign18210_e18409 * var_terfc);
        let assign18210_e18412: f64 = (var_cerfc * assign18210_e18411);
        let assign18210_e18413: f64 = (assign18210_e18405 + assign18210_e18412);
        let assign18210_e18415: f64 = (assign18210_e18413 * var_tmp);
        (assign18210_e18415, (((((0.29214664 * var_terfc_dn6) + (var_berfc * ((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)))) + (var_cerfc * ((((var_terfc_dn6 * var_terfc) + (var_terfc * var_terfc_dn6)) * var_terfc) + (assign18210_e18409 * var_terfc_dn6)))) * var_tmp) + (assign18210_e18413 * var_tmp_dn6)), (((((0.29214664 * var_terfc_dn7) + (var_berfc * ((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)))) + (var_cerfc * ((((var_terfc_dn7 * var_terfc) + (var_terfc * var_terfc_dn7)) * var_terfc) + (assign18210_e18409 * var_terfc_dn7)))) * var_tmp) + (assign18210_e18413 * var_tmp_dn7)), (((((0.29214664 * var_terfc_dn8) + (var_berfc * ((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)))) + (var_cerfc * ((((var_terfc_dn8 * var_terfc) + (var_terfc * var_terfc_dn8)) * var_terfc) + (assign18210_e18409 * var_terfc_dn8)))) * var_tmp) + (assign18210_e18413 * var_tmp_dn8)), (((((0.29214664 * var_terfc_dn9) + (var_berfc * ((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)))) + (var_cerfc * ((((var_terfc_dn9 * var_terfc) + (var_terfc * var_terfc_dn9)) * var_terfc) + (assign18210_e18409 * var_terfc_dn9)))) * var_tmp) + (assign18210_e18413 * var_tmp_dn9)),)
    } else {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    }
};
        var_erfcpos = assign18210_e18417;
        var_erfcpos_dn6 = assign18210_e18417_d_n6;
        var_erfcpos_dn7 = assign18210_e18417_d_n7;
        var_erfcpos_dn8 = assign18210_e18417_d_n8;
        var_erfcpos_dn9 = assign18210_e18417_d_n9;

        let assign18220_e18420: f64 = if var_xerfc > 0.0 { 1.0 } else { 0.0 };
        var_guard327 = assign18220_e18420;

        let (assign18230_e18434, assign18230_e18434_d_n6, assign18230_e18434_d_n7, assign18230_e18434_d_n8, assign18230_e18434_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard327 != 0.0)) {
        (var_erfcpos, var_erfcpos_dn6, var_erfcpos_dn7, var_erfcpos_dn8, var_erfcpos_dn9,)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign18230_e18434;
        var_erfctimesexpmtat_dn6 = assign18230_e18434_d_n6;
        var_erfctimesexpmtat_dn7 = assign18230_e18434_d_n7;
        var_erfctimesexpmtat_dn8 = assign18230_e18434_d_n8;
        var_erfctimesexpmtat_dn9 = assign18230_e18434_d_n9;

        let assign18240_e18437: f64 = (-230.25850929940458);
        let assign18240_e18438: f64 = if var_mtat > assign18240_e18437 { 1.0 } else { 0.0 };
        var_guard328 = assign18240_e18438;

        let (assign18250_e18456, assign18250_e18456_d_n6, assign18250_e18456_d_n7, assign18250_e18456_d_n8, assign18250_e18456_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard327 == 0.0)) && (var_guard328 != 0.0)) {
        let assign18250_e18454: f64 = (var_mtat).exp();
        (assign18250_e18454, (assign18250_e18454 * var_mtat_dn6), (assign18250_e18454 * var_mtat_dn7), (assign18250_e18454 * var_mtat_dn8), (assign18250_e18454 * var_mtat_dn9),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18250_e18456;
        var_tmp_dn6 = assign18250_e18456_d_n6;
        var_tmp_dn7 = assign18250_e18456_d_n7;
        var_tmp_dn8 = assign18250_e18456_d_n8;
        var_tmp_dn9 = assign18250_e18456_d_n9;

        let (assign18260_e18499, assign18260_e18499_d_n6, assign18260_e18499_d_n7, assign18260_e18499_d_n8, assign18260_e18499_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard327 == 0.0)) && (var_guard328 == 0.0)) {
        let assign18260_e18475: f64 = (-230.25850929940458);
        let assign18260_e18477: f64 = (assign18260_e18475 - var_mtat);
        let assign18260_e18481: f64 = (-230.25850929940458);
        let assign18260_e18483: f64 = (assign18260_e18481 - var_mtat);
        let assign18260_e18486: f64 = (-230.25850929940458);
        let assign18260_e18488: f64 = (assign18260_e18486 - var_mtat);
        let assign18260_e18490: f64 = (assign18260_e18488 * 0.3333333333333333);
        let assign18260_e18491: f64 = (1.0 + assign18260_e18490);
        let assign18260_e18492: f64 = (assign18260_e18483 * assign18260_e18491);
        let assign18260_e18493: f64 = (0.5 * assign18260_e18492);
        let assign18260_e18494: f64 = (1.0 + assign18260_e18493);
        let assign18260_e18495: f64 = (assign18260_e18477 * assign18260_e18494);
        let assign18260_e18496: f64 = (1.0 + assign18260_e18495);
        let assign18260_e18497: f64 = (1e-100 / assign18260_e18496);
        (assign18260_e18497, (-((1e-100 * (((-var_mtat_dn6) * assign18260_e18494) + (assign18260_e18477 * (0.5 * (((-var_mtat_dn6) * assign18260_e18491) + (assign18260_e18483 * ((-var_mtat_dn6) * 0.3333333333333333))))))) / (assign18260_e18496 * assign18260_e18496))), (-((1e-100 * (((-var_mtat_dn7) * assign18260_e18494) + (assign18260_e18477 * (0.5 * (((-var_mtat_dn7) * assign18260_e18491) + (assign18260_e18483 * ((-var_mtat_dn7) * 0.3333333333333333))))))) / (assign18260_e18496 * assign18260_e18496))), (-((1e-100 * (((-var_mtat_dn8) * assign18260_e18494) + (assign18260_e18477 * (0.5 * (((-var_mtat_dn8) * assign18260_e18491) + (assign18260_e18483 * ((-var_mtat_dn8) * 0.3333333333333333))))))) / (assign18260_e18496 * assign18260_e18496))), (-((1e-100 * (((-var_mtat_dn9) * assign18260_e18494) + (assign18260_e18477 * (0.5 * (((-var_mtat_dn9) * assign18260_e18491) + (assign18260_e18483 * ((-var_mtat_dn9) * 0.3333333333333333))))))) / (assign18260_e18496 * assign18260_e18496))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18260_e18499;
        var_tmp_dn6 = assign18260_e18499_d_n6;
        var_tmp_dn7 = assign18260_e18499_d_n7;
        var_tmp_dn8 = assign18260_e18499_d_n8;
        var_tmp_dn9 = assign18260_e18499_d_n9;

        let (assign18270_e18518, assign18270_e18518_d_n6, assign18270_e18518_d_n7, assign18270_e18518_d_n8, assign18270_e18518_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) && (var_guard327 == 0.0)) {
        let assign18270_e18514: f64 = (2.0 * var_tmp);
        let assign18270_e18516: f64 = (assign18270_e18514 - var_erfcpos);
        (assign18270_e18516, ((2.0 * var_tmp_dn6) - var_erfcpos_dn6), ((2.0 * var_tmp_dn7) - var_erfcpos_dn7), ((2.0 * var_tmp_dn8) - var_erfcpos_dn8), ((2.0 * var_tmp_dn9) - var_erfcpos_dn9),)
    } else {
        (var_erfctimesexpmtat, var_erfctimesexpmtat_dn6, var_erfctimesexpmtat_dn7, var_erfctimesexpmtat_dn8, var_erfctimesexpmtat_dn9,)
    }
};
        var_erfctimesexpmtat = assign18270_e18518;
        var_erfctimesexpmtat_dn6 = assign18270_e18518_d_n6;
        var_erfctimesexpmtat_dn7 = assign18270_e18518_d_n7;
        var_erfctimesexpmtat_dn8 = assign18270_e18518_d_n8;
        var_erfctimesexpmtat_dn9 = assign18270_e18518_d_n9;

        let (assign18280_e18538, assign18280_e18538_d_n6, assign18280_e18538_d_n7, assign18280_e18538_d_n8, assign18280_e18538_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18280_e18530: f64 = (1.772453850905516 * 0.5);
        let assign18280_e18533: f64 = (var_atatsti * var_erfctimesexpmtat);
        let assign18280_e18535: f64 = (assign18280_e18533 / var_ktat);
        let assign18280_e18536: f64 = (assign18280_e18530 * assign18280_e18535);
        (assign18280_e18536, (assign18280_e18530 * ((((var_atatsti * var_erfctimesexpmtat_dn6) * var_ktat) - (assign18280_e18533 * var_ktat_dn6)) / (var_ktat * var_ktat))), (assign18280_e18530 * ((((var_atatsti * var_erfctimesexpmtat_dn7) * var_ktat) - (assign18280_e18533 * var_ktat_dn7)) / (var_ktat * var_ktat))), (assign18280_e18530 * ((((var_atatsti * var_erfctimesexpmtat_dn8) * var_ktat) - (assign18280_e18533 * var_ktat_dn8)) / (var_ktat * var_ktat))), (assign18280_e18530 * ((((var_atatsti * var_erfctimesexpmtat_dn9) * var_ktat) - (assign18280_e18533 * var_ktat_dn9)) / (var_ktat * var_ktat))),)
    } else {
        (var_gammamax, var_gammamax_dn6, var_gammamax_dn7, var_gammamax_dn8, var_gammamax_dn9,)
    }
};
        var_gammamax = assign18280_e18538;
        var_gammamax_dn6 = assign18280_e18538_d_n6;
        var_gammamax_dn7 = assign18280_e18538_d_n7;
        var_gammamax_dn8 = assign18280_e18538_d_n8;
        var_gammamax_dn9 = assign18280_e18538_d_n9;

        let (assign18290_e18556, assign18290_e18556_d_n6, assign18290_e18556_d_n7, assign18290_e18556_d_n8, assign18290_e18556_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard323 == 0.0)) {
        let assign18290_e18551: f64 = (var_asrh * var_gammamax);
        let assign18290_e18553: f64 = (assign18290_e18551 * var_wtat);
        let assign18290_e18554: f64 = (p.p863 * assign18290_e18553);
        (assign18290_e18554, (p.p863 * ((((var_asrh_dn6 * var_gammamax) + (var_asrh * var_gammamax_dn6)) * var_wtat) + (assign18290_e18551 * var_wtat_dn6))), (p.p863 * ((((var_asrh_dn7 * var_gammamax) + (var_asrh * var_gammamax_dn7)) * var_wtat) + (assign18290_e18551 * var_wtat_dn7))), (p.p863 * ((((var_asrh_dn8 * var_gammamax) + (var_asrh * var_gammamax_dn8)) * var_wtat) + (assign18290_e18551 * var_wtat_dn8))), (p.p863 * ((((var_asrh_dn9 * var_gammamax) + (var_asrh * var_gammamax_dn9)) * var_wtat) + (assign18290_e18551 * var_wtat_dn9))),)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign18290_e18556;
        var_itat_dn6 = assign18290_e18556_d_n6;
        var_itat_dn7 = assign18290_e18556_d_n7;
        var_itat_dn8 = assign18290_e18556_d_n8;
        var_itat_dn9 = assign18290_e18556_d_n9;

        let assign18300_e18559: f64 = if p.p869 == 0.0 { 1.0 } else { 0.0 };
        var_guard329 = assign18300_e18559;

        let (assign18310_e18570, assign18310_e18570_d_n6, assign18310_e18570_d_n7, assign18310_e18570_d_n8, assign18310_e18570_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign18310_e18570;
        var_ibbt_dn6 = assign18310_e18570_d_n6;
        var_ibbt_dn7 = assign18310_e18570_d_n7;
        var_ibbt_dn8 = assign18310_e18570_d_n8;
        var_ibbt_dn9 = assign18310_e18570_d_n9;

        let assign18320_e18573: f64 = if p.p849 == 0.5 { 1.0 } else { 0.0 };
        var_guard330 = assign18320_e18573;

        let (assign18330_e18592, assign18330_e18592_d_n6, assign18330_e18592_d_n7, assign18330_e18592_d_n8, assign18330_e18592_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) && (var_guard330 != 0.0)) {
        let assign18330_e18587: f64 = (p.p846 - var_vbbt);
        let assign18330_e18589: f64 = (assign18330_e18587 * var_vbirstiinv);
        let assign18330_e18590: f64 = (assign18330_e18589).sqrt();
        (assign18330_e18590, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18330_e18592;
        var_tmp_dn6 = assign18330_e18592_d_n6;
        var_tmp_dn7 = assign18330_e18592_d_n7;
        var_tmp_dn8 = assign18330_e18592_d_n8;
        var_tmp_dn9 = assign18330_e18592_d_n9;

        let (assign18340_e18613, assign18340_e18613_d_n6, assign18340_e18613_d_n7, assign18340_e18613_d_n8, assign18340_e18613_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) && (var_guard330 == 0.0)) {
        let assign18340_e18607: f64 = (p.p846 - var_vbbt);
        let assign18340_e18609: f64 = (assign18340_e18607 * var_vbirstiinv);
        let assign18340_e18611: f64 = (assign18340_e18609).powf(p.p849);
        (assign18340_e18611, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18340_e18613;
        var_tmp_dn6 = assign18340_e18613_d_n6;
        var_tmp_dn7 = assign18340_e18613_d_n7;
        var_tmp_dn8 = assign18340_e18613_d_n8;
        var_tmp_dn9 = assign18340_e18613_d_n9;

        let (assign18350_e18633, assign18350_e18633_d_n6, assign18350_e18633_d_n7, assign18350_e18633_d_n8, assign18350_e18633_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) {
        let assign18350_e18626: f64 = (p.p846 - var_vbbt);
        let assign18350_e18628: f64 = (assign18350_e18626 * var_wdepnulrinvsti);
        let assign18350_e18630: f64 = (assign18350_e18628 / var_tmp);
        let assign18350_e18631: f64 = (var_one_over_one_minus_psti * assign18350_e18630);
        (assign18350_e18631, (var_one_over_one_minus_psti * (-((assign18350_e18628 * var_tmp_dn6) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign18350_e18628 * var_tmp_dn7) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign18350_e18628 * var_tmp_dn8) / (var_tmp * var_tmp)))), (var_one_over_one_minus_psti * (-((assign18350_e18628 * var_tmp_dn9) / (var_tmp * var_tmp)))),)
    } else {
        (var_fmaxr, var_fmaxr_dn6, var_fmaxr_dn7, var_fmaxr_dn8, var_fmaxr_dn9,)
    }
};
        var_fmaxr = assign18350_e18633;
        var_fmaxr_dn6 = assign18350_e18633_d_n6;
        var_fmaxr_dn7 = assign18350_e18633_d_n7;
        var_fmaxr_dn8 = assign18350_e18633_d_n8;
        var_fmaxr_dn9 = assign18350_e18633_d_n9;

        let assign18360_e18635: f64 = (-var_fbbtsti);
        let assign18360_e18637: f64 = (assign18360_e18635 / var_fmaxr);
        let assign18360_e18638: f64 = (assign18360_e18637).abs();
        let assign18360_e18640: f64 = if assign18360_e18638 < 230.25850929940458 { 1.0 } else { 0.0 };
        var_guard331 = assign18360_e18640;

        let (assign18370_e18658, assign18370_e18658_d_n6, assign18370_e18658_d_n7, assign18370_e18658_d_n8, assign18370_e18658_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) && (var_guard331 != 0.0)) {
        let assign18370_e18653: f64 = (-var_fbbtsti);
        let assign18370_e18655: f64 = (assign18370_e18653 / var_fmaxr);
        let assign18370_e18656: f64 = (assign18370_e18655).exp();
        (assign18370_e18656, (assign18370_e18656 * (-((assign18370_e18653 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))), (assign18370_e18656 * (-((assign18370_e18653 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))), (assign18370_e18656 * (-((assign18370_e18653 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))), (assign18370_e18656 * (-((assign18370_e18653 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18370_e18658;
        var_tmp_dn6 = assign18370_e18658_d_n6;
        var_tmp_dn7 = assign18370_e18658_d_n7;
        var_tmp_dn8 = assign18370_e18658_d_n8;
        var_tmp_dn9 = assign18370_e18658_d_n9;

        *var_erfcpos_slot = var_erfcpos;
        *var_erfcpos_dn6_slot = var_erfcpos_dn6;
        *var_erfcpos_dn7_slot = var_erfcpos_dn7;
        *var_erfcpos_dn8_slot = var_erfcpos_dn8;
        *var_erfcpos_dn9_slot = var_erfcpos_dn9;
        *var_erfctimesexpmtat_slot = var_erfctimesexpmtat;
        *var_erfctimesexpmtat_dn6_slot = var_erfctimesexpmtat_dn6;
        *var_erfctimesexpmtat_dn7_slot = var_erfctimesexpmtat_dn7;
        *var_erfctimesexpmtat_dn8_slot = var_erfctimesexpmtat_dn8;
        *var_erfctimesexpmtat_dn9_slot = var_erfctimesexpmtat_dn9;
        *var_fmaxr_slot = var_fmaxr;
        *var_fmaxr_dn6_slot = var_fmaxr_dn6;
        *var_fmaxr_dn7_slot = var_fmaxr_dn7;
        *var_fmaxr_dn8_slot = var_fmaxr_dn8;
        *var_fmaxr_dn9_slot = var_fmaxr_dn9;
        *var_gammamax_slot = var_gammamax;
        *var_gammamax_dn6_slot = var_gammamax_dn6;
        *var_gammamax_dn7_slot = var_gammamax_dn7;
        *var_gammamax_dn8_slot = var_gammamax_dn8;
        *var_gammamax_dn9_slot = var_gammamax_dn9;
        *var_guard324_slot = var_guard324;
        *var_guard325_slot = var_guard325;
        *var_guard326_slot = var_guard326;
        *var_guard327_slot = var_guard327;
        *var_guard328_slot = var_guard328;
        *var_guard329_slot = var_guard329;
        *var_guard330_slot = var_guard330;
        *var_guard331_slot = var_guard331;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_ktat_slot = var_ktat;
        *var_ktat_dn6_slot = var_ktat_dn6;
        *var_ktat_dn7_slot = var_ktat_dn7;
        *var_ktat_dn8_slot = var_ktat_dn8;
        *var_ktat_dn9_slot = var_ktat_dn9;
        *var_ltat_slot = var_ltat;
        *var_ltat_dn6_slot = var_ltat_dn6;
        *var_ltat_dn7_slot = var_ltat_dn7;
        *var_ltat_dn8_slot = var_ltat_dn8;
        *var_ltat_dn9_slot = var_ltat_dn9;
        *var_mtat_slot = var_mtat;
        *var_mtat_dn6_slot = var_mtat_dn6;
        *var_mtat_dn7_slot = var_mtat_dn7;
        *var_mtat_dn8_slot = var_mtat_dn8;
        *var_mtat_dn9_slot = var_mtat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_terfc_slot = var_terfc;
        *var_terfc_dn6_slot = var_terfc_dn6;
        *var_terfc_dn7_slot = var_terfc_dn7;
        *var_terfc_dn8_slot = var_terfc_dn8;
        *var_terfc_dn9_slot = var_terfc_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wtat_slot = var_wtat;
        *var_wtat_dn6_slot = var_wtat_dn6;
        *var_wtat_dn7_slot = var_wtat_dn7;
        *var_wtat_dn8_slot = var_wtat_dn8;
        *var_wtat_dn9_slot = var_wtat_dn9;
        *var_xerfc_slot = var_xerfc;
        *var_xerfc_dn6_slot = var_xerfc_dn6;
        *var_xerfc_dn7_slot = var_xerfc_dn7;
        *var_xerfc_dn8_slot = var_xerfc_dn8;
        *var_xerfc_dn9_slot = var_xerfc_dn9;
        *var_ysq_slot = var_ysq;
        *var_ysq_dn6_slot = var_ysq_dn6;
        *var_ysq_dn7_slot = var_ysq_dn7;
        *var_ysq_dn8_slot = var_ysq_dn8;
        *var_ysq_dn9_slot = var_ysq_dn9;
    }

    pub(super) fn stamp_transient_block_31(
        p: &Parameters,
        var_alphaav: f64,
        var_atatgat: f64,
        var_btatpartgat: f64,
        var_fbbtsti: f64,
        var_fmaxr: f64,
        var_fmaxr_dn6: f64,
        var_fmaxr_dn7: f64,
        var_fmaxr_dn8: f64,
        var_fmaxr_dn9: f64,
        var_fstopsti: f64,
        var_ftdgat: f64,
        var_guard175: f64,
        var_guard192: f64,
        var_guard319: f64,
        var_guard329: f64,
        var_guard331: f64,
        var_idmult: f64,
        var_idsatgat: f64,
        var_lgsource_i: f64,
        var_one_minus_pgat: f64,
        var_one_over_one_minus_pgat: f64,
        var_slopesti: f64,
        var_two_psistar: f64,
        var_v2: f64,
        var_vav: f64,
        var_vbigat: f64,
        var_vbirgatinv: f64,
        var_vbrinvsti: f64,
        var_vjsrh: f64,
        var_wdepnulrgat: f64,
        var_zinv: f64,
        var_asrh_slot: &mut f64,
        var_asrh_dn6_slot: &mut f64,
        var_asrh_dn7_slot: &mut f64,
        var_asrh_dn8_slot: &mut f64,
        var_asrh_dn9_slot: &mut f64,
        var_btat_slot: &mut f64,
        var_btat_dn6_slot: &mut f64,
        var_btat_dn7_slot: &mut f64,
        var_btat_dn8_slot: &mut f64,
        var_btat_dn9_slot: &mut f64,
        var_dwsrh_slot: &mut f64,
        var_fbreakdown_slot: &mut f64,
        var_fbreakdown_dn6_slot: &mut f64,
        var_fbreakdown_dn7_slot: &mut f64,
        var_fbreakdown_dn8_slot: &mut f64,
        var_fbreakdown_dn9_slot: &mut f64,
        var_guard332_slot: &mut f64,
        var_guard333_slot: &mut f64,
        var_guard334_slot: &mut f64,
        var_guard335_slot: &mut f64,
        var_guard336_slot: &mut f64,
        var_guard337_slot: &mut f64,
        var_guard338_slot: &mut f64,
        var_guard339_slot: &mut f64,
        var_guard340_slot: &mut f64,
        var_guard341_slot: &mut f64,
        var_ibbt_slot: &mut f64,
        var_ibbt_dn6_slot: &mut f64,
        var_ibbt_dn7_slot: &mut f64,
        var_ibbt_dn8_slot: &mut f64,
        var_ibbt_dn9_slot: &mut f64,
        var_id__blk212_slot: &mut f64,
        var_ijungat_slot: &mut f64,
        var_ijungat_dn6_slot: &mut f64,
        var_ijungat_dn7_slot: &mut f64,
        var_ijungat_dn8_slot: &mut f64,
        var_ijungat_dn9_slot: &mut f64,
        var_ijunsti_slot: &mut f64,
        var_ijunsti_dn6_slot: &mut f64,
        var_ijunsti_dn7_slot: &mut f64,
        var_ijunsti_dn8_slot: &mut f64,
        var_ijunsti_dn9_slot: &mut f64,
        var_isrh_slot: &mut f64,
        var_isrh_dn6_slot: &mut f64,
        var_isrh_dn7_slot: &mut f64,
        var_isrh_dn8_slot: &mut f64,
        var_isrh_dn9_slot: &mut f64,
        var_itat_slot: &mut f64,
        var_itat_dn6_slot: &mut f64,
        var_itat_dn7_slot: &mut f64,
        var_itat_dn8_slot: &mut f64,
        var_itat_dn9_slot: &mut f64,
        var_sqrtumax_slot: &mut f64,
        var_sqrtumax_dn6_slot: &mut f64,
        var_sqrtumax_dn7_slot: &mut f64,
        var_sqrtumax_dn8_slot: &mut f64,
        var_sqrtumax_dn9_slot: &mut f64,
        var_tmp_slot: &mut f64,
        var_tmp_dn6_slot: &mut f64,
        var_tmp_dn7_slot: &mut f64,
        var_tmp_dn8_slot: &mut f64,
        var_tmp_dn9_slot: &mut f64,
        var_twoatatoverthreebtat_slot: &mut f64,
        var_twoatatoverthreebtat_dn6_slot: &mut f64,
        var_twoatatoverthreebtat_dn7_slot: &mut f64,
        var_twoatatoverthreebtat_dn8_slot: &mut f64,
        var_twoatatoverthreebtat_dn9_slot: &mut f64,
        var_umax_slot: &mut f64,
        var_umax_dn6_slot: &mut f64,
        var_umax_dn7_slot: &mut f64,
        var_umax_dn8_slot: &mut f64,
        var_umax_dn9_slot: &mut f64,
        var_umaxbeforelimiting_slot: &mut f64,
        var_umaxbeforelimiting_dn6_slot: &mut f64,
        var_umaxbeforelimiting_dn7_slot: &mut f64,
        var_umaxbeforelimiting_dn8_slot: &mut f64,
        var_umaxbeforelimiting_dn9_slot: &mut f64,
        var_umaxpoweronepointfive_slot: &mut f64,
        var_umaxpoweronepointfive_dn6_slot: &mut f64,
        var_umaxpoweronepointfive_dn7_slot: &mut f64,
        var_umaxpoweronepointfive_dn8_slot: &mut f64,
        var_umaxpoweronepointfive_dn9_slot: &mut f64,
        var_vbi_minus_vjsrh_slot: &mut f64,
        var_wdep_slot: &mut f64,
        var_wdep_dn6_slot: &mut f64,
        var_wdep_dn7_slot: &mut f64,
        var_wdep_dn8_slot: &mut f64,
        var_wdep_dn9_slot: &mut f64,
        var_wgamma_slot: &mut f64,
        var_wgamma_dn6_slot: &mut f64,
        var_wgamma_dn7_slot: &mut f64,
        var_wgamma_dn8_slot: &mut f64,
        var_wgamma_dn9_slot: &mut f64,
        var_wsrh_slot: &mut f64,
        var_wsrhstep_slot: &mut f64,
    ) {
        let mut var_asrh: f64 = *var_asrh_slot;
        let mut var_asrh_dn6: f64 = *var_asrh_dn6_slot;
        let mut var_asrh_dn7: f64 = *var_asrh_dn7_slot;
        let mut var_asrh_dn8: f64 = *var_asrh_dn8_slot;
        let mut var_asrh_dn9: f64 = *var_asrh_dn9_slot;
        let mut var_btat: f64 = *var_btat_slot;
        let mut var_btat_dn6: f64 = *var_btat_dn6_slot;
        let mut var_btat_dn7: f64 = *var_btat_dn7_slot;
        let mut var_btat_dn8: f64 = *var_btat_dn8_slot;
        let mut var_btat_dn9: f64 = *var_btat_dn9_slot;
        let mut var_dwsrh: f64 = *var_dwsrh_slot;
        let mut var_fbreakdown: f64 = *var_fbreakdown_slot;
        let mut var_fbreakdown_dn6: f64 = *var_fbreakdown_dn6_slot;
        let mut var_fbreakdown_dn7: f64 = *var_fbreakdown_dn7_slot;
        let mut var_fbreakdown_dn8: f64 = *var_fbreakdown_dn8_slot;
        let mut var_fbreakdown_dn9: f64 = *var_fbreakdown_dn9_slot;
        let mut var_guard332: f64 = *var_guard332_slot;
        let mut var_guard333: f64 = *var_guard333_slot;
        let mut var_guard334: f64 = *var_guard334_slot;
        let mut var_guard335: f64 = *var_guard335_slot;
        let mut var_guard336: f64 = *var_guard336_slot;
        let mut var_guard337: f64 = *var_guard337_slot;
        let mut var_guard338: f64 = *var_guard338_slot;
        let mut var_guard339: f64 = *var_guard339_slot;
        let mut var_guard340: f64 = *var_guard340_slot;
        let mut var_guard341: f64 = *var_guard341_slot;
        let mut var_ibbt: f64 = *var_ibbt_slot;
        let mut var_ibbt_dn6: f64 = *var_ibbt_dn6_slot;
        let mut var_ibbt_dn7: f64 = *var_ibbt_dn7_slot;
        let mut var_ibbt_dn8: f64 = *var_ibbt_dn8_slot;
        let mut var_ibbt_dn9: f64 = *var_ibbt_dn9_slot;
        let mut var_id__blk212: f64 = *var_id__blk212_slot;
        let mut var_ijungat: f64 = *var_ijungat_slot;
        let mut var_ijungat_dn6: f64 = *var_ijungat_dn6_slot;
        let mut var_ijungat_dn7: f64 = *var_ijungat_dn7_slot;
        let mut var_ijungat_dn8: f64 = *var_ijungat_dn8_slot;
        let mut var_ijungat_dn9: f64 = *var_ijungat_dn9_slot;
        let mut var_ijunsti: f64 = *var_ijunsti_slot;
        let mut var_ijunsti_dn6: f64 = *var_ijunsti_dn6_slot;
        let mut var_ijunsti_dn7: f64 = *var_ijunsti_dn7_slot;
        let mut var_ijunsti_dn8: f64 = *var_ijunsti_dn8_slot;
        let mut var_ijunsti_dn9: f64 = *var_ijunsti_dn9_slot;
        let mut var_isrh: f64 = *var_isrh_slot;
        let mut var_isrh_dn6: f64 = *var_isrh_dn6_slot;
        let mut var_isrh_dn7: f64 = *var_isrh_dn7_slot;
        let mut var_isrh_dn8: f64 = *var_isrh_dn8_slot;
        let mut var_isrh_dn9: f64 = *var_isrh_dn9_slot;
        let mut var_itat: f64 = *var_itat_slot;
        let mut var_itat_dn6: f64 = *var_itat_dn6_slot;
        let mut var_itat_dn7: f64 = *var_itat_dn7_slot;
        let mut var_itat_dn8: f64 = *var_itat_dn8_slot;
        let mut var_itat_dn9: f64 = *var_itat_dn9_slot;
        let mut var_sqrtumax: f64 = *var_sqrtumax_slot;
        let mut var_sqrtumax_dn6: f64 = *var_sqrtumax_dn6_slot;
        let mut var_sqrtumax_dn7: f64 = *var_sqrtumax_dn7_slot;
        let mut var_sqrtumax_dn8: f64 = *var_sqrtumax_dn8_slot;
        let mut var_sqrtumax_dn9: f64 = *var_sqrtumax_dn9_slot;
        let mut var_tmp: f64 = *var_tmp_slot;
        let mut var_tmp_dn6: f64 = *var_tmp_dn6_slot;
        let mut var_tmp_dn7: f64 = *var_tmp_dn7_slot;
        let mut var_tmp_dn8: f64 = *var_tmp_dn8_slot;
        let mut var_tmp_dn9: f64 = *var_tmp_dn9_slot;
        let mut var_twoatatoverthreebtat: f64 = *var_twoatatoverthreebtat_slot;
        let mut var_twoatatoverthreebtat_dn6: f64 = *var_twoatatoverthreebtat_dn6_slot;
        let mut var_twoatatoverthreebtat_dn7: f64 = *var_twoatatoverthreebtat_dn7_slot;
        let mut var_twoatatoverthreebtat_dn8: f64 = *var_twoatatoverthreebtat_dn8_slot;
        let mut var_twoatatoverthreebtat_dn9: f64 = *var_twoatatoverthreebtat_dn9_slot;
        let mut var_umax: f64 = *var_umax_slot;
        let mut var_umax_dn6: f64 = *var_umax_dn6_slot;
        let mut var_umax_dn7: f64 = *var_umax_dn7_slot;
        let mut var_umax_dn8: f64 = *var_umax_dn8_slot;
        let mut var_umax_dn9: f64 = *var_umax_dn9_slot;
        let mut var_umaxbeforelimiting: f64 = *var_umaxbeforelimiting_slot;
        let mut var_umaxbeforelimiting_dn6: f64 = *var_umaxbeforelimiting_dn6_slot;
        let mut var_umaxbeforelimiting_dn7: f64 = *var_umaxbeforelimiting_dn7_slot;
        let mut var_umaxbeforelimiting_dn8: f64 = *var_umaxbeforelimiting_dn8_slot;
        let mut var_umaxbeforelimiting_dn9: f64 = *var_umaxbeforelimiting_dn9_slot;
        let mut var_umaxpoweronepointfive: f64 = *var_umaxpoweronepointfive_slot;
        let mut var_umaxpoweronepointfive_dn6: f64 = *var_umaxpoweronepointfive_dn6_slot;
        let mut var_umaxpoweronepointfive_dn7: f64 = *var_umaxpoweronepointfive_dn7_slot;
        let mut var_umaxpoweronepointfive_dn8: f64 = *var_umaxpoweronepointfive_dn8_slot;
        let mut var_umaxpoweronepointfive_dn9: f64 = *var_umaxpoweronepointfive_dn9_slot;
        let mut var_vbi_minus_vjsrh: f64 = *var_vbi_minus_vjsrh_slot;
        let mut var_wdep: f64 = *var_wdep_slot;
        let mut var_wdep_dn6: f64 = *var_wdep_dn6_slot;
        let mut var_wdep_dn7: f64 = *var_wdep_dn7_slot;
        let mut var_wdep_dn8: f64 = *var_wdep_dn8_slot;
        let mut var_wdep_dn9: f64 = *var_wdep_dn9_slot;
        let mut var_wgamma: f64 = *var_wgamma_slot;
        let mut var_wgamma_dn6: f64 = *var_wgamma_dn6_slot;
        let mut var_wgamma_dn7: f64 = *var_wgamma_dn7_slot;
        let mut var_wgamma_dn8: f64 = *var_wgamma_dn8_slot;
        let mut var_wgamma_dn9: f64 = *var_wgamma_dn9_slot;
        let mut var_wsrh: f64 = *var_wsrh_slot;
        let mut var_wsrhstep: f64 = *var_wsrhstep_slot;

        let assign18380_e18660: f64 = (-var_fbbtsti);
        let assign18380_e18662: f64 = (assign18380_e18660 / var_fmaxr);
        let assign18380_e18664: f64 = if assign18380_e18662 < 0.0 { 1.0 } else { 0.0 };
        var_guard332 = assign18380_e18664;

        let (assign18390_e18715, assign18390_e18715_d_n6, assign18390_e18715_d_n7, assign18390_e18715_d_n8, assign18390_e18715_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) && (var_guard331 == 0.0)) && (var_guard332 != 0.0)) {
        let assign18390_e18682: f64 = (-230.25850929940458);
        let assign18390_e18684: f64 = (-var_fbbtsti);
        let assign18390_e18686: f64 = (assign18390_e18684 / var_fmaxr);
        let assign18390_e18687: f64 = (assign18390_e18682 - assign18390_e18686);
        let assign18390_e18691: f64 = (-230.25850929940458);
        let assign18390_e18693: f64 = (-var_fbbtsti);
        let assign18390_e18695: f64 = (assign18390_e18693 / var_fmaxr);
        let assign18390_e18696: f64 = (assign18390_e18691 - assign18390_e18695);
        let assign18390_e18699: f64 = (-230.25850929940458);
        let assign18390_e18701: f64 = (-var_fbbtsti);
        let assign18390_e18703: f64 = (assign18390_e18701 / var_fmaxr);
        let assign18390_e18704: f64 = (assign18390_e18699 - assign18390_e18703);
        let assign18390_e18706: f64 = (assign18390_e18704 * 0.3333333333333333);
        let assign18390_e18707: f64 = (1.0 + assign18390_e18706);
        let assign18390_e18708: f64 = (assign18390_e18696 * assign18390_e18707);
        let assign18390_e18709: f64 = (0.5 * assign18390_e18708);
        let assign18390_e18710: f64 = (1.0 + assign18390_e18709);
        let assign18390_e18711: f64 = (assign18390_e18687 * assign18390_e18710);
        let assign18390_e18712: f64 = (1.0 + assign18390_e18711);
        let assign18390_e18713: f64 = (1e-100 / assign18390_e18712);
        (assign18390_e18713, (-((1e-100 * (((-(-((assign18390_e18684 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign18390_e18710) + (assign18390_e18687 * (0.5 * (((-(-((assign18390_e18693 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * assign18390_e18707) + (assign18390_e18696 * ((-(-((assign18390_e18701 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18390_e18712 * assign18390_e18712))), (-((1e-100 * (((-(-((assign18390_e18684 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign18390_e18710) + (assign18390_e18687 * (0.5 * (((-(-((assign18390_e18693 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * assign18390_e18707) + (assign18390_e18696 * ((-(-((assign18390_e18701 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18390_e18712 * assign18390_e18712))), (-((1e-100 * (((-(-((assign18390_e18684 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign18390_e18710) + (assign18390_e18687 * (0.5 * (((-(-((assign18390_e18693 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * assign18390_e18707) + (assign18390_e18696 * ((-(-((assign18390_e18701 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18390_e18712 * assign18390_e18712))), (-((1e-100 * (((-(-((assign18390_e18684 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign18390_e18710) + (assign18390_e18687 * (0.5 * (((-(-((assign18390_e18693 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * assign18390_e18707) + (assign18390_e18696 * ((-(-((assign18390_e18701 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr)))) * 0.3333333333333333))))))) / (assign18390_e18712 * assign18390_e18712))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18390_e18715;
        var_tmp_dn6 = assign18390_e18715_d_n6;
        var_tmp_dn7 = assign18390_e18715_d_n7;
        var_tmp_dn8 = assign18390_e18715_d_n8;
        var_tmp_dn9 = assign18390_e18715_d_n9;

        let (assign18400_e18764, assign18400_e18764_d_n6, assign18400_e18764_d_n7, assign18400_e18764_d_n8, assign18400_e18764_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) && (var_guard331 == 0.0)) && (var_guard332 == 0.0)) {
        let assign18400_e18734: f64 = (-var_fbbtsti);
        let assign18400_e18736: f64 = (assign18400_e18734 / var_fmaxr);
        let assign18400_e18738: f64 = (assign18400_e18736 - 230.25850929940458);
        let assign18400_e18742: f64 = (-var_fbbtsti);
        let assign18400_e18744: f64 = (assign18400_e18742 / var_fmaxr);
        let assign18400_e18746: f64 = (assign18400_e18744 - 230.25850929940458);
        let assign18400_e18749: f64 = (-var_fbbtsti);
        let assign18400_e18751: f64 = (assign18400_e18749 / var_fmaxr);
        let assign18400_e18753: f64 = (assign18400_e18751 - 230.25850929940458);
        let assign18400_e18755: f64 = (assign18400_e18753 * 0.3333333333333333);
        let assign18400_e18756: f64 = (1.0 + assign18400_e18755);
        let assign18400_e18757: f64 = (assign18400_e18746 * assign18400_e18756);
        let assign18400_e18758: f64 = (0.5 * assign18400_e18757);
        let assign18400_e18759: f64 = (1.0 + assign18400_e18758);
        let assign18400_e18760: f64 = (assign18400_e18738 * assign18400_e18759);
        let assign18400_e18761: f64 = (1.0 + assign18400_e18760);
        let assign18400_e18762: f64 = (1e100 * assign18400_e18761);
        (assign18400_e18762, (1e100 * (((-((assign18400_e18734 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign18400_e18759) + (assign18400_e18738 * (0.5 * (((-((assign18400_e18742 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * assign18400_e18756) + (assign18400_e18746 * ((-((assign18400_e18749 * var_fmaxr_dn6) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18400_e18734 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign18400_e18759) + (assign18400_e18738 * (0.5 * (((-((assign18400_e18742 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * assign18400_e18756) + (assign18400_e18746 * ((-((assign18400_e18749 * var_fmaxr_dn7) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18400_e18734 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign18400_e18759) + (assign18400_e18738 * (0.5 * (((-((assign18400_e18742 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * assign18400_e18756) + (assign18400_e18746 * ((-((assign18400_e18749 * var_fmaxr_dn8) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))), (1e100 * (((-((assign18400_e18734 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign18400_e18759) + (assign18400_e18738 * (0.5 * (((-((assign18400_e18742 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * assign18400_e18756) + (assign18400_e18746 * ((-((assign18400_e18749 * var_fmaxr_dn9) / (var_fmaxr * var_fmaxr))) * 0.3333333333333333))))))),)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18400_e18764;
        var_tmp_dn6 = assign18400_e18764_d_n6;
        var_tmp_dn7 = assign18400_e18764_d_n7;
        var_tmp_dn8 = assign18400_e18764_d_n8;
        var_tmp_dn9 = assign18400_e18764_d_n9;

        let (assign18410_e18784, assign18410_e18784_d_n6, assign18410_e18784_d_n7, assign18410_e18784_d_n8, assign18410_e18784_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard329 == 0.0)) {
        let assign18410_e18777: f64 = (var_v2 * var_fmaxr);
        let assign18410_e18779: f64 = (assign18410_e18777 * var_fmaxr);
        let assign18410_e18781: f64 = (assign18410_e18779 * var_tmp);
        let assign18410_e18782: f64 = (p.p869 * assign18410_e18781);
        (assign18410_e18782, (p.p869 * (((((var_v2 * var_fmaxr_dn6) * var_fmaxr) + (assign18410_e18777 * var_fmaxr_dn6)) * var_tmp) + (assign18410_e18779 * var_tmp_dn6))), (p.p869 * (((((var_v2 * var_fmaxr_dn7) * var_fmaxr) + (assign18410_e18777 * var_fmaxr_dn7)) * var_tmp) + (assign18410_e18779 * var_tmp_dn7))), (p.p869 * (((((var_v2 * var_fmaxr_dn8) * var_fmaxr) + (assign18410_e18777 * var_fmaxr_dn8)) * var_tmp) + (assign18410_e18779 * var_tmp_dn8))), (p.p869 * (((((var_v2 * var_fmaxr_dn9) * var_fmaxr) + (assign18410_e18777 * var_fmaxr_dn9)) * var_tmp) + (assign18410_e18779 * var_tmp_dn9))),)
    } else {
        (var_ibbt, var_ibbt_dn6, var_ibbt_dn7, var_ibbt_dn8, var_ibbt_dn9,)
    }
};
        var_ibbt = assign18410_e18784;
        var_ibbt_dn6 = assign18410_e18784_d_n6;
        var_ibbt_dn7 = assign18410_e18784_d_n7;
        var_ibbt_dn8 = assign18410_e18784_d_n8;
        var_ibbt_dn9 = assign18410_e18784_d_n9;

        let assign18420_e18787: f64 = if p.p878 > 1000.0 { 1.0 } else { 0.0 };
        var_guard333 = assign18420_e18787;

        let (assign18430_e18798, assign18430_e18798_d_n6, assign18430_e18798_d_n7, assign18430_e18798_d_n8, assign18430_e18798_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard333 != 0.0)) {
        (1.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign18430_e18798;
        var_fbreakdown_dn6 = assign18430_e18798_d_n6;
        var_fbreakdown_dn7 = assign18430_e18798_d_n7;
        var_fbreakdown_dn8 = assign18430_e18798_d_n8;
        var_fbreakdown_dn9 = assign18430_e18798_d_n9;

        let assign18440_e18801: f64 = (-var_alphaav);
        let assign18440_e18803: f64 = (assign18440_e18801 * p.p878);
        let assign18440_e18804: f64 = if var_vav > assign18440_e18803 { 1.0 } else { 0.0 };
        var_guard334 = assign18440_e18804;

        let assign18450_e18807: f64 = if p.p881 == 4.0 { 1.0 } else { 0.0 };
        var_guard335 = assign18450_e18807;

        let (assign18460_e18837, assign18460_e18837_d_n6, assign18460_e18837_d_n7, assign18460_e18837_d_n8, assign18460_e18837_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard333 == 0.0)) && (var_guard334 != 0.0)) && (var_guard335 != 0.0)) {
        let assign18460_e18823: f64 = (var_vav * var_vbrinvsti);
        let assign18460_e18826: f64 = (var_vav * var_vbrinvsti);
        let assign18460_e18827: f64 = (assign18460_e18823 * assign18460_e18826);
        let assign18460_e18830: f64 = (var_vav * var_vbrinvsti);
        let assign18460_e18831: f64 = (assign18460_e18827 * assign18460_e18830);
        let assign18460_e18834: f64 = (var_vav * var_vbrinvsti);
        let assign18460_e18835: f64 = (assign18460_e18831 * assign18460_e18834);
        (assign18460_e18835, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18460_e18837;
        var_tmp_dn6 = assign18460_e18837_d_n6;
        var_tmp_dn7 = assign18460_e18837_d_n7;
        var_tmp_dn8 = assign18460_e18837_d_n8;
        var_tmp_dn9 = assign18460_e18837_d_n9;

        let (assign18470_e18859, assign18470_e18859_d_n6, assign18470_e18859_d_n7, assign18470_e18859_d_n8, assign18470_e18859_d_n9,) = {
    if ((((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard333 == 0.0)) && (var_guard334 != 0.0)) && (var_guard335 == 0.0)) {
        let assign18470_e18854: f64 = (var_vav * var_vbrinvsti);
        let assign18470_e18855: f64 = (assign18470_e18854).abs();
        let assign18470_e18857: f64 = (assign18470_e18855).powf(p.p881);
        (assign18470_e18857, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18470_e18859;
        var_tmp_dn6 = assign18470_e18859_d_n6;
        var_tmp_dn7 = assign18470_e18859_d_n7;
        var_tmp_dn8 = assign18470_e18859_d_n8;
        var_tmp_dn9 = assign18470_e18859_d_n9;

        let (assign18480_e18877, assign18480_e18877_d_n6, assign18480_e18877_d_n7, assign18480_e18877_d_n8, assign18480_e18877_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard333 == 0.0)) && (var_guard334 != 0.0)) {
        let assign18480_e18874: f64 = (1.0 - var_tmp);
        let assign18480_e18875: f64 = (1.0 / assign18480_e18874);
        (assign18480_e18875, (-((-var_tmp_dn6) / (assign18480_e18874 * assign18480_e18874))), (-((-var_tmp_dn7) / (assign18480_e18874 * assign18480_e18874))), (-((-var_tmp_dn8) / (assign18480_e18874 * assign18480_e18874))), (-((-var_tmp_dn9) / (assign18480_e18874 * assign18480_e18874))),)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign18480_e18877;
        var_fbreakdown_dn6 = assign18480_e18877_d_n6;
        var_fbreakdown_dn7 = assign18480_e18877_d_n7;
        var_fbreakdown_dn8 = assign18480_e18877_d_n8;
        var_fbreakdown_dn9 = assign18480_e18877_d_n9;

        let (assign18490_e18900, assign18490_e18900_d_n6, assign18490_e18900_d_n7, assign18490_e18900_d_n8, assign18490_e18900_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) && (var_guard333 == 0.0)) && (var_guard334 == 0.0)) {
        let assign18490_e18894: f64 = (var_alphaav * p.p878);
        let assign18490_e18895: f64 = (var_vav + assign18490_e18894);
        let assign18490_e18897: f64 = (assign18490_e18895 * var_slopesti);
        let assign18490_e18898: f64 = (var_fstopsti + assign18490_e18897);
        (assign18490_e18898, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_fbreakdown, var_fbreakdown_dn6, var_fbreakdown_dn7, var_fbreakdown_dn8, var_fbreakdown_dn9,)
    }
};
        var_fbreakdown = assign18490_e18900;
        var_fbreakdown_dn6 = assign18490_e18900_d_n6;
        var_fbreakdown_dn7 = assign18490_e18900_d_n7;
        var_fbreakdown_dn8 = assign18490_e18900_d_n8;
        var_fbreakdown_dn9 = assign18490_e18900_d_n9;

        let (assign18500_e18919, assign18500_e18919_d_n6, assign18500_e18919_d_n7, assign18500_e18919_d_n8, assign18500_e18919_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard319 == 0.0)) {
        let assign18500_e18910: f64 = (var_id__blk212 + var_isrh);
        let assign18500_e18912: f64 = (assign18500_e18910 + var_itat);
        let assign18500_e18914: f64 = (assign18500_e18912 + var_ibbt);
        let assign18500_e18915: f64 = (p.p29 * assign18500_e18914);
        let assign18500_e18917: f64 = (assign18500_e18915 * var_fbreakdown);
        (assign18500_e18917, (((p.p29 * ((var_isrh_dn6 + var_itat_dn6) + var_ibbt_dn6)) * var_fbreakdown) + (assign18500_e18915 * var_fbreakdown_dn6)), (((p.p29 * ((var_isrh_dn7 + var_itat_dn7) + var_ibbt_dn7)) * var_fbreakdown) + (assign18500_e18915 * var_fbreakdown_dn7)), (((p.p29 * ((var_isrh_dn8 + var_itat_dn8) + var_ibbt_dn8)) * var_fbreakdown) + (assign18500_e18915 * var_fbreakdown_dn8)), (((p.p29 * ((var_isrh_dn9 + var_itat_dn9) + var_ibbt_dn9)) * var_fbreakdown) + (assign18500_e18915 * var_fbreakdown_dn9)),)
    } else {
        (var_ijunsti, var_ijunsti_dn6, var_ijunsti_dn7, var_ijunsti_dn8, var_ijunsti_dn9,)
    }
};
        var_ijunsti = assign18500_e18919;
        var_ijunsti_dn6 = assign18500_e18919_d_n6;
        var_ijunsti_dn7 = assign18500_e18919_d_n7;
        var_ijunsti_dn8 = assign18500_e18919_d_n8;
        var_ijunsti_dn9 = assign18500_e18919_d_n9;

        let assign18510_e18922: f64 = if var_lgsource_i == 0.0 { 1.0 } else { 0.0 };
        var_guard336 = assign18510_e18922;

        let (assign18520_e18930, assign18520_e18930_d_n6, assign18520_e18930_d_n7, assign18520_e18930_d_n8, assign18520_e18930_d_n9,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_ijungat, var_ijungat_dn6, var_ijungat_dn7, var_ijungat_dn8, var_ijungat_dn9,)
    }
};
        var_ijungat = assign18520_e18930;
        var_ijungat_dn6 = assign18520_e18930_d_n6;
        var_ijungat_dn7 = assign18520_e18930_d_n7;
        var_ijungat_dn8 = assign18520_e18930_d_n8;
        var_ijungat_dn9 = assign18520_e18930_d_n9;

        let (assign18530_e18941,) = {
    if (((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) {
        let assign18530_e18939: f64 = (var_idsatgat * var_idmult);
        (assign18530_e18939,)
    } else {
        (var_id__blk212,)
    }
};
        var_id__blk212 = assign18530_e18941;

        let assign18540_e18948: f64 = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };
        var_guard337 = assign18540_e18948;

        let (assign18550_e18959, assign18550_e18959_d_n6, assign18550_e18959_d_n7, assign18550_e18959_d_n8, assign18550_e18959_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign18550_e18959;
        var_isrh_dn6 = assign18550_e18959_d_n6;
        var_isrh_dn7 = assign18550_e18959_d_n7;
        var_isrh_dn8 = assign18550_e18959_d_n8;
        var_isrh_dn9 = assign18550_e18959_d_n9;

        let (assign18560_e18973,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign18560_e18971: f64 = (var_vbigat - var_vjsrh);
        (assign18560_e18971,)
    } else {
        (var_vbi_minus_vjsrh,)
    }
};
        var_vbi_minus_vjsrh = assign18560_e18973;

        let (assign18570_e18992,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign18570_e18987: f64 = (var_two_psistar / var_vbi_minus_vjsrh);
        let assign18570_e18988: f64 = (1.0 - assign18570_e18987);
        let assign18570_e18989: f64 = (assign18570_e18988).sqrt();
        let assign18570_e18990: f64 = (1.0 - assign18570_e18989);
        (assign18570_e18990,)
    } else {
        (var_wsrhstep,)
    }
};
        var_wsrhstep = assign18570_e18992;

        let assign18580_e18995: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard338 = assign18580_e18995;

        let (assign18590_e19009,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) && (var_guard338 != 0.0)) {
        (0.0,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign18590_e19009;

        let (assign18600_e19041,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) && (var_guard338 == 0.0)) {
        let assign18600_e19024: f64 = (var_wsrhstep * var_wsrhstep);
        let assign18600_e19026: f64 = (var_wsrhstep).ln();
        let assign18600_e19027: f64 = (assign18600_e19024 * assign18600_e19026);
        let assign18600_e19030: f64 = (1.0 - var_wsrhstep);
        let assign18600_e19031: f64 = (assign18600_e19027 / assign18600_e19030);
        let assign18600_e19033: f64 = (assign18600_e19031 + var_wsrhstep);
        let assign18600_e19037: f64 = (2.0 * p.p850);
        let assign18600_e19038: f64 = (1.0 - assign18600_e19037);
        let assign18600_e19039: f64 = (assign18600_e19033 * assign18600_e19038);
        (assign18600_e19039,)
    } else {
        (var_dwsrh,)
    }
};
        var_dwsrh = assign18600_e19041;

        let (assign18610_e19055,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign18610_e19053: f64 = (var_wsrhstep + var_dwsrh);
        (assign18610_e19053,)
    } else {
        (var_wsrh,)
    }
};
        var_wsrh = assign18610_e19055;

        let assign18620_e19058: f64 = if p.p850 == 0.5 { 1.0 } else { 0.0 };
        var_guard339 = assign18620_e19058;

        let (assign18630_e19075, assign18630_e19075_d_n6, assign18630_e19075_d_n7, assign18630_e19075_d_n8, assign18630_e19075_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) && (var_guard339 != 0.0)) {
        let assign18630_e19072: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign18630_e19073: f64 = (assign18630_e19072).sqrt();
        (assign18630_e19073, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18630_e19075;
        var_tmp_dn6 = assign18630_e19075_d_n6;
        var_tmp_dn7 = assign18630_e19075_d_n7;
        var_tmp_dn8 = assign18630_e19075_d_n8;
        var_tmp_dn9 = assign18630_e19075_d_n9;

        let (assign18640_e19094, assign18640_e19094_d_n6, assign18640_e19094_d_n7, assign18640_e19094_d_n8, assign18640_e19094_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) && (var_guard339 == 0.0)) {
        let assign18640_e19090: f64 = (var_vbi_minus_vjsrh * var_vbirgatinv);
        let assign18640_e19092: f64 = (assign18640_e19090).powf(p.p850);
        (assign18640_e19092, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_tmp, var_tmp_dn6, var_tmp_dn7, var_tmp_dn8, var_tmp_dn9,)
    }
};
        var_tmp = assign18640_e19094;
        var_tmp_dn6 = assign18640_e19094_d_n6;
        var_tmp_dn7 = assign18640_e19094_d_n7;
        var_tmp_dn8 = assign18640_e19094_d_n8;
        var_tmp_dn9 = assign18640_e19094_d_n9;

        let (assign18650_e19108, assign18650_e19108_d_n6, assign18650_e19108_d_n7, assign18650_e19108_d_n8, assign18650_e19108_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign18650_e19106: f64 = (var_wdepnulrgat * var_tmp);
        (assign18650_e19106, (var_wdepnulrgat * var_tmp_dn6), (var_wdepnulrgat * var_tmp_dn7), (var_wdepnulrgat * var_tmp_dn8), (var_wdepnulrgat * var_tmp_dn9),)
    } else {
        (var_wdep, var_wdep_dn6, var_wdep_dn7, var_wdep_dn8, var_wdep_dn9,)
    }
};
        var_wdep = assign18650_e19108;
        var_wdep_dn6 = assign18650_e19108_d_n6;
        var_wdep_dn7 = assign18650_e19108_d_n7;
        var_wdep_dn8 = assign18650_e19108_d_n8;
        var_wdep_dn9 = assign18650_e19108_d_n9;

        let (assign18660_e19126, assign18660_e19126_d_n6, assign18660_e19126_d_n7, assign18660_e19126_d_n8, assign18660_e19126_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign18660_e19121: f64 = (var_zinv - 1.0);
        let assign18660_e19123: f64 = (assign18660_e19121 * var_wdep);
        let assign18660_e19124: f64 = (var_ftdgat * assign18660_e19123);
        (assign18660_e19124, (var_ftdgat * (assign18660_e19121 * var_wdep_dn6)), (var_ftdgat * (assign18660_e19121 * var_wdep_dn7)), (var_ftdgat * (assign18660_e19121 * var_wdep_dn8)), (var_ftdgat * (assign18660_e19121 * var_wdep_dn9)),)
    } else {
        (var_asrh, var_asrh_dn6, var_asrh_dn7, var_asrh_dn8, var_asrh_dn9,)
    }
};
        var_asrh = assign18660_e19126;
        var_asrh_dn6 = assign18660_e19126_d_n6;
        var_asrh_dn7 = assign18660_e19126_d_n7;
        var_asrh_dn8 = assign18660_e19126_d_n8;
        var_asrh_dn9 = assign18660_e19126_d_n9;

        let (assign18670_e19142, assign18670_e19142_d_n6, assign18670_e19142_d_n7, assign18670_e19142_d_n8, assign18670_e19142_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard337 == 0.0)) {
        let assign18670_e19139: f64 = (var_asrh * var_wsrh);
        let assign18670_e19140: f64 = (p.p859 * assign18670_e19139);
        (assign18670_e19140, (p.p859 * (var_asrh_dn6 * var_wsrh)), (p.p859 * (var_asrh_dn7 * var_wsrh)), (p.p859 * (var_asrh_dn8 * var_wsrh)), (p.p859 * (var_asrh_dn9 * var_wsrh)),)
    } else {
        (var_isrh, var_isrh_dn6, var_isrh_dn7, var_isrh_dn8, var_isrh_dn9,)
    }
};
        var_isrh = assign18670_e19142;
        var_isrh_dn6 = assign18670_e19142_d_n6;
        var_isrh_dn7 = assign18670_e19142_d_n7;
        var_isrh_dn8 = assign18670_e19142_d_n8;
        var_isrh_dn9 = assign18670_e19142_d_n9;

        let assign18680_e19145: f64 = if p.p864 == 0.0 { 1.0 } else { 0.0 };
        var_guard340 = assign18680_e19145;

        let (assign18690_e19156, assign18690_e19156_d_n6, assign18690_e19156_d_n7, assign18690_e19156_d_n8, assign18690_e19156_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 != 0.0)) {
        (0.0, 0.0, 0.0, 0.0, 0.0,)
    } else {
        (var_itat, var_itat_dn6, var_itat_dn7, var_itat_dn8, var_itat_dn9,)
    }
};
        var_itat = assign18690_e19156;
        var_itat_dn6 = assign18690_e19156_d_n6;
        var_itat_dn7 = assign18690_e19156_d_n7;
        var_itat_dn8 = assign18690_e19156_d_n8;
        var_itat_dn9 = assign18690_e19156_d_n9;

        let (assign18700_e19174, assign18700_e19174_d_n6, assign18700_e19174_d_n7, assign18700_e19174_d_n8, assign18700_e19174_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18700_e19169: f64 = (var_wdep * var_one_minus_pgat);
        let assign18700_e19171: f64 = (assign18700_e19169 / var_vbi_minus_vjsrh);
        let assign18700_e19172: f64 = (var_btatpartgat * assign18700_e19171);
        (assign18700_e19172, (var_btatpartgat * ((var_wdep_dn6 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn7 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn8 * var_one_minus_pgat) / var_vbi_minus_vjsrh)), (var_btatpartgat * ((var_wdep_dn9 * var_one_minus_pgat) / var_vbi_minus_vjsrh)),)
    } else {
        (var_btat, var_btat_dn6, var_btat_dn7, var_btat_dn8, var_btat_dn9,)
    }
};
        var_btat = assign18700_e19174;
        var_btat_dn6 = assign18700_e19174_d_n6;
        var_btat_dn7 = assign18700_e19174_d_n7;
        var_btat_dn8 = assign18700_e19174_d_n8;
        var_btat_dn9 = assign18700_e19174_d_n9;

        let (assign18710_e19190, assign18710_e19190_d_n6, assign18710_e19190_d_n7, assign18710_e19190_d_n8, assign18710_e19190_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18710_e19186: f64 = (0.666666666666667 * var_atatgat);
        let assign18710_e19188: f64 = (assign18710_e19186 / var_btat);
        (assign18710_e19188, (-((assign18710_e19186 * var_btat_dn6) / (var_btat * var_btat))), (-((assign18710_e19186 * var_btat_dn7) / (var_btat * var_btat))), (-((assign18710_e19186 * var_btat_dn8) / (var_btat * var_btat))), (-((assign18710_e19186 * var_btat_dn9) / (var_btat * var_btat))),)
    } else {
        (var_twoatatoverthreebtat, var_twoatatoverthreebtat_dn6, var_twoatatoverthreebtat_dn7, var_twoatatoverthreebtat_dn8, var_twoatatoverthreebtat_dn9,)
    }
};
        var_twoatatoverthreebtat = assign18710_e19190;
        var_twoatatoverthreebtat_dn6 = assign18710_e19190_d_n6;
        var_twoatatoverthreebtat_dn7 = assign18710_e19190_d_n7;
        var_twoatatoverthreebtat_dn8 = assign18710_e19190_d_n8;
        var_twoatatoverthreebtat_dn9 = assign18710_e19190_d_n9;

        let (assign18720_e19204, assign18720_e19204_d_n6, assign18720_e19204_d_n7, assign18720_e19204_d_n8, assign18720_e19204_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18720_e19202: f64 = (var_twoatatoverthreebtat * var_twoatatoverthreebtat);
        (assign18720_e19202, ((var_twoatatoverthreebtat_dn6 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn6)), ((var_twoatatoverthreebtat_dn7 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn7)), ((var_twoatatoverthreebtat_dn8 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn8)), ((var_twoatatoverthreebtat_dn9 * var_twoatatoverthreebtat) + (var_twoatatoverthreebtat * var_twoatatoverthreebtat_dn9)),)
    } else {
        (var_umaxbeforelimiting, var_umaxbeforelimiting_dn6, var_umaxbeforelimiting_dn7, var_umaxbeforelimiting_dn8, var_umaxbeforelimiting_dn9,)
    }
};
        var_umaxbeforelimiting = assign18720_e19204;
        var_umaxbeforelimiting_dn6 = assign18720_e19204_d_n6;
        var_umaxbeforelimiting_dn7 = assign18720_e19204_d_n7;
        var_umaxbeforelimiting_dn8 = assign18720_e19204_d_n8;
        var_umaxbeforelimiting_dn9 = assign18720_e19204_d_n9;

        let (assign18730_e19225, assign18730_e19225_d_n6, assign18730_e19225_d_n7, assign18730_e19225_d_n8, assign18730_e19225_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18730_e19216: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign18730_e19219: f64 = (var_umaxbeforelimiting * var_umaxbeforelimiting);
        let assign18730_e19221: f64 = (assign18730_e19219 + 1.0);
        let assign18730_e19222: f64 = (assign18730_e19216 / assign18730_e19221);
        let assign18730_e19223: f64 = (assign18730_e19222).sqrt();
        (assign18730_e19223, ((((((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)) * assign18730_e19221) - (assign18730_e19216 * ((var_umaxbeforelimiting_dn6 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn6)))) / (assign18730_e19221 * assign18730_e19221)) / (2.0 * assign18730_e19223)), ((((((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)) * assign18730_e19221) - (assign18730_e19216 * ((var_umaxbeforelimiting_dn7 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn7)))) / (assign18730_e19221 * assign18730_e19221)) / (2.0 * assign18730_e19223)), ((((((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)) * assign18730_e19221) - (assign18730_e19216 * ((var_umaxbeforelimiting_dn8 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn8)))) / (assign18730_e19221 * assign18730_e19221)) / (2.0 * assign18730_e19223)), ((((((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)) * assign18730_e19221) - (assign18730_e19216 * ((var_umaxbeforelimiting_dn9 * var_umaxbeforelimiting) + (var_umaxbeforelimiting * var_umaxbeforelimiting_dn9)))) / (assign18730_e19221 * assign18730_e19221)) / (2.0 * assign18730_e19223)),)
    } else {
        (var_umax, var_umax_dn6, var_umax_dn7, var_umax_dn8, var_umax_dn9,)
    }
};
        var_umax = assign18730_e19225;
        var_umax_dn6 = assign18730_e19225_d_n6;
        var_umax_dn7 = assign18730_e19225_d_n7;
        var_umax_dn8 = assign18730_e19225_d_n8;
        var_umax_dn9 = assign18730_e19225_d_n9;

        let (assign18740_e19238, assign18740_e19238_d_n6, assign18740_e19238_d_n7, assign18740_e19238_d_n8, assign18740_e19238_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18740_e19236: f64 = (var_umax).sqrt();
        (assign18740_e19236, (var_umax_dn6 / (2.0 * assign18740_e19236)), (var_umax_dn7 / (2.0 * assign18740_e19236)), (var_umax_dn8 / (2.0 * assign18740_e19236)), (var_umax_dn9 / (2.0 * assign18740_e19236)),)
    } else {
        (var_sqrtumax, var_sqrtumax_dn6, var_sqrtumax_dn7, var_sqrtumax_dn8, var_sqrtumax_dn9,)
    }
};
        var_sqrtumax = assign18740_e19238;
        var_sqrtumax_dn6 = assign18740_e19238_d_n6;
        var_sqrtumax_dn7 = assign18740_e19238_d_n7;
        var_sqrtumax_dn8 = assign18740_e19238_d_n8;
        var_sqrtumax_dn9 = assign18740_e19238_d_n9;

        let (assign18750_e19252, assign18750_e19252_d_n6, assign18750_e19252_d_n7, assign18750_e19252_d_n8, assign18750_e19252_d_n9,) = {
    if ((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) {
        let assign18750_e19250: f64 = (var_umax * var_sqrtumax);
        (assign18750_e19250, ((var_umax_dn6 * var_sqrtumax) + (var_umax * var_sqrtumax_dn6)), ((var_umax_dn7 * var_sqrtumax) + (var_umax * var_sqrtumax_dn7)), ((var_umax_dn8 * var_sqrtumax) + (var_umax * var_sqrtumax_dn8)), ((var_umax_dn9 * var_sqrtumax) + (var_umax * var_sqrtumax_dn9)),)
    } else {
        (var_umaxpoweronepointfive, var_umaxpoweronepointfive_dn6, var_umaxpoweronepointfive_dn7, var_umaxpoweronepointfive_dn8, var_umaxpoweronepointfive_dn9,)
    }
};
        var_umaxpoweronepointfive = assign18750_e19252;
        var_umaxpoweronepointfive_dn6 = assign18750_e19252_d_n6;
        var_umaxpoweronepointfive_dn7 = assign18750_e19252_d_n7;
        var_umaxpoweronepointfive_dn8 = assign18750_e19252_d_n8;
        var_umaxpoweronepointfive_dn9 = assign18750_e19252_d_n9;

        let assign18760_e19254: f64 = (-p.p850);
        let assign18760_e19256: f64 = (assign18760_e19254 * var_one_over_one_minus_pgat);
        let assign18760_e19258: f64 = (-1.0);
        let assign18760_e19259: f64 = if assign18760_e19256 == assign18760_e19258 { 1.0 } else { 0.0 };
        var_guard341 = assign18760_e19259;

        let (assign18770_e19279, assign18770_e19279_d_n6, assign18770_e19279_d_n7, assign18770_e19279_d_n8, assign18770_e19279_d_n9,) = {
    if (((((var_guard175 != 0.0) && (var_guard192 != 0.0)) && (var_guard336 == 0.0)) && (var_guard340 == 0.0)) && (var_guard341 != 0.0)) {
        let assign18770_e19275: f64 = (var_btat * var_umaxpoweronepointfive);
        let assign18770_e19276: f64 = (1.0 + assign18770_e19275);
        let assign18770_e19277: f64 = (1.0 / assign18770_e19276);
        (assign18770_e19277, (-(((var_btat_dn6 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn6)) / (assign18770_e19276 * assign18770_e19276))), (-(((var_btat_dn7 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn7)) / (assign18770_e19276 * assign18770_e19276))), (-(((var_btat_dn8 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn8)) / (assign18770_e19276 * assign18770_e19276))), (-(((var_btat_dn9 * var_umaxpoweronepointfive) + (var_btat * var_umaxpoweronepointfive_dn9)) / (assign18770_e19276 * assign18770_e19276))),)
    } else {
        (var_wgamma, var_wgamma_dn6, var_wgamma_dn7, var_wgamma_dn8, var_wgamma_dn9,)
    }
};
        var_wgamma = assign18770_e19279;
        var_wgamma_dn6 = assign18770_e19279_d_n6;
        var_wgamma_dn7 = assign18770_e19279_d_n7;
        var_wgamma_dn8 = assign18770_e19279_d_n8;
        var_wgamma_dn9 = assign18770_e19279_d_n9;

        *var_asrh_slot = var_asrh;
        *var_asrh_dn6_slot = var_asrh_dn6;
        *var_asrh_dn7_slot = var_asrh_dn7;
        *var_asrh_dn8_slot = var_asrh_dn8;
        *var_asrh_dn9_slot = var_asrh_dn9;
        *var_btat_slot = var_btat;
        *var_btat_dn6_slot = var_btat_dn6;
        *var_btat_dn7_slot = var_btat_dn7;
        *var_btat_dn8_slot = var_btat_dn8;
        *var_btat_dn9_slot = var_btat_dn9;
        *var_dwsrh_slot = var_dwsrh;
        *var_fbreakdown_slot = var_fbreakdown;
        *var_fbreakdown_dn6_slot = var_fbreakdown_dn6;
        *var_fbreakdown_dn7_slot = var_fbreakdown_dn7;
        *var_fbreakdown_dn8_slot = var_fbreakdown_dn8;
        *var_fbreakdown_dn9_slot = var_fbreakdown_dn9;
        *var_guard332_slot = var_guard332;
        *var_guard333_slot = var_guard333;
        *var_guard334_slot = var_guard334;
        *var_guard335_slot = var_guard335;
        *var_guard336_slot = var_guard336;
        *var_guard337_slot = var_guard337;
        *var_guard338_slot = var_guard338;
        *var_guard339_slot = var_guard339;
        *var_guard340_slot = var_guard340;
        *var_guard341_slot = var_guard341;
        *var_ibbt_slot = var_ibbt;
        *var_ibbt_dn6_slot = var_ibbt_dn6;
        *var_ibbt_dn7_slot = var_ibbt_dn7;
        *var_ibbt_dn8_slot = var_ibbt_dn8;
        *var_ibbt_dn9_slot = var_ibbt_dn9;
        *var_id__blk212_slot = var_id__blk212;
        *var_ijungat_slot = var_ijungat;
        *var_ijungat_dn6_slot = var_ijungat_dn6;
        *var_ijungat_dn7_slot = var_ijungat_dn7;
        *var_ijungat_dn8_slot = var_ijungat_dn8;
        *var_ijungat_dn9_slot = var_ijungat_dn9;
        *var_ijunsti_slot = var_ijunsti;
        *var_ijunsti_dn6_slot = var_ijunsti_dn6;
        *var_ijunsti_dn7_slot = var_ijunsti_dn7;
        *var_ijunsti_dn8_slot = var_ijunsti_dn8;
        *var_ijunsti_dn9_slot = var_ijunsti_dn9;
        *var_isrh_slot = var_isrh;
        *var_isrh_dn6_slot = var_isrh_dn6;
        *var_isrh_dn7_slot = var_isrh_dn7;
        *var_isrh_dn8_slot = var_isrh_dn8;
        *var_isrh_dn9_slot = var_isrh_dn9;
        *var_itat_slot = var_itat;
        *var_itat_dn6_slot = var_itat_dn6;
        *var_itat_dn7_slot = var_itat_dn7;
        *var_itat_dn8_slot = var_itat_dn8;
        *var_itat_dn9_slot = var_itat_dn9;
        *var_sqrtumax_slot = var_sqrtumax;
        *var_sqrtumax_dn6_slot = var_sqrtumax_dn6;
        *var_sqrtumax_dn7_slot = var_sqrtumax_dn7;
        *var_sqrtumax_dn8_slot = var_sqrtumax_dn8;
        *var_sqrtumax_dn9_slot = var_sqrtumax_dn9;
        *var_tmp_slot = var_tmp;
        *var_tmp_dn6_slot = var_tmp_dn6;
        *var_tmp_dn7_slot = var_tmp_dn7;
        *var_tmp_dn8_slot = var_tmp_dn8;
        *var_tmp_dn9_slot = var_tmp_dn9;
        *var_twoatatoverthreebtat_slot = var_twoatatoverthreebtat;
        *var_twoatatoverthreebtat_dn6_slot = var_twoatatoverthreebtat_dn6;
        *var_twoatatoverthreebtat_dn7_slot = var_twoatatoverthreebtat_dn7;
        *var_twoatatoverthreebtat_dn8_slot = var_twoatatoverthreebtat_dn8;
        *var_twoatatoverthreebtat_dn9_slot = var_twoatatoverthreebtat_dn9;
        *var_umax_slot = var_umax;
        *var_umax_dn6_slot = var_umax_dn6;
        *var_umax_dn7_slot = var_umax_dn7;
        *var_umax_dn8_slot = var_umax_dn8;
        *var_umax_dn9_slot = var_umax_dn9;
        *var_umaxbeforelimiting_slot = var_umaxbeforelimiting;
        *var_umaxbeforelimiting_dn6_slot = var_umaxbeforelimiting_dn6;
        *var_umaxbeforelimiting_dn7_slot = var_umaxbeforelimiting_dn7;
        *var_umaxbeforelimiting_dn8_slot = var_umaxbeforelimiting_dn8;
        *var_umaxbeforelimiting_dn9_slot = var_umaxbeforelimiting_dn9;
        *var_umaxpoweronepointfive_slot = var_umaxpoweronepointfive;
        *var_umaxpoweronepointfive_dn6_slot = var_umaxpoweronepointfive_dn6;
        *var_umaxpoweronepointfive_dn7_slot = var_umaxpoweronepointfive_dn7;
        *var_umaxpoweronepointfive_dn8_slot = var_umaxpoweronepointfive_dn8;
        *var_umaxpoweronepointfive_dn9_slot = var_umaxpoweronepointfive_dn9;
        *var_vbi_minus_vjsrh_slot = var_vbi_minus_vjsrh;
        *var_wdep_slot = var_wdep;
        *var_wdep_dn6_slot = var_wdep_dn6;
        *var_wdep_dn7_slot = var_wdep_dn7;
        *var_wdep_dn8_slot = var_wdep_dn8;
        *var_wdep_dn9_slot = var_wdep_dn9;
        *var_wgamma_slot = var_wgamma;
        *var_wgamma_dn6_slot = var_wgamma_dn6;
        *var_wgamma_dn7_slot = var_wgamma_dn7;
        *var_wgamma_dn8_slot = var_wgamma_dn8;
        *var_wgamma_dn9_slot = var_wgamma_dn9;
        *var_wsrh_slot = var_wsrh;
        *var_wsrhstep_slot = var_wsrhstep;
    }
}
